---
title: Type Erasure
description: ''
template: blog/page.html
date: 2024-02-20 08:58:12
updated: 2024-02-20 08:58:12
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["design pattern", "cpp", "GoF", "llvm"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---
# What is type erasure
Type erasure, in general, is a programming technique by which the explicit type information is removed from the program. It is a type of abstraction that ensures that the program does not explicitly depend on some of the data types.

The aim here is to increase the level of abstraction -- instead of writing some type-specific code, perhaps serveral versions of it for different types, we can write just one version that is more abstract, and expressed the concept -- for example, instead of writing a function whose interface expresses the concept *sort an array of integers*, we want to write a more abstract function, *sort any array*.

## From example to generalization
In c++, `std::function` is a general-purpose polymorphic function wrapper, or a general callable object. It's used to store any callable entity such as a function, a lambda expression, a functor(an object with the operator()), or a memeber function pointer.

```cpp
std::function<size_t(const std::string&)> f;
size_t f1(const std::string& s) {
  return s.capacity();
}
f = f1;
std::cout << f("abcde"); // 15
char c = 'b';
f = [=](const std::string& s) { return s.find(c); }
std::cout << f("abcde"); // 1
f = &std::string::size;
std::cout << f("abcde"); // 5
```
We can now see type erasure in its more general form: it is an abstraction for many different implementations that all provide the same behavior.

# Type erasue as a design pattern?
type erasure offers an non-intrusive way to separate the interface from the implementation. It's different from inheritance. We can say that type erasure provides "external polyorphism": there is no unifying hierarchy required, and the set of types that can be used to implement a particular abstraction is extensible, not limited to just classes derived from a common base.

## Why doesn't type erasure completely replace inheritance in C++?
1. performance, high-performance implementations of type erasure became available only recently.
2. convenience.

# Type erasure as an implementation technique
Type erasure is a great tool for breaking dependencies between components of a large system.

For example, we are building a large distributed software system, so one of our core components it the network communication layer:
```cpp
class Network {
  ...
  void send(const char* data);
  void receive(const char* buffer);
  ...
}
```
Now, in one specific application, we have a need to process the data packets before and after they are sent across the network:
```cpp
class Network {
  ...
  bool needs_processing;
  void send(const char* data) {
    if (needs_processing) apply_processing(buffer);
    ...
  }
  ...
}
```
Is it a dependency nightmare: the low-level library now has to built with the `apply_processing` function from the specific application. Even worse, all other programs that do not require this functionality must still be compiled and linked with this code, even if they never set `needs_processing`.

While this problem can be handled the 'old school' way - with some function pointer or (worse) global variables, type erasure offers an elegant solution:
```cpp
class Network {
  static const char* default_processor(const char* data) {
    std::cout << "default processing" << std::endl;
    return data;
  }
  std::function<const char*(const char*)> processor = default_processor;
  void send(const char* data) {
    data = processor(data);
    ...
  }
  public:
  template<typename F>
  void set_processor(F&& f) {
    processor = f;
  }
};
```

This is an example of the strategy design pattern, where the implementation of a particular behavior can be chosen at run-time.

# How is type erasure implemented in C++?
## Very old type erasure
```c
int less(const void* a, const void* b) {
  return *(const int*)a - *(const int*)b;
}
int main() {
  int a[10] = {1, 10, 2, 9, 3, 8, 4, 7, 5, 0};
  qsort(a, 10, sizeof(int), less);
}
// void qsort(void *base, size_t nmemb, size_t size, int (*compare)(const void*, const void*));
```
The major downside of this C approach is that the programmer is wholly responsible for ensuring that all the pieces of the type-erased code are consistent.

In C++, we can do a lot better, but the idea is still the same: the erased type is reified by the implementation of some type-specific code that is invoked through the type-agnostic interface. The key difference is that we are going to force the compiler to generate this code for use. There are, fundamentally, two techniues that can be used. The first one relies on run-time polymorphism(inheritance), and the second one uses template magic.

## Type erasure using inheritance
```cpp
template<typename T>
class smartptr {
  struct destroy_base {
    virtual void operator()(void*) = 0;
    virtual ~destroy_base() {}
  }
  template<typename Deleter>
  struct destory: public destory_base {
    destroy(Deleter d): d_(d) {}
    void operator()(void* p) override {
      d_(static_cast<T*>(p));
    }
    Deleter d_;
  }
  public:
  template<typename Deleter> smartptr(T* p, Deleter d):
    p_(p), d_(new destroy<Deleter>(d)) {}

    ~smartptr() {
      (*d_)(p_); delete d_;
    }
    T* operator->() { return p_; }
    const T* operator->() const {return p_;}
  private:
  T* p_;
  destroy_base* d_;
}
```

Befor we learn the other (usually more efficient) way to implement type erasure, we have to address one glaringly obvious inefficiency in our design: every time we create or delete a shared pointer or a `std::function` object that is implemented as described above, we must allocate and deallocate memory for the derived object that conceals the erased type.

### Type erasue without memory allocation
```cpp
template<typename T>
class smartptr {
  ...
  public:
    template<typename Deleter>
    smartptr(T*p, Deleter d): p_(p) {
      static_assert(sizeof(Deleter) <= sizeof(buf_));
      ::new (static_cast<void*>(buf_)) destroy<Deleter>(d);
    }
    ~smartptr() {
      destroy_base* d = (destroy_base*)buf_;
      (*d)(p_);
      d->~destroy_base();
    }
  private:
    T* p_;
    alignas(8) char buf_[16];
}
```

## Type erasure without inheritance
```cpp
template<typename T>
class smartptr_te_static {
  T* p_;
  using destroy_t = void(*)(T*, void*);
  destroy_t destroy_;
  alignas(8) char buf_[8];
  template<typename Deleter>
  static void invoke_destroy(T*p, void* d) {
    (*static_cast<Deleter*>(d))(p);
  }

  public:
  template<typename Deleter>
  smartptr_te_static(T* p, Deleter d): p_(p), destroy_(invoke_destroy<Deleter>) {
    static_assert(sizeof(Deleter) <= sizeof(buf_));
    ::new (static_cast<void*>(buf_)) Deleter(d);
  }
  ~smartptr_te_static() {
    this->destroy_(p_, buf_);
  }
  T* operator->() {return p_;}
  const T* operator->() const { return p_; }
};
```
The function template that remains the information about the erased type is `invoke_destroy()`, note that is a static function.

We can use `vtable` to sim inheritance to avoid the smartptr bloat.

```cpp
template<typename T>
class smartptr_te_vtable {
  T* p_;
  struct vtable_t {
    using destroy_t = void(*)(T*, void*);
    using destructor_t = void(*)(void*);
    destroy_t destroy_;
    destructor_t dtor_;
  };
  const vtable_t* vtable_ = nullptr;
  template<typename Deleter>
  constexpr static vtable_t vtable = {
    smartptr_te_vtable::template destroy<Deleter>,
    smartptr_te_vtable::template destructor<Deleter>
  };
  template<typename Deleter>
  static void destroy(T*p, void* d) {
    (*static_cast<Deleter*>(d))(p);
  }
  template<typename Deleter>
  static void destructor(void*d) {
    static_cast<Deleter*>(d)->~Deleter();
  }
  alignas(8) char buf_[8];
  public:
  template<typename Deleter>
  smartptr_te_vtable(T*p, Deleter d): p_(p), vtable_(&vtable<Deleter>) {
    static_assert(sizeof(Deleter) <= sizeof(buf_));
    ::new (static_cast<void*>(buf_)) Deleter(d);
  }
  ~smartptr_te_vtable() {
    this->vtable_->destroy_(p_, buf_);
    this->vtable_->dtor_(buf_);
  }
  T* operator->() {return p_;}
  const T* operator->() const {return p_;}
}
```

# Example in real world

LLVM new `PassManager` manage new pass (`PassInfoMixin` template pass) by type erase( concepts and model)

The code location is at 
```markdown
llvm-project
 └─llvm
    └─include
       └─llvm
          └─IR
             ├─PassManager.h
             ├─PassManagerInternal.h(passconcept and passmodel)
             └─PassManagerImpl.h (run or addPass)

```


# reference
- [concepts and model(template) in one class that represent the concept](https://davekilian.com/cpp-type-erasure.html)
- C++ template - the complete guide 2nd ediditon chapter 22
- andrzej's series on type erasure [part1](https://akrzemi1.wordpress.com/2013/11/18/type-erasure-part-i/) [part2](https://akrzemi1.wordpress.com/2013/12/06/type-erasure-part-ii/) [part3](https://akrzemi1.wordpress.com/2013/12/11/type-erasure-part-iii/) [part4](https://akrzemi1.wordpress.com/2014/01/13/type-erasure-part-iv/)