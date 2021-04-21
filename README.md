# rust-qqwry
[![docs](https://docs.rs/qqwry/badge.svg)](https://docs.rs/qqwry)
search ip address with "qqwry.data" using rust

usage example:
```rust
mod tests {
    use crate::qqwry;
    #[test]
    fn it_works() {
       let mut wry = qqwry::QQWry::from(String::from("/Users/zfh/Documents/qqwry.dat"));
       let location = wry.read_ip_location("127.0.0.1");
        println!("{:?}",location.unwrap());
    }
}
```