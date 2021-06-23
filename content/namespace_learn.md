+++
template = "page.html"
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

## 缘起
在读 Google 开源的代码时，有很多封装到很深 nested namespace 
于是在写业务代码时，遇到了 namespace 使用的问题

按照对于 cpp 类的理解对应 namespace，在 `foo.h` 文件中声明
```cpp
class Foo {
public:
    void foo1();
    void foo2();
};
```
在 `foo.cc` 文件中定义

```cpp
void Foo::foo1() {
    foo2(); // correct 可以使用
}

void Foo::foo2() {

}
```

但是对于 namespace 来说，这样是不可以的，在 `ns.h` 中声明
```cpp
namespace foo {
namespace bar {
    void foo1();

    namespace internal {
        void foo2();
    }
}
}
```
在 `ns.cc` 中定义
```cpp
namespace foo {
namespace bar {
    void foo1() {
        internal::foo2(); // error, can't find internal 
    }

    namespace internal {
        void foo2() {}
    }
}
}
```