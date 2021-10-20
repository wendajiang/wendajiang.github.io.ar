+++
template = "page.html"
date = "2021-10-18 21:14:37"
title = "【翻译】零拷贝的意义"
[taxonomies]
tags = ["translate", "linux", "kernel"]

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

[原文](https://www.linuxjournal.com/article/6345)

## 什么是零拷贝

为了更好地理解解决了什么问题，首先要理解问题本身。我们来看下下面的代码发生了什么

```cpp
read(file, tmp_buf, len);
write(socket, tmp_buf, len);
```

看起来简单，你可能觉得两次系统调用也不是多大的开销。实际上，远比你想象中的大。这两次系统调用之下，**数据拷贝了四次**，并且还有内核态和用户态之间的上下文切换（实际上这个进程更复杂，但是先保持这种程度的简化）。为了更好地了解这个进程的调用，看下图

![image-20211019174214718](https://wendajiang.github.io/pics/zero_copy/image-20211019174214718.png)

第一步：read 系统调用发生了用户态到内核态的切换。第一次拷贝由DMA执行，从 disk 读取文件内容到内核空间的 buffer

第二步：数据从内核空间的 buffer 拷贝到用户空间的 buffer，然后 read 系统调用 return。return 又发生了内核态到用户态的切换。现在数据存储在用户态的 buffer 中

第三步：write 系统调用发生了用户态到内核态的切换。第三次拷贝从用户空间 buffer 到内核空间 buffer，这次内核空间的buffer 与 socket 关联

第四步：write 系统调用 return，触发了第四次上下文切换。单独异步的拷贝由DMA控制将数据从内核空间的 buffer 拷贝到协议栈中。你可能会问“什么叫独立异步？数据不是在return的时候就传输完成了吗？”事实上，return 并不保证传输完成；甚至不保证传输开始。可能在这次调用之前队列中有很多包，除非硬件驱动实现了优先队列，并且本次数据在第一优先级的队列中

正如你所看到的，很多数据拷贝实际本不必要发生。有些拷贝可以规避掉来减少开销提升程序性能。对一个驱动开发者，硬件实际有很多增强功能。有些硬件支持 bypass 主存，直接从将数据传输到其他设备。这个特性可以减少系统存储中的拷贝，但并不是所有硬件都支持。还有一个问题就是从disk得到数据需要为了网络重新封装，这带来了复杂性。为了减少开销，我们可以从减少内核空间和用户空间之间的拷贝开始。

### mmap

一个方法就是避免使用read，而使用 mmap

```cpp
tmp_buf = mmap(file, len);
write(socket, tmp_buf, len);
```

为了更好地理解这个过程，看下图

![image-20211019192834679](https://wendajiang.github.io/pics/zero_copy/image-20211019192834679.png)

第一步：mmap 系统调用使得文件内容通过 DMA 从disk 拷贝到内核空间的buffer，这个buffer可以直接在用户空间访问，不需要拷贝到用户空间

第二步：write 系统调用使得内核将内核空间buffer中的数据拷贝到 socket 关联的内核buffer中

第三步：第三次拷贝是 DMA 将socket buffer中的数据拷贝到协议栈中

通过使用 mmap，我们减少了一次使用 CPU 的拷贝。当大量数据传输时这比较有用，但并不是没有代价，有些隐藏的问题其实。当另一个进程也mmap到了同一个文件，然后对其进行了写入操作。上面代码的 write 就会被 SIGBUS 中断。这个信号默认的行为是杀死进程并coredump -- 这个操作对于网络服务器恨不能接受。有两种方法解决这个问题

第一个是定义一下 SIGBUS 的信号处理程序，直接return。这样 write 系统调用就会返回中断前写入的字节数，然后 errno 被设置为成功。这是个垃圾方案，无视问题，因为 SIGBUS 实际表示进程发生了严重错误，不应该无视

第二个是使用内核的文件租约（也叫“乐观锁”）。这是正确的解决方案。你可以从内核请求 read/write lease，当另一个进程视图对同一个文件进行截断时，内核会发送 RT_SINGAL_LEASE信号给有租约的进程，告诉你内核正在打断你的租约。你的写请求就会在程序访问无效地址之前被中断而不会导致 SIGBUS。write的返回值就是已经写入的字节数，errno被设置为成功，代码如下

```cpp
if (fcntl(fd, F_SETSIG, RT_SIGNAL_LEASE) == -1) {
  perror("kernel lease set signal");
  return -1;
}
/*l_type can be F_RDLCK F_WRLCK*/
if (fcntl(fd, F_SETLEASE, l_type)) {
  peror("kernel lease set type");
  return -1;
}
```

你应该在 mmap file之前获取租约，然后完成写入后主动放弃租约。通过调用 fcntl(F_SETLEASE, F_UNLCK) 

### Sendfile

在内核2.1版本，sendfile 系统调用简化了将数据从文件传输到socket的方法，不仅减少了数据拷贝，还减少了内核与用户空间的上下文切换

```cpp
sendfile(socket, file, len);
```

为了更好地理解底层过程，如下图

![image-20211019194434427](https://wendajiang.github.io/pics/zero_copy/image-20211019194434427.png)

第一步：sendfile 系统调用使得文件内容通过 DMA 拷贝到内核buffer，然后数据被内核拷贝到 socket buffer

第二步：第三次拷贝 DMA 控制从 socket buffer 拷贝到协议栈

你可能想知道如果另一个进程截断了这个文件，sendfile会发生什么。如果没有注册信号处理函数，sendfile 调用会返回已经传输的字节，errno被设置为成功

如果 sendfile 之前获取了文件租约，行为相同。同时sendfile return前会有 RT_SIGNAL_LEASE 信号

### sendfile + gather(hardware)

如此说来，我们已经避免了内核进行多次拷贝，但是还有一次拷贝，可以也避免吗？当然，但是需要硬件的支持。为了消除内核的所有拷贝，需要网卡支持 gather 操作。内核2.4版本引入了socket对这种操作的支持，这种方法不需要内核态切换，也不需要处理器进行数据拷贝。同时用户代码不变

```cpp
sendfile(socket, file, len);
```

底层流程如下图

![image-20211019200216564](https://wendajiang.github.io/pics/zero_copy/image-20211019200216564.png)

第一步：sendfile 系统调用可以将文件内容通过 DMA 拷贝到内核 buffer

第二步：没有数据被拷贝到 socket buffer。socket 文件描述符只有数据的长度信息。DMA 直接将数据从kernel buffer 拷贝到协议栈

因为数据从 disk 到主存的拷贝和从主存到总线的拷贝，有些人觉得这也不是零拷贝。这只是从操作系统角度的零拷贝，因为在操作系统中只有一个buffer。当使用零拷贝时，性能提升还来自，比如更少的内核用户态切换，更少的 CPU cache 污染以及不需要 CPU 校验

现在我们知道了什么是零拷贝，练习些代码吧。你可以下载[完整代码]([www.xalien.org/articles/source/sfl-src.tgz](http://www.xalien.org/articles/source/sfl-src.tgz))。

[论文](https://www.uidaho.edu/-/media/UIdaho-Responsive/Files/engr/research/csds/publications/2012/Performance-Review-of-Zero-Copy-Techniques-2012.pdf?la=en&hash=B5F37435875AAD15C55C7DFC1FDA53DBF242C0E3)

### splice

sendfile 只适用于将数据从文件拷贝到socket（依赖于具体实现，linux系统可以从文件到文件），而且使用 gather 需要硬件的支持。Linux 在2.6.17版本中实现了 splice 系统调用

