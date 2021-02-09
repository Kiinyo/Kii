pub fn aeiou () -> String {
    ("aeiou").to_string()
}
pub fn uoiea () -> String {
    ("uoiea").to_string()
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
    #[test]
    fn uoiea_test() {
        assert_eq!(crate::uoiea(), "uoiea");
    }
}
