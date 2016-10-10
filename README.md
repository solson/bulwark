# Bulwark

An implementation of "scope guards", little bits of code scheduled to run when
the current scope exits.

* `scope_exit! { ... }` executes the block whenever the surrounding scope exits.
* `scope_success! { ... }` executes the block only when the surrounding scope
  exits without panicking.
* `scope_failure! { ... }` executes the block only when the surrounding scope
  exits due to a panic.

## Cargo.toml

```toml
[dependencies]
bulwark = "0.1.0"
```

## Example

**Code**:

```rust
#[macro_use]
extern crate bulwark;

fn main() {
  scope_failure! {
    println!("The main thread has panicked!");
  }

  {
    scope_exit! {
      println!("Leaving inner scope.");
    }

    scope_success! {
      println!("Never printed, because of the below panic in this scope.");
    }

    println!("In the inner scope.");
    panic!("Woman overboard!");
  }

  println!("End of main never reached, but the scope_failure still runs.");
}
```

**Output**:

```
In the inner scope.
thread 'main' panicked at 'Woman overboard!', src/main.rs:19
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Leaving inner scope.
The main thread has panicked!
```

## License

Licensed under either of
  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
    http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
    http://opensource.org/licenses/MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
