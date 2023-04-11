---
template: "page.html"
date: "2022-09-08 10:54:17"
title: "IEEE Std 1800-2017 DPI"
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
    tags: ["ieee1800"]

extra:
    mermaid: true
    usemathjax: true

# mermaid example:
# <div class="mermaid">
#     mermaid program
# </div>
---

## 35.1 General

按照下面的方面来描述：

- Direct programming interface (DPI) tasks and functions
- DPI layers
- Importing and exporting functions
- Importing and exporting tasks
- Disabling DPI tasks and functions

## 35.2 Overview

本节强调 DPI 并提供了 SystemVerilog 接口层面的详细描述。C 层面在[附录](https://wendajiang.github.io/ieee-dpi-c-layer)

DPI 是 SV 和其他语言的接口。包含了两个部分：SV 层和其他语言层，这两个部分是分离的。实际上，哪种语言在使用 DPI 对于 SV 来说是透明无关的。SV 的编译器和其他语言的编译器都不需要分析对方的代码。不同的编译语言都可以支持 DPI。不过，现在 SV 只定义了 C 语言的接口。

动机来自两个方面。接口的方法论要求应该可以允许构建异构系统（design 或者 testbench）。另一方面，也有一些实际需要要连接已经存在的代码，通常使用 C/C++ 写成，而不需要 VPI 的知识和开销。

DPI 依据黑箱原则：组件的规范和实现分离，每个组件的实现对系统其他部分是透明的。因此，真实编程语言中的实现也是透明的，即使目前只有 C 语言的语法支持。其他语言和 SV 分离基于使用函数作为 SV unit 的自然封装。总的来说，任何函数都可以看作黑盒，并以透明的方式在 SV 或者其他语言中实现，并且不需要修改调用方。

### 35.2.1 Tasks and functions

DPI 允许两侧的语言直接进行调用。需要强调的是，在外部语言实现，在 SV 中调用的函数称为 *imported function*。在外部语言中调用的 SV 的函数称为 *exported functions*。DPI 允许通过函数参数和结果在两种语言之间传递，没有内部开销。

也可以通过执行任务跨越语言边界。外部语言可以调用 SV task，SV 也可以调用 imported task。imported task 有 SV task 相同的语义：没有返回值，会消耗模拟时间。

在 DPI 中使用的所有函数都假定会立刻执行完不消耗模拟时间，就像 SV 的函数一样。DPI 除了通过数据交换和显式传输的控制，没有提供同步的手段。

每个 imported subroutine 需要被声明。而且这个声明相当于 SV 的实现，当然实际上是在外部语言实现的。同样的外部 subroutine 可以用来实现多个 SV 的 task 和 function（这对于相同的 subroutine 提供不同的默认参数很有用），但是一个给定的 name 在 SV 的一个 scope 只能使用一次。Imported 函数可以有零个或者多个参数（formal **input, output, inout**）。Imported 任务总是没有返回值，所以可以用在 statement context。

DPI 完全基于 SV 结构构造的。imported 函数使用上就像 SV 函数一样。除了少数功能，调用起来一样。这可以有效减少学习曲线。类似的语义相似性也存在于 imported 任务和 SV 任务。

### 35.2.2 Data types

SV 的数据类型是唯一可以跨越语言边界的数据类型。不能直接使用其他语言中的数据类型。SV 中丰富的数据类型子集都可以用于 imported 和 exported 函数参数，当然也有一些限制和一些符号扩展。函数返回类型只限于 small values，但是请参考[35.5.5](#35-5-5-function-result)

imported 函数的参数可以是 open arrays，参考[35.5.6.1](#35-5-6-1-open-arrays)

#### 35.2.2.1 Data representation

DPI 并没有在 SV 的数据类型实现增加任何约束。优化 representation 依赖平台。2- or 4-state packed structrues 和 arrays 的 layout 实现依赖平台。

4-state values, structures, arrays 的 representation 和 layout 与 SV 语义无关，并且只作用于外部语言的接口。

## 35.3 Two layers of DPI

DPI 包含了两个部分：SV 和外部语言。SV 只要求实现支持 C 的协议和链接。

### 35.3.1 DPI SystemVerilog layer

DPI 的 SV 侧不依赖外部语言。特别的是，外部语言的函数调用协议和参数传递机制对 SV 是透明的。无论外部代码如何 SV 代码看起来都应该相同。SV 侧的接口语义应该与外部语言的接口语义独立。

本章不会描述完整的接口规范，只是描述 SV 侧接口的功能，语义和语法。另一半外部接口侧，参数传递机制，如何从外部代码访问到参数，请参考[附录](https://wendajiang.github.io/ieee-dpi-c-layer)

#### 35.3.2 DPI foreign language layer

外部语言接口侧应该规定参数如何传递，如何从外部代码被访问，SV 规范的数据类型如何被表征，如何与预定义的类似 C 的类型转换。

被允许的参数数据类型和返回类型基本就是 SV 的数据类型（加一些限制和 open arrays 的额外扩展）。用户负责在外部代码中的参数使用 SV 等效的数据类型。像是 SV 编译器这种软件工具可以方便地生成正确 SV 的外部代码的函数头（头文件声明）。

SV 编译器或者模拟器应该使用外部语言使用的函数调用协议和参数传递机制生成。相同的 SV 代码应该可以兼容不同的外部语言，尽管数据方法访问在不同语言中不同。[附录](https://wendajiang.github.io/ieee-dpi-c-layer)定义了 C 语言的外部语言规范。

## 35.4 Global name space of imported and exported functions

每个 imported 的 subroutine 都应该对应一个全局符号。类似的，每个 exported 定义了一个全局符号。因此 SV 的 imported 和 exported 的函数和任务都自己的全局命名空间在链接中，不同于编译单元的 scope 命名空间。全局符号命名应该是唯一的，并且与 C 语言的命名保持一致（ABI 兼容）；尤其是，名字应该以字母或者下划线开头，后面可以是数字或者字母或者下划线。但是 exported 和 imported 的任务和函数应该可以使用 local 的 SV 命名声明，它们允许用户指定一个全局名字。如果与 SV 的关键字和保留字冲突，应该使用 escaped identifier 形式。`\` 反斜线符号和结尾空白应该被 SV 工具忽略。最后的链接符号应该符合 C 标准。如果全局命名没有被显式指定，就等于 SV 的 subroutine 命名。比如：

```sv
export "DPI-C" f_plus = function \f+; // "f+" exported as "f_plus"
export "DPI-C" function f; // "f" exported under its own name
import "DPI-C" init_1 = function void \init[1] (); // "init_1" is a linkage name
import "DPI-C" \begin = funciton void \init[2] (); // "begin" is a linkage name
```

相同的全局 subroutine 可以被多个在不同 scope 的 import 声明引用（参考[35.5.4](#35-5-4-import-declarations)）

多个 export 声明也可以使用同一个 C 的标识符，隐式或者显式，在不同的 scope 使用同样的类型声明（[35.5.4](#35-5-4-import-declarations) 中定义的 imported 任务和函数）。多个 export 声明使用同一个 C 标识符在同样的 scope 是被禁止的。

【略：deprecated "DPI" 版本】

## 35.5 Imported tasks and functions

imported 的函数类似 SV 本身的函数

### 35.5.1 Required properties of imported tasks and functions - semantic constraints

本小节定义 imported subroutine 的语义约束。有些限制是 subroutine 共同的。有些依赖特定的 properties `pure (35.5.2), context(35.5.3)`。SV 编译器无法验证这些限制；如果有些限制没有被满足，这些 imported subroutine 调用就会有不可预知的影响。

#### 35.5.1.1 Instant completion of imported functions

imported 函数不消耗模拟时间。

#### 35.5.1.2 input, output and inout arguments

imported 函数可以有 input, output, inout 参数。input 参数不可被修改。

imported 函数不可以对 output 参数的初始值有什么假设，是未定义的。

imported 函数可以访问 inout 参数的初始值，inout 参数的修改是外部可见的。

#### 35.5.1.3 Special properties pure and context

一个结果严格依赖 input 参数的函数，并且没有 side effect 可以看作 `pure`。这通常可以更好的优化，并且在模拟中有很好的性能。35.5.2 详细说明了 `pure` 函数应该遵守的规则。imported 任务不能被声明为 `pure`

imported subroutine 会调用 exported subroutines 或者反问 SV 的数据对象而不是参数（比如通过 VPI）应该称为 `context`。对 `context` 函数和任务的调用会影响 SV 编译器的优化；因此当不必要的 `context` 被指定会影响模拟性能。没有声明 `context` 的 subroutine 对于 VPI 的调用或者 exported SV subroutine 的调用会导致未定义行为，非常危险。35.5.3 详细说明了非 `context` subroutine 应该遵守的限制。

如果 `pure, context` 都没有声明，imported subroutine 不应该访问 SV 数据对象，并且也可以写入文件或者修改全局变量（施加 side effect）。

#### 35.5.1.4 Memory management

外部代码和 SV 代码分配和持有的内存空间是分开的。自己负责自己分配的空间。尤其是，imported 函数不应该释放 SV 代码申请的内存反之亦然。

#### 35.5.1.5 Reentrancy of imported tasks

对于 imported 的调用会导致当前执行线程的挂起，这发生于 imported 任务调用了一个 exported 任务，exported 任务中执行了延迟操作，事件控制，或者等待语句。因此 imported 任务的 C 代码可能在多个执行线程中活跃。C 开发者应该考虑可重入性。比如使用静态变量，确保使用了线程安全的 C 标准库 API。

#### 35.5.1.6 C++ exceptions

可能会使用 C++ 完成 imported 函数和任务，即使在接口上遵循了 C 约束。如果使用 C++，异常不能传递出来。如果跨语言边界存在异常会导致未定义行为。

### 35.5.2 Pure functions

`pure`函数如果它的结果不需要，或者之前有相同参数的调用，可以被消除。只有有返回值，并且没有 ouput 和 inout 参数的函数可以被声明为 `pure`。`pure` 函数的返回值只依赖于 input 参数，没有 side effect。这种函数的调用可能被 SV 编译器优化掉或者使用之前的值代替。

特别的是，`pure`函数假定不会直接或者间接执行以下操作：

- 文件操作
- 读写操作，包括 I/O，环境变量，操作系统或者其他进程的对象，共享内存，socket 等
- persistent 数据的访问，比如全局或者静态变量。

如果 `pure`没有遵守这些约束，SV 编译器的优化会导致不可预测的行为。

### 35.5.3 Context tasks and functions

有些 DPI imported subroutines 需要知道调用它们的上下文。需要特殊的调用来支持这种上下文；比如，一个关联“当前实例”的内部变量需要被赋值。为了避免不必要的开销，SV 代码中 imported subroutine 的特殊调用只会标签 `context` 发生。

DPI export 任务和函数必须知道被调用的时机，包括被 import 调用的 SV 上下文。当 import 调用 export 之前先调用了 `svSetScope` ，显式设置了上下文。否则，上下文就是调用处实例 scope 的上下文。因为不同实例 scope 的 import 可以 export 相同的 subroutine，在 elaboration 之后可能存在多个 export 实例。在进行 `svSetScope` 之前，这些 export 实例有不同的上下文反映不同的 imported 调用者实例 scope。

通过外部语言支持的一些其他接口（比如 VPI 回调）的 subroutine ，也可以调用 `svSetScope` 或者 DPI scope 相关的 API。外部语言 subroutine 也可以调用 `svSetScope` 指定的上下文中的 export subroutine。DPI scope 相关的 API 行为和 DPI export subroutine 的请求由模拟器定义，超出了 DPI 定义的 scope。

*call chains* 的概念对于理解 *context* 如何在 SV 和外部语言之间控制转换很有帮助。为了描述清晰，使用 *inter-language call* 表示跨语言调用，*intra-language call* 在 SV 或者外部语言内部的调用，不跨语言。

DPI import 调用链是 inter-language 调用链，从 SV 到外部语言定义的 subroutine。SV 的调用开始点称为调用链的根。调用链可以包含多个 inter-language call 和多个 intra-language call 。

![image-20220910003552091](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220910003552091.png)

对于 context subroutine 的调用会阻止 SV 编译器优化。

### 35.5.4 Import declarations

每个 imported 的 subroutine 都需要声明。声明的格式如下

![image-20220909102105075](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220909102105075.png)

任务的返回值总是 int 类型，所以外部函数定义也使用 int。

import 声明指定了 subroutine 名字，函数返回类型，参数类型。也可以提供可选的参数默认值。参数名字也是可选的，除非需要名字绑定。import 函数声明也可以可选 `pure or context`标注，import 任务声明可选 `context ` 标注。

由于 import 声明等于 SV 内部的定义，所以在同一个 scope 不能出现多次相同的 import 声明。

*dpi_spec_string* 可以是 "DPI-C" 或者 "DPI"。"DPI" 用来表明要用到 SV 已经弃用的 packed array 参数传递语义。在这种语义下，参数传递机制不同。并且会给出编译器警告或者错误：

- "DPI" is deprecated and should be replaced with "DPI-C"
- Use of the "DPI-C" string may require changes in the DPI application's C code.

*c_identifier* 提供了给外部语言的链接名称。如果没有，就使用 SV 的 subroutine 名字。链接名称需要满足 C 规范，不满足应该报错。

对于给定的 *c_identifier* 所有的声明应该有相同的类型签名。签名包括返回类型，参数的数量，顺序，方向，和类型。类型也包括了 array 的维度。签名还包括了 `pure / context` 标签，以及*dpic_spec_string*。

在不同的 scope 是允许存在相同的声明的，因此参数名称和默认值可以不同。

下面是一些例子：

![image-20220909171529703](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220909171529703.png)

### 35.5.5 Function result

imported 函数声明应该显式指定数据类型或者 void。函数返回类型被限制为 samll value。下面的 SV 数据类型可以作为返回类型：

- void, byte, shortint, int, longint, real, shortreal, chandle, string
- scalar values of type `bit` and `logic`

exported 函数返回类型有相同的限制。

### 35.5.6 Types of formal arguments

SV 的数据类型很大的子集可以作为参数类型。通常，C 兼容类型，packed 类型，和基于这两种类型的用户定义类型都可以用于 DPI 的 subroutine。

![image-20220909172451991](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220909172451991.png)

![image-20220909172733051](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220909172733051.png)

#### 35.5.6.1 Open arrays

packed 大小，unpakced 大小没有定义的还没涉及：这种情况属于 open array（unsized arrays）。open array 允许通用代码处理不同的大小。

imported 函数的参数可以作为 open arrays（exported SV 函数不行）。

![image-20220909173145522](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220909173145522.png)

## 35.6 Calling imported functions

调用 imported 函数与 SV 函数是一样的。有默认参数的不能在调用中被省略，如果形参有名字，参数可以通过名字绑定。

### 35.6.1 Argument passing

imported 函数的参数传递依据 WYSIWYG 原则：What You Specify Is What You Get。形参的运算顺序参照 SV 规则。

参数兼容性和强制转换规则也和 SV 一样。如果需要转换类型，会创建一个临时变量，然后作为实参传入。对于 input 和 inout 参数类型，临时变量使用实参的值来初始化。对于 output 或者 inout 参数，临时变量使用实参赋值。

SV 侧的接口，input 参数的实参不会被被调影响。output 参数初始值未定义。imported 函数不应该修改 input 参数。

对于 SV 侧的接口，参数传递的语义是：input 参数 copy-in，output 参数 copy-out，inout 参数 copy-in， copy-out。

参数传递的实现对于 SV 是透明的。尤其是对于 SV 来说通过值传递还是引用传递是透明的。实参传递机制是外部语言定义的 layout。

#### 35.6.1.1 WYSIWYG principle

WYSIWYG 原则保证了 imported 函数形参类型：实参保证符合形参的类型，除了 open arrays。除了 open arrays，形参完全由 import 声明定义。

另一种说法是，没有编译器（C 或者 SV）可以在调用者声明的形参和被调者声明的形参之间强制转换。这是因为被调者形参声明的语言与调用者不同，因此两种形参是互相不可见的。

### 35.6.2 Value changes for output and inout arguments

SV 模拟器需要负责处理 output 和 inout 参数的修改。这种修改在 imported 函数完成回到 SV 代码之后应该是完成的。

对于 output 和 inout 参数，值的传播（值修改事件）发生在 imported 函数返回后实参的值马上赋值给形参。如果有超过一个参数，赋值顺序参照 SV 的规则。

## 35.7 Exported functions

DPI 还允许 SV 函数被外部语言调用。但是，函数的限制与 imported 函数相同。通过声明为 exported 不改变 SV 函数的语义。

SV exported 函数可以被外部语言调用，但是 export 的声明只在函数定义的 scope 有效。

有一个重要限制，类的成员函数不能被 exported，其他所有函数都可以。

声明格式在介绍 imported 函数一节已经描述过。**所有的 exported 函数都是 context 函数**。

## 35.8 Exported tasks

SV 允许外部函数调用 task。

35.7 对于 exported 函数的都有描述都适用于 exported 任务。

在 imported 函数中调用 exported 任务是不合法的，与 SV 自己的定义规则相同：在函数中调用任务不合法。

只有在 context imported 任务中调用 exported 任务才合法。

exported 的任务和函数的不同点在于 SV 任务没有返回值。任务的返回值是一个 int 值，表明禁用是否处于当前执行的线程。

类似的，imported 任务返回 int 值用于表明 imported 任务关于禁用的知识。

## 35.9 Disabling DPI tasks and functions

可以使用 disable 语句使得 block 禁用当前正在执行混合语言调用链。当一个 DPI impor subroutine 被禁用，C 代码需要符合一个简单的禁用协议：使得 C 代码可以执行必要的资源清理，不如关闭打开的文件，关闭打开的 VPI handle，或者释放堆内存。

当 disable 语句在设计目标或者父模块正在 diabling，imported subroutine 就处于禁止状态。imported subroutine 只会在调用 exported subroutine 返回之后进入禁止状态。协议的重要一点是禁止的 import 任务和函数应该在语义上了解自己被禁止了。subroutine 可以通过调用 `svIsDisabledState()`来了解是否被禁止。

![image-20220910113931996](https://wendajiang.github.io/pics//ieee_1800_2017_chapter35/image-20220910113931996.png)

b，c，d 是 imported DPI 任务和函数的强制行为。DPI 开发者有责任正确实现这个行为。

a 由 SV 模拟器保证。并且模拟器应该也对 b，c，d 验证，如果没有满足应该报错。

外部语言包含禁止协议，这个协议是用户代码与模拟器一起工作的。禁止协议允许外部模型参与模拟器的禁止处理。

任务的特殊返回值没有改变 SV 代码的调用语法。export 任务的返回值由模拟器保证，import 任务的返回值由 DPI 应用保证。

exported 任务本省是禁止的，但是其父 imported 任务不在禁止状态。

当一个 DPI imported subroutine 由于禁用返回，output 和 inout 参数的值是未定义的。同时，函数的返回值也是未定义的。C 程序员可以从禁用函数返回，修改 output 和 inout 参数，但是 SV 模拟器不会传递这个值。