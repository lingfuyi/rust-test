// src/main.rs
mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;
use question1::question1;
use question3::question3;
use question4::question4;
use question5::question5;
use question6::question6;
use std::env;
use std::io;
use std::thread::sleep;
use std::time::Duration;
fn main() {
    //question 1需要读取命令行第一个参数
    println!("问题1测试结果：");
    let args: Vec<String> = env::args().collect();
    let n: i32 = match args.get(1) {
        Some(arg) => arg.parse().unwrap_or(5),
        None => 5,
    };
    //测试问题1

    question1(n).iter().for_each(|s| println!("{}", s));
    //测试问题2
    println!("问题2测试结果：");
    let student = question2::Student::new("lingfuyi", 22, 100.0);
    student.show();
    student.is_passed();
    //测试问题3
    println!("问题3测试结果：");
    let mut input = String::new();
    println!("请输入一行字符串用来测试问题3:");
    io::stdin().read_line(&mut input).expect("读取失败");
    let result = question3(&input.trim());
    for (word, count) in result {
        println!("单词:{} 出现次数:{}", word, count);
    }
    //测试问题4,读取命令行第二个参数作为文件路径
    println!("问题4测试结果：");
    let file_path = match args.get(2) {
        Some(arg) => arg,
        None => {
            println!("没有提供文件路径");
            return;
        }
    };
    let result = question4(file_path);
    match result {
        Ok(output) => println!("输出内容：{}", output),
        Err(e) => println!("错误信息：{}", e),
    }
    println!("问题5测试结果：");
    //测试问题5
    question5();
    sleep(Duration::from_secs(1)); //休眠1s等待线程结束
    println!("问题6测试结果：");
    //测试问题6
    question6(
        "rust".to_string(),
        "/home/lingfuyi/work/rust/rust-test/src".to_string(),
        false,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_question1() {
        assert_eq!(question1(3), vec!["1", "2", "3Fizz"]);
        assert_eq!(question1(5), vec!["1", "2", "3Fizz", "4", "5Buzz"]);
    }
    #[test]
    fn test_question2() {
        use question2::Student;
        let s = Student::new("Alice", 20, 80.0);
        s.show();
        assert_eq!(s.is_passed(), true);
    }
    //     fn test_question3() {
    //         let input = "apple banana pear banana apple banana";
    //         let expected_output = vec![("banana", 3), ("apple", 2), ("pear", 1)];
    //         assert_eq!(question3(&input), &expected_output);
    //     }
}
