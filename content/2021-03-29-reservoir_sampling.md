+++
template = "blog/page.html"
date = "2021-03-29 12:50:38"
title = "蓄水池采样"

[taxonomies]
tags = ["math", "alg"]
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

https://en.wikipedia.org/wiki/Reservoir_sampling

蓄水池采样是随机算法中选择一次随机采样的算法，从未知数据规模中挑选一次采样的算法。算法不知道数据的规模 n，并且算法不存储过去的数值，只能在当前的数中作出选择--选还是不选。

### 动机

假设我们看到一个序列，一次看到序列的一项。我们想要将10项存储在内存中，然后从10项中随机挑选出1项。如果我们知道序列的大小n，即一共有多少项，这很容易，问题是我们不知道n的大小。

### 简单算法

简单但是比较慢的算法，被称为 *Aligorithm R*

```python
ReservoirSample(S[1...n], R[1...k])
	// fill the reservoir array
	for i := 1 to k
    R[i] := S[i]
  
  for i := k + 1 to n
    j := randomInteger(1, i)
    if j <= k
    	R[j] := S[i]
```

该算法建立了 size k 的蓄水池，开始保存输入的 k项。随着输入的迭代，按照条件更新蓄水池。对于第$i^{th}$元素来说，被选上的概率为 $\frac{k}{i}$。对于蓄水池的项，被代替的概率为 $ \frac{1}{k} \times \frac{k}{i} = \frac{1}{i} $。当算法结束时，每个输入项被选到蓄水池的概率相等$\frac{k}{n}$ （$\frac{k}{i} \times (1 - \frac{1}{i+1}) \times (1 - \frac{1}{i + 2}) \times ... \times (1 - \frac{1}{n}) = \frac{k}{n}$），算法时间复杂度为 O(n)

### 一个优化算法

*Algorithm L* 优化了这个算法，通过计算下一个item进入蓄水池之前已经丢弃了多少项。关键点在于这个值满足**几何分布**，因此可以在常数时间计算

```python
/* S has items to sample, R will contain the result */
ReservoirSample(S[1..n], R[1..k])
  // fill the reservoir array
  for i = 1 to k
      R[i] := S[i]

  /* random() generates a uniform (0,1) random number */
  W := exp(log(random())/k)

  while i <= n
      i := i + floor(log(random())/log(1-W)) + 1
      if i <= n
          /* replace a random item of the reservoir with item i */
          R[randomInteger(1,k)] := S[i]  // random index between 1 and k, inclusive
          W := W * exp(log(random())/k)
```

这个算法在不会被选中的项上不花时间，算法时间复杂度为 O(k(1 + log(n/k)))

### 随机排序(Reservoir Sampling with Radnom Sort)

如果将输入的每项绑定其对应的随机数，然后在选中的k项中看随机数大小取用，比如选随机数最大的，可以通过优先队列维护k的蓄水池

```python
/*
  S is a stream of items to sample
  S.Current returns current item in stream
  S.Next advances stream to next position
  min-priority-queue supports:
    Count -> number of items in priority queue
    Minimum -> returns minimum key value of all items
    Extract-Min() -> Remove the item with minimum key
    Insert(key, Item) -> Adds item with specified key
 */
ReservoirSample(S[1..?])
  H := new min-priority-queue
  while S has data
    r := random()   // uniformly random between 0 and 1, exclusive
    if H.Count < k
      H.Insert(r, S.Current)
    else
      // keep k items with largest associated keys
      if r > H.Minimum
        H.Extract-Min()
        H.Insert(r, S.Current)
    S.Next
  return items in H
```

期望复杂度为 O(n + klogklog(n/k)) ，这个算法意义在于容易扩展到有权重的item

### 权重蓄水池

Some applications require items' sampling probabilities to be according to weights associated with each item. For example, it might be required to sample queries in a search engine with weight as number of times they were performed so that the sample can be analyzed for overall impact on user experience. Let the weight of item *i* be $$, and the sum of all weights be *W*. There are two ways to interpret weights assigned to each item in the set:[[4\]](https://en.wikipedia.org/wiki/Reservoir_sampling#cite_note-efraimidis-4)

1. In each round, the probability of every *unselected* item to be selected in that round is proportional to its weight relative to the weights of all unselected items. If *X* is the current sample, then the probability of an item $i \notin X$ to be selected in the current round is $\frac{w_i}{W - \sum_{j \in X}w_j}$
2. The probability of each item to be included in the random sample is proportional to its relative weight, i.e. $\frac{w_i}{W}$. Note that this interpretation might not be achievable in some cases, e.g., $k = n$.

#### Algorithm A-Res

```python
/*
  S is a stream of items to sample
  S.Current returns current item in stream
  S.Weight  returns weight of current item in stream
  S.Next advances stream to next position
  The power operator is represented by ^
  min-priority-queue supports:
    Count -> number of items in priority queue
    Minimum() -> returns minimum key value of all items
    Extract-Min() -> Remove the item with minimum key
    Insert(key, Item) -> Adds item with specified key
 */
ReservoirSample(S[1..?])
  H := new min-priority-queue
  while S has data
    r := random() ^ (1/S.Weight)   // random() produces a uniformly random number in (0,1)
    if H.Count < k
      H.Insert(r, S.Current)
    else
      // keep k items with largest associated keys
      if r > H.Minimum
        H.Extract-Min()
        H.Insert(r, S.Current)
    S.Next
  return items in H
```

此算法除了item key的生成与 Reservoir Sampling with Radnom Sort 是一样的。算法等于为每个item赋值一个key $r^{\frac{1}{w_i}}$，$r$是随机数，然后选择k个item最大key的那个。数学上是相等的计算key $ln(r) / w_i$

#### Algorithms A-ExpJ

```python
/*
  S is a stream of items to sample
  S.Current returns current item in stream
  S.Weight  returns weight of current item in stream
  S.Next advances stream to next position
  The power operator is represented by ^
  min-priority-queue supports:
    Count -> number of items in the priority queue
    Minimum -> minimum key of any item in the priority queue
    Extract-Min() -> Remove the item with minimum key
    Insert(Key, Item) -> Adds item with specified key
 */
ReservoirSampleWithJumps(S[1..?])
  H := new min-priority-queue
  while S has data and H.Count < k
    r := random() ^ (1/S.Weight)   // random() produces a uniformly random number in (0,1)
    H.Insert(r, S.Current)
    S.Next
  X := log(random()) / log(H.Minimum) // this is the amount of weight that needs to be jumped over
  while S has data
    X := X - S.Weight
    if X <= 0
      t := H.Minimum ^ S.Weight
      r := random(t, 1) ^ (1/S.Weight) // random(x, y) produces a uniformly random number in (x, y)
    
      H.Extract-Min()
      H.Insert(r, S.Current)

      X := log(random()) / log(H.Minimum)
    S.Next
  return items in H
```

这个版本比A-Res效率高些，但是基本原理相同，不像A-Res版本无论这个item是不是被选中到蓄水池中，为每个item计算key，A-Expl 只计算被选中的 item。

#### Algorithm A-Chao

```python
/*
  S has items to sample, R will contain the result
  S[i].Weight contains weight for each item
 */
WeightedReservoir-Chao(S[1..n], R[1..k])
  WSum := 0
  // fill the reservoir array
  for i := 1 to k
      R[i] := S[i]
      WSum := WSum + S[i].Weight
  for i := k+1 to n
    WSum := WSum + S[i].Weight
    p := S[i].Weight / WSum // probability for this item
    j := random();          // uniformly random between 0 and 1
    if j <= p               // select item according to probability
        R[randomInteger(1,k)] := S[i]  //uniform selection in reservoir for replacement
```

对于每个item，都会计算其权重，并用于决定item是不是被加入蓄水池。如果item被选中，蓄水池中的一个item要被替换掉。这里的窍门是如果所有蓄水池的item概率已经与权重成比例，那归一化之后比例不变

### Relation to Fisher-Yates shuffle

Fihser-Yates shuffle

```python
Shuffle(S[1...n], R[1...n])
	R[1] := S[1]
  for i from 2 to n do
  	j := randomInteger(1, i)
    R[i] := R[j]
    R[j] := S[i]
```

但是当只需要取k张牌时，将洗牌的全部结果保存浪费空间，所以可以节省空间

```python
ReservoirSample(S[1..n], R[1..k])
  R[1] := S[1]
  for i from 2 to k do
      j := randomInteger(1, i)  // inclusive range
      R[i] := R[j]
      R[j] := S[i]
  for i from k + 1 to n do
      j := randomInteger(1, i)  // inclusive range
      if (j <= k)
          R[j] := S[i]
```

