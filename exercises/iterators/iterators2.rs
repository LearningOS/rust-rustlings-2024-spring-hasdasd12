// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {  
        None => String::new(), // 如果输入为空字符串，返回空字符串  
        Some(first) => {  
            // 将第一个字符转换为大写（如果它是小写ASCII字符的话）  
            let first_char = if first.is_ascii_lowercase() {  
                first.to_ascii_uppercase()  
            } else {  
                first  
            };  
              
            // 收集剩余的字符到字符串中  
            let rest_of_string = chars.collect::<String>();  
            
            // 拼接第一个字符和剩余字符串  
            format!("{}{}", first_char, rest_of_string)  
        }  
    } 
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //vec![]
    words.iter()  
    .map(|word| {  
        let mut chars = word.chars();  
        let first = chars.next().unwrap_or(char::default()).to_ascii_uppercase();  
        let rest = chars.as_str().to_string();  
        format!("{}{}", first, rest)  
    })  
    .collect::<Vec<String>>()  
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter()  
         .filter(|&word| !word.is_empty()) // 过滤掉空字符串  
         .map(|word| {  
             let mut chars = word.chars();  
             let first = chars.next().unwrap_or(char::default()).to_ascii_uppercase();  
             let rest = chars.as_str().to_string();  
             format!("{}{}", first, rest)  
         })  
         .collect::<String>() // 使用collect将迭代器转换为单个字符串  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
