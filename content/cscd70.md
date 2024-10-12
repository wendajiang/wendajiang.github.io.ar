---
title: compiler optimization
description: ""
template: blog/page.html
date: 2024-06-01 20:37:02
updated: 2024-06-01 20:37:02
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags:
    - compiler
extra:
  mermaid: true
  usemathjax: true
  lead: ""
---

[course](https://www.cs.toronto.edu/~pekhimenko/courses/cscd70-w19/content.html)

# Introduction

## What do compiler do?
- Translate one language into another
- Improve("Optimization") the code (*Execution time = Operation count product Machine cycles per operation*)
  - Minimize the number of operations
  - Replace expensive operations with simpler ones
  - Minimize cache misses
  - Perform work in parallel


## Ingredients in a compiler optimization
- Formulate optimization problem
- Representation
- Analysis
- Code Transformation
- Expreimental evaluation

### Basic block
a sequence of 3-address statements

### Flow graphs
- Nodes: basic blocks
- Edges: Bi -> Bj, iff Bj can follow Bi immediately in some execution
## Sources of optimization
### Algorithm optimization
### Algebraic optimization
### local optimizations
within a basic block -- across instructions
like examples:

- local common subexpression elimination
- constant folding or elimination
- dead code elimination
### Global(intraprocedural) optimization
within a flow graph -- across basic blocks
- global versions of local optimizations
  - global common subexpression elimination
  - global constant folding or elimination
  - dead code elimination
- loop optimizations
  - reduce code to be executed in each iteration
  - code motion
  - induction variable elimination
- other control structures
  - code hoisting
### Interprocedural analysis
within a program -- across procedures (flow graphs)

# Dataflow


