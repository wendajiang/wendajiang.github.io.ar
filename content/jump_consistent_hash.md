+++
template = "page.html"
date = "2021-05-11 19:30:38"
title = "A Fast Minimal Memory, Consistent Hash Algorithm"
[taxonomies]
tags = ["awesomepaper", "algorithm", "translate"]

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

[原文](https://arxiv.org/pdf/1406.2294.pdf)

# Abstract
我们提出了 jump consistent hash,一种快速，内存占用小，一致性哈希算法，可以用 5 行代码实现。相比于 Karger 提出的算法，jump consistent hash 不需要内存，更快，在桶的数量变化时，可以将 key 的空间划分的更加均匀。主要局限性是必须对存储桶进行顺序编号，这使其更适用于数据存储应用而不是分布式 web 缓存

# Introduction
Karger 在[文章](https://www.akamai.com/it/it/multimedia/documents/technical-publication/consistent-hashing-and-random-trees-distributed-caching-protocols-for-relieving-hot-spots-on-the-world-wide-web-technical-publication.pdf)中提出一致性哈希的概念，并给出了一个算法实现。一致性哈希确保数据这样分布在服务器上，当服务器增加或者删除时，不会重排数据。最开始提出来是为了缓存互联网的 Web 缓存，为了解决客户端可能不知道所有缓存服务器的问题。

从那时起，一致性哈希广泛应用于数据存储应用。这里，描述问题为，将数据拆分到 shard 的集合上，典型的每个 shard 就是一个服务器。当数据量变化时，我们对机器进行增减。这要求将数据从老的 shard 集合移动到新的 shard 集合时，移动的数据量尽可能小

假设，比如，kv 数据被分散到 10 个 shard。简单的方法就是计算一个 key 的 hash 函数，`h(key)`，将 kv 数据存储到 `h(key) mod 10` 的 shard 上。但是如果数据规模增大，现在需要12个 shard 来存储，最简单的方法就是计算改为 `h(key) mod 12`，但是相同 key 计算出不同的结果，所以数据需要重新排布。

但是如果只需要移动存储在 10 shard 中的 $1/6$ 的数据，以便在 12 个 shard 中平衡，一致性哈希可以做到。我们的 jump consistent hash 函数需要两个参数，key 和桶的数量，返回一个桶的编号。这函数满足两个性质
- 每个桶的 key 个数相等
- 当桶的个数发生变化时，需要重映射的 key 的数量尽可能少

相比 Karger 提出的算法，jump consistent hash 算法非常快并且内存占用具有很大优势。Karger 的算法每个候选 shard 需要数千个字节的存储，以便获得 key 的分配。在大数据存储应用中，可能有数千个 shard，那意味着每个 client 需要 MB 内存来存储这个结构，并且要长期存储保证算法有效。相反，jump consistent hash 几乎不需要内存，并且分配 key 更均匀。另一方面，jump consistent hash 不支持服务器名称，只能返回服务器编号，因此主要适用于数据存储案例。

```cpp
int32_t JumpConsistentHash(uint64_t key, int32_t num_buckets)
{
    int64_t b = -1, j = 0;
    while (j < num_buckets) {
        b = j;
        key = key * 2862933555777941757ULL + 1;     
        j = (b + 1) * (double(1LL << 31) / double((key >> 33) + 1));
    }
    return b;
}
```
这就是实现。输入 64 位的整数 key，和桶的数量。输出一个 [0, num_buckets) 之间的数。本文的剩余部分就是解释代码意义，并给出理论证明和性能结果

性能分析对比和相关工作请参考[原文](https://arxiv.org/pdf/1406.2294.pdf)

# Explanation of the algorithm
jump consistent hash 当桶数量增加时，计算输出。当 num_buckets 个桶时， `ch(key, num_buckets)` 为 key 的映射即桶号。所以，对任何 key -> k, ch(k, 1) 是 0，因为仅有一个桶。然后增大桶数量，ch(k, 2) 需要将一半 key 移动到新桶 1 中。由此可见，ch(k, n + 1) 需要保持 ch(k, n) 中 $n/(n + 1)$ 的 key，然后移动 $1/(n + 1)$ 的 key 到桶 n 中。

举例，有三个 key：k1, k2, k3 ，随着桶数量增长的表格：

|      | 1    | 2    | 3    | 4    | 5    | 6    | 7    | 8    | 9    | 10   | 11   | 12   | 13   | 14   |
| ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| k1   | 0    | 0    | 2    | 2    | 4    | 4    | 4    | 4    | 4    | 4    | 4    | 4    | 4    | 4    |
| k2   | 0    | 1    | 1    | 1    | 1    | 1    | 1    | 7    | 7    | 7    | 7    | 7    | 7    | 7    |
| k3   | 0    | 1    | 1    | 1    | 1    | 5    | 5    | 7    | 7    | 7    | 10   | 10   | 10   | 10   |

可以通过 j 增加时 ch(key, j) 是不是需要 jump 的概率定义一个线性时间的算法。如同上面表格一行的变化一样。给定一个 key 和桶的数量，算法考虑下一个桶,j, 从 1 到 num_buckets - 1，使用 ch(key, j) 计算 ch(key, j + 1)。对于每个桶,j ，决定 ch(k, j + 1) 是否保持 ch(k, j) 的结果，还是将其移动到桶 j 中。为了得到 jump 的正确百分比，将 key 作为伪随机数的种子。如果随机值小于 $1/(j + 1)$ 就移动到当前桶，循环结束，计算出了想要的结果。代码为：

```cpp
int ch(int key, int num_buckets) {
    random.seed(key);
    int b = 0; // this will track ch(key, j + 1)
    for (int j = 1; j < num_buckets; j++) {
        if (random.next() < 1.0 / (j + 1)) b = j;
    }
    return b;
}
```
我们可以利用 j 增加时，ch(key, j + 1) 通常不变的特性，将算法转换为 log 时间开销。算法只需要计算那些 j ch(key, j + 1) != ch(key, j)。注意对于这些 j， ch(key, j + 1) = j。为了得到这个算法，我们将 ch(key, j) 看做一个随机变量，从而可以用随机变量的符号来分析

假设这个算法跟踪的是对于键 k 的桶序号的跳跃，假设 b 是最后一个 jump 的目标，表示 ch(k, b) != ch(k, b + 1)，并且 ch(k, b + 1) = b。现在，我们想要发现下一跳。最小的 j，使得 ch(k, j + 1) != ch(k, b + 1)，或者等效的，最大的 j 使得 ch(k, j) = ch(k, b + 1)。我们使用随机变量来分析 j。为了得到 j 的概率约束，注意到对于任意桶数量 i，我们有 j >= i，当且仅当一致性哈希值不随 i 变化，等效为当且仅当 ch(k, i) = ch(k, b + 1)，因此 j 的分布满足

$$P(j \ge i) = P(ch(k,i) = ch(k, b + 1))$$

幸运的是，这个分布很容易计算。因为 $P(ch(k,10)) = ch(k,11)$ 是 $10/11$，$P(ch(k,11)) = ch(k,12)$ 是 $11/12$，所以 $P(ch(k,10) = ch(k, 11))$ 是 $10/11 \times 11/12 = 10/11$，推广，如果 $n \ge m, P(ch(k,n) = ch(k,m)) = m / n$，因此对于任意 $i \gt b$,

$$P(j \ge i) = P(ch(k,i) = ch(k, b + 1)) = (b + 1)/i$$

现在，我们生成一个伪随机数，$r$， (依赖 k 和 j)，归一化到 0 到 1 之间。因为我们想要 $P(j \ge i) = (b + 1)/i$，我们假设 $P(j \ge i) iff r \ge (b + 1)/i$。解决 $i$ 的不等式 $i / P(j \ge i) iff i \ge (b + 1)/r$，因为 $i \ge j$，那么 j 等于 最大的 i ， 因此最大的 i 满足 $i \ge (b+1)/r$，因此通过 floor 方法，$j = floor((b + 1)/r)。$

使用这个公式，jump consistent hash 通过直到发现一个正数等于或者大于 num_buckets 来选择下一跳得到 ch(key, num_buckets)。然后我们知道上一跳就是结果

```cpp
int ch(int key, int num_buckets) {
    random.seed(key);
    int b = -1; // bucket number before the previous jump
    int j = 0; // bucket number before the current jump
    while (j < num_buckets) {
        b = j;
        r = random.next();
        j = floor((b + 1) / r);
    }
    return b;
}
```

同最上面的代码相比，已经很像了，下面需要实现随机部分，想要最快，还有良好的连续值。使用 64 位余数生成器，[此文章](https://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf)有详细讲解。当使用的 key 不满足 64 位时，需要使用 hash 函数变为 64 位。测试表明这个方法很好

值得注意的是，不像 Karger 的算法，如果 key 已经是个整数，不需要哈希一次，因为算法每次迭代，已经重新哈希过 key。这个 hash 不是很好（线性余数），但是因为重复执行，对于 key 的额外哈希也就不是特别必要。