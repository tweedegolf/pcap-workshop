# Workshop PCAP parsing

## Useful links

- https://doc.rust-lang.org/std/ documentation of the rust standard library
- https://pcapng.com/ documentation of the PCAPNG format

## Project setup

* `Cargo.toml` specifies our project, and would contain dependencies if we had any
* `Cargo.lock` is a lock file on exact dependency versions
* `src/lib.rs` defines a library. In this case, it contains three modules
* `src/step*.rs` our actual implementation files, containing code and tests

you can run the tests of a particular step like so:

```txt
> cargo test step1
    Finished test [unoptimized + debuginfo] target(s) in 0.21s
     Running unittests src/lib.rs (target/debug/deps/pcap_parser-e7409aea6a7ffffa)

running 1 test
test step1::tests::it_works ... FAILED

failures:

---- step1::tests::it_works stdout ----
thread 'step1::tests::it_works' panicked at 'assertion failed: `(left == right)`
  left: `Ok("")`,
 right: `Ok("\u{3}\0-\0Mac OS X 10.10.4, build 14E46 (Darwin 14.4.0)\0\0\0\u{4}\04\0Dumpcap 1.12.6 (v1.12.6-0-gee1fc")`', src/step1.rs:83:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    step1::tests::it_works

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s
```

This will also generate a bunch of warnings about unused or unreachable code. This is expected. We will fix these warnings as we go.

### step 1

This is a working example, to just get you used to what rust code looks like. Try to get familiar with the source. 

- parse the minor and major version
- Run the tests using `cargo test step1`. This test fails, but shows you the actii text in `options` 
- adjust the test to check the major and minor version. The major version is 1, the minor version is 0.

### step 2

Error handling using `std::option::Option`. So far we have panicked (effectively, aborted) the program on invalid input. For some programs, this is quite reasonable. But random panics that are deeply nested in code are not so nice from a software engineering perspective. We can use the `Option` type and its methods to fail parsing in a controlled way. 

implement the same logic as in step1.rs, but this time

- return an `Option<Self>`, where `None` indicates some sort of parse error
- use (and implement) the `parse_u*` functions to clean up number decoding
- use the `?` operator (see example code) to do an early return when a `None` is hit

### step 3

In the previous step, we added some error handling. But all errors turn into a `None`, so from the outside it is impossible to know what happened exactly. Where did that `None` get created?

When it is important to figure out what the error cause was (and it's not implicit), the `Result<T, E>` type is useful. Instead of just `None`, the error case `Err` contains a value of some type `E`, which can be used to inform consumers about what exactly went wrong.

implement the same logic as in step1.rs, but this time

- return a `Result<Self, ParseError>`
- extend the ParseError with additional error cases
- use the `parse_u*` functions from step2 to clean up number decoding
- use the `?` operator to do an early return when an error is hit

This is meant as a training exercise. In practice you have to decide whether `Option` or `Result` is the better approach. Personally I think `Option` works better in this specific case.

### step 4

Parse another part of the pcapng format. This could be breaking down the `options` part of the SectionBlockHeader, or some other block type like the interface description block.
