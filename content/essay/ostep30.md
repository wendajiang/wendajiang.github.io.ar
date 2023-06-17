---
title: condition variable
description: 'ostep 30 chapter understand'
template: blog/page.html
date: 2023-06-14 11:08:39
updated: 2023-06-14 11:08:39
typora-copy-images-to: ../../static/pics/${filename}
taxonomies:
  tags: ["ostep"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'the understanding of ostep chapter 30'
---

In particular, there are many cases where a thread wishes to check whether a condition is true before continuing its execution. For example, a parent thread might wish to check whether a child thread has completed before continuing (this is often called a `join`); how should such a wait be implemented? Let's look at one example:

```cpp
void *child(void *arg) {
  printf("child\n");
  /// how to indicate we are done?
  return NULL;
}
int main() {
  printf("parent: begin\n");
  pthread_t c;
  pthread_create(&c, nullptr, child, nullptr);
  /// how to wait for child
  printf("parent: end\n");
  return 0;
}
```

The first approach is using a shared varaible, like follow:
```cpp
volatile int done = 0;
void *child(void *arg) {
  printf("child\n");
  done = 1;
  return NULL;
}
int main() {
  printf("parent: begin\n");
  pthread_t c;
  pthread_create(&c, nullptr, child, nullptr);
  while(!done); // spin
  printf("parent: end\n");
  return 0;
}
```
But it is inefficient as parent spins and wastes CPU time. What we would like here insead is some way to put the parent to sleep until the condition we are waiting for (e.g. the child is done executing) comes ture.

# Definition and Routines

To wait for a condition to become true, a thread can make use of what is known as a **conditino variable.** *A **condition variable** is an explicit queue that threads can put themselves on when some state of execution(i.e. some condition) is not as desired(by waiting on the condition); some other thread, when it changes said state, can then wake one(or more) of those waiting threads and thus allow them to continue(by signaling on the condition).*

The idea goes back to Dijkstra's use of `private semaphores`.

A condition variable has two operations associated with it: wait() and signal().
- the wait() call is executed when a thread wishes to put itself to sleep
- the signal() call is executed when a thread has changed something in the program and thus wants to wake a sleeping thread waiting on this condition.

Let's show the last paragraph example:
```cpp
int done = 0;
pthread_mutex_t m = PTHREAD_MUTEX_INITIALIZER;
pthread_cond_t cv = PTHREAD_COND_INITIALIZER;

void thr_exit() {
  pthread_mutex_lock(&m);
  done = 1;
  pthread_cond_signal(&cv);
  pthread_mutex_unlock(&m);
}

void *child(void *arg) {
  printf("child\n");
  thr_exit();
  return NULL;
}

void thr_join() {
  pthread_mutex_lock(&m);
  while(done == 0) 
    pthread_cond_wait(&cv, &m);
  pthread_mutex_unlock(&m);
}

int main() {
  printf("parent: begin\n");
  pthread_t p;
  pthread_create(&p, NULL, child, NULL);
  thr_join();
  printf("parent: end\n");
}
```


One thing you might notice about the wait() call is that it also takes a mutex as a parameter; it assumes that this mutex is blocked when wait() is called. **The responsibility of wait() is to release the lock and put the calling thread to sleep (atomically); when the thread wakes up, it must re-acquire the lock before returning to the caller.** This purpose is to prevent certain race conditions from occurring when a thread is trying to put itself to sleep.

There are two case that maybe happen:

{% mermaid() %}
sequenceDiagram
  parent->>child: thr_join
  parent->parent: wait() and release the lock
  child->>child: acquire the lock and set done
  parent->parent: return from wait() wich lock held and continue
{% end %}

{% mermaid() %}
sequenceDiagram
  child ->> child: acquire the lock and set done
  parent -> parent: wait() directly return and continue
{% end %}

One last noe: you might observe that the parent use a `while` loop instead of just an `if` stmt when deciding whether to wait on the condition. 

1. If we remove the state variable done, what is happened?
   Think about, first child calls `thr_exit` immediately; in this case, the child will signal, but there is no thread asleep on the condition. When the parent runs, it will simply call wait and be stuck; no thread will ever wake it.
2. If we remove the mutext ? The issue here is a subtle race condition. 

# Let's explain if stmt problem in the bounded-buffer problem(producer/consumer)

```cpp
int buffer;
int count = 0;

int loops;
cond_t cond;
mutext_t mutex;

void put(int value) {
  assert(count == 0);
  count = 1;
  buffer = value;
}
int get() {
  assert(count == 1);
  count = 0;
  return buffer;
}

void *producer(void *arg) {
  int i;
  for (i = 0; i < loops; i++) {
    pthread_mutex_lock(&mutex);              // p1
    if (count == 1)                          // p2
      pthread_cond_wait(&cond, &mutex);      // p3
    put(i);                                  // p4
    pthread_cond_signal(&cond);              // p5
    pthread_mutex_unlock(&mutex);            // p6
  }
}

void *consumer(void *arg) {
  int i;
  for(i = 0; i < loops; i++) {
    pthread_mutex_lock(&mutex);             // c1
    if (count == 0)                         // c2
      pthread_cond_wait(&cond, &mutex);     // c3
    int temp = get();                       // c4
    pthread_cond_signal(&cond);             // c5
    pthread_mutext_unlock(&mutex);          // c6
    printf("%d\n", temp);
  }
}
```

one-producer and one-consumer this works. However, if we have more than one of these threads, the solution has two critical problems. What are they?

If stmt cause race conditon:

| TC1 State | TC2 State | TP       | count | Comment                             |
| --------- | --------- | -------- | ----- | ----------------------------------- |
| c1  run   | ready     | ready    | 0     |                                     |
| c2  run   | ready     | ready    | 0     |                                     |
| c3  sleep | ready     | ready    | 0     |                                     |
| sleep     | ready     | p1 run   | 0     |                                     |
| sleep     | ready     | p2 run   | 0     |                                     |
| sleep     | ready     | p4 run   | 1     | buffer now fill TC1 awoken          |
| ready     | ready     | p5 run   | 1     |                                     |
| ready     | ready     | p6 run   | 1     |                                     |
| ready     | ready     | p1 run   | 1     |                                     |
| ready     | ready     | p2 run   | 1     |                                     |
| ready     | ready     | P3 sleep | 1     | buffer rull; sleep tc2 sneaks in... |
| ready     | c1 run    | sleep    | 1     |                                     |
| ready     | c2 run    | sleep    | 1     |                                     |
| ready     | c4 run    | sleep    | 0     | ... and grabs data TP awoken        |
| ready     | c5 run    | ready    | 0     |                                     |
| ready     | c6 run    | ready    | 0     |                                     |
| c4 run    | ready     | ready    | 0     | Oh ho! No data                      |

Changing p2 and c2 if stmt to while will fix the issue. But there is another problem, as there is only one condition variable.

| TC1 State  | TC2 State | TP        | count | Comment                    |
| ---------- | --------- | --------- | ----- | -------------------------- |
| c1  run    | ready     | ready     | 0     |                            |
| c2  run    | ready     | ready     | 0     |                            |
| c3  sleep  | ready     | ready     | 0     |                            |
| sleep      | c1 run    | ready     | 0     |                            |
| sleep      | c2 run    | ready     | 0     |                            |
| sleep      | c3 sleep  | ready     | 0     | nothing to get             |
| sleep      | sleep     | p1 run    | 0     |                            |
| sleep      | sleep     | p2 run    | 0     |                            |
| sleep      | sleep     | p4 run    | 1     | buffer now full TC1 awoken |
| ready      | sleep     | p5 run    | 1     |                            |
| ready      | sleep     | P6 run    | 1     |                            |
| ready      | sleep     | p1 run    | 1     |                            |
| ready      | sleep     | p2 run    | 1     |                            |
| ready      | sleep     | P3 sleep  | 1     | must sleep (full)          |
| c2 run     | sleep     | sleep     | 1     | recheck condition          |
| c4 run     | sleep     | sleep     | 0     | TC1 grab data              |
| **c5 run** | **ready** | **sleep** | **0** | **Oops! Woek TC2**         |
| c6 run     | ready     | sleep     | 0     |                            |
| c1 run     | ready     | sleep     | 0     |                            |
| c2 run     | ready     | sleep     | 0     |                            |
| c3 sleep   | ready     | sleep     | 0     |                            |
| sleep      | c2 run    | sleep     | 0     |                            |
| sleep      | c3 sleep  | sleep     | 0     |                            |

The problem happen at the blackbody line. TC1 awake TC2, as there is only one condition variable.



# Summary about using

- Using mutex with condition variable
- When checking for a condition in a multi-threaded program, using while loop is always correct.

# Next problem, which one thread should be awoke?

```cpp
int bytes_left = MAX_HEAP_SIZE;

cond_t c;
mutex m;
void allocate(int size) {
  pthread_mutex_lock(&m);
  while(bytes_left < size) 
    pthread_cond_wait(&c, &m);
  void *ptr = ...;
  bytes_left -= size;
  pthread_mutex_unlock(&m);
  return ptr;
}

void free(void *ptr, int size) {
  pthread_mutex_lock(&m);
  bytes_left += size;
  pthread_cond_signal(&c); // whom to signal???
  pthread_mutex_unlock(&m);
}

// threada allocate(100);
// threadb allocate(10);
// threadc free(50);
```

As comments, threadc free 50 bytes memory, if it awake threadb, the result is correct, but if it awake threada, it's error. The sollution is introduce the `pthread_cond_broadcase` semantic. 

In general, if you find that your program only works when you change your signals to broadcasts (but you don't think it should need to), you probably have a buf, fix it!



# Implementation uing mutex

```cpp
struct condition {
  proc next;
  proc prev;
  mutex mx; // protect operations on this queue. This semaphore should be spin-lock since it will only be held for very short periods of time
};

void wait(condition *c, mutex *mx) {
  mutex_acquire(&c->mx);
  enqueue(&c->next, &c->prev, thr_self());
  mutex_release(&c->mx);
  
  // the suspend and release_mutex() operation should be atomic
  release_mutex(mx);
  thr_suspend(self);
  
  mutex_acquire(mx); // woke up -- our turn, get resource lock
  return;
}

void signal(condition *c) {
  thread_id tid;
  mutex_acquire(&c->mx);
  tid = dequeue(&c->next, &c->prev);
  mutex_release(&c->mx);
  
  if (tid > 0) {
    thr_continue(tid);
  }
  return;
}

void boradcast(condition *c) {
  thread_id tid;
  mutex_acquire(&c->mx);
  while(&c->next) {
    tid = dequeue(&c->next, &c->prev);
    thr_continue(tid);
  }
  mutex_release(&c->mx);
}
```



# reference

- https://cseweb.ucsd.edu/classes/sp17/cse120-a/applications/ln/lecture7.html