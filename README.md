# WASI Random

A proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API.

### Current Phase

Phase 2

### Champions

- Dan Gohman

### Phase 4 Advancement Criteria

 - Two independent implementations.
 - At least one host implementation on each of:
    - Linux
    - macOS
    - Windows

## Table of Contents

- [Introduction](#introduction)
- [Goals](#goals)
- [Non-goals](#non-goals)
- [API walk-through](#api-walk-through)
- [Detailed design discussion](#detailed-design-discussion)
  - [Should WASI random be async?](#should-wasi-random-be-async)
  - [Should WASI random specify a number of bits of security?](#should-wasi-random-specify-a-number-of-bits-of-security)
- [Considered alternatives](#considered-alternatives)
  - [[Alternative 1]](#alternative-1)
  - [[Alternative 2]](#alternative-2)
- [Stakeholder Interest & Feedback](#stakeholder-interest--feedback)
- [References & acknowledgements](#references--acknowledgements)

### Introduction

WASI random provides APIs for obtaining random data, from a [CSPRNG].

[CSPRNG]: https://en.wikipedia.org/wiki/Cryptographically-secure_pseudorandom_number_generator

### Goals

Goals include providing high-quality random data without burdening
applications with the need to find ways to recover in case random data
is either temporarily or permenantly unavailable.

### Non-goals

WASI random is not aiming to be a low-level TRNG or DRBG API. It does not
publish entropy estimates or provide low-level access to entropy sources. It
does not provide ways for users to manually reseed the CSPRNG.

And, WASI random is not aiming to be a high-level general-purpose randomness
API. It just returns random bytes, and expects other APIs will provide
more convenient interfaces on top of it.

### API walk-through

There's just one function, `random_bytes`, and it takes a `pull-buffer` to
write bytes into.

### Detailed design discussion

#### Should WASI random be async?

During "early boot", some host platforms start in a state where they do not
have sufficient entropy to seed their CSPRNGs. One way to handle this is to
make the `random_bytes` function be async. Alternatively, it could return a
specialized error value indicating that randomness is temporarily unavailable.

However, this puts the burden on applications to have other meaningful work
to do while waiting for randomness to become available, work which may be
expected to generate network traffic or other I/O which would generate
entropy. It's not clear that applications in general are prepared for this
burden.

And, this would put the burden on all applications that need random data.
The WASI random design puts the burden on platform designers to ensure that,
if they need programs that need random data to run during early boot, their
platforms should be designed to support that.

#### Should WASI random specify a number of bits of security?

Best practices suggest that implementations should provide at least 196 bits
of security. However, many host platforms' CSPRNG APIs do not currently
document their bits of security, and it doesn't seem desirable to require
wasm engines to run their own CSPRNG on a platform which alreay has one, so
for now, the API does not specify a specific number.

### Stakeholder Interest & Feedback

There are currently several major WASI implementations of the WASI random API.
It provides the basis for implementing `getentropy` in wasi-libc, and it's
widely used.

### References & acknowledgements

Many thanks for valuable feedback and advice from:

 - Zach Lym
 - Luke Wagner
 - Linux Weekly News' many articles about Linux random APIs including [this one].

[this one]: https://lwn.net/Articles/808575/
