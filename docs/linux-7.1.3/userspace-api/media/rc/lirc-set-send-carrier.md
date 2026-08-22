######## ioctl LIRC_SET_SEND_CARRIER


## Name


LIRC_SET_SEND_CARRIER - 设置用于调制 IR 发射的发送载波

## Synopsis



`int ioctl(int fd, LIRC_SET_SEND_CARRIER, __u32 *frequency)`

## Arguments


`fd`
    open() 返回的文件描述符

`frequency`
    待调制载波的频率，单位为 Hz

## Description


设置用于调制 IR PWM 脉冲与间隔的发送载波

## Return Value


成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
