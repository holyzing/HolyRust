
#[test]
fn test_tuple() {
    let tup = ("holy", 1.7, 72);
    let (name, height, weight) = tup;
    println!("{}, {}, {}", name, height, weight);
    println!("{:?}, {:#?}", tup, tup);

    // NOTE tuple 不能 切片
    println!("{}, {}", tup.1, tup.2)


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

    let a = &a[..2];
    println!("{:?}", a);

}

#[test]
fn test_slice() {
    // NOTE 这个切片是动词,不是golang中既是名词又是动词, Goalng中的字符串也是可以切片的
    // NOTE 但是无论是golang还是rust,它们的切片都是按照字节进行切分, 如果存在 中文等字符,
    // NOTE 则字符串切片可能会存在乱码问题

    // [..]: Goalng,Python
    // [:] : Rust
    // [to|until] Scala

    // NOTE 切片是对数据值的 "部分引用"

    // 字符串切片

    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{}={}+{}", s, part1, part2);

    /*
     * NOTE the size for values of type `str` cannot be known at compilation time
     * the trait `Sized` is not implemented for `str`
     * all local variables must have a statically known size
     * unsized locals are gated as an unstable feature
     */
    // let part11 = s[0..5];

    // NOTE Rust 中的字符串类型实质上记录了字符在内存中的起始位置和其长度

    let s1 = &s[..s.len()];
    let s2 = &s[0..];
    let s3 = &s[..];
    println!("{s1}, {s2}, {s3}");

    let s = String::from("Hello Rust");
    let s1 = &s[3..4];
    let s2 = &s1[1..2];
    // NOTE 被切片引用的字符串禁止更改其值
    /*
     * cannot borrow `s` as mutable because it is also borrowed as immutable
     * mutable borrow occurs hererustcE0502
     * test_collections.rs(78, 15): immutable borrow occurs here
     * test_collections.rs(82, 21): immutable borrow later used here
     */
    // s.push_str("string");
    println!("{s}, {s1}, {s2}");

    /*
     * str: Rust 核心语言类型, String 或者 str 的切片后的类型就是 str, 字面量的字符串的所有者类型是 &str
     * String: Rust 标准公共库提供的一种数据类型，它的功能更完善——它支持字符串的追加、清空等实用的操作。
     *
     * String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性
     *
       ??? 切片后的字符串来源于母字符串,与母字符串共享地址,所以必须通过引用方式进行访问,而不是通过值拷贝
     */

     // str to String
    let s1 = String::from("hello");
    // String to str
    let s2 = &s1[..];
    println!("{s1}, {s2}");
}