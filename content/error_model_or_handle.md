---
title: Error modle and handle
description: 'error model and handle thinking from boost.outcome'
template: blog/page.html
date: 2023-07-19 16:21:51
updated: 2023-07-19 16:22:51
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ['error model', 'error handle', 'cpp']
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

# The origin
**原文**
[The Error Model](http://joeduffyblog.com/2016/02/07/the-error-model/) and [知乎翻译](https://zhuanlan.zhihu.com/p/55835404)

## summary
## Error Codes
```cpp
int err = foo();
if (err)  // error! deal with it
```

Many functional languages use return codes disguised in monads and names things like Option<T>, Maybe<T>, Error<T>, which, when coupled with ad dataflow-style of programming and pattern matching, feel far more natural. This approach removes several majar drawbacks to return codes that we're about to discuss, especially compared to C. Rust has largely adopted this model but has some exciting things with it for system programmers.

Dispite theis simplicity, return codes do some with some baggage; in summary:
- performance can suffer (branch that compiler can't optimize)
- Programming model usabiliry can be poor
- The biggie: you can accidentally forget to check for errors (**Rust guaratee by compiler**)

上面文章也提到 Rust 在除了性能之外的方面已经做的很好 (pattern matching and try! macro)

But we need fail-fast -> exception

## Exception
[cpp deprecate exception specifications](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2010/n3051.html), so `noexcept` is also bad. [cpp exception](@/cpp_exception.md)

## recap
|  | the good | the bad | the ugly |
|--|--|--|--|
| Error Code | 1. All function that can fail are explicit annotated<br />2. All error handling at callsites is explicit | 1. you can forget to check them<br />2. Performance of success paths suffers | usability is often subpar |
| All Excpetions | first class language support | 1. Performance is typically worse than it could be<br />2. handling is often done in a non-local manner, where less information about an error is known(goto-like) |  |
| Unchecked Exceptions | conducive to rapid development where dealing with errors reliably isn't critical | Anything can fail without warning from the language | reliability is as bad as it gets |
| Checked Exceptions | all function that can fail are explicitly annotated | 1. callsites aren't explicit about can fail and error propagation<br />2. systems that let some subset of exceptions go unchecked poison the well(not all errors are explicit) | people hate them(in java, at least) |

Wouldn't it be great if we could take all of the Goods and leave out the Bads and The Uglies?



## Bugs aren't recoverable errors

- A recoverable error is usually the result of programmatic data validation.
- A bug is a kind of error the programmer didn't expect.

> The wrong point: Exceptions and return codes are equally expressive, they should however not be used to describe errors. For example, the return codes contained definitions like ARRAY_INDEX_OUT_OF_RANGE. It cannot be handled or fixed at runtime, it can only be fixed by its developer. Thus there should be no according return code, but instead there should be assert.



## Reliabiliry, Fault-Tolerance, and Isolation

### To build a reliable system

As with most distributed systems, our architecture assumed process failure was inevitable. We went to great length to defend aginst cascading failures, journal regularly, and to enable restartability of programs and services.

### Abandonment

Perhaps the most important technique is regularly journaling and checkpointing precious persistent state. Jim Gray’s 1985 paper, [Why Do Computers Stop and What Can Be Done About It?](http://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.110.9127&rep=rep1&type=pdf), describes this concept nicely.

### Recoverable errors: type-directed exceptions

For example, file I/O, network I/O, parsing data, validating user data.



The exception has good performance on the good path, it's zero-cost.



## Restrospective and conslusion

The final model in the article featured:

- An architecture that assumed fine-grained isolation and recoverability from failure.
- Distinguishing between bugs and recoverable errors.
- Assertions, abandonment for all bugs

---------

So next, we should look at the C++ system. And if we do not use exception, how to introduce the Rust smell code into C++.

# Boost.outcome , std::expected, and boost.leaf

`Boost.outcome.result` is more like `std::variant`, and `std::expected(C++23)` is more like `std::optional` extension. The detail in reference [how std::expected interacts with boost.outcome].Dissimilarities section.

## Rust error handling

[rust book](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) show rust how to modle the error and handle it. Recommend [the guidelines for error handling](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling)

- Unrecoverable error with panic!
  - `panic!` can record the backtrace
- Recoverable errors with Result
  - pattern matching
  - Propagating error with `?`

## Result (Either) in C++

I personally like the Rust abstract, instead of error code, it propose the `Reulst<T, E>`, and outcome.result can be used in c++.



## Leaf comfortable scenoria

If you need an error handling framework which has predictable sad path overhead unlike C++ exceptions, but you otherwise want similar syntax and use experience to C++ exceptions, LEAF is a very solid choice. ~~Do not use in my ray-like project, i do not like the exception when should support API for application user.~~

# reference

- https://www.boost.org/doc/libs/develop/libs/outcome/doc/html/index.html
  - https://www.boost.org/doc/libs/develop/libs/outcome/doc/html/reference/types/basic_result.html
  - https://www.boost.org/doc/libs/develop/libs/outcome/doc/html/alternatives/leaf.html
  - https://boostorg.github.io/leaf/#_reporting_errors
- [std::expected proposal](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2022/p0323r12.html#cool-story)
- [how std::expected interacts with boost.outcome](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2017/p0762r0.pdf)
- [what is vocabulary type?](https://open-std.org/JTC1/SC22/WG21/docs/papers/2020/p2125r0.pdf)
- [What is a monad?](https://builtin.com/software-engineering-perspectives/monads)