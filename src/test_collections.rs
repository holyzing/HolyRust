
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