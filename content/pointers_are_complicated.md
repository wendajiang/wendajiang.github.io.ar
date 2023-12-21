---
title: pointers are complicated
description: ''
template: blog/page.html
date: 2023-12-21 14:46:37
updated: 2023-12-21 14:46:37
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["compiler", "gcc", "llvm"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

Provenance notion for LLVM IR is important when implementing compiler which has pointer(e.g. Rust, C, C++).

Otherwise, compiler optimization would cause error result.

# reference
- [src](https://www.ralfj.de/blog/2020/12/14/provenance.html)
- [open-std Aprovenance-aware Memory Object Model for C](https://www.open-std.org/jtc1/sc22/wg14/www/docs/n3005.pdf)
- [gcc(6.4.0) bug report](https://gcc.gnu.org/bugzilla/show_bug.cgi?id=82282s)
