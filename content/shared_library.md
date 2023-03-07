---
title: shared library
description: ''
template: blog/page.html
date: 2023-03-04 09:07:19
updated: 2023-03-07 11:01:19
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