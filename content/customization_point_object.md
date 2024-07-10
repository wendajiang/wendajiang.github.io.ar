---
title: cusomization point object
description: ''
template: blog/page.html
date: 2023-12-21 14:54:34
updated: 2023-12-21 14:54:34
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: []
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

Derivation:

When imitate format code liking fmt library for SystemVerilog $display, using gcc9.2.0 compile the code, throw compile error,
the ambiguous error the `to_string_view` which in fmt namespace and svfmt namespace.

So to fix this, I search the result of ADL mechanism.

# ADL

First, we should introduce the principle of ADL. Argument-dependent lookup, also known as ADL, or Koenig lookup, is the set of rules for looking up the unqualified function names in function-call expressions, including implicit function calls to overloaded operators. These function names are looked up in the namespaces of their arguments in addition to the scopes and namespaces considered by the usual unqualified name lookup.

ADL makes it possible to use operators defined in a different namespace. Example:

```cpp
#include <iostream>
int main() {
  std::cout << "Test\n"; // There is no operator<< in global namespace, but ADL examines std namespace because the left argument is in std and finds std::operator<<(std::ostream&, const char*)
  operator<<(std::cout, "Test\n"); // Same, using function call notation.

  std::cout << endl; // Error: 'endl' is not declared in this namespace. This is not a function call to endl(), so ADL does not apply
  endl(std::cout); // Ok: This is a function call: ADL examines std namespace because the argument of endl is in std, and finds std::endl

  (endl)(std::cout); // Error: 'endl' is not declared in this namespace. The sub-expression (endl) is not an unqualified-id
}
```

Another example:
```cpp
using std::swap;
swap(a, b); // not std::swap(a, b); as if this namespace define the swap, first using this namespace swap, not std::swap.
```

More detailed rules, please see [cpp-reference](https://en.cppreference.com/w/cpp/language/adl).

## Name lookup
[src](https://en.cppreference.com/w/cpp/language/lookup)

For example, to compile `std::cout << std::endl;`, the compiler performs:
- unqualified name lookup for the name std, which finds the declaration of namespace std in the header `<iostream>`
- qualified name lookup for the name cout, which finds a variable declaration in the namespace std
- qualified name lookup for the name endl, which finds a function template declaration in the namespace std
- both argument-dependent lookup for the name operator<<, which finds multiple function template declarations in the namespace std, and qualified name lookup for the name `std::ostream::operator<<`, which find multiple member function declarations in class `std::ostream`

For function and function template names, name lookup can associate multiple declarations with the same name, and may obtain additional declarations from ADL. Template argument deduction may also apply, and the set of declarations is passed to overload resolution, which selects the declaration that will be used. Member access rules, if applicable, are considered only after name lookup and overload resolution.

For all other names (variables, namespaces, classed, etc), name lookup can associate multiple declarations only if they declare the same entity, otherwise it must produce a single declaration in order for the program to compile. Lookup for a name in a scope finds all declarations of that name, with one exception, known as the "struct hack" or "type/non-type hiding": Within the same scope, some occurrences of a name may refer to a declaration of a class/struct/union/enum that is not a typedef, while all other occurrences of the same name either all refer to the same variable, non-static data member, or enumerator, or they all refer to possibly overloaded function or function template names. In this case, there is no error, but the type name is hidden from lookup.

### Types of lookup
If the name appears immediately to the right of the scope resolution operator:: or possibly after :: followed by the disambiguating keyword template, see
####  Qualified name lookup
A qualified name may refer to a
- class member (include static / non-static functions, types, templates, etc).
- namespace member (including another namespace)
- enumerator

Otherwise, see
#### Unqualified name lookup
  - which, for function names, includes Argument-dependent lookup

# CPO
c++20 ranges library import CPO, using function object not function to inhibit ADL.

```cpp
using std::vector;
using namespace std::ranges;

vector<int> a;
find(a.begin(), a.end(), 2);
// should call std::ranges::find
```

- `vector` in namespace `std`, so ADL would find the `std::find`
- `std::find` need `begin/end` have the same type, so `std::find` is more specific to `std::ranges::find`
  and `std::ranges::find` has Concepts, but specific is more high priority to the concepts.

If `std::ranges::find` is function, above code would call `std::find`. So `std::ranges::find` is function object.

But if we just don't want to call it an object. We want call it a function. It became a **niebloid**.

## Niebloid
It's possible tha a future language feature might come around that would allow us to explicity opt functions and functions
templates out of ADL. This would allow an implementation strategy like:

```cpp
namespace ranges {
  template<intput_iterator I, sentinel_for<T> S, weakly_incrementable O>
    requires indirectly_copyable<I, O>
    no_adl constexpr copy_result<I, O> copy(I first, S last, O result);

  template<input_range R, weakly_incrementable O>
    requires indirectly_copyable<iterator_t<R>, O>
    no_adl constexpr copy_result<borrowed_iterator_t<R>, O> copy(R&& r, O result);
}
```

For instance, Matt Calabrese's [CPF](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p1292r0.html) proposal would allow you to declare a function `final` to get this desired ADL-inhibiting behavior.

And the specification is written in a way to allow such future language evolution without having to change anything.
Indeed, it is within the implementation purview today for GCC to do something like add a `__gcc_no_adl` specifier that itself magically inhibits ADL and ends up with `std::ranges::copy` not being an object(although they do not do that today). Which means that while:

```cpp
auto f = std::ranges::begin;
```
is specified to be valid code, the same is not true for:
```cpp
auto g = std::ranges::copy;
```

# [tag_invoke model](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p1895r0.pdf)


# reference
- [cpp reference](https://en.cppreference.com/w/cpp/ranges/cpo)
- [tag invoke and cpo](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2019/p1895r0.pdf)
- 重点：[blog: cpo biebloid](https://brevzin.github.io/c++/2020/12/19/cpo-niebloid/)
- [chinese blog: cpo and biebloid](https://mysteriouspreserve.com/blog/2023/04/18/Cpp-CPO-and-Niebloids/)
- [ADL(argument-dependent lookup)](https://en.cppreference.com/w/cpp/language/adl)
- [C++特殊定制](https://zhuanlan.zhihu.com/p/532859426)
- [why tag_invoke is not the solution I want](https://brevzin.github.io/c++/2020/12/01/tag-invoke/)
- [CPF: proposal replacement of the ADL](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p1292r0.html)
- [Customization Point Design in C++11 and Beyond](http://ericniebler.com/2014/10/21/customization-point-design-in-c11-and-beyond/)
- [what is ADL](https://stackoverflow.com/questions/8111677/what-is-argument-dependent-lookup-aka-adl-or-koenig-lookup)