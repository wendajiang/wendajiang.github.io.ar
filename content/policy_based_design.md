---
title: strategy / policy based design
description: ''
template: blog/page.html
date: 2023-03-10 10:16:24
updated: 2023-03-10 10:16:24
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["design pattern"]
extra:
  mermaid: true
  usemathjax: true 
  lead: 'The Policy-Based Design'

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

# Strategy pattern and policy-based design
The classic Strategy pattern is a behavioral design pattern that enables the runtime selection of a specific algorithm for a particular behavior, usually from a predefined family of algorithms. The aim of the Strategy pattern is to allow the decision about which specific algorithm to use is defered until runtime.

This pattern is also known as the policy pattern; the name predateds its application to the generic programming in C++. The Plicy pattern applies the same approach to algorithm selection at compile time.

# When we need this pattern
The Strategy pattern should be considered whenever we design a system that does certain operations, but the exact implementation of these operations is uncertain, varied, or can change after the system is implemented. In other words, **when we know the answer to what the system must do, but not how.**