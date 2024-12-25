import serial
import struct
import time

# 串口设置
port = "/dev/ttyUSB0"  # 串口设备路径，根据实际修改
baud_rate = 115200     # 波特率
timeout = 5            # 读取超时时间

# 打开串口
ser = serial.Serial(port, baud_rate, timeout=timeout)

# 发送数据函数
def send_data(n):
    # 将n转化为大端格式的四字节
    data = struct.pack('>I', n)  # '>I' 表示将整数打包成大端格式的4字节
    # 发送 0xeb 0x90 n（4字节）
    ser.write(bytes([0xeb, 0x90]) + data)

# 测试发送数据
try:
    while True:
        # 发送一个测试的n值
        n = 2  # 你可以修改这个值
        send_data(n)
        print(f"Sent: 0xeb 0x90 {n:#010x}")
        # 接收数据
        recv = ser.read(6)
        print(f"Received: {recv.hex()}")
        time.sleep(5)  # 每隔5秒发送一次
except KeyboardInterrupt:
    print("Test interrupted by user.")
finally:
    ser.close()

