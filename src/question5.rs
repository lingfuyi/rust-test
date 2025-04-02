// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。

use std::sync::mpsc;
use std::thread;

fn download(url: &str) -> String {
    // 模拟下载过程，这里只是简单地返回 URL 字符串
    format!("{} downloaded OK", url)
}

pub fn question5() {
    let urls = vec![
        "https://www.lingfuyi.github.io",
        "https://www.google.com",
        "https://www.github.com",
    ];
    let (tx, rx) = mpsc::channel();

    for url in urls {
        let tx = tx.clone();
        thread::spawn(move || {
            let result = download(url);
            tx.send(result).unwrap();
        });
    }

    for result in rx {
        println!("{}", result);
    }
}
// 运行结果：
// https://www.lingfuyi.github.io downloaded
// https://www.google.com downloaded
// https://www.github.com downloaded
