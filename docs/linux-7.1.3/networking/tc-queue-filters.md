
## 基于 TC 队列的过


TC 可用于将流量引导到一组队列或单个队列，无论是发送侧还是接收侧

在发送侧

1) 使用 action skbedit priority 进行 Tx 优先级选择，可将流量引导到一组队列，当使mqprio 配置队列集时，优先级映射到流量类（队列集）

2) TC 过滤器使action skbedit queue_mapping $tx_qid 将流量引导到发送队列。发送队列的 action skbedit queue_mapping 仅在软件中执行，无法卸载到硬件

同样地，在接收侧，支持以下两种用于选择一组队列和/或单个队列的过滤器：

1) TC flower 过滤器使'hw_tc' 选项将传入流量引导到一组队列
   hw_tc $TCID - 指定一个硬件流量类，将匹配的数据包传递给它。TCID 范围0 15

2) 带有 action skbedit queue_mapping $rx_qid TC 过滤器选择一个接收队列。接收队列的 action skbedit queue_mapping 仅支持在硬件中实现。多个过滤器可能在硬件中竞争队列选择。在这种情况下，硬件流水线根据优先级解决冲突。在 Intel E810 设备上，将流量引导到队列TC 过滤器比分配队列的流导向（flow director）过滤器具有更高的优先级。哈希过滤器的优先级最低
