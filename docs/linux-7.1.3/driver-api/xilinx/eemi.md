## Xilinx Zynq MPSoC EEMI 文档


### Xilinx Zynq MPSoC 固件接口

zynqmp-firmware 节点描述了与平台固件（platform firmware）的接口。ZynqMP 有一个与安全管理固件通信的接口。固件驱动提供了访问固件 API 的接口。任何驱动都可以使用接口 API 与 PMC（平台管理控制器，Platform Management Controller）通信。

### 嵌入式能源管理接口（EEMI）

嵌入式能源管理接口用于允许在芯片或设备上不同处理簇上运行的软件组件与设备上的电源管理控制器（PMC）通信，以发出或响应电源管理请求。

任何希望通过 EEMI API 与 PMC 通信的驱动都使用为每个函数提供的函数。

### IOCTL

IOCTL API 用于设备控制和配置。它不是系统 IOCTL，而是 EEMI API。该 API 可由主设备（master）用于控制任何特定于设备的配置。IOCTL 定义可能特定于平台。该 API 还管理共享设备配置。

以下 IOCTL ID 对设备控制有效：
- IOCTL_SET_PLL_FRAC_MODE	8
- IOCTL_GET_PLL_FRAC_MODE	9
- IOCTL_SET_PLL_FRAC_DATA	10
- IOCTL_GET_PLL_FRAC_DATA	11

有关 IOCTL 特定参数和其他 EEMI API，请参阅 EEMI API 指南[^0^]。

### 参考

[^0^] 嵌入式能源管理接口（EEMI）API 指南：
    https://www.xilinx.com/support/documentation/user_guides/ug1200-eemi-api.pdf
