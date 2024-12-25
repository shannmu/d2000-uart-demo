use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};

use serialport::SerialPort;

fn fibonacci_iterative(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

// 定义任务函数
fn task(serial: &mut Box<dyn SerialPort>, n: u32, log_file: &mut std::fs::File) -> io::Result<()> {
    let message = "Executing Task A";
    write!(log_file, "[{:?}] {}\n", SystemTime::now(), message)?;
    // 执行任务A的逻辑
    let result = fibonacci_iterative(n);
    serial.write(
        [
            0xeb,
            0x90,
            result.to_be_bytes()[0],
            result.to_be_bytes()[1],
            result.to_be_bytes()[2],
            result.to_be_bytes()[3],
        ]
        .as_ref(),
    )?;

    serial.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let port_path = "/dev/ttyAMA1"; // 串口设备名称
    let baud_rate = 115200; // 串口波特率
    let timeout = Duration::from_secs(5); // 超时时间

    // 打开或创建日志文件
    let log_file_path = "/serial_log.txt";
    let mut log_file = OpenOptions::new()
        .create(true) // 如果文件不存在，则创建文件
        .append(true) // 向文件追加内容
        .open(log_file_path)?;

    // 配置 & 打开串口
    let mut port = serialport::new(port_path, baud_rate)
        .timeout(timeout)
        .open()
        .unwrap();

    loop {
        // 读取数据format: [eb, 90, task_id(4 bytes)]
        let mut buffer: Vec<u8> = vec![0; 6];

        // 读取串口数据
        match port.read_exact(&mut buffer) {
            Ok(()) => {
                let n = u32::from_be_bytes([buffer[2], buffer[3], buffer[4], buffer[5]]);
                task(&mut port, n, &mut log_file)?;
            }
            Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                write!(
                    log_file,
                    "[{:?}] Timeout reached, repeating the last task...\n",
                    SystemTime::now()
                )?;
            }
            Err(e) => {
                write!(
                    log_file,
                    "[{:?}] Error reading from serial port: {:?}\n",
                    SystemTime::now(),
                    e
                )?;
            }
        }
    }
}
