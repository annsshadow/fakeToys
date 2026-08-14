
## Amlogic SoC DDR 带宽性能监控单元（PMU）


Amlogic Meson G12 SoC 在 DRAM 控制器内部包含一个带宽监视器。该监视器包含 4 个通道。每个通道可以统计访问 DRAM 的请求。该通道可以同时统计最多 3 个 AXI 端口。它有助于显示性能瓶颈是否出现在 DDR 带宽上。

目前，该驱动支持以下 5 个 perf 事件：

| meson_ddr_bw/total_rw_bytes/ |
| --- |
| meson_ddr_bw/chan_2_rw_bytes/ |

meson_ddr_bw/chan_{1,2,3,4}_rw_bytes/ 事件是与通道相关的事件。每个通道支持过滤，可以让通道监控 SoC 中单独的 IP 模块。

以下是 DDR 访问请求事件过滤关键字：

| arm             - 来自 CPU |
| --- |
| gpu             - 来自 3D GPU |
| hdcp            - 来自 HDCP 控制器 |
| usb3_0          - 来自 USB3.0 控制器 |
| h265enc         - 来自 HEVC 编码器 |
| vpu_write1      - 来自 VDIN 写 |
| vdec            - 来自传统编解码器视频解码器 |
| ge2d            - 来自 ge2d |
| usb0            - 来自 USB2.0 控制器 0 |
| arb0            - 来自 arb0 |
| usb1            - 来自 USB2.0 控制器 1 |
| sd_emmc_c       - 来自 SD eMMC c 控制器 |

示例：

  - 显示每秒的总 DDR 带宽：

    .. code-block:: bash

       perf stat -a -e meson_ddr_bw/total_rw_bytes/ -I 1000 sleep 10

  - 分别显示来自 CPU 和 GPU 的独立 DDR 带宽，以及它们的总和：

    .. code-block:: bash

       perf stat -a -e meson_ddr_bw/chan_1_rw_bytes,arm=1/ -I 1000 sleep 10
       perf stat -a -e meson_ddr_bw/chan_2_rw_bytes,gpu=1/ -I 1000 sleep 10
       perf stat -a -e meson_ddr_bw/chan_3_rw_bytes,arm=1,gpu=1/ -I 1000 sleep 10
