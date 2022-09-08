+++
template = "page.html"
date = "2022-09-08 10:54:17"
title = "IEEE Std 1800-2017 DPI"
[taxonomies]
tags = ["Verilog", "System Verilog", "Verification", "C"]

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

## 35.1 General

按照下面的方面来描述：

- Direct programming interface (DPI) tasks and functions
- DPI layers
- Importing and exporting functions
- Importing and exporting tasks
- Disabling DPI tasks and functions

## 35.2 Overview

本节强调 DPI 并提供了 SystemVerilog 接口层面的详细描述。C 层面在[附录中](#35-10-c-layer-fu-lu)

### 35.2.1 Tasks and functions

### 35.2.2 Data types

#### 35.2.2.1 Data representation



## 35.3 Two layers of DPI

### 35.3.1 DPI SystemVerilog layer

### 35.2 DPI foreign language layer



## 35.4 Global name space of imported and exported functions



## 35.5 Imported tasks and functions

### 35.5.1 Required properties of imported tasks and functions - semantic constraints

#### 35.5.1.1 Instant completion of imported functions

#### 35.5.1.2 input, output and inout arguments



#### 35.5.1.3 Special properties pure and context



#### 35.5.1.4 Memory management



#### 35.5.1.5 Reentrancy of imported tasks



#### 35.5.1.6 C++ exceptions

### 35.5.2 Pure functions



### 35.5.3 Context tasks and functions



### 35.5.4 Import declarations



### 35.5.5 Function result



### 3.5.6 Types of formal arguments

#### 3.5.6.1 Open arrays

## 35.6 Calling imported functions

### 35.6.1 Argument passing

#### 35.6.1.1 WYSIWYG principle

### 35.6.2 Value changes for output and inout arguments



## 35.7 Exported functions



## 35.8 Exported tasks



## 35.9 Disabling DPI tasks and functions



## 35.10 C Layer 附录