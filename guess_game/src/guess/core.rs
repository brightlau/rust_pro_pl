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
 * 
 * 语言	            核心目标	                        输出机制实现思路
 * Rust	           安全 + 零成本抽象 + 元编程	         宏系统在编译期构建AST，保证安全高效
 * Go	           简单 + 快速开发 + 运行时安全	         基于反射的运行时检查，牺牲性能换安全
 * C/C++	       极简主义 + 硬件控制	                原始指针操作，信任程序员
 * 
 * 核心差异矩阵
 * 维度	            Rust (println!)	                   Go (fmt.Println)	                C/C++ (printf)
 * 类型安全	        编译期担保 (100% 安全)	              运行时检查 (安全但有损)	          无保护 (高危)
 * 性能	           零成本 (编译期优化)	                  运行时反射 (有开销)	             解析开销 (可部分优化)
 * 灵活性	       宏 + Trait (高扩展性)	             接口 + 反射 (中等)	                仅基础类型 (低)
 * 错误处理	        编译失败 (快速反馈)	                  运行时错误输出 (延迟发现)	           未定义行为 (难调试)
 * 适用场景	        系统编程 + 高性能应用	               应用开发 + 快速原型	              底层/遗留系统
 * 
 * 关键结论：
 * Rust 的 println! 通过宏系统和编译期检查，在输出领域实现了：
 * ✅ 绝对类型安全 + ✅ 零成本抽象 + ✅ 语法扩展自由
 * 这是 Rust 追求「内存安全不妥协」+「性能不妥协」的典型体现。
 */

pub fn guess_fn() {
    /* println! 调用的是一个 rust 的宏，没有 ! 的调用是 rust 的一个函数。当看到符号 ! 时表示调用的是一个宏而不是函数
    宏并不总是遵循函数的规则 */
    println!("Guess a number!");

    // 输入一个猜测的数字
    println!("Please input your guess!");
    /* let 语句来创建变量 mut 使变量可变 */
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("You guessed: {guess}");

    
}
