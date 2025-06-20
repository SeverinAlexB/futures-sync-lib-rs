# futures-sync-lib-rs

An example of a Rust library that is async internally and can be used with any async executor like tokio or smol, but also without any executor synchronously.

This is one way to solve the problem of writing Rust libraries that work both synchronously and asynchronously at the same time.


## Structure

- `mylib` is the main library.
- `executable` is a project using `mylib`.

`mylib` is async by default and can be used with any executor.

```rust
/// Tokio executor
#[tokio::main]
async fn main() {
    mylib::asyn::say_hello("tokio").await;
}

/// async_io executor
fn main() {
    async_io::block_on(async {
        mylib::asyn::say_hello("async_io").await;
    })
}
```

To use it synchronously, activate the `sync` feature and use the `mylib::sync` module. 
Internally, this will add sync wrappers around the async functions and add [async_io](https://github.com/smol-rs/async-io) to execute them.
This works well in combination with any other global async executor like tokio because async_io is only used to execute mylib async functions.

```toml
mylib = {path = "../mylib", features = [ "sync" ] }
```

```rust
fn main() {
    mylib::sync::say_hello("sync");
}
```

### MyRpc Example

The [MyRpc](./mylib/src/asyn/my_rpc.rs) struct represents a more complex example where requests and responses are correlated.

## FAQ

**Can I Use Tokio Methods?**
Make sure to only use executor-independent methods in the library like [futures_lite](https://github.com/smol-rs/futures-lite) and not
executor-specific libs like tokio or smol to guarantee compatibility.

**How much will this increase the library size?** The async library is not any bigger than a regular library. Only when activating the `sync` feature, the async_io executor will add 100KB additionally. So this approach might not be the perfect fit for tiny libraries.





