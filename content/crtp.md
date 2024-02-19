---
title: CRTP
description: ''
template: blog/page.html
date: 2023-06-08 15:45:06
updated: 2023-06-08 15:45:06
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ['templates', 'cpp']
extra:
  mermaid: true
  usemathjax: true
  lead: 'The Curiously Recurring Template Pattern (CRTP) is a C++ idiom whose name was coined by James Coplien in 1996, in early C++ template code.'

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

# What CRTP is 
The CRTP consists in:
- inheriting from a template class
- use the derived class itself as a template parameter of the base class

This is what it looks like in code:
```cpp
template <typnema T>
class Base {
  
};

class Derived: public Base<Derived> {

};
```
The purpose of doing this is using the derived class in the base class. From the perspective of the base object, the derived object is itself, but downcasted. Therefore the base class can access the derived class by `static_casting` itself into the derived class.

```cpp
template<typnename T>
class Base {
  public:
    void do_something() {
      T &derived = static_cast<T&>(*this);
      use derived...
    }
};
```
## What could go wrong
If two classes happen to derive from the same CRTP base class we likely get to undefined behaviour when the CRTP will try to use the wrong class:
```cpp
class Derived1: public Base<Derived1> {}
class Derived2: public Base<Derived1> {} // bug in this line of code
```
There is solution to prevent this, that has been proposed by [fluentcpp comment](https://www.fluentcpp.com/2017/05/12/curiously-recurring-template-pattern/). It consists in adding a private constructor in the base class, and making the base class friend with the templated class.
```cpp
template<typename T>
class Base {
  public:
    //
  private:
    Base() {}
    friend T;
};
```

Indeed, the constructors of the derived class have to call the constructor of the base class. Since the ctor in the base class is private, no one can access it except the friend classes. And the only friend class is ... the template class! So if the derived class is different from the template class, The code doesn't compile.


# What the CRTP can bring to your code.
## static(compile-time) polymorphism (vs dynamic polymorphism)

```cpp
template<typename T>
class Amount {
  public:
    double get_value() const {
      return static_cast<T const&>(*this).get_value();
    }
};

class Constant42: public Amount<Constant42> {
  public:
    double get_value() const {return 42;}
};
class Variable: public Amount<Variable> {
  public:
    explicit Variable(int v) :_value(v) {}
    double get_value() const { return _value; }
  private:
    int _value;
};

// client

template<typename T>
void print(Amount<T> const& amout) {
  std::cout << amount.get_value() << "\n";
}

Constant42 c42;
print(c42);
Variable v(43);
print(v);
// 42
// 43
```

## CRTP as a delegation pattern(static interface)

For example, auto generate the != operator
```cpp
template<typename D> struct not_euqal {
  friend bool operator!=(const D& lhs, const D& rhs) {
    return !(lhs == rhs);
  }
};

class C: public not_euqal<C> {
  int _i;
  public:
    C(int i) :_i(i) {}
    friend bool operator==(const C& lhs, const C& rhs) {
      return lhs._i == rhs._i;
    }
};
```


# ref
- [fluentcpp](https://www.fluentcpp.com/2017/05/16/what-the-crtp-brings-to-code/)
- Hands-on Design Patterns with C++ Chapter 8
