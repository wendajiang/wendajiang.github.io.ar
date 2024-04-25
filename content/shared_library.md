---
title: shared library
description: ''
template: blog/page.html
date: 2023-03-04 09:07:19
updated: 2024-04-25 11:01:19
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["so", "link"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'summary of shared library'

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

update at 2023-06-15

update at 2023-07-27

# problem
In one scenoria, I find this:

```bash
> ldd main
>    libstdc++.so.6 => /lib64/libstdc++.so.6 (0x00007ffff7828000)
     ...
     /home/david/foo.so (0xxxxx)
     /lib64/ld-linux-x86-64.so.2 (0x00007ffff7dd4000)
```

And if the /home/david/foo.so do not exist, I try to set LD_LIBRARY_PATH, change the rpath, and all not work. Why?

## What is mean? 
In the ELF execute file format, the dynamic library record the \[NEEDED\] section, and when \[NEEDED\] secion has no `/` slash, the library directory resolve at run-time, and the rules order is

- rpath
- LD_LIBRARY_PATH environment
- runpath
- /etc/ld.so.cache checked to see if it contains an entry for the library
- /lib or /usr/lib

But if the \[NEEDED\] section has / slash, the execute do not search by above rules, and directly use the path dynamic library.

We should note, -Wl,-rpath=some/path
[Why LD_LIBRARY_PATH is bad](http://xahlee.info/UnixResource_dir/_/ldpath.html)


# How to resolve it?

## First should find why it happended that the section write absulote path

```bash
> gcc -o main /home/david/foo.so
```

This command cause it. However, I try this
```bash
> gcc -o main /home/david/foo.so /lib64/libstdc++.so.6
```
The libstdc++.so.6 is still the name and need to be searched at runtime, Why? 

## The underhood
```bash
> man ld
```
I find the -soname option, it's decribed as below:

> When creating and ELF shared object, set the internal DT_SONAME field to the specified name. When an executable is linked with a shared object which has a DT_SONAME field, then when the executable is run the dynamic linker will attempt to load hte shared object specified by the DT_SOMANE field rather than the using the file name given to the linker.

## Solution

So, I can create the foo.so by
```bash
> gcc -shared -Wl,-soname,libfoo.so foo.cc -o libfoo.so
```
And, the main executable \[NEEDED\] section will be the libfoo.so name rather than the absolute path.

-----------------

# Continuation
## -Bsymbolic linker option
After learning the soname, I read the `Bsymbolic` linker option in the tlpi book 41.12\[Run-Time Symbol Resolution\].

```cpp
// prog
void xyz() {
  printf("main-xyz\n");
}

void main() {
  func();
}

// libfoo.so
void xyz() {
  printf("foo-xyz\n");
}

void func() {
  xyz();
}
```

What's happened? It print `main-xzy`. 

- A definition of a global symbol in the main program overrides a definition in a library.
- If a global symbol is defined in multiple libraries, then a reference to that symbol is bound to the first definition found by scanning libraries in the left-to-right order in which they were listed on the static link command line.

These semantics make the transition from static to shared libraries relatively strightforward. But the most significant problem is that these semantics conflict with the model of a shared library as implementing a self-contained subsystem. By default, a shared library can't guarantee that a reference to one of its own glboal symbols will actually be bound to the library's definition of that symbol.

In the above scenario, if we wanted to ensure that the invocation of `xyz()` in the shared library actually called the version of the function defined within the library, then we could use the `-Bsymbolic` linker option when building the shared library.

```bash
man ld
```
> -Bsymbolic 
> 
> When creating a shared library, bind references to global symbols to the definition within the shared library, if any. Normmally, it is possible for a program linked against a shared library to override the definition within the shared library. This option can also be used with the --export-dynamic option, when creating a position independent executable, to bind references to global symbols to the definition within the executable. This option is only meaningful on ELF paltforms which support shared libraries and position independent executables.


## But, the Bsymbolic maybe cause ugly problem

Oppus, I learn it first, and want to using it for my shared library. And the option conflict with [doctest](@/testing_framework.md) using. 

I deep learn it and find if using singleton, it's expected behavior is if main and shared-library using the one single instance, the `-Bsymbolic` option destroy it.

So the `-Bsymbolic` using scenoria is the shared-library is self-contained model.

the reference https://maskray.me/blog/2021-05-16-elf-interposition-and-bsymbolic explain the reason (pointer equality)
and https://flameeyes.blog/2012/10/07/symbolism-and-elf-files-or-what-does-bsymbolic-do/

# [Supplement](https://www.linuxjournal.com/article/6463) Linkers & Loaders
*Linking* is the process of combining various pieces of code and data together to form a single executable that can be loader in memory. Linking can be done at compile time, at load time(by loaders) and also at run time (by application programs).

## Compiler, Linker and Loader in Action
```bash
cpp other-command-line-options a.c /tmp/a.i # preprocessor
cc1 other-command-line-options /tmp/a.i -o /tmp/a.s # compiler proper
as other-command-line-options /tmp/a.s -o /tmp/a.o # assembler
# repeat for file b.c

ld other-command-line-options /tmp/a.o /tmp/b.o -o a.out
```

## Linkers vs Loaders
- Program Loading. This refers to copying a program image from hard disk to the main memory in order to put the program in ready-to-run state. In some cases, program loading also might invole allocating storage space or mapping virtual addresses to disk pages.
- Relocation. Compilers and assemblers generate the object code for each input module with a starting address of zero. Relocation is the process of assigning load addresses to different parts of the program by merging all sections of the same type into one section. The code and data section also are adjusted so they point to the correct runtime addresses.
- Symbol Resolution. A program is made up of multiple subprograms; reference of one subprogram to another is made through symbols. A linker's job to resolve the reference by noting the symbol's location and patching the caller's object code.

So a condiderable overlap exists between the functions of linkers and loaders. One way of think of them is: the loader does the program loading; the linker does the symbol resolution; and either of them can do the relocation.

## Object Files
- Relocatable object file, which contains binary code and data in a form that can be combined with other relocatable object files at compile time to create an executable object file.
- Executable object file, which contains binary code and data in a form that can be directly loader into memory and executed.
- Shared object file, which is a special type of relocatable object files that can be loaded into memory and linker dynamically, either at load time or at run time.

ELF:

| ELF Header | starts with a 4-byte magic string \177ELF                    |
| ---------- | ------------------------------------------------------------ |
| .text      | the machine code of the compiled program                     |
| .rodata    | Read-only data, such as the format strings in printf stmt    |
| .data      | initialized global variables                                 |
| .bss       | uninitialized global varitables. BSS stands for block storage start, and this section actually occupies no space in the object file; it is merely a placer holder |
| .symtab    | a symbol table with information about functions and global variables defined and referenced in the program. This table does not contain any entries for local variables; those are maintained on the stack |
| .rel.text  | a list of locations in the .text section that need to be modified when the linker combines this object file with other object files |
| .rel.data  | relocation information for global variables referenced but not defined in the current module |
| .debug     | a debugging symbol table with entries for local and global variables. This section is present only if the compiler is invoked with -g option |
| .line      | a mapping between line numbers in the original C source program and machine code instructions in the .text section. This information is required by debugger programs |
| .strtab    | a string table for the symbol tables in the .symtab and .debug sections |

## Symbols and symbol resolution
Every relocatable object file has a symbol table and associated symbols. In the context of a linker, the following kins of symbols are present:
- Global symbols defined by the module and referenced by other modules. All non-static functions and global varitables fall in this category
- Global symbols referenced by the input module but defined elsewhere. All functions and variables with extern declaration fall in this category
- Local symbols defined and referenced exlusively by the input module. All static functions and static variables fall here.

**At compile time, the compilers exports each global symbol as either strong or weak. Functions and initialzed global variables get strong weight, while global uninitialized varaibles are weak.**
### Linking with static libraries
During the process of symbol resolution using static libraries, linkers scans the relocatable object files and archives from left to right as input on the command line. During this scan, linker maintains a set of O, relocatable object files that go into the executable; a set U, unresolved symbols; and a set of D, symbols defined in previous input modules. Initially, all three sets are empty.
- for each input argument on the command line, linkers determintes if input is an object files or an archive. If input is relocatable object file, linkers adds it to set O, updates U and D and proceeds to next input file.
- if input is an archive, it scans through the list of member modules that constitute the archive to match any unresolved symbols present in U. If some archive member defines any unresolved symbol that archive member is added to the list O, and U and D are updated per symbols found in the archive member. This process is iterated for all member object files.
- After all the input arguments are processed through the above two steps, if U is found be not empty, linker prints an error report and terminates. Otherwise, it merges and relocated the object files in O to build the ouput executable file.

This also explains why static libraries are placed at the end of the linker command. **Special care must be taken in cases of cyclic dependencies libraries.** Input libraries must be ordered so each symbol is referenced by a member of an archive and at least one definition of a symbol is followed by a reference to it on the command line. Also, if an unresolved symbol is defined in more than one static library moduels, the definition is picked from the first library found in the command line.
## Relocation
Once the linker has resolved all the symbols, each symbol reference has exactly one definition. At this point, linker starts the process of relocation, which involves the following two steps:

- Relocaing sections and symbol definitions. Linker merges all the sections of the same type into a new single section. The linker then assigns runtime memory addresses to new aggregate sections, to each section defined by the input module and also to each symbol. After the completion of this step, every instruction and global variable in the program has a unique loadtime address
- Relocating symbolo reference within sections. In this step, linker modifies every symbol reference in the code and data sections so they point to the correct loadtime addresses.

Whenever assembler encounters an unresolved symbol, it generates a relocation entry for that object and places it in the .relo.text/.relo.data sections. A relocation entry contains information about how to resolve the reference. A typical ELF relocation entry contains the following members:

- Offset, a section offset of the reference that needs to be relocated. For a relocatable file, this value is the byte offset from the beginning of the section to the storage unit affected by relocation.
- Symbol, a symbol the modified reference should point to. It is the symbol table index with respect to which the relocation must be made.
- Type, the relocation type, normally R_386_PC32, that signifies PC-relative addressing. R_386_32 signifies absolute addressing.

The linker iterates over all the relocation entries present in the relocatable object modules and relocates the unresolved symbols depending on the type. For R_386_PC32, the relocating address is calculated as S + A - P; for R_386_32 type, the address is calculated as S + A. In these calculations, S denotes the value of the symbol from the relocation entry, P denotes the section offset or address of the storage unit being relocated (computed using the value of offset from relocation entry) and A is the address needed to compute the value of the relocatable field.

## Dynamic linking: Shared libraries
A shared library is an object module that can be loaded at run time at an arbitrary memory address, and it can be linked to by a program in memory.

-fPIC option tells the compiler to generate position independent code (PIC)

```bash
gcc -shared -fPIC -o libfoo.so a.o b.o
gcc bar.o ./libfoo.so
```
The executable simply contains some relocation and symbol table information that allow references to code and data in libfoo.so to be resolved at run time. Thus, a.out here is a partially executable file that still has its dependency in libfoo.so. The executable also contains a .interp section that contains the name of the dynamic linker, which itself is a shared object on Linux systems (ld-linux.so). So, when the executable is loaded into memory, the loader passes control to the dynamic linker. The dynamic linker contains some start-up code that maps the shared libraries to the program's address space. It then does the following:

- relocates the text and data of libfoo.so into memory segment; and
- relocateds any references in a.out to symbols defined by libfoo.so

Finaly, the dynamic linker passes control to the application. From this point on, location of shared object is fixed in the memory.

## Loading shared libraries from applications
dlopen / dlsym / dlclose


## Tools for manipulating object files
- ar: creates static libraries
- objdump: this is the most important binary tool; it can be used to display all the information in an object binary file
- strings: list all the printable strings in binary file
- nm: lists the symbols defined in the symbol table of an object file
- ldd: lists the shared libraries on which the object binary is dependent
- strip: deletes the symbol table information

# executable shared-library
This [blog](http://www.rkoucha.fr/tech_corner/executable_lib.html) describe in detail.

[stackoverflow](https://stackoverflow.com/questions/1449987/building-a-so-that-is-also-an-executable/1451482#1451482) introduct a more realistic example [pcap](https://git.kernel.org/pub/scm/libs/libcap/libcap.git/tree/libcap/execable.h).


# reference
- https://tldp.org/HOWTO/Program-Library-HOWTO/shared-libraries.html
- ```bash
  nm -g main | rg rpath
  readelf -p man | rg -i needed
  objdump -p main | rg -i needed
  ```
- the rules order of runtime search dynamic library is tlpi book section 41.11
- Another useful environment variable in the GNU C loader is LD_DEBUG. This triggers the dl* functions so that they give quite verbose  information on what they are doing. For example:
  ```bash
  export LD_DEBUG=files
  command_to_run
  ```
  displays the processing of files and libraries when handling libraries, telling you what dependencies are detected and which SOs are loaded in what order. Setting LD_DEBUG to `bindings` displays information about symbol binding, setting it to `libs` displays the library search paths, and setting it to `versions` displays the version depdendencies.
  Setting LD_DEBUG to `help` and then trying to run a program will list the possible options. Again, LD_DEBUG isn't intended for normal use, but it can be handy when debugging and testing.
- https://zerol.me/2021/06/13/Linker-Symbol-Conflict/
- https://maskray.me/blog/2021-05-16-elf-interposition-and-bsymbolic
- https://flameeyes.blog/2012/10/07/symbolism-and-elf-files-or-what-does-bsymbolic-do/
- [ldd and linker(ld) both use RPATH/RUNPATH to find the dependencies](https://stackoverflow.com/questions/49138195/whats-the-difference-between-rpath-link-and-l)
- [rpath and runpath order and exploration](https://medium.com/obscure-system/rpath-vs-runpath-883029b17c45)