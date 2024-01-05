---
template: blog/page.html
date: 2022-11-28 10:20:07
updated: 2024-01-05 10:20:00
title: pimpl
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["C++", "Idiom"]
extra:
  mermaid: true
  usemathjax: true
  toc: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

from [GotW100](https://herbsutter.com/gotw/_100/)

## 1. What is the Pimpl Idiom, and why is it useful ?

C++ 中，头文件中类定义的改变会导致所有用到这个类的地方重新编译--即使修改只涉及到私有成员。这是因为 C++ 的编译模型基于文本包含，以及 C++ 假设调用者知道有关类的两个主要信息，这些信息可能会受到私有成员的影响：

- 尺寸和布局：调用代码必须知道类的尺寸和布局，包含私有数据成员。这种总是能看到的约束增加了调用者和被调者的耦合成本，但是这是 C++ 对象模型和哲学的核心，因为编译器默认可以直接访问对象是时 C++ 可以实现高度优化效率的保证
- 函数：调用者必须可以 resolve 到类的成员函数，包括被非私有函数重载的不可访问的私有函数--如果私有函数匹配度更高，调用代码会编译失败（C++ 采用了慎重的设计决策由于安全原因在访问性检查之前执行重载解决。比如，将函数的属性从私有改为公有不能改变调用代码的合法性）

为了减少这些编译依赖，常用的方法就是使用一个 opaque 指针来隐藏这些实现细节：

```cpp
// Pimpl idiom - basic idea
class widget {
  // :::
  private:
  struct impl;
  impl* pimpl_;
};
```

这种 idiom 的一个好处就是打破了编译时间的依赖。首先，系统构建能运行的更快。其次，可以将代码修改的影响局部化，pimpl 可以自由修改--成员可以自由增加或者删除--不需要重新编译使用端代码。因为对于消除编译级联影响如此有用，所以也经常被称为 compilation firewall.

## 2. What is the best way to express the basic Pimpl Idiom in C++11?

避免使用原始指针以及显式的 delete。pimpl 使用 *unique_ptr*

```cpp
// .h
class widget {
  public:
  widget();
  ~widget();
  private:
  class impl;
  unique_ptr<impl> pimpl;
};

// .cc
class widget::impl {
  
};
widget::widget(): pimpl{new impl{}} {}
widget::~widget() {}
```

这个模式有一些需要注意的：

- 使用 unique_ptr 而不是 shared_ptr，可以正确表达 pimpl 对象不能被分享
- 在实现的 cc 文件中定义和使用 pimpl 对象
- 显式使用构造器，分配 pimpl 对象
- 显式声明 destructor ，即使与编译器生成的一样。因为即使 unique_ptr 和 shared_ptr 都可以使用 incomplete type 实例，unique_ptr 析构需要 complete type 来调用 delete（不像 shared_ptr 在构造时可以获得更多信息）。通过在实现文件中显式定义，强制 impl 也定义了，可以成功避免编译器试图自动生成析构时 impl 没有定义
- 以上的模式 class 没有默认的拷贝和移动语义，因为 C++ 倾向于不默认生成拷贝和移动语义。因为我们显示声明了析构，所以拷贝和移动被关闭了，如果需要有，你需要自己添加定义。

Pimpl 在 C++11 中又了新的优势，就是使用更方便了。

## 3. What parts of the class should go into the *impl* object? Some potential options include:

- put all private data(but not functions) into impl;
  
  需要把 function 也加入
- put all private members into impl;
  1. 不能在 pimpl 中隐藏 virtual member function，即使 virtual function 是私有的。
- put all private and protected members into impl; ❌
- put all private non virtual members into impl; ✅
- put everything into impl, and write the public class itself as only the public interface, each implemented as a simple forwarding function(a handle/body variant)
  
  有些情况可以这样用，避免了 back pointer。主要的缺点是要求一个额外的 wrapper function

关键信息是，在任何 OO 语言中有三个部分：

1. interface for callers = public members. 外部可以访问
2. interface for derivers = protected or virtual members. 派生类可以看到
3. everything else = private and nonvirtual members. 

第三个部分可以放到 pimpl 中隐藏。

## 4. Does the impl require a back pointer to the public object? If yes, what is the best way to provide it? If not, why not?

1. 在 impl 中存储 back pointer
2. 建议：pass *this* 作为参数给 pimpl function。这只会增加栈的消耗

---
2024-1-5 更新
## 5. 关于显式定义 deconstrctor 的补充说明
### default 是不可以的
通过声明 `~T() = default` 在 pimpl 惯用法中是不可行的，会出现编译报错 `incomplete type` 
### 在 desconstructor 中显式调用 `impl_->~T()` 非法
这样会出现两次调用 deconstructor，如果有指针操作就可能会出现 double free

Destructor is meant to be called when an object goes out of scope if the object is in the stack as in this case or called when it is explicitly destructed with delete when the object is created on the heap with new operator at the first place.

There is no way for the compiler or the run time system to keep track whether the destructor is called by you manually or not. Also it is a very bad practice to make a call to the destructor. Except, the *placement new*, when it should be invoked manually.


# reference
- [dtors-faq](https://isocpp.org/wiki/faq/dtors)
- [explicit calling dtor](https://stackoverflow.com/a/14188066/6885532)
- [Manually invoking the dtor in C++: A guide](https://copyprogramming.com/howto/how-to-call-destructor-in-c-manually)
