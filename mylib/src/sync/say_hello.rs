use crate::asyn;

/// Sync version of say_hello
pub fn say_hello(name: &str) {
    async_io::block_on(asyn::say_hello(name));
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say_hello_sync() {
        say_hello("sync");
    }
}