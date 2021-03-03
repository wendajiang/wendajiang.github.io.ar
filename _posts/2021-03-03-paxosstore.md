---
layout: post
date: 2021-03-03
categories: awesomepaper translate
title: "PaxosStore: High-availability Storage Made Practical in WeChat"
---

# 摘要

# 1. 介绍(Introduction)

# 2. 设计(Design)

## 2.1 架构(Overall Architecture)

## 2.2 共识层(Consensus Layer)

### 2.2.1 Paxos

### 2.2.2 PaxosLog

### 2.2.3 一致性读写(Consistent Scheme)

## 2.3 存储层(Storage Layer)

# 3. 容错性与可用性(Fault Tolerance and Availability)

## 3.1 容错(Fault-tolerant Scheme)

## 3.2 数据恢复(Data Recovery)

## 3.3 优化(Optimizations)

# 4. 实现(Implementation)

# 5. 评估(Evaluation)

## 5.1 实验步骤

## 5.2 延迟

## 5.3 容错性

## 5.4 错误恢复

## 5.5 Effectiveness of PaxosLog-Entry Batched Applying

## 5.6 PaxosStore在微信中的应用

### 5.6.1 服务即时消息(Serving Instant Messaging)

### 5.6.2 服务社交网络(Serving Social Networking)

# 6. Related Work

# 7. Conslusion

# 8. References
略