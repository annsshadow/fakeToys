
## Devlink 共享实例


## 概述


共享 devlink 实例允许同一芯片上的多个物理功能（PF）共享一devlink 实例，用于芯片级的操作
多个 PF 可能驻留在同一物理芯片上，运行单一固件。这PF 之间可能会共享部分资源与配置。共devlink 实例提供了一个对象，用以固定配置旋钮
有两种可能的使用模型
1. 共享 devlink 实例与单独的 PF devlink 实例一起使用，在每-PF 配置之外提供芯片级配置2. 共享 devlink 实例是唯一devlink 实例，没有每-PF 实例
使用哪种模型由驱动决定
共享 devlink 实例不依托于任何 struct **device**
## 实现


### 架构


实现使用了：

- **芯片识别**：PF 通过驱动特定的标识符按芯片分- **共享实例管理**：带引用计数的共享实例全局列表

### API 函数


提供以下函数用于管理共享 devlink 实例
- `devlink_shd_get()`：获取或创建由字符串 ID 标识的共devlink 实例
- `devlink_shd_put()`：释放对共享 devlink 实例的引- `devlink_shd_get_priv()`：从共享 devlink 实例获取私有数据

### 初始化流

1. **PF 调用共享 devlink 初始* 在驱probe 期间
2. **芯片识别** 使用驱动特定的方法确定设备身3. **获取或创建共享实* 使用 `devlink_shd_get()`
   - 该函数按标识符查找已有实   - 如果不存在，则创建新实例     - 分配并注devlink 实例
     - 加入全局共享实例列表
     - 增加引用计数

4. **设置嵌套 devlink 实例** PF devlink 实例，在注册 PF devlink 实例之前使用
   `devl_nested_devlink_set()`

### 清理流程


1. **清理** PF 被移除时
2. **调用** `devlink_shd_put()` 释放引用（减少引用计数）
3. **共享实例在最后一PF 移除时（引用计数达到零）自动销*

### 芯片识别


属于同一芯片PF 使用驱动特定的方法识别。驱动可以自由选择任何适合确定两个 PF 是否属于同一设备标识符。示例包括：

- **PCI VPD 序列*：从 PCI VPD 提取
- **设备树属*：从设备树读取芯片标识符
- **其他硬件特定的标识符**：任何按芯片PF 分组的唯一标识
### 閿。

一个全局互斥量（`shd_mutex`）在注册/注销期间保护共享实例列表
与其他嵌devlink 实例关系类似，共享实例的 devlink 锁应始终PF devlink 锁之后获取
### 引用计数


每个共享 devlink 实例维护一个引用计数（`refcount_t refcount`）。引用计数在调用 `devlink_shd_get()`
时增加，在调`devlink_shd_put()` 时减少。当引用计数达到零时，共享实例被自动销毁