use std::fs::File;
use std::io;
use std::io::Read;

// 不使用 ? 运算符的情况
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,                    // 成功时继续
//         Err(e) => return Err(e),              // 失败时提前返回错误
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),                       // 成功时返回结果
//         Err(e) => Err(e),                      // 失败时返回错误
//     }
// }

// 使用了 ? 运算符简化之后的版本。
// ? 运算符的工作原理
// 如果结果是 Ok ：提取出其中的值，继续执行。
// 如果结果是 Err ：从当前函数提前返回这个错误。
fn read_username_from_file() -> Result<String, io::Error> {
    // 依旧是读取本目录下的文件。
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn main() {
    let s: String = read_username_from_file().expect("文件不存在");

    println!("{s}");
}

