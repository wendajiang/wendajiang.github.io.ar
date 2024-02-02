---
template: blog/page.html
date: 2022-12-15 18:07:49
title: gcc/g++ compile options
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["gcc"]
extra:
  mermaid: true
  usemathjax: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

onenote 迁移文



| option                  | 解释                                                         |
| ----------------------- | ------------------------------------------------------------ |
| -L                      | 指定链接库需要的目录 -L/usr/lib                              |
| -l                      | 链接库名称 -ltest.    -Wl,-Bstatic 静态库 -Wl,-Bdynamic 动态库 |
| -I                      | 头文件目录                                                   |
| -shared                 | 指定生成的动态库                                             |
| -fPIC                   | shared library 独立位置代码，如果不用，每个使用 so 的 binary 会 copy 到自己进程空间一份，无法实现真正的 share library |
| gcc -E test.c -o test.i | 预处理                                                       |
| gcc -S test.i -o test.s | 编译                                                         |
| gcc -c test.s -o test.o | 汇编                                                         |
| gcc test.o -o test      | 链接                                                         |
| -O                      | 编译优化 -O0, -O1(default), -O2, -O3                         |
| -g                      | 生成gdb 调试信息                                             |
| -pg                     | gprof extra code                                             |
| -Wall                   | 所有的警告信息 -W 很多选项                                   |
| -Wl                     | 传递给 linker 的 option。-Wl,-R runtime lib 路径             |
| -Wa                     | 传递给 assmbler 的 option                                    |



> ar 工具可以将 object file 压缩到一起，ex. ar cqs libstaticlib.a one.o two.o three.o
