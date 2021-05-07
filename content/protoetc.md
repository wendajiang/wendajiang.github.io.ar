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

[TOC]

## cap'n proto

https://capnproto.org/index.html

## protobuf2语言

https://developers.google.com/protocol-buffers/docs/proto

## protobuf3语言

https://developers.google.com/protocol-buffers/docs/proto3

## protobuf style
https://developers.google.com/protocol-buffers/docs/style

## 编码

https://developers.google.com/protocol-buffers/docs/encoding

### 翻译

这篇文档阐述了protobuf的message二进制编码的原理。你在应用中使用pb时不需要理解这个，但是知道这些可以更好地帮助你了解使用pb编码之后的消息大小

#### A simple Message

来看下这个简单的message 定义:

```protobuf
message Test1 {
	optional int32 a = 1;
}
```

应用中，你可以创建`Test1`message然后`set a `为150，然后序列化这个message作为输出流。如果打印这个流，你可以看到三个字节的内容：

```shell
08 96 01
```

哇哦，如此小，这意味着什么？且慢慢往下读

#### Base 128 Varints

为了理解简单的protocol buffer编码，首先你需要理解 ***varints***，Varints是一种使用一个或者更多字节序列化整数的方法，越小的数字使用越少的字节数。

除了最后一个字节外，varint每个字节都有最高有效位（msb），这表明还有更多的字节在后面。这个字节的低7位为一组来存储数字的补码表示，**最低有效组在前**。

比如，这有一个数字1，简单一个字节：

```shell
0000 0001 # 1
```

300 就要复杂一些：

```shell
1010 1100 0000 0010 # 300
```

你如何认出这是300呢，首先丢掉每个字节的最高有效位（只是表示是不是最后一个字节）

```shell
1010 1100 0000 0010
->
 010 1100  000 0010
```

然后以7位一组反向整个二进制序列，就像前面所述，varints保存数字式，**最小有效组在前**，然后你可以按照补码的正常计算方式来计算数字（正数的补码就是直接转化的二进制）

```shell
000 0010  010 1100
→  000 0010 ++ 010 1100
→  100101100
→  256 + 32 + 8 + 4 = 300
```

#### Message Structure

如你所知，一个pb message就是一系列的key-value对，序列化为二进制后的message使用域号(field number)作为key--每个字段的名称和声明的类型只能在解码端通过引用消息类型的定义来确定。

当message编码时，keys和values被连接成字节流。当message解码时，解析器需要跳过不认识的域。这样的话，新的域才能在不影响老程序的情况下添加到message的后面。最终，编码后的message中每对key-value取决于两个值--**`.proto`文件中的域号（field number)，声明的类型（为了了解信息的字节长度）**。在大多数的语言实现中，key也被称为tag。

有效的类型声明如下表：

| Type | Meaning          | Used For                                                 |
| :--- | :--------------- | :------------------------------------------------------- |
| 0    | Varint           | int32, int64, uint32, uint64, sint32, sint64, bool, enum |
| 1    | 64-bit           | fixed64, sfixed64, double                                |
| 2    | Length-delimited | string, bytes, embedded messages, packed repeated fields |
| 3    | Start group      | groups (deprecated)                                      |
| 4    | End group        | groups (deprecated)                                      |
| 5    | 32-bit           | fixed32, sfixed32, float                                 |

字节流里的每个key的value都是`(field_number << 3) | wire_type`这样组成，换句话说，最后三位存储声明类型

现在，让我们再次看那个简单的例子。你已经知道第一个第一个数字总是varint key，这里就是`08`，或者丢掉最高标志位：

```shell
000 1000
```

你可以通过获取最后三位得到声明的类型（0），然后右移三位获得域号（1）。现在你知道了域号是1，接下来的字节还是varint。使用前面知道的解码知识，你可以看到后面两个字节存储了150这个值

```shell
96 01 = 1001 0110  0000 0001
       → 000 0001  ++  001 0110 (drop the msb and reverse the groups of 7 bits)
       → 10010110
       → 128 + 16 + 4 + 2 = 150
```

#### More Value Types

##### Signed Integers

如同在前面小节看到，所有的pb类型为0的都被编码为varints。但是在有符号整形（sint32/sint64），和标准整形（int32/int64）之间如果编码负数存在巨大区别。如果使用标准整形编码负数，varints的结果*总是是个字节长度*，实际上，它被看做一个非常大的无符号整数，如果使用有符号整形，varint使用ZigZag编码方式，这更有效率。

ZigZag编码将有符号整数映射为无符号整数，使得具有较小绝对值的数字（比如-1）也具有较短的varint编码值。正是通过正数和负数"zig-zags"的方式，使得 -1 被编码为 1， 1 被编码为 2， -2 被编码为3，以此类推，如同下面表格展示：

| Signed Original | Encoded As |
| :-------------- | :--------- |
| 0               | 0          |
| -1              | 1          |
| 1               | 2          |
| -2              | 3          |
| 2147483647      | 4294967294 |
| -2147483648     | 4294967295 |

换句话说，n会这样编码：

```shell
(n << 1) ^ (n >> 31)   # for sint32
(n << 1) ^ (n >> 63)   # for sint64
```

注意第二个位右移--`(n >> 31)`是算术移位的。换句话说，移位的结果不是全0（n为正数），就是全1（n为负数）。

当`sint32/sint64`被解析时，其值将解码回原始的带符号版本。

##### Non-Varint Number

非varint数字类型就简单的两种`double和fixed64`，域类型为1，告诉解析器是一个固定的64位的数据块；类似的`float 和 fixed32`，域类型为5，告诉解析器固定为32位。这两种情况都被存储为小端字节序。

##### Strings

域类型2（长度受限）表示该值是varint编码的长度，后面跟着指定长度的字节。

```protobuf
message Test2 {
	optional string b = 2;
}
```

设置值为"testing"编码为：（TLV）

```shell
12 07 74 65 73 74 69 6e 67
```

这是"testing"的UTF-8。key是0x12 ->

```shell
0001 0010 
-> 00010 010
```

得到field_number = 2，wire_type = 2。varint的中的值长度为7，lo且为0，我们在其后找到七个字节---我们的字符串。

#### Embedded Messages

有一个message定义中包含了之前的Test1：

```protobuf
message Test3 {
	optional Test1 c = 3;
}
```

这是将Test1的a设置为150的编码版本：

```shell
1a 03 08 96 01
```

正如你所看到的，最后三个字节跟我们第一个例子中一样（`08 96 01`），在数字3后面 - 内嵌的message跟strings的方式是一样的。

#### Optional and Repeated Elements

如果proto2的message定义中有`repeated`类型元素（没有包含参数`[packed=true]`），编码后的message具有零个或者多个具有相同字段编号的key-value对。这些重复的值不必要连续，可能跟其他域交叉出现。解析时，元素之间的顺序会保留下来，尽管其他字段的顺序会丢失。在proto3中，`repeated`声明使用`packed encoding`（下面的packed repeated fields节阐述)。

对于任意proto3非`repeated`域，和proto2中的`optional`域，编码后的message可能有也可能没有该字段编号的key-value对。

通常，编码后的message不会有一个以上非`repeated`域。但是解析器必须能够处理这种情况。对于数字和字符串类型，如果同一个域出现了多次，解析器取最后一个值。对于内嵌的message域，解析器合并同一个域的多个实例，就像使用`Message::MergeFrom`方法，表现为所有的奇异域都会被后面的实例替代，奇异内嵌message都会被递归式合并，`repeated`域会被串联起来。这些规则的结果就是，解码两个串联起来的编码message提供了解码两条message得到相同的结果，例如：

```cpp
MyMessage message;message.ParseFromString(str1 + str2);// 等于下面的MyMessage message, message2;message.ParseFromString(str1);message2.ParseFromString(str2);message.MergeFrom(message2);
```

这个特性非常有用，允许你合并两条你不知道类型的message。

##### Packed Repeated Fields

版本2.1.0引入了`packed repeated field`，在proto2中声明为`repeated`域而且具有`[packed=ture]`参数的，在proto3中，`repeated`默认使用packed打包标量数字类型。这些功能类似`repeated fields`，但是编码方式不同。`packed repeated field`包含0的元素不会出现在编码后的message中。另外，该域的所有元素都打包成域类型为2的单个key-value对。每个元素的编码方式与之前相同，除了在前面没有key。

比如说，假设有一个message type：

```protobuf
message Test4 {	repeated int32 d = 4 [packed = true];}
```

让我们现在构造一个Test4，提供d的值为3270和86942。然后编码后的形式如下：

```shell
22        // key (field number 4, wire type 2)06        // payload size (6 bytes)03        // first element (varint 3)8E 02     // second element (varint 270)9E A7 05  // third element (varint 86942)
```

只有`repeated`的数字类型（使用varint，32/64位的wire-type）会被打包编码。

值得注意的是尽管没有理由为一个`repeated field`编码超多一个key-value对，编码器必须能够接受多个key-value对。这种情况下，payload应该串联起来。每个对必须包含一整个元素。

pb解析器必须能够解析通过`packed`编码的`repeated`域，就好像没有使用`packed`一样，反之亦然。这才能使得有没有加`[packed = true]`的前后兼容的方式添加新字段。

#### Field Order

域号（Field Number）可以在`.proto`文件中以任何顺序使用。顺序的选择不会影响message的序列化。

当message序列化时，对于如何写入已知或者未知域没有顺序保证。序列化顺序是一个实现细节，将来任何实现的细节可能都会改变。因此protobuf的解析器必须能够解析任意顺序域的编码。

##### Implications 含义

- 不要假定序列化之后的message字节输出是稳定的，对于那些表示其他pb序列化消息的传递性字节域尤其是。

- 默认情况下，在同一个pb message实例上多次调用序列化方法可能会得到不同的输出；即，默认序列化输出是不确定性的

  - 确定性序列化只能保证相同的二进制字节序列。字节输出可能在不通的版本之间有变化

- 下列检查对于pb message实例foo可能是失败的：

  - ```cpp
    foo.SerializeAsString() == foo.SerializeAsString()Hash(foo.SerializeAsString()) == Hash(foo.SerializeAsString())CRC(foo.SerializeAsString()) == CRC(foo.SerializeAsString())FingerPrint(foo.SerializeAsString()) == FingerPrint(foo.SerializeAsString())
    ```

- 有一些逻辑等效的pb message实例`foo`和`bar`可能序列化输出不同的场景：

  - `bar`被老服务器序列化表示有些域是未知的
  - `bar`被序列化时是不同语言实现的导致序列化域号顺序不同
  - `bar`存在不确定性序列化的域
  - `bar`有一个域，用于存储pb message的序列化字节输出，该message的序列化输出不同
  - `bar`使用新的服务器序列化，实现不通导致输出不同
  - `foo`和`bar`被单独的message以不通顺序串联起来

----------

## 序列化原理

Protocol Buffers 是一种轻便高效的结构化数据存储格式，可以用于结构化数据串行化，或者说序列化。它很适合做数据存储或数据交换格式。可用于通讯协议、数据存储等领域的语言无关、平台无关、可扩展的序列化结构数据格式

protobuf2中修饰符：

- required : 不可以增加或删除的字段，必须初始化；
- optional :  可选字段，可删除，可以不初始化；
- repeated : 可重复字段， 对应到java文件里，生成的是List。

### protobuf的使用

```shell
protoc -I=SRC_DIR --cpp_out=DST_DIR person.proto
```

### 编码原理（重点是类型和域号）

protobuf中的message中有很多字段，每个字段的格式：

```protobuf
修饰符 字段类型 字段名 = 域号;
```

在序列化时，protobuf按照`TLV`的格式序列化每一个字段，T即Tag，V是该字段对应的value，L是value的长度。如果字段是一个整形，L部分会省略。

序列化后的Value按照原样保存在字符串或者文件中，Tag按照一定转换条件保存起来，序列化之后的结果就是: TagValueTagValue...

Tag的格式化序列是按照message中字段后面的域号和字段类型类转换的，转换公式为:

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

protobuf协议使用二进制格式表示Tag字段；对value而言，不同的类型采用的编码方式也不同，如果是整型，采用二进制表示；如果是字符，会直接原样写入文件或者字符串（即不编码）。


## cpp-serializers 对比 benchmark
https://github.com/thekvs/cpp-serializers

## 更新记录
2021-05-07 init 翻译protobuffer官网 encode 的文档