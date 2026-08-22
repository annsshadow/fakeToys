## IBM s390 QDIO 以太网驱

## OSA HiperSockets 桥接端口（Bridge Port）支

### Uevents


要生成这些事件，设备必须被赋予主（primary）或次（secondary）桥接端口的角色。更多信息，请参z/VM Connectivity, SC24-6174"
当运行在 OSA HiperSockets 桥接能力端口硬件上，并且通道上某个已配置的桥接端口设备状态发生变化时，会代表相应ccwgroup 设备发出一ACTION=CHANGE udev 事件。该事件具有以下属性：

BRIDGEPORT=statechange
  表示该桥接端口设备改变了其状态
ROLE={primary|secondary|none}
  赋予该端口的角色
STATE={active|standby|inactive}
  该端口新采纳的状态
当运行在启用了主机地址通知HiperSockets 桥接能力端口硬件上时，会发出一ACTION=CHANGE udev 事件。当某个主机VLAN 在该设备所服务的网络上注册或注销时，会代表相应的 ccwgroup 设备发出该事件。该事件具有以下属性：

BRIDGEDHOST={reset|register|deregister|abort}
  主机地址
  通知重新启动、在桥接端口 HiperSockets 通道上注册或注销一个新的主机或 VLAN，或者中止地址通知
VLAN=numeric-vlan-id
  事件发生所在的 VLAN ID。如果事件不涉及 VLAN，则不包含此项
MAC=xx:xx:xx:xx:xx:xx
  正在被注册或注销的主机的 MAC 地址。如果事件报告的VLAN 的创建或销毁，则不报告此项
NTOK_BUSID=x.y.zzzz
  设备总线 ID（CSSID、SSID device number）
NTOK_IID=xx
  设备 IID
NTOK_CHPID=xx
  设备 CHPID
NTOK_CHID=xxxx
  设备通道 ID
请注意，`NTOK_*` 属性指的是与运行该 OS 的系统所连接设备不同的设备