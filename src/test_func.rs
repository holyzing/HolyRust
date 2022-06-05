#[test]
fn test_func() {
    // NOTE 1- rust 中函数也是一等公民
    // NOTE 2- Rust 函数名称的命名风格是小写字母以下划线分割
    // NOTE 3- Rust不在乎您在何处定义函数，只需在某个地方定义它们即可 LATER

    fn sum_sub(a: i32, b: i32) -> (i32, i32) {
        // NOTE 4- 函数返回类型必须声明，不能被推断
        (a + b, a - b)
        // NOTE 和Scala的一些语法概念很相似，Scala 中几乎所有事物都可视为函数，也可视为对象
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