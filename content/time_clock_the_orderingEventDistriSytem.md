+++
template = "page.html"
date = "2021-04-06 21:31:59"
title = "【考古】【必读】分布式理论奠基paper"
[taxonomies]
tags = ["awesomepaper", "translate"]

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

原文标题：Time, Clocks, and the Ordering of Events in a Distributed System
原文作者：Leslie Lamport
[原文链接](https://lamport.azurewebsites.net/pubs/time-clocks.pdf)

译者声明：

本文中的进程（process）表示过程，不是典型意义上计算机的进程

偏序、局部顺序 partial ordering

## Abstract

在分布式系统中一个事件发生于另一个事件之前的概念需要被确定，定义为这两个事件的偏序(partial ordering)。给出一个分布式算法来同步系统的逻辑时钟，时钟可以用来全排序所有的事件。全序可以被用来解决事件同步问题。这个算法还被特化为解决同步物理时钟问题，可以容忍多长的时钟同步延迟。

## Introduction

时间是一个基本的概念。可以表示事件发生的顺序。我们说事件发生在 3:15 在时钟表示3:15，以及3:16之前。事件时间顺序的思考无处不在。比如，在航空公司预留系统中，我们声明在票买完之前，可以提供预留服务。与此同时，在考虑分布式系统中的事件时，我们必须慎重审视这个概念。

分布式系统中包含了一系列分离的进程，通过消息互通。网络连接起来的互联网（译者注：此处的互联网不是现在概念上的互联网，本文写于计算机组网早期，比如 ARPA网络系统）就是一个分布式系统。单独的一台计算也可以被看做分布式系统，其中中控单元，内存单元，I/O都是分离的进程。**如果消息的传递延迟相比于单个进程中的事件之间的事件是不能忽略的，我们就说系统是分布式的。**

我们主要考虑空间上分隔的计算机。然而，很多结论可以延伸到更广泛的其他系统中。尤其是，计算机上的多进程系统跟多个计算机组成的分布式系统本质上是相同的，因为都具备**事件发生顺序不可预测的特征。**

在分布式系统中，有时是说不清楚两个事件的哪个先发生的（译者注：物理时间意义的先）。因此"happened before"的关系实际上应该是两个事件局部顺序。这个问题经常被提出因为人们并没有仔细考虑其背后的意义。

本文中，我们就来讨论如何通过"happened before"关系定义局部顺序，然后给出一个算法来计算所有事件的全部顺序（全序）。这个算法可以提供一个有效的实现分布式系统的机制。我们可以看到通过这个算法的简单应用可以解决同步问题。如果通过该算法获得的排序与用户感知的排序不同，则可能发生非预期的动作。这可以通过引入一个真实的物理时钟来避免。我们同样提出了一个简单的方法来同步这些时钟，并且给出容忍的时钟漂移上限。

## The Partial Ordering 

大多数人可能这样描述事件a在事件b之前发生，如果a发生的时间早于b。这就是物理意义上的早于的定义。但是，如果系统有规范说明，应该通过规范说明来定义，如果规范中说明了根据物理时钟，系统必须包括物理时钟，然而，即使系统包含了物理时钟，也可能没有保持与真实事件的时间同步而发生错误。所以，"happened before"关系我们不用物理时钟来定义。

我们首先要将系统定义的更加精确。我们假定系统是由一系列进程构成。每个进程中包含了事件的序列。依赖于具体场景，一个进程中某个子函数的的执行可能是一个事件，或者一条机器指令的执行是一个事件。我们假定这些进程内的事件构成了一个序列，在进程内a发生于b之前。换句话说，一个进程定义了一个序列事件的全序。看起来事情已经解决了，甚至可以扩展定义将进程分离为子进程，但是我们不这样做。

我们假定发送或者接收消息是一个事件。通过"$\rightarrow$"定义"happened before"

*定义*. '$\rightarrow$' 是系统中事件的最小关系，满足以下三个条件：

1. 如果$a$和$b$是同一个进程中的事件，而且$a$发生于$b$之前，那么$a \rightarrow b$
2. 如果$a$是一个进程中发送消息，$b$是另一个进程中接收消息，那么$a \rightarrow b$
3. 如果$a \rightarrow b$并且$b \rightarrow c$，那么$a \rightarrow c$

两个事件$a$和 $b$，如果$a \nrightarrow b$，而且$b \nrightarrow a$，就说$a$和$b$是并发的。

对于任意事件$a$，$a \nrightarrow a$。这意味着$\rightarrow$就可以表示系统中所有事件的局部顺序。

![image-20210409111648200](../static/pics/time_clock_the_orderingEventDistriSytem/image-20210409111648200.png)

上图对于理解这个定义

## Logical Clocks

## Ordering the Events Totally

## Anomalous Behavior

## Physical Clocks

## Conslusion

## Appendix 
略



## 参考文章：

1. https://lrita.github.io/2018/10/24/lamport-logical-clocks-vector-lock/
2. 狭义相对论与分布式系统中的时间  https://www.jianshu.com/p/0c79d650d13f
   1. 阐述了分布式系统中时间的本质，探索了分布式理论的本质
   2. 提出了Logical Clock算法，是后续Vector Clock，HLC(混合逻辑时钟，包含了logic clock, physical clock)等的基础
   3. 提出了Replicated State Machine的理念，是后续Paxos及其应用的基础
   4. 设计了无中心的分布式临界资源算法，是后续多种无中心分布式算法的鼻祖
   5. 设计了时间同步的雏形算法，后续NTP等的基础
3. 阿里数据库的HLC https://database.51cto.com/art/201911/606198.htm
4. https://lrita.github.io/2018/10/19/communication-model-in-distribution/
