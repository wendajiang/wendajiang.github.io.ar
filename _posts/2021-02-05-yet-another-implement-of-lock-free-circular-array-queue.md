---
layout: post
title:  "lock-free circular array queue[翻译]"
date:   2021-02-03 01:26:18 +0800
categories: translate, lock-free, ABA, queue
---

[原链接](https://www.codeproject.com/Articles/153898/Yet-another-implementation-of-a-lock-free-circul)

推荐：

[Lock-free Programming](https://www.cs.cmu.edu/~410-s05/lectures/L31_LockFree.pdf)

https://preshing.com/20120612/an-introduction-to-lock-free-programming/



[TOC]

## 1. Introduction

毫无疑问，当今提升应用的性能的一个方式就是：**并发**。Thread 已经存在很长时间了，过去，大多数计算机只有一个处理器核心时，线程主要用于将总任务分解为小的执行单元，从而使一些执行单元等待资源的时候，可以执行其他执行单元。一个简单的例子是，监听TCP端口的网络应用，请求到达该端口之后会进行一些处理。在单线程方式下，应用直到处理完这个请求不能响应更多的请求，因此可能用户会认为这个应用宕机了。多线程时，可以由新线程处理请求，主线程一直监听请求。

单处理器计算机上，使用多线程的应用程序可能不会达到预期的结果。为了使处理器工作，多个线程可能互相影响，比如多个线程的通信的开销和共享数据，最终甚至不如单线程应用的性能好。

在SMP(symmetric multiprocessing)上，多线程应用可以同时（译者注：物理上的同时）work。每个线程使用一个物理处理器核心。在 N-处理器机器上运行 N-thread 程序理论上可以节省 N 倍的运行时间（当然实际上由于通信开销和共享数据开销，没有达到理论值）

SMP机器在过去很昂贵，只有极力推崇这种软件（译者注：大公司？）才能负担起费用，但是当今时代，多核处理器越来越便宜，所以将应用并发获得更高的性能越来越流行。

但是并发开发不是简单的工作。线程必须共享数据并互相通信，你会发现你面对的都是相同的老问题：**死锁，共享数据失控，跨线程动态内存分配/销毁等**。此外，如果你有幸在开发具有高性能（并发）的应用，你还会发现一系列不同的问题，有些会严重影响性能：

- [Cache Trashing](#no1)
- [Contention on your synchonization mechanism. Queues](#no2)
- [Dynamic memory allocation](#no3)

本文就提出了通过使用 array based lock-free queue 来最小化上面三个问题对性能的影响。尤其是动态内存分配的使用，因为这是设计无锁队列时的主要目标

## 2. How synchronizing threads can reduce the overall performance 
<a name="no1">
### 2.1 Cache trashing

Threads are (from Wikipedia): "the smallest unit of processing that can be scheduled by an operating system".每个系统有自己的线程实现，但是基本是一个进程内的指令集合加上一些进程内的local内存。线程执行一些指令，但是共享进程的内存空间。在Linux中（本文中写的queue优先在这个系统上执行），一个线程就是“执行的上下文（context of execution）”，没有线程概念。Linux 将线程实现为标准进程。Linux 内核没有提供任何关于线程特殊的调度语义或者数据结构。线程仅仅是与其他进程共享某些资源的进程。

每个执行的任务，线程，执行上下文，随你怎么称呼它【译者注：任务，线程，执行上下文在本文中同义】，使用一系列 CPU 的寄存器去运行。它们包含了任务的内部数据，比如正在执行的指令地址，操作或者操作结果，栈的指针等。这些信息就叫做“context（上下文）”。任何抢占式系统（大多数现代系统是抢占式的）都必须能够在几乎任何地方停止正在运行的任务，将上下文保存在某个地方，以便将来还原时使用（少数例外系统，比如进程声明自己一段时间使用CPU）。任务恢复后，就从停止的地方继续执行，就像没有什么发生一样。处理器是多任务共享的，所以等待 IO 的任务就可以被其它任务抢占。单处理系统上表现的想多处理器一样，但是就像生活的所有事情一样，存在 trade-off：处理器共享，但是每次任务被抢占，都会存在上下文保存和恢复的开销。

在save/restore context 过程中，存在隐藏开销：存在 **cache**中的数据是之前任务的，对于新任务无用。考虑到**处理器比内存处理速度快几倍**，所以大量的时间浪费在等待从内存读写数据到处理器。这也是 cache 在处理器和标准 RAM memory 之间的原因。Cache 很小但是很快，用于保存在不久将来会被访问的从 RAM memory 复制的数据。在处理密集型程序中，缓存miss对于性能影响非常大，如果没有缓存miss，性能将会有几倍的提升。

所以每次任务被强占 cache 都会被重写，这意味着对于 CPU 来说恢复运行要等一段时间。（有些操作系统，比如 Linux 会试图在最后一个正在执行任务的处理器上还原进程，但是取决于最后一个进程需要的内存大小，cache仍然可能会失效【译者注：这里没看懂。。。】）。当然我不是说抢占是坏事，抢占对于系统正确运行是必要的，但是可能由于你的设计，有些线程过于频繁的被抢占，会由于 cache 垃圾大大降低性能

什么时候任务会被抢占？取决于你的系统，但是中断处理，timers和系统调用很可能导致操作系统抢占子系统从而导致系统为其他进程分配处理时间。这是操作系统的难题一部分，因为没有人想要进程“饥饿”太长时间。某些系统调用处理“阻塞”状态，这意味着任务向操作系统请求资源继续执行。这是抢占式任务的很合适的例子，因为在资源准备好之前没有事情做，所以操作系统会挂起这个任务并执行其他任务。

资源通常表示内存，硬盘，外围设备或者网络中的数据，阻塞的同步机制比如信号量或者互斥量。如果任务试图获取已经被持有的互斥量，就会被抢占，一旦互斥量被释放，线程就会被加入到“ready-to-run”的任务队列中。所以如果你担心任务被频繁抢占，应该尽可能避免使用阻塞同步机制。

但是就像生活一样，没有事情是简单的。如果在避免使用阻塞同步机制的同时，使用的线程数量远多于物理处理器核心数，则可能会导致系统延迟。操作系统轮换任务的次数越少，任务等待执行的时间越长。甚至可能在应用整个生命周期内都一直等待空余出来的处理器。没有标准，依赖于你的应用和系统。比如，在处理密集的实时应用中，我会选择非阻塞机制同步线程，然后执行比物理核心更少的线程数，尽管这并不是万能药。在一些其他应用中，有大量等待数据的场景，比如网络，非阻塞同步会危害系统。没有不老之泉，每个方案都有其优势和局限，完全由你来决定如何选择。

<a name="no2">

### 2.2 Contentin on your synchonization mechanism. Queues

queue可以轻松应用于各种多线程场景。如果多线程需要通信，第一个浮现在我脑海的就是 queue。易于理解，易于使用，方便测试，还易于传授【译者：:)】世界上的每个开发都应该会queue，它无处不在。

queue 易于在单线程程序中使用，并且可以简单地适配到多线程系统。你需要的就是一个没有保护的队列（比如 C++中的 `std::queue`）和阻塞的同步机制（比如 mutex 和 conditional variables）。我上传了一个简单的使用glib的阻塞队列实现，不过由于 GAysncQueue是已经包含在 glib 中的线程安全队列实现，所以并不需要这样一个重复造的轮子，但是这份代码是一个将标准队列转换为线程安全队列的绝佳例子。

让我们来看下大多数 queue 中公共方法(IsEmpty, Push, Pop)的实现。基本无保护的 queue 是 `std::queue`声明为 `std::queue<t> m_theQueue`。三个非线程安全的方法实现使用 glib 的互斥量和条件变量（声明为`GMutex* m_mutex Cond* m_cond`）。从本文可以下载代码，还包含了`TryPush,TryPop`的方法实现，这两个方法当 queue 满或者空时不会阻塞。

```cpp
template<typename T>
bool BlockingQueue<T>::IsEmpty()
{
  bool rv;
  g_mutex_lock(m_mutex);
  rv = m_theQueue.empty();
  g_mutex_unlock(m_mutex);
  
  return rv;
}
```

`IsEmpty`当queue没有元素时返回`true`，但是必须对这个原始队列加上保护，这意味着要使用这个队列的线程必须等待 mutex 被释放后才能使用。

```cpp
template <typename T>
bool BlockingQueue<T>::Push(const T &a_elem)
{
    g_mutex_lock(m_mutex);

    while (m_theQueue.size() >= m_maximumSize)
    {
        g_cond_wait(m_cond, m_mutex);
    }

    bool queueEmpty = m_theQueue.empty();

    m_theQueue.push(a_elem);

    if (queueEmpty)
    {
        // wake up threads waiting for stuff
        g_cond_broadcast(m_cond);
    }

    g_mutex_unlock(m_mutex);

    return true;
}
```

`Push`往队列里插入元素。如果其他线程正在操作队列，这个线程会阻塞。如果队列已满，线程会阻塞，直到其他线程执行`Pop`操作，当然调用线程在等待其他线程弹出元素时不会占用任何 CPU 时间，因为操作系统已将其置为睡眠状态。

```cpp
template<typename T>
void BlockingQueue<T>::Pop(T &out_data)
{
  g_mutex_lock(m_mutex);
  while(m_theQueue.empty()) {
    g_cond_wait(m_cond, m_mutex);
  }
  bool queueFull = (m_theQueue.size() >= m_maximumSize) ? true : false;
  
  out_data = m_theQueue.front();
  m_theQueue.pop();
  
  if (queueFull) {
    // wake up threads waiting for stuff
    g_cond_broadcase(m_cond);
  }
  
  g_mutex_unlock(m_mutex);
}
```

`Pop`从对列中弹出一个元素（并删除它）。要使用的线程会阻塞如果另一个线程正在使用该队列。如果队列是空的，线程会阻塞，直到有其他队列往队列里加入元素，当然在等待其他线程插入元素时不会占用 CPU 时间，因为已被置为睡眠状态。

正如我在上一节解释的一样，**阻塞不是一件平凡操作**。它涉及操作系统将当前任务“挂起”，或者睡眠（不使用CPU等待）。一旦资源（比如mutex）可用，阻塞的任务就会被唤醒，同样不是平凡操作。**在负载很重的应用中使用这种阻塞队列在线程间传递消息就引起竞争，**意味着，**将会花费大量时间（sleeping, waiting, awakenging）在试图访问queue，而不是执行真正的任务**

在最简单的场景中，一个生产者线程往队列插入数据，一个消费者线程消费数据，两个线程都在竞争线程安全队列的互斥量。如果我们自己来实现而不是包装现成的队列，就可以使用两个不同的互斥量，一个用来插入数据，一个用来弹出数据。这种实现里面，竞争只会发生在边界情况，就是队列满或者空之时。现在，一旦需要超过一个生产者线程或者消费者线程，问题就又回来了。

这就是应用无阻塞机制的地方。**Tasks don't "fight" for any resource, they "reserve" a place in the queue without being blocked or unblocked, and then they insert/remove data from the queue**【译者注：这里不翻译】。这个机制需要一个特殊操作：CAS（Compare And Swap），维基百科中的定义：a special instruction that atomically compares the contents of a memory location to a given value and, only if they are the same, modifies the contents of that memory location to a given new value，举例如下

```cpp
volatile int a;
a = 1;
// this will loop while 'a' is not equal to 1. If it is equal to 1 the operation will atomically
// set a to 2 and return true
while(!CAS(&a, 1, 2)) ;
```

使用 CAS 实现lock-free 并不是新话题，有很多数据结构的示例，大部分使用链表举例。可以看看文献中2，3，[4](https://www.drdobbs.com/parallel/writing-lock-free-code-a-corrected-queue/210604448)【译者注: 2,3 连接找不到了】，这里目的不是描述什么  lock-free queue，而是：

- insert 新数据到新节点（malloc出来），然后使用CAS操作放到队列上
- 从队列 remove 数据，使用 CAS 从链表上将节点删除

这是链表 lock-free queue 的简单实现，从 2【译者注：反正找不到这个原文】 中复制，基于 [5](https://dl.acm.org/doi/10.1145/248052.248106)

```cpp
typedef struct _Node Node;
typedef struct _Queue Queue;

struct _Node {
    void *data;
    Node *next;
};

struct _Queue {
    Node *head;
    Node *tail;
};

Queue*
queue_new(void)
{
    Queue *q = g_slice_new(sizeof(Queue));
    q->head = q->tail = g_slice_new0(sizeof(Node));
    return q;
}

void
queue_enqueue(Queue *q, gpointer data)
{
    Node *node, *tail, *next;

    node = g_slice_new(Node);
    node->data = data;
    node->next = NULL;

    while (TRUE) {
        tail = q->tail;
        next = tail->next;
        if (tail != q->tail)
            continue;

        if (next != NULL) {
            CAS(&q->tail, tail, next);
            continue;
        }

        if (CAS(&tail->next, null, node)
            break;
    }

    CAS(&q->tail, tail, node);
}

gpointer
queue_dequeue(Queue *q)
{
    Node *node, *tail, *next;

    while (TRUE) {
        head = q->head;
        tail = q->tail;
        next = head->next;
        if (head != q->head)
            continue;

        if (next == NULL)
            return NULL; // Empty

        if (head == tail) {
            CAS(&q->tail, tail, next);
            continue;
        }

        data = next->data;
        if (CAS(&q->head, head, next))
            break;
    }

    g_slice_free(Node, head); // This isn't safe
    return data;
}
```

在没有 GC 的语言中（C++就是其中之一），由于所谓的 [ABA 问题](https://en.wikipedia.org/wiki/ABA_problem) ，对`g_slice_free`的调用是不安全的：

1. Thread T1 reads a value to dequeue and stops just before the first call to CAS in `queue_dequeue`
2. Thread T1 is preemtped. T2 attempts a `CAS` operation to remove the same node T1 was about to dequeue
3. It succedes and frees the memory allocated for that node
4. That same thread (or a new one, for instace T3) is going to enqueue a new node. the call to malloc returns the same address that was being used by the node removed in step 2-3. It adds that node into the queue
5. T1 takes the processor again, the `CAS` operation succeds incorrectly since the address is the same, but it's not the same node. T1 removes the wrong node

ABA 问题可以对每个节点加上引用计数解决。在假定CAS操作正确之前必须检查引用计数以避免 ABA 问题。好消息时，本文提到的 queue 不会收 ABA 问题影响，因为不使用动态内存分配。

<a name="no3">
### 2.3 Dynamic memory allocation

在多线程系统中，必须慎重考虑内存分配。**标准内存分配机制在为一个任务在堆上分配内存时会阻塞所有共享内存空间的任务**（进程内的所有线程）。这种方式简单正确，不会出现两个线程分配相同的地址，因为不会同时分配内存。但是当存在很多内存分配操作时就会导致性能很差（**必须提到的是类似标准库中 queue 或者 map 的插入操作就会在堆上分配内存**）

有一些库可以覆盖标准分配机制，提供无锁的内存分配机制来减少堆的争用，比如，[libhoard](https://github.com/pld-linux/libhoard)。有许多不同的类型的库，但是当你使用它们覆盖标准库操作时可能会对你的系统产生巨大的影响，因为你可能需要修改你的同步机制来适配无锁的内存分配机制。

## 3. The circular array based lock-free queue

### 3.1 How does it work?

#### 3.1.1 The CAS operation 

#### 3.1.2 Inserting elements into the queue

#### 3.1.3 Removing elements from the queue

#### 3.1.4 On the need for yielding the processor when there is more than 1 producer thread

## 4. Known issues of the queue

### 4.1 Using more than one producer thread 

### 4.2 Using the queue with smart pointers

### 4.3 Calculating size of queue

## 5. Compiling the code 

## 6. A few figures

### 6.1 The impact on performance of the 2nd CAS operation 

### 6.2 Lock-free vs. blocking queue. Number of Threads

### 6.3 Performance using 4 threads

#### 6.3.1 One producer thread 

#### 6.3.2 Two producer threads 

#### 6.3.3 Three producer threads 

### 6.4 A 4-core machine 

## 7. Conclusions

基于 lock-free 队列的 array 已经被证明了两个版本都是线程安全的，其中一个版本支持多生产者，另一个支持多消费者。队列用在多线程应用作为同步机制是安全的，因为：

- CAS 操作是原子的，线程试图并行对队列执行push或者pop操作**不会引起死锁**
- 多个线程同时push元素进queue，不会**写入到array同一个位置**
- 多个线程同时pop元素出queue，不会**多次获取到同一个位置元素**
- queue满了不能继续push元素，空了不能pop元素
- push和pop操作都不会导致ABA问题

然而需要注意的是即使算法是线程安全的，它在多个生产者场景中变现也比基于simple block-based queue好。因此，仅在以下两种情况下选择block-based queue才有意义：

- 只有一个生产者线程（版本2的 single-producer 更快）
- 有一个一直繁忙的生产者线程（译者注：一个长时间占用锁的生产者线程），然后我们仍然需要线程安全，因为还是可能会有其他生产者插入进去搞点事情

## 8. History 

4rd January 2011: Initial version

27th April 2011: Highlighting of some key words. Removed unnecesary uploaded file. A few typos fixed

31st July 2015: Updated code after [Artem Elkin](http://www.codeproject.com/script/Membership/View.aspx?mid=8409915) ([single producer](http://www.codeproject.com/Messages/4995350/Single-producer-push.aspx and)) and [Member 11590800](http://www.codeproject.com/script/Membership/View.aspx?mid=11590800) [(overflow bug](http://www.codeproject.com/Messages/5038886/overflow-bug.aspx)) comments.

## 9. References
请参考原文中的引用