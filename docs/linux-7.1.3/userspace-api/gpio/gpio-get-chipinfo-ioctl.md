######## GPIO_GET_CHIPINFO_IOCTL


## 名称


GPIO_GET_CHIPINFO_IOCTL - 获取芯片公开可用的信息

## 摘要



`int ioctl(int chip_fd, GPIO_GET_CHIPINFO_IOCTL, struct gpiochip_info *info)`

## 参数


`chip_fd`
    GPIO 字符设备`open()` 返回的文件描述符

`info`
    待填充的 `chip_info<gpiochip_info>`

## 说明


获取特定 GPIO 芯片公开可用的信息

## 杩斿洖鍊。


成功时返0 并填`info` 芯片信息

出错时返-1，并相应地设`errno` 变量
常见错误码在 error-codes.rst 中描述
