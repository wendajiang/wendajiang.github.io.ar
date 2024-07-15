---
title: c++ exception that is how to implement in zero-cost?
description: ''
template: blog/page.html
date: 2024-07-14 17:19:09
updated: 2024-07-14 17:19:09
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: []
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
Zero-overhead is not zero cost, Hutter proposal one determintic-exception, but Bjarne Stroutrup is negative about it.

The origins of exception handling lie in the problems experienced managing a variety of error-handling approaches, such as **C’s errno, error-states, callbacks, return codes, and return objects**. In addition, it was observed that there were no really good approaches to reporting errors detected in constructors and in operators.

# overview
1. SjLj(setjump-longjump)
   It is simpler to implement but has more runtime overhead, and now is deprecated in modern compiler.
2. table-based (zero-overhead)
   - .eh_frame
   - .gcc_except_table


# reference
- https://www.quora.com/How-does-gcc-implement-C++-exception-handling
  - https://www.quora.com/Do-exceptions-slow-down-C++-programs-even-if-no-exception-is-thrown-since-it-has-to-be-checked-whether-a-function-has-returned-or-not/answer/David-Vandevoorde
  - https://www.quora.com/How-is-RAII-implemented-by-the-C++-compiler/answer/David-Vandevoorde
- [the true assembly analysis](https://stackoverflow.com/questions/307610/how-do-exceptions-work-behind-the-scenes-in-c/307716#307716)
  - [the C++ ABI](https://itanium-cxx-abi.github.io/cxx-abi/)
- [.eh_frame](https://www.airs.com/blog/archives/460)
- [.gcc_except_table](https://www.airs.com/blog/archives/464)
- [Compiler Internals: Exceptions and RTTI. 2012. windows and Unix-like system](https://wendajiang.github.io/pdf/Compiler-Internals-exception.pdf)
- [C++ Exception Handling for IA-64](https://wendajiang.github.io/pdf/exception.pdf)
- [C++ exceptions and alternatives.Bjarne Stroustrup](https://wendajiang.github.io/pdf/p1947r0exception.pdf)
- a tour of c++, 3rd edition chapter 4.