#[cfg(test)]
mod tests {
    use crate::qqwry;
    #[test]
    fn it_works() {
       let mut wry = qqwry::Qqwry::from(String::from("/Users/zfh/Documents/qqwry.dat"));
       let location = wry.read_ip_location("127.0.0.1");
        println!("{:?}",location.unwrap());
    }
}

pub mod qqwry;