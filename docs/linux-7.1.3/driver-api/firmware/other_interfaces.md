## 其他固件接口


### DMI 接口


   :export:

### EDD 接口


   :internal:

### 通用系统帧缓冲接口


   :export:

### Intel Stratix10 SoC 服务层


Intel Stratix10 SoC 的某些特性需要比内核被授予的级别更高的特权。此类安全
特性包括 FPGA 编程。就 ARMv8 架构而言，内核运行在异常级别 1 (EL1)，而访问
这些特性需要异常级别 3 (EL3)。

Intel Stratix10 SoC 服务层为驱动程序提供了一个内核内 API，用于请求访问
安全特性。请求被排队并逐个处理。ARM 的 SMCCC 用于将请求的执行传递给安全
监视器 (EL3)。

   :functions: stratix10_svc_command_code

   :functions: stratix10_svc_client_msg

   :functions: stratix10_svc_command_config_type

   :functions: stratix10_svc_cb_data

   :functions: stratix10_svc_client

   :export:
