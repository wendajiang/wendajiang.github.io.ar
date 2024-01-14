---
title: flexible array member
description: ''
template: blog/page.html
date: 2024-01-12 14:59:41
updated: 2024-01-12 14:59:41
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["c"]
extra:
  mermaid: true
  usemathjax: true
  lead: ''

# mermaid example: 
# {% mermaid() %}
#     mermaid program
# {% end %}
---

# Introduction
Rencently, I raise one crash on coding about IPC (inter-process communication).

The scenario is we have one struct which need to place on shared memory, and the struct size tha I want it to be determined on the runtime. 

```cpp
struct FAM {
  size_t len;
  int mem[];
}

FAM* create(const char* key, int len) {
  int fd = open(key, O_RDWR | O_CREAT, 0666);
  int size = sizeof(FAM) + sizeof(int) * len;
  char *strs = (char*)malloc(size);
  int a = write(fd, strs, size);
  assert(a == size);
  free(strs);
  auto* fam = (FAM*)mmap(nullptr, size, PORT_READ | PORT_WRITE, MAP_SHARED, fd, 0);
  fam->len = len;
  close(fd);
  return fam;
}

FAM* connect(const char* key, int len) {
  int fd = open(key, O_RDWR, 0666);
  FAM* fam;
  while(fd < 0) {
    printf("waiting for memory created...");
    fd = open(key, O_RDWR, 0666);
    sleep(1);
  }
  fam = (FAM*)mmap(nullptr, sizeof(FAM) + sizeof(int) * len, PORT_READ | PORT_WRITE, MAP_SHARED, fd, 0);
  close(fd);
  assert(fam != MAP_FAILED);
  return fam;
}

int release(FAM* fam, int len) {
  munmap(fam, sizeof(FAM) + sizeof(int) * len);
  return 0;
}

```

# support
GCC and Clang extention of C99 and C++.

# reference
- [C++ proposal of FAM replacement](https://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p1039r0.html)
- [benefits and limitions of FAM](https://developers.redhat.com/articles/2022/09/29/benefits-limitations-flexible-array-members#limitations_of_sized_arrays)
- [gcc extention for FAM](https://stackoverflow.com/a/67894135/6885532)
- [arrays of length zero](https://gcc.gnu.org/onlinedocs/gcc/Zero-Length.html#Zero-Length)

