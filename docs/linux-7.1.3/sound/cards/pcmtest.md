
## 虚拟 PCM 测试驱动

虚拟 PCM 测试驱动模拟一个通用PCM 设备，可用于对用户ALSA 应用进行
测试/模糊测试，也可用于对 PCM 中间层进行测模糊测试。此外，它还可用模拟难以复现PCM 设备问题
#### 该驱动能做什么？

目前该驱动可以完成以下事情：
 - 模拟采集（capture）与回放（playback）过 - 生成随机或基于模式（pattern）的采集数据
 - 向回放和采集过程中注入延 - PCM 回调期间注入错误

它最多支8 个子流（substream）和 4 个通道。同时支持交错（interleaved）和
非交错（non-interleaved）访问模式
此外，该驱动可以检查回放数据流是否包含预定义的模式，该模式在对应的自测（alsa/pcmtest-test.sh）中用于检PCM 中间层的数据传输功能。另外，该驱重定义了默认RESET ioctl，自测试也会覆盖PCM API 功能
### 配置

除了通用ALSA 模块参数外，该驱动还有以下参数：

 - fill_mode (bool) - 缓冲区填充模式（见下文）
 - inject_delay (int)
 - inject_hwpars_err (bool)
 - inject_prepare_err (bool)
 - inject_trigger_err (bool)

### 采集数据生成

该驱动有两种数据生成模式：第一种（fill_mode 参数0）表示随机数据生成，
第二种（fill_mode 1）表示基于模式的数据生成。我们来看看第二种模式
首先，你可能需要指定用于数据生成的模式。可以通过把模式写debugfs 文件实现。每个通道都有对应的模式缓冲区 debugfs 项，以及包含模式缓冲区长度的
项
 - /sys/kernel/debug/pcmtest/fill_pattern[0-3]
 - /sys/kernel/debug/pcmtest/fill_pattern[0-3]_len

要为通道 0 设置模式，可以执行以下命令：


	echo -n mycoolpattern > /sys/kernel/debug/pcmtest/fill_pattern0

之后，对 'pcmtest' 设备的每次采集操作结束后，通道 0 的缓冲区都会包含
'mycoolpatternmycoolpatternmycoolpatternmy...'銆。
模式本身最长可4096 字节
### 延迟注入

该驱动有 'inject_delay' 参数，其名称非常直观，可用于模拟时间延迟/加速。该
参数为整数类型，表示在模块内部定时器节拍之间添加的延迟
如果 'inject_delay' 值为正，缓冲区填充会变慢；如果为负，则会变快。你可以
自己尝试：在任何录音应用（如 Audacity）中启动录制，并选择 'pcmtest' 设备
作为音源
该参数也可用于在一个非常短的时间段内生成大量的声音数据（使用负'inject_delay' 值）
### 错误注入

该模块可用于PCM 通信过程中注入错误。这一操作有助于你了解用户ALSA
程序在异常情况下如何表现
例如，可以通过'inject_hwpars_err' 模块参数写入 '1'，使所'hw_params'
PCM 回调返回 EBUSY 错误

	echo 1 > /sys/module/snd_pcmtest/parameters/inject_hwpars_err

可以向以PCM 回调注入错误
 - hw_params (EBUSY)
 - prepare (EINVAL)
 - trigger (EINVAL)

### 回放测试

该驱动也可用于回放功能测试——每当你'pcmtest' PCM 设备写入回放数据并关它时，驱动会检查缓冲区是否包含循环模式（该模式在每个通道fill_pattern
debugfs 文件中指定）。如果回放缓冲区内容表示循环模式，则 'pc_test' debugfs
项被设为 '1'。否则，驱动将其设为 '0'
### ioctl 重定义测
该驱动重定义了所PCM 设备默认'reset' ioctl。要测试此功能，我们可以
触发 reset ioctl 并检'ioctl_test' debugfs 项：


	cat /sys/kernel/debug/pcmtest/ioctl_test

如果 ioctl 触发成功，该文件将包'1'，否则为 '0'