+++
template = "blog/page.html"
title =  "lock-free circular array queue[翻译]"
date =   "2021-02-05 17:26:18"
[taxonomies]
tags=["translate", "lock-free", "ABA", "queue"]
+++

[原链接](https://www.codeproject.com/Articles/153898/Yet-another-implementation-of-a-lock-free-circul)

推荐：

[Lock-free Programming](https://www.cs.cmu.edu/~410-s05/lectures/L31_LockFree.pdf)

https://preshing.com/20120612/an-introduction-to-lock-free-programming/

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

ABA 问题可以对每个节点加上引用计数解决。在假定CAS操作正确之前必须检查引用计数以避免 ABA 问题。不过好消息是，本文提到的 queue 不会受 ABA 问题影响，因为不使用动态内存分配。

<a name="no3">

### 2.3 Dynamic memory allocation

在多线程系统中，必须慎重考虑内存分配。**标准内存分配机制在为一个任务在堆上分配内存时会阻塞所有共享内存空间的任务**（进程内的所有线程）。这种方式简单正确，不会出现两个线程分配相同的地址，因为不会同时分配内存。但是当存在很多内存分配操作时就会导致性能很差（**必须提到的是类似标准库中 queue 或者 map 的插入操作就会在堆上分配内存**）

有一些库可以覆盖标准分配机制，提供无锁的内存分配机制来减少堆的争用，比如，[libhoard](https://github.com/pld-linux/libhoard)。有许多不同的类型的库，但是当你使用它们覆盖标准库操作时可能会对你的系统产生巨大的影响，因为你可能需要修改你的同步机制来适配无锁的内存分配机制。

## 3. The circular array based lock-free queue

在此基于 array 的 lock-free 的环形队列首次登场，尽可能降低第2节三个问题的影响。可以总结为以下特性：

- 作为lock-free同步机制，降低了任务被强占的频率，从而降低了 cache trashing
- 同样的作为 lock-free 队列，线程之间竞争减少，因为lock-free：线程基本上是先声明空间已被占用，然后将数据填入
- 不需要在堆上分配空间
- 不会受到 ABA 问题的影响，当然也加入了 array 一些操作的开销

### 3.1 How does it work?

queue 基于 array 和三个 index：

- `writeIndex`:新元素要被插入的地方
- `readIndex`:下一个被弹出的元素位置
- `maximumReadIndex`:上一个已经‘commit’要插入的元素位置。如果与`writeIndex`位置不同，意味着有写入被挂起，也意味着这个位置已经被声明占用了，但是数据还没有写进去到队列里，所以试图读的线程需要等待数据被填入

值得一提的是，三个index是必要的，因为队列允许多生产者和多消费者。有[文章](https://wendajiang.github.io/translate/queue/lock-free/2021/02/07/single-p-single-c-lock-free-cricular-queue.html)研究了单生产者单消费者，这篇文章值得一读（我一直很喜欢 [KISS 原则](https://en.wikipedia.org/wiki/KISS_principle)）。这里的事情变得复杂很多，因为队列必须对于各种线程配置都要线程安全。

#### 3.1.1 The CAS operation 

Lock-free queue 同步机制基于 CAS 的 CPU 指令。CAS 操作已经在 GCC 4.1.0 实现。因此在GCC4.4 版本编译，我采用了 GCC 的 `build_in operation: __sync_bool_compare_and_swap`（[这里是GCC文档](https://gcc.gnu.org/onlinedocs/gcc-4.4.2/gcc/Atomic-Builtins.html)），为了移植性考虑，这个操作通过宏定义了 `CAS`，在`atomic_ops.h`文件中：

```cpp
#define CAS(a_ptr, a_oldVal, a_newVal) __sync_bool_compare_and_swap(a_ptr, a_oldVar, a_newVal)
```

如果要使用其他编译器或者其他版本，你需要宏定义CAS操作，接口满足如下条件：

- 第一个是可变的地址参数
- 第二个参数是老值
- 第三个参数是新值
- 成功返回`true`，否则返回`false`

<a name="no5">

#### 3.1.2 Inserting elements into the queue

这是插入元素的代码：

```cpp
/* ... */
template <typename ELEM_T, uint32_t Q_SIZE>
inline
uint32_t ArrayLockFreeQueue<ELEM_T, Q_SIZE>::countToIndex(uint32_t a_count)
{
    return (a_count % Q_SIZE);
}

/* ... */

template <typename ELEM_T>
bool ArrayLockFreeQueue<ELEM_T>::push(const ELEM_T &a_data)
{
    uint32_t currentReadIndex;
    uint32_t currentWriteIndex;

    do
    {
        currentWriteIndex = m_writeIndex;
        currentReadIndex  = m_readIndex;
        if (countToIndex(currentWriteIndex + 1) ==
            countToIndex(currentReadIndex))
        {
            // the queue is full
            return false;
        }

    } while (!CAS(&m_writeIndex, currentWriteIndex, (currentWriteIndex + 1)));

    // We know now that this index is reserved for us. Use it to save the data
    m_theQueue[countToIndex(currentWriteIndex)] = a_data;

    // update the maximum read index after saving the data. It wouldn't fail if there is only one thread
    // inserting in the queue. It might fail if there are more than 1 producer threads because this
    // operation has to be done in the same order as the previous CAS

    while (!CAS(&m_maximumReadIndex, currentWriteIndex, (currentWriteIndex + 1)))
    {
        // this is a good place to yield the thread in case there are more
        // software threads than hardware processors and you have more
        // than 1 producer thread
        // have a look at sched_yield (POSIX.1b)
        sched_yield();
    }

    return true;
}

```

下图描述了一个queue的初始状态，每个格子表示queue的位置，如果标记了 X 表示包含了数据，空白格子就是空的。图中表示当前queue已经插入了两个元素。`writeIndex`指向新元素将要插入的位置，`readIndex`指向下次`pop`弹出的元素位置

![image-20210205150248827](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205150248827.png)

基本上，当新元素被`push`操作写入队列时，writeIndex increment。MaximumReadIndex指向最新的有效数据

![image-20210205150433877](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205150433877.png)

一旦新空间被占用，当前线程就会开始将数据拷贝进queue。然后increment maximumReadIndex

![image-20210205150546567](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205150546567.png)

此时，队列中有了三个被插入的元素。下一步，另一个任务试图继续插入新元素

![image-20210205150728199](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205150728199.png)

已经腾出了数据的空间，但是这时被其它线程抢占也要插入一个元素（再占用一个）

![image-20210205150852717](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205150852717.png)

此时，线程开始往占用的位置拷贝数据，但是**必须**按照**严格的顺序**：第一个生产者线程 increment maximumReadIndex，然后第二个线程再 increment。这个顺序很重要，因为在允许消费线程将其从队列中`pop`之前，要确保数据被保存到'commited'的位置。【译者注：这个顺序通过CAS对于maximumReadIndex保证】

![image-20210205151841109](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205151841109.png)

第一个生产者【译者注：1，2的先后顺序由reversed位置定义】将数据提交位置。现在该第二个线程继续自己的任务了

![image-20210205151944816](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205151944816.png)

现在队列插入了5个元素

#### 3.1.3 Removing elements from the queue

这是`pop`的代码：

```cpp
/* ... */

template <typename ELEM_T>
bool ArrayLockFreeQueue<ELEM_T>::pop(ELEM_T &a_data)
{
    uint32_t currentMaximumReadIndex;
    uint32_t currentReadIndex;

    do
    {
        // to ensure thread-safety when there is more than 1 producer thread
        // a second index is defined (m_maximumReadIndex)
        currentReadIndex        = m_readIndex;
        currentMaximumReadIndex = m_maximumReadIndex;

        if (countToIndex(currentReadIndex) ==
            countToIndex(currentMaximumReadIndex))
        {
            // the queue is empty or
            // a producer thread has allocate space in the queue but is
            // waiting to commit the data into it
            return false;
        }

        // retrieve the data from the queue
        a_data = m_theQueue[countToIndex(currentReadIndex)];

        // try to perfrom now the CAS operation on the read index. If we succeed
        // a_data already contains what m_readIndex pointed to before we
        // increased it
        if (CAS(&m_readIndex, currentReadIndex, (currentReadIndex + 1)))
        {
            return true;
        }

        // it failed retrieving the element off the queue. Someone else must
        // have read the element stored at countToIndex(currentReadIndex)
        // before we could perform the CAS operation

    } while(1); // keep looping to try again!

    // Something went wrong. it shouldn't be possible to reach here
    assert(0);

    // Add this return statement to avoid compiler warnings
    return false;
}
```

还是用插入数据一节的queue的初始状态。有两个元素已经插入队列。

![image-20210205152150470](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205152150470.png)

消费线程，从`readIndex`复制数据，然后尝试在相同的`readIndex`执行`CAS`操作。如果线程执行`CAS`成功，表示数据已经从队列取出，因为`CAS`是原子的。如果`CAS`失败，下次尝试就会从新的位置重复这个过程，如下图

![image-20210205152645278](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205152645278.png)

结果就是下图

![image-20210205152726271](https://wendajiang.github.io/pics/2021-02-05-yet-another-implement-of-lock-free-circular-array-queue/image-20210205152726271.png)

如果这个时候还有线程要读取数据，就会失败，因为队列已空。

现在一个任务尝试在队列中插入新元素，已经占到了位置但是提交数据被挂起，然后另一个线程想要`pop`一个元素，现在知道队列不空了，（因为`writeIndex != readIndex`），但是还不能读取，因为`maximumReadIndex != readIndex`。这个线程试图`pop`，将陷入循环中，直到数据被提交让`maximumReadIndex == readIndex`或者队列称为空（如果另有一个消费线程进来先消费了，然后再次`writeIndex == readIndex`）

【译者注：这里原文两个图不贴了，上面文字描述的更清楚】

<a name="no4">

#### 3.1.4 On the need for yielding the processor when there is more than 1 producer thread

【译者注：本节详细阐述了push第二次 CAS 是为了保证 FIFO 顺序，这里如果单纯++，会导致数据错乱】

读者可能注意到了`push`函数中对于`sched_yield`的调用出让CPU，对于宣称 lock-free 的算法看起来有点奇怪。正如文章开头就提到的一样，多线程的性能下降原因之一就是**cache trashing**。典型造成`cache trashing`就是被强占的线程上下文需要操作系统从cache转移到主存，恢复时又要转移回来。

当算法调用`sched_yield`时，就是告诉操作系统：hi，你能把别的任务搞过来吗，我必须等会才能执行。lock-free 和 lock 同步机制的主要区别就是我们不需要阻塞，所以为啥要主动告诉操作系统赶紧抢占我？回答这个问题并不简单，涉及到了生产者将新数据保存到队列中，需要以FIFO 顺序执行两次 CAS 操作，一次申请分配空间，另一次通知消费者数据已经提交。

如果应用只有一个生产线程，`sched_yield`就不需要调用了，因为第二次 CAS 绝对不会失败。操作自然就会按照 FIFO 的顺序执行，因为只有一个线程在插入数据

当大于一个线程插入数据时，问题就来了。如同[3.1.2节](#no5)表述那样插入数据，1CAS已经按照 FIFO 顺序申请空间后，2CAS必须也按照 FIFO 顺序执行。让我们考虑下面的场景，三个生产者一个消费者：

- 线程1，2，3按照顺序申请空间。2CAS必须按照相同顺序执行，1，2，3
- 线程2先开始执行2CAS，但是因为Thread 1还没执行所以失败了，Thread 3也会失败
- 2和3线程陷入循环直到线程1执行了2CAS
- 线程1执行完之后，线程3必须等2执行
- 最终按序执行完

2CAS失败是可能spin也不失为一个好选择。当使用的多处理器机器的处理器数量大于线程数量时，这可能很好：线程卡住一直尝试在volatile variable(maximumReadIndex)上执行CAS操作，但是等待的线程可能分配到物理处理器核心，因此最终比如Thread1可能执行2CAS，自旋的操作也能结束。总而言之，该算法有保留线程循环的可能性，但是要保证这种行为是可行的，在某些特定情况可以删除`scheld_yield`。事实上，删除`scheld_yield`才是最好的性能。

但是`scheld_yield`对于多个生产者和线程数量大于物理核心数的场景是必要的。考虑之前的问题，当三个线程试图插入新数据到队列中，线程1在分配空间后被抢占，线程2，3会一直死循环直到线程1被唤醒，这就需要`sheld_yield`，操作系统不比一直让线程2，3保持循环，它们必须尽快阻塞，以让线程1执行2CAS，让自己能够继续执行。

## 4. Known issues of the queue

这个lock-free 队列的主要目的是提供一个不需要动态内存分配的 lock-free 队列，已经搞定，但是算法存在一些已知的缺点，在生产环境中使用该算法应该考虑这些缺点是不是你关注的。

### 4.1 Using more than one producer thread 

如同在[3.1.4](#no4)中描述一样（如果你要在多生产者环境中使用这个算法要仔细阅读这一节内容），如果有超过一个生产线程，由于必须按照 FIFO 顺序操作 `maximumReadIndex`，所以可能多次调用`sched_yield`导致花费很多开销。这个队列设计的原始场景只有一个生产线程，因此在多生产线程的场景，性能是肯定会下降的。

此外，如果你计划将此队列用于单生产者线程的场景，不需要第二个 CAS 操作。对于 `m_maximumReadIndex`的 CAS 可以被删除，所以代码如下：

```cpp
template <typename ELEM_T>
bool ArrayLockFreeQueue<ELEM_T>::push(const ELEM_T &a_data)
{
    uint32_t currentReadIndex;
    uint32_t currentWriteIndex;

    currentWriteIndex = m_writeIndex;
    currentReadIndex  = m_readIndex;
    if (countToIndex(currentWriteIndex + 1) ==
        countToIndex(currentReadIndex))
    {
        // the queue is full
        return false;
    }

    // save the date into the q
    m_theQueue[countToIndex(currentWriteIndex)] = a_data;

    // No need to increment write index atomically. It is a 
    // requierement of this queue that only one thred can push stuff in
    m_writeIndex++;

    return true;
}

template <typename ELEM_T>
bool ArrayLockFreeQueue<ELEM_T>::pop(ELEM_T &a_data)
{
uint32_t currentMaximumReadIndex;
uint32_t currentReadIndex;

do
{
    // m_maximumReadIndex doesn't exist when the queue is set up as
    // single-producer. The maximum read index is described by the current
    // write index
    currentReadIndex        = m_readIndex;
    currentMaximumReadIndex = m_writeIndex;

    if (countToIndex(currentReadIndex) ==
        countToIndex(currentMaximumReadIndex))
    {
        // the queue is empty or
        // a producer thread has allocate space in the queue but is
        // waiting to commit the data into it
        return false;
    }

    // retrieve the data from the queue
    a_data = m_theQueue[countToIndex(currentReadIndex)];

    // try to perfrom now the CAS operation on the read index. If we succeed
    // a_data already contains what m_readIndex pointed to before we
    // increased it
    if (CAS(&m_readIndex, currentReadIndex, (currentReadIndex + 1)))
    {
        return true;
    }

    // it failed retrieving the element off the queue. Someone else must
    // have read the element stored at countToIndex(currentReadIndex)
    // before we could perform the CAS operation

} while(1); // keep looping to try again!

// Something went wrong. it shouldn't be possible to reach here
assert(0);

// Add this return statement to avoid compiler warnings
return false;
}
```

如果你要在单生产者，单消费者场景使用，再次推荐[此文章](https://wendajiang.github.io/translate/queue/lock-free/2021/02/07/single-p-single-c-lock-free-cricular-queue.html)，用了类似的环形队列设计。

### 4.2 Using the queue with smart pointers

如果队列每个位置保存的是只能指针，请注意，插入队列的元素会由于智能指针的保护不能被完整删除，保存过元素的位置直到被新的智能指针占用才会完全删除这个数据，所以对于繁忙的队列这不是问题，不过开发者要注意的是，一旦队列第一次被占满，应用使用的内存就不会降下去了，即使队列已空【译者注：适配这个场景，要改代码，比如使用偏特化指定smart_pointer data_type要加入多余的处理？】

### 4.3 Calculating size of queue

原始`size`可能会返回错的size数据

```cpp
template <typename ELEM_T>
inline uint32_t ArrayLockFreeQueue<ELEM_T>::size()
{
    uint32_t currentWriteIndex = m_writeIndex;
    uint32_t currentReadIndex  = m_readIndex;

    if (currentWriteIndex >= currentReadIndex)
    {
        return (currentWriteIndex - currentReadIndex);
    }
    else
    {
        return (m_totalSize + currentWriteIndex - currentReadIndex);
    }
}
```

下面的场景会返回错误size：

1. `uint32_t currentWriteIndex = m_writeIndex`被执行，`m_writeIndex`为3，`m_readIndex`为2，真实的size是1
2. 这个线程被抢占，当此线程处于非活动状态时，队列中被插入，删除了两个元素，此时`m_writeIndex`为5，`m_readIndex`为4，size还是1
3. 现在线程回来继续执行，`uint32_t currentReadIndex = m_readIndex`，读取到为4
4. `currentReadIndex > currentWriteIndex`，所以返回`m_totalSize + currentWriteIndex - currentReadIndex`，也就是说当队列几乎为空的时候，返回队列几乎已满

本文上传的代码已经解决了这个问题，加入了一个新的类数据成员，表示当前队列的数据个数，也是用原子操作进行操作。这个方案引入了很大的开销，因为不能被编译器优化，所以原子操作是昂贵的。

这也是为什么留给开发者选择是否激活这个size变量的原因，取决于应用对于需求的要求，这个`size`操作是不是必要的，来决定是不是要引入这个开销。编译器会预处理`array_lock_free_queue.h`调用`ARRAY_LOCK_FREE_Q_KEEP_REAL_SIZE`定义，来决定size开销是否会激活。如果没有定义就不会激活这个开销，但是函数可能会返回错误size

## 5. Compiling the code 

【译者注：本节简单意译】

这里的代码都是模板代码，所以只需要头文件，不过提供了对应测试代码，是`cpp`格式。测试代码中使用了 `comp`，跨平台内存共享并行编程OpenMP 接口的 GNU c/c++实现已经包含在了 GCC4.2 中。OpenMP 是跨平台开发并发程序一个简单灵活的接口。

所以代码分为了三个部分：

1. array based lock-free queue:

   - 有两个版本的代码，分别在 array_lock_free_queue.h 和 array_lock_free_queue_single_producer.h 中，单生产者版本为场景优化过
   - 注意，代码没有在64位环境中测试过

2. Glib based blocking queue: 

   - 首先你的系统中需要有Glib，Linux系统一般已经包含
   - 使用了glib的 mutex 和 cond variable ，所以编译时要连接上

3. 测试程序：

   - 上面两个部分

   - GNU make 程序

     ```bash
     make N_PRODUCERS=1 N_CONSUMERS=1 N_ITERATIONS=10000000 QUEUE_SIZE=1000
     ```

     - N_PRODUCERS 是生产线程的数量
     - N_CONSUMERS 是消费线程的数量
     - N_ITERATIONS 是将要插入和弹出元素之和
     - QUEUE_SIZE 是队列的最大长度

   - GCC版本大于 4.2

   - 需要加上`OMP_NESTED=TRUE`前缀，例如`OMP_NESTED=TRUE ./test_lock_free_q`

## 6. A few figures

【译者注：本大节全是测试图例，不搬运，感兴趣请在原链接看，或者自己运行代码得到直观对比】

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
【译者注：请参考原文中的引用，有些引用连接已经失效，在正文翻译中已经尽可能还原了可用链接】