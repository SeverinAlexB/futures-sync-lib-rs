use std::{sync::{Arc, Mutex}, thread, time::{Duration, Instant}};


pub type Response = u16;

/// A struct that represents a message in flight.
pub struct InFlightMessage {
    pub message_id: u16,
    pub start_time: Instant,
    pub tx: flume::Sender<Response>,
}


/// An RPC client that can send requests and receive responses.
/// It auto-correlates the request and response by message_id.
/// It supports both async and sync requests.
pub struct MyRpc {
    inflight_messages: Arc<Mutex<Vec<InFlightMessage>> >,
    next_message_id: u16,
    background_thread: Option<thread::JoinHandle<()>>,
}

impl MyRpc {
    pub fn new() -> Self {
        Self { inflight_messages: Arc::new(Mutex::new(Vec::new())), next_message_id: 0, background_thread: None }
    }

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

    pub async fn request(&mut self) -> Result<Response, String> {
        let message_id = self.next_message_id;
        self.next_message_id += 1;
        let (tx, rx) = flume::bounded(1);
        {
            let mut inflight = self.inflight_messages.lock().unwrap();
            inflight.push(InFlightMessage { message_id, start_time: Instant::now(), tx });
        }

        let response = rx.into_recv_async().await;
        response.map_err(|_| "Failed to receive response".to_string())
    }

    #[cfg(feature = "sync")]
    pub fn request_sync(&mut self) -> Result<Response, String> {
        smol::block_on(self.request())
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

    #[test]
    fn test_my_rpc_sync() {
        let mut my_rpc = MyRpc::new();
        my_rpc.start_background_processor();

        let start = Instant::now();
        let response = my_rpc.request_sync();
        let elapsed = start.elapsed();
        assert!(response.is_ok());
        println!("elapsed: {:?}, response: {:?}", elapsed, response.unwrap());
    }
}