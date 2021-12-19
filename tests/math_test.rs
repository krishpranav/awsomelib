#[cfg(test)]
mod lib_test {
    #[test]
    fn test_add() {
        assert_eq!(awsomelib::add(2, 2), 4);
    }
}