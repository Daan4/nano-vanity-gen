# nano-vanity-gen
Nano currency vanity address generator

Uses a cryptographically secure RNG ChaCha20rng https://docs.rs/rand_chacha/0.2.1/rand_chacha/struct.ChaCha20Rng.html

Set up the desired vanity properties (starting with, ending with, containing) in main.rs and the number of threads to use, then run the project. It will output the first matching seed and (index 0) address.

Also check out the Feeless project: https://github.com/feeless/feeless , which was a helpful resource for creating this project. Specifically for figuring out how to do the key derivations using Rust.

## Run program

```bash
# install rust (https://www.rust-lang.org/tools/install)
# download and cd to source

cargo run
```
