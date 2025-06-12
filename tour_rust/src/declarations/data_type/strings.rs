pub fn strings_fn() {
    let hello = String::from("你好");
    println!("strings is {}, 长度: {}", hello, hello.len());

    let mut s = String::new();
    s.push_str("hello");
    s.push_str("string");
    println!("s is {}, len: {}", s, s.len());

    let slice = "hello";
    println!("slice is {}, len: {}", slice, slice.len());
}
    