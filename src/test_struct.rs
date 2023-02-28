/**
 * Rust 中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体，
 * 但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。
 * 元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。
 *
 * 在 Rust 里 struct 语句仅用来定义，不能声明实例，结尾不需要 ; 符号，而且每个字段定义之后用 , 分隔。
 */
#[test]
fn test_struct() {
    #[derive(Debug)] // 不能全局导入,并全局应用
    struct Site {
        domain: String,
        name: String,
        nation: String,
        found: u32,
        ignore: bool,
    }

    let domain = String::from("A");
    let name = String::from("a");

    let site = Site {
        domain, // 等同于 domain : domain,
        name,   // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
        // missing field `ignore` in initializer of `Site`
        ignore: true,
    };

    println!(
        "{}-{}-{}-{}",
        site.domain, site.name, site.nation, site.found
    );
    let site = Site {
        domain: String::from("B"),
        name: String::from("b"),
        ..site
    };
    println!(
        "{}-{}-{}-{}-{}",
        site.domain, site.name, site.nation, site.found, site.ignore
    );

    let site = Site {
        ..site // 最后一行
    };
    println!("{:#?}", site);

    // ---------------------------------------------------------------------------------------------

    /*
     * 元组结构体是一种形式是元组的结构体。
     * 与元组的区别是它有名字和固定的类型格式。
     * 它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：
     *
     * "颜色"和"点坐标"是常用的两种数据类型，但如果实例化时写个大括号再写上两个名字就为了可读性牺牲了便捷性，
     * Rust 不会遗留这个问题。元组结构体对象的使用方式和元组一样，通过 . 和下标来进行访问
     */

    struct Color(String, u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(String::from("black"), 0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    // ---------------------------------------------------------------------------------------------
    /**
     * 方法（Method）和函数（Function）类似，只不过它是用来操作结构体实例的。
     * Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。
     * 结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
     */

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 可以理解为Java中的静态函数
        fn create(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }

        // duplicate definitions with name `area`
        // fn area(&self) -> u32 {
        //     self.width * self.height
        // }
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn wider(&self, rect: &Rectangle) -> bool {
            self.width > rect.width
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    println!("{}, {}", rect1.area(), rect1.wider(&rect2));

    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);

    // ---------------------------------------------------------------------------------------------
    // 单元结构体: 结构体可以只作为一种象征而无需任何成员：
    #[derive(Debug)]
    struct UnitStruct;
    let empty_struct = UnitStruct {};
    println!("{:?}", empty_struct)
}

/**
* 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
* 这就是为什么本章的案例中使用了 String 类型而不使用 &str 的原因。
* 但这不意味着结构体中不定义引用型字段，这需要通过"生命周期"机制来实现。
*/
#[test]
fn test_struct_ownership() {}
