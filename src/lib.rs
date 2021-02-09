pub fn aeiou () -> String {
    ("aeiou").to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn aeiou_test() {
        assert_eq!(crate::aeiou(), "aeiou");
    }
}
