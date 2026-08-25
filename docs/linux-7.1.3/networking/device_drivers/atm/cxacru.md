
## ATM cxacru 设备驱动


该设备需要固件：http://accessrunner.sourceforge.net/

虽然设备能够在未加载模块的情况下管理/维护 ADSL 连接，但有时在卸载驱动后设备会停止响应，此时必须拔下设备电源或断电以修复该问题
注意：对 cxacru-cf.bin 的支持已被移除。该文件之前未被正确加载，因此对设备配置没有效果。修复它可能在提供了无效配置时导致现有设备无法工作
提供了一个脚cxacru-cf.py，用于将现有文件转换sysfs 形式
检测到的设备会作为名为 "cxacru" ATM 设备出现。在 /sys/class/atm/ 下，它们是名cxacruN 的目录，其中 N 为设备号。一个名device 的符号链接指USB 接口设备的目录，其中包含了几个用于获取设备统计信息的 sysfs 属性文件：

- adsl_controller_version

- adsl_headend
- adsl_headend_environment

 - 关于远端局端（headend）的信息
- adsl_config

 - 配置写入接口 - 以十六进制格式写入参<index>=<value>	  以空白分隔，例如
		"1=0 a=5"

 - 一次最多发7 个参数，设置任何值时调制解调器都会重	  ADSL 连接。这些参数会被记录下来以备将来参考
- downstream_attenuation (dB)
- downstream_bits_per_frame
- downstream_rate (kbps)
- downstream_snr_margin (dB)

 - 下行统计信息
- upstream_attenuation (dB)
- upstream_bits_per_frame
- upstream_rate (kbps)
- upstream_snr_margin (dB)
- transmitter_power (dBm/Hz)

 - 上行统计信息
- downstream_crc_errors
- downstream_fec_errors
- downstream_hec_errors
- upstream_crc_errors
- upstream_fec_errors
- upstream_hec_errors

 - 错误计数
- line_startable

 - 表示设备上的 ADSL 支持
	  可以被启用，参见 adsl_start
- line_status

  - "initialising"（初始化中）
  - "down"（断开  - "attempting to activate"（尝试激活）
  - "training"（训练）
  - "channel analysis"（信道分析）
  - "exchange"（交换）
  - "waiting"（等待）
  - "up"（已连接
	如果没有信号，会"down" "attempting to activate"
	之间切换
- link_status

  - "not connected"（未连接  - "connected"（已连接  - "lost"（丢失）

- mac_address

- modulation

  - ""（未连接时）
  - "ANSI T1.413"
  - "ITU-T G.992.1 (G.DMT)"
  - "ITU-T G.992.2 (G.LITE)"

- startup_attempts

 - 初始ADSL 的总尝试次数
要启禁用 ADSL，可以向 adsl_state 文件写入以下内容
  - "start"（启动）
  - "stop"（停止）
  - "restart"（停止，等待 1.5s，然后启动）
  - "poll"（在因失败而被禁用后，用于恢复状态轮询）

```

	[4942145.150704] ATM dev 0: ADSL state: running
	[4942243.663766] ATM dev 0: ADSL line: down
	[4942249.665075] ATM dev 0: ADSL line: attempting to activate
	[4942253.654954] ATM dev 0: ADSL line: training
	[4942255.666387] ATM dev 0: ADSL line: channel analysis
	[4942259.656262] ATM dev 0: ADSL line: exchange
	[2635357.696901] ATM dev 0: ADSL line: up (8128 kb/s down | 832 kb/s up)

```
