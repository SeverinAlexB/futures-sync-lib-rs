use crate::asyn::my_rpc::MyRpc;
use crate::asyn::my_rpc::Response;

/// This is only available if the `sync` feature is enabled.
impl MyRpc {
    /// Request a response synchronously.
    pub fn request_sync(&mut self) -> Result<Response, String> {
        smol::block_on(self.request())
    }

    /// Request two responses synchronously.
    pub fn request_two_sync(&mut self) -> Result<(Response, Response), String> {
        smol::block_on(self.request_two())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "sync")]
    fn test_my_rpc_sync() {
        use std::time::Instant;

        let mut my_rpc = MyRpc::new();
        my_rpc.start_background_processor();

        let start = Instant::now();
        let response = my_rpc.request_sync();
        let elapsed = start.elapsed();
        assert!(response.is_ok());
        println!("elapsed: {:?}, response: {:?}", elapsed, response.unwrap());
    }

    #[test]
    #[cfg(feature = "sync")]
    fn test_my_rpc_two_requests_sync() {
        use std::time::Instant;

        let mut my_rpc = MyRpc::new();
        my_rpc.start_background_processor();

        let start = Instant::now();
        let response = my_rpc.request_two_sync();
        let elapsed = start.elapsed();
        assert!(response.is_ok());
        let (response1, response2) = response.unwrap();
        println!("elapsed: {:?}, responses: {:?}, {:?}", elapsed, response1, response2);
    }
}