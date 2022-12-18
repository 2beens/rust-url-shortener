<div align="center">

# 🦀 Rust 🦀 URL Shortener 🕸
Learning and practicing the lovely Rust 🦀 through making a URL Shortener service that I use.

</div>

Disclaimer: I deliberatelly don't use any frameworks or such "helpers" whith convenient functions, templates, macros, etc. The reason is to first try to learn pure Rust as much as possible, then at later points, I can use frameworks like [actix-web](https://crates.io/crates/actix-web), [Tokio for async runtime](https://tokio.rs/), etc.

Disclaimer 2: A Rust project made by a Rust beginner, many parts re suboptimal and non-Rust idiomatic (all that exacerbated by Rust being actually one of the hardest modern languages). Even TCP is used for now, and HTTP responses are written to TCP streams (this is giving me some headaches... will change this to use some HTTP lib, to abstract this away).

## TODOs:
 - ✔ create a simple web server
 - ✔ integrate Redis client
 - ✔ automated CI/CD via GitHub Actions;
    - ✔ when a feature branch is merged after a PR, automated CI/CD is triggered, and will deploy the new version to the machine
 - ✔ add business logic
 - add logger
 - add unit tests
