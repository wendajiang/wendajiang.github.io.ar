---
title: strip
description: ''
template: blog/page.html
date: 2023-06-08 00:07:57
updated: 2023-06-08 00:07:57
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ['binutils', 'strip', 'link', 'so']
extra:
  mermaid: false
  usemathjax: true
  lead: ''
---

# real-world scenario

When we release our products, that is cpp project, we `strip` all files, then we can't not compile. The problem is the cpp project's dependencies have static library and striped static library can not be linked.

So I google the strip

## strip execute binary
no problem when execute
## strip object file
After strip the object file, and compile it into binary, **link error happen**.
## strip shared library
After strip the shared library, compile success, and run it correctly.

So why the shared library (the .symtab section removed) can run correctly?

# [Shared library](@/shared_library.md)
Even stripped libraries still must retain the symbols necessary for dynamic linking. There are usually placed in a section named `.dynsym` and are also pointed to by the entries in the dynamic secion.

I can see that event though the stripped library miss the `.symtab` section, the `.dymsym` is still present. In fact, the section table can be removed as well(.e.g with sstrip) and the file will still work. This is because the dynamic linker only uses the program headers(aka the segment table), the `DYNAMIC` segment corresponds to the `.dynamic` section and contains information for the dynamic linker.

In the `.dynamic` section, `STRTAB` and `SYMTAB` the two entries are necessary for symbol resolution. The together make up the dynamic symbol table:
```bash
Symbol table '.dynsym' contains 91 entries:
Num: Value   Size Type     Bind  Vis      Ndx   Name
  0: 0000000    0  NOTYPE  LOCAL  DEFAULT UND
  ...
  6: 0000000    0  FUNC    GLOBAL  DEFAULT UND  clock_gettime
  7: 000026e1   88 FUNC    GLOBAL  DEFAULT 7    ifc_init
```
You can see that is contains both `UND`(undefined) symbols - those required by the library and imported from other .so, and the 'normal' global symbols which are exported by the library for its users. The exported symbols have their addresses inside the library listed in the Value column.

# reference
- https://reverseengineering.stackexchange.com/questions/2038/how-are-stripped-shared-libraries-linked-against
- http://wen00072.github.io/blog/2015/11/16/tan-tan-strip/
- https://www.internalpointers.com/post/journey-across-static-dynamic-libraries

# futher read
- https://tldp.org/HOWTO/Program-Library-HOWTO/index.html
- how to write shared library [Ulrich Drepper]
