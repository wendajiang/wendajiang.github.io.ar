---
title: faster compile
description: 'how to speed up compile time '
template: blog/page.html
date: 2024-07-31 11:50:08
updated: 2024-07-31 11:50:08
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["compiler"]
extra:
  mermaid: false
  usemathjax: true
  lead: ''
---

# profile
- clang -ftime-trace would output one json file, that can generate flame graph
- gcc -ftime-report would output the phase time and percent

# speed up
## do not change the code
1. faster linker 
    lld faster than gnu bsd linker
2. PCH(pre-compiled header)
    `target_precompile_headers(<target> PUBLIC <headers>)`
3. cache compile result
   - ccache
   - sccache(cache shareing)
4. unity builds
    `cmake -DCMAKE_UNITY_BUILD=ON` when cmake 3.16+
5. LTO (link time optimization)
    `cmake -DLLVM_ENABLE_LTO=Thin` for clang
6. PGO(profile guided optimization)
   1. build clang with cmake -DLLVM_BUILD_INSTRUMENTED=IR
   2. use this to train the compiler
      1. we build some application
      2. generate a profraw file
   3. merge all profraw files with llvm-prodata
   4. feed output to clang cmake with -DLLVM_PROFDATA_FILE=<path>
   5. combine with LTO for best results
7. Post link optimization
   LLVM-BOLT
   LLVM-Propeller

## Grab bag
1. -fvisibility=hidden
2. -fexperimental-new-pass-manager
3. distcc
4. LTO on your code
5. -ftime-trace

## change the code
1. split one large file into many small file , so that is benefit from parallel compiling
2. PIMPL
3. fwd class instead of include header

# reference
- [youtube](https://www.youtube.com/watch?v=X4pyOtawqjg)