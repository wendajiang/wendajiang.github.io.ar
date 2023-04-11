---
template: blog/page.html
date: 2022-09-14 20:49:31
title: DPI C Layer
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["ieee1800"]
extra:
  mermaid: true
  usemathjax: true
  toc: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

[svdpi.h](https://wendajiang.github.io/svdpi.h)

# Overview

# Naming Conventions

# Portablility

# Semantic constrints

以上基本与 ieee_1800_2017 描述相似

# Data types

### Data representation

DPI 在 SV 数据类型表征上加入了如下约束：

- unpakced 的 SV 类型和不包含 packed 元素的类型兼容 C 表征
- 基本整数和实数类型在[这里](#basic-types)定义
- packed 类型，包括 time ，整数和用户定义类型，使用[这里](#mapping-between-sv-ranges-and-c-ranges) 定义的规范形式
- ![image-20220913185228713](https://wendajiang.github.io/pics/ieee_dpi_c_layer/image-20220913185228713.png)
- 嵌入 structure 的 unpacked array 与 C 的 layout 兼容。类似的标准 array 作为实参传递给定长形参也是可以的
- ![image-20220913185517604](https://wendajiang.github.io/pics/ieee_dpi_c_layer/image-20220913185517604.png)
- ![image-20220913185541864](https://wendajiang.github.io/pics/ieee_dpi_c_layer/image-20220913185541864.png)

### Basic types

| SV type         | C type        |
| --------------- | ------------- |
| byte            | char          |
| shortint        | short int     |
| int             | int           |
| longint         | long long     |
| real            | double        |
| shortreal       | float         |
| chandle         | void *        |
| string          | const char *  |
| $$bit^a$$       | unsigned char |
| $$logic^a/reg$$ | unsigned char |

input 模式的参数 byte unsigned 和 shorting unsigned 不同于 bit[7:0] bit[15:0] ，前面使用 C 的 unsigned char 和 unsigned short ，后面的使用 svBitVecVal 类型。类似的等效性也存在于 output 和 inout 模式，但是是通过 C 的对应指针传递。

### Normialized and linearized ranges

packed array 是一维的；unpacked 是多维的。归一化 ranges 意味着 [n - 1:0] 索引 packed array，[0:n - 1] 索引 unpacked array。除了 open array 的 unpacked 部分，array 参数都适用归一化。标准 packed array 表示也使用归一化索引。

线性化多维 SV array，表示 (i, j, k) 展开成 i * j * k 的一维。一维数组与多维的具有相同的内存布局。用户的 C 代码引用线性数组元素时可以使用原始维度。比如 SV packed 2-state 数组维度 (i, j, k) 对应的 `myArray[l][m][n]`（归一化之后的） 在 C 代码中使用 `bit(n + (m * k) + (l * j * k))` 访问。

### Mapping between SV ranges and C ranges

![image-20220914192359417](https://wendajiang.github.io/pics/ieee_dpi_c_layer/image-20220914192359417.png)

比如 `logic[2:3][1:3][2:0] b[1:10][31:0]` 的归一化版本 `logic[17:0] b[0:9][0:31]`

### Canonical representation of packed arrays

DPI 定义了 canonical 的 packed 2-state 表示 `svBitVecVal` 和 4-state 数组 `svLogicVecVal` 

# Argument passing modes

SV exported  的函数和任务不能有 open array 参数。

small value 都是传值。函数返回值也是传值。

按引用传递就是指针。caller 需要负责分配内存空间。比如 C 调用 SV，需要先申请好内存，然后指针传递。

passing by handle -- open array 没有尺寸的数组。handle 的实现依赖工具，对用户是透明的。handle 通常使用 `void *` 表示 (`svOpenArrayHandle`)。handle 总是使用 const 修饰。

input 参数在 C 中也应该总是 const 修饰的，small value 除外使用引用传递， small value 如下

- byte, shortin, int, longint, real, shortreal
- scalar bit and logic
- chandle, string

函数返回值只能是 small value

### String 参数

string 从 SV 传递到 C 中，内存布局应该兼容 C 的，包括尾部的 `'\0'` 。类似 C 传递到 SV 也应该确保 null-terminated。

string 参数的方向适用于 string 的指针，而不是构成字符串的字符

对于 imported 的函数和任务：

- input 模式的 string 可以是 SV 提供的指针访问，并且不能在 C 代码中释放。在 C 的应用中不能有字符串存储声明周期的假设。并且用户（C 代码）不能修改
- output 模式的 string 开始没有意义。通过 `const char **` 方式提供。等回到  SV 侧，DPI 应用应该已经写入了有效值进去 `const char * 地址指向的 const char ** 的值`。SV 不能释放这个地址的内容
- inout 模式的 string 开始就有意义。string 的存储不应该被 DPI C 的应用释放。也不应该有生命周期的假设。任何对于 string 的修改应该由 C 程序提供新的地址，SV 不能修改和释放。如果修改了，SV 需要复制到自己的内存空间

对于 exported 的函数和任务：

- input 模式的 string 通过 `const char *` 传递给 SV。SV 只读。
- output 模式的 string 通过 `const char ** `表示。没有初始值。SV 会写一个有效地址进去，但是不保证生命周期，如果 C 中要用，需要复制数据到自己的内存空间。
- inout 模式的 string 也是 `const char **`。SV 只读用户的 string 空间，也不会试图释放。如果 SV 需要修改，就会对地址重新赋值。用户也不应该有生命周期的假设，如果需要使用，要拷贝自己的空间。

# Context tasks and functinos

略

# include files

头文件解析：

### Scalars of type bit and logic

```c
#define sv_0 0
#define sv_1 1
#define sv_z 2
#define sv_x 3

typedef unsigned char svScalar;
typedef svScalar svBit;
typedef svScalar svLogic;
```

### Canonical representation of packed arrays

```c
#ifndef VPI_VECVAL
#define VPI_VECVAL
typedef struct t_vpi_vecval {
  uint32_t aval;
  uint32_t bval;
} s_vpi_vecval, *p_vpi_vecval;
#endif

/* (a chunk of) packed logic array */
typedef s_vpi_vecval svLogicVecVal;

/* (a chunk of) packed bit array */
typedef uint32_t svBitVecVal;

/* Number of chunks required to represent the given width packed array */
#define SV_PACKED_DATA_NELEMS(WIDTH) (((WIDTH) + 31) >> 5)

/*
 * Because the contents of the unused bits is undetermined,
 * the following macros can be handy.
 */
#define SV_MASK(N) (~(-1 << (N)))

#define SV_GET_UNSIGNED_BITS(VALUE, N) \
    ((N) == 32 ? (VALUE) : ((VALUE)&SV_MASK(N)))

#define SV_GET_SIGNED_BITS(VALUE, N)                               \
    ((N) == 32 ? (VALUE)                                           \
               : (((VALUE) & (1 << (N))) ? ((VALUE) | ~SV_MASK(N)) \
                                         : ((VALUE)&SV_MASK(N))))
```

### Implementation-dependent representation

```c
const char* svDpiVersion();
typedef void* svCdope;
typedef void* svOpenArrayHandle;
```

# Arrays

略

# Open arrays

如果数组元素复合 C 的布局，不需要辅助函数，可以直接通过指针访问。【译者注：下面的辅助函数都是 canonical representation 访问，实现也是根据 spec 实现的，对用户透明。】

辅助函数查询数组的范围。

![image-20220914200459670](https://wendajiang.github.io/pics/ieee_dpi_c_layer/image-20220914200459670.png)

还有辅助函数可以从 open array 中复制数据。

访问数据辅助函数

![image-20220914200639050](https://wendajiang.github.io/pics/ieee_dpi_c_layer/image-20220914200639050.png)

# SV3.1a-compatible access to packed data(deprecated functionallity)

略
