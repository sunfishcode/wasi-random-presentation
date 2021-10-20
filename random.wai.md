# WASI Random API

## `random_bytes`

```wai
/// Return a sequence of bytes from a CSPRNG.
///
/// This function never fails (though it may trap), and never blocks.
///
/// See the [Concepts] document for details on random data provided by
/// WASI Random.
///
/// [Concepts] ./RandomDataConcepts.md
random-bytes: function(buffer: pull-buffer<u8>)
```
