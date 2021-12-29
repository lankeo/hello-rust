fn main() {
    let x = add_one(5);
    println!("{}", x);
}

// 函数名规范为小写字母下划线分割
fn add_one(x: i32) -> i32 {
    // x + 1  // 表达式会返回值, 大部分函数隐式的返回最后的表达式作为函数的返回值
    return x + 1; // 语句不会返回值, 可以去掉 ";"，或者使用 return
}
