


######## ioctl FE_DISEQC_SEND_BURST


## 名称


FE_DISEQC_SEND_BURST - 为 2x1 mini DiSEqC 卫星选择发送 22KHz 音调突发。

## 概要



`int ioctl(int fd, FE_DISEQC_SEND_BURST, enum fe_sec_mini_cmd tone)`

## 参数


`fd`
    由 `open()` 返回的文件描述符。

`tone`
    在 `fe_sec_mini_cmd` 中描述的整数枚举值。

## 描述


此 ioctl 用于为 2x1 开关的 mini DiSEqC 卫星选择设置 22kHz 音调突发的
生成。此调用需要读/写权限。

它支持 `Digital Satellite Equipment Control (DiSEqC) - Simple "ToneBurst" Detection Circuit specification. <http://www.eutelsat.com/files/contributed/satellites/pdf/Diseqc/associated%20docs/simple_tone_burst_detec.pdf>`__
中规定的内容。

## 返回值


成功时返回 0。

出错时返回 -1，并相应地设置 `errno` 变量。

通用错误码的描述见通用错误码 <gen-errors> 章节。
