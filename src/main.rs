fn main() {
    // tips(1): 类型转化时必须添加类型注解 .expect(msg &str)，否则会报错，msg 可以随便输（不建议）
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // 1. scalar(标量)
    // 整型， arch 依赖运行平台，64 位的机器则是 64 位，整形溢出， debug 下会导致 panic 错误，release 下会直接溢出
    // 有符号: i8(一字节), i16(两字节), i32(默认，四字节), i64(八字节), i128(十六字节), isize(arch)
    // 无符号: u8(一字节), u16(两字节), u32(四字节), u64(八字节), u128(十六字节), usize(arch)
    let mut _num = -2147483648; // 十进制
    _num = 0xff; // 十六进制
    _num = 0o77; // 八进制
    _num = 0b1111_0000; // 二进制
    let _char = b'A'; // 单字节字符（仅限于 u8）

    // 浮点型: f64(默认), f32
    let _x = -2.0;
    let _y: f32 = 3.0;

    // tips(2): 只有相同类型直接才能进行计算
    // let quotient = 56.7 / 2; // error

    // 布尔型
    let _t = true;
    let _f: bool = false; // 显式指定类型注解

    // 字符类型（四个字节），与其他语言的不同，可以存下中文，韩文，emoji 等
    let _lizi = '🌰';

    // 2. compound (复合类型)
    // 元组
    let _info: (&str, i32, u8) = ("kean", 20, 1);
    let _nums = (500, 8, 1.0);
    let _single = (); // 单元类型, _single 称为单元值（unit value）
    let (name, age, gender) = _info; // 元组解构
    println!("{}-{}-{}({})", name, age, gender, _info.2); // 通过索引访问，超出索引范围会报错

    // 数组
    let _array = [1, 2, 3, 4, 5];
    let _array1: [i32; 5] = [1, 2, 3, 4, 5];
    let _array2 = [3; 4]; // 相当于 [3, 3, 3, 3]
    println!("{})", _array[2]); // 通过索引访问，超出索引范围会报错
}
