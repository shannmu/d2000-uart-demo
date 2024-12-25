use std::io::{self, Write};
use std::time::{Duration, Instant};

// 定义任务函数A
fn task_a() {
    println!("Executing Task A");
    // 执行任务A的逻辑
}

// 定义任务函数B
fn task_b() {
    println!("Executing Task B");
    // 执行任务B的逻辑
}

fn main() -> io::Result<()> {
    let port_path = "/dev/ttyAMA1"; // 串口设备名称
    let baud_rate = 115200; // 串口波特率
    let timeout = Duration::from_secs(1); // 超时时间

    // 配置 & 打开串口
    let mut port = serialport::new(port_path, baud_rate)
        .timeout(Duration::from_secs(5))
        .open()
        .unwrap();

    let mut previous_task: Option<Box<dyn Fn()>> = None; // 用于存储上一个任务

    loop {
        let start_time = Instant::now();

        // 读取数据format: [eb, 90, task_id(4 bytpes)]
        let mut buffer: Vec<u8> = vec![0; 6];

        // 读取串口数据
        match port.read_exact(&mut buffer) {
            Ok(()) => {
                let task_type = u32::from_be_bytes([buffer[2], buffer[3], buffer[4], buffer[5]]);
                match task_type {
                    0x01 => {
                        task_a(); // 执行任务
                        previous_task = Some(Box::new(|| task_a())); // 更新上一个任务
                    }
                    0x02 => {
                        task_b(); // 执行任务
                        previous_task = Some(Box::new(|| task_b())); // 更新上一个任务
                    }
                    _ => {
                        println!("Received unknown data: {:X}", task_type);
                    }
                }
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                println!("Timeout reached, repeating the last task...");

                // 如果超时并且有上一个任务，重复执行
                if let Some(task) = &previous_task {
                    task();
                }
            }
            Err(e) => {
                println!("Error reading from serial port: {:?}", e);
            }
        }

        // 检查是否超时
        if start_time.elapsed() > timeout {
            println!("Timeout reached, repeating the last task...");

            // 如果超时并且有上一个任务，重复执行
            if let Some(task) = &previous_task {
                task();
            }
        }
    }
}
