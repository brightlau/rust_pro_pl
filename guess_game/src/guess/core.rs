/* std::io 标准输入输出库，默认情况 rust 会预导入设定的一些标准库到程序的作用域中，可以在标准库文档中查看。
如果需要的库类型不在预导入的设定中（如 io::stdin），则需要使用 use 显示的引入其程序的作用域中。*/
use std::io;

pub fn guess_fn() {
    /* println! 调用的是一个 rust 的宏，没有 ! 的调用是 rust 的一个函数。当看到符号 ! 时表示调用的是一个宏而不是函数
    宏并不总是遵循函数的规则 */
    println!("Guess a number!");

    // 输入一个猜测的数字
    println!("Please input your guess!");
    /* let 语句来创建变量 mut 使变量可变 
    let mut guess 会引入一个叫做 guess 的可变变量。等号（=）告诉 Rust 我们现在想将某个值绑定在变量上。
    等号的右边是 guess 所绑定的值，它是 String::new 的结果，这个函数会返回一个 String 的新实例。
    let mut guess = String::new(); 这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上。
    */
    let mut guess = String::new();
    /*
    在程序的第一行使用 use std::io; 从标准库中引入了输入/输出功能。现在调用 io 库中的函数 stdin，这允许我们处理用户输入，等价 std::io::stdin <=> use std::io; io::stdin()
    stdin 函数返回一个 std::io::Stdin 的实例，这是一种代表终端标准输入句柄的类型。
    .read_line(&mut guess) 调用了标准输入句柄上的 read_line 方法，以获取用户输入，将 &mut guess 作为参数传递给 read_line 函数，让其将用户输入储存到这个字符串中。
    read_line 的工作是，无论用户在标准输入中键入什么内容，都将其追加（不会覆盖其原有内容）到一个字符串中，因此它需要字符串作为参数。这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去

    & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。引用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用。

    使用 Result 类型来处理潜在的错误。 read_line 会将用户输入附加到传递给它的字符串中，不过它也会返回一个类型为 Result 的值。Result 是一种枚举类型，通常也写作 enum，它可以是多种可能状态中的一个。我们把每种可能的状态称为一种 枚举成员（variant）。
    Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值。Err 成员则意味着操作失败，并且 Err 中包含有关操作失败的原因或方式的信息
    Result 类型的值，像其他类型一样，拥有定义于其实例上的方法。Result 的实例拥有 expect 方法。如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并输出当做参数传递给 expect 的信息。所以当 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。
    */
    io::stdin().read_line(&mut guess).expect("failed to read line");

    println!("You guessed: {guess}");

    
}
