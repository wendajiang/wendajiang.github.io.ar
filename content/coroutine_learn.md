---
title: coroutine learn
description: ''
template: blog/page.html
date: 2023-06-12 12:29:41
updated: 2023-06-12 12:29:41
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["cpp", "coroutine"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2021/p2464r0.html
反对 asio 进入标准

# Coroutine Theory
## Coroutines are Functions are Coroutines
A coroutine is a generalisation of a function that allows the function to be suspended and then later resumed.

### "Normal" Functions
A normal function can be thought of as having two operations: **Call** and **Return**(note that I'm lumping "throwing an exception" here broadly under the Return operation)

The **Call** operation creates an activation frame, suspends execution of the calling function and transfers execution to the start of the function being called.

The **Return** operation passes the return-value to the caller, destroys the activation frame and then resumes execution of the caller just after the point at which it called the function.

> Activation Frames
> The block of memory that holds the current state of a particular invocation of a function. This state includes the values of any parameters that were passed to it and the values of any local variables.
>
> For "normal" functions, the activation frame also includes the return-address. You can think of these pieces of information together as describing the 'continuation' of the function-call. ie. they describe which invocation of which function should continue executing at which point when this function completes.
>
> With "normal" functions, all activation frames have strictly nested lifetimes. This strict nesting allows use of a highly efficient memory allocation data-structure for allocating and freeing the activation frames for each of the function calls. This data-structure is commonly refered to as "the stack".

#### The 'Call' operation
When a funtion calls another function, the caller must first prepare itself for suspension.


#### The 'Return' operation
When a function returns via `return`-stmt, the function first stores the return value (if any) where the caller can access it. This could either be in the caller's activation frame or the function's activation frame.

Then the function destorys the activation frame by:
- destroying any local variables in-scope at the return-point
- destroying any parameter objects
- freeing memory used by the activation-frame

And finally, it resumes execution of the caller by:
- restoring the activation frame of the caller by setting the stack register to point to the activation frame of the caller and restoring any registers that might have been clobbered by the function.
- jumping to the resume-point of the caller that was stored during the 'Call' operation.

## Coroutines
Coroutines generalise the operations of a function by separating out some of the steps performed in the Call and Return operations into three extra operations: **Suspend, Resume, Destroy**.

The **Suspend** operation suspends execution of the coroutine at the current point within the function and transfers execution back to the caller or resumer without destroying the activation frame. Any Objects in-scope at the point of suspension remain alive after the coroutine execution is suspended.

Note that, like the **Return** operation of a function, a coroutine can only be suspended from within the coroutine itself at well-defined suspend-points.(`co_await, co_yield`)

The **Resume** operation resumes execution of a suspended coroutine at the point at which is was suspended. This reactivates the coroutine's activation frame.

The **Destroy** operation destroys the activation frame without resuming execution of the coroutine. Any objects that were in-scope at the suspend point will be destroyed. Memory used to store the activation frame is freed.

### Coroutine activation frames
Since coroutines can be suspended without destroying the activation frame, we can no longer guarantee that activation frame frame lifetimes will be stricted nested. This means that activation frames cannot in general be allocated using a stack data-structure and so may need to be stored on the heap instead.

There are some provisions in the C++ Coroutines TS to allow the memory for the coroutine frame to be allocated from the activation frame of the caller if the compiler can prove that the lifetime of the coroutine is indeed strictly nested within the lifetime of the caller. This can avoid heap allocations in many case provided you have a sufficiently smart pointer.

With coroutins there are some parts of the activation frame that need to be preserved across coroutine suspension and there are some parts that only need to be kept around while the coroutine is executing. For example, the lifetime of a variable with scope that does not span any coroutine suspend-points can potentially be stored on the stack.

You can logically think of the activation frame of a corouine as being comprised of two parts: the **coroutine fame** and the **stack frame**.

The 'coroutine frame' holds part of the coroutine's activation frame that **persists** while the coroutine if suspended and the 'stack frame' part only exists while the coroutine is executing and is freed when the coroutine suspends and transfers execution back to the caller/resumer.



# reference
- https://devblogs.microsoft.com/oldnewthing/20210504-01/?p=105178
- https://lewissbaker.github.io/
