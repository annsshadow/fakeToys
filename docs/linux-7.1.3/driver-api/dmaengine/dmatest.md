## DMA 测试 Guide


Andy Shevchenko <andriy.shevchenko@linux.intel.com>

small 文档 introduces 测试 DMA 驱动 使用 dmatest 模块.

dmatest 模块 测试 DMA memcpy, memset, XOR RAID6 P+Q 操作 使用
various lengths various offsets source destination buffers.
初始buffers repeatable pattern verify DMA
engine copies requested region nothing more. verify
bytes aren't swapped around, source 缓冲isn't modified.

dmatest 模块 configured 测试 specific channel.
测试 multiple channels same time, start multiple 线程
competing same channel.

测试 suite works channels least one
capability following: DMA_MEMCPY (内存- -内存), DMA_MEMSET
(const- -内存 内存- -内存, emulated), DMA_XOR, DMA_PQ.

case related questions 使用 official mailing 列表
dmaengine@vger.内核.org.

## Part 1 - 构建 测试 模块


menuconfig 包含 选项 could found following path:

设备 驱动 -> DMA Engine 支持 -> DMA 测试 client

配置 文件 选项 called CONFIG_DMATEST. dmatest could
built 模块 inside 内核. Let's consider cases.

## Part 2 - dmatest built 模块


```

    % modprobe dmatest timeout=2000 iterations=1 channel=dma0chan0 run=1

```
```

    % modprobe dmatest
    % echo 2000 > /sys/module/dmatest/parameters/timeout
    % echo 1 > /sys/module/dmatest/parameters/iterations
    % echo dma0chan0 > /sys/module/dmatest/parameters/channel
    % echo 1 > /sys/module/dmatest/parameters/run

```
```

    dmatest.timeout=2000 dmatest.iterations=1 dmatest.channel=dma0chan0 dmatest.run=1

```
```

    % modprobe dmatest
    % echo 2000 > /sys/module/dmatest/parameters/timeout
    % echo 1 > /sys/module/dmatest/parameters/iterations
    % echo dma0chan0 > /sys/module/dmatest/parameters/channel
    % echo dma0chan1 > /sys/module/dmatest/parameters/channel
    % echo dma0chan2 > /sys/module/dmatest/parameters/channel
    % echo 1 > /sys/module/dmatest/parameters/run

```
测试, starting 5.0 内核, single- multi-channel,
channel 参数(s) 设置 参数.
time existing 参数 acquired 使用
线程(s). 参数 shared. Therefore, changes made
参数, additional channel specified,
(shared) 参数 使用 线程 使用 new 
channels specified, 线程 设置 pending. 线程
begin execution 运行 参数 设置 1.

```

    % ls -1 /sys/class/dma/

```
Once started message like " dmatest: Added 1 线程 使用 dma0chan0"
emitted. 线程 specific channel created now pending,
pending 线程 started once 运行 1.

说明 运行 new 测试 stop progress 测试.

```

    % cat /sys/module/dmatest/parameters/run

```
wait 测试 completion userspace poll '运行' until false, 使用
wait 参数. Specifying 'wait=1' loading 模块 causes 模块
初始pause until 测试 运行 completed, reading
/sys/模块/dmatest/参数/wait waits 运行 测试 complete
returning. 示例, following scripts wait 42 测试
complete exiting. 说明 'iterations' 设置 'infinite'
waiting 禁用.

```

    % modprobe dmatest run=1 iterations=42 wait=1
    % modprobe -r dmatest

```
```

    % modprobe dmatest run=1 iterations=42
    % cat /sys/module/dmatest/parameters/wait
    % modprobe -r dmatest

```
## Part 3 - built- 内核


模块 参数 supplied 内核 命令 line 使用
first performed 测试. 用户 gets 控制, 测试 could
re-运行 same different 参数. details 参见
章节`Part 2 - When dmatest is built as a module`_.

cases 模块 参数 使用 actual 测试
```

    % grep -H . /sys/module/dmatest/parameters/*

```
## Part 4 - Gathering 测试 results


```

    "dmatest: result <channel>: <test id>: '<error msg>' with src_off=<val> dst_off=<val> len=<val> (<err code>)"

```
```

    % dmesg | tail -n 1
    dmatest: result dma0chan0-copy0: #1: No errors with src_off=0x7bf dst_off=0x8ad len=0x3fea (0)

```
message format unified across different 类型 错误.
编号 parentheses represents additional 信息, e.g. 错误
code, 错误 counter, 状 测试 线程 emits 摘要 line
completion listing 编号 测试 executed, 编号 failed,
result code.

```

    % dmesg | tail -n 1
    dmatest: dma0chan0-copy0: summary 1 test, 0 failures 1000 iops 100000 KB/s (0)

```
details 数据 miscompare 错误 emitted, follow
format.

## Part 5 - Handling channel 分配


### Allocating Channels


Channels need configured prior starting 测试 运行. Attempting
运行 测试 configuring channels result testing
channels 可用.

```

    % echo 1 > /sys/module/dmatest/parameters/run
    dmatest: No channels configured, continue with any

```
Channels registered 使用 "channel" 参数. Channels requested
名称, once requested, channel registered pending 线程 added 测试 列表.

```

    % echo dma0chan2 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan2

```
More channels added repeating 示例 .
Reading back channel 参数 返回 名称 last channel added successfully.

```

    % echo dma0chan1 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan1
    % echo dma0chan2 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan2
    % cat /sys/module/dmatest/parameters/channel
    dma0chan2

```
method requesting channels 请求 channel empty string, Doing
请求 channels 可用 tested:

```

    % echo "" > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan0
    dmatest: Added 1 threads using dma0chan3
    dmatest: Added 1 threads using dma0chan4
    dmatest: Added 1 threads using dma0chan5
    dmatest: Added 1 threads using dma0chan6
    dmatest: Added 1 threads using dma0chan7
    dmatest: Added 1 threads using dma0chan8

```
point 测试 配置, reading "test_list" 参数
print 列表 currently pending 测试.

```

    % cat /sys/module/dmatest/parameters/test_list
    dmatest: 1 threads using dma0chan0
    dmatest: 1 threads using dma0chan3
    dmatest: 1 threads using dma0chan4
    dmatest: 1 threads using dma0chan5
    dmatest: 1 threads using dma0chan6
    dmatest: 1 threads using dma0chan7
    dmatest: 1 threads using dma0chan8

```
说明: Channels configured 测试 运行 channel configurations
carry across next 测试 运行.

### Releasing Channels


Channels freed setting 运行 0.

```

    % echo dma0chan1 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan1
    % cat /sys/class/dma/dma0chan1/in_use
    1
    % echo 0 > /sys/module/dmatest/parameters/run
    % cat /sys/class/dma/dma0chan1/in_use
    0

```
Channels allocated previous 测试 runs automatically freed new
channel requested completing successful 测试 运行.
