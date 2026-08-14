## DMA 娴嬭瘯 Guide


Andy Shevchenko <andriy.shevchenko@linux.intel.com>

small 鏂囨。 introduces 娴嬭瘯 DMA 椹卞姩 浣跨敤 dmatest 妯″潡.

dmatest 妯″潡 娴嬭瘯 DMA memcpy, memset, XOR RAID6 P+Q 鎿嶄綔 浣跨敤
various lengths various offsets source destination buffers.
鍒濆鍖?buffers repeatable pattern verify DMA
engine copies requested region nothing more. verify
bytes aren't swapped around, source 缂撳啿鍖?isn't modified.

dmatest 妯″潡 configured 娴嬭瘯 specific channel.
娴嬭瘯 multiple channels same time, start multiple 绾跨▼
competing same channel.

娴嬭瘯 suite works channels least one
capability following: DMA_MEMCPY (鍐呭瓨- -鍐呭瓨), DMA_MEMSET
(const- -鍐呭瓨 鍐呭瓨- -鍐呭瓨, emulated), DMA_XOR, DMA_PQ.

case related questions 浣跨敤 official mailing 鍒楄〃
dmaengine@vger.鍐呮牳.org.

## Part 1 - 鏋勫缓 娴嬭瘯 妯″潡


menuconfig 鍖呭惈 閫夐」 could found following path:

璁惧 椹卞姩 -> DMA Engine 鏀寔 -> DMA 娴嬭瘯 client

閰嶇疆 鏂囦欢 閫夐」 called CONFIG_DMATEST. dmatest could
built 妯″潡 inside 鍐呮牳. Let's consider cases.

## Part 2 - dmatest built 妯″潡


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
娴嬭瘯, starting 5.0 鍐呮牳, single- multi-channel,
channel 鍙傛暟(s) 璁剧疆 鍙傛暟.
time existing 鍙傛暟 鍊?acquired 浣跨敤
绾跨▼(s). 鍙傛暟 shared. Therefore, changes made
鍙傛暟, additional channel specified,
(shared) 鍙傛暟 浣跨敤 绾跨▼ 浣跨敤 new 鍊?
channels specified, 绾跨▼ 璁剧疆 pending. 绾跨▼
begin execution 杩愯 鍙傛暟 璁剧疆 1.

```

    % ls -1 /sys/class/dma/

```
Once started message like " dmatest: Added 1 绾跨▼ 浣跨敤 dma0chan0"
emitted. 绾跨▼ specific channel created now pending,
pending 绾跨▼ started once 杩愯 1.

璇存槑 杩愯 new 娴嬭瘯 stop progress 娴嬭瘯.

```

    % cat /sys/module/dmatest/parameters/run

```
wait 娴嬭瘯 completion userspace poll '杩愯' until false, 浣跨敤
wait 鍙傛暟. Specifying 'wait=1' loading 妯″潡 causes 妯″潡
鍒濆鍖?pause until 娴嬭瘯 杩愯 completed, reading
/sys/妯″潡/dmatest/鍙傛暟/wait waits 杩愯 娴嬭瘯 complete
returning. 绀轰緥, following scripts wait 42 娴嬭瘯
complete exiting. 璇存槑 'iterations' 璁剧疆 'infinite'
waiting 绂佺敤.

```

    % modprobe dmatest run=1 iterations=42 wait=1
    % modprobe -r dmatest

```
```

    % modprobe dmatest run=1 iterations=42
    % cat /sys/module/dmatest/parameters/wait
    % modprobe -r dmatest

```
## Part 3 - built- 鍐呮牳


妯″潡 鍙傛暟 supplied 鍐呮牳 鍛戒护 line 浣跨敤
first performed 娴嬭瘯. 鐢ㄦ埛 gets 鎺у埗, 娴嬭瘯 could
re-杩愯 same different 鍙傛暟. details 鍙傝
绔犺妭`Part 2 - When dmatest is built as a module`_.

cases 妯″潡 鍙傛暟 浣跨敤 actual 鍊?娴嬭瘯
```

    % grep -H . /sys/module/dmatest/parameters/*

```
## Part 4 - Gathering 娴嬭瘯 results


```

    "dmatest: result <channel>: <test id>: '<error msg>' with src_off=<val> dst_off=<val> len=<val> (<err code>)"

```
```

    % dmesg | tail -n 1
    dmatest: result dma0chan0-copy0: #1: No errors with src_off=0x7bf dst_off=0x8ad len=0x3fea (0)

```
message format unified across different 绫诲瀷 閿欒.
缂栧彿 parentheses represents additional 淇℃伅, e.g. 閿欒
code, 閿欒 counter, 鐘舵€? 娴嬭瘯 绾跨▼ emits 鎽樿 line
completion listing 缂栧彿 娴嬭瘯 executed, 缂栧彿 failed,
result code.

```

    % dmesg | tail -n 1
    dmatest: dma0chan0-copy0: summary 1 test, 0 failures 1000 iops 100000 KB/s (0)

```
details 鏁版嵁 miscompare 閿欒 emitted, follow
format.

## Part 5 - Handling channel 鍒嗛厤


### Allocating Channels


Channels need configured prior starting 娴嬭瘯 杩愯. Attempting
杩愯 娴嬭瘯 configuring channels result testing
channels 鍙敤.

```

    % echo 1 > /sys/module/dmatest/parameters/run
    dmatest: No channels configured, continue with any

```
Channels registered 浣跨敤 "channel" 鍙傛暟. Channels requested
鍚嶇О, once requested, channel registered pending 绾跨▼ added 娴嬭瘯 鍒楄〃.

```

    % echo dma0chan2 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan2

```
More channels added repeating 绀轰緥 .
Reading back channel 鍙傛暟 杩斿洖 鍚嶇О last channel added successfully.

```

    % echo dma0chan1 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan1
    % echo dma0chan2 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan2
    % cat /sys/module/dmatest/parameters/channel
    dma0chan2

```
method requesting channels 璇锋眰 channel empty string, Doing
璇锋眰 channels 鍙敤 tested:

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
point 娴嬭瘯 閰嶇疆, reading "test_list" 鍙傛暟
print 鍒楄〃 currently pending 娴嬭瘯.

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
璇存槑: Channels configured 娴嬭瘯 杩愯 channel configurations
carry across next 娴嬭瘯 杩愯.

### Releasing Channels


Channels freed setting 杩愯 0.

```

    % echo dma0chan1 > /sys/module/dmatest/parameters/channel
    dmatest: Added 1 threads using dma0chan1
    % cat /sys/class/dma/dma0chan1/in_use
    1
    % echo 0 > /sys/module/dmatest/parameters/run
    % cat /sys/class/dma/dma0chan1/in_use
    0

```
Channels allocated previous 娴嬭瘯 runs automatically freed new
channel requested completing successful 娴嬭瘯 杩愯.
