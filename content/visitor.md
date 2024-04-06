---
title: visitor pattern
description: ''
template: blog/page.html
date: 2024-04-06 21:36:41
updated: 2024-04-06 21:36:41
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["visitor", "design pattern"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'Visitor design pattern explanation.'

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

# what is visitor pattern
A pattern that separates the algorithm from the object structure, which is the data for the algorithm. Using the visitor pattern, we can add a new operation to the class hierarchy without modifying the classes themselves.

Different, more technical way to describe the visitor pattern is to say that it implements *double dispatch*. Let's explain the term.

```cpp
class Base {
  virtual void f() = 0;
};
class D1 : public Base {
  void f() override;
};
class D2: public Base {
  void f() override;
};
```
The regular virtual function calls, if we invoke the `b->f()`  virtual function through a pointer to the base class, the call is dispatched to `D1::f()` or `D2::f()`, depending on the real type of the object. This is the **single dispatch**. Now let's assume that the function `f()` also take an argument that is the pointer to a base class:
```cpp
class Base {
  virtual void f(Base *p) = 0;
};
class D1 : public Base {
  void f(Base *p) override;
};
class D2: public Base {
  void f(Base *p) override;
};
```
This would be **double dispatch**.

# why we need visitor pattern
Why would we want to add an operation externally instaed of implementing it in every class in herarchy class-tree? Consider the example of the serialization/deserialization problem.

For example, we may need to write an object into a memory buffer, to be transmitted across the network and decerialized on another machine. Alternatively, we may need to save the object to disk, or else we may need to convert all objects in a container to a markup format such as JSON.

1. The straighforward approach would have us add a serialization and a deserialization method to every object for every serialization machanism. If a new and different serialization approach is needed, we have to go over the entire class hierarchy and add support for it.
2. An alternative is to implement the entire serialization/deserialization operation in a separate function that can handle all classes. The resulting code is a loop that iterates over all objects, with a large decision tree inside of it. The code must interrogate every object and determine its type, for example, using dynamic casts. When a new class is added to the hierarchy, all serialization and deserialization implementations must be updated to handle the new objects.

Both are difficult to maintain for large hierarchies. The visitor pattern offers a solution.

# visitor pattern
```cpp
class Pet {
  public:
  virtual ~Pet() {}
  Pet(std::string& color): color_(color) {}
  const std:string& color() const {
    return color_;
  }
  private: 
  const std::string color_;
};
class Cat: public Pet {
  public:
  Cat(std::string& color): Pet(color) {}
};
class Dog: public Pet {
  public:
  Dog(std::string& color): Pet(color) {}
};
```
Now we want to add some operations to ousr classes, such as "feed the pet" or "play with the pet"

```cpp
class Cat;
class Dog;
class PetVisitor {
  public:
  virtual void visitor(Cat* c) = 0;
  virtual void visitor(Dog* d) = 0;
};
```
We need to make the `Pet` hierarchy visitable, which means we do need to modify it, but only once, regradless of how many operations we want to add later.
```cpp
class Pet {
  public:
  virtual void accept(PetVisitor& v) = 0;
  ...
};
class Cat {
  public:
  void accept(PetVisitor& v) override {
    v.visit(this);
  }
  ...
};
class Dog {
  public:
  void accept(PetVisitor& v) override {
    v.visit(this);
  }
  ...
};
```
Now out `Pet` hierarchy is visitable, and we have an abstract `PetVisitor` class. Everything is ready to implement new operations for our classes.
```cpp
class FeedingVisitor: public PetVisitor {
  public: 
  void visit(Cat *c) override {
    std::cout << "Feeding to the " << c->color() << "cat" << std::endl;
  }
  void visit(Dog *d) override {
    std::cout << "Feeding to the " << d->color() << "dog" << std::endl;
  }
};
class PlayingVisitor: public PetVisitor {
  public: 
  void visit(Cat *c) override {
    std::cout << "Playing with the " << c->color() << "cat" << std::endl;
  }
  void visit(Dog *d) override {
    std::cout << "Playing with the " << d->color() << "dog" << std::endl;
  }
};
```

The call `accept()` ends up dispatched to a particular `visit()` function based on two factors - the type of the visitable `*p` object and the type of the `*v` visitor. Stress the aspect of the visitor pattern, we can write code like this:

```cpp
void dispatch(Pet& p, PetVisitor& v) {
  p.accept(v);
}
std::unique_ptr<Pet> p = ...;
std::unique_ptr<PetVisitor> v = ...;
display(*p, *v); // double dispatch
```

## visit complex objects
The correct way to handle the component objects is to simply visit each one, and thus delegate the problem to someone else.
