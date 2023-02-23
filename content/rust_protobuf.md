+++
template = "blog/page.html"
date = "2022-01-19 14:38:11"
title = "Rust Protobuf 对比分析(prost vs rust-protobuf)"
[taxonomies]
tags = ["protobuf", "rust"]

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

# [prost](https://github.com/tokio-rs/prost)
优势：
- generates simple, idiomatic and readable Rust types
- 保留 proto 中的注释
- **可以在已有的 Rust types 中增加 attributes 来转换为 Message**
- 使用 `bytes::{Buf, BufMut}` 来抽象而不是 `std::io::{Read, Write}`
- 可以识别 package 自动转换为 Rust module

*不支持 reflection*

## generated code
prost 可以根据 `.proto` 文件生成 rust 代码
## Packages
`package foo.bar` -> `foo::bar` module
### Messages
message <-> Rust struct
```
message Foo {
}

// <->

#[derive(Clone, Debug, PartialEq, Message)]
pub struct Foo {
}
```
### Fields
protobuf message fields 对应 Rust struct pub fields
#### Scalar Values
|protobuf type| rust type|
|---|---|
|double| f64|
|float|f32|
|int32,sint32,sfixed32|i32|
|int64,sint64,sfixed64|i64|
|uint32, fixed32|u32|
|uint64, fixed64|u64|
|bool|bool|
|string|String|
|bytes|Vec<u8>|
#### Enumerations
`.proto` 枚举对应 Rust 的 `i32`
```
enum PhoneType {
  MOBILE = 0;
  HOME = 1;
  WORK = 2;
}

pub enum PhoneType {
  Mobile = 0,
  Home = 1,
  Work = 2,
}

// PhoneType::Mobile as i32 可行
// #[derive(::prost::Enumeration)] 会给 `PhoneType` 增加方法
impl PhoneType {
  pub fn is_valid(value: i32) -> bool
  pub fn from_i32(value: i32) -> Option<PhoneType>
}
```

#### Field Modifiers
|`.proto` Version|modifier|Rust Type|
|----|----|----|
|proto2|optional|Option<T>|
|proto2|required|T|
|proto3|default|T|
|proto2/proto3|repeated|Vec<T>|
#### Map Fields
Rust `HashMap`
#### Message Fields
`proto3` 默认会生成 `Option<T>`

message 会 Box 避免无限 size 的 struct
#### Oneof Fileds
Oneof 会转换成 enum

```
message Foo {
  oneof widget {
    int32 quux = 1;
    string bar = 2;
  }
}

pub struct Foo {
  pub widget: Option<foo::Widget>,
}
pub mod foo {
  pub enum Widget {
    Quux(i32),
    Bar(String),
  }
}
```
### Services
`prost-build` 支持自定义处理 `service` 定义，根据项目需要生成 `trait`

### Example
```proto
syntax = "proto3";
package tutorial;

message Person {
  string name = 1;
  int32 id = 2; // unique id number for this person
  string email = 3;

  enum PhoneType {
    MOBILE = 0;
    HOME = 1;
    WORK = 2;
  }

  message PhoneNumber {
    string number = 1;
    PhoneNumber type = 2;
  }
  repeated PhoneNumber phones = 4;
}

// our address book file is just one of these
message AddressBook {
  repeated Person people = 1;
}
```

生成的代码
```rust
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// unique id number for this person
    #[prost(int32, tag="2")]
    pub id: i32,
    #[prost(string, tag="3")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub phones: ::prost::alloc::vec::Vec<person::PhoneNumber>,
}
/// Nested message and enum types in `Person`.
pub mod person {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhoneNumber {
        #[prost(string, tag="1")]
        pub number: ::prost::alloc::string::String,
        #[prost(message, optional, boxed, tag="2")]
        pub r#type: ::core::option::Option<::prost::alloc::boxed::Box<PhoneNumber>>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PhoneType {
        Mobile = 0,
        Home = 1,
        Work = 2,
    }
}
/// our address book file is just one of these
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBook {
    #[prost(message, repeated, tag="1")]
    pub people: ::prost::alloc::vec::Vec<Person>,
}

```

## 序列化已经存在的类型
Prost 会根据声明的顺序自动推断 struct 中 field 的 tag，加上合适的 derive and field annotation 就能实现序列化/反序列化已有的类型

```rust
use prost;
use prost::{Enumeration, Message};

#[derive(Clone, PartialEq, Message)]
struct Person {
    #[prost(string, tag = "1")]
    pub id: String, // tag=1
    // NOTE: Old "name" field has been removed
    // pub name: String, // tag=2 (Removed)
    #[prost(string, tag = "6")]
    pub given_name: String, // tag=6
    #[prost(string)]
    pub family_name: String, // tag=7
    #[prost(string)]
    pub formatted_name: String, // tag=8
    #[prost(uint32, tag = "3")]
    pub age: u32, // tag=3
    #[prost(uint32)]
    pub height: u32, // tag=4
    #[prost(enumeration = "Gender")]
    pub gender: i32, // tag=5
    // NOTE: Skip to less commonly occurring fields
    #[prost(string, tag = "16")]
    pub name_prefix: String, // tag=16  (eg. mr/mrs/ms)
    #[prost(string)]
    pub name_suffix: String, // tag=17  (eg. jr/esq)
    #[prost(string)]
    pub maiden_name: String, // tag=18
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
pub enum Gender {
    Unknown = 0,
    Female = 1,
    Male = 2,
}
```


# [rust-protobuf](https://github.com/stepancheg/rust-protobuf)
[doc](https://docs.rs/protobuf/latest/protobuf/index.html)
[rust codegen doc](https://docs.rs/protobuf-codegen-pure/2.25.2/protobuf_codegen_pure/)

优势：
- 生成的代码有运行时库 ？（prost 可以 no_std 支持），支持 reflection
- 更像 cpp protobuf 版本的生成文件（不好说是优势还是劣势）

劣势：
- 文档较少，比如类型之间的对应关系


# 对比
生成的代码行数 rust-protobuf 是 prost 的十倍，
prost 生成的 `.rs` 几乎与 `.proto` 行数一致

----
PS.
[cargo build script doc](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
cargo build 在编译之前会先编译 build.rs 文件并执行，The script may communicate with Cargo by printing specially formatted commands prefixed with `cargo:` to stdout，比如 `println!("cargo:rerun-if-changed={}", file_name);`，默认 package 文件任何一个变更都会重新执行

```rust
// build.rs
use anyhow::Result;

#[rustfmt::skip]
const PROTOBUF_FILES: &[(&str, &str)] = &[
    ("a", "a.proto"),
    ("b", "b.proto"),
    ("c", "c.proto"),
];

fn main() -> Result<()> {
    for (dir, file) in PROTOBUF_FILES {
        let file_path = format!("{}/{}", dir, file);
        println!("cargo:rerun-if-changed={}", file_path);
        // prost
        prost_build::Config::new()
            .out_dir(dir)
            .compile_protos(&[&file_path], &[dir])?;

        // rust-protobuf
        protobuf_codegen_pure::Codegen::new()
            .out_dir("src/protos")
            .input(&file_path)
            .include(dir)
            .run()
            .expect("Codegen failed");
    }

    Ok(())
}

```





