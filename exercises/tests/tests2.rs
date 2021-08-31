// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)

fn returnTwo() -> i32{
    return 2;
} 

#[cfg(test)]
mod tests {
    use returnTwo;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2,returnTwo());
    }
}
