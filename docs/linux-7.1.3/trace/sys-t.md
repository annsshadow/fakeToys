
## 通过 STP MIPI SyS-T


MIPI SyS-T 协议驱动可以STM 类设备一起使用，以生成标准化的跟踪流（trace stream）。除了作为标准之外，它还提供更好的跟踪源识别与时间戳关联（timestamp correlation）
为了MIPI SyS-T 协议驱动用于你的 STM 设备，首先你需CONFIG_STM_PROTO_SYS_T
现在，你可以在为 STM 设备创建策略（policy）时，通过在策略名称中指定来选择要使用的协议驱动
# mkdir /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/

换句话说，策略名称格式扩展如下：

  <device_name>:<protocol_name>.<policy_name>

因此，使Intel TH 时它可能看起来像 "0-sth:p_sys-t.my-policy"
如果省略协议名称，STM 类将选择最先加载的那个协议驱动
你也可以通过以下方式再次确认一切按预期工作
# cat /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/protocol
p_sys-t

现在，使MIPI SyS-T 协议驱动时，configfs 中的每个策略节点都会获得一些额外的属性，它们决定了特定于该协议的每源（per-source）参数：

# mkdir /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/default
# ls /config/stp-policy/dummy_stm.0:p_sys-t.my-policy/default
channels
clocksync_interval
do_len
masters
ts_interval
uuid

其中最重要的是 "uuid"，它决定了用于标记来自此源的所有数据的 UUID。当创建一个新节点时它会自动生成，但你很可能会想要更改它
do_len 开关闭 MIPI SyS-T 消息头中的附加“payload length（负载长度）”字段。默认关闭，因为 STP 已经标记了消息边界
ts_interval clocksync_interval 分别决定了在消息头中包含协议（而非传输，即 STP）时间戳或发CLOCKSYNC 包之前，可以经过多少毫秒时间
详见 Documentation/ABI/testing/configfs-stp-policy-p_sys-t
- [^1^] https://www.mipi.org/specifications/sys-t
