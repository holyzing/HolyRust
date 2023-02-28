mod test_base;
mod test_collections;
mod test_func;
mod test_ownership;

mod test_enum;
mod test_pointer;
mod test_struct;

mod test_mod;

mod advance_mods;
mod test_error;
mod test_generic;
mod test_lifecycle;
mod test_trait;

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
/**
 * 文档注释，在定义的函数，类等之上时， rustdoc才会生效，在语句块中rustdoc是不会生产文档的。
 */
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
