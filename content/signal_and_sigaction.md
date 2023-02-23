---
template: blog/page.html
date: 2022-12-16 09:43:00
title: Signal and sigaction
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: []
extra:
  mermaid: true
  usemathjax: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---



First, the example

```cpp
#include <stdio.h>
#include <unistd.h>
#include <signal.h>
#include <errno.h>

static void sig_usr(int signum) {
  if (signum == SIGUSR1) {
    printf("SIGUSR1 received\n");
  } else if (signum == SIGUSR2) {
    printf("SIGUSR2 received\n");
  } else {
    printf("signal %d received\n", signum);
  }
}

int main(void) {
  char buf[512];
  int n;
  struct sigaction sa_usr;
  sa_usr.sa_flags = 0;
  sa_usr.sa_handler = sig_usr;
  sigaction(SIGUSR1, &sa_usr, NULL);
  sigaction(SIGUSR2, &sa_usr, NULL);
  printf("My PID is %d\n", getpid());
  while(1) {
    if (n = read(STDIN_FILENO, buf, 511) == -1) {
      if (errno == EINTR) {
        printf("read is interrupted by signal\n");
      }
    } else {
      buf[n] = '\0';
      printf("%d bytes read: %s\n", n, buf);
    }
  }
  return 0;
}
```

> scatter read
>
> gatter write

signal 通常使用一个 32 位无符号整数( *man 7 signal*), 使用单 bit 表示，因此可以表示 32 个信号。signal() sigaction() 功能类似，更改信号 handler。signal() 每次处理完需要重新设置。

但是 signal() 在已经进入自定义信号处理函数时，但是重新设置之前，可能系统又发生了信号，会丢失。sigaction 更可靠的原因是，在信号处理中，被捕捉的信号会被屏蔽，并且不用每次重新设置处理函数。

signal 中断当前进程进入内核原理在于 do_signal() 函数：

![signal.drawio](https://wendajiang.github.io/pics/signal_and_sigaction/signal.drawio.png)
