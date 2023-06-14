pub fn hash_code_str(some_str: &str) -> usize {
    return hash_code(String::from(some_str));
}

//
// Simple String hashCode implementation inspired in Java :-)
// System.out.println("Test".hashCode()) would produce same: 2603186
//
pub fn hash_code(some_string: String) -> usize {
    let l:usize = some_string.len();
    let mut h:usize = 0;
    let mut i:usize = 0;
    if l>0 {
      while i<l {
        let c = some_string.chars().nth(i).unwrap();
        h = (h << 5) - h + (c as usize) | 0;
        i += 1;
      }
    }
    return h;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_code() {
        let result = hash_code(String::from("Test"));
        assert_eq!(2603186,result);
    }

    #[test]
    fn test_hash_code_str() {
        let result = hash_code_str("Test");
        assert_eq!(2603186,result);
    }
}
