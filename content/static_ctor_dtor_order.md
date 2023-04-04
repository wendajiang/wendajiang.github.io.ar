---
template: blog/page.html
date: 2023-01-03 13:33:19
title: Static object initialization and deinitialization
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["cpp", "fap", "singleton"]
extra:
  mermaid: true
  usemathjax: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

## 引言
在工作中遇到了，[isocpp](https://isocpp.org/wiki/faq) 中提到的 subtle 问题，在 isocpp faq 中原始问题来源于 [fiasco](https://isocpp.org/wiki/faq/ctors#static-init-order)，简单来说就是，如果有两个 static 对象 x,y 分布在不同的源文件中。假设 y 对象的初始化中会使用 x 对象，就可能出现问题。因为 static 对象初始化发生在 main 之前，并且编译器没有保证初始化顺序，所以就可能出现 crash。

那么如何消除这种问题呢？*Construct On First Use Idiom* 提出来，就是使用 wrapper 来获取 static 对象，而不是直接使用。

```cpp
// file x.cpp
#include "fred.h"
Fred x;

// file y.cpp
#include "barney.h"
Barney y;

Barney::Barney() {
  // ..
  x.do_something();
}

// => 
Fred& x() {
  static Fred* ans = new Fred();
  return *ans;
}

Barney::Barney() {
  // ..
  x().do_something();
}
```
这就意味着 leak 了一个 Fred 对象，如果 Fred 对象析构函数必须被调用，可以使用 v2 版本的 *Construct On First Use Idiom*

```cpp
// 为什么不使用一个 local scope static object 来代替指针
Fred& x() {
  static Fred ans;
  return ans;
}
```

实际呢这里引入了另一个微妙我遇到的问题。让我们来回顾一下，这些技术是为了达到什么目的？

- (a)第一次使用 static 对象肯定是已经构造完成的。
- (b)直到最后一次使用才析构

显然如果在第一次使用前没有构造，或者最后一次使用前已经析构都是问题所在。意味着我们要考虑头和尾。现在 v2 版本的方案已经解决了使用前一定构造的问题，并且没有造成对象的 leak【实际呢，如果是一个会退出的程序，这种泄漏也可以接受，毕竟在进程退出后，系统会回收内存，并没有很大影响】

但是，这种方案存在问题，就是不能确保多个 static 对象的析构顺序。

## 我遇到的问题

```cpp
// logger.h
class GlobalLogger;

GlobalLogger& logger() {
  static GlobalLogger& logger;
  return logger;
}

// file_monitor.h
class FileMonitor;
/* FileMonitor is one data member of a global static object*/
FileMonitor::~FileMonitor() {
  // do something
  logger().log("something");
}

```

大部分时间运行没有问题，但是并没有保证在 FileMonitor 析构中使用 GlobalLogger 是有效的。所以在某些情况，会 crash。

那么如何解决这个问题？[ioscpp](https://isocpp.org/wiki/faq/ctors#nifty-counter-idiom) 也提出了解决方案 [**Nifty Counter Idiom**](https://en.wikibooks.org/wiki/More_C%2B%2B_Idioms/Nifty_Counter)。

原理就是在头文件中定义一个辅助 static object 的计数 class，这样每个 include 头文件的编译单元就会创建一个内部的计数对象，在计数对象构造函数中积累计数，析构函数中消除计数，同时控制一个 extern global 要使用的 static object 对象的创建与销毁，达到

- (a)第一个次使用 static 对象肯定是已经构造完成的。
- (b)直到最后一次使用才析构

这个目的，即同时控制构造和析构。

nifty counter idiom 例子：

```cpp
// Stream.h
#ifndef STREAM_H
#define STREAM_H

struct Stream {
  Stream ();
  ~Stream ();
};
extern Stream& stream; // global stream object

static struct StreamInitializer {
  StreamInitializer ();
  ~StreamInitializer ();
} streamInitializer; // static initializer for every translation unit

#endif // STREAM_H

// Stream.cpp
#include "Stream.h"

#include <new>         // placement new
#include <type_traits> // aligned_storage

static int nifty_counter; // zero initialized at load time
static typename std::aligned_storage<sizeof (Stream), alignof (Stream)>::type
  stream_buf; // memory for the stream object
Stream& stream = reinterpret_cast<Stream&> (stream_buf);

Stream::Stream ()
{
  // initialize things
}
Stream::~Stream ()
{
  // clean-up
} 

StreamInitializer::StreamInitializer ()
{
  if (nifty_counter++ == 0) new (&stream) Stream (); // placement new
}
StreamInitializer::~StreamInitializer ()
{
  if (--nifty_counter == 0) (&stream)->~Stream ();
}
```

其中不同与一般方式的实现是：定义了一个 static(local to the translation unit) buffer，这个 buffer 足够存放一个 Stream。Stream 对象的引用定义在头文件中，然后设置成指向 buffer 的指针。这样实现能更好的控制 Stream 对象的构造和析构的调用。上面的例子中，构造函数在 StreamInitializer 第一次调用前被调用，然后使用 placement new 设置指针，析构函数在最后一个 StreamInitializer 使用后被调用。

如果是在 Stream.cpp 中定义一个 Stream 变量，会在 StreamInitializer 之后定义（这个在头文件中定义）。那么 StreamInitializer 构造函数就会在 Stream 构造函数之前运行，更糟糕的是 Initializer 的析构函数会在 Stream 析构之后运行。buffer 方案避免了这个问题。

由此，解决了 static object 对象构造和析构有依赖关系的问题。

## Singleton ？
在这里发现 v2 版本的 "Construct On First Use Idiom" 基本就是 Meyer's Singleton 的写法，这种写法在 C++11 之后是一个线程安全（有了内存模型和顺序）的延迟单例写法。这里回顾一下单例模式在 C++ 中的各种实现

### 单线程写法

```cpp
template<typename T>
class Singleton {
  public:
  static T& instance() {
    if (!value) {
      value_ = new T();
    }
    return *value_;
  }
  private:
  Singleton();
  ~Singleton();
  static T* value_;
};

template<typename T>
T* Singleton<T>::value_ = nullptr;
```

多线程中，会出现使用 nullptr 的场景，或者多次初始化

### 用锁

```cpp
template<typename T>
class Singleton {
  public:
  static T& instance() {
    MutexGuard guard(mutex_); // RAII
    if (!value) {
      value_ = new T();
    }
    return *value_;
  }
  private:
  Singleton();
  ~Singleton();
  static T* value_;
  static Mutex mutex_;
};

template<typename T>
T* Singleton<T>::value_ = nullptr;

template<typename T>
T* Singleton<T>::mutex_;
```

但是每次 instance() 都会进入 race condition，加锁，严重影响性能。

### double check locking
```cpp
template<typename T>
class Singleton {
  public:
  static T& instance() {
    if (!value_) {
      MutexGuard guard(mutex_); // RAII
      if (!value) {
        value_ = new T();
      }
    }
    return *value_;
  }
  private:
  Singleton();
  ~Singleton();
  static T* value_;
  static Mutex mutex_;
};

template<typename T>
T* Singleton<T>::value_ = nullptr;

template<typename T>
T* Singleton<T>::mutex_;
```

但是呢，value_ = new T() 这一句分为 3 步

- 分配一个 T 类型对象的内存
- 在分配的内存处构造 T 对象
- 分配的指针赋值给 value_

但是 2，3 步不一定是顺序的，比如 thread A 中执行了 1，3，然后 thread B 中 value_ 没有加锁，直接检查发现有值，直接返回使用，但是这个指针有值，但是没有构造好。这个问题的详细讨论在[这里](https://www.aristeia.com/Papers/DDJ_Jul_Aug_2004_revised.pdf) Scott Meyers 专门写了文章讨论这个问题。

那么可以在 C++11 之前使用 DCL 吗？

```cpp
static T& instance() {
    if (!value_) {
      MutexGuard guard(mutex_); // RAII
      if (!value) {
        T* p = static_cast<T*>(operator new(sizeof(T)));
        new (p) T();
        // insert some memory barier
        value_ = p;
      }
    }
    return *value_;
}
```
在赋值和构造之间加上 [memory barier](https://mechanical-sympathy.blogspot.com/2011/07/memory-barriersfences.html) 就能保证不乱序。

### Meyers Singleton
```cpp
template<typename T>
class Singleton {
  public:
  static T& instance() {
    static T value;
    return value;
  }
  private:
  Singleton();
  ~Singleton();
};
```

C++11 之后定义了 local static 变量的内存模型，可以保证一个线程在初始化一个变量时，其他线程必须等待完成才能访问。[C++11](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2012/n3337.pdf) 的 6.7 中

> If control enters the declaration concurrently while the variable is being initialized, the concurrent execution shall wait for completion of the initialization.

在 [stackoverflow](http://stackoverflow.com/questions/1661529/is-meyers-implementation-of-singleton-pattern-thread-safe) 中有讨论

### Atomic Singleton
```cpp
template<typename T>
class Singleton {
  public:
  static T& instance() {
    while(true) {
      if (ready_.get()) {
        return *value_;
      } else {
        if (initializing_.get_and_set(true)) {
          // another thread is initializing, waiting in circulation
        } else {
          value_ = new T();
          ready_.set(true);
          return *value_;
        }
      }
    }
  }
  private:
  Singleton();
  ~Singleton();
  static T* value_;
  static Atomic<bool> ready_;
  static Atomic<bool> initializing_;
};
template<typename T>
T* Singleton<T>::value_ = nullptr;

template<typename T>
Atomic<bool> Singleton<T>::ready_(false);

template<typename T>
Atomic<bool> Singleton<T>::initializing_(false);

```

原则上就是区分三种状态：

- 对象已经构造完成
- 对象还没构造完成，某个线程正在构造中
- 对象还没构造完成，没有线程正在构造中

### phread_once

使用 Unix 平台，使用 pthread_once 实现

> ```int pthread_once(pthread_once_t *once_control, void (*init_routine)(void))```

APUE 中提到，如果每个线程都调用 pthread_once，系统保证 init_routine 只被调用一次

```cpp
template<typename T>
class Singleton {
  public:
  static T& instance() {
    threads::pthread_once(&once_control_, init);
    return *value_;
  }
  private:
  Singleton();
  ~Singleton();
  static void init() {
    value_ = new T();
  }
  static T* value_;
  static pthread_once_t once_control_;
};
template<typename T>
T* Singleton<T>::value_ = nullptr;

template<typename T>
pthread_once_t Singleton<T>::once_control_ = PTHREAD_ONCE_INIT;

```

### 恶汉方式
直接构造，在 main 之后使用就没有问题了，然后关于 static storage C++ 的保证

> The storage for objects with static storage duration (basic.stc.static) shall be zero-initialized (dcl.init) before any other initialization takes place. Zero-initialization and initialization with a constant expression are collectively called static initialization; all other initialization is dynamic initialization. Objects of POD types (basic.types) with static storage duration initialized with constant expressions (expr.const) shall be initialized before any dynamic initialization takes place. Objects with static storage duration defined in namespace scope in the same translation unit and dynamically initialized shall be initialized in the order in which their definition appears in the translation unit.

我们看到在同一个 tranlation uint 中，初始化顺序与定义顺序相同。不同 translation unit 之间的顺序无定义，就需要第二节说的各种方式来规避可能出现的问题
