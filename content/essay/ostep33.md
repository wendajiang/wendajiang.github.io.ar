---
title: Event-based Concurrency
description: ''
template: blog/page.html
date: 2023-03-09 11:00:08
updated: 2023-03-09 11:00:08
typora-copy-images-to: ../../static/pics/${filename}
taxonomies:
  tags: ["ostep"]
extra:
  mermaid: true
  usemathjax: true
  lead: 'the understanding of ostep chapter 33'

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

Specifically, a different style of concurrent programming is often used in both GUI-based applications as well as some types of internet servers. This style, known as event-based concurrency.

The problem that event-based concurrency addresses is two-fold.
- Manageing concurrency correctly in multi-threaded applications.
  - missing locks, deadlock, and so on.
- In a multi-threaded application, the developer has little or no control over what is scheduled at a given moment in time
  - The programmer simply creates threads and then hopes that the underlying OS schedules them in a reasonable manner across avaiable CPUs.

> how to build concurrent servers without threads

# An Event Loop
You simple wait for something(i.e., an "event") to occur; when it does, you check what type of event it is and do the small amout of work it requires.

Canonical event-based server pseudocode:
```cpp
while(1) {
  events = get_events();
  for (e in events) {
    process_event(e);
  }
}
```

But this leaves us with a bigger question: how exactly does an event-based server determine which events are taking place?
## Import API: select/poll/epoll

These interfaces enable a program to do is simple: check whether there is any incomming I/O that should be attended to.

> Blocking vs. Non-Blocking Interfaces
>
> Blocking(or synchronous) interface do all of their work before returning to the caller; non-blocking(or asynchronous) interfaces begin some work but return immediately, thus letting whatever work that needs to be doen get done in the background.
> Non-blocking interfces are essential in the event-based approach, as a call that blocks will halt all progress.


## Why simpler? No locks needed
With a single CPU and an event-based application, the problems found in concurrent programs are no longer present. Specifically, because only one event is being handled at a time, there is no need to acquire or release locks.

# A problem: blocking system calls
With an event-based approach ,however, there are no other threads to run: just the main event loop. And this implies that if an event handler issues a call that blocks, the *entire* server will do just that: block until the call completes. When the event loop blocks, the system sits idle, and thus is a huge potential waste of resources. We thus have a rule that must be obeyed in event-based systems: no blocking calls are allowed.

## A solution: asynchronous I/O
loop check the asynchronous issue completed.

## Another problem: state management
Thread-based code is more simpler. Packaging up some program state for the next event handler is not needed in thread-based programs, as the state the program need is on the stack of the thread.

For example, within a thread-based server:
```cpp
int rc = read(fd, buffer, size);
rc = write(sd, buffer, size);
```
Doing this kind of work is trivial. But in an event-based system, life is not so easy. You must manually manage the state of program.

The [solution](https://web.eecs.umich.edu/~mosharaf/Readings/Fibers-Coop-Tasks.pdf) sounds complicated, the idea is rahter simple: basically, record the needed information to finish processing this event in some data structure; when the event happens(i.e., when the disk I/O complates), look up the needed information and process the event.

# What is still difficult with events
- when systems moved from a single CPU to multiple CPUs, in order to utilize more than one CPU, the event server has to run multiple event handlers in parallel; when doing so, the usual syschronization problems(e.g., critical sections) arise.
- implicit blcking. Event though the server has been structured to avoid explicit blocking, page faults is hard to avoid.
- hard to manager over time
- asynchronous disk I/O is never quite integrates with asynchronous network I/O in as simple and uniform a manaer. you maybe need some combination of select() for networking and the AIO calls for disk I/O


# reference
- [staged event-based concurrency](http://www.sosp.org/2001/papers/welsh.pdf)