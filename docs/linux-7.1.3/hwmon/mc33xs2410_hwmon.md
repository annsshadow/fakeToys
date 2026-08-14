## 内核驱动 mc33xs2410_hwmon


支持的设备：

  - NXPs MC33XS2410

    Datasheet: https://www.nxp.com/docs/en/data-sheet/MC33XS2410.pdf

Authors:

	Dimitri Fedrau <dimitri.fedrau@liebherr.com>

### 说明


MC33XS2410 是一款四通道自保护高边开关（high-side switch），具备硬件监控功能，可对四个通道各自的温度、电流和电压进行监控。

### Sysfs 条目


======================= ======================================================
temp1_label		"中央芯片温度"（Central die temperature）
temp1_input		中央芯片的测量温度

temp[2-5]_label		"通道 [1-4] 温度"
temp[2-5]_input		单个通道的测量温度
temp[2-5]_alarm		温度告警
temp[2-5]_max		最高温度
======================= ======================================================
