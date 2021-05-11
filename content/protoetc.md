+++
template = "page.html"
date = "2021-05-07 14:26:39"
title = "protobuffer etc."
[taxonomies]
tags = ["protobuf", "capn proto", "flatterproto"]

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

## protobuf2 语言

[原文](https://developers.google.com/protocol-buffers/docs/proto)

## protobuf3 语言

[原文](https://developers.google.com/protocol-buffers/docs/proto3)

## protobuf style
[原文](https://developers.google.com/protocol-buffers/docs/style)

请注意，PB 已经随着时间而发展，因此你可能会看到不同风格编写的 .proto 文件，修改这些文件时，请尊重现有风格，一致性是关键。但是创建新的 .proto 文件时，请采用当前的最新风格
### 标准文件格式
- 每行不超过 80 字符
- 使用 2 空格缩进
- 最好对字符串使用双引号

### 文件结构

文件命名 `lower_snake_case.proto`

所有文件应该以下列顺序排布：
1. License header (if applicable)
2. File overview
3. Syntax
4. Package
5. Imports (sorted)
6. File options
7. Everything else 

### Packages
Packages 名称应该小写，还应该匹配文件层级。比如如果文件在 `my/package/`，package 名称应该为 `my.package`

### Message 和 field names
使用 CamelCase 命名 message -- 比如，`SongServerRequest`，使用 underscore_separated_names 命名 field name，比如 `song_name`

```protobuf
message SongServerRequest {
  optional string song_name = 1;
}
```
这种命名约定，语言生成为

```cpp
// cpp
const string& song_name() {...}
void set_song_name(const string& x) {...}
```

```java
// java
public String getSongName() {...}
public Builder setSongName(String v) {...}
```

如果你的 field name 包含一个数字，不需要加下划线，比如应该是 `song_name1` 而不是 `song_name_1`

### Repeated fields
使用复数命名这种 field

```protobuf
repeated string keys = 1;
...
repeated MyMessage accounts = 17;
```

### Enums
使用 CamelCase 命名 enum type name，使用 CPPITALS_WITH_UNDERSOCRES 命名 value name

```protobuf
enum FooBar {
  FOO_BAR_UNSPECIFIED = 0;
  FOO_BAR_FIRST_VALUE = 1;
  FOO_BAR_SECOND_VALUE = 2;
}
```
每个 enum 应该以分号结尾而不是逗号。需要为 enum 值添加前缀，因为历史代码的兼容，没有引入 C++11 的 enum class scope，零 enum 值应该有后缀

### Services
如果你的 .proto 文件定义 RPC 服务，应该使用 CamelCase 风格，同时应用于服务名和任何RPC方法名

```protobuf
service FooService {
  rpc GetSomething(FooRequest) returns (FooResponse);
}
```

### Things to avoid
- 只有 proto2 存在 required field
- 只有 proto2 存在 Groups

## 编码

[原文](https://developers.google.com/protocol-buffers/docs/encoding)

### 翻译

这篇文档阐述了 protobuf 的 message 二进制编码的原理。你在应用中使用 pb 时不需要理解这个，但是知道这些可以更好地帮助你了解使用 pb 编码之后的消息大小

#### A simple Message

来看下这个简单的 message 定义：

```protobuf
message Test1 {
	optional int32 a = 1;
}
```

应用中，你可以创建 `Test1` message 然后 `set a ` 为 150，然后序列化这个 message 作为输出流。如果打印这个流，你可以看到三个字节的内容：

```shell
08 96 01
```

哇哦，如此小，这意味着什么？且慢慢往下读

#### Base 128 Varints

为了理解简单的 protocol buffer 编码，首先你需要理解 ***varints***，Varints 是一种使用一个或者更多字节序列化整数的方法，越小的数字使用越少的字节数。

除了最后一个字节外，varint 每个字节都有最高有效位（msb），这表明还有更多的字节在后面。这个字节的低 7 位为一组来存储数字的补码表示，**最低有效组在前**。

比如，这有一个数字 1，简单一个字节：

```shell
0000 0001 # 1
```

300 就要复杂一些：

```shell
1010 1100 0000 0010 # 300
```

你如何认出这是 300 呢，首先丢掉每个字节的最高有效位（只是表示是不是最后一个字节）

```shell
1010 1100 0000 0010
->
 010 1100  000 0010
```

然后以 7 位一组反向整个二进制序列，就像前面所述，varints 保存数字式，**最小有效组在前**，然后你可以按照补码的正常计算方式来计算数字（正数的补码就是直接转化的二进制）

```shell
000 0010  010 1100
→  000 0010 ++ 010 1100
→  100101100
→  256 + 32 + 8 + 4 = 300
```

#### Message Structure

如你所知，一个 pb message 就是一系列的 key-value 对，序列化为二进制后的 message 使用域号 (field number) 作为 key--每个字段的名称和声明的类型只能在解码端通过引用消息类型的定义来确定。

当 message 编码时，keys 和 values 被连接成字节流。当 message 解码时，解析器需要跳过不认识的域。这样的话，新的域才能在不影响老程序的情况下添加到 message 的后面。最终，编码后的 message 中每对 key-value 取决于两个值--**`.proto`文件中的域号（field number)，声明的类型（为了了解信息的字节长度）**。在大多数的语言实现中，key 也被称为 tag。

有效的类型声明如下表：

| Type | Meaning          | Used For                                                 |
| :--- | :--------------- | :------------------------------------------------------- |
| 0    | Varint           | int32, int64, uint32, uint64, sint32, sint64, bool, enum |
| 1    | 64-bit           | fixed64, sfixed64, double                                |
| 2    | Length-delimited | string, bytes, embedded messages, packed repeated fields |
| 3    | Start group      | groups (deprecated)                                      |
| 4    | End group        | groups (deprecated)                                      |
| 5    | 32-bit           | fixed32, sfixed32, float                                 |

字节流里的每个 key 的 value 都是`(field_number << 3) | wire_type`这样组成，换句话说，最后三位存储声明类型

现在，让我们再次看那个简单的例子。你已经知道第一个第一个数字总是 varint key，这里就是`08`，或者丢掉最高标志位：

```shell
000 1000
```

你可以通过获取最后三位得到声明的类型（0），然后右移三位获得域号（1）。现在你知道了域号是 1，接下来的字节还是 varint。使用前面知道的解码知识，你可以看到后面两个字节存储了 150 这个值

```shell
96 01 = 1001 0110  0000 0001
       → 000 0001  ++  001 0110 (drop the msb and reverse the groups of 7 bits)
       → 10010110
       → 128 + 16 + 4 + 2 = 150
```

#### More Value Types

##### Signed Integers

如同在前面小节看到，所有的 pb 类型为 0 的都被编码为 varints。但是在有符号整形（sint32/sint64），和标准整形（int32/int64）之间如果编码负数存在巨大区别。如果使用标准整形编码负数，varints 的结果*总是是个字节长度*，实际上，它被看做一个非常大的无符号整数，如果使用有符号整形，varint 使用 ZigZag 编码方式，这更有效率。

ZigZag 编码将有符号整数映射为无符号整数，使得具有较小绝对值的数字（比如-1）也具有较短的 varint 编码值。正是通过正数和负数"zig-zags"的方式，使得 -1 被编码为 1， 1 被编码为 2， -2 被编码为 3，以此类推，如同下面表格展示：

| Signed Original | Encoded As |
| :-------------- | :--------- |
| 0               | 0          |
| -1              | 1          |
| 1               | 2          |
| -2              | 3          |
| 2147483647      | 4294967294 |
| -2147483648     | 4294967295 |

换句话说，n 会这样编码：

```shell
(n << 1) ^ (n >> 31)   # for sint32
(n << 1) ^ (n >> 63)   # for sint64
```

注意第二个位右移--`(n >> 31)`是算术移位的。换句话说，移位的结果不是全 0（n 为正数），就是全 1（n 为负数）。

当`sint32/sint64`被解析时，其值将解码回原始的带符号版本。

##### Non-Varint Number

非 varint 数字类型就简单的两种`double 和 fixed64`，域类型为 1，告诉解析器是一个固定的 64 位的数据块；类似的`float 和 fixed32`，域类型为 5，告诉解析器固定为 32 位。这两种情况都被存储为小端字节序。

##### Strings

域类型 2（长度受限）表示该值是 varint 编码的长度，后面跟着指定长度的字节。

```protobuf
message Test2 {
	optional string b = 2;
}
```

设置值为"testing"编码为：（TLV）

```shell
12 07 74 65 73 74 69 6e 67
```

这是"testing"的 UTF-8。key 是 0x12 ->

```shell
0001 0010 
-> 00010 010
```

得到 field_number = 2，wire_type = 2。varint 的中的值长度为 7，lo 且为 0，我们在其后找到七个字节---我们的字符串。

#### Embedded Messages

有一个 message 定义中包含了之前的 Test1：

```protobuf
message Test3 {
	optional Test1 c = 3;
}
```

这是将 Test1 的 a 设置为 150 的编码版本：

```shell
1a 03 08 96 01
```

正如你所看到的，最后三个字节跟我们第一个例子中一样（`08 96 01`），在数字 3 后面 - 内嵌的 message 跟 strings 的方式是一样的。

#### Optional and Repeated Elements

如果 proto2 的 message 定义中有`repeated`类型元素（没有包含参数`[packed=true]`），编码后的 message 具有零个或者多个具有相同字段编号的 key-value 对。这些重复的值不必要连续，可能跟其他域交叉出现。解析时，元素之间的顺序会保留下来，尽管其他字段的顺序会丢失。在 proto3 中，`repeated`声明使用`packed encoding`（下面的 packed repeated fields 节阐述）。

对于任意 proto3 非`repeated`域，和 proto2 中的`optional`域，编码后的 message 可能有也可能没有该字段编号的 key-value 对。

通常，编码后的 message 不会有一个以上非`repeated`域。但是解析器必须能够处理这种情况。对于数字和字符串类型，如果同一个域出现了多次，解析器取最后一个值。对于内嵌的 message 域，解析器合并同一个域的多个实例，就像使用`Message::MergeFrom`方法，表现为所有的奇异域都会被后面的实例替代，奇异内嵌 message 都会被递归式合并，`repeated`域会被串联起来。这些规则的结果就是，解码两个串联起来的编码 message 提供了解码两条 message 得到相同的结果，例如：

```cpp
MyMessage message;
message.ParseFromString(str1 + str2);
```
// 等于下面的 
```cpp
MyMessage message, message2;
message.ParseFromString(str1);
message2.ParseFromString(str2);
message.MergeFrom(message2);
```


这个特性非常有用，允许你合并两条你不知道类型的 message。

##### Packed Repeated Fields

版本 2.1.0 引入了`packed repeated field`，在 proto2 中声明为`repeated`域而且具有`[packed=ture]`参数的，在 proto3 中，`repeated`默认使用 packed 打包标量数字类型。这些功能类似`repeated fields`，但是编码方式不同。`packed repeated field`包含 0 的元素不会出现在编码后的 message 中。另外，该域的所有元素都打包成域类型为 2 的单个 key-value 对。每个元素的编码方式与之前相同，除了在前面没有 key。

比如说，假设有一个 message type：

```protobuf
message Test4 {	
  repeated int32 d = 4 [packed = true];
}
```

让我们现在构造一个 Test4，提供 field d 的值为 3270 和 86942。然后编码后的形式如下：

```cpp
22        // key (field number 4, wire type 2)
06        // payload size (6 bytes)
03        // first element (varint 3)
8E 02     // second element (varint 270)
9E A7 05  // third element (varint 86942)
```

只有`repeated`的数字类型（使用 varint，32/64 位的 wire-type）会被打包编码。

值得注意的是尽管没有理由为一个`repeated field`编码超多一个 key-value 对，编码器必须能够接受多个 key-value 对。这种情况下，payload 应该串联起来。每个对必须包含一整个元素。

pb 解析器必须能够解析通过`packed`编码的`repeated`域，就好像没有使用`packed`一样，反之亦然。这才能使得有没有加`[packed = true]`的前后兼容的方式添加新字段。

#### Field Order

域号（Field Number）可以在`.proto`文件中以任何顺序使用。顺序的选择不会影响 message 的序列化。

当 message 序列化时，对于如何写入已知或者未知域没有顺序保证。序列化顺序是一个实现细节，将来任何实现的细节可能都会改变。因此 protobuf 的解析器必须能够解析任意顺序域的编码。

##### Implications 含义

- 不要假定序列化之后的 message 字节输出是稳定的，对于那些表示其他 pb 序列化消息的传递性字节域尤其是。

- 默认情况下，在同一个 pb message 实例上多次调用序列化方法可能会得到不同的输出；即，默认序列化输出是不确定性的

  - 确定性序列化只能保证相同的二进制字节序列。字节输出可能在不通的版本之间有变化

- 下列检查对于 pb message 实例 foo 可能是失败的：

  - `foo.SerializeAsString() == foo.SerializeAsString()`
  - `Hash(foo.SerializeAsString()) == Hash(foo.SerializeAsString())`
  - `CRC(foo.SerializeAsString()) == CRC(foo.SerializeAsString())`
  - `FingerPrint(foo.SerializeAsString()) == FingerPrint(foo.SerializeAsString())`

- 有一些逻辑等效的 pb message 实例`foo`和`bar`可能序列化输出不同的场景：

  - `bar`被老服务器序列化表示有些域是未知的
  - `bar`被序列化时是不同语言实现的导致序列化域号顺序不同
  - `bar`存在不确定性序列化的域
  - `bar`有一个域，用于存储 pb message 的序列化字节输出，该 message 的序列化输出不同
  - `bar`使用新的服务器序列化，实现不通导致输出不同
  - `foo`和`bar`被单独的 message 以不通顺序串联起来

----------

## 序列化原理

Protocol Buffers 是一种轻便高效的结构化数据存储格式，可以用于结构化数据串行化，或者说序列化。它很适合做数据存储或数据交换格式。可用于通讯协议、数据存储等领域的语言无关、平台无关、可扩展的序列化结构数据格式

protobuf2 中修饰符：

- required : 不可以增加或删除的字段，必须初始化；
- optional :  可选字段，可删除，可以不初始化；
- repeated : 可重复字段， 对应到 java 文件里，生成的是 List。

### protobuf 的使用

```shell
protoc -I=SRC_DIR --cpp_out=DST_DIR person.proto
```

### 编码原理（重点是类型和域号）

protobuf 中的 message 中有很多字段，每个字段的格式：

```protobuf
修饰符 字段类型 字段名 = 域号；
```

在序列化时，protobuf 按照`TLV`的格式序列化每一个字段，T 即 Tag，V 是该字段对应的 value，L 是 value 的长度。如果字段是一个整形，L 部分会省略。

序列化后的 Value 按照原样保存在字符串或者文件中，Tag 按照一定转换条件保存起来，序列化之后的结果就是：TagValueTagValue...

Tag 的格式化序列是按照 message 中字段后面的域号和字段类型类转换的，转换公式为：

```cpp
(field_number << 3) | write_type
```

| wire_type | meaning       | type                                                      |
| --------- | ------------- | --------------------------------------------------------- |
| 0         | Vaint         | int32、int64、uint32、uint64、sint32、sint64、bool、enum  |
| 1         | 64-bit        | fixed、sfixed64、double                                   |
| 2         | Length-delimi | string、bytes、embedded、messages、packed repeated fields |
| 3         | Start group   | Groups(deprecated)                                        |
| 4         | End group     | Groups(deprecated)                                        |
| 5         | 32-bit        | fixed32、sfixed32、float                                  |

protobuf 协议使用二进制格式表示 Tag 字段；对 value 而言，不同的类型采用的编码方式也不同，如果是整型，采用二进制表示；如果是字符，会直接原样写入文件或者字符串（即不编码）。

## cap'n proto
[官网](https://capnproto.org/index.html)

## cpp-serializers 对比 benchmark
[github](https://github.com/thekvs/cpp-serializers)

## 更新记录
2021-05-07 init 翻译 protobuffer 官网 encode 的文档