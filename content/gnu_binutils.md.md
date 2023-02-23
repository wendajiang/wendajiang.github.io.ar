+++
template = "blog/page.html"
date = "2021-04-20 16:44:55"
title = "GNU gcc 工具集整理, GNU Binutils"
[taxonomies]
tags = ["gcc", "tool"]

[extra]
mermaid = true
usemathjax = true
+++
<!--
mermaid example:
<div class="mermaid">
    mermaid program
</div>
-->

[GNU Binutils](https://www.gnu.org/software/binutils/)

### GNU Binutils
GNU Binutils 是一套二进制工具集，主要由：

- **ld** -- 链接器 
- **as** -- 汇编器

还包括

- **addr2line** - Converts addresses into filenames and line numbers.
  backtrace/backtrace_symbol 输出的地址可以使用addr2line转化为 
- **ar** - A utility for creating, modifying and extracting from archives.
  ar，Linux系统的一个备份压缩命令，用于创建、修改备存文件（archive），或从备存文件中提取成员文件。ar命令最常见的用法是将目标文件打包为静态链接库。
- **c++filt** - Filter to demangle encoded C++ symbols.
  将符号转为代码,比如 
  c++filt    _Z6myfunci    -> myfunc(int)   (centos 系统示例)
  
- **gold** - A new, faster, ELF only linker, still in beta test.
  暂时忽略
- **gprof** - Displays profiling information.
- **nlmconv** - Converts object code into an NLM.
- **nm** - Lists symbols from object files.
- **objcopy** - Copies and translates object files.
- **objdump** - Displays information from object files.
  
- **ranlib** - Generates an index to the contents of an archive.
- **readelf** - Displays information from any ELF format object file.
- **size** - Lists the section sizes of an object or archive file.
  例子：
  size bt_test
   text    data     bss     dec     hex filename
   2553     672      16    3241     ca9 bt_test

- **strings** - Lists printable strings from files.
- **strip** - Discards symbols.
  "脱裤"，将文件中的符号表脱掉，"脱裤"后的文件无法使用addr2line来转化代码位置

windows？暂时不管
- **dlltool** - Creates files for building and using DLLs.
- **windmc** - A Windows compatible message compiler.
- **windres** - A compiler for Windows resource files.

大多数程序使用 BFD(Binary File Descriptor library)来做 manipulation。其中很多还是用 opcodes 库来汇编和反汇编机器指令

### 参考
[ar,nm,ranlib使用详解](https://www.jianshu.com/p/2ec7ee43e3a1)


### 附录
nl -- 列出文件以及行数
