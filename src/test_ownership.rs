/**
 * 所有权对大多数开发者而言是一个新颖的概念，它是 Rust 语言为高效使用内存而设计的语法机制。
 * 所有权概念是为了让 Rust 在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念。
 *
 * 编译器无法确定占用内存大小的变量,在运行时肯定是要被分配到堆上的
 * Rust并没有直接提供操作内存(释放内存)的指令, 而是在编译器就确定是否在变量作用域结束的时候,就主动释放该变量占用的资源
 *
 * 在一个函数内,基本数据类型的"变量本身就是变量的值", 它们所占用的内存空间一般都是栈中的内存空间,
 * 而引用类型的变量的值是是保存在栈中的一个指向堆内存的指针(地址),该堆则是该变量所代表的 "真实内容".
 *
 *
 * 所有权有以下三条规则：
 *      Rust 中的每个值都有一个变量，称为其所有者
 *      一次只能有一个所有者。
 *      当所有者不在程序运行范围时，该值将被删除。
 *
 * rust 是集合 编译时内存安全,快速,表现力强的特点.
 * 但是内存安全并不是说rust解决了线程安全问题, 毕竟这是运行时问题
 *
 */

#[test]
fn test_str_memory() {
    let s1 = String::from("this is a string");
    println!("{s1}");
    let s2 = s1;
    println!("{s2}");

    // NOTE 发生了值移动,也就是堆中的内容只能被一个变量所指向
    // println!("{s1}");
    /*
     borrow of moved value: `s1`
     value borrowed here after moverustcE0382
     macros.rs(103, 28): Error originated from macro call here
     test_manage_memory.rs(20, 6): Error originated from macro call here
     test_manage_memory.rs(17, 15): value moved here
     test_manage_memory.rs(15, 10): move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    */

    /*
    Rust会尽可能地降低程序的运行成本，所以默认情况下，长度较大的数据存放在堆中，且采用引用移动的方式进行数据交互。
    但如果需要将数据单纯的复制一份以供他用，可以使用数据的第二种交互方式——克隆。
    */

    // NOTE 值克隆
    let s3 = s2.clone();
    println!("{s3}");
}

#[test]
fn test_fn_args_memory() {
    // 如果将变量当作参数传入函数，那么它和移动的效果是一样的。
    // 被当作函数返回值的变量所有权将会被移动出函数并返回到调用函数的地方，而不会直接被无效释放。
}

#[test]
fn test_reference() {
    // 实质上"引用"是变量的间接访问方式。
    // 当一个变量的值被引用时，变量本身不会被认定无效。因为"引用"并没有在栈中复制变量的值：

    // 引用不会获得值的所有权。
    // 引用只能租借（Borrow）值的所有权。
    // 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权

    let s1 = String::from("hello");
    let s2 = &s1;
    println!("房东 s1:{s1}, 租客 s2:{s2}");

    fn borrow(str: &String) -> usize {
        println!("租客 str:{str}");
        str.len()
    }

    let str_len1 = borrow(s2);
    println!("字符串长度 {str_len1}");

    // NOTE 需要一个借用,却给了一个移动, 可见rust的官方支持 文档措辞上 还是挺贴近其实现理念的
    // let str_len2 = borrow(s1);
    // consider borrowing here: `&s1`rustcE0308
    // test_manage_memory.rs(72, 27): original diagnostic
    // expected &String, found Stringrust-analyzertype-mismatch
    // mismatched types
    // expected `&String`, found struct `String`rustcE0308
    // test_manage_memory.rs(72, 27): consider borrowing here: `&s1`

    let s1 = String::from("hello");
    let s2 = &s1;
    // NOTE cannot move out of `s1` because it is borrowed,
    // NOTE in other words, can not move a borrowed var befor the borrower to be used.
    // let s3 = s1;
    println!("{}", s2);

    let mut s2 = &s1;
    println!("{}", s2);
    // NOTE s2` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // s2.push_str("rust");

    let s3 = s1;
    s2 = &s3;
    println!("{}", s2);

    // mismatched types; expected reference `&String` found reference `&&'static str`
    // s2 = &"";

    // NOTE 二房东
    let mut s = String::from("hello");
    println!("s:{s}");
    let s1 = &s;
    println!("s:{s}, s1:{s1}");

    s.push_str("rust");
    // NOTE cannot borrow `s` as mutable because it is also borrowed as immutable
    // NOTE mutable borrow occurs here
    // println!("{s1}");
    println!("s:{s}");

    let s2 = &mut s;
    s2.push_str("again");
    println!("s2:{s2}");

    // NOTE 可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("{}", r1);
    // NOTE cannot borrow `s` as mutable more than once at a time
    // NOTE second mutable borrow occurs here
    let r2 = &mut s;
    // println!("{}", r1);
    println!("{}", r2);

    // NOTE Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种事情的发生。
    // NOTE 由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同时被至少一个其他使用者读或写，
    // NOTE 所以在一个值被可变引用时不允许再次被任何引用。

    // NOTE 悬空指针 (非空指针)
    // NOTE golang 和 rust 都在弱化指针,但又没像java和python一样完全弱化
    // missing lifetime specifier
    // this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    /*
    fn dangle() -> &String {
        let s = String::from("hello");
        &s
    }
    // 伴随着 dangle 函数的结束，其局部变量的值本身没有被当作返回值，被释放了。但它的引用却被返回，
    // 这个引用所指向的值已经不能确定的存在，故不允许其出现。
    */

    /*
     * Rust 区别与其他高级语言的重要特征，在于其内存管理的两个特点：
     * （1）变量超出作用域会自动释放。对于简单值类型的栈内存（如int，struct）超出作用域后自动释放，这个逻辑在各个语言都有实现。
     *      而对于 new 出来的堆内存，在c/c++中是要手动释放的，在java和python以及golang中要委托垃圾回收释放或手动写 dispose 语句释放。
     *      而垃圾回收不是实时的，会影响性能。而释放语句总会有懒人忘记写的。而 Rust 对栈内存和堆内存一视同仁，超出作用域一律自动释放。
     *      Rust 的这个特点在兼顾性能的情况下、有效的减少了代码量和内存泄漏隐患。
     * （2）“所有权” ：某段内存只能被最后的变量名所有，前面声明过的变量都作废，这有效的避免被多个变量释放的问题，
     *      而且该操作是在编译期就可以检查到的，这策略可在编译期就能有效的避免空指针问题。
     *
     * 其实本质上就是在语言层面禁止了同一个可变数据会有多个变量引用的情况，一旦作为参数传递了，
     * 就会发生所有权的移动（Move）或借用（Borrow）。
     * 赋值给另一个变量也就自动放弃了所有权。从根本上杜绝了并发情景下的数据共享冲突。
     */
}
