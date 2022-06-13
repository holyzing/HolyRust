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

mod nation {                    // 国家
    pub mod government {        // 政府
        pub fn govern() {       // 行政
            say();
            self::say()
        }
        fn say(){}
    }
    mod congress {              // 议会
        pub fn legislate() {    // 立法

        }
    }
    mod court {                 // 法院
        fn judicial() {         // 司法
            super::congress::legislate();
        }
    }
    fn useNation() {

    }
}

// NOTE 绝对路径从 crate 关键字开始描述。相对路径从 self 或 super 关键字或一个标识符开始描述
#[test]
fn test_mod() {
    crate::test_mod::test_mod::nation::government::govern();
    self::nation::government::govern();
    super::test_mod::nation::government::govern();
    nation::government::govern();

}
