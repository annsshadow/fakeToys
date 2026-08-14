## 内核驱动 vexpress


支持的系统：

  - ARM Ltd. Versatile Express 平台

    Prefix: 'vexpress'

    Datasheets:

      - Versatile Express 电路板的《技术参考手册》中的“硬件描述”章节：

 - http://infocenter.arm.com/help/topic/com.arm.doc.subset.boards.express/index.html

      - V2M-P1 TRM 中的“4.4.14. 系统配置寄存器”章节：

 - http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0447-/index.html

Author: Pawel Moll

### 描述


Versatile Express 平台（http://www.arm.com/versatileexpress/）是 ARM Ltd. 处理器的参考与原型系统。它可由种类繁多的电路板组成，每块电路板（除了主芯片/FPGA 之外）都包含若干个负责平台配置与控制的微控制器。这些微控制器还可以通过多个内部和外部传感器监视电路板及其环境，提供有关电源线电压和电流、电路板温度以及功耗的信息。其中一些还会计算消耗的能源并提供累计使用计数器。

配置设备_不_是内存映射的，必须通过由 "vexpress_config" API 抽象的自定义接口访问。

由于这些设备是不可发现的，必须在传递给内核的 Device Tree 中描述。它们的 DT 绑定的详细信息可以在 Documentation/devicetree/bindings/hwmon/vexpress.txt 中找到。
