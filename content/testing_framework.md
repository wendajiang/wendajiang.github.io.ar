---
template: blog/page.html
date: 2023-02-09 16:22:43
title: Testing Framework
typora-copy-images-to: ../static/pics/${filename}
taxonomies:
  tags: ["unit test", "testing framework", "C++"]
extra:
  mermaid: true
  usemathjax: true
  toc: true

# mermaid example: 
# <div class="mermaid">
#     mermaid program
# </div>
---

# From Catch2

When I need to unit-test for some software, I find the Catch2 library, v2 version is only header-library. I read the document: [Why do we need yet another C++ test framework?](https://github.com/catchorg/Catch2/blob/devel/docs/why-catch.md#top), for c++ there are so many established frameworks, including to(but not limited to), Google Test, Boost.Test, CppUnit and so on. Summary as follow:

- Easy to use. Just download some file, add them into your project. No external dependencies.
- BDD-style, Given-When-Then sections as well as traditional unit test cases
- Write test cases as, self-registering, functions(or methods, if you prefer)
- Tests are named using free-form strings - no more couching names in legal identifiers
- ...

For me simple and practical using, the first tip is important.

But, yes, there is always but, the v3 version brings a bunch of significant changes, the big one being that Catch2 is no longer a single-header library, and behaves as a normal library, with multiple headers and separately compiled implementation.

It's ok, and what I need particular requirement is I want to write test with productive code. And personly I like the single-header library, it's easiest to use.

And I find the doctest repo.

# Doctest

Tests can be a form of documentation and should be able to reside near the production code which they test. 

- This makes the barrier for writing tests much lower - you don't have to: 1) make a separate source file 2) include a bunch of stuff in it 3) add it to the build system and 4) add it to source control - You can just write the tests for a class or a piece of functionality at the bottom of its source file - or even header file!
- Tests in the production code can be thought of as documentation/up-to-date comments - showcasing the APIs
- Testing internals that are not exposed through the public API and headers is no longer a mind-bending exercise
- Test-driven development in C++ has never been easier!

And there is the DOCTEST_CONFIG_DISABLE config to remove all tests from the library or binary.


## Assertion Micro

- REQUIRE : will immediately quit the test case if the assert fails and will mark the test case as failed
- CHECK : will mark the test case as failed if the assert fails but will continue with the test case
- WARN : will only print a message if the assert fails but will not mark the test case as failed
- \<LEVEL\>_THOWS : except a throw

## Test cases

- TEST_CASE(test name)
- SUBCASE 

### BDD-style test cases
- SCENARIO(scenario name) -> map to TEST_CASE, and the test case name will be prefixed by "Scenario: "
- GIVEN / WHEN/ THEN - map to SUBCASE, the prefixed would be "given: / when: / then: "

### Test suites
Test cases can be grouped into test suites.

- TEST_SUITE(\<suite name\>) {}
- TEST_SUITE_BEGIN(\<suite name\>)
- TEST_SUITE_END()

### Decorators

Test cases can be decorated with additional attributes like this:
```cpp
TEST_CASE("name"
      * doctest::description("shouldn't take more than 500ms")
      * doctest::timeout(0.5))
```

Multiple decorators can be used at the same time.

- skip(bool = true)
- no_breaks(bool = true)
- no_output(bool = true)
- may_fail(bool = true) doesn't fail the test case if any given assertion fails(but still report it)
- should_fail(bool = true)
- expected_failures(int)
- timeout(double) fails the test case if its execution exceeds this limit (in seconds) - but doesn't terminate it - that would require subprocess support
- test_suite("name")
- description("text")