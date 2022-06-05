mod test_base;

/**
 * 区块链
 * Ethereum
 * 智能合约
 * Solana
 * 密码学
 * EVM
 * 跨链技术
 * 金融
 *
 *
 * cargo clippy: 类似eslint，lint工具检查代码可以优化的地方
 * cargo fmt: 类似go fmt，代码格式化
 * cargo tree: 查看第三方库的版本和依赖关系
 * cargo bench: 运行benchmark(基准测试,性能测试)
 * cargo udeps(第三方): 检查项目中未使用的依赖
 * 另外 cargo build/run --release 使用 release 编译会比默认的 debug 编译性能提升 10 倍以上，
 * 但是 release 缺点是编译速度较慢，而且不会显示 panic backtrace 的具体行号
 */

/// doc line comment 1
/// doc line comment 2
/// doc line comment 3
/// doc line comment 4
fn main() {
    // Tip：Cargo 具有 cargo doc 功能，开发者可以通过这个命令将工程中的说明注释转换成 HTML 格式的说明文档。

    // 普通单行注释
    /*
       普通段落注释
    */
    // rustdoc does not generate documentation for statements
    let commented_clause = "aaa";
    println!("{}", commented_clause);
}

/**
 * 文档注释，在定义的函数，类等之上时， rustdoc才会生效，在语句块中rustdoc是不会生产文档的。
 *
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
fn test_tuple() {
    let tup = ("holy", 1.7, 72);
    let (name, height, weight) = tup;
    println!("{}, {}, {}", name, height, weight);
    println!("{:?}, {:#?}", tup, tup)
}

#[test]
fn test_array() {
    // 长度为5类型为i32的不可变数组
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    // 长度为3类型为&str的不可变数组
    let b = ["January", "February", "March"];
    println!("{:?}", b);

    // 带类型声明的数组
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", c);

    // 快速初始化数组元素初始值
    let d = [3; 5];
    println!("{:?}", d);

    // 数组下标访问
    let first = a[0];
    let second = a[1];
    println!("{},{}", first, second);

    // NOTE 不可变数组
    // a[0] = 123;

    // 重影为可变数组
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
    println!("{:?}", a);
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
    // NOTE 三元运算符
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);
}

#[test]
fn test_func() {
    // NOTE 1- rust 中函数也是一等公民
    // NOTE 2- Rust 函数名称的命名风格是小写字母以下划线分割
    // NOTE 3- Rust不在乎您在何处定义函数，只需在某个地方定义它们即可 LATER

    fn sum_sub(a: i32, b: i32) -> (i32, i32) {
        // NOTE 4- rust 函数不能用
        (a + b, a - b)
    }
    // NOTE 赋值表达式不能作为赋值表达式的右值
    let (sum, sub) = sum_sub(1, 2);

    // NOTE `mut` must be attached to each individual binding
    // NOTE `mut` may be followed by `variable` and `variable @ pattern`
    // let mut (sum, sub) = sum_sub(1, 2);

    println!("{sum},{sub}");

    // NOTE 空元组, 无返回值的函数，可以认为是一个返回空元组的函数
    let empty_tuple = {
        println!("语句块表达式最后一条非;结束的语句 作为语句块表达式的值");
    };
    println!("{:?}", empty_tuple);

    let mul = {
        // NOTE 函数返回类型必须声明，不能被推断
        // NOTE 最后一个非赋值语句作为函数体表达式的值，函数体表达式不等同于函数体不能使用return结束后续语句
        2 * 3
    };
    println!("{mul}");

    fn show(i: i32) {
        if i > 3 {
            return;
        }
        println!("{i}");
    }
    show(4);
}
