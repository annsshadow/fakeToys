## w1-uart 内核驱动


Author: Christoph Winklhofer <cj.winklhofer@gmail.com>


### 描述


UART 1-Wire 总线驱动。该驱动通过串行设备总线（Serial Device Bus）利UART
接口来创1-Wire 时序模式，如文档 `"Using a UART to Implement a 1-Wire Bus Master"`_
中所述
简而言之，UART 外设必须支持全双工并以开漏模式工作。时序模式由特定的波特率
与发送字节组合生成，对应1-Wire 读位、写位或复位脉冲
例如-Wire 复位与存在检测的时序模式使用波特9600，即每比104.2 微秒通过 UART 发送的字节 0xf0（最低有效位优先，起始位为低）将 1-Wire 的复位低
电平时间设为 521 微秒。存在的 1-Wire 设备通过拉低线路改变接收字节，驱动借此
评估 1-Wire 操作的结果
1-Wire 读位或写位类似，使用波特115200，即每比8.7 微秒。发送的字节 0x80
用于0 操作（低电平时间 69.6 微秒），字节 0xff 用于0、读 1 和写 1
（低电平时间 8.7 微秒）
复位与存在检测的默认波特率为 9600-Wire 读或写操作为 115200。如果实际波特率
与请求的波特率不同，则会调整发送字节以生成 1-Wire 时序模式

### 用法


通过在设备树中向串行节点（如 uart0）添加单个子节点 onewire 来指UART
1-wire 总线。例如：

```
  @uart0 {
    ...
    onewire {
      compatible = "w1-uart";
    };
  };

```
