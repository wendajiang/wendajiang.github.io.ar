---
title: Profile (CPU or Memory)
description: ''
template: blog/page.html
date: 2023-08-10 00:18:41
updated: 2023-08-10 00:18:41
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["profile"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

# introduction

There alternative through to profile the memory usage:
- perf record -e syscalls:sys_enter_mmap/brk -e page-faults && perf script && flamegraph git repo perl tools output the flamegraph.svg
- bpf tools recommand the linux kernel version is new
- using tcmalloc&jemalloc and enable profile feature, using jeprof/pprof to show the heap malloc dump
  - tcmalloc must compile application with -ltcmalloc
  - jemalloc maybe not, need test
    after test, jemalloc can be used without -ljemalloc, but the prof result is inaccurate, so should be compile with -ljemalloc

a)	perf record only can record the syscalls:sys_enter_mmap/brk, can’t trace the malloc, and the perf data is huge
b)	ebpf need newer linux kernel version, dev host can not support
c)	Using jemalloc to compile with vcom, and jeprof tool to show the sample data, cause vcom various strange crash or hangs.
d)	tcmalloc profiler too slow

    

# references
- [brendangregg website](https://www.brendangregg.com/)
  - https://www.brendangregg.com/blog/2021-05-23/what-is-observability.html
  - https://www.brendangregg.com/perf.html#Prerequisites
- [jemalloc how to transfer flamegraph](https://zhuanlan.zhihu.com/p/558677729)
- [jemalloc basic usage of heap profiling](https://github.com/jemalloc/jemalloc/wiki/Use-Case%3A-Heap-Profiling)
- [jemalloc tune](http://jemalloc.net/jemalloc.3.html#tuning)
- [jemalloc deadlock, and enable-debug to catch dange memory operation](https://github.com/jemalloc/jemalloc/issues/1318)
- http://ithare.com/testing-memory-allocators-ptmalloc2-tcmalloc-hoard-jemalloc-while-trying-to-simulate-real-world-loads/
- https://www.cyningsun.com/07-07-2018/memory-allocator-contrasts.html
- [tcmalloc-depecrated gperftools](https://github.com/google/tcmalloc/blob/master/docs/gperftools.md)
  - [gperftools](https://gperftools.github.io/gperftools/heapprofile.html)
