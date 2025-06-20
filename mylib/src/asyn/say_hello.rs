use std::time::{Duration, Instant};

/// Async version of say_hello, independent of an async executor like smol or tokio
pub async fn say_hello(name: &str) {
    let start = Instant::now();
    println!("Hello, {}! sleep for 1 second", name);
    
    futures_timer::Delay::new(Duration::from_secs(1)).await;
    println!("Hello, {}! after 1 second. elapsed: {:?}", name, start.elapsed());
}


#[cfg(test)]
mod tests {
    use super::*;
    use async_io::block_on;

    #[test]
    fn test_say_hello_async_io() {
        block_on(say_hello("async_io"));
    }


    #[tokio::test]
    async fn test_say_hello_tokio() {
        block_on(say_hello("tokio"));
    }
}