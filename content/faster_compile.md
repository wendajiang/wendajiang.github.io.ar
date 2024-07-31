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
   ```markdown
   build clang with cmake -DLLVM_BUILD_INSTRUMENTED=IR
   use this to train the compiler
     - we build some application
     - generate a profraw file
   merge all profraw files with llvm-prodata
   feed output to clang cmake with -DLLVM_PROFDATA_FILE=<path>
   combine with LTO for best results
   ```
1. Post link optimization
   - LLVM-BOLT
   - LLVM-Propeller


## Grab bag
1. -fvisibility=hidden
2. -fexperimental-new-pass-manager
3. distcc
4. LTO on your code
5. -ftime-trace

[bloaty](https://github.com/google/bloaty) a size profiler for binaries

## change the code

1. split one large file into many small file , so that is benefit from parallel compiling
2. PIMPL
3. fwd class instead of include header

# reference

- [youtube](https://www.youtube.com/watch?v=X4pyOtawqjg)
- [cmake3.16 introduce and something](https://onqtam.com/programming/2019-12-20-pch-unity-cmake-3-16/)