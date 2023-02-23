---
template: blog/page.html
date: 2022-12-22 16:39:52
title: Cpp Iterator
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["C++", "iterator"]
extra:
  mermaid: true
  usemathjax: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

C++17 has deprecated a few components that has been in C++ since its beginning, and std::iterator is one of them.

首先看一下如何使用 std::iterator ，以及为什么要废弃，然后替代品是什么

## Iterator traits

generic code 使用 iterator，比如 STL 算法强依赖 iterator，需要其中的信息，比如 value_type

```cpp
std::vector<int> numbers = {1, 2, 3, 4, 5};
std::cout << std::reduce(begin(numbers), end(numbers)) << "\n";
```

`std::reduce`使用两个 iterator， 然后输出这个区间中元素的求和。那么 `std::reduce` 如何获取 iterator 元素的类型呢。事实上，这与 `std::vector` 没有关系，iterator 中有 value_type 表示元素类型。然后 iterator 还需要表示是不是可以随机访问，是不是只能递增不能递减等，通过 tag dispatch 实现 iterator_category:

- std::input_iterator_tag
- std::forward_iterator_tag
- std::bidirectional_iterator_tag
- std::random_access_iterator_tag

最后，STL iterator 还需要

- difference_type: - 操作结果的类型
- pointer： 元素的指针类型
- reference：元素的引用类型

如果元素就是指针呢，所以还需要 `std::iterator_traits<Iterator>::value_type` 消除指针

## std::iterator

It is a helper to define the iterator traits of an iterator.
```cpp
template<typename Category,
  typename T,
  typename Distance = std::ptrdiff_t,
  typename Pointer = T*,
  typename Reference = T&> struct iterator;
```

比如可以这样用

```cpp
class MyIterator: public std::iterator<std::random_access_iterator, int> {}
```

## 为什么废弃
至少有一个问题，使用上是不明确的

```cpp
class MyIterator: public std::iterator<std::random_acess_iterator, int, int, int*, int&> {}
```
并不能清晰表明，哪里是 value_type，哪里是 reference_type

更清晰的方式是直接这样写

```cpp
class MyIterator {
  public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = int;
    using difference_type = int;
    using pointer = int*;
    using reference = int&;
}
```

这个原因已经可以说服委员会废弃，不过还有一个缺陷，你无法直接访问 base struct 中的 value_type

```cpp
class MyIterator: public std::iterator<std::random_acess_iterator, int, int, int*, int&> {
  base_type a; // error
}
```

## reference

https://www.fluentcpp.com/2018/05/08/std-iterator-deprecated/

