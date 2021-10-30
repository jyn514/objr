#84738 reproducer

See https://github.com/rust-lang/rust/issues/84738

```bash
$ ./REPRODUCE.sh
warning: output filename collision.
The lib target `objr` in package `objr v0.1.0 (https://github.com/drewcrawford/objr#84d444b4)` has the same output filename as the lib target `objr` in package `objr v0.1.0 (/Users/drew/Code/rustc-84738/objr)`.
Colliding filename is: /Users/drew/Code/rustc-84738/corevideor/target/doc/objr/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
 Documenting coregraphicsr v0.1.0 (/Users/drew/Code/rustc-84738/coregraphicsr)
warning: unused import: `objr::bindings::PerformsSelector`
 --> /Users/drew/Code/rustc-84738/coregraphicsr/src/lib.rs:3:5
  |
3 | use objr::bindings::PerformsSelector;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `coregraphicsr` (lib) generated 1 warning
 Documenting corevideor v0.1.0 (/Users/drew/Code/rustc-84738/corevideor)
thread 'rustc' panicked at 'index out of bounds: the len is 21 but the index is 21', compiler/rustc_metadata/src/creader.rs:146:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic

error: Unrecognized option: 'crate-version'

error: could not document `corevideor`

Caused by:
  process didn't exit successfully: `rustdoc --edition=2021 --crate-type lib --crate-name corevideor src/lib.rs -o /Users/drew/Code/rustc-84738/corevideor/target/doc --error-format=json --json=diagnostic-rendered-ansi -L dependency=/Users/drew/Code/rustc-84738/corevideor/target/debug/deps --extern coregraphicsr=/Users/drew/Code/rustc-84738/corevideor/target/debug/deps/libcoregraphicsr-c2095cd49d9ea14f.rmeta --extern objr=/Users/drew/Code/rustc-84738/corevideor/target/debug/deps/libobjr-781c21f5cee1157f.rmeta --crate-version 0.1.0` (exit status: 1)
  
$ uname -a
Darwin shadowfax.rivendell 21.1.0 Darwin Kernel Version 21.1.0: Wed Oct 13 17:33:23 PDT 2021; root:xnu-8019.41.5~1/RELEASE_X86_64 x86_64

$ cargo --version
cargo 1.56.0 (4ed5d137b 2021-10-04)
$ rustc --version
rustc 1.56.0 (09c42c458 2021-10-18)
```