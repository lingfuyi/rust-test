// 实现一个命令行工具，对指定目录下的所有文本文件进行搜索，将匹配结果汇总后输出。
// 为增强可玩性和综合性，该工具需要支持：
// - 命令行参数（接收搜索关键词、目录路径、是否忽略大小写等）。
// - 并发搜索。
// - 消息通信。
// - 数据结构。
// - 错误处理。
// - 文件操作。
// - 迭代器与泛型（文本行迭代、搜索函数可考虑使用泛型或 trait 做一定延伸）。
// - 可选扩展：忽略大小写、正则匹配、统计行数或文件数等。

use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration; // 添加 Duration 的导入

// 定义搜索任务的数据结构
struct SearchTask {
    path: String,
    keyword: String,
    ignore_case: bool,
}

// 定义搜索结果的数据结构
struct SearchResult {
    path: String,
    line_number: usize,
    content: String,
}

// 定义搜索状态的数据结构
struct SearchStatus {
    processed_files: usize,
    total_files: usize,
    results: Vec<SearchResult>,
}

// 实现一个函数来遍历目录并生成搜索任务
fn find_files_in_dir<P>(
    path: P,
    keyword: String,
    ignore_case: bool,
    tx: Sender<SearchTask>,
) -> io::Result<()>
where
    P: AsRef<Path>,
{
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_dir() {
                    find_files_in_dir(entry.path(), keyword.clone(), ignore_case, tx.clone())?;
                } else {
                    let task = SearchTask {
                        path: entry.path().to_string_lossy().to_string(),
                        keyword: keyword.clone(),
                        ignore_case,
                    };
                    tx.send(task).unwrap();
                }
            }
        }
    }
    Ok(())
}

// 实现一个函数来处理搜索任务
fn search_in_file(task: SearchTask, tx: Sender<SearchResult>) {
    let mut file = match fs::File::open(&task.path) {
        Ok(f) => f,
        Err(_) => return,
    };

    let mut contents = String::new();
    if let Err(_) = file.read_to_string(&mut contents) {
        return;
    }

    let contents = if task.ignore_case {
        contents.to_lowercase()
    } else {
        contents
    };
    let keyword = if task.ignore_case {
        task.keyword.to_lowercase()
    } else {
        task.keyword
    };

    for (line_number, line) in contents.lines().enumerate() {
        if line.contains(&keyword) {
            tx.send(SearchResult {
                path: task.path.clone(),
                line_number: line_number + 1,
                content: line.to_string(),
            })
            .unwrap();
        }
    }
}

pub fn question6(keyword: String, directory: String, ignore_case: bool) {
    // 创建消息通道
    let (task_tx, task_rx): (Sender<SearchTask>, Receiver<SearchTask>) = mpsc::channel();
    let (result_tx, result_rx): (Sender<SearchResult>, Receiver<SearchResult>) = mpsc::channel();

    // 使用 Arc 和 Mutex 来共享搜索状态
    let status = Arc::new(Mutex::new(SearchStatus {
        processed_files: 0,
        total_files: 0,
        results: vec![],
    }));

    // 启动一个线程来遍历目录并生成搜索任务
    let directory_clone = directory.clone();
    let keyword_clone = keyword.clone();
    let ignore_case_clone = ignore_case;
    let status_clone = Arc::clone(&status);
    let task_tx_clone = task_tx.clone();
    thread::spawn(move || {
        find_files_in_dir(
            directory_clone,
            keyword_clone,
            ignore_case_clone,
            task_tx_clone,
        )
        .unwrap();
    });

    // 启动一定数量的线程来处理搜索任务
    let mut handles = vec![];
    let num_threads = 4; // 你可以根据需要调整线程数
    for _ in 0..num_threads {
        let result_tx_clone = result_tx.clone();
        let task_rx_clone = task_rx.clone(); // 直接克隆 Receiver
        let status_clone = Arc::clone(&status);
        let handle = thread::spawn(move || {
            while let Ok(task) = task_rx_clone.recv() {
                search_in_file(task, result_tx_clone.clone());
                {
                    let mut status = status_clone.lock().unwrap();
                    status.processed_files += 1;
                }
            }
        });
        handles.push(handle);
    }

    // 主线程接收搜索结果并输出
    loop {
        let mut processing = false;
        let mut status = status.lock().unwrap();
        if let Ok(result) = result_rx.try_recv() {
            processing = true;
            status.results.push(result);
        }
        if status.processed_files == status.total_files {
            break;
        }
        if processing {
            status.total_files = status.processed_files;
        }
        drop(status);
        // 让出 CPU 时间片，避免 CPU 占用过高
        thread::sleep(Duration::from_millis(100));
    }

    // 等待所有工作线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出搜索结果
    let status = status.lock().unwrap();
    for result in &status.results {
        println!(
            "文件: {}, 行号: {}, 内容: {}",
            result.path, result.line_number, result.content
        );
    }
}
