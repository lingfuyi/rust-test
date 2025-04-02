// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。

use std::io::Read;
use std::io::Write;
use std::result::Result;
pub fn question4(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut char_count = 0;
    let mut line_count = 0;
    for c in contents.chars() {
        if c != '\n' {
            char_count += 1;
        }
        line_count += 1;
    }
    let output = format!(
        "Character count: {}\nLine count: {}",
        char_count, line_count
    );
    let mut output_file = std::fs::File::create("output.txt")?;
    output_file.write_all(output.as_bytes())?;
    Ok(output)
}
