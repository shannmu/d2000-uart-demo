import serial
import struct
import time

def get_result_from_non_root_cell(tid: int):
    if not isinstance(tid, int):
        raise
    
    # 串口设置
    port = "/dev/ttyUSB0"  # 串口设备路径，根据实际修改
    baud_rate = 115200     # 波特率
    timeout = 5            # 读取超时时间
    
    ser = serial.Serial(port, baud_rate, timeout=timeout)
    
     # 将n转化为大端格式的四字节
    data = struct.pack('>I', tid)  # '>I' 表示将整数打包成大端格式的4字节
    # 发送 0xeb 0x90 n（4字节）
    ser.write(bytes([0xeb, 0x90]) + data)
    ser.flush()
    
    recv = ser.read(6)
    # 过滤特殊标记字符0xeb0x90
    recv = recv[2:]
    # recv剩余四字节, 大端法转化为int
    res = struct.unpack('>I', recv)[0]
    
    
    ser.close()
    
    return res
