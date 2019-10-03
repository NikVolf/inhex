fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_data() {
        let data: Vec<u8> = inhex::inhex!("
            0efd
            5f74
        ");

        assert_eq!(&data[..], &[0x0e, 0xfd, 0x5f, 0x74][..]);
    }
}