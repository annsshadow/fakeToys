
## Surface ACPI Notify


Surface ACPI Notify（SAN）设备提供了 ACPI 与 SAM 控制器之间的桥梁。具体而言，ACPI 代码可以通过此接口执行请求并处理电池和热量事件。除此之外，与 Surface Book 2 独立 GPU（dGPU）相关的事件可以从 ACPI 代码发送（注意：Surface Book 3 使用不同的方法）。目前已知通过此接口发送的唯一事件是 dGPU 上电通知。虽然该驱动在内部处理前一部分，但它仅通过其公共 API 将 dGPU 事件转发给任何感兴趣的其它驱动，而不处理它们。

该驱动的公共接口分为两部分：客户端注册与通知块（notifier-block）注册。

SAN 接口的客户端可以通过 |san_client_link| 作为消费者链接到 SAN 设备。这可用于确保接收 dGPU 事件的客户端不会由于 SAN 接口尚未建立而错过任何事件，因为这会强制客户端驱动在 SAN 驱动解绑时一并解绑。

只要模块被加载，任何设备都可以注册通知块，无论是否作为客户端链接。注册通过 |san_dgpu_notifier_register| 完成。如果不再需要该通知块，应通过 |san_dgpu_notifier_unregister| 注销。

更多细节请参阅下面的 API 文档。


## API 文档


    :export:
