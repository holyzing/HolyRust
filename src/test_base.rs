/**
 * 不可变变量
 * 变量
 * 常量
 */
#[test]
fn test_var() {
    // println 不是一个函数，而是一个宏规则
    println!("Hello, world!");
    let a = 12;
    let b: i8 = 13;
    println!("a is {}", a);
    println!("a is {{}}");
    println!("a is {0} {1}", a, b);
    println!("a is {} {}", a, b);

    // NOTE rust 不允许有精度损失的数据转换， golang 中则直接没有隐式类型转换
    // a = 3.2

    // Rust 语言为了高并发安全而做的设计：在语言层面尽量少的让变量的值可以改变
    let mut v = "var";
    // NOTE 66666， 相邻两次赋值没有去使用变量 就会编译报错
    println!("{}", v);
    v = "varvar";
    println!("{}", v);
    // NOTE you might be missing a string literal to format with: `"{}", `rustc
    // println!(v)

    // NOTE 不可变变量或者变量均可被 同名不可变变量或者可变变量重复定义,即使前后类型不一致,
    // NOTE 这被称为 “重影”， Golang 中是没有重影的。
    let a = 2.3;
    println!("a is {}", a);
    // NOTE 不可变变量,不能被重复赋值
    // a = 3.0;

    let v = 2;
    println!("var is {}", v);

    // NOTE Syntax Error: missing type for `const` or `static`, 常量和变量之间不能 重影
    // const a = "a";
}

#[test]
fn test_datatype() {
    // 整型
    let int8: i8 = -8;
    let int16: i16 = -16;
    let int32: i32 = -32;
    let int64: i64 = -64;
    let int128: i128 = -128;
    let arch: isize = 32;
    println!("{}-{}-{}-{}-{}-{}", int8, int16, int32, int64, int128, arch);

    // 无符号整型
    let uint8: u8 = b'A';
    let uint16: u16 = 0o77;
    let uint32: u32 = 0xff;
    let uint64: u64 = 64;
    let uint128: u128 = 128;
    let uarch: usize = 0b1111_0000;
    println!(
        "{}-{}-{}-{}-{}-{}",
        uint8, uint16, uint32, uint64, uint128, uarch
    );

    // 浮点
    let mut float64 = 0.64;
    let float32: f32 = 0.32;
    float64 += 1.0;
    // NOTE 不支持 ++， --, 但是 golang 支持 i++, i--
    // int8 ++;
    println!("{float64},{float32}");

    // 布尔
    let wrong = false;
    let right = true;
    println!("{}-{}", wrong, right);

    // 字符 unicode 标量值  U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF
    let s = "string";
    println!("{}", s);
}

#[test]
fn test_if() {
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b is {}", b);

    let a = 3;
    // NOTE 三元运算符, 或者像scala一样任务它是 if 表达式， 并不只限于 if...else... 两个分支
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}

#[test]
fn test_for() {
    let mut number = 1;
    // NOTE 截止 rustc 1.61.0 版本 do 任作为保留字，并未支持 do...while...

    let empty_tuple = while number != 4 {
        println!("number is {number}");
        number += 1
    };
    // NOTE while 表达式不像if表达式一样，它的返回值始终是一个空的元组
    println!("{:?}", empty_tuple);

    // NOTE Rust不支持普通for (i = 0; i < 10; i++) {} 循环，但是Golang支持，
    // NOTE golang不支持 while 循环，但是rust支持

    let a = [1, 2, 3, 4];
    let empty_tuple = for i in a.iter() {
        println!("{i}");
        // can't compare `&{integer}` with `{integer}`
        // the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
        // if i == 2 {
        if i == &2 {
            break;
        }
    };
    // NOTE for 表达式不像if表达式一样，它的返回值始终是一个空的元组
    println!("{:?}", empty_tuple);

    // NOTE 特别像scala中的那些 until to 这些语法糖
    for i in 0..a.len() {
        println!("a[{i}]={}", a[i])
    }

    // NOTE loob 表达式属于值表达式
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        if s[i] == 'D' {
            break i;
        }

        if s[i] == 'B' {
            // mismatched types expected `usize`, found `()`
            // break
        }
        i += 1;
        // NOTE loop 内必须包含break， 否则将是死循环
        if i >= a.len() {
            break i;
        }
    };
    println!(" \'O\' 的索引为 {}", location);
}
