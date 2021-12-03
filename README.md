# nano-vanity-gen
Nano currency vanity address generator

**Proof of concept; work in progress; no guarantees given as per the license; should only be used for educational purposes**

Also check out the Feeless project: https://github.com/feeless/feeless , which was a helpful resource for creating this project. Specifically for figuring out how to do the key derivations using Rust.

Uses a cryptographically secure RNG ChaCha20rng https://docs.rs/rand_chacha/0.2.1/rand_chacha/struct.ChaCha20Rng.html

Set up the desired vanity properties (start with, end with, containing) in main.rs and the number of threads, then run the project. It will output the first matching seed and index 0 address.

## Run program

```bash
cargo run
```
