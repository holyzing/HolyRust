/**
 * Rust 有一套独特的处理异常情况的机制，它并不像其它语言中的 try 机制那样简单。
 * 首先，程序中一般会出现两种错误：可恢复错误和不可恢复错误。
 * 可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。
 * 但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。
 * 大多数编程语言不区分这两种错误，并用 Exception （异常）类来表示错误。
 * 在 Rust 中没有 Exception。
 * 对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。
 */
#[test]
fn test_panic() {
    panic!("使用 panic 宏 触发一个不可恢复错误")
    // 不可恢复的错误一定会导致程序受到致命的打击而终止运行。
    // 回溯是不可恢复错误的另一种处理方式，它会展开运行的栈并输出所有的信息，然后程序依然会退出

    // RUST_BACKTRACE=full, 告诉编译器运行时,如何打印堆栈信息
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}

#[test]
fn test_result() {
    // 可恢复错误, 此概念十分类似于 Java 编程语言中的异常。
    // 实际上在 C 语言中 一般会将将函数返回值设置成整数来表达函数最终返回的状态,成功或者因为何种原因失败，
    // 在 Rust 中通过 Result<T, E> 枚举类作返回值来进行异常表达, 当然你也可以自定义自己函数的返回值.

    // NOTE 在 Rust 标准库中可能产生异常的函数的返回值都是 Result 类型的
    use std::fs::File;
    let f = File::open("tmp/test.txt");
    if let Result::Ok(file) = f {
        if let Ok(meta) = file.metadata() {
            let ft = meta.file_type();
            println!("{:?}", ft)
        };
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }

    // NOTE 如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：
    // unwrap() 和 expect(message: &str) ：
    // 相当于在 Result 为 Err 时调用 panic! 宏。两者的区别在于 expect 能够向 panic! 宏发送一段指定的错误信息。
    let f1 = File::open("hello.txt").unwrap();
    println!("{}", f1.metadata().is_err());
    let f2 = File::open("hello.txt").expect("Failed to open.");
    println!("{}", f2.metadata().is_err())
}

#[test]
fn test_err_pass() {
    // NOTE 复习 Goalng 的 recover
    fn f(i: i32) -> Result<i32, bool> {
        if i >= 0 {
            Ok(i)
        } else {
            Err(false)
        }
    }
    let r = f(10000);

    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }

    /**
     * 函数 g 传递了函数 f 可能出现的错误（这里的 g 只是一个简单的例子，实际上传递错误的函数一般还包含很多其它操作）。
     * 这样写有些冗长，Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去：
     */
    fn g(i: i32) -> Result<i32, bool> {
        let t = f(i)?;
        Ok(t)
        // 表达冗长
        /*
        return match t {
            Ok(i) => Ok(i),
            Err(b) => Err(b)
        };
         */
    }

    /*
      NOTE ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
    * 所以，? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 Result 的 E 类型一致。
    */

    let r = g(10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}

#[test]
fn test_kind() {
    /*
     * Rust似乎没有像try块一样可以令任何位置发生的同类异常都直接得到相同的解决的语法，但这样并不意味着Rust实现不了：
     * 完全可以把 try 块在独立的函数中实现，将所有的异常都传递出去解决。
     * 实际上这才是一个分化良好的程序应当遵循的编程方法：应该注重独立功能的完整性。
     *
     * 但是这样需要判断 Result 的 Err 类型，获取 Err 类型的函数是 kind()。
     */

    use std::fs::File;
    use std::io;
    use std::io::Read;
    fn read_text_from_file(path: &str) -> Result<String, io::Error> {
        let mut f = File::open(path)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("No such file");
            }
            _ => {
                println!("Cannot read the file");
            }
        },
    }
}
