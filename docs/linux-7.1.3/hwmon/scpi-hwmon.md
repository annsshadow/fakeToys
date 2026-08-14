## 内核驱动 scpi-hwmon


支持的芯片：

 - 基于 ARM 系统控制处理器接口的芯片

   扫描的地址：-

   数据表：http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0922b/index.html

作者：Punit Agrawal <punit.agrawal@arm.com>

### 描述


该驱动程序支持对基于 ARM 系统控制处理器 (SCP) 实现系统控制处理器接口 (SCPI)
的 SoC 进行硬件监控。SCP 支持以下传感器类型：

  - 温度
  - 电压
  - 电流
  - 功率

SCP 接口提供了一个 API 来查询可用传感器及其值，然后由该驱动程序导出到
用户空间。

### 使用说明


该驱动依赖设备树节点来指示内核中存在 SCPI 支持。有关设备树节点的详细信息，
请参阅 Documentation/devicetree/bindings/firmware/arm,scpi.yaml。
