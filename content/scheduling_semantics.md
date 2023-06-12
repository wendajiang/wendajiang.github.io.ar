---
title: Scheduling semantics
description: ''
template: blog/page.html
date: 2023-04-11 15:02:54
updated: 2023-04-11 15:02:54
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["ieee1800"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'event-based simulation scheduling semantics'

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

# 4.1 General
- event-based simulation scheduling semantics
- system verilog's stratified event scheduling algorithm
- determinism and nondeterminism of event ordering
- possible sources of race conditions
- PLI callback control points

# 4.2 execution of a hardware model and its verification environment
The elements that make up the System Verilog language can be used to describe the behavior, at varying levels of abstraction, of electronic hardware. System Verilog is a parallel programming language. The execution of certain language constructs is defined by parallel execution of blocks or processes. It is important to understand what execution order is guaranteed to the user and what execution order is indeterminate.

# 4.3 Event Simulation
SV 语言是离散事件执行模型。根据定义，各个厂商实现的模拟器用户侧行为一致即可，具体实现使用什么算法是自由的。

SV 描述了相连接的执行线程或者 process。process 就是执行对象，有状态，将输入与输出关联起来。process 是并发调度元素，比如 `initial` 块。process 包括但不限于 `initial, always, always_comb, always_latch, always_ff` 块，primitives, continuous assignments; asynchonous tasks; procedural assignment statements.

net 或者 variable 状态的改变都被认为是 update event。

process 对于 update event 是敏感的。当 update event 发出，所有相关 process 以一定的顺序 evaluation。 evaluation of process 也是 event，*evaluation event*.

evaluation event 也包括 PLI callback

为了完整支持清晰可预测的交互，一个 time slot 被分为多个区域从而 event 可以被调度为特定顺序执行。这样也可以 properties 和 checkers 让 dut 在一个稳定状态采样。property expression 可以安全计算，tb 也可以没有延时执行，所有这些在可预测的行为中。这个机制也可以支持设计中的非零延时，clock propagation，以及 cyclu-accurate descriptions 的响应。

# 4.4 Stratified event scheduler
遵守标准的仿真器应该构建随时可动态调度，执行，删除的一系列数据结构。数据结构通常使用时间排序的链表实现，这种数据结构也容易实现二次划分。

第一次划分是根据时间。每个事件有且仅有一个仿真执行时间。所有可调度的 event 在特定时间都处于一个 time slot。仿真将本 time slot 的 event 执行或者清理完毕后才移动到下一个 time slot。这个过程保证了仿真器时间上不会回退。

一个 time slot 分为，**划分的目的是在 design 和 testbench code 之间提供可预测的交互**

- preponed
- pre-active
- active
- inactive
- pre-NBA
- NBA
- post-NBA
- pre-Observed
- Observed
- post-Observed
- reactive
- re-inactive
- pre-re-NBA
- re-NBA
- post-re-NBA
- pre-postponed
- postponed

可以分为 active region set 和 reactive region set。也可以分为 simulation region 和 PLI region。

PS. *NBA (nonblocking assignment update)*

![image-20230411163845834](https://wendajiang.github.io/pics/scheduling_semantics/image-20230411163845834.png)

# 4.5 SystemVerilog simulation reference algorithm
```cpp
execute_simulation {
  T = 0;
  initialize the values of all nets and variables;
  schedule all initialization events into time zero slot;
  while (some time slot is nonempty) {
    move to the first nonempty time slot and set T;
    execute_time_slot(T);
  }
}

execute_time_slot {
  execute_region(Preponed);
  execute_region(Pre-Active);
  while(any region in [Active ... Pre-Postponed] is nonempty) {
    while(any region in [Active ... Post-Observed] is nonempty) {
      execute_region(Active);
      R = first nonempty region in [Active ... Post-Observed];
      if (R is nonempty) {
        move events in R to the Active region;
      }
    } 
    while(any resion in [Reactive ... Post-Re-NBA] is nonempty) {
      execute_region(Reactive);
      R = first nonempty region in [Reactive ... Post-Re-NBA];
      if (R is nonempty) {
        move events in R to the Reactive region;
      }
    }
    if (all regions in [Active ... Post-Re-NBA] are empty) {
      execute_region(Pre-PostPoned);
    }
    execute_region(PostPoned);
  }
}

execute_region {
  while(region is nonempty) {
    E = any event from region;
    remove E from the region;
    if (E is an update event) {
      update the modified object;
      schedule evaluation event for any process sensitive to the object;
    } else { // E is an evaluation event
      evaluate the process associated with the event and possibly schedule further events for execution;
    }
  }
}
```

# 4.6 Determinism
标准保证了这样的顺序：
1. 在 begin-end 块中的 stmt 应该按序执行。begin-end 块中的 stmt 可以按需挂起（支持其他 process）
2. NBA 应该按序执行

# 4.7 Nondeterminism
不确定性来源
1. active events 可以从 active 或者 reactive event 中取出执行
2. 没有时间控制块不必作为一个 event 执行。在 procedural stmt 中执行任何时候仿真器可以允许 process 交错执行，这样就是不确定的并且不受用户控制

# 4.8 Race conditions
```verilog
assign p = q;
initial begin
  q = 1;
  #1 q = 0;
  $display(p);
end
```
输出1 0 都对。

# 4.9 Scheduling implication of assignments

# 4.10 PLI callback control points
两种 PLI cb：特定事件出现时执行；注册为 one-shot evaluation event


| Callback           | Event Region        |
| ------------------ | ------------------- |
| cbAfterDelay       | Pre-Active          |
| cbNextSimTime      | Pre-Active          |
| cbReadWriteSynch   | Pre-NBA or Post-NBA |
| cbAtStartOfSimTime | Pre-Active          |
| cbNBASynch         | Pre-NBA             |
| CbAtEndOfSimTime   | Pre-Postponed       |
| cbReadOnlySynch    | PostPoned           |

