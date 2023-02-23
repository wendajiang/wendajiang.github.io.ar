+++
template = "blog/page.html"
date = "2021-06-23 18:59:13"
title = "namespace 一知半解"
[taxonomies]
tags = ["cpp"]

[extra]
mermaid = true
usemathjax = true
+++
<!--
mermaid example:
<div class="mermaid">
    mermaid program
</div>
-->

## 基本使用
1. namespace ns_name { declarations }
命名空间定义
2. inline namespace ns_name { declarations } (since C++11)
类似于 class private 成员，只有在定义的 namespace 中才可见
3. namespace { declarations } 
   匿名命名空间类似声明 static 函数，只在本编译单元(文件)内可见
4. ns_name::name
   命名空间内成员的使用
5. using namspace ns_name;
   using-directive 将 ns_name 中的名字提升到使用这个语句的 scope 
6. using ns_name::name;
   using-declaration 将 ns_name 中的 name 提升到这个语句的 scope
7. namespace name = qualified-namespace;
   命名空间的别名，嵌套很深的命名空间可以这样方便使用
8. namespace ns:name::inline(since C++20)(optional) name {declarations } (since C++17)
   嵌套定义
   `namespace A::B::C {...}` 等于 `namespace A{ namespace B { namespace C {...} } }`
   `namespace A::B::inline C {...}` 等于 `namespace A::B{inline namespace C {...} } `
   `inline` 要在除了第一个 namespace 的前面

## 注意

- 命名空间必须在全局命名空间或者其他命名内定义，*比如不能在函数内定义*

- 命名空间的扩展 *extension-namespace-definition*

  ```cpp
  namespace A {
  	int a = 0;
  }
  
  namespace A {
    int b = 0;
  }
  ```

- 命名空间成员的定义会影响 name-lookup

  

  
### Using-declarations

Introduce a name that is defined elsewhere into the declarative region where this using-declaration appears

1. using name 将 name 引入到特定命名空间，**有点类似 Rust 模块系统的模块提升**

   ```cpp
   void f();
   namespace A {
       void g();
   }
    
   namespace X {
       using ::f;        // global f is now visible as ::X::f
       using A::g;       // A::g is now visible as ::X::g
       using A::g, A::g; // (C++17) OK: double declaration allowed at namespace scope
   }
    
   void h()
   {
       X::f(); // calls ::f
       X::g(); // calls A::g
   }
   ```

2. using 之后 reopen 命名空间的内容不生效

   ```cpp
   namespace A {
       void f(int);
   }
   using A::f; // ::f is now a synonym for A::f(int)
    
   namespace A {     // namespace extension
       void f(char); // does not change what ::f means
   }
    
   void foo() {
       f('a'); // calls f(int), even though f(char) exists.
   }
    
   void bar() {
       using A::f; // this f is a synonym for both A::f(int) and A::f(char)
       f('a');     // calls f(char)
   }
   ```

3. 不能引入命名空间，只能引入命名空间内的一个名字，并且在同一个 scope 内的所有限制都一样

   ```cpp
   namespace A {
       int x;
   }
    
   namespace B {
       int i;
       struct g { };
       struct x { };
       void f(int);
       void f(double);
       void g(char); // OK: function name g hides struct g
   }
    
   void func() {
       int i;
       using B::i;   // error: i declared twice
    
       void f(char);
       using B::f;   // OK: f(char), f(int), f(double) are overloads
       f(3.5);       // calls B::f(double)
    
       using B::g;
       g('a');       // calls B::g(char)
       struct g g1;  // declares g1 to have type struct B::g
    
       using B::x;
       using A::x;   // OK: hides struct B::x
       x = 99;       // assigns to A::x
       struct x x1;  // declares x1 to have type struct B::x
   }
   
   //-------- 分隔
   namespace B {
       void f(int);
       void f(double);
   }
    
   namespace C {
       void f(int);
       void f(double);
       void f(char);
   }
    
   void h() {
       using B::f;  // introduces B::f(int), B::f(double)
       using C::f;  // introduces C::f(int), C::f(double), and C::f(char)
       f('h');      // calls C::f(char)
       f(1);        // error: B::f(int) or C::f(int)?
       void f(int); // error: f(int) conflicts with C::f(int) and B::f(int)
   }
   ```

### Using-directives

Using-directives are allowed only in namespace [scope](https://en.cppreference.com/w/cpp/language/scope) and in block scope

Using-directive does not add any names to the declarative region in which it appears (unlike the using-declaration), and thus does not prevent identical names from being declared.

```cpp
namespace D {
    int d1;
    void f(char);
}
using namespace D; // introduces D::d1, D::f, D::d2, D::f,
                   //  E::e, and E::f into global namespace!
 
int d1;            // OK: no conflict with D::d1 when declaring
namespace E {
    int e;
    void f(int);
}
 
namespace D {          // namespace extension
    int d2;
    using namespace E; // transitive using-directive
    void f(int);
}
 
void f() {
    d1++;    // error: ambiguous ::d1 or D::d1?
    ::d1++;  // OK
    D::d1++; // OK
    d2++;    // OK, d2 is D::d2
    e++;     // OK: e is E::e due to transitive using
    f(1);    // error: ambiguous: D::f(int) or E::f(int)?
    f('a');  // OK: the only f(char) is D::f(char)
}

```





## 一些例子

```cpp
namespace Q {
    namespace V {   // V is a member of Q, and is fully defined within Q
// namespace Q::V { // C++17 alternative to the above two lines
        class C { void m(); }; // C is a member of V and is fully defined within V
                               // C::m is only declared
        void f(); // f is a member of V, but is only declared here
    }
 
    void V::f() // definition of V's member f outside of V
                // f's enclosing namespaces are still the global namespace, Q, and Q::V
    {
        extern void h(); // This declares ::Q::V::h
    }
 
    void V::C::m() // definition of V::C::m outside of the namespace (and the class body)
                   // enclosing namespaces are the global namespace, Q, and Q::V
    {}
}
```

