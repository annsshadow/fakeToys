


## CEC Pin 框架错误注入


CEC Pin 框架是一个针对仅对 CEC 总线提供底层支持的 CEC 硬件的 CEC 核心框架。
当今大多数硬件都具备高层 CEC 支持，由硬件负责驱动 CEC 总线，但一些较老的
设备没那么高级。不过，该框架也允许您将 CEC pin 连接到例如 Raspberry Pi 的
GPIO 上，从而自制一个 CEC 适配器。

这样做之所以有趣，是因为我们完全掌控总线，因此可以轻松支持错误注入。这对于
测试 CEC 适配器处理错误状况的能力非常理想。

目前只有 cec-gpio 驱动（当 CEC 线路直接连接到上拉 GPIO 线路时）以及
AllWinner A10/A20 的 drm 驱动支持该框架。

如果启用了 `CONFIG_CEC_PIN_ERROR_INJ`，则可经由 debugfs 使用错误注入。具体地，
在 `/sys/kernel/debug/cec/cecX/` 下现在会有一个 `error-inj` 文件。


    错误注入命令不属于稳定的 ABI，未来可能会发生变化。

通过 `cat error-inj` 您可以查看可用命令以及当前
```
	$ cat /sys/kernel/debug/cec/cec0/error-inj
	# 清除错误注入：
	#   clear          清除所有 rx 和 tx 错误注入
	#   rx-clear       清除所有 rx 错误注入
	#   tx-clear       清除所有 tx 错误注入
	#   <op> clear     清除 <op> 的所有 rx 和 tx 错误注入
	#   <op> rx-clear  清除 <op> 的所有 rx 错误注入
	#   <op> tx-clear  清除 <op> 的所有 tx 错误注入
	#
	# RX 错误注入设置：
	#   rx-no-low-drive                    不生成 low-drive 脉冲
	#
	# RX 错误注入：
	#   <op>[,<mode>] rx-nack              回复 NACK 而非发送 ACK
	#   <op>[,<mode>] rx-low-drive <bit>   在该位位置强制 low-drive 状态
	#   <op>[,<mode>] rx-add-byte          向接收到的 CEC 消息添加伪字节
	#   <op>[,<mode>] rx-remove-byte       移除接收到的 CEC 消息的最后一个字节
	#    any[,<mode>] rx-arb-lost [<poll>] 生成 POLL 消息以触发仲裁丢失
	#
	# TX 错误注入设置：
	#   tx-ignore-nack-until-eom           在 EOM 之前忽略早期 NACK
	#   tx-custom-low-usecs <usecs>        定义自定义脉冲的 'low' 时间
	#   tx-custom-high-usecs <usecs>       定义自定义脉冲的 'high' 时间
	#   tx-custom-pulse                    总线空闲后发送一次自定义脉冲
	#   tx-glitch-low-usecs <usecs>        定义毛刺脉冲的 'low' 时间
	#   tx-glitch-high-usecs <usecs>       定义毛刺脉冲的 'high' 时间
	#   tx-glitch-falling-edge             在每个下降沿之后发送毛刺脉冲
	#   tx-glitch-rising-edge              在每个上升沿之后发送毛刺脉冲
	#
	# TX 错误注入：
	#   <op>[,<mode>] tx-no-eom            不设置 EOM 位
	#   <op>[,<mode>] tx-early-eom         将 EOM 位提前一个字节设置
	#   <op>[,<mode>] tx-add-bytes <num>   向消息追加 <num> (1-255) 个伪字节
	#   <op>[,<mode>] tx-remove-byte       丢弃消息的最后一个字节
	#   <op>[,<mode>] tx-short-bit <bit>   使该位短于允许值
	#   <op>[,<mode>] tx-long-bit <bit>    使该位长于允许值
	#   <op>[,<mode>] tx-custom-bit <bit>  发送自定义脉冲替代该位
	#   <op>[,<mode>] tx-short-start       发送过短的起始脉冲
	#   <op>[,<mode>] tx-long-start        发送过长的起始脉冲
	#   <op>[,<mode>] tx-custom-start      发送自定义脉冲替代起始脉冲
	#   <op>[,<mode>] tx-last-bit <bit>    在该位之后停止发送
	#   <op>[,<mode>] tx-low-drive <bit>   在该位位置强制 low-drive 状态
	#
	# <op>       CEC 消息操作码 (0-255) 或 'any'
	# <mode>     'once' (默认)、'always'、'toggle' 或 'off'
	# <bit>      CEC 消息位 (0-159)
	#            每个 'byte' 10 位：位 0-7：数据，位 8：EOM，位 9：ACK
	# <poll>     用于测试仲裁丢失的 CEC poll 消息 (0x00-0xff，默认 0x0f)
	# <usecs>    微秒 (0-10000000，默认 1000)

	clear

```
您可以使用 `echo 'cmd' >error-inj` 或 `cat cmd.txt >error-inj` 将错误注入命令
写入 `error-inj`。`cat error-inj` 的输出包含当前生效的错误命令。您可以将该输出
保存到一个文件，并在以后作为 `error-inj` 的输入使用。

### 基本语法


前导空格/制表符会被忽略。如果下一个字符是 `#` 或已到达行尾，则整行被忽略。
否则预期为一个命令。

错误注入命令分为两大类：与接收 CEC 消息相关的，以及与发送 CEC 消息相关的。
此外，还有用于清除已有错误注入命令，以及在 CEC 总线上产生自定义脉冲的命令。

大多数错误注入命令可以针对特定的 CEC 操作码执行，也可以针对所有操作码
（`any`）执行。每个命令还有一个 'mode'（模式），可以是 `off`（用于关闭一个
已有的错误注入命令）、`once`（默认，仅对下一条接收或发送的消息触发一次错误
注入）、`always`（总是触发错误注入）以及 `toggle`（对每一条发送或接收交替
开启或关闭错误注入）。

因此 '`any rx-nack`' 会对下一条接收到的 CEC 消息回复 NACK，'`any,always
rx-nack`' 会对所有接收到的 CEC 消息回复 NACK，而 '`0x82,toggle rx-nack`' 仅当
收到 Active Source 消息时才回复 NACK，且只对每隔一条接收的消息如此。

以 `once` 模式注入错误后，该错误注入命令会自动清除，因此 `once` 是一次性操作。

`<op>` 与错误注入命令的所有组合可以共存。所以
```
	0x9e tx-add-bytes 1
	0x9e tx-early-eom
	0x9f tx-add-bytes 2
	any rx-nack

```
这四条错误注入命令将同时生效。

但是，如果指定了相同的 `<op>` 与命令组合，
```
	0x9e tx-add-bytes 1
	0x9e tx-add-bytes 2

```
那么第二条会覆盖第一条。

### 清除错误注入


`clear`
    清除所有错误注入。

`rx-clear`
    清除所有接收错误注入

`tx-clear`
    清除所有发送错误注入

`<op> clear`
    清除给定操作码的所有错误注入。

`<op> rx-clear`
    清除给定操作码的所有接收错误注入。

`<op> tx-clear`
    清除给定操作码的所有发送错误注入。

### 接收消息


`<op>[,<mode>] rx-nack`
    NACK 广播消息以及定向到本 CEC 适配器的消息。如果发送方在第一个字节被
    NACK 后继续发送，则消息的每个字节都会被 NACK。

`<op>[,<mode>] rx-low-drive <bit>`
    在该位位置强制 Low Drive 状态。如果 <op> 指定了特定的 CEC 操作码，则位
    位置必须至少为 18，否则操作码尚未被接收。这用于测试发送方能否正确处理
    Low Drive 状态并正确报告错误。注意，前 4 位中的 Low Drive 也可能被发送方
    解释为 Arbitration Lost（仲裁丢失）状态。这取决于具体实现。

`<op>[,<mode>] rx-add-byte`
    向接收到的 CEC 消息添加伪 0x55 字节，前提是消息长度不超过 15 字节。这对
    测试高层协议很有用，因为伪字节应当被忽略。

`<op>[,<mode>] rx-remove-byte`
    移除接收到的 CEC 消息的最后一个字节，前提是消息长度至少为 2 字节。这对
    测试高层协议很有用，因为过短的消息应当被忽略。

`<op>[,<mode>] rx-arb-lost <poll>`
    生成 POLL 消息以触发 Arbitration Lost 状态。该命令仅允许 `<op>` 取值为
    `next` 或 `all`。一旦收到起始位，CEC 适配器将切换到发送模式并发送一条
    POLL 消息。默认该值为 0x0f，但也可以通过 `<poll>` 参数显式指定。

    该命令可用于测试远端 CEC 发送方的 Arbitration Lost 状态。当两台 CEC 适配器
    同时开始发送消息时会发生仲裁。此时前导零最多的发起方获胜，另一发送方必须
    停止发送（'Arbitration Lost'）。这非常难以测试，除非借助此错误注入命令。

    如果远端 CEC 发送方的逻辑地址为 0（'TV'），则此方法无效，因为它总是获胜。

`rx-no-low-drive`
    接收方将忽略通常会产生 Low Drive 脉冲（3.6 ms）的情况。这通常是在接收消息
    时检测到伪脉冲后进行的，它向发送方表明由于接收方发生混乱，消息必须重传。
    禁用此功能可用于测试其他 CEC 设备如何处理毛刺，因为可以确保我们不会成为
    产生 Low Drive 的一方。

### 发送消息


`tx-ignore-nack-until-eom`
    该设置改变发送 CEC 消息的行为。通常一旦接收方 NACK 某个字节，发送就会
    停止，但规范也允许发送完整消息，仅在末尾才检查 ACK 位。这并非推荐行为，
    因为让 CEC 总线不必要地繁忙毫无意义，尤其考虑到总线如此之慢。

    该设置可用于测试接收方如何处理在消息末尾之前忽略 NACK 的发送方。

`<op>[,<mode>] tx-no-eom`
    不设置 EOM 位。通常消息最后一个字节会设置 EOM（End-Of-Message，消息结束）
    位。使用该命令后，发送将直接停止，而不发送任何 EOM。这可用于测试接收方
    如何处理这种情况。通常接收方会在超时后返回 Idle（空闲）状态。

`<op>[,<mode>] tx-early-eom`
    将 EOM 位提前一个字节设置。这显然只对两个字节及以上的消息有效。EOM 位将
    设置在倒数第二个字节而非最后一个字节上。接收方在这种情况下应忽略最后一个
    字节。由于同样的原因得到的消息很可能过短，因此整条消息通常会被忽略。在
    最后一个字节发送后，接收方应处于 Idle 状态。

`<op>[,<mode>] tx-add-bytes <num>`
    向消息追加 `<num>`（1-255）个伪字节。额外字节的值为其在消息中的字节位置。
    因此如果您发送一条两字节消息（例如 Get CEC Version 消息）并追加 2 字节，
    则远端 CEC 适配器收到的完整消息为 `0x40 0x9f 0x02 0x03`。

    该命令可用于测试接收方的缓冲区溢出。例如，当它收到超过最大消息大小 16
    字节时会如何。

`<op>[,<mode>] tx-remove-byte`
    丢弃消息的最后一个字节，前提是消息长度至少为两字节。接收方应忽略过短的消息。

`<op>[,<mode>] tx-short-bit <bit>`
    使该位周期短于允许值。位位置不能是 Ack 位。如果 <op> 指定了特定的 CEC
    操作码，则位位置必须至少为 18，否则操作码尚未被接收。通常数据位的周期在
    2.05 到 2.75 毫秒之间。使用该命令后该位的周期为 1.8 毫秒，这是通过减少
    CEC 总线为高电平的时间实现的。该位周期小于允许值，接收方应以 Low Drive
    状态响应。

    该命令对位位置 0 到 3 的 0 位会被忽略。这是因为接收方也会在前四位中寻找
    Arbitration Lost 状态，如果它看到一个过短的 0 位，结果将是不确定的。

`<op>[,<mode>] tx-long-bit <bit>`
    使该位周期长于有效值。位位置不能是 Ack 位。如果 <op> 指定了特定的 CEC
    操作码，则位位置必须至少为 18，否则操作码尚未被接收。通常数据位的周期在
    2.05 到 2.75 毫秒之间。使用该命令后该位的周期为 2.9 毫秒，这是通过增加
    CEC 总线为高电平的时间实现的。

    尽管该位周期长于有效值，但接收方会做什么并不确定。它可能只是接受它，也可能
    超时并返回 Idle 状态。遗憾的是 CEC 规范对此保持沉默。

    该命令对位位置 0 到 3 的 0 位会被忽略。这是因为接收方也会在前四位中寻找
    Arbitration Lost 状态，如果它看到一个过长的 0 位，结果将是不确定的。

`<op>[,<mode>] tx-short-start`
    使起始位周期短于允许值。通常起始位的周期在 4.3 到 4.7 毫秒之间。使用该命令
    后起始位的周期为 4.1 毫秒，这是通过减少 CEC 总线为高电平的时间实现的。该
    起始位周期小于允许值，接收方在检测到时应返回 Idle 状态。

`<op>[,<mode>] tx-long-start`
    使起始位周期长于有效值。通常起始位的周期在 4.3 到 4.7 毫秒之间。使用该命令
    后起始位的周期为 5 毫秒，这是通过增加 CEC 总线为高电平的时间实现的。该
    起始位周期大于有效值，接收方在检测到时应返回 Idle 状态。

    尽管该起始位周期长于有效值，但接收方会做什么并不确定。它可能只是接受它，
    也可能超时并返回 Idle 状态。遗憾的是 CEC 规范对此保持沉默。

`<op>[,<mode>] tx-last-bit <bit>`
    仅在该位之后停止发送。如果 <op> 指定了特定的 CEC 操作码，则位位置必须至少
    为 18，否则操作码尚未被接收。该命令可用于测试当消息突然停止时接收方的
    反应。它应当超时并返回 Idle 状态。

`<op>[,<mode>] tx-low-drive <bit>`
    在该位位置强制 Low Drive 状态。如果 <op> 指定了特定的 CEC 操作码，则位位置
    必须至少为 18，否则操作码尚未被接收。这可用于测试接收方如何处理 Low Drive
    状态。注意，如果这发生在位位置 0-3，接收方可能将其解释为 Arbitration Lost
    状态。这取决于具体实现。

### 自定义脉冲


`tx-custom-low-usecs <usecs>`
    定义自定义脉冲将 CEC 线路拉低的持续时间（微秒）。默认为 1000 微秒。

`tx-custom-high-usecs <usecs>`
    定义自定义脉冲保持 CEC 线路为高电平的持续时间（微秒，除非另一 CEC 适配器
    在该时间内将其拉低）。默认为 1000 微秒。自定义脉冲的总周期为
    `tx-custom-low-usecs + tx-custom-high-usecs`。

`<op>[,<mode>] tx-custom-bit <bit>`
    发送自定义位而非常规数据位。位位置不能是 Ack 位。如果 <op> 指定了特定的
    CEC 操作码，则位位置必须至少为 18，否则操作码尚未被接收。

`<op>[,<mode>] tx-custom-start`
    发送自定义位而非常规起始位。

`tx-custom-pulse`
    CEC 总线空闲后立即发送一次自定义脉冲。

### 毛刺脉冲


这模拟 CEC 线路上出现伪脉冲时的情况。通常这发生在下降沿或上升沿之后，此时
存在短暂的电压波动，如果 CEC 硬件不做去毛刺处理，就可能被视为伪脉冲，并可能
引发 Low Drive 状态或破坏数据。

`tx-glitch-low-usecs <usecs>`
    定义毛刺脉冲将 CEC 线路拉低的持续时间（微秒）。默认为 1 微秒。范围为
    0-100 微秒。若为 0，则不产生毛刺脉冲。

`tx-glitch-high-usecs <usecs>`
    定义毛刺脉冲保持 CEC 线路为高电平的持续时间（微秒，除非另一 CEC 适配器在该
    时间内将其拉低）。默认为 1 微秒。范围为 0-100 微秒。若为 0，则不产生毛刺
    脉冲。毛刺脉冲的总周期为 `tx-custom-low-usecs + tx-custom-high-usecs`。

`tx-glitch-falling-edge`
    在下降沿之后立即发送毛刺脉冲。

`tx-glitch-rising-edge`
    在上升沿之后立即发送毛刺脉冲。
