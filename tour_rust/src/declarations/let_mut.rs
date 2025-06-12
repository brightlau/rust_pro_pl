/**
 * 一、使用 let 语句来创建变量
 * 
 * 在 Rust 中，变量默认是不可变的，这意味着一旦我们给变量赋值，这个值就不可以再修改了。
 * 
 * 在变量名前使用 mut 来使一个变量可变
 * 
 */

pub fn let_mut_fn() {
    // let apple = 5; 新建了一个叫做 apples 的变量并把它绑定到值 5 上
    let apple = 5;
    // 在变量名前使用 mut 来使一个变量可变
    let mut bananas = 10;
    println!("apple is {}, bananas is {}", apple, bananas);
    bananas = 15;
    println!("apple is {}, mut bananas is {}", apple, bananas);
}