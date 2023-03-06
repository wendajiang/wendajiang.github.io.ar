---
template: blog/page.html
date: 2023-01-11 15:43:29
title: One dynamic library problem
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["so"]
extra:
  mermaid: true
  usemathjax: true
  lead: library link, export symbol control

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

记一次库连接的问题。

## 问题
场景是这样的
```
1. liba.so link the grpc++.a
2. binary b link the liba.so and the grpc++.a
```

因为 link 顺序的导致 binary b 优先使用 liba.so 中有的 grpc++.a symbol，liba.so 中没有的才去 link grpc++.a，这导致一些比较微妙的运行时问题，而且是偶现的

所以还是对于写动态库没有经验，这个问题说明要控制动态库 visibility 的 symbol

## 控制 symbol visibility 的方法

### 1. -fvisibility=hidden
在代码中加入 
```cpp
void __attribute((visibility("default")))__ foo();
```
然后编译时使用 `-fvisibility=hidden` 选项就可以只暴露 foo 这种 API

但是这个方法不适合开始介绍的问题，因为 grpc++.a 中的 symbol visibility 不受我们的代码控制

### 2. -Wl,--exclude-libs,...
ld 中有 `--exclude-libs` 这个选项，可以 man 查看详情，会移除 archive 中 automatic export 的符号，经实验，使用前 grpc 相关的符号有 4000，使用后 grpc 相关的符号 300，还是很有用

### 3. ld version-script
最强控制 export，参考 [script 写法](https://sourceware.org/binutils/docs/ld/VERSION.html)，甚至支持 C++ mangle 前的写法
```bash
{
  global:
   extern "C++" {
    foo::zoo::*;
   };
   moo;
}
```
liba.so 只会暴露 script 指定的 symbol，grpc public 的只剩下两个

![image-20230111174331686](https://wendajiang.github.io/pics/one_dynamic_library_problem/image-20230111174331686.png)

## 番外

```cpp
void foo(std::string p) {
  std::cout << "foo:" << p << std::endl;
}

void foo(std::string p) asm("rename");
extern "C" {
  void rename(std::string p);
}

int main() {
  rename("test");
  return 0;
}
```

会发生什么？调用到了 foo 函数！

## reference
### 使用到的 research 工具
```bash
nm -C -g liba.so
readelf -a liba.so | rg RPATH
chrpath liba.so -r <new path> 
  or use patchelf, but do not this time
readelf -a liba.so | rg -i RPATH
```

### link
- https://blog.csdn.net/found/article/details/105263450
- https://holtstrom.com/michael/blog/post/437/Shared-Library-Symbol-Conflicts-%28on-Linux%29.html
- https://stackoverflow.com/questions/8432431/ld-script-for-symbol-hidding-in-c
- https://www.akkadia.org/drepper/dsohowto.pdf
- https://jdhao.github.io/2020/02/16/ripgrep_cheat_sheet/
