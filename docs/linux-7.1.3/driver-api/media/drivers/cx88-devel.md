
## cx88 驱动


作者： Gerd Hoffmann

### cx88 数据手册中缺失的文档


MO_OUTPUT_FORMAT (0x310164)


  来自 DScaler 的先前默认值：0x1c1f0008
  数字 8：31-28
  28：PREVREMOD = 1

  数字 7：27-24（0xc = 12 = b1100）
  27：COMBALT = 1
  26：PAL_INV_PHASE
    （DScaler 显然将其设为 1，导致画面很差）

  数字 6、5：23-16
  25-16：COMB_RANGE = 0x1f [默认值]（9 位 -> 最大 512）

  数字 4：15-12
  15：DISIFX = 0
  14：INVCBF = 0
  13：DISADAPT = 0
  12：NARROWADAPT = 0

  数字 3：11-8
  11：FORCE2H
  10：FORCEREMD
  9：NCHROMAEN
  8：NREMODEN

  数字 2：7-4
  7-6：YCORE
  5-4：CCORE

  数字 1：3-0
  3：RANGE = 1
  2：HACTEXT
  1：HSFMT

0x47 是 MPEG-2 传输流包的同步字节。
数据手册错误地写成了使用十进制 47。188 是长度。
所有符合 DVB 规范的前端都输出带有该起始码的数据包。

### Hauppauge WinTV cx88 IR 信息


mux 的控制为：源选择使用 GPIO [0,1]，静音使用 GPIO 2。

====== ======== =================================================
GPIO0  GPIO1
====== ======== =================================================
  0        0    TV 音频
  1        0    FM 收音机
  0        1    线路输入（Line-In）
  1        1    单声道调谐器旁路或 CD 直通（取决于调谐器）
====== ======== =================================================

GPIO 16（我推测）连接到 IR 端口（如果存在）。


来自数据手册：

- 寄存器 24'h20004  PCI 中断状态

 - 位 [^18^]  IR_SMP_INT 当通过
 - gpio[^16^] 引脚向 GP_SAMPLE 寄存器收集了 32 个输入采样时置位。

数据手册中缺失的内容：

- 设置 4KHz 采样率（大约 2 倍过采样；对我们的 RC5
  兼容遥控器已足够）
- 将寄存器 0x35C050 设为 0xa80a80
- 使能采样
- 将寄存器 0x35C054 设为 0x5
- 在中断掩码寄存器中使能 IRQ 位 18（并
  提供相应的处理函数）

GP_SAMPLE 寄存器位于 0x35C058

随后位会以指定速率右移进入 GP_SAMPLE 寄存器；当接收到一个完整的 DWORD 时会触发中断。
你需要从（过采样的）IR 传感器位中恢复出实际的 RC5 位。（提示：寻找 RC5 双相数据的 0/1 与 1/0 跳变）一个
实际的原始 RC5 码将跨越 2-3 个 DWORD，具体取决于实际对齐情况。

我相当确定当没有 IR 信号时，接收器始终处于标记状态（1）；但杂散光等也会导致间歇性噪声值。
记住，这是对 IR 接收器状态随时间变化的自由运行采样，因此不要假设任何采样都从某个特定位置开始。

#### 附加信息


这份数据手册（谷歌搜索）似乎对 RC5 基础有不错的描述：
http://www.atmel.com/dyn/resources/prod_documents/doc2817.pdf

这篇文档包含更多数据：
http://www.nenya.be/beor/electronics/rc5.htm

这篇文档介绍了如何解码双相数据流：
http://www.ee.washington.edu/circuit_archive/text/ir_decode.txt

这篇文档包含更多信息：
http://www.xs4all.nl/~sbp/knowledge/ir/rc5.htm
