
## 内核驱动 lattepanda-sigma-ec


支持的系统：

  - LattePanda Sigma（Intel 13 i5-1340P
    DMI 厂商：LattePanda

    DMI 产品：LattePanda Sigma

    BIOS 版本.27（已验证
    数据手册：无（EC 寄存器通过实验发现
作者：Mariano Abad <weimaraner@gmail.com>

### 描述


该驱动为 DFRobot 制造的 LattePanda Sigma 单板计算机提供硬件监控。该板使用一ITE IT8613E 嵌入式控制器（EC）来管理 CPU 散热风扇和热传感器
BIOS ACPI 嵌入式控制器（`PNP0C09`）声明为 `_STA` 返回 0，从而阻止内核的 ACPI EC
子系统进行初始化。该驱动通过标准ACPI EC I/O 端口（`0x62` 数据、`0x66` 命令/状态）
直接读取 EC
### Sysfs 属


======================= ===============================================
`fan1_input`          风扇转速，单位 RPM（EC 寄存0x2E:0x2F                        16 位大端）
`fan1_label`          "CPU Fan"
`temp1_input`         板载/环境温度，单位毫摄氏                        （EC 寄存0x60，无符号`temp1_label`         "Board Temp"
`temp2_input`         CPU 邻近温度，单位毫摄氏                        （EC 寄存0x70，无符号`temp2_label`         "CPU Temp"
======================= ===============================================

### 模块参数


`force`（bool，默false    BIOS 版本5.27 时强制加载。驱动仍要求 DMI 厂商与产品名匹配
### 已知限制


- 不支持风扇转速控制。风扇始终处EC 的自动控制之下- EC 寄存器映射仅BIOS 版本 5.27 上得到验证。其他版本可能使用不同的寄存器偏移；
  使用 `force` 参数需自行承担风险