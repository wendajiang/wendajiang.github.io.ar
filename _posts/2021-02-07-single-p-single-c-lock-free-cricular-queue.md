# Part I

## Introduction

## Background: Circular FIFO Queue
## Background: Article Rationale
## Disclaimer
## Memory model: sequential or relaxed/acquire/release?
## Using the code
## Code Explained: Circularfifo‹Type, Size›
## How it works
### Empty
### Producer adds items
### Consumer retrieves item
### Full
## Thread Safe Use and Implementation
# Part II

## Making It Work

在元素被读取和写入（pop或者push）后马上更新头或者尾index是必要的。因此，任何对于头尾index的更新必须保证正确，所以这需要是原子操作并且不能被重排序。

## Atomic operations and the different Memory Models



## Sequential Consistent
### Rules for Sequential Consistent Memory Model and Operations
### Sequential Consistent Atmic Code
### Caveats with the Sequential Memory Model
### Simplified Atomic Sequential operations
## Refined Memory Order
## Relaxed-Memory Model
### Rules for the Relaxed Memory Model and Operations
## Acquire-Release Memory model
### Rules for the Acquire-Release Memory Model and Operations
## Code Explained: Acquire-Release/Relaxed Circularfifo
## x86-x64 Comparison: Sequential vs Refined Memory Order
## Conclusion
## History
## References