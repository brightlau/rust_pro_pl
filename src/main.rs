mod manager;

use manager::game::game;

// 程序入口函数 fn 声明函数， () 参数， {} 函数体
fn main() {
    
    // 猜字游戏库
    game();

    // tour_rust rust 学习库
    println!("{}", tour_rust::add(10, 20));
    tour_rust::tour_rust_fn();
}
