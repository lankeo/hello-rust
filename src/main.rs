fn main() {
    // 1. 变量默认不可变，加 mut 使变量可变
    let mut name = "kean";

    // 打印必须要 {}, 否则报错
    println!("{}", name);
    name = "zzz";
    println!("{}", name);

    // 2. 常量必须大写
    const OLD: u64 = 100;
    println!("{}", OLD);

    // 3. Shadowing(隐藏)
    // 创建一个同名变量覆盖前面的这个变量, 与 mut 不同的是可以改变类型
    let age = "  "; // 字符串
    let age = age.len(); // 数字
    println!("{}", age);

    // mut 改变类型则会报错
    let mut some = "  ";
    // some = some.len() // error
    println!("{}", some);
}
