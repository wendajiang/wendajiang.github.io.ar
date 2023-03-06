---
title: shared library
description: ''
template: blog/page.html
date: 2023-03-04 09:07:19
updated: 2023-03-04 09:07:19
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

# How to solute it?

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
  displays the processing of files and libraries when handling libraries, telling you what dependencies are detected and which SOs are loaded in what order. Setting LD_DEBUG to ``bindings'' displays information about symbol binding, setting it to ``libs'' displays the library search paths, and setting ti to ``versions'' displays the version depdendencies.
  Setting LD_DEBUG to ``help'' and then trying to run a program will list the possible options. Again, LD_DEBUG isn't intended for normal use, but it can be handy when debugging and testing.