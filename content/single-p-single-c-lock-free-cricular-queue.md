+++
title = "Lock-Free Single-Producer - Single Consumer Circular Queue[翻译]"
template = "blog/page.html"
date = "2021-02-07 10:36:06"
updated = "2023-02-28 10:36:06"
[taxonomies]
tags = ["translate", "queue", "lock-free"]

+++

[原文链接](https://www.codeproject.com/articles/43510/lock-free-single-producer-single-consumer-circular)

# Part I

## Introduction

对于时间和内存敏感的系统，wait-free 和 lock-free 循环队列是一种有用的技术。队列的 wait-free 特性使每个操作以固定次步骤完成。 lock-free 特性使得单生产线程和单消费线程通信不需要锁。

Wait-free 和  lock-free 的特性可以应用于广泛的领域，比如实时系统的中断和信号处理或者其他时间敏感软件。

## Background: Circular FIFO Queue

这里的循环 FIFO 队列只能适用于最多两个线程的通信，这个场景一般是单生产者单消费者问题。

**问题描述**：高优先级任务不能延迟处理，并且要求处理数据是按照开始的FIFO顺序产生结果。比如，中断/信号处理程序，GUI事件，数据记录。

**解决方案**：使用FIFO队列将任务发送给另一个线程（消费者），当任务从生产者传递到消费者时，使用FIFO队列可以将使得异步委派和处理提供可预测的行为。
生产者可以在队列没有满时，将任务加入到队列中，消费者可以从队列中将任务取出。对队列执行添加和取出操作时，不需要等待。如果消费者处理完一个任务，可以继续执行其他任务，当队列已满，生产者也不会阻塞。
显然队列满对于生产者不是什么好的事情，所以必须仔细考虑其最大尺寸以及满队列的不良影响。涉及此问题的方法，比如覆盖旧数据、多个优先级队列或其他处理方式。

## Background: Article Rationale

略

## Disclaimer

【译者注：免责声明以及一些类似 lock-free guarante 的内容】略

## Memory model: sequential or relaxed/acquire/release?

本文提供了两种内存模型的队列：`memory_order_seq_cst`默认的，还有结合了`memory_order_acquire, memory_order_release, memory_order_relaxed`三种的，然后讨论了默认的不仅简单而且在x86架构的服务器上还很快

## Using the code

牢记：单生产者，单消费者

```cpp
CicularFifo<Message, 128> queue;

Message m = ...
if (false == queue.push(m)) { /*false equals full queue */}

Message m;
if (false == queue.pop(m)) { /*false equals empty queue */}
```



## Code Explained: Circularfifo‹Type, Size›

简化的代码如下：

```cpp
template‹typename Element, size_t Size› 
class CircularFifo{
public:
  enum { Capacity = Size+1 };

  CircularFifo() : _tail(0), _head(0){}   
  virtual ~CircularFifo() {}

  bool push(const Element& item); 
  bool pop(Element& item);

  bool wasEmpty() const;
  bool wasFull() const;
  bool isLockFree() const;

private:
  size_t increment(size_t idx) const; 

  std::atomic‹size_t›  _tail;  
  Element              _array[Capacity];
  std::atomic‹size_t›  _head; 
};
```

## How it works

第一个版本使用默认顺序原子操作，顺序模型使得程序容易理解并轻松推断出原子操作在各个线程中的顺序。在硬件强内存序结构的x86和x64架构上，使用顺序原子操作相比后面提到的弱内存序是有开销的

在弱内存架构的处理器上，这个差异会更明显。

### Empty

队列为空，头尾相等

```cpp
bool wasEmpty() const 
{
  return (_head.load()) == (_tail.load());
}
```



### Producer adds items

```cpp
/* Producer only: updates tail index after setting the element in place */
bool push(Element& item_)
{	
  auto current_tail = _tail.load();            
  auto next_tail = increment(current_tail);    
  if(next_tail != _head.load())                         
  {
    _array[current_tail] = item;               
    _tail.store(next_tail);                    
    return true;
  }
  
  return false;  // full queue
}
```

### Consumer retrieves item

```cpp
/* Consumer only: updates head index after retrieving the element */
bool pop(Element& item)
{
  const auto current_head = _head.load();  
  if(current_head == _tail.load())  
    return false;   // empty queue

  item = _array[current_head]; 
  _head.store(increment(current_head)); 
  return true;
}
```

### Full

```cpp
bool wasFull() const
{
  const auto next_tail = increment(_tail.load());
  return (next_tail == _head.load());
}

size_t increment(size_t idx) const
{
  return (idx + 1) % Capacity;
}
```

## Thread Safe Use and Implementation

略

# Part II

## Making It Work

在元素被读取和写入（pop或者push）后马上更新头或者尾index是必要的。因此，任何对于头尾index的更新必须保证正确，所以这需要是原子操作并且不能被重排序。

## Atomic operations and the different Memory Models

Below is described a subset of the functionality that you can get from the different C++11 memory models and atomic operations. It is in no way comprehensive but contains the necessary pieces to explain both types of the C++11 empowered wait-free, lock-free circular queue.

There are **three different memory-ordering models** in C++11 and with them six ordering options.

1. Sequentially consistent model
   - `memory_order_seq_cst`
2. acquire-release model
   - `memory_order_consume`
   - `memory_order_acquire`
   - `memory_order_release`
   - `memory_order_acq_rel`
3. relaxed model
   - `memory_order_relaxed`


Please refer to [atomic and memory order](@atomic_and_memory_order.md)
