+++
template = "blog/page.html"
date = "2021-10-20 17:39:41"
title = "【翻译】为什么协议栈在内核中实现"
[taxonomies]
tags = []

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

为什么TCP协议栈实现在Linux内核中

https://blog.cloudflare.com/why-we-use-the-linux-kernels-tcp-stack

CloudFlare提出工作经验在于大量的生产环境的机器，尝试在这个角度给出理解。

从一个更广泛的问题切入---操作系统运行的意义是什么？如果是计划运行一个应用程序，必须使用数百万行代码组成内核，似乎是负担。但是事实上，我们会运行某种类型的OS，有两个理由：

- 操作系统层提供了更容易使用的硬件抽象API，可以专注于写机器无关的代码
- 操作系统增加了时分系统，使得同一个时间可以运行多个应用，比如运行HTTP服务器和Bash session，在多个进程共享资源的能力是很重要的，纳入操作系统管理的资源可以被多个进程共享

### 用户空间的网络

对于协议栈来说没有什么不同，通过操作系统网络协议栈的通用性，我们可以获得运行多个网络应用的能力。如果将网卡硬件运行用户空间的网络协议栈专用于运行单个应用，这个能力就没了。通过一个进程获取网卡，你将失去与服务同时运行比如SSH会话的能力

听起来很疯狂，这正是大多数现成的用户空间网络协议栈提出的。通用术语是“完全内核旁路（full kernel by pass）”，这个想法是绕过内核，直接从用户空间进程使用网络硬件。

在Linux生态系统中，有一些可用技术，并非全部都是开源的：

- [PF_RING](http://www.ntop.org/products/packet-capture/pf_ring/)
- [Snabbswitch](https://github.com/snabbco/snabb)
- [DPDK](http://dpdk.org/)
- [Netmap](http://info.iet.unipi.it/~luigi/netmap/)

在之前的文章中已经写过 [kernelbypass](https://blog.cloudflare.com/kernel-bypass/)，所有的这些技术都需要将整个网卡移交给一个进程，换句话说：完全可以编写自己的网络协议栈，使其出色，专注于超级功能，并针对性进行性能优化。但是这会产生很大的成本---你将被限制为每个网卡最多运行一个进程。

关于虚拟化网卡（VFs），让我们专门提一下---这不起作用，在这篇文章中[the "Virtualization approach" paragraph](https://blog.cloudflare.com/kernel-bypass/#virtualizationapproach)提到过。

但是即使有些障碍，我还是不能忽略kernel bypass的好处，许多人确实运行自定义的网络协议栈，因为两个原因：

- 时延
- 性能（更低的CPU开销，更好的吞吐量）

延迟对于HFT（高频交易）人员非常重要，交易者可以负担起定制硬件和专有的网络协议栈。运行封闭的TCP堆栈会让其非常不舒服

### Kernel bypass at CloudFlare

就像已经说过的，CloudFlare使用旁路内核技术（kernel bypass）。我们属于第二个群体，关注性能。更具体地说，我们遭受IRQ风暴的影响。Linux网络协议栈有一个每秒处理多少包的限制。当达到限制后，所有的CPU变得忙，仅接受数据包。在这种情况下，要不丢弃数据包，要不应用程序卡死（CPU被占满）。一般我们不需要处理IRQ风暴，但是当受到L3层的DDos攻击时，确实会发生这种情况。这种攻击充斥着不属于有效连接的数据包，典型的欺骗数据包。

在受到攻击期间，我们被洪泛到每秒3M数据包量。Linux的iptables的通用规则可以处理1Mbps的包速很好，并且可以继续提供足够的CPU给应用使用。这个数字还可以通过调优有所提升。

当攻击的规模变大，Linux内核能力不足以应付时，我们必须能够处理这种情况，但是我们不采用前面提到的“full kernel bypass”，使用我们叫做"partial kernel bypass"。在内核拥有网络硬件的同时，允许我们处理仅通过一个“RX queue”旁路出来的数据包。我们在Solarflare NIC上使用Solarflare's EFVI API。为了支持Intel NIC，我们在[Netmap](https://blog.cloudflare.com/single-rx-queue-kernel-bypass-with-netmap/)中添加了部分旁路内核的功能。通过这种技术，可以将防御DDoS iptables卸载到非常快速的用户空间进程中，这种可以避免Linux内核处理攻击数据包，从而避免IRQ风暴。

### 完整的用户空间网络协议栈呢

我的同事经常问我：为什么我们不适用快速用户空间TCP协议栈运行带有Solarflare OpenOnLoad框架的Nginx？是的，那将很快，但是没有证据表明它会产生多大的实际差异。我们服务器上的CPU大部分都用于运行用户空间的Nginx，操作系统并没有使用多少。CPU主要用于常规的Nginx记录和Lua应用逻辑，而不是网络处理。我估计使用旁路可以节省大约5~10%的CPU，但是这目前并不值得

其次，为了Nginx使用旁路内核技术将是的我们常用的调试工具失效。Linux netstat统计将停止记录，tcpdump也将失效。

然后缓解DDoS的部分旁路协议栈也存在一些问题，我们重度依赖iptables，。自定义的TCP协议栈没有类似'hashlimits'和'ipsets'这种功能。

而且不仅是防火墙功能，Linux TCP协议栈还提供了一些RFC4821 sys.net.ipv4.tcp_mut_probing sysctl 的非平凡支持。这些支持对于党用户位于ICMP黑洞后面时非常有用。（[PMTU](https://blog.cloudflare.com/path-mtu-discovery-in-practice/)）

最后，每种TCP协议栈都有一些自己实现上的bug和quirks。我们已经整理了三种Linux协议栈中不明显的quirks：

- 垃圾收集器插入读取缓冲区（https://blog.cloudflare.com/the-story-of-one-latency-spike/）
- 当太多的监听socket时的问题
- 套接字可写意味着什么（https://blog.cloudflare.com/the-curious-case-of-slow-downloads/）

想象一下在封闭的源码或者年轻的TCP协议栈中调试问题。

## 结论

有两个主要话题：

首先，没有稳定开源的partial kernel bypass技术，我们希望Netmap能填补这个空白。其次，Linux内核TCP协议栈有许多重要特性和良好的debug能力。这个丰富的生态系统竞争将需要多年。

有很多理由用户空间协议栈不会成为主流，事实上，我只能想到有限应用需要旁路内核功能：

- 软件交换机和路由器。这里你希望直接处理网络硬件的原始数据包，绕过内核
- 专用负载均衡器。相似地，如果机器只需要处理数据包，绕过内核就很有用
- 部分旁路为了挑选出来的高吞吐/低延迟的应用。这是我们对于DDoS的调整。不幸的是，我不知道其他的适合此场景的其他的开源TCP协议栈

对于大多数用户Linux内核协议栈就是很好的选择。尽管没有重写TCP协议栈那么让人兴奋，we should focus on [understanding the Linux stack performance](https://blog.cloudflare.com/how-to-achieve-low-latency/) and fixing its problems.There are some [serious initiatives underway](http://lists.openwall.net/netdev/2016/01/15/51) to improve the performance of the good old Linux TCP stack.

@todo其中的链接看一下

