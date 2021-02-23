fn main() {
    println!("Hello, world!");
}

mod test {
    #[test]
    fn test_zoneid() {
        assert_eq!(zonename::getzoneid().expect("failed to get zoneid"), 0);
    }

    #[test]
    fn test_zonename() {
        let zoneid = zonename::getzoneid().expect("failed to get zoneid");
        let zonename = zonename::getzonenamebyid(zoneid).expect("failed to get zone name");

        assert_eq!(&zonename, "global");
    }
}
