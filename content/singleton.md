---
title: singleton
description: ''
template: blog/page.html
date: 2021-05-07 12:12:17
updated: 2023-06-15 14:24:10
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["singleton", "thread safe"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'The singleton design pattern'

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

The singleton is a pattern that's used to restrict the number of class instantiations to exactly one.

## thread-safety

在拥有**共享数据**的多线程执行程序中，线程安全代码会通过同步机制保证各个线程正确执行，不会出现数据污染等情况

## how to guarantee thread-safety

1. 给**共享资源**加锁，保证每个资源变量至多被一个线程占用
2. 线程 local 变量

## singleton

单例模式指在整个系统生命周期里，保证一个类只能产生一个实例，确保该类的**唯一性**。

### 单例模式分类

单例模式可以分为**懒汉式**和**饿汉式**，两者之间的区别在于**创建实例的时间不同**：

- **懒汉式**：指系统运行中，实例并不存在，只有当需要使用该实例时，才会去创建并使用实例。**（这种方式要考虑线程安全）**
- **饿汉式**：指系统一运行，就初始化创建实例，当需要时，直接调用即可。**（本身就线程安全，没有多线程的问题）**

### 单例类特点

- 构造函数和析构函数为** private **类型，目的**禁止**外部构造和析构
- 拷贝构造和赋值构造函数为** private **类型，目的是**禁止**外部拷贝和赋值，确保实例的唯一性
- 类里有个获取实例的**静态函数**，可以全局访问

### 普通懒汉单例（线程不安全）

```cpp
///////////////////  普通懒汉式实现 -- 线程不安全 //////////////////
#include <iostream> // std::cout
#include <mutex>    // std::mutex
#include <pthread.h> // pthread_create

class SingleInstance
{

public:
    // 获取单例对象
    static SingleInstance *GetInstance();

    // 释放单例，进程退出时调用
    static void deleteInstance();
	
	// 打印单例地址
    void Print();

private:
	// 将其构造和析构成为私有的，禁止外部构造和析构
    SingleInstance();
    ~SingleInstance();

    // 将其拷贝构造和赋值构造成为私有函数，禁止外部拷贝和赋值
    SingleInstance(const SingleInstance &signal);
    const SingleInstance &operator=(const SingleInstance &signal);

private:
    // 唯一单例对象指针
    static SingleInstance *m_SingleInstance;
};

//初始化静态成员变量
SingleInstance *SingleInstance::m_SingleInstance = NULL;

SingleInstance* SingleInstance::GetInstance()
{

	if (m_SingleInstance == NULL)
	{
	    m_SingleInstance = new (std::nothrow) SingleInstance;  // 没有加锁是线程不安全的，当线程并发时会创建多个实例
	}

    return m_SingleInstance;
}

void SingleInstance::deleteInstance()
{
    if (m_SingleInstance)
    {
        delete m_SingleInstance;
        m_SingleInstance = NULL;
    }
}

void SingleInstance::Print()
{
	std::cout << "我的实例内存地址是：" << this << std::endl;
}

SingleInstance::SingleInstance()
{
    std::cout << "构造函数" << std::endl;
}

SingleInstance::~SingleInstance()
{
    std::cout << "析构函数" << std::endl;
}
///////////////////  普通懒汉式实现 -- 线程不安全  //////////////////

// 线程函数
void *PrintHello(void *threadid)
{
    // 主线程与子线程分离，两者相互不干涉，子线程结束同时子线程的资源自动回收
    pthread_detach(pthread_self());

    // 对传入的参数进行强制类型转换，由无类型指针变为整形数指针，然后再读取
    int tid = *((int *)threadid);

    std::cout << "Hi, 我是线程 ID:[" << tid << "]" << std::endl;

    // 打印实例地址
    SingleInstance::GetInstance()->Print();

    pthread_exit(NULL);
}

#define NUM_THREADS 5 // 线程个数

int main(void)
{
    pthread_t threads[NUM_THREADS] = {0};
    int indexes[NUM_THREADS] = {0}; // 用数组来保存 i 的值

    int ret = 0;
    int i = 0;

    std::cout << "main() : 开始 ... " << std::endl;

    for (i = 0; i < NUM_THREADS; i++)
    {
        std::cout << "main() : 创建线程：[" << i << "]" << std::endl;
        
        indexes[i] = i; //先保存 i 的值
		
        // 传入的时候必须强制转换为 void* 类型，即无类型指针
        ret = pthread_create(&threads[i], NULL, PrintHello, (void *)&(indexes[i]));
        if (ret)
        {
            std::cout << "Error: 无法创建线程，" << ret << std::endl;
            exit(-1);
        }
    }

    // 手动释放单实例的资源
    SingleInstance::deleteInstance();
    std::cout << "main() : 结束！" << std::endl;
	
    return 0;
}

```

### 加锁的懒汉单例（线程安全）

```cpp
///////////////////  加锁的懒汉式实现  //////////////////
class SingleInstance
{

public:
    // 获取单实例对象
    static SingleInstance *&GetInstance();

    //释放单实例，进程退出时调用
    static void deleteInstance();
	
    // 打印实例地址
    void Print();

private:
    // 将其构造和析构成为私有的，禁止外部构造和析构
    SingleInstance();
    ~SingleInstance();

    // 将其拷贝构造和赋值构造成为私有函数，禁止外部拷贝和赋值
    SingleInstance(const SingleInstance &signal);
    const SingleInstance &operator=(const SingleInstance &signal);

private:
    // 唯一单实例对象指针
    static SingleInstance *m_SingleInstance;
    static std::mutex m_Mutex;
};

//初始化静态成员变量
SingleInstance *SingleInstance::m_SingleInstance = NULL;
std::mutex SingleInstance::m_Mutex;

SingleInstance *&SingleInstance::GetInstance()
{

    //  这里使用了两个 if 判断语句的技术称为双检锁；好处是，只有判断指针为空的时候才加锁，
    //  避免每次调用 GetInstance 的方法都加锁，锁的开销毕竟还是有点大的。
    if (m_SingleInstance == NULL) 
    {
        std::unique_lock<std::mutex> lock(m_Mutex); // 加锁
        if (m_SingleInstance == NULL)
        {
            m_SingleInstance = new (std::nothrow) SingleInstance;
        }
    }

    return m_SingleInstance;
}

void SingleInstance::deleteInstance()
{
    std::unique_lock<std::mutex> lock(m_Mutex); // 加锁
    if (m_SingleInstance)
    {
        delete m_SingleInstance;
        m_SingleInstance = NULL;
    }
}

void SingleInstance::Print()
{
	std::cout << "我的实例内存地址是：" << this << std::endl;
}

SingleInstance::SingleInstance()
{
    std::cout << "构造函数" << std::endl;
}

SingleInstance::~SingleInstance()
{
    std::cout << "析构函数" << std::endl;
}
///////////////////  加锁的懒汉式实现  //////////////////

```

### 内部静态变量的懒汉单例（C++11 线程安全）-- Meyer's Singleton

```cpp
///////////////////  内部静态变量的懒汉实现  //////////////////
class Single
{

public:
    // 获取单实例对象
    static Single &GetInstance();
	
	// 打印实例地址
    void Print();

private:
    // 禁止外部构造
    Single();

    // 禁止外部析构
    ~Single();

    // 禁止外部复制构造
    Single(const Single &signal);

    // 禁止外部赋值操作
    const Single &operator=(const Single &signal);
};

Single &Single::GetInstance()
{
    // 局部静态特性的方式实现单实例
    static Single signal;
    return signal;
}

void Single::Print()
{
    std::cout << "我的实例内存地址是：" << this << std::endl;
}

Single::Single()
{
    std::cout << "构造函数" << std::endl;
}

Single::~Single()
{
    std::cout << "析构函数" << std::endl;
}

```

cpp standard 6.7 [stmt.dcl]
> If control ents the declaration concurrently while the variable is being initialized, the concurrent execution shall wait for completion of the initialization.

### 饿汉单例（线程安全）

```cpp
////////////////////////// 饿汉实现 /////////////////////
class Singleton
{
public:
    // 获取单实例
    static Singleton* GetInstance();

    // 释放单实例，进程退出时调用
    static void deleteInstance();
    
    // 打印实例地址
    void Print();

private:
    // 将其构造和析构成为私有的，禁止外部构造和析构
    Singleton();
    ~Singleton();

    // 将其拷贝构造和赋值构造成为私有函数，禁止外部拷贝和赋值
    Singleton(const Singleton &signal);
    const Singleton &operator=(const Singleton &signal);

private:
    // 唯一单实例对象指针
    static Singleton *g_pSingleton;
};

// 代码一运行就初始化创建实例 ，本身就线程安全
Singleton* Singleton::g_pSingleton = new (std::nothrow) Singleton;

Singleton* Singleton::GetInstance()
{
    return g_pSingleton;
}

void Singleton::deleteInstance()
{
    if (g_pSingleton)
    {
        delete g_pSingleton;
        g_pSingleton = NULL;
    }
}

void Singleton::Print()
{
    std::cout << "我的实例内存地址是：" << this << std::endl;
}

Singleton::Singleton()
{
    std::cout << "构造函数" << std::endl;
}

Singleton::~Singleton()
{
    std::cout << "析构函数" << std::endl;
}
////////////////////////// 饿汉实现 /////////////////////

```

### 特点与选择

- 懒汉式是以时间换空间，适应于访问量较**小**时；推荐使用**内部静态变量的懒汉单例**，代码量少
- 饿汉式是以空间换时间，适应于访问量较**大**时，或者线程比较多的的情况

--- update: 2024-01-13 ----

## singleton and shared library
@todo 

## reference
- Hands-on deisng patterns with C++, chapter 15
- [singleton , dlopen and shared library](https://stackoverflow.com/questions/8623657/multiple-instances-of-singleton-across-shared-libraries-on-linux)
- [singleton is dangerous(static variable ctor/dtor order)](https://yanqiyu.info/2023/11/11/singleton/)