/*
 * 箱（Crate）
 *      "箱"是二进制程序文件或者库文件，存在于"包"中。
 *      "箱"是树状结构的，它的树根是编译器开始运行时编译的源文件所编译的程序。
 *      注意："二进制程序文件"不一定是"二进制可执行文件"，只能确定是是包含目标机器语言的文件，文件格式随编译环境的不同而不同。
 *
 * 包（Package）
 *      当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。
 *      工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。
 *      一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
 *
 *      当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，
 *      Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。
 *
 * 模块（Module）
 *      对于一个软件工程来说，我们往往按照所使用的编程语言的组织规范来进行组织，组织模块的主要结构往往是树。
 *      Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
 *      这些先进的语言的组织单位可以层层包含，就像文件系统的目录结构一样。Rust 中的组织单位是模块（Module）。
 */

// NOTE 绝对路径从 crate 关键字开始描述。相对路径从 self 或 super 关键字或一个标识符开始描述
// NOTE 非根目录下的文件夹内必须包含 mod.rs 文件, 该文件中必须 mod 引入相对于它的一级 mod
// NOTE 每一个源文件就是一个mod, main,必须声明所有的一级mod,以便于层层引入. 所谓 "难以发现的模块"
// NOTE 同级源文件之间不属于同级 mod,需要通过crate或者super来绝对引入或者是相对引入
#[test]
fn test_mod() {
    /*
     * Rust 中有两种简单的访问权：公共（public）和私有（private）。
     * 默认情况下，如果不加修饰符，模块中的成员访问权将是私有的。
     * 如果想使用公共权限，需要使用 pub 关键字。
     *
     * 对于私有的模块，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问。
     *
     * 访问路径上必须是绝对公开,即所有路径上的资源必须公开,才能访问到目标公开资源.
     */
    crate::advance_mods::test_mod::nation::government::govern();
    super::advance_mods::test_mod::nation::government::govern();
    self::hello_mod();
    hello_mod();

    // NOTE 如果模块中定义了结构体，结构体除了其本身是私有的以外，其字段也默认是私有的。
    // NOTE 所以如果想使用模块中的结构体以及其字段，需要 pub 声明
    // NOTE Rust 结构体初始化,必须显式初始化所有成员, 这就有点恶心

    /*
    super::advance_mods::test_mod::Breakfast {
        toast: String::from("toast"),
        // NOTE field `seasonal_fruit` of struct `Breakfast` is private field
        seasonal_fruit: String::from("toast"),
    };
    */
    let mut meal = super::advance_mods::test_mod::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // NOTE 枚举类的枚举项以及枚举项内含的字段，显然不能定义它们的访问权限,它们的访问权限只和枚举类保持一致
    let person = crate::advance_mods::test_mod::Person::King {
        name: String::from("Blue"),
    };
    match person {
        crate::advance_mods::test_mod::Person::King { name } => {
            println!("{}", name);
        }
        _ => {}
    }

    // NOTE use 关键字能够将模块标识符引入当前作用域, 可以解决引入局部模块路径过长的问题。
    use crate::advance_mods::test_mod;
    test_mod::govern();
    // NOTE 有些情况下存在两个相同的名称，且同样需要导入，可以使用 as 关键字为标识符添加别名
    use crate::advance_mods::test_mod::govern;
    govern();
    // the name `govern` is defined multiple times `govern` must be
    // defined only once in the value namespace of this block
    // use super::advance_mods::test_mod::nation::government::govern;
    use super::advance_mods::test_mod::nation::government::govern as inner_govern;
    inner_govern();

    // NOTE module `court` is private module
    // super::advance_mods::test_mod::nation::court::judicial();
    super::advance_mods::test_mod::nation::judicial();

    use std::f64::consts::PI;
    println!("{}", (PI / 2.0).sin());
}

#[allow(dead_code)]
fn hello_mod() {
    println!("Hello Mod !")
}
