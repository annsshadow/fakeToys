## 内核驱动 raspberrypi-hwmon


支持的板卡：

  - Raspberry Pi A+ （通过 SoC 上的 GPIO
  - Raspberry Pi B+ （通过 SoC 上的 GPIO
  - Raspberry Pi 2 B （通过 SoC 上的 GPIO
  - Raspberry Pi 3 B （通过端口扩展器上GPIO
  - Raspberry Pi 3 B+ （通过 PMIC

作者：Stefan Wahren <stefan.wahren@i2se.com>

### 描述


该驱动定期轮VC4 固件的邮箱属性以检测欠压状况

### Sysfs 条目


======================= ==================
in0_lcrit_alarm		欠压告警
======================= ==================
