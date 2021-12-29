fn main() {
    // 1. 分支
    let num = 6;

    // 条件不用加括号（加了会报警告），条件必须为 bool 值
    if num < 5 {
        println!("value({}) less than 5", num)
    } else if num == 6 {
        println!("value({}) is right    ", num)
    } else {
        println!("value({}) greater than 5", num)
    }

    // 类似三目运算，两个代码块的值类型必须相同（否则报错）
    let _num2 = if num == 6 { 5 } else { 6 };

    // 2. 循环
    // loop {
    //     println!("loop again!");
    // }

    // 循环标签（loop label）, 在跳出嵌套循环的时候很有用，可以通过 break 一个表达式作为返回值
    let mut count = 0;

    let result = 'outside_label: loop {
        let mut count1 = 0;
        loop {
            if count == 2 {
                break 'outside_label count * 2;
            }
            if count1 == 3 {
                break;
            }
            count1 += 1;
        }
        count += 1;
    };
    println!("{} {}", count, result);


    // 条件循环 （while）
    let mut count2 = 0;
    while count2 < 3 {
        println!("{}", count2);
        count2 += 1;
    }

    // for 循环遍历集合
    let array = [10, 20, 30, 40, 50];

    // iter 方法返回迭代器，rev 方法反转迭代器
    for element in (array.iter()).rev() {
        println!("the value is: {}", element);
    }
}
