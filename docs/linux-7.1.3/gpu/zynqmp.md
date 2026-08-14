## Xilinx ZynqMP Ultrascale+ DisplayPort 子系统


该子系统负责 ZynqMP 上的 DisplayPort 视频与音频输出。它支持使用 DisplayPort DMA
控制器（xilinx-dpdma）的片内帧缓冲，以及来自可编程逻辑（PL）的“实时”视频与音频。
该子系统可执行多种变换，包括色彩空间转换、alpha 混合与音频混音，尽管目前并非所有
特性都受支持。

### debugfs


为支持调试与一致性测试，可通过 debugfs 启用若干测试模式。/sys/kernel/debug/dri/X/DP-1/test/
下的以下文件用于控制 DisplayPort 测试模式：

active：
        向该文件写入 1 将激活测试模式，写入 0 将停用测试模式。在测试模式已激活/已停用时
        写入 1 或 0 将重新激活/重新停用测试模式。当测试模式未激活时，对其他文件所作的更改
        不会（立即）生效，但这些设置会被保存，待测试模式激活时生效。当测试模式激活时，
        对其他文件所作的更改会立即生效。

custom：
        自定义测试图案值

downspread：
        通过写入 1/0 来启用/禁用时钟扩频（spread-spectrum clocking）

enhanced：
        启用/禁用增强帧

ignore_aux_errors：
        设为 1 时忽略 AUX 错误。对该文件的写入会立即生效（无论测试模式是否激活），
        并影响所有 AUX 传输。

ignore_hpd：
        设为 1 时忽略热插拔事件（例如线缆拔除或显示器链路重训练请求）。

laneX_preemphasis：
        lane X 的预加重，从 0（最低）到 2（最高）

laneX_swing：
        lane X 的电压摆幅，从 0（最低）到 3（最高）

lanes：
        要使用的通道数（1、2 或 4）

pattern：
        测试图案。可以是以下之一：

                video
                        使用常规视频输入

                symbol-error
                        符号错误测量图案

                prbs7
                        PRBS7（x^7 + x^6 + 1）多项式的输出

                80bit-custom
                        自定义的 80 位图案

                cp2520
                        HBR2 一致性眼图图案

                tps1
                        链路训练符号图案 TPS1（/D10.2/）

                tps2
                        链路训练符号图案 TPS2

                tps3
                        链路训练符号图案 TPS3（用于 HBR2）

rate：
        速率（单位赫兹）。为以下之一：

                - 5400000000 (HBR2)
                - 2700000000 (HBR)
                - 1620000000 (RBR)

```

        for prop in /sys/kernel/debug/dri/1/DP-1/test/*; do
                printf '%-17s ' ${prop##*/}
                if [ ${prop##*/} = custom ]; then
                        hexdump -C $prop | head -1
                else
                        cat $prop
                fi
        done

```

```

        active            1
        custom            00000000  00 00 00 00 00 00 00 00  00 00                    |..........|
        downspread        0
        enhanced          1
        ignore_aux_errors 1
        ignore_hpd        1
        lane0_preemphasis 0
        lane0_swing       3
        lane1_preemphasis 0
        lane1_swing       3
        lanes             2
        pattern           prbs7
        rate              1620000000

```
推荐的测试流程是：将开发板连接到显示器，配置测试模式，激活测试模式，然后拔下线缆并
连接到你选择的测试设备。例如，可以

```

        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/enhanced
        echo tps1 > /sys/kernel/debug/dri/1/DP-1/test/pattern
        echo 1620000000 > /sys/kernel/debug/dri/1/DP-1/test/rate
        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/ignore_aux_errors
        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/ignore_hpd
        echo 1 > /sys/kernel/debug/dri/1/DP-1/test/active

```
此时即可将线缆从显示器上拔下。

### 内部实现





