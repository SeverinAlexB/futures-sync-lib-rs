use std::{sync::{Arc, Mutex, atomic::{AtomicU16, Ordering}}, thread, time::{Duration, Instant}};


pub type Response = u16;

/// A struct that represents a message in flight.
pub struct InFlightMessage {
    pub message_id: u16,
    pub start_time: Instant,
    pub tx: flume::Sender<Response>,
}


/// An RPC client that can send requests and receive responses.
/// It auto-correlates the request and response by message_id.
/// 
/// `request()` returns a future that resolves to the response.
/// `request_two()` returns a future that resolves to a tuple of two responses.
/// 
/// To use it sync, activate the `sync` feature and use the `mylib::sync` module.
pub struct MyRpc {
    /// List of inflight requests/messages
    inflight_messages: Arc<Mutex<Vec<InFlightMessage>>>,
    /// Next message ID to use for a new request
    next_message_id: AtomicU16,
    /// Background thread that will receive responses.
    background_thread: Option<thread::JoinHandle<()>>,
}

impl MyRpc {
    pub fn new() -> Self {
        Self { inflight_messages: Arc::new(Mutex::new(Vec::new())), next_message_id: AtomicU16::new(0), background_thread: None }
    }

    /// Start the background thread that will receive responses.
    /// This enables the request() method to be called asynchronously.
    pub fn start_background_processor(&mut self) {
        if self.background_thread.is_some() {
            return;
        }

        let inflight_messages = self.inflight_messages.clone();
        let handle = thread::spawn(move || {
            
            loop {
                // Simulate a response after 400ms
                thread::sleep(Duration::from_millis(100));
                let mut inflight_messages = inflight_messages.lock().unwrap();
                let message = inflight_messages.pop();
                if message.is_none() {
                    continue;
                }
                let message = message.unwrap();
                let _ = message.tx.send(message.message_id);
            }
        });
        self.background_thread = Some(handle);
    }

    /// Send a request and wait for a response.
    pub async fn request(&self) -> Result<Response, String> {
        let message_id = self.next_message_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = flume::bounded(1);
        {
            let mut inflight = self.inflight_messages.lock().unwrap();
            inflight.push(InFlightMessage { message_id, start_time: Instant::now(), tx });
        }

        let response = rx.into_recv_async().await;
        response.map_err(|_| "Failed to receive response".to_string())
    }

    /// Send two requests and wait for two responses.
    /// This demonstrates how to use the request() method to send multiple requests concurrently.
    pub async fn request_two(&self) -> Result<(Response, Response), String> {
        let future1 = self.request();
        let future2 = self.request();
        let (result1, result2) = futures_lite::future::zip(future1, future2).await;
        Ok((result1?, result2?))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_rpc() {
        let mut my_rpc = MyRpc::new();
        my_rpc.start_background_processor();

        let start = Instant::now();
        let response = my_rpc.request().await;
        let elapsed = start.elapsed();
        assert!(response.is_ok());
        println!("elapsed: {:?}, response: {:?}", elapsed, response.unwrap());
    }

    #[tokio::test]
    async fn test_my_rpc_two_requests() {
        let mut my_rpc = MyRpc::new();
        my_rpc.start_background_processor();

        let start = Instant::now();
        let response = my_rpc.request_two().await;
        let elapsed = start.elapsed();
        assert!(response.is_ok());
        let (response1, response2) = response.unwrap();
        println!("elapsed: {:?}, responses: {:?}, {:?}", elapsed, response1, response2);
    }
}