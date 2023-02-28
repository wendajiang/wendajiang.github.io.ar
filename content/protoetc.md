+++
template = "blog/page.html"
date = "2022-01-20 17:26:39"
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

# protobuf2 语言
[原文](https://developers.google.com/protocol-buffers/docs/proto)

## 定义 message 
### 三种标识符
- required
- optional
- repeated

历史原因，`repeated` 类型不能高效编码，[packed=true] 新代码可以使用这个选项使得编码更高效

```proto
repeated int32 samples = 4 [packed=true];
repeated ProtoEnum results = 5 [packed=true];
```
### reserved field
如果删除了 field 或者注释掉，未来可能会重用，这很危险，所以使用 `reserved` 来避免这个问题

```proto
message Foo {
  reserved 2, 15, 9 to 11;
  reserved "foo", "bar";
}
```

## Scalar Value Types

A scalar message field can have one of the following types – the table shows the type specified in the `.proto` file, and the corresponding type in the automatically generated class:

| .proto Type | Notes                                                        | C++ Type | Java Type  | Python Type[2]                       | Go Type  |
| :---------- | :----------------------------------------------------------- | :------- | :--------- | :----------------------------------- | :------- |
| double      |                                                              | double   | double     | float                                | *float64 |
| float       |                                                              | float    | float      | float                                | *float32 |
| int32       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32    | int        | int                                  | *int32   |
| int64       | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64    | long       | int/long[3]                          | *int64   |
| uint32      | Uses variable-length encoding.                               | uint32   | int[1]     | int/long[3]                          | *uint32  |
| uint64      | Uses variable-length encoding.                               | uint64   | long[1]    | int/long[3]                          | *uint64  |
| sint32      | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32    | int        | int                                  | *int32   |
| sint64      | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64    | long       | int/long[3]                          | *int64   |
| fixed32     | Always four bytes. More efficient than uint32 if values are often greater than 228. | uint32   | int[1]     | int/long[3]                          | *uint32  |
| fixed64     | Always eight bytes. More efficient than uint64 if values are often greater than 256. | uint64   | long[1]    | int/long[3]                          | *uint64  |
| sfixed32    | Always four bytes.                                           | int32    | int        | int                                  | *int32   |
| sfixed64    | Always eight bytes.                                          | int64    | long       | int/long[3]                          | *int64   |
| bool        |                                                              | bool     | boolean    | bool                                 | *bool    |
| string      | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string   | String     | unicode (Python 2) or str (Python 3) | *string  |
| bytes       | May contain any arbitrary sequence of bytes.                 | string   | ByteString | bytes                                | []byte   |

## Optional Fields and Default Values
如果 optional field 没有被设置，序列化会被设置一个默认值，默认值可以定义时写好

```proto
optional int32 result_per_page = 3 [default = 10];
```
如果没有设置：对于 string，默认是空串。对于 bytes，默认也是空。对于 bools， 默认是 false。对于整型，默认是 0。对于 enums，默认是第一个。

## Enumerations
```proto
message SearchRequest {
  required string query = 1;
  optional int32 page_number = 2;
  optional int32 result_per_page = 3 [default = 10];
  enum Corpus {
    UNIVERSAL = 0;
    WEB = 1;
    IMAGES = 2;
    LOCAL = 3;
    NEWS = 4;
    PRODUCTS = 5;
    VIDEO = 6;
  }
  optional Corpus corpus = 4 [default = UNIVERSAL];
}
```
可以对于不同的 enum 定义相同的值，但是要声明 `allow_alias = true`,如果没有声明，protoc 就会报错

```proto
enum EnumAllowingAlias {
  option allow_alias = true;
  UNKNOWN = 0;
  STARTED = 1;
  RUNNING = 1;
}
enum EnumNotAllowingAlias {
  UNKNOWN = 0;
  STARTED = 1;
  // RUNNING = 1;  // Uncommenting this line will cause a compile error inside Google and a warning message outside.
}
```

enum 内容必须是**32位整数**。因为 enum 类型使用 varint encoding，负数的编码效率很低，所以不推荐使用负数

### Reserved Values
如果你要删除一些 enum value 或者注释，也要使用 reverved 声明保留，以免后人重用引发问题

```proto
enum Foo {
  reserved 2, 15, 9 to 11, 40 to max;
  reserved "FOO", "BAR";
}
```

## Import
将 PB3 的 message import 到 PB2 的文件中使用是可行的，反之亦然。但是 PB2 的 enum 不能用在 PB3 中

## 类型嵌套
```proto
message SearchResponse {
  message Result {
    required string url = 1;
    optional string title = 2;
    repeated string snippets = 3;
  }
  repeated Result result = 1;
}

message SomeOtherMessage {
  optional SearchResponse.Result result = 1;
}
```
### Groups
不要用这个特性，同样 required 最好也不要用，这两个特性 PB3 中移除了

## 更新 Message

这里请看[原文](https://developers.google.com/protocol-buffers/docs/proto#updating)，这是更新一个 message 尽可能遵守的原则

## Extensions

extensions 可以让你在 message 声明一些 field numbers 给第三方 extensions 使用。extensions 是占位符，field number 没有在本 .proto 文件中定义。允许其他 .proto 文件定义这些 field number，看个例子：

```proto 
message Foo {
  // ...
  extensions 100 to 199;
}
```
这表明 [100,199] 保留给 extensions 使用。其他用户可以通过 import 这个文件定义 Foo 的 field number，比如

```proto
extend Foo {
  optional int32 bar = 126;
}
```
在访问 extend 的字段时，代码与一般定义的不同，比如 C++ 中

```cpp
Foo foo;
foo.SetExtension(bar, 15);
```

Extensions 可以是已经提到任意类型，不能是后面提到的 oneof 或者 map

### 嵌套
```proto
message Baz {
  extend Foo {
    optional int32 bar = 126;
  }
  ...
}
```
代码使用类似这样:

```cpp
Foo foo;
foo.SetExtensino(Baz::bar, 15);
```
影响就是 Foo 的扩展定义位于 message 的 scope，C++ 就是名字空间加了限制

```proto
message Baz {
  ...
}

// This can even be in a different file.
extend Foo {
  optional Baz foo_baz_ext = 127;
}
```
这样的写法可能更清晰

### 选择扩展数字
```proto
message Foo {
  extensions 1000 to max;
}
```
max 是 $2^{29} - 1$，或者 536,870,911

与一般选择 field number 一样，也要避免使用 19000 到 10000（FieldDescriptor :: kFirstReservedNumber到FieldDescriptor :: kLastReservedNumber）这是为 PB 实现保留的。

## Oneof
如果你的 message 中有很多 optional 的字段，并且同时最多只有一个 optional 的字段会被 set，那么可以使用 oneof 特性来提升编码效率，进一步压缩空间。类似 CPP 中 UNION

### 使用 Oneof
```proto
message SampleMessage {
  oneof test_oneof {
    string name = 4;
    SubMessage sub_message = 9;
  }
}
```

oneof 中定义的字段不能使用 required, optional, repeated 标识关键字。如果需要增加一个 repeated 字段，需要使用 Message 包裹起来

### Oneof 特性
- set 其中一个字段，其他字段都被清空，所以只会最后一个 set 的生效
  ```cpp
  SampleMessage message;
  message.set_name("name");
  CHECK(message.has_name());
  message.mutable_sub_message();   // Will clear name field.
  CHECK(!message.has_name());
  ```
- 如果解析器看到 oneof 中有多个字段，解析出来的 message 只会是最后一个
- Extensions 不支持 oneof
- 反射 API 也可以用于 oneof
- 可以设置一个 oneof 字段默认值，然后最后一个被序列化
- 如果使用 C++，注意代码不要 crash。下面的代码就会 crash，因为 sub_message 已经被删除了，当调用 set_name 时
  ```cpp
  SampleMessage message;
  Submessage *sub_message = message.mutable_sub_message();
  message.set_name("name"); // will delete sub_message
  sub_message->set_... // crash here
  ```
- 还是 C++，如果你 Swap 两个有 oneof 特性的 message，会有对方最后一个 oneof，例子中， msg1 有 sub_message， msg2 有 name
  ```cpp
  SampleMessage msg1;
  msg1.set_name("name");
  SampleMessage msg2;
  msg2.mutable_sub_message();
  msg1.swap(&msg2);
  CHECK(msg1.has_sub_message());
  CHECK(msg2.has_name());
  ```
### 后向兼容问题
注意增加，删除 oneof 字段时，如果 check 返回 None/NOT_SET，意味着 oneof 没有被 set 或者设置其他版本的 oneof。无法区分

所以 ！！！ 就认为 oneof 没有兼容性

## Maps
```proto
map<key_type, value_type> map_field = N;
```
**key_type 任意整数类型或者字符串类型。注意 enum 不能作为 key_type。value_type 可以是出了 map 之外的任何类型**

map API 现在支持所有 PB2 的语言

### 特性
- Extensions 不支持 map
- maps 不使用 repeated, optional, required
- Wire format 顺序和 map 迭代是不确定的
- 生成文本格式时，map 通过 key 排序，数字 key 以数字顺序
- 从 wire 格式解析，或者 merging，如果有重复 map key，使用后面的值。解析文本格式，如果有重复 key 就会报错

### 兼容
map 只是一个语法糖，实际上与下面的定义相等，所以即使 PB 实现不支持 map，也可以处理数据

```proto
message MapFieldEntry {
  optional key_type key = 1;
  optional value_type value = 2;
}

repeated MapFieldEntry map_field = N;
```

## 定义服务
```proto
service SearchService {
  rpc Search(SearchRequest) returns (SearchResponse);
}
```

默认，PB 编译器会生成抽象接口 `SearchService` 以及关联 stub 实现。stub 传递所有的调用为 RpcChannel，是一个抽象接口，你需要自己定义接口逻辑。比如，你可以将 RpcChannel 实现为序列化一个 message 然后通过 HTTP 发送到一个服务端。换句话说，生成的 stub 提供了一个类型安全的接口，并没有限制你做任何实现。所以，C++代码的例子如下：


```cpp
using google::protobuf;

protobuf::RpcChannel* channel;
protobuf::RpcController* controller;
SearchService* service;
SearchRequest request;
SearchResponse response;

void DoSearch() {
  // You provide classes MyRpcChannel and MyRpcController, which implement
  // the abstract interfaces protobuf::RpcChannel and protobuf::RpcController.
  channel = new MyRpcChannel("somehost.example.com:1234");
  controller = new MyRpcController;

  // The protocol compiler generates the SearchService class based on the
  // definition given above.
  service = new SearchService::Stub(channel);

  // Set up the request.
  request.set_query("protocol buffers");

  // Execute the RPC.
  service->Search(controller, request, response, protobuf::NewCallback(&Done));
}

void Done() {
  delete service;
  delete channel;
  delete controller;
}
```
所有的 service 类实现 Service 接口，这样提供了编译器不知道方法名称或者输入输出类型的进行调用的方法。服务端，可以这样实现：

```cpp
using google::protobuf;

class ExampleSearchService : public SearchService {
 public:
  void Search(protobuf::RpcController* controller,
              const SearchRequest* request,
              SearchResponse* response,
              protobuf::Closure* done) {
    if (request->query() == "google") {
      response->add_result()->set_url("http://www.google.com");
    } else if (request->query() == "protocol buffers") {
      response->add_result()->set_url("http://protobuf.googlecode.com");
    }
    done->Run();
  }
};

int main() {
  // You provide class MyRpcServer.  It does not have to implement any
  // particular interface; this is just an example.
  MyRpcServer server;

  protobuf::Service* service = new ExampleSearchService;
  server.ExportOnPort(1234, service);
  server.Run();

  delete service;
  return 0;
}
```

如果不想实现你自己的 RPC 系统，可以直接使用 gRPC。
## Options

`.proto`文件可以通过 *options* 来 annotate，会影响处理的上下文。所有的 *options* 定义在 `google.protobuf/descriptor.proto`

- file-level options 
- Message-level options
- filed-level options

比如：

- java_package (file option) 

  ```proto
  option java_package = "com.example.foo";
  ```

  生成的 java 代码 package 

- optimize_for (file option) SPEED(default), CODE_SIZE, LITE_RUNTIME
  会影响 C++ 和 Java 代码的生成

- message_set_wire_format (message option) 对于C++代码开启 arena allocation 

  ```proto
  message Foo {
  	option message_set_wire_format = true;
  	extensions 4 to max;
  }
  ```

  只是例子，Google 之外的开发者不需要这个

- packed (field option)：如果在 repeated numeric type 上开启，encode 更加紧凑，这个 option 没有坏处，pb3 默认开启

  ```proto
  repeated int32 samples = 4 [packed = true];
  ```

- deprecated (filed option): 开启表明这个 filed 废弃，Java 中会加上 @Deprecated 注解

### custom options

还可以自定义 options

example: 

```proto
import "google/protobuf/descriptor.proto"

extend google.protobuf.MessageOptions {
	optional string my_option = 51234;
}

message MyMessage {
	optional (my_option) = "Hello world!";
}

// 这样我们定义了 message-level option
```

然后 C++ 中可以这样使用

```cpp
string value = MyMessage::descriptor()->options().GetExtension(my_option);
```

```proto
import "google/protobuf/descriptor.proto"

extend google.protobuf.FileOptions {
	optional string my_file_option = 50000;
}
extend google.protobuf.MessageOptions {
  optional int32 my_message_option = 50001;
}
extend google.protobuf.FieldOptions {
  optional float my_field_option = 50002;
}
extend google.protobuf.OneofOptions {
  optional int64 my_oneof_option = 50003;
}
extend google.protobuf.EnumOptions {
  optional bool my_enum_option = 50004;
}
extend google.protobuf.EnumValueOptions {
  optional uint32 my_enum_value_option = 50005;
}
extend google.protobuf.ServiceOptions {
  optional MyEnum my_service_option = 50006;
}
extend google.protobuf.MethodOptions {
  optional MyMessage my_method_option = 50007;
}

option (my_file_option) = "Hello world!";

message MyMessage {
  option (my_message_option) = 1234;

  optional int32 foo = 1 [(my_field_option) = 4.5];
  optional string bar = 2;
  oneof qux {
    option (my_oneof_option) = 42;

    string quux = 3;
  }
}

enum MyEnum {
  option (my_enum_option) = true;

  FOO = 1 [(my_enum_value_option) = 321];
  BAR = 2;
}

message RequestType {}
message ResponseType {}

service MyService {
  option (my_service_option) = FOO;

  rpc MyMethod(RequestType) returns(ResponseType) {
    // Note:  my_method_option has type MyMessage.  We can set each field
    //   within it using a separate "option" line.
    option (my_method_option).foo = 567;
    option (my_method_option).bar = "Some string";
  }
}
```



# protobuf3 语言

[原文](https://developers.google.com/protocol-buffers/docs/proto3)

```proto
syntax = "proto3";
message SearchRequest {
  string query = 1;
  int32 page_number = 2;
  int32 result_per_page = 3;
}
```

需要声明 syntax = "proto3"

同 PB2 一样，tag number 范围 $1 到 2^{29} - 1$，并且不能使用 19000 到 19999

PB3 默认是 optional，没有 required 关键字，还有 repeated，并且 repeated 默认使用 packed 特性编码

## define a message type

```proto
syntax = "proto3";

message SearchRequest {
	string query = 1;
	int32 page_number = 2;
	int32 result_per_page = 3;
}

// 默认是 optional , repeated 才需要声明
```

###  Reverse field

同 pb2 中的内容

## Scalar Value Types

与 PB2 基本一样，不过 string 和 bytes 加了最长限制 
- string 不能超过 $2^{32}$
- bytes 不能超过 $2^{32}$

## 默认值
- string，默认空串
- bytes，默认空序列
- bool，默认 false
- 整型，默认 0
- enum，默认第一个，必须是0
- message 字段，看特定语言 API
- repeated，默认没有

## Enumerations
- PB3 中第一是0，作为默认值
- 0 必须是第一个元素，为了与 PB2 兼容，作为默认值

PB2 不要求 0 是第一个值

## Unknown Fields
老代码解析新数据，可能有不认识的 field

最开始，pb3 在碰到不识别的 field，简单丢弃，在 version 3.5 我们为了匹配 pb2 行为重新引入了保护机制。在 version 3.5 或者更新，在解析过程中不认识的 field 被保留，不会删除
## Any
特性开发中，有点类似 pb2 中的 message extensions 机制

## JSON mapping
PB3 支持了官方的对 JSON 的编码，方便与系统之间共享数据。

If a value is missing in the JSON-encoded data or if its value is `null`, it will be interpreted as the appropriate [default value](https://developers.google.com/protocol-buffers/docs/proto3#default) when parsed into a protocol buffer. If a field has the default value in the protocol buffer, it will be omitted in the JSON-encoded data by default to save space. An implementation may provide options to emit fields with default values in the JSON-encoded output.

| proto3                 | JSON          | JSON example                              | Notes                                                        |
| :--------------------- | :------------ | :---------------------------------------- | :----------------------------------------------------------- |
| message                | object        | `{"fooBar": v, "g": null, …}`             | Generates JSON objects. Message field names are mapped to lowerCamelCase and become JSON object keys. If the `json_name` field option is specified, the specified value will be used as the key instead. Parsers accept both the lowerCamelCase name (or the one specified by the `json_name` option) and the original proto field name. `null` is an accepted value for all field types and treated as the default value of the corresponding field type. |
| enum                   | string        | `"FOO_BAR"`                               | The name of the enum value as specified in proto is used. Parsers accept both enum names and integer values. |
| map<K,V>               | object        | `{"k": v, …}`                             | All keys are converted to strings.                           |
| repeated V             | array         | `[v, …]`                                  | `null` is accepted as the empty list `[]`.                   |
| bool                   | true, false   | `true, false`                             |                                                              |
| string                 | string        | `"Hello World!"`                          |                                                              |
| bytes                  | base64 string | `"YWJjMTIzIT8kKiYoKSctPUB+"`              | JSON value will be the data encoded as a string using standard base64 encoding with paddings. Either standard or URL-safe base64 encoding with/without paddings are accepted. |
| int32, fixed32, uint32 | number        | `1, -10, 0`                               | JSON value will be a decimal number. Either numbers or strings are accepted. |
| int64, fixed64, uint64 | string        | `"1", "-10"`                              | JSON value will be a decimal string. Either numbers or strings are accepted. |
| float, double          | number        | `1.1, -10.0, 0, "NaN", "Infinity"`        | JSON value will be a number or one of the special string values "NaN", "Infinity", and "-Infinity". Either numbers or strings are accepted. Exponent notation is also accepted. -0 is considered equivalent to 0. |
| Any                    | `object`      | `{"@type": "url", "f": v, … }`            | If the Any contains a value that has a special JSON mapping, it will be converted as follows: `{"@type": xxx, "value": yyy}`. Otherwise, the value will be converted into a JSON object, and the `"@type"` field will be inserted to indicate the actual data type. |
| Timestamp              | string        | `"1972-01-01T10:00:20.021Z"`              | Uses RFC 3339, where generated output will always be Z-normalized and uses 0, 3, 6 or 9 fractional digits. Offsets other than "Z" are also accepted. |
| Duration               | string        | `"1.000340012s", "1s"`                    | Generated output always contains 0, 3, 6, or 9 fractional digits, depending on required precision, followed by the suffix "s". Accepted are any fractional digits (also none) as long as they fit into nano-seconds precision and the suffix "s" is required. |
| Struct                 | `object`      | `{ … }`                                   | Any JSON object. See `struct.proto`.                         |
| Wrapper types          | various types | `2, "2", "foo", true, "true", null, 0, …` | Wrappers use the same representation in JSON as the wrapped primitive type, except that `null` is allowed and preserved during data conversion and transfer. |
| FieldMask              | string        | `"f.fooBar,h"`                            | See `field_mask.proto`.                                      |
| ListValue              | array         | `[foo, bar, …]`                           |                                                              |
| Value                  | value         |                                           | Any JSON value. Check [google.protobuf.Value](https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#google.protobuf.Value) for details. |
| NullValue              | null          |                                           | JSON null                                                    |
| Empty                  | object        | `{}`                                      | An empty JSON object                                         |

### JSON options

A proto3 JSON implementation may provide the following options:

- **Emit fields with default values**: Fields with default values are omitted by default in proto3 JSON output. An implementation may provide an option to override this behavior and output fields with their default values.
- **Ignore unknown fields**: Proto3 JSON parser should reject unknown fields by default but may provide an option to ignore unknown fields in parsing.
- **Use proto field name instead of lowerCamelCase name**: By default proto3 JSON printer should convert the field name to lowerCamelCase and use that as the JSON name. An implementation may provide an option to use proto field name as the JSON name instead. Proto3 JSON parsers are required to accept both the converted lowerCamelCase name and the proto field name.
- **Emit enum values as integers instead of strings**: The name of an enum value is used by default in JSON output. An option may be provided to use the numeric value of the enum value instead.

# protobuf style
[原文](https://developers.google.com/protocol-buffers/docs/style)

请注意，PB 已经随着时间而发展，因此你可能会看到不同风格编写的 .proto 文件，修改这些文件时，请尊重现有风格，一致性是关键。但是创建新的 .proto 文件时，请采用当前的最新风格
## 标准文件格式
- 每行不超过 80 字符
- 使用 2 空格缩进
- 最好对字符串使用双引号

## 文件结构

文件命名 `lower_snake_case.proto`

所有文件应该以下列顺序排布：
1. License header (if applicable)
2. File overview
3. Syntax
4. Package
5. Imports (sorted)
6. File options
7. Everything else 

## Packages
Packages 名称应该小写，还应该匹配文件层级。比如如果文件在 `my/package/`，package 名称应该为 `my.package`

## Message 和 field names
使用 CamelCase 命名 message -- 比如，`SongServerRequest`，使用 underscore_separated_names 命名 field name，比如 `song_name`

```proto
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

## Repeated fields
使用复数命名这种 field

```proto
repeated string keys = 1;
...
repeated MyMessage accounts = 17;
```

## Enums
使用 CamelCase 命名 enum type name，使用 CPPITALS_WITH_UNDERSOCRES 命名 value name

```proto
enum FooBar {
  FOO_BAR_UNSPECIFIED = 0;
  FOO_BAR_FIRST_VALUE = 1;
  FOO_BAR_SECOND_VALUE = 2;
}
```
每个 enum 应该以分号结尾而不是逗号。需要为 enum 值添加前缀，因为历史代码的兼容，没有引入 C++11 的 enum class scope，零 enum 值应该有后缀

## Services
如果你的 .proto 文件定义 RPC 服务，应该使用 CamelCase 风格，同时应用于服务名和任何RPC方法名

```proto
service FooService {
  rpc GetSomething(FooRequest) returns (FooResponse);
}
```

## Things to avoid
- 只有 proto2 存在 required field
- 只有 proto2 存在 Groups

# 编码
[原文](https://developers.google.com/protocol-buffers/docs/encoding)

这篇文档阐述了 protobuf 的 message 二进制编码的原理。你在应用中使用 pb 时不需要理解这个，但是知道这些可以更好地帮助你了解使用 pb 编码之后的消息大小
## A simple Message

来看下这个简单的 message 定义：

```proto
message Test1 {
	optional int32 a = 1;
}
```

应用中，你可以创建 `Test1` message 然后 `set a ` 为 150，然后序列化这个 message 作为输出流。如果打印这个流，你可以看到三个字节的内容：

```bash
08 96 01
```

哇哦，如此小，这意味着什么？且慢慢往下读

## Base 128 Varints

为了理解简单的 protocol buffer 编码，首先你需要理解 ***varints***，Varints 是一种使用一个或者更多字节序列化整数的方法，越小的数字使用越少的字节数。

除了最后一个字节外，varint 每个字节都有最高有效位（msb），这表明还有更多的字节在后面。这个字节的低 7 位为一组来存储数字的补码表示，**最低有效组在前**。

比如，这有一个数字 1，简单一个字节：

```bash
0000 0001 # 1
```

300 就要复杂一些：

```bash
1010 1100 0000 0010 # 300
```

你如何认出这是 300 呢，首先丢掉每个字节的最高有效位（只是表示是不是最后一个字节）

```bash
1010 1100 0000 0010
->
 010 1100  000 0010
```

然后以 7 位一组反向整个二进制序列，就像前面所述，varints 保存数字式，**最小有效组在前**，然后你可以按照补码的正常计算方式来计算数字（正数的补码就是直接转化的二进制）

```bash
000 0010  010 1100
→  000 0010 ++ 010 1100
→  100101100
→  256 + 32 + 8 + 4 = 300
```

## Message Structure

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

```bash
000 1000
```

你可以通过获取最后三位得到声明的类型（0），然后右移三位获得域号（1）。现在你知道了域号是 1，接下来的字节还是 varint。使用前面知道的解码知识，你可以看到后面两个字节存储了 150 这个值

```bash
96 01 = 1001 0110  0000 0001
       → 000 0001  ++  001 0110 (drop the msb and reverse the groups of 7 bits)
       → 10010110
       → 128 + 16 + 4 + 2 = 150
```

## More Value Types
### Signed Integers

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

```bash
(n << 1) ^ (n >> 31)   # for sint32
(n << 1) ^ (n >> 63)   # for sint64
```

注意第二个位右移--`(n >> 31)`是算术移位的。换句话说，移位的结果不是全 0（n 为正数），就是全 1（n 为负数）。

当`sint32/sint64`被解析时，其值将解码回原始的带符号版本。

### Non-Varint Number

非 varint 数字类型就简单的两种`double 和 fixed64`，域类型为 1，告诉解析器是一个固定的 64 位的数据块；类似的`float 和 fixed32`，域类型为 5，告诉解析器固定为 32 位。这两种情况都被存储为小端字节序。

### Strings

域类型 2（长度受限）表示该值是 varint 编码的长度，后面跟着指定长度的字节。

```proto
message Test2 {
	optional string b = 2;
}
```

设置值为"testing"编码为：（TLV）

```bash
12 07 74 65 73 74 69 6e 67
```

这是"testing"的 UTF-8。key 是 0x12 ->

```bash
0001 0010 
-> 00010 010
```

得到 field_number = 2，wire_type = 2。varint 的中的值长度为 7，lo 且为 0，我们在其后找到七个字节---我们的字符串。

## Embedded Messages

有一个 message 定义中包含了之前的 Test1：

```proto
message Test3 {
	optional Test1 c = 3;
}
```

这是将 Test1 的 a 设置为 150 的编码版本：

```bash
1a 03 08 96 01
```

正如你所看到的，最后三个字节跟我们第一个例子中一样（`08 96 01`），在数字 3 后面 - 内嵌的 message 跟 strings 的方式是一样的。

## Optional and Repeated Elements

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

### Packed Repeated Fields

版本 2.1.0 引入了`packed repeated field`，在 proto2 中声明为`repeated`域而且具有`[packed=ture]`参数的，在 proto3 中，`repeated`默认使用 packed 打包标量数字类型。这些功能类似`repeated fields`，但是编码方式不同。`packed repeated field`包含 0 的元素不会出现在编码后的 message 中。另外，该域的所有元素都打包成域类型为 2 的单个 key-value 对。每个元素的编码方式与之前相同，除了在前面没有 key。

比如说，假设有一个 message type：

```proto
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

## Field Order

域号（Field Number）可以在`.proto`文件中以任何顺序使用。顺序的选择不会影响 message 的序列化。

当 message 序列化时，对于如何写入已知或者未知域没有顺序保证。序列化顺序是一个实现细节，将来任何实现的细节可能都会改变。因此 protobuf 的解析器必须能够解析任意顺序域的编码。

### Implications 含义

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

# 序列化原理

Protocol Buffers 是一种轻便高效的结构化数据存储格式，可以用于结构化数据串行化，或者说序列化。它很适合做数据存储或数据交换格式。可用于通讯协议、数据存储等领域的语言无关、平台无关、可扩展的序列化结构数据格式

protobuf2 中修饰符：

- required : 不可以增加或删除的字段，必须初始化；
- optional :  可选字段，可删除，可以不初始化；
- repeated : 可重复字段， 对应到 java 文件里，生成的是 List。

## protobuf 的使用

```bash
protoc -I=SRC_DIR --cpp_out=DST_DIR person.proto
```

## 编码原理（重点是类型和域号）

protobuf 中的 message 中有很多字段，每个字段的格式：

```proto
修饰符 字段类型 字段名 = 域号；
```

在序列化时，protobuf 按照`TLV`的格式序列化每一个字段，T 即 Tag，L 是 value 的长度, V 是该字段对应的 value，。如果字段是一个整形，L 部分会省略。

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

# cap'n proto
[官网](https://capnproto.org/index.html)

# cpp-serializers 对比 benchmark
[github](https://github.com/thekvs/cpp-serializers)

# 更新记录
2021-05-07 init 翻译 protobuffer 官网 encode 的文档

2022-01-20 补充增加了 Options 和 custom options 部分