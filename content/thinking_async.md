---
title: thinking async
description: 'clutter thinking about async'
template: blog/page.html
date: 2023-07-19 15:22:51
updated: 2023-07-19 15:22:51
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ['async', 'cpp']
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

This blog is only my clutter thinking about async(ASIO / SR model comparison) and record the resouces that I readed.

# the investigating process record
First, I only use asio as my networking library, and then I find the [think-async](https://think-async.com/Asio/) and the video in it. So I know asio also is the asynchrounous modle at the first time, and I begin to investigate the async model in c++ standard committee. Then refer to the reference, I figure out the NetTS(aka. ASIO) has been the experimental standard at the 2014, but not layering as the P2300 Sender/Receiver model proposed. I want to know the detail and query some materials that is the references.

# The Sender/Receiver understand
At the beginning, I do not understand what is the sender/recevier, instinctly it seems the goroutine concept, one channel that has sender and receiver. And I learn the [video](https://www.youtube.com/watch?v=h-ExnuD6jms) from Eric Niebler[The Range library author]. It explain `std::future and std::promise pair` limitation, and create the  `lazyfuture/lazypromise`  concept to break through. The LazyFuture and LazyPromise rename to Sender and Receiver. IMP, the Sender is send the value that program need, and Receiver receive the value from producer. So connecting them can connect the producer and consumer.

The important idea is lazy. Lazy cause the compiler can construct the task graph at compile time and optimize the compute process.

# The ASIO design philosophy (from P2444)
- Be flexible in supporting composition mechanisms, since the appropriate choice depends on the specific use case
- Aim to support as many of the semantic and syntactic properties of synchronous operations as possible, since they enable simpler composition and abstraction
- Application code should be largely shilded from the complexify of threads and synchronisation, due to the complexiry of handling events from different sources.

PS. eager execution matters for performance

# references
- [std::execution](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/p2300r7.html#design-implementer)
- [asio async model](https://isocpp.org/files/papers/P2444R0.pdf)
- https://www.reddit.com/r/cpp/comments/q6tgod/c_committee_polling_results_for_asynchronous/
- [stdexec github repo](https://github.com/NVIDIA/stdexec)
  - [eric niebler slides and presentation](https://www.youtube.com/watch?v=h-ExnuD6jms)
  - [eric niebler live codeing](https://www.youtube.com/watch?v=xiaqNvqRB2E)
  - [executors: A change of perspective](https://accu.org/journals/overload/29/165/teodorescu/)
- [NetTS, ASIO and Sender Library Design Comparison](https://isocpp.org/files/papers/P2471R1)
- [Ruminations on networking and executors](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2021/p2464r0.html)
- [Response to P2464: The Networking TS is baked, P2300 Sender/Receiver is not](https://github.com/cplusplus/papers/issues/1114)
  - https://isocpp.org/files/papers/P2469R0.pdf



# extra reference
- [Sender/Receiver Interface For Networking](https://github.com/cplusplus/papers/issues/1447)
  - [only an education presentation](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2023/p2762r0.pdf)
- [concurrencpp git repo](https://github.com/David-Haim/concurrencpp)
  - when_any
  - when_all