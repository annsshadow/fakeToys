
## 基于 Generic Netlink 的 DRM RAS


DRM RAS（Reliability, Availability, Serviceability，可靠性、可用性、可服务性）接口为 GPU/加速器
驱动提供了一种标准化的方式，通过 Generic Netlink 向用户空间暴露错误计数器及其它可靠性节点。
这使得诊断工具、监控守护进程或测试基础设施能够以统一的方式跨不同 DRM 驱动查询硬件健康状态。

主要目标：

- 为 GPU 与加速器驱动提供标准化的 RAS 解决方案，以支持数据中心监控与可靠性运维。
- 实现一个单一的 drm-ras Generic Netlink 系列，以满足现代 Netlink YAML 规范，并将所有 RAS 相关
  通信集中到同一命名空间中。
- 支持基础的错误计数器接口，满足当前紧迫且必要的监控需求。
- 提供灵活、面向未来的接口，未来可扩展以支持其它类型的 RAS 数据。
- 允许每个驱动拥有多个节点，使驱动能够为不同的 IP 块、子块或其它适用的逻辑细分单元注册独立的
  节点。

## 节点


节点是表示设备内部错误类型或错误来源的逻辑抽象。目前仅支持错误计数器节点。

驱动负责通过 `drm_ras_node_register()` 与 `drm_ras_node_unregister()` API 注册和注销节点。

### 节点管理


   :doc: DRM RAS Node Management
   :internal:

## Generic Netlink 用法


该接口实现为一个名为 `drm-ras` 的 Generic Netlink 系列。用户空间工具可以：

- 使用 `list-nodes` 命令列出已注册的节点。
- 使用 `get-error-counter` 命令，并以 `node-id` 作为参数，列出某个节点中的所有错误计数器。
- 使用 `get-error-counter` 命令，同时以 `node-id` 与 `error-id` 作为参数，查询特定的错误计数器值。

### 基于 YAML 的接口


该接口由一个 YAML 规范 `Documentation/netlink/specs/drm_ras.yaml` 描述。

此 YAML 通过 `tools/net/ynl/pyynl/ynl_gen_c.py` 自动生成用户空间绑定，并驱动 netlink 属性与
操作的结构。

### 使用说明


- 用户空间必须首先枚举节点以获取其 ID。
- 节点 ID 或节点名可用于所有后续查询，例如错误计数器。
- 错误计数器可以通过错误 ID 或错误名查询。
- 查询参数应定义为 uAPI 的一部分，以确保用户接口的稳定性。
- 该接口支持通过添加新的节点类型与额外属性来扩展。

示例：使用 ynl 列出节点


    sudo ynl --family drm_ras --dump list-nodes
    [{'device-name': '0000:03:00.0',
    'node-id': 0,
    'node-name': 'correctable-errors',
    'node-type': 'error-counter'},
    {'device-name': '0000:03:00.0',
     'node-id': 1,
     'node-name': 'uncorrectable-errors',
     'node-type': 'error-counter'}]

示例：使用 ynl 列出所有错误计数器


    sudo ynl --family drm_ras --dump get-error-counter --json '{"node-id":0}'
    [{'error-id': 1, 'error-name': 'error_name1', 'error-value': 0},
    {'error-id': 2, 'error-name': 'error_name2', 'error-value': 0}]

示例：查询某个给定节点的错误计数器


    sudo ynl --family drm_ras --do get-error-counter --json '{"node-id":0, "error-id":1}'
    {'error-id': 1, 'error-name': 'error_name1', 'error-value': 0}
