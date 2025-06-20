

#[tokio::main]
async fn main() {
    mylib::asyn::say_hello("tokio").await;

    mylib::sync::say_hello("sync in tokio");
}
