
## AMD 边带（SIDE BAND）接

部分基于 AMD Zen 的处理器通过称为高级平台管理链路（APML, Advanced Platform
Management Link）的边带接口（SBI）支持系统管理功能。APML 是一个基I2C/I3C 两线处理器目标接口。APML 用于与远程管理接口（SB 远程管理接口（SB-RMI）与 SB 温度
传感器接口（SB-TSI））通信
关于该接口的更多细节可以在家型号 PPR [^1^]_ 的 Advanced Platform Management
Link (APML)”章节中找到

## SBRMI 设备


drivers/misc/amd-sbi 下的 apml_sbrmi 驱动创建 miscdevice /dev/sbrmi-*，以让用空间程序运行 APML mailbox、CPUID、MCAMSR register xfer 命令
寄存器集APML 协议之间是通用的。IOCTL 在协议之间提供同步，因为事务可能产生
竞争条件

   $ ls -al /dev/sbrmi-3c
   crw-------    1 root     root       10,  53 Jul 10 11:13 /dev/sbrmi-3c

apml_sbrmi 驱动注册 hwmon 传感器，用于监控 power_cap_max、当前功耗以及管power_cap
dev 节点的特性：
 - 定义了不同的 xfer 协议 - Mailbox
 - CPUID
 - MCA_MSR
 - Register xfer

访问限制 - 只有 root 用户才允许打开该文件 - APML Mailbox 消息Register xfer 访问是可读写的，
 - CPUID MCA_MSR 访问是只读的
## 驱动 IOCTL


   :doc: SBRMI_IOCTL_MBOX_CMD
   :doc: SBRMI_IOCTL_CPUID_CMD
   :doc: SBRMI_IOCTL_MCAMSR_CMD
   :doc: SBRMI_IOCTL_REG_XFER_CMD

## 用户空间用法


C 程序访问边带接口```

  #include <uapi/misc/amd-apml.h>

```
其中定义了受支持IOCTL 以及要从用户空间传入的数据结构```

  int file;

  file = open("/dev/sbrmi-*", O_RDWR);
  if (file < 0) {
    /* 错误处理 */
    exit(1);
  }

```
定义了以IOCTL
`#define SB_BASE_IOCTL_NR      	0xF9`
`#define SBRMI_IOCTL_MBOX_CMD		_IOWR(SB_BASE_IOCTL_NR, 0, struct apml_mbox_msg)`
`#define SBRMI_IOCTL_CPUID_CMD		_IOWR(SB_BASE_IOCTL_NR, 1, struct apml_cpuid_msg)`
`#define SBRMI_IOCTL_MCAMSR_CMD	_IOWR(SB_BASE_IOCTL_NR, 2, struct apml_mcamsr_msg)`
`#define SBRMI_IOCTL_REG_XFER_CMD	_IOWR(SB_BASE_IOCTL_NR, 3, struct apml_reg_xfer_msg)`


用户空间 C-API esmi_oob_library 提供，托管于 [^2^]_，由 E-SMS 项目 [^3^]_ 提供