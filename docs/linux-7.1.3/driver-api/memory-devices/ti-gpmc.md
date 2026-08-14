
## GPMC（General Purpose Memory Controller，通用内存控制器）


GPMC 是一个专用于连接外部存储设备（如以下各类）的统一内存控制器：

 - 异步 SRAM 类存储器以及专用集成电路（ASIC）设备。
 - 异步、同步与页模式突发（burst）NOR 闪存设备
 - NAND 闪存
 - 伪 SRAM 设备

GPMC 见于德州仪器（Texas Instruments）的 SoC（基于 OMAP）上
IP 细节：https://www.ti.com/lit/pdf/spruh73 第 7.1 节


## GPMC 通用时序计算：


GPMC 有一些必须编程的时序，才能使外设正常工作，而外设自身又有另一组时序。要让外设与 gpmc 协同工作，必须把外设时序转换为 gpmc 能理解的形式。转换方式取决于所连接的外设。此外，某些 gpmc 时序还依赖于 gpmc 时钟频率。因此开发了通用时序例程来满足上述需求。

通用例程提供了一种从 gpmc 外设时序计算 gpmc 时序的通用方法。struct gpmc_device_timings 的字段必须用连接至 gpmc 的外设数据手册中的时序更新。少数外设时序既可用时间也可用周期数给出，已提供处理此情形的机制（参见 struct gpmc_device_timings 定义）。可能会出现外设数据手册中规定的时序在时序结构中不存在的情况，此时应尝试把此外设时序关联到已有可用时序。若仍不行，可尝试根据需要为外设添加新字段，并教会通用时序例程处理它，同时确保不破坏任何已有逻辑。另外还可能存在外设数据手册未提及 struct gpmc_device_timings 某些字段的情况，此时将这些条目置零。

通用时序例程已验证可在多个 onenand 与 tusb6010 外设上正常工作。

注意事项：通用时序例程是基于对 gpmc 时序、外设时序、已有自定义时序例程的理解，在缺乏大多数数据手册与硬件（确切地说，主线中受支持的都没有自定义时序例程）的情况下，通过某种"逆向工程"加仿真开发出来的。

gpmc 时序对外设时序的依赖：

[<gpmc_timing>: <peripheral timing1>, <peripheral timing2> ...]

1. common（通用）

cs_on:
	t_ceasu
adv_on:
	t_avdasu, t_ceavd

2. sync common（同步通用）

sync_clk:
	clk
page_burst_access:
	t_bacc
clk_activation:
	t_ces, t_avds

3. read async muxed（异步复用读）

adv_rd_off:
	t_avdp_r
oe_on:
	t_oeasu, t_aavdh
access:
	t_iaa, t_oe, t_ce, t_aa
rd_cycle:
	t_rd_cycle, t_cez_r, t_oez

4. read async non-muxed（异步非复用读）

adv_rd_off:
	t_avdp_r
oe_on:
	t_oeasu
access:
	t_iaa, t_oe, t_ce, t_aa
rd_cycle:
	t_rd_cycle, t_cez_r, t_oez

5. read sync muxed（同步复用读）

adv_rd_off:
	t_avdp_r, t_avdh
oe_on:
	t_oeasu, t_ach, cyc_aavdh_oe
access:
	t_iaa, cyc_iaa, cyc_oe
rd_cycle:
	t_cez_r, t_oez, t_ce_rdyz

6. read sync non-muxed（同步非复用读）

adv_rd_off:
	t_avdp_r
oe_on:
	t_oeasu
access:
	t_iaa, cyc_iaa, cyc_oe
rd_cycle:
	t_cez_r, t_oez, t_ce_rdyz

7. write async muxed（异步复用写）

adv_wr_off:
	t_avdp_w
we_on, wr_data_mux_bus:
	t_weasu, t_aavdh, cyc_aavhd_we
we_off:
	t_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_wr_cycle

8. write async non-muxed（异步非复用写）

adv_wr_off:
	t_avdp_w
we_on, wr_data_mux_bus:
	t_weasu
we_off:
	t_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_wr_cycle

9. write sync muxed（同步复用写）

adv_wr_off:
	t_avdp_w, t_avdh
we_on, wr_data_mux_bus:
	t_weasu, t_rdyo, t_aavdh, cyc_aavhd_we
we_off:
	t_wpl, cyc_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_ce_rdyz

10. write sync non-muxed（同步非复用写）

adv_wr_off:
	t_avdp_w
we_on, wr_data_mux_bus:
	t_weasu, t_rdyo
we_off:
	t_wpl, cyc_wpl
cs_wr_off:
	t_wph
wr_cycle:
	t_cez_w, t_ce_rdyz


Note（注意）：
  许多 gpmc 时序依赖于其它 gpmc 时序（少数 gpmc 时序纯粹依赖其它 gpmc 时序，这也是上面缺失部分 gpmc 时序的原因），这将导致外设时序对除上述之外的其它 gpmc 时序产生间接依赖，更多细节参见时序例程。要了解这些外设时序对应的含义，请参见 struct gpmc_device_timings 定义中的说明。对于 gpmc 时序，请参考 IP 细节（上面链接）。
