## 驱动变更


本文件详述了 2.6 中影PCMCIA 卡驱动作者的变更
- pcmcia_loop_config() 与自动配置（2.6.36 起）
   如果相应设置`struct pcmcia_device *p_dev->config_flags`   pcmcia_loop_config() 现在会自动设置某些配置值，尽管驱动仍可在回调函数中
   覆盖这些设置。目前提供以下自动配置选项
 - CONF_AUTO_CHECK_VCC : 检查匹配的 Vcc
 - CONF_AUTO_SET_VPP   : 设置 Vpp
 - CONF_AUTO_AUDIO     : 若需要则自动启用音频 - CONF_AUTO_SET_IO    : 设置 ioport 资源>resource[0,1] - CONF_AUTO_SET_IOMEM : 设置第一iomem 资源>resource[^2^]
- pcmcia_request_configuration -> pcmcia_enable_device（自 2.6.36 起）
   pcmcia_request_configuration() 已重命名pcmcia_enable_device()，因   它与 pcmcia_disable_device() 相对应。配置设置现在存储在 struct
   pcmcia_device 中，例如 config_flags、config_index、config_base、vpp
   等字段
- pcmcia_request_window 变更（自 2.6.36 起）
   驱动现在不再使用 win_req_t，而是需要填`struct pcmcia_device
   *p_dev->resource[2,3,4,5]` 以支持最多四ioport 范围。调   pcmcia_request_window() 后，找到的区域会被保留，并可立即使用——直到调   pcmcia_release_window()
- pcmcia_request_io 变更（自 2.6.36 起）
   驱动现在不再使用 io_req_t，而是需要填`struct pcmcia_device
   *p_dev->resource[0,1]` 以支持最多两ioport 范围。调pcmcia_request_io()
   后，找到的端口会被保留；在调pcmcia_request_configuration() 后，它们方可
   使用
- 不再dev_info_t，不再有 cs_types.h（自 2.6.36 起）
   dev_info_t 以及另外几个 typedef 已被移除。不要再PCMCIA 设备驱动中使   它们。同时，不要包含 pcmcia/cs_types.h，因为该文件已不存在
- 不再dev_node_t（自 2.6.35 起）
   不再需要填"dev_node_t" 结构
- 新的 IRQ 请求规则（自 2.6.35 起）
   驱动现在不再使用旧的 pcmcia_request_irq() 接口，而是可以在以下两者间选择
   - 直接调用 request_irq/free_irq。使用来`*p_dev->irq` IRQ   - 使用 pcmcia_request_irq(p_dev, handler_t)；PCMCIA 核心会在调用
     pcmcia_disable_device() 或设备弹出时自动清理
- 不再cs_error / CS_CHECK / CONFIG_PCMCIA_DEBUG（自 2.6.33 起）
   请使Linux 风格的检查返回值的方式，来代替 cs_error() 回调CS_CHECK()
   宏；如有必要，调试信息请使用 "dev_dbg()" "pr_debug()"
- 新的 CIS 元组访问（自 2.6.33 起）
   驱动应使"pcmcia_get_tuple()"（如果只对单个（原始）元组感兴趣）或
   "pcmcia_loop_tuple()"（如果对某一类型的所有元组感兴趣），来代   pcmcia_get_{first,next}_tuple()、pcmcia_get_tuple_data()    pcmcia_parse_tuple()。为了从 CISTPL_FUNCE 解码 MAC，新增了辅助函数
   "pcmcia_get_mac_from_cis()"銆。
- 新的配置循环辅助函数（自 2.6.28 起）
   通过调用 pcmcia_loop_config()，驱动可以遍历所有可用的配置选项。在驱动   probe() 阶段，在大多数（如果不是全部）情况下，都无需直接使用
   pcmcia_get_{first,next}_tuple、pcmcia_get_tuple_data pcmcia_parse_tuple
- 新的释放辅助函数（自 2.6.17 起）
   现在不再需要调pcmcia_release_{configuration,io,irq,win}，只需调用
   pcmcia_disable_device 即可。由于已没有合理的理由去调用 pcmcia_release_io
   pcmcia_release_irq，它们的导出已被移除
- 统一 detach REMOVAL 事件代码，以attach INSERTION 事件代码

```
       void (*remove)          (struct pcmcia_device *dev);
       int (*probe)            (struct pcmcia_device *dev);

```
```

       int (*suspend)          (struct pcmcia_device *dev);
       int (*resume)           (struct pcmcia_device *dev);

  should be initialized in struct pcmcia_driver, and handle
  (SUSPEND == RESET_PHYSICAL) and (RESUME == CARD_RESET) events

```
- 事件处理程序struct pcmcia_driver 中的初始化（2.6.13 起）
   事件处理程序会收到所有事件的通知，并且必须作为驱struct pcmcia_driver
   中的 event() 回调进行初始化
- 不应再使pcmcia/version.h（自 2.6.13 起）
   该文件最终将被移除
- 内核内的设备<->驱动匹配（自 2.6.13 起）
   PCMCIA 设备及其正确的驱动现在可以在内核空间中进行匹配。详   'devicetable.txt'
- 设备模型集成（自 2.6.11 起）
   struct pcmcia_device 会注册到设备模型核心，并可通过
   handle_to_dev(client_handle_t * handle) 使用（例如用SET_NETDEV_DEV）
- 将内I/O 端口地址转换unsigned int（自 2.6.11 起）
   PCMCIA 卡驱动中，ioaddr_t 应替换为 unsigned int
- irq_mask irq_list 参数（自 2.6.11 起）
   irq_mask irq_list 参数不应再在 PCMCIA 卡驱动中使用。相反，确定应使   哪个 IRQ PCMCIA 核心的职责。因此，link->irq.IRQInfo2 会被忽略
- client->PendingEvents 宸茬Щ闄わ紙鑷?2.6.11 璧凤級
   client->PendingEvents 不再可用
- client->Attributes 宸茬Щ闄わ紙鑷?2.6.11 璧凤級
   client->Attributes 未被使用，因此已从所PCMCIA 卡驱动中移除

- 核心函数不再可用（自 2.6.11 起）
   以下函数已从内核源码中移除，因为所有内核内驱动都不使用它们，且没有外部

```
	pcmcia_get_first_region()
	pcmcia_get_next_region()
	pcmcia_modify_window()
	pcmcia_set_event_mask()
	pcmcia_get_first_window()
	pcmcia_get_next_window()

```
- 模块移除时的设备列表遍历（自 2.6.10 起）
   在模块移除时，不再需要遍历驱动的内部客户端列表并调用 ->detach() 函数
- 资源管理。（2.6.8 起）
   尽管 PCMCIA 子系统会为卡分配资源，但它不再将这些资源标记为忙。这意味着
   驱动作者现在有责任Linux 中的其他驱动一样声明您的资源。您应使   request_region() 将您IO 区域标记为使用中，并使用 request_mem_region()
   将您的内存区域标记为使用中。name 参数应是指向您驱动名称的指针。例如，对于
   pcnet_cs，name 应指向字符串 "pcnet_cs"
- CardServices 已移   2.4 中的 CardServices() 只是一个用于调用各种服务的switch 语句。在 2.6
   中，所有这些入口点都被导出并直接调用（pcmcia_report_error() 除外，直接改   cs_error() 即可）
- struct pcmcia_driver
   您需要使struct pcmcia_driver pcmcia_{un,}register_driver，而不   {un,}register_pccard_driver
