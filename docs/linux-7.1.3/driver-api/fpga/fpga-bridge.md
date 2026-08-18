## FPGA 桥


#### 实现新 FPGA 桥的 API


- struct fpga_bridge - FPGA 桥结构
- struct fpga_bridge_ops - 底层桥驱动操作
- __fpga_bridge_register() - 创建并注册一个桥
- fpga_bridge_unregister() - 注销一个桥

辅助宏 `fpga_bridge_register()` 会自动将注册该 FPGA 桥的模块设为所有者。

   :functions: fpga_bridge

   :functions: fpga_bridge_ops

   :functions: __fpga_bridge_register

   :functions: fpga_bridge_unregister
