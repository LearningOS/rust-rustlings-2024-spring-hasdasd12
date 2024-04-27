// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.

use core::ops::Mul;  
use core::ops::Deref;  
  
// 获取给定参数的字节数（不是字符数）。  
fn byte_counter<T: Deref<Target = str>>(arg: T) -> usize {    
    arg.deref().bytes().count()    
}  
  
// 获取给定参数的字符数（不是字节数）。  
fn char_counter<T: Deref<Target = str>>(arg: T) -> usize {    
    arg.deref().chars().count()    
}  


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }


}
