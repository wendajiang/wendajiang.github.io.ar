+++
template = "blog/page.html"
date = "2021-08-15 23:30:00"
title = "raft-extended"
[taxonomies]
tags=["awesomepaper", "translate"]

[extra]
mermaid = true
usemathjax = true
+++

![Raft-extend](https://wendajiang.github.io/pics/2021-03-02-raft-extended/Raft-extend.png)

# 摘要

Raft是管理复制日志的共识算法。它产生相当于Paxos的结果，并且和Paxos效率相当，但是结果与Paxos不同，使得Raft比Paxos更容易理解，提供了对于构建实用系统更好的基础。为了加强可理解性，Raft分离了共识算法的关键要素，比如领导者选举，日志复制和安全性，并且它强调强共识以减少必须考虑的状态数量。一项用户调研显示Raft对于学生来说比Paxos更容易学习。Raft还包括了一个更改集群成员的新机制，该机制用重叠的多数来保证安全性。

## 1.介绍

共识算法允许一组计算机作为一个整体工作，这个整体允许某些成员出现故障。因此，在构建大规模可用软件系统中它扮演了重要的角色。过去的十年中，Paxos主导了共识算法的讨论，大多数共识算法的实现都是基于Paxos或者受其影响，并且Paxos已经成为了向学生传授共识算法的工具。

不幸的是，Paxos相当难理解，尽管有多种尝试使其更加适用于声场系统，但是它的结构为了支持实用系统需要复杂的变化，结果就是，系统构建者和学生都不喜欢Paxos。

在亲自与Paxos争斗之后，我们着手寻找一种新的共识算法，该算法可以为构建实用系统和学习提供更好的基础。。我们的方法目标是易于理解：我们是否能够为实用系统定义共识算法以及它是否比Paxos更容易学习？此外，我们希望该算法能够促进系统构建者的开发直觉。重要的不仅是算法可以工作，而是为什么可以工作。

工作的结论就是名为Raft的算法。在设计Raft时，我们使用各种手段提升可理解性，包括解构（Raft分离了领导者选举，日志复制和安全性），减少状态空间（相对于Paxos，Raft降低了不确定性的程度，并降低了服务器之间的不一致方式）。

Raft在许多方面与现有的共识算法类似（最著名的Oki和Liskov的Viewstamped Replication），但是它也有一些新颖的特性：

- 强领导者：Raft使用了比其他共识算法更强力的领导方式。比如，Log entries只能从领导者流向其他节点。这简化了复制日志的管理，使其更容易理解
- 领导者选举：Raft使用随机定时器来选举领导者。这相对于其他共识算法心跳加入的一点机制，可以简单快速地解决冲突
- 成员变化：Raft更改集群中服务器组的机制使用了新的联合共识的方法，其中两种不同的配置在转换过程中会重叠。这允许集群在配置更改时可以继续正常用运行

我们认为Raft在教育目的和实施基础上优于Paxos和其他共识算法。它更加简单易懂；描述了足够构建实用系统的细节，同时有了几种开源实现，并被多家公司使用。其安全性已经得到保证和证明；效率足以与其他共识算法媲美。

本文的剩余部分介绍了复制状态机问题（第二节——，讨论了Paxos的优缺点（第三节），描述了我们对于可理解性的一般方法（第四节），介绍了Raft共识算法（5-8节），Raft的评估（第九节），讨论了相关工作（第10节）

## 2. 复制状态机

共识算法通常出现在复制状态机的环境中。通过这种方法，服务器集合上的状态机可以计算相同状态的副本，即使某些服务器宕机也可以继续运行。复制状态机用来解决分布式系统中的各种容错问题。例如，具有单个集群领导者的打醒系统，比如GFS，HDFS和RAMCloud，通常使用单独的复制状态机来管理领导者选举和保存配置信息以保证领导者崩溃时的可用性。复制状态机的例子还包括Chubby和ZooKeeper

![图一](https://wendajiang.github.io/pics/raft-extended/image-20200901151351972.png)

**复制状态机通常如上图使用复制日志来实现**。【译者注：每个独立的的服务个体实现了该状态机，通过Raft算法保证一组raft服务进程维持相同的状态机状态】每个服务器保存包含了一系列按序执行命令的日志。**每份日志包含了同序的相同命令**，所以每个状态机处理了相同的命令序列。因此状态机是确定性的，每个在相同的状态执行了相同的命令得到相同的输出。

保持复制日志一致是共识算法的工作。服务器上的共识模块接收来自客户端的命令加入到日志中。它负责与其他安装有共识模块的服务器通信保证日志以相同的顺序记录了相同的请求，即使有些服务器宕机。一旦命令被正确的复制，每个服务器的状态机以日志中的顺序处理命令，然后将输出返回客户端。结果就是，服务器表现为单个，高可用状态机。

生产系统的共识算法通常具有如下性质：

- 确保安全（不会返回一个不正确结果）在非拜占庭环境中，包括网络延迟、分区，数据包丢失、复制和重排序。
- 只要大多数服务器可以运行并且相互通信并与客户端通信，它们就可以正常运行。因此由5台服务器组成的典型集群可以容忍任何2个服务器的故障。假定服务器因停止而发生故障，它们可以稍后从稳定存储的状态机中恢复并重新加入集群
- 不依赖时间来保证日志的共识：错误的时钟和极端的消息延迟最坏情况下会导致可用性问题
- 通常情况下，只要集群的大多数服务器都响应了一次RPC，命令就可以完成，少数响应慢的服务器不影响整体系统的性能

## 3. Paxos的问题是什么

在过去的十年中，Leslie Lamport的Paxos协议几乎已经成为共识的代名词：这是课程中最常教授的协议，共识的大多数实现都是以他为起点。Paxos首先定义了一种能够在单个决定中达成一致的协议，比如单条复制日志。我们称它为"single-decree Paxos"（单判决Paxos）。Paxos接着合并多个这种协议完成一些列比如日志的决定（*multi-Paxos*）。Paxos同时保证安全性和活性，支持集群成员的改变。它的正确性已经得到了证明，并且通常情况下是高效的。

不幸的是，Paxos有两个重要缺陷。第一个是Paxos非常难以理解。[15]论文中的完整解释并不透明，很少人能成功理解它。结果就是，有数篇论文试图简单地解释Paxos。这些解释集中于*single-decree*子集，虽然这仍旧存在挑战性。在NSDI2012与会者的非正式调查中，我们发现即使是经验丰富的研究人员，也很少保持对Paxos的理解共识。我们自己也与Paxos争斗过，我们直到读了大量简单解释性的论文和设计自己的替代系统之后，这花了将近一年时间，才理解了这套协议。

我们假定Paxos的不透明性院子其single-decree子集作为基本元素的设计。Single-decree的Paxos是复杂精细的，导致整个协议分成了两个阶段，使其难以形成简单的直觉性解释而且不能单独来解释。因此，很难形成为什么single-decree协议能够工作的直观理解。multi-Paxos的组成引入了更多的复杂性和微妙性。我们相信能通过其他更为直接和简单的方式分解在多个决策阶段（比如一条日志代替一条entry）达到共识的问题。

第二个问题是Paxos不能给与构建实际系统良好的指导。原因之一是没有针对multi-Paxos的广泛的共识。Lamport的描述更多的是single-decree的Paxos，他只做了multi-Paxos实现方式的大致构想，没有太多细节。有几种具体化和优化Paxos的尝试，比如文献26,39,13，但是这些文献之间互相不同并且不同于Lamport的概述。比如Chubby这种系统实现了类Paxos算法，但是大部分细节没有公开发表。

另外，Paxos的结构不利于实现实际系统。这是single-decree分解的另一个结果。比如，单独选择一组日志条目然后融合为序列化的日志是有点好处的，这只增加了复杂性。围绕日志设计一个系统更加容易和高效，在该系统中以约束顺序添加日志条目即可。另一个问题是Paxos的核心使用对称的点对点方法（尽管它暗示了通过使用弱领导形式来优化性能）。在仅做出一个决定的问题中这是有道理的，但是几乎没有实际系统可以使用这种方法。如果需要做出一系列决策，首先选举出一个领导者更加简单快捷，随后领导者来协调决策。

结果就是，实际系统与Paxos没有什么相似之处。每种开始于Paxos的实现最终以不同的结构来呈现。这是费时而且易错的，Paxos的难以理解性加剧了这个问题。Paxos的形式化可能是证明该理论正确性的良好表达，但是实际的实现跟Paxos的证明完全不同，所以毫无实际价值。下面关于Chubby实现的论述非常经典：

> Paxos算法的描述与真实系统的需要有着巨大的差异...因此最终的系统将基于一个没有被证明的协议

由于这些问题，我们得出的结论是Paxos不能为系统构建或教育提供良好的基础。 考虑到共识在大型软件系统中的重要性，我们决定看看是否可以设计一种性能比Paxos好的替代共识算法。 Raft就是这个实验的结果。

## 4. 易理解性设计

我们设计Raft时，有几个目标：比如提供一个完整，易于构建系统的实际基础，以便减少开发人员的设计工作量；它必须在所有条件下是安全的，典型操作场景是可用的，大多数操作是高效的。但是最重要的目标是，也是最大的挑战 -- 易于理解。对于大多数人来说可以容易地理解该算法。另外必须能够建立该算法的直觉，以便系统构建者可以进行实际实现中不可避免的扩展。

在Raft设计中有很多要点，我们必须在多种方法中进行选择。 在这种情况下，我们根据可理解性评估了替代方案：解释每种替代方案有多难（例如，其状态空间有多复杂，是否有微妙的暗示？），以及对读者来说有多容易完全了解该方法及其含义？

我们意识到这种分析具有相当高的主观性；不过，我们还是用了两种通用技术。第一个是众所周知的问题分解方法：无论如何，我们分解问题更小使得可以解决，解释和理解。例如，我们将Raft分为了**领导者选举，日志复制，安全性和成员改变**几个问题。

我们的第二个方法是通过减少需要考虑的状态简化状态空间，使系统更加协调一致，并在可能的情况下消除不确定性。尤其是，日志不允许有holes，Raft限制了导致互相之间不一致的方式。尽管大多数情况下，我们试图降低不确定性，有些场景引入不确定性可以提升可理解性。尤其是，随机方法引入了不确定性，但是通过相似的处理方法处理所有的可能性减少了状态空间（"随便选一个，无所谓"）。我们使用随机化来简化Raft领导者选举算法。

## 5. Raft共识算法

Raft是一个管理第二节描述的复制日志的算法。图2简介的展示了该算法，图3列出了算法的关键属性。图中剩余的属性会在本节分开阐述

![image-20200902005835715](https://wendajiang.github.io/pics/raft-extended/raft-2.png)

![image-20200902005912684](https://wendajiang.github.io/pics/raft-extended/raft-3.png)

Raft 通过首先选举一个 leader，然后让这个 leader 完全管理 replicated log 的方式实现共识（consensus）。leader 从客户端接收 log entries，在其他服务器上 replicate log entries ，并告诉其他服务器何时可以安全地将 log entries 应用于它们的状态机。选择一个 leader 简化了管理 replicated log 的工作。比如，leader 可以决定自行决定如何处理新的 log entry，而不需要询问其他服务器，数据流的方向只有从 leader 到 follower。leader 可以挂掉或者与其他服务器断开网络连接，这时新的 leader 会被选出来。

基于 leader 的方法，Raft 将共识问题分解为三个独立的子问题

- **Leader election**：当现有的 leader 挂掉，必须选出一个新 leader
- **Log replication**：leader 必须接受来自客户端的 log entries，然后将它们复制到整个集群，强制其他服务器使用 leader 的覆盖
- **Safety**：安全属性的关键是图3中的 state machine safety 属性：如果任何服务器已经应用了某个 log entry 到状态机，不会有其他服务器在相同的 log index 执行不同的命令。5.4 小节描述 Raft 如何确保这个特性；**解决方案就是加了成为 leader 约束**

### 5.1 Raft basics

Raft 集群包含了一组服务器，5是一个典型的数据，可以容忍两台机器挂掉。在任何给定的事件内，服务器处于三种状态之一：*leader, follower, candidate*。所谓 normal operation 存在一个 leader ，其他服务器都是 follower。follower 是被动的：不会提出请求只是响应来自 leader 或者 candidate 的请求。leader 来处理所有来自客户端的请求（如果客户端向 follower 发出请求，follower 会将请求重定向到 leader）【译者注：所以TiKV中对这种读请求做了特殊处理，更好的降低leader处理请求的负载，属于case by case 的优化？】。第三种状态， candidate 处于参与选举为 leader 的状态。图4展示了三种状态以及转移条件

![image-20210815221414705](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210815221414705.png)

Raft 将时间切分成了任意长度的 *term*，如图5所示。Terms 使用连续整数表示。每个 Term 开始于一次选举，一个或者多个 candidates 试图成为 leader。如果一个 candidate 赢得了选举，这次 term 的剩余时间将作为 leader。有时候，选举不一定只有一个leader【译者注：典型的每个 candidate 同时为自己发起投票，这就出现了冲突】，这种场景，这次 term 就没有 leader， 就会马上进入下一次 term。Raft 保证一个给定的 term 中只有一个 leader。

![image-20210815221905157](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210815221905157.png)

不同的服务器可以在不同的时间观察到状态的转移，并且某些情况下，服务器可能无法观察到选举甚至整个 term。**Term 在 Raft 是逻辑时钟**，这可以使得 Raft 算法判断比如老的leader是不是过时的信息。每个服务器存储了*当前的 termid*，随着时间这个值只能增加。服务器通信的时候，*current termid*的信息会被交换；如果一个服务的 term 数字小于另一个，就会更新成更大的那个。如果 candidate 或者 leader 发现它们的 term 的已经落后，就会马上转移到 follower 的状态。如果一个服务器接受了过时 term 的请求，就会拒绝这次请求

Raft 的服务器使用 RPC 通信【译者注：所以 TiKV 中的 gRPC 是更基础的组件？raft-rs 中的 mpsc 是一个模拟】，基本的一致性算法只需要两个 RPC **RequestVote** 和 **AppendEntries**。RequestVote 在 candidate 选举时使用，AppendEntries 在 leader 复制自己的 log entry 到整个集群时使用，同时内容为空的时候也作为心跳使用。后面还会再加一个 RPC 用来在服务器之间转移 snapshots。服务器没有收到响应会重试，并且可以并发 RPC 提高性能。

### 5.2 Leader 选举

**Raft 使用心跳机制触发 leader 选举**，当服务器启动时，都是 follower。服务器会在收到 leader 或者 candidate 的有效 RPC 时保持 follower 的状态。leader 会周期性发出心跳到所有 follower 来保证自己的 “权威”。如果follower 超过一个周期没有收到通信，称为 **election timeout**，意味着没有可用 leader，并且开始选出新 leader

为了开始选举，follower 增加 term 的数字，然后转化为 candidate 的状态，然后为自己发出投票。candidate 会一直保持这个状态直到：（a）它自己赢得选举 （b）另一个服务器赢得选举 （c）一个周期时间过去没有任何服务器称为 leader。下面分开讨论这三种情况

candidate 在一个 term 内赢得了半数以上的投票。每个服务器最多投一张选票，先来发起的先得（**注意后面会为了 saftey 加上更强的约束**）。多数的规则确保了最多只有一个 candidate 能赢得选举。一旦 candidate 赢得一次选举，就成为 leader。然后开始向所有其他的服务器发送心跳请求【译者注：这会重置其他服务器的定时器，避免新的选举开始】

当等待投票时，candidate 可能会收到 AppendEntries 的 RPC。如果 leader 的 term 比 candidate 当前的不小于，那么 candidate 会意识到 leader 是合法的，自己就会退回到 follower 的状态。如果 RPC 中的 term 比自己的小，candidate 会拒绝这次 RPC，继续保持 candidate 状态

第三种可能的情况就是没有 candidate 赢得选举，如同前面提到的如果很多 follower 同时成为 candidate，就会出现冲突。这种情况发生时，每个 candidate 就会增加自己的 term，开始新一轮选举，但是为了避免无限循环的状况发生，这里引入了随机性来规避

**Raft 使用随机 election timeout来确保投票冲突很少发生，并且如果发生可以快速恢复**。固定间隔中随机一个时间（150ms ~ 300ms）【译者注：后面有提到这个时间选择的策略，以及三个时间之间的关系（rpc的通信时间 << election timeout << 服务器宕机间隔），并通过证反形式证明 Raft 的正确性】。通常这样做之后只有一台服务器发起投票。

【译者注：原文还有一段最开始并没有使用随机退避的策略，而是引入优先级系统来解决冲突，但是这使得系统复杂性变高，与容易理解的初衷相悖，这里是一个简要说明，翻译略去，有兴趣可以翻阅原文】

### 5.3 Log 复制

一旦 leader 被选出来，开始服务客户端。每个客户端请求包含了状态机需要执行的命令。leader 追加命令到自己的 log 中成为 new log entry。然后通过 AppendEnteries RPC 并行扩散到集群中的其他服务器。当 entry 已经被安全复制之后，leader 将 entry 应用到状态机，然后将结果返回客户端。如果 follower 宕机或者运行较慢，或者网络丢包，leader 会不断重试，直到所有 follower 保存了所有 log entries

![image-20210815225731424](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210815225731424.png)

log 如图6这样组织，**每个 log entry 包含一条状态机的命令，和一个 term number**。term number 被用来检查 log 之间的矛盾确保图3中的一些性质。**每个 log entry 还有一个整数 index 表示在 log 中的位置**

leader 决定是时候将 log entry 应用到状态机时，这样的 entry 被称为 ***commited***。Raft 保证提交的 entry 是持久化的，并且被所有的状态机执行过了。一条 log entry 在 leader 获得多数 follower 复制响应后被提交（图6中是 entry 7）。这意味着之前的 log entry 都已经被提交了，包括上一任 leader 提交的。5.4 节中讨论了一些微妙的场景，当在 leader 发生变化之后应用这条规则，也展现了 commitent 的定义是安全的。 leader 追踪着已知最新的被提交的 log entry，这会在以后的 AppendEntries RPC 中包含，所以其他服务器也会知道。一旦 follower 学习到这个信息，就会应用到自己的状态机。

我们设计 Raft log 机制来位置不同服务器之间 log 的高度一致。这不仅简化系统行为，更重要的是保证安全【译者注：安全的定义是什么？】。Raft 建立了以下属性，一起确保了 Log Matching Property（图3中）：

- 如果不同的 log 【译者注：不同服务器/服务进程上的log】的两条 entry有相同的 index 和 term，命令也是一样的
- 如果不同的 log 的两条 entry 有相同的 index 和 term，则前面所有的 log entry都是相同的

第一条性质基于这样一个事实，leader 在给定的 term 的给定 log index 最多创建一条 log entry，然后 log entry 不会更改位置。第二条性质由 AppendEntries 进行一致性检查得到保证。当发送 AppendEntries RPC， leader 将 entry 的 index 和 term 信息带上。如果 follower 没有在相同的 term 和 index 找到这条 entry，就会拒绝新的 entry。一致性检查的步骤为：空的log 状态机满足 Log Matching Property， 一致性检查维护了 Log Matching Property。结果就是，**leader 的 AppendEntries 返回成功，表示follower 的log在这条 log entry 之前跟自己是保持一致的。**

![image-20210816105214650](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210816105214650.png)

整个正常操作期间，leader 的 log 和 follower 的保持一致，所以 AppendEntries 一致性检查一般不会失败。但是如果 leader 宕机就会丢失 log 一致性（老 leader 可能没有将自己的全部 entry 复制出去）。这种不一致性会导致 leader 和 follower 的一系列问题。图7表明了 follower 的 log 可能与新 leader 的不同。follower 可能会丢失一些 leader 的信息，也可能有一些 log leader 不知道，或者同时存在这两种情况。**miss 或者 extraneous entry 可能横跨多个 term**

在 Raft 中，leader 通过强制 follower 使用自己的来处理不一致问题。这意味着，follower 与 leader 不一致的 log 要被 leader 的覆盖。5.4 节会说明，加上一个限制之后这是安全的

为了使 follower 的 log 与自己的保持一致，leader 必须找到两个 log 的最近交汇点，然后将自己之后的发送给 follower 用来覆盖。通过 AppendEntries 的一致性检查来执行这些操作。 leader 建立起每个 follower 的 *nextIndex* 的认识，这表示 leader 发送给 follower 的下一个 index。当 leader 初始化时，nextIndex 就是自己 log 的最后一个 index（图7中是11）。如果一个 follower 的 log 与 leader 不一致，AppendEntries RPC 的一致性检查会失败，然后 leader 会将 nextIndex 减 1然后重新发起请求，直到 leader 和 follower 的 nextIndex 匹配上，然后 AppendEntries 成功，会移除 follower 该匹配点之后的所有 log ，然后使用 leader 的来覆盖。

如果可以，这里多次 RPC 回退 index 的方式可以优化，比如 follower 可以直接回退到 term 的最开始位置。总之就是加速匹配过程。但是实际上我们怀疑这个优化是不是必要，因为实际情况中这种场景应该很小。

通过这种机制，leader 不需要执行任何特殊操作就可以保持与 follower 的一致性。leader 也不会覆盖自己或者删除自己（图3中的 Leader Append-Only Property）

log 复制机制表现出第2节中描述的理想共识属性：Raft 可以接受，复制，应用新的 log entry；正常情况下，log 复制通过一次 RPC 时间就可以达成；一个慢服务器不会影响性能

### 5.4 Safety

前面的小结描述了 Raft 如何选举 leader ，如何复制 log entry。但**是都没有提到状态机是否按照相同的顺序执行相同的命令。**比如，当 leader 提交几条 log entry 时，一个 follower 不可用了，然后他自己被选举成了 leader， 然后使用新的 entry 覆盖了本该执行的命令，这时不同机器的状态机就执行了不同的命令

**本节通过加入哪些服务器可以参与选举leader的约束来完成 Raft 算法**。这个约束确保 leader 在给定的 term 包含了所有被提交的 log entry（图3中的 Leader Completeness Property）。给出选举约束之后，提交规则更清晰了。最后，我们给出了 Leader Completeness Property 的证明。

#### 5.4.1 选举约束

任何基于 leader 的共识算法中，leader 必须存储了所有被提交的 log entry。有些共识算法没有这个要求，但是这些算法存储了额外的信息，确保新 leader 可以补齐或者丢弃一些 log entry。不幸的是，这会需要算法引入额外的机制和复杂度。Raft 使用更简单的方法来保证所有被提交的 entry 都被 leader 知道，而不想引入额外的转换机制。这意味着 log entry 只需要保证一个流动方向，就是从 leader 到 follower，leader 不需要覆盖写已有的 log

Raft 在投票过程中拒绝没有包含所有被提交 log entry 的票来实现这一点。candidate 必须联系集群中的多数服务器来获得投片，意味着提交的 entry 至少存在与集群中的一台机器上。如果 candidate 的 log 在多数机器上是 up-to-date （**Raft 对比两个 log 的最新 log entry 的 index 和 term 来定义 up-to-date，如果最新 log-entry 有不同的 term，term 大的更加 up-to-date， 相同 term，更大的 index 更加 up-to-date）的，表示持有所有被提交的 entry。RequestVote RPC 这样实现：RPC 中包含 candidate log 的信息，投票者对比这些信息与自己的，如果自己的更 up-to-date 就拒绝这个投票**

#### 5.4.2 在先前的 term 提交 entry 【译者注：这里实际没太看懂，需要查看博士的原论文。】

5.3节提到过，leader 知道当前 term 已经被提交 entry 已经被多数服务器接受。如果 leader 在提交这条 entry 之前宕机，新的 leader 会试图完成这次复制。但是，leader 不能直接得出结论如果一条 entry 是上一个 term 提交的，图3.7【译者注：这里从 Phd 的原论文中截图，比论文中的画法更容易理解】表明了这个场景，老的 log entry 被存储在了多数服务器上，但是可能被以后的 leader 覆盖。

![image-20210816154629358](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210816154629358.png)

这个问题的避免，Raft 规定计算复制时，绝对不提交先前 term 的 entry。只有 leader 当前 term 的 log entry 可以被提交，一旦当前 term 的 entry提交，所有先前的 log entry 不能再被修改。有些场景可能 leader 可以安全得出哪些老的 log entry 被提交的结论，但是 Raft 使用保守的策略。

Raft 接受这种提交规则的复杂性，因为当 leader 从上一个 term 复制 log entry， 会保存他们原始的 term 数字。在其他共识算法中，如果一个新 leader 复制先前 term 的 log entry，必须使用新的 term 来做。Raft 使其变得简单，因为在 log 保持相同的 term。此外 Raft 的新 leader 会比其他共识算法发送更少的先前 term 的 log entry（其他算法发送冗余 log entry 必须重新计算它们）

#### 5.4.3 Safety 论证（略）

### 5.5 Follower 和 candidate 宕机

到此为止，我们专注于 leader 失败的情况。Follower 或者 candidate 宕机的处理比 leader 更容易，而且处理方式相同。如果 follower 或者 candidate 宕机，将要到来的 RequestVote ， AppendEntries RPC 会失败。Raft 直接无限重试；如果宕机的服务器重启了，RPC 重试成功。如果服务器在接受了请求但是没有发出响应之间宕机，就会收到重复请求。Raft RPC 是可重入的，所以这没有什么问题。

### 5.6 时间和可用性

Raft 安全的一个要求是不依赖时间：系统不能因为某些事件发生的快些或者慢些就返回错误结果。但是，可用性（系统能够连续提供服务的能力）不可避免要依赖时间。比如，如果消息交换消耗比服务器宕机间隔还长，candidate 不能赢得选举，没有稳定 leader ，整个机制就挂了

Leader 选举是 Raft 中时间起着至关重要的一个方面。Raft 需要满足这个时间条件才能平稳运行：

$$broadcastTime \ll electionTimeout \ll MTBF(average time between failures for a single server)$$

前面的不等式中 $broadcastTime$ 表示并行 RPC 到集群中每个server的平均时间。$electionTimeout$ 前面描述过，$MTBF$ 表示单台server 的宕机间隔【译者注：一般是月计】。

前一个不等号保证了 leader 可以成功发送心跳消息，并且不会使集群发生分裂；第二个不等号是系统平稳运行保证。

$broadcastTime$ 和 $MTBF$ 是底层问题，我们无法控制，但是 $electionTimeout$ 是我们可选的。Raft 一般用于持久化存储的场景，所以可以假定 $broardcastTime$ 在 0.5ms 到 20ms 之间，取决于存储使用的技术。这么看，$electionTimeout$ 应该在 10ms 和 500ms 之间。

## 6. 集群成员变化

【译者注：TiKV 2019-10 才支持论文中提到的 joint consensus，最开始只有一次变动一个成员，所谓的 one point chang at a time】

之前所有的讨论都是一个集群的配置是固定的。在实际系统中，不可避免会更改配置，比如替换机器。最简单的就是停机，然后更改配置，然后重启集群。这回导致集群有一个不可用时间段。并且如果有操作失误，不可用时间段会更长。为了避免这个问题，决定将配置变更合并到 Raft 共识算法中

为了使配置变更也安全，在配置迁移过程中不能有点的变化，这可能导致同一个 term 被选举出两个 leader。不幸的是，任何直接切换机器配置的方法都不能避免这个问题。不能直接切换，那么就需要搞个两阶段。直接切换会引起分裂问题，如图10所示

![image-20210816124806914](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210816124806914.png)

为了保证安全性，**配置变更是一个两阶段方法**。有多种手段实现两阶段方法。比如有些系统使用第一个阶段来关闭老的配置，这时候不能处理客户端请求；然后第二阶段切换到新配置。【译者注：这只是将上面停机该配置自动化处理了。还是存在不能提供服务的问题】。Raft 中集群首先切换到联合配置，称为 ***join consensus***，一旦 joint sonsensus 被提交，系统就可以继续切换到新配置。Joint consunsus 既包含了就配置，也包含了新配置：

- log entry 被复制到新老配置中所有服务器中
- 任何一台服务器可以作为 leader
- **选举和 entry 提交需要旧配置和新配置中不同的多数才行**

joint consensus 允许单独的服务器在不同时间在配置之间转移，而且不违反安全性。同时，joint consensus 还支持不停机服务客户端，在配置更换期间

![image-20210816002558380](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210816002558380.png)

**集群配置的通信和存储在复制 log 中是特殊的 entry**；图11表明了配置更新过程。当leader 收到 切换配置的请求（从 $C_{old}到 C_{new}$），存储 joint consensus 的配置 $C_{old,new}$ 为一条 log entry，然后复制到整个集群。一旦集群都加了这条 log entry，使用最新的配置在将来的决策中。这意味着 leader 将使用 $C_{old,new}$ 的规则来决定 log entry 被提交。如果 leader 宕机，新 leader 是从$C_{old}$或者 $C_{old,new}$ 中选择，取决于 candidate 是否已经收到 $C_{old,new}$。这时，$C_{new}$不能单方面做出决策。

一旦 $C_{old,new}$ 被提交，就可以继续下一个阶段，leader 可以创建一个 $C_{new}$ 的 log entry，然后扩散到集群。当 $C_{new}$ 被提交时，$C_{old}$ 就失效了，不再新配置的机器就可以关机了。图11中表示，没有任何时间 $C_{old}$ 和 $C_{new}$ 同时分别做出决策，这保证了安全。

重新配置也提出了三个问题：

1. 新服务器可能没有存储 log 。如果新加入集群，需要一定时间初始化，这个时间内不能提交新的 log entry。为了避免可用性间隔，Raft 引入了一个额外的阶段在更改配置之前，加入集群的服务器没有投票权（leader 会复制 log 给它们，但是在多数选择时不被考虑）。一旦新服务器已经跟上了其他服务器，配置变更过程才会被触发
2. 集群 leader 可能不在新配置中。这种情况，leader 一旦要提交 log entry 在 $C_{new}$ 就回退到 follower 状态。这表示当 leader 管理本身不在其中的集群时，有一定时间，复制 log entry 但是不能在多数计算时，计算自己。当 $C_{new}$ 生效时，会发生领导者转换。在这之前，可能有来自 $C_{old}$ 的机器称为 leader
3. 被新配置移除的机器可能会破坏集群。这些机器不能收到心跳，所以会 timeout然后开始新选举。然后使用新的 term 发起 RequestVotes RPC，会造成当前的 leader 回退到 follower 的状态，新leader被选举，但是被移除的机器会不断超时发起新一轮选举。为了避免这个问题，服务器会丢弃 RequestVotes 请求，如果服务器收到请求在来自当前 leader 告知的最小 election timeout 之内。这不会影响正常选举，每个服务器会等待至少 minimum election timeout 在开始下一次选举之前。

## 7. Log 压缩

Raft 的 log 随着不断地 normal operation【译者注：normal operation 指的是正常请求，confchange 应该不属于 normal operation】，实际系统中，这个大小是无界的。随着 log 的增长，占用更多的空间，需要更多的时间重放。如果没有机制来丢弃 log 中累积的过时信息，就会造成可用性问题。

Snapshotting 是最简单的压缩方法。当前整个系统的状态被写入到硬盘的 snapshot 文件中，然后这个时间之前的 log 都可以废弃。Chubby 和 Zookeeper 中使用了 snapshotting，本节接下来就会阐述 Raft 中如何使用 snapshot

增量压缩，比如 log clean 和 LSM-tree，也是可能的。这些操作一次作用于部分数据，所以可以更快的压缩。首先选择数据的 region，然后重写活动的数据，然后释放 region。这需要额外的机制，并且相比 snapshotting 更加复杂。虽然 log clean 需要修改 Raft 算法，但是状态机可以实现 LSM-tree 使用与 snapshotting 相同的接口

![image-20210815234203909](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210815234203909.png)

图12 展示了 Raft 中 snapshotting 的基本思想，每个服务器独立 snapshot，覆盖已经提交的 log。包括：状态机将当前状态写入 snapshot。Raft 好包括了一部分 snapshot 的 metadata ：snapshot 中的 *last included index*（最新的状态机提交的 index），*last included term*是这个 entry 的term。要存这些信息是为了支持 AppendEntries 连贯性检查在 snapshot 之后的第一条新 log entry。为了支持第6节描述的成员变更，snapshot 还要包含最新 log 的配置信息。一旦服务器完成 snapshot，可以删除 *last included entry*之前的所有 log 和之前的所有 snapshot

尽管服务器可以独立的 snapshot，leader 也需要偶尔发送 snapshot 给 follower。当 leader 丢弃了 next log entry 时，需要通知 follower。幸运的是，通常不会发生这种场景，一般 follower 追随 leader 很好。但是一个非常慢的 follower 或者新加入集群的服务器没有最新信息，需要 leader 发送 snapshot 。

![image-20210816000824514](https://wendajiang.github.io/pics/2021-03-02-raft-extended/image-20210816000824514.png)

leader 使用 RPC InstallSnapshot 来向 follower 发送 snapshot，图13描述了这个 RPC。当 follower 收到一个 snapshot，必须决定如何处理已有的存在的 log entry。通常 snapshot 包含的就是全部最新信息。这时，follower 丢弃全部自己的 log 全部由 snapshot 替代。如果 follower 收到的 snapshot 是自己 log 的前缀（错误？），快照覆盖的部分使用 snapshot 的信息，后面的自己要保留

snapshot 方法从 Raft 的强 leader 原则中分离出来，因为 follower 可以不需要 leader 的信息自行 snapshot。所以，分离也是合理的，同时有一个 leader 帮助在达成共识时避免冲突，在 snapshot 时已经达成了共识，所以没有决策上的冲突。数据流还是从 leader 到 follower ，只是 follower 可以自行组织自己的数据。

【译者注：这里如果使用基于 leader 的 snapshot，将浪费巨大的网络带宽，并且 snaphot 会很慢。而且，leader 实现更复杂，比如， leader 需要并行处理 snapshot 到 follower 以及 new log entry 到 follower】

这里 snapshot 有两个主要问题：

1. 服务器决定什么时候 snapshot。如果过于频繁 snapshot，浪费硬盘带宽和功耗，如果频率太低，会撑爆存储，而且会增加重放 log 的时间。一个简单的策略就是当 log 到达一个固定大小的时候进行 snapshot。如果此大小设置明显大于 snapshot 的预期大小，则 snapshot 的磁盘带宽开销将很小
2. 第二个性能问题就是 snapshot 的写入会消耗时间，我们不能延迟正常操作。解决方案是使用 copy-on-write 技术，所以新的 log entry 可以被接受而不会影响 snapshot 的写入。操作系统的 copy-on-write 支持可以创造整个状态机的的 in-memory snapshot

## 8. 客户端协作

本节描述客户端如何与 Raft 交互，包括客户端如何与集群 leader 交互和 Raft 如何保证线性化语义。这些问题适用于所有共识系统，Raft 的解决方案是类似的

Raft 的客户端发送所有请求到 leader，当客户端启动时，随机连接一个服务器。如果客户端第一次选择不是 leader，服务器会拒绝这次请求并返回 leader 的地址。如果 leader 宕机，客户端会超时，然后随机重试一台服务器。

Raft 的目标是实现线性化语义（每个操作连续执行，并且执行一次）。但是 **Raft 可能会执行一条命令多次**：比如，如果 leader 在提交了 log entry 但是返回客户端响应之间宕机，客户端会重试这条命令，造成一条命令执行多次。**解决方法就是客户端在每条命令中增加一个唯一的序列号，然后状态机跟踪每个客户端执行的最新的序列号。如果收到过时的序列号，不会执行这次请求中的命令**

只读操作不写入 log，但是，如果**没有额外信息，这会冒着返回过时数据的风险**，因为响应请求的 leader 可能已经被新的 leader 取代。不能返回过时数据，所以 Raft 采用两个措施来避免这个问题（a）leader 必须有哪条 entry 是最新被提交的信息。Leader Completeness Property 保证了 leader 有所有的提交 entry，但是term的开始，可能不知道，**所以每次 term 开始马上提交一次 entry，Raft 会使 leader 在新的 term 提交一个空的 entry 来保证这一点**（b）leader 必须在处理只读请求时检查是否是过时请求（如果更新的 leader 被选举出来，这次请求就是过时的），Raft 在处理请求前与集群交换心跳信息来处理这个问题。或者，**leader 可以通过心跳机制来提供租约，但是这对系统的时间要求比较高（系统不能存在严重的时间不一致）**

## 9，10，11，12 

为实现的代码，实验的性能数据与对比

相关工作

总结结论

致谢