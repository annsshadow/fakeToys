


######## ioctl FE_DISEQC_SEND_BURST


## 名称


FE_DISEQC_SEND_BURST - 2x1 mini DiSEqC 卫星选择发22KHz 音调突发
## 概要



`int ioctl(int fd, FE_DISEQC_SEND_BURST, enum fe_sec_mini_cmd tone)`

## 参数


`fd`
    `open()` 返回的文件描述符
`tone`
    `fe_sec_mini_cmd` 中描述的整数枚举值
## 描述


ioctl 用于2x1 开关的 mini DiSEqC 卫星选择设置 22kHz 音调突发生成。此调用需要读/写权限
瀹冩敮鎸?`Digital Satellite Equipment Control (DiSEqC) - Simple "ToneBurst" Detection Circuit specification. <http://www.eutelsat.com/files/contributed/satellites/pdf/Diseqc/associated%20docs/simple_tone_burst_detec.pdf>`__
中规定的内容
## 杩斿洖鍊。

成功时返0
出错时返-1，并相应地设`errno` 变量
通用错误码的描述见通用错误<gen-errors> 章节