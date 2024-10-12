---
title: team topologies review
description: ''
template: blog/page.html
date: 2024-09-28 18:16:59
updated: 2024-09-28 18:16:59
typora-copy-images-to: ../../static/pics/${filename}
taxonomies:
  tags: ["team design"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

## conway's law and why it matters

康威定律：如果设计一个编译器，你有四个团队，你会获得一个 4-pass 的编译器。

组织结构大于软件架构 -> 如果最开始设计了软件架构，但是没有对应的组织架构，最后获得的软件产品设计一定更像组织结构。

通常人们不会将认知负载考虑进来，但是这是非常影响团队效率的问题，尤其是团队时间长之后，一直增加负责项目，就忽略掉了对于老项目维护的负担，项目切换是需要消耗时间和精力的。

低耦合，高内聚一样适用于团队设计。提出了检测团队健康度的一个方法，如果团队之间发生了非预期的交流，就要看看是不是软件设计不匹配当前的团队设计，是不是 API 接口设计不合理，或者少了一些中间件等。

### naive 应用 conway's law 也会负面作用

每个子系统都设计成单独的小团队并不是很好的设计 -- @todo

重新调整组织结构的底层驱动过去常常是裁员或者增加领导的“领地”威望。但是根据康威定律来调整是为了提升软件系统。这两者是不兼容的。强有力的说，为了管理方便或者减少 hc 进行的组织调整会摧毁组织有效构建软件的能力。忽视康威定律进行组织调整就像是儿童进行心脏手术：高度破坏性。



## Team-first thinking

这本书，team 一词的特定含义：稳定的，5-9个人工作于一个目标的团队单元，组织结构中不可分割的一部分。



