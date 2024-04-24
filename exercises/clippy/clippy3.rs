// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


fn main() {  
    let my_option: Option<()> = None;  
  
    if my_option.is_none() {  
        // 不需要调用 unwrap，因为我们已经知道它是 None  
        println!("my_option is None");  
    }  
  
    let my_arr = &[-1, -2, -3, // 这里添加了逗号  
                   -4, -5, -6]; // 这里也添加了逗号  
  
    println!("My array! Here it is: {:?}", my_arr);  
  
    let value_a = 45;  
    let value_b = 66;  
  
    // 使用元组解构来交换这两个值  
    let (value_a, value_b) = (value_b, value_a);  
  
    println!("value a: {}; value b: {}", value_a, value_b);  
}