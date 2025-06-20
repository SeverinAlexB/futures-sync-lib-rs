# futures-sync-lib-rs

An example of a rust library that is async internally, that can be used with any async executor but also without any executor.


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

/// Smol executor
fn main() {
    smol::block_on(async {
        mylib::asyn::say_hello("smol").await;
    })
}
```

To use it sync, activate the `sync` feature and use the sync module.

```toml
mylib = {path = "../mylib", features = [ "sync" ] }
```

```rust
fn main() {
    mylib::sync::say_hello("tokio");
}
```

### MyRpc Example

[MyRpc](./mylib/src/asyn/my_rpc.rs) struct represents a more example where request/responses are correlated.

