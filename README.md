# futures-sync-lib-rs

An example of a rust library that is async internally and that can be used with any async executor but also without any executor.

This is one way to solve the problem of writing rust libraries sync or async at the same time.


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

To use it sync, activate the `sync` feature and use the `mylib::sync` module. 
Internally, this will add sync wrappers around the async functions and add [async_io](https://github.com/smol-rs/async-io) to execute it.
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

[MyRpc](./mylib/src/asyn/my_rpc.rs) struct represents a more example where request/responses are correlated.

## FAQ

**Can I Use Tokio Methods?**
Make sure to only use executor independent libraries in the library like [futures_lite](https://github.com/smol-rs/futures-lite) and not
executor specific libs like tokio or smol to guarantee compatability.

**How much will this increase the library size?** The async library is not any bigger than a regular library. Only when activating the `sync` feature, the async_io executor will add 100KB additionally.





