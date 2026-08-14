
## AMD HSMP 接口


更新的 Fam19h（型号 0x00-0x1f、0x30-0x3f、0x90-0x9f、0xa0-0xaf）、
Fam1Ah（型号 0x00-0x1f）AMD EPYC 服务器系列处理器通过 HSMP（Host System Management
Port，主机系统管理端口）支持系统管理功能。

主机系统管理端口（HSMP）是一个接口，用于向操作系统级别的软件提供对一组邮箱寄存器的
系统管理功能的访问。

关于该接口的更多细节可以在对应 family/model 的 PPR 的"7 Host System Management Port
(HSMP)"章节中找到，例如：https://docs.amd.com/v/u/en-US/55898_B1_pub_0_50


HSMP 接口在 EPYC 系列服务器 CPU 和 MI300A（APU）上受支持。


## HSMP 设备


位于 drivers/platforms/x86/amd/hsmp/ 下的 amd_hsmp 驱动，为基于 ACPI 对象的探测、基于
平台设备的探测，以及这两个驱动的公共代码，分别提供独立的驱动文件。

Kconfig 选项 CONFIG_AMD_HSMP_PLAT 编译 plat.c 并生成 amd_hsmp.ko。
Kconfig 选项 CONFIG_AMD_HSMP_ACPI 编译 acpi.c 并生成 hsmp_acpi.ko。
选择这两个配置中的任意一个都会自动选中 CONFIG_AMD_HSMP。这会编译公共代码 hsmp.c 并
生成 hsmp_common.ko 模块。

ACPI 和 plat 两个驱动都会创建 miscdevice /dev/hsmp，以便用户空间程序运行 hsmp 邮箱
命令。

驱动支持的 ACPI 对象格式定义如下。

$ ls -al /dev/hsmp
crw-r--r-- 1 root root 10, 123 Jan 21 21:41 /dev/hsmp

设备节点的特性：
 - 写模式用于运行 set/configure（设置/配置）命令
 - 读模式用于运行 get/status（获取/状态）监视命令

访问限制：
 - 只有 root 用户被允许以写模式打开该文件。
 - 所有用户都可以以读模式打开该文件。

内核内集成：
 - 内核中的其它子系统可以使用导出的传输函数 hsmp_send_message()。
 - 跨调用方的加锁由驱动负责。


## HSMP sysfs 接口


1. 指标表二进制 sysfs

AMD MI300A MCM 提供了 GET_METRICS_TABLE 消息，用于一次性从 SMU 获取大部分的系统管理
信息。

指标表作为十六进制 sysfs 二进制文件提供，位于每个 socket 的 sysfs 目录下，该目录创建于
/sys/devices/platform/amd_hsmp/socket%d/metrics_bin

注意：不支持 lseek()，因为整个指标表会被读取。

指标表的定义将作为 Public PPR 的一部分进行文档化。同样的定义也在 amd_hsmp.h 头文件中。

2. HSMP 遥测 sysfs 文件

以下 sysfs 文件在 /sys/devices/platform/AMDI0097:0X/ 下可用。

- c0_residency_input：处于 C0 状态的核的百分比。
- prochot_status：如果处理器处于热阈值则返回 1，否则返回 0。
- smu_fw_version：SMU 固件版本。
- protocol_version：HSMP 接口版本。
- ddr_max_bw：理论最大 DDR 带宽，单位为 GB/s。
- ddr_utilised_bw_input：当前已使用的 DDR 带宽，单位为 GB/s。
- ddr_utilised_bw_perc_input(%)：当前已使用 DDR 带宽的百分比。
- mclk_input：内存时钟频率，单位为 MHz。
- fclk_input：Fabric 时钟频率，单位为 MHz。
- clk_fmax：socket 的最大频率，单位为 MHz。
- clk_fmin：socket 的最小频率，单位为 MHz。
- cclk_freq_limit_input：每个 socket 的核时钟频率限制，单位为 MHz。
- pwr_current_active_freq_limit：socket 当前的活动频率限制，单位为 MHz。
- pwr_current_active_freq_limit_source：当前活动频率限制的来源。

## ACPI 设备对象格式


amd_hsmp 驱动期望的 ACPI 对象格式
```

  Device(HSMP)
		{
			Name(_HID, "AMDI0097")
			Name(_UID, "ID00")
			Name(HSE0, 0x00000001)
			Name(RBF0, ResourceTemplate()
			{
				Memory32Fixed(ReadWrite, 0xxxxxxx, 0x00100000)
			})
			Method(_CRS, 0, NotSerialized)
			{
				Return(RBF0)
			}
			Method(_STA, 0, NotSerialized)
			{
				If(LEqual(HSE0, One))
				{
					Return(0x0F)
				}
				Else
				{
					Return(Zero)
				}
			}
			Name(_DSD, Package(2)
			{
				Buffer(0x10)
				{
					0x9D, 0x61, 0x4D, 0xB7, 0x07, 0x57, 0xBD, 0x48,
					0xA6, 0x9F, 0x4E, 0xA2, 0x87, 0x1F, 0xC2, 0xF6
				},
				Package(3)
				{
					Package(2) {"MsgIdOffset", 0x00010934},
					Package(2) {"MsgRspOffset", 0x00010980},
					Package(2) {"MsgArgOffset", 0x000109E0}
				}
			})
		}

```
## HSMP HWMON 接口


HSMP 电源传感器向 hwmon 接口注册。为每个 socket 创建一个独立的 hwmon 目录，并在该 hwmon
目录中生成以下文件。
- power1_input（只读）
- power1_cap_max（只读）
- power1_cap（读、写）

## 一个示例


从 C 程序访问 hsmp 设备。
```

  #include <linux/amd_hsmp.h>

```
其定义了受支持的消息/消息 ID。
```

  int file;

  file = open("/dev/hsmp", O_RDWR);
  if (file < 0) {
    /* ERROR HANDLING; 你可以检查 errno 来了解出了什么问题 */
    exit(1);
  }

```
定义了以下 IOCTL：

`ioctl(file, HSMP_IOCTL_CMD, struct hsmp_message *msg)`
```

    struct hsmp_message {
    	__u32	msg_id;				/* 消息 ID */
    	__u16	num_args;			/* 消息中输入参数字的个数 */
    	__u16	response_sz;			/* 期望的输出/响应字的个数 */
    	__u32	args[HSMP_MAX_MSG_LEN];		/* 参数/响应缓冲 */
    	__u16	sock_ind;			/* socket 编号 */
    };

```
该 ioctl 在失败时会返回非零值；你可以读取 errno 来了解发生了什么。该事务在成功时返回 0。

关于该接口和消息定义的更多细节可以在对应 family/model 的 PPR 的"7 Host System
Management Port (HSMP)"章节中找到，例如：https://docs.amd.com/v/u/en-US/55898_B1_pub_0_50

用户空间 C-API 可通过链接 esmi 库获得，该库由 E-SMS 项目提供
https://www.amd.com/en/developer/e-sms.html。参见：https://github.com/amd/esmi_ib_library
