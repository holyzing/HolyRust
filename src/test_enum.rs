#[test]
fn test_enum() {
    #[derive(Debug)]
    enum Book {
        Papery,
        Electronic,
    }

    // expected value, found enum `Book`, you might have meant to use one of the enum's variants
    // println!("{:#?}", Book)

    let elec = Book::Electronic;
    println!("{:#?}, {:#?}", Book::Papery, elec);

    // NOTE 同一作用域内, 不能定义同名类型 [struct enum fn]

    #[derive(Debug)]
    enum Book2 {
        Papery(u32),
        Electronic(String),
    }
    println!(
        "{:#?}, {:#?}",
        Book2::Papery(1001),
        Book2::Electronic(String::from("url://..."))
    );
}

#[test]
fn test_match() {
    /* NOTE match
     * 枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。
     * 基于这个原理，往往枚举类最终都会被分支结构处理（许多语言中的 switch ）。
     * switch 语法很经典，但在 Rust 中并不支持，很多语言摒弃 switch 的原因都是因为
     * switch 容易存在因忘记添加 break 而产生的串接运行问题，Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
     *
     * Scala中的模式匹配
     */

    #[derive(Debug)]
    enum Book3 {
        Papery { index: u32, name: String }, // 具有带有名称的属性的一类,但不能像访问结构体属性一样访问
        Electronic(String, u8),              // 具有元组属性的一类
        Music,                               // 无属性的一类
    }

    let book = Book3::Papery {
        index: 1001,
        name: String::from("测试"),
    };
    println!(
        "{:#?}, {:#?}, {:#?}",
        Book3::Music,
        Book3::Electronic(String::from("抵达抵达"), 3),
        book
    );

    // NOTE 枚举类的模式匹配必须穷举完
    let r = match book {
        Book3::Papery { index, name } => {
            println!("Papery book {name}-{}", index);
            true
        }
        // NOTE Named Tuple, 带元组熟悉一类在match时,也必须指定名称
        Book3::Electronic(name, len) => {
            println!("E-book {}-{}", name, len);
            true
        }
        Book3::Music => {
            println!("{:?}", Book3::Music);
            true
        } // unreachable pattern
          // _ => {
          //     false
          // }
    };
    println!("{r}");

    // NOTE match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择。
    // NOTE 其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。

    let t = "abc";
    match t {
        "abc" => println!("Yes"),
        // non-exhaustive patterns: `&_`, not covered the matched value is of type `&str`
        _ => {
            println!("Empty")
        }
    }
}

#[test]
fn test_option() {
    /*
     * Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
     * 许多语言支持 null 的存在（C/C++、Java），这样很方便，但也制造了极大的问题，
     * null 的发明者也承认这一点，"一个方便的想法造成累计 10 亿美元的损失"。
     *
     * null 经常在开发者把一切都当作不是 null 的时候给予程序致命一击：
     * 毕竟只要出现一个这样的错误，程序的运行就要彻底终止。
     *
     * 为了解决这个问题，很多语言默认不允许 null，但在语言层面支持 null 的出现（常在类型前面用 ? 符号修饰）。
     * Java 默认支持 null，但可以通过 @NotNull 注解限制出现 null，这是一种应付的办法。
     *
     * Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：
     */
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }
    // 如果你的变量刚开始是空值，你体谅一下编译器，它怎么知道值不为空的时候变量是什么类型的呢？
    let opt: Option<String> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 这种设计会让空值编程变得不容易，但这正是构建一个稳定高效的系统所需要的。
    // 由于 Option 是 Rust 编译器默认引入的，在使用时可以省略 Option:: 直接写 None 或者 Some()。
    // Option 是一种特殊的枚举类，它可以含值分支选择：
    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }
    let t: Option<i32> = None;
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    // if let 语法可以认为是只区分两种情况的 match 语句的"语法糖", 可以在之后添加一个 else 块来处理例外情况。
    #[derive(Debug)]
    enum Book {
        Papery(u32),
        Electronic(String),
    }
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }

    let book = Book::Papery(32);
    if let Book::Papery(31) = book {
        println!("Papery {:?}", book);
    } else if let Book::Papery(index) = book {
        println!("Papery {:?}", index);
    } else {
        println!("Not papery book");
    }
}
