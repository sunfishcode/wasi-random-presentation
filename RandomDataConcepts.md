# WASI Random

WASI RANDOM is a CSPRNG-style API, and not a TRNG or DRBG API. WASI might
add new APIs in the future to address other use cases, but within this API,
the focus is on providing applications with simple and infallible interfaces
to effectively unlimited streams of quality random data.

## WASI Random implies a CSPRNG

Implementations must be based on a [CSPRNG] algorithm, rather than return raw
data from entropy sources. In particular:

 - Returning random data is infallible. As with all of wasm, it may trap, but
   it never continues execution in a failure state.

 - Entropy never "runs out". Modern CSPRNG algorithms do not have a concept of
   entropy "pools" that are "drained". They can generate a practically unlimited
   stream of data without waiting.

 - This API is not slow. Once initialized, CSPRNG algorithms do not need to
   block waiting for entropy.

## WASI Random is always initialized, as seen by Wasm instances

Implementations must block instantiation of any module that imports any random
API function until sufficient entropy is available to service calls the
function without blocking.

Implementations must initialize their CSPRNG from entropy sources whose
behavior cannot be predicted by someone that knows the contents of all wasm
modules the engine has instantiated and all data input to those modules.

## WASI Random data is never shared

Data returned from WASI random is never shared or reused within an instance,
between instances, or with any external consumer of random data.

Implementations needing deterministic execution must withhold WASI Random at
link time, and prevent modules importing it from instantiating, rather than
providing implementations that return deterministic results.

## Best practices

Implementations must follow best practices for CSPRNG security, including
using backtracking-resistant reseeding, as appropriate for the host
environment.

[CSPRNG]: https://en.wikipedia.org/wiki/Cryptographically-secure_pseudorandom_number_generator
