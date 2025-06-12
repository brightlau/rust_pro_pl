/* std::io 标准输入输出库，默认情况 rust 会预导入设定的一些标准库到程序的作用域中，可以在标准库文档中查看。
如果需要的库类型不在预导入的设定中（如 io::stdin），则需要使用 use 显示的引入其程序的作用域中。*/
use std::{cmp::Ordering, io};
use rand::Rng;

/**
 * 知识点：
 * 
 * 0、toolchain::cargo 项目构建 cargo new 、cargo build 、cargo run 
 * 1、macros 宏处理
 * 2、function 函数定义 pub
 * 3、library::iostream 标准输入输出 println、stdin read_line()
 * 4、declarations 声明 let mut
 * 5、declarations 数据类型 string、new() trim() parse() cmp() Ordering 
 * 6、declarations 数据类型 枚举类型 enum
 * 7、exceptions 异常处理 result、expect() Ok() Err() 
 * 8、declarations 数据类型 指针与引用
 * 9、basic_concepts::crate 编译单元、库文件、可执行文件
 * 10、trait 接口定义共同行为
 * 11、library::rand 随机数库 rand::Rng thread_rng() gen_range()
 * 12、expressions::match {} 模式匹配
 * 13、statements::loop {} 循环 break
 * 
 */

pub fn guess_fn() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("your guessed is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("ok");
                break;
            }
        };        
    }

}
