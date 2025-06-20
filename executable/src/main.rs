

/// Tokio executor
// #[tokio::main]
// async fn main() {
//     mylib::asyn::say_hello("tokio").await;
// }

// /// Smol executor
// fn main() {
//     smol::block_on(async {
//         mylib::asyn::say_hello("smol").await;
//     })
// }


fn main() {
    mylib::sync::say_hello("tokio");
}