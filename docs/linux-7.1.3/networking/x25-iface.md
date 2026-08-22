## X.25 设备驱动接口


版本 1.1

			   Jonathan Naylor 26.12.96

本文描述了在 X.25 分组层（Packet Layer）与 X.25 设备驱动之间传递的消息。它被设计成便于从分组层内部轻松设置 LAPB 模式
X.25 设备驱动将按Linux 设备驱动标准正常编写。大多数 X.25 设备驱动与已有的
以太网设备驱动大体相似。但与那些驱动不同，X.25 设备驱动带有与之关联的状态，
且这些信息需要在分组层之间传入传出，以保证正常运行
所有消息都存放sk_buff 中，就像要通过 LAPB 链路传输的真实数据一样。skbuff
的第一个字节指示其余部分的含义（如果还存在更多信息）

### 分组层到设备驱动


First Byte = 0x00 (X25_IFACE_DATA)

表示 skbuff 的其余部分包含要通过 LAPB 链路传输的数据。在传递任何数据之前，
LAPB 链路应当已经建立
First Byte = 0x01 (X25_IFACE_CONNECT)

建立 LAPB 链路。如果链路已经建立，则连接确认消息应尽快返回
First Byte = 0x02 (X25_IFACE_DISCONNECT)

终止 LAPB 链路。如果已经断开，则断开确认消息应尽快返回
First Byte = 0x03 (X25_IFACE_PARAMS)

LAPB 参数。待定义

### 设备驱动到分组层


First Byte = 0x00 (X25_IFACE_DATA)

表示 skbuff 的其余部分包含已通过 LAPB 链路接收的数据
First Byte = 0x01 (X25_IFACE_CONNECT)

LAPB 链路已建立。同一条消息既用于 LAPB 链路connect_confirmation（连接确认）也用connect_indication（连接指示）
First Byte = 0x02 (X25_IFACE_DISCONNECT)

LAPB 链路已终止。同一条消息既用于 LAPB 链路disconnect_confirmation（断开
确认），也用disconnect_indication（断开指示）
First Byte = 0x03 (X25_IFACE_PARAMS)

LAPB 参数。待定义

### 对设备驱动的要求


在分组层与设备驱动之间传递数据包时，不应重排序或丢弃
为避免从设备驱动向分组层传递数据包时发生重排序或丢弃，设备驱动不应调用
"netif_rx" 来递交接收到的数据包，而应softirq 上下文调"netif_receive_skb_core" 来递交它们