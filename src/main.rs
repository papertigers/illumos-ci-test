fn main() {
    println!("Hello, world!");
}

mod test {
    #[test]
    fn test_zoneid() {
        assert_eq!(zonename::getzoneid().expect("failed to get zoneid"), 0);
    }
}
