# WASI Random API

## `random_bytes`

```wai
/// Return a sequence of bytes from a CSPRNG.
///
/// This function never fails (though it may trap), and never blocks.
///
/// See the [Definitions] document for details on random data provided by
/// WASI Random.
///
/// [Definitions] ./Definitions.md
random-bytes: function(buffer: pull-buffer<u8>)
```
