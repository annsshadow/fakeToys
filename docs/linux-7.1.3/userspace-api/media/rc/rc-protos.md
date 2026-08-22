

######## 遥控协议与扫描码


IR 使用某种协议，被编码为一系列脉冲和间隔。这些协议可以编码例如地址（应由哪个设响应）和命令：它应当做什么。对于给定协议，这些值在不同设备之间并不总是保持一致
因此，IR 解码器的输出是一个扫描码（scancode）；一个单独的 u32 值。使用键映射表，
可以将其映射Linux 按键码
也可以编码其他内容。某IR 协议会编码一个翻转位（toggle bit）；这是用来区分同一
按钮是被按住，还是被松开后再次按下。如果是被松开后再次按下，翻转位会从前一IR
消息到下一条消息发生反转
某些遥控器有一个指针型设备，可用于控制鼠标；某些空调系统可以通过 IR 设定目标
温度
以下是内核所知晓的协议，并列出每种协议的扫描码是如何编码的
### rc-5 (RC_PROTO_RC5)


IR 协议使用曼彻斯特（manchester）编码来编码 14 位。这里有详细描述
https://www.sbprojects.net/knowledge/ir/rc5.php銆。
扫描码编码与 lirc 守护进程（lircd）的 rc5 协议，或 manchester BPF 解码**
一致
   :widths:       1 1 2

   - - rc-5 浣。
     - 扫描码位

     - 描述

   - - 1

     - 鏃。
     - 起始位，始终置位

   - - 1

     - 6（取反）

     - rc5 中的第二个起始位，被复用为第 6 个命令位

   - - 1

     - 鏃。
     - 翻转
   - - 5

     - 8 鍒?13

     - 地址

   - - 6

     - 0 鍒?5

     - 命令

rc5 有一个称rc5x extended rc5 的变体，其中第二个停止位是第 6 个命令位，但
被取反。这样做是为了使扫描码和编码与现有方案兼容。该位以取反形式存储在扫描码6 位。这样做是为了使其与具有两个起始位的普rc-5 保持兼容
### rc-5-sz (RC_PROTO_RC5_SZ)

这与 rc-5 很相似，但多一位。扫描码采用不同的编码方式
   :widths:       1 1 2

   - - rc-5-sz 浣。
     - 扫描码位

     - 描述

   - - 1

     - 鏃。
     - 起始位，始终置位

   - - 1

     - 13

     - 地址
   - - 1

     - 鏃。
     - 翻转
   - - 6

     - 6 鍒?11

     - 地址

   - - 6

     - 0 鍒?5

     - 命令

### rc-5x-20 (RC_PROTO_RC5X_20)


这是扩展为编20 位的 rc-5。在8 位之后有一3555 微秒的间隔
   :widths:       1 1 2

   - - rc-5-sz 浣。
     - 扫描码位

     - 描述

   - - 1

     - 鏃。
     - 起始位，始终置位

   - - 1

     - 14

     - 地址
   - - 1

     - 鏃。
     - 翻转
   - - 5

     - 16 鍒?20

     - 地址

   - - 6

     - 8 鍒?13

     - 地址

   - - 6

     - 0 鍒?5

     - 命令


### jvc (RC_PROTO_JVC)


jvc 协议nec 很相似，但没有取反的值。它在此处有描述
https://www.sbprojects.net/knowledge/ir/jvc.php銆。
扫描码是一16 位的值，其中地址是低 8 位，命令是高 8 位；这与 IR 中的顺序相反
### sony-12 (RC_PROTO_SONY12)


sony 协议是一种脉冲宽度编码。有三种变体，仅在位数和扫描码编码上有所不同
   :widths:       1 1 2

   - - sony-12 浣。
     - 扫描码位

     - 描述

   - - 5

     - 16 鍒?20

     - 设备

   - - 7

     - 0 鍒?6

     - 功能

### sony-15 (RC_PROTO_SONY15)


sony 协议是一种脉冲宽度编码。有三种变体，仅在位数和扫描码编码上有所不同
   :widths:       1 1 2

   - - sony-12 浣。
     - 扫描码位

     - 描述

   - - 8

     - 16 鍒?23

     - 设备

   - - 7

     - 0 鍒?6

     - 功能

### sony-20 (RC_PROTO_SONY20)


sony 协议是一种脉冲宽度编码。有三种变体，仅在位数和扫描码编码上有所不同
   :widths:       1 1 2

   - - sony-20 浣。
     - 扫描码位

     - 描述

   - - 5

     - 16 鍒?20

     - 设备

   - - 7

     - 0 鍒?7

     - 设备

   - - 8

     - 8 鍒?15

     - 扩展
### nec (RC_PROTO_NEC)


nec 协议编码一8 位地址和一8 位命令。它在此处有描述
https://www.sbprojects.net/knowledge/ir/nec.php。注意该协议先发送最低有效位
作为校验，nec 协议将地址和命令发送两次；第二次发送时取反。这样做是为了验证
一条普通的 nec IR 消息16 位；8 位是地址，低 8 位是命令
### nec-x (RC_PROTO_NECX)


扩展 nec 有一16 位地址和一8 位命令。它被编码为一24 位的值（如你所料）8 位是命令，高 16 位是地址
### nec-32 (RC_PROTO_NEC32)


nec-32 不发送取反的地址或取反的命令；整个消息（全部 32 位）都被使用
为了能被正确解码，第二个 8 位不得是第一8 位的取反值，并且最后一8 位也不得
是第三个 8 位值的取反值
扫描码有一种不太寻常的编码方式

   - - nec-32 浣。
     - 扫描码位

   - - 鍓?8 浣。
     - 16 鍒?23

   - - 第二8 
     - 24 鍒?31

   - - 第三8 
     - 0 鍒?7

   - - 第四8 
     - 8 鍒?15

### sanyo (RC_PROTO_SANYO)


sanyo 协议nec 协议类似，但地址13 位而非 8 位。地址和命令之后都跟着各自的取版本，但这些取反版本并不出现在扫描码中
扫描码的8 20 位是 13 位地址，低 8 位是命令
### mcir2-kbd (RC_PROTO_MCIR2_KBD)


该协议由 Microsoft MCE 键盘为键盘事件生成。请参ir-mce_kbd-decoder.c 了解编码方式
### mcir2-mse (RC_PROTO_MCIR2_MSE)


该协议由 Microsoft MCE 键盘为指针事件生成。请参ir-mce_kbd-decoder.c 了解编码方式
### rc-6-0 (RC_PROTO_RC6_0)


这是模式 0 下的 rc-6。rc-6 在此处有描述
https://www.sbprojects.net/knowledge/ir/rc6.php扫描码就是协议中精确16 位。它还有一个翻转位
### rc-6-6a-20 (RC_PROTO_RC6_6A_20)


这是模式 6a 下的 rc-60 位。rc-6 在此处有描述
https://www.sbprojects.net/knowledge/ir/rc6.php扫描码就是协议中精确20 位。它还有一个翻转位
### rc-6-6a-24 (RC_PROTO_RC6_6A_24)


这是模式 6a 下的 rc-64 位。rc-6 在此处有描述
https://www.sbprojects.net/knowledge/ir/rc6.php扫描码就是协议中精确24 位。它还有一个翻转位
### rc-6-6a-32 (RC_PROTO_RC6_6A_32)


这是模式 6a 下的 rc-62 位。rc-6 在此处有描述
https://www.sbprojects.net/knowledge/ir/rc6.php16 位是厂商，低 16 位是厂商特定位。该协议用于Microsoft MCE 变体
（vendor != 0x800f）

### rc-6-mce (RC_PROTO_RC6_MCE)


这是模式 6a 下的 rc-62 位。高 16 位是厂商，低 16 位是厂商特定位。该协议用于
Microsoft MCE 变体（vendor = 0x800f）。协议本身的翻转位被忽略，应当把16 作为翻转位
### sharp (RC_PROTO_SHARP)


这是Sharp 录像机使用的一个协议，在此处有描述
https://www.sbprojects.net/knowledge/ir/sharp.php。在正常值和取反值之间有一个很长的
0ms）间隔，某些 IR 接收器无法解码
它有一5 位地址和一8 位命令。在扫描码中，地址位于8 12 位，命令位于
0 7 位
### xmp (RC_PROTO_XMP)


该协议有多个版本，仅支持版本 1。请参考解码器（ir-xmp-decoder.c）了解其编码方式

### cec (RC_PROTO_CEC)


这不是一IR 协议，而是一个基CEC 的协议。CEC 基础设施使用 rc-core 来处CEC
命令，以便它们可以轻松地被重映射
### imon (RC_PROTO_IMON)


该协议由 Antec Veris/SoundGraph iMON 遥控器使用
该协议同时描述按键按下和指针移动。它编码 31 位，扫描码就是这 31 位，最高位始终
涓?0銆。
### rc-mm-12 (RC_PROTO_RCMM12)


rc-mm 协议在此处有描述
https://www.sbprojects.net/knowledge/ir/rcmm.php。扫描码就是12 位
### rc-mm-24 (RC_PROTO_RCMM24)


rc-mm 协议在此处有描述
https://www.sbprojects.net/knowledge/ir/rcmm.php。扫描码就是24 位
### rc-mm-32 (RC_PROTO_RCMM32)


rc-mm 协议在此处有描述
https://www.sbprojects.net/knowledge/ir/rcmm.php。扫描码就是32 位
### xbox-dvd (RC_PROTO_XBOX_DVD)


该协议由为初Xbox 制造的 Xbox DVD 遥控器使用。内核中没有该协议的解码器或编码器usb 设备解码该协议。v4l-utils 中提供了一BPF 解码器