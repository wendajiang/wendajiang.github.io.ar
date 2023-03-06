+++
template = "blog/page.html"
date = "2021-04-25 22:45:06"
title = "Clean C++ 读书笔记1"
[taxonomies]
tags = ["book note"]

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

## 原则

### KISS
**Keep it Simple and stupid** 

保持代码简单，不要为了炫技而炫技，代码的第一位永远是可读和可维护性

控制代码的复杂度非常重要

### YAGNI
**You Aren't Gonna Need It!**

不要写将来的代码，永远不要假设什么代码将来会需要，需要的时候再写即可

记住重构的力量

### DRY
**Don't repeat yourself**

不要复制代码，否则修改代码要记住修改所有 copy 的位置，基本这就肯定会带来 bug

### Information Hiding
类或者什么暴露给其他人使用的内容尽可能隐藏内部实现，这样调整类或者 API 实现时用户代码尽可能少的修改

优点：
- 模块修改的影响
- 提高模块可用性
- 更易测试

信息隐藏于封装很容易混淆，但是两者不同。信息隐藏是一个设计原则，帮助开发者更好构建模块，原则可以应用于多个层次的抽象，尤其是大型系统

封装是依赖语言的，比如在 C++ 中，你可以将方法使用 private 关键字使之于外部无法访问。封装是信息隐藏的一种实现方式，但是不能保证。

### Strong Cohesion 
强内聚，软件开发中一般建议是任何软件实体(module, component, unit, class, function ...)应该是强内聚的

### Loose Coupling 
低耦合，有个例子

```cpp
class Lamp {
public:
    void on() {}
    void off() {}
};

class Switch {
private:
    Lamp &lamp;
    bool state {false};
public:
    Switch(Lamp &lamp): lamp(lamp) {}

    void toggle() {
        if (state) {
            state = false;
            lamp.off();
        }
        else {
            state = true;
            lamp.on();
        }
    }
};
```

Lamp 与 Switch 是强耦合的

可以拆解为：
```cpp
class Swithable {
public:
    virtual void on() = 0;
    virtual void off() = 0;
};

class Switch {
private:
    Switchadble& switchable;
    bool state {false};
public:
    Switch(Switchable &sw): switchable(sw) {}
    void toggle() {
        if (state) {
            state = false;
            switchable.off();
        }
        else {
            state = true;
            switchable.on();
        }
    }
};

class Lamp: public Switchable {
public:
    void on() override {

    }
    void off() override {

    }
};
```
这样，Lamp 就与 Switch 进行了解耦，扩展比如 Fan Radiator 就变得非常简单

### Be Careful with Optimizations
不要过早优化，如果没有现象表明需要优化，那就不要优化

如果一定要优化，不要凭借直觉进行优化，使用 Profile 工具(比如[火焰图](https://github.com/brendangregg/FlameGraph))对代码进行性能测试，结果可能与你直觉迥然不同


### Principle of Least Astonishment (PLA)
更知名的名称为 *Principle of Least Surprice(POLS)*，用户接口设计中，不要让用户吃惊，比如 Ctrl-C 通常是复制的意思，这已经被广泛接受，不要重定义为比如退出程序的逻辑

### The Boy Scout Rule
这个原则是关于开发者的。表示只要你发现了不干净的代码，马上整理它。比如：

- 重命名命名很失败的类，函数，方法，成员
- 将大函数拆分成小的逻辑单位
- 删除没必要的注释
- 清理复杂的if-else
- 删除重复代码

这与迭代重构是一个意思，一次重构一点，逐步重构整个系统，其中关键的是没一点修改都要经过单元测试

实际中重要的一点，就是共有代码权限，所有人可以修改所有代码 ---- 这通常在国内的公司有部分实现不了:) 逃