# inhex
Indented hex strings for your test data

## Usage

```rust
    #[test]
    fn test_data() {
        let data: Vec<u8> = inhex::inhex!("
            0efd
            5f74
        ");

        assert_eq!(&data[..], &[0x0e, 0xfd, 0x5f, 0x74][..]);
    }
```

# License

`inhex` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0), at your choice.

See LICENSE-APACHE, and LICENSE-MIT for details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in inhex by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
