---
title: atomic and memory order in c++11
description: ''
template: blog/page.html
date: 2023-03-14 20:58:47
updated: 2023-03-14 20:58:47
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["atomic", "memory order"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

[reprint](https://changkun.de/modern-cpp/en-us/07-thread/#7-5-Atomic-Operation-and-Memory-Model)

# Problem
First, let's look at the code, and what is the output of the follow code ?
```cpp
#include <thread>
#include <iostream>
int main() {
  int a = 0;
  volatile int flag = 0;
  std::thread t1([&]() {
    while(flag != 1);
    int b = a;
    std::cout << "b = " << b << std::endl;
  });
  std::thread t2([&]() {
    a = 5;
    flag = 1;
  });
  t1.join();
  t2.join();
  return 0;
}
```
Intuitively, it seems that `a=5`, in `t2` always executes before `flag = 1`, and `while(flag != 1)` in `t1`. It looks like there is a guarantee the line `std::cout << "b = " << b << std::endl;` will not be executed before the mark is changed. Logically, the value of `b` is 5. **But the code behavior is undefined.** Out-of-order execution of the CPU and the impact of the compiler on the rearrangement of instructions. Cause `a = 5` to occur after `flag = 1`. Thus `b` maybe output `0`.

# Atomic operation
`std::mutex` can solve the problem of concurrent read and write, but the mutext is an operating system-level function. This is because the implementation of a mutex usually contains two basic principles:
- provide automatic state transition between threads, that is , "lock" state
- Ensure that the memory of the manipulated variable is isolated from the critical section during the mutex operation.

This is a very strong set of synchronization conditions, in other words when it is finally compiled into a CPU instruction, it will behave like a lot of instructions.

We should understand that under the modern CPU architecture, atomic operations at the CPU instruction level are provided. Therefore, int the C++11 multi-threaded shared variable reading and writing, the introduction of the `std::atomic` template, so that we instantiate an atomic type, will be an Atomic type read and write operations are minimized from a set of instructions to a single CPU instruction.

Of course, not all types provide atomic operations because the feasibility of atomic operations depends on the architecture of the CPU and whether the type structure being instantiated satisfies the memory alignment requirements of the architecture, so we can always pass `std::atomic<T>::is_lock_free` to check if the atom type needs to support atomic operations, for example:
```cpp
#include <atomic>
#include <iostream>
struct A {
  float x;
  int y;
  long long z;
}
int main() {
  std::atomic<A> a;
  std::cout << std::boolalpha << a.is_lock_free() << std::endl;
  return 0;
}
```

# Consistency model
Mutliple threads executing in parallel, discussed at some macro level, can be roughly considered a distributed system. In a distributed system, any communication or even local operation takes a certain amount of time, and even unreliable communication occurs.

If we force the operation of a variable `v` between multiple threads to be atomic, that is, any thread after the operation of `v`, other threads can syncchronize to perceive changes in `v`, for the variable `v`, which appears as a sequential execution of the program, it does not have any efficiency gains due to the introduction of multithreading. Is there any way to accelerate this properly? The answer is to weaken the sychronization conditions between processes in atomic operations.

In principle, each thread can correspond to a cluster node, and the communication between threads is almost equivalent to communication between cluster node. Weakening the synchronization conditions between processes, usually we will consider four different consistency models:

## Linear consistency
Also known as strong consistency or atomic consistency. It requires that any read operation can read the most recent write of a certain data, the the order of operation of all threads is consistent with the order under the global clock.
```bash
        x.store(1)      x.load()
T1 ---------+----------------+------>


T2 -------------------+------------->
                x.store(2)
```
Thread T1 , T2 is twice atomic to x, and x.store(1) is strictly before x.store(2), x.store(2) strictly occurs before x.load(). It is worth mentioning that linear consistency requirements for global clocks are difficult to achieve, which is why people continue to study other consistent algorithms under this weaker consistency.

### Sequential consistency
It is also required that any read operation can read the last data writen by the data, but it is not required to be consistent to be consistent with the order of the global clock.
```bash
        x.store(1)  x.store(3)   x.load()
T1 ---------+-----------+----------+----->


T2 ---------------+---------------------->
              x.store(2)

or

        x.store(1)  x.store(3)   x.load()
T1 ---------+-----------+----------+----->


T2 ------+------------------------------->
      x.store(2)
```
under the order consistency requirement, x.load() must read the last written data, so x.store(2) and x.store(1) do not have any guarantees, as long as x.store(2) of T2 occur before x.store(3).

## Causal consistency
It requirements are further reduced, only the sequence of causal operations is guaranteed, and the order of non-causal operations is not required.
```bash
      a = 1      b = 2
T1 ----+-----------+---------------------------->


T2 ------+--------------------+--------+-------->
      x.store(3)         c = a + b    y.load()

or

      a = 1      b = 2
T1 ----+-----------+---------------------------->


T2 ------+--------------------+--------+-------->
      x.store(3)          y.load()   c = a + b

or

     b = 2       a = 1
T1 ----+-----------+---------------------------->


T2 ------+--------------------+--------+-------->
      y.load()            c = a + b  x.store(3)
```
The three examples given above are all causal consistent as, in the whole process, only c has a dependency on a and b, and x and y are not related in this example.

## Final consistency
It is the weakest consistency requirement. It only guarantees that an operation will be observed at a certain point in the future, but does not require the observed time. So we can strenthen this condition a bit, for example, to specify that the time observed for an operation is always bounded. Of course, this is no longer within our discussion.

```bash
    x.store(3)  x.store(4)
T1 ----+-----------+-------------------------------------------->


T2 ---------+------------+--------------------+--------+-------->
         x.read()      x.read()           x.read()   x.read()
```
In the above case, if we assume that the initial value of x is 0, then the four times x.read() in T2 maybe but not limited to the following:
```bash
3 4 4 4 // The write operation of x was quickly observed
0 3 3 4 // There is a delay in the observed time of the x write operation
0 0 0 4 // The last read read the final value of x, 
        // but the previous changes were not observed.
0 0 0 0 // The write operation of x is not observed in the current time period, 
        // but the situation that x is 4 can be observed 
        // at some point in the future.
```

# Memory order
To achieve the ultimate performance and achieve consistency of various strength requirements, C++11 defines six different memory sequences for atomic operations. The option `std::memory_order` expresses four synchronization models between multiple threads:

## Relaxed model
Under this model, atomic operations within a single thread are executed sequentially, and instruction recordering is not allowed, but the order of atomic operations between different threads is arbitrary. The type is specified by `std::memory_order_relaxed`
```cpp
std::atomic<int> counter = {0};
std::vector<std::thread> vt;
for (int i = 0; i < 100; ++i) {
    vt.emplace_back([&](){
        counter.fetch_add(1, std::memory_order_relaxed);
    });
}

for (auto& t : vt) {
    t.join();
}
std::cout << "current counter:" << counter << std::endl;
```

## ~~Rlease/consumption model~~
In this model, we begin to limit the order of operations between processes. If a thread needs to modify a value, but another thread will have a dependency on that opertion of the value, that is, the latter depends on the former. Specifically, thread A has completed three writes to x, and thread B relies only on the third x wrtie operation, regardless of the first two write behaviors of x, then A when active x.release()(ie using `std::memory_order_release`), the option `std::memory_order_consume` ensures that B observes A when calling x.load().

```cpp
// initialize as nullptr to prevent consumer load a dangling pointer
std::atomic<int*> ptr(nullptr);
int v;
std::thread producer([&]() {
  int *p = new int(42);
  v = 1024;
  ptr.store(p, std::memory_order_release);
});
std::thread consumer([&]() {
    int* p;
    while(!(p = ptr.load(std::memory_order_consume)));

    std::cout << "p: " << *p << std::endl;
    std::cout << "v: " << v << std::endl;
});
producer.join();
consumer.join();
```
## Release/acquire model
Under this model, we can further tighten the order of atomic operations between different threads, specifying the timing between releasing `std::memory_order_release` and getting `std::memory_order_acquire`. All write operations before the release operation is visible to any other thread, i.e., happends before.

As you can see, `std::memory_order_release` ensures that a write before a release does not occur after the release operation, which is **backward barrier**, and `std::memory_order_acquire` ensures that a subsequent read or write after a acquire does not occur before the acquire operation, which is **forward barrier**. For the `std::memory_order_acq_rel` option, combines the charateristics of the two barriers and determines a unique memory barrier, such that reads and writes of the current thread will not be rearranged across the barrier.

```cpp
std::vector<int> v;
std::atomic<int> flag = {0};
std::thread release([&]() {
    v.push_back(42);
    flag.store(1, std::memory_order_release);
});
std::thread acqrel([&]() {
    int expected = 1; // must before compare_exchange_strong
    while(!flag.compare_exchange_strong(expected, 2, std::memory_order_acq_rel)) 
        expected = 1; // must after compare_exchange_strong
    // flag has changed to 2
});
std::thread acquire([&]() {
    while(flag.load(std::memory_order_acquire) < 2);

    std::cout << v.at(0) << std::endl; // must be 42
});
release.join();
acqrel.join();
acquire.join();
```

In this case, we used `compare_exchange_strong`, which is the Compare-and-swap primitive, which has a weaker version, `compare_exchange_weak`, which allows a failure to be returned even if the exchange is successful. The reason is due to a false failure on some platforms, specifically when the CPU performs a context switch, another threads loads the same address to produce an inconsistency. In addition, the performance of `compare_exchange_strong` maybe slightly worse than `compare_exchange_weak`. However, in mose cases, `compare_exchange_weak` is discouraged due to the complexity of its usage.

## Sequential consistent model
Under this model, atomic operations satisfy sequence consistency, which in turn can cause performance loss. It can be specified explicityly by `std::memory_order_seq_cst`.
```cpp
std::atomic<int> counter = {0};
std::vector<std::thread> vt;
for (int i = 0; i < 100; ++i) {
    vt.emplace_back([&](){
        counter.fetch_add(1, std::memory_order_seq_cst);
    });
}

for (auto& t : vt) {
    t.join();
}
std::cout << "current counter:" << counter << std::endl;
```
This example is essentially the same as the first loose model example. Just change the memory order of the atomic operation to `memory_order_seq_cst`.

# further reading
- c++ concurrency in action(2nd)
- https://en.cppreference.com/w/cpp/thread
- https://blog.the-pans.com/cpp-memory-model-as-a-distributed-system/