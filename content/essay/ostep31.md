---
title: semaphore
description: ''
template: blog/page.html
date: 2023-06-15 09:28:10
updated: 2023-06-15 09:28:10
typora-copy-images-to: ../../static/pics/${filename}
taxonomies:
  tags: ["ostep"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'the understand of ostep chapter 31'

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

Indeed, Dijkstra and colleagues invented the semaphore as a single primitive for all things related to synchronization; as you will see, one can use semaphores as both locks and condition variables.

# Definition
A semaphore is an object with an integer value that we can manipulate with two routines; in the POSIX standard, these routines are sem_wait() P and sem_post() V. 

```cpp
int sem_wait(semt_t *s) {
  decrement the value of semephore s by one
  wait if value of semaphore s is negative
}

int sem_post(sem_t *s) {
  increment the value of semphore s by one
  if there are one or more threads waiting, wake one
}
```

## Implementation

```cpp
void P(csem) {
  while(1) {
    mutex_acquire(csem.mx);
    if (csem.value <= 0) {
      mutex_release(csem.mx);
      continue;
    } else {
      csem.value -= 1;
      mutex_release(csem.mx);
      break;
    }
  }
}

void V(csem) {
  mutex_acquire(csem.mx);
  csem.value += 1;
  mutex_release(csem.mx);
}
```

Re-think the P implementation. If the critical section is large, we could spend a great deal of time spinning.

# Binary Semaphores (Locks)

@todo

# reference

- [cse170 semaphores and cv](https://cseweb.ucsd.edu/classes/sp17/cse120-a/applications/ln/lecture7.html)
- [信号量与mutex区别](https://www.zhihu.com/question/47704079/answer/136200849)
- [Goodbyes semaphores](https://lwn.net/Articles/166195/)
- [How are mutexes and semaphores different with respect to their implementation in a Linux kernel?](https://www.quora.com/How-are-mutexes-and-semaphores-different-with-respect-to-their-implementation-in-a-Linux-kernel) PS - Ealrier mutex was implemented using binary semaphore but that is changed now. Please see below for more reference.
  - http://lwn.net/Articles/163807/
  - https://www.kernel.org/doc/Documentation/locking/mutex-design.txt
  - https://lkml.org/lkml/2005/12/22/154
