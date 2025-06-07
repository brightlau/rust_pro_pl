/* std::io 标准输入输出库，默认情况 rust 会预导入设定的一些标准库到程序的作用域中，可以在标准库文档中查看。
如果需要的库类型不在预导入的设定中（如 io::stdin），则需要使用 use 显示的引入其程序的作用域中。*/
use std::io;

/**
 * 一、宏调用
 * 在 Rust 中，println! 和 print! 都是宏（都带 ! 符号），Rust 标准库中没有名为 println 或 print 的函数
 * 
 *  // ✅ 正确的宏调用（带感叹号）
 *  println!("Hello World"); 
 *  print!("Rust ");
 *
 *  // ❌ 以下写法都会编译失败（不存在这些函数）
 *  // println("Missing bang!");  // 错误：找不到 `println` 函数
 *  // print(42);                 // 错误：找不到 `print` 函数
 * 
 * 总结关键点：在 Rust 中，所有格式化输出操作都通过宏实现，! 是宏调用的必须标记。这种设计使得 Rust 能在编译期完成格式验证和代码生成，既保证了类型安全，又提升了运行时性能。
 */

pub fn guess_fn() {
    /* println! 调用的是一个 rust 的宏，没有 ! 的调用是 rust 的一个函数。当看到符号 ! 时表示调用的是一个宏而不是函数
    宏并不总是遵循函数的规则 */
    println!("Guess a number!");

    // 输入一个猜测的数字
    println!("Please input your guess!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("You guessed: {guess}");

    
}
