## I915 GuC 提交/DRM 调度器章节


## 上游计划


要让 GuC 提交落地并让 i915 与 DRM 调度器集成，整体上游计划是：

- 合并基本的 GuC 提交
 - 对所有 gen11+ 平台的基本提交支持
 - 默认不在任何当前平台上启用，但可以通过 modparam enable_guc 启用
 - 需要与 DRM 调度器集成的大量重构工作，因此无需对代码吹毛求疵，只要功能正常、
	  没有重大编码风格/分层错误、并且不使 execlists 退步即可
 - 根据需要更新 IGT/selftest 以配合 GuC 提交工作
 - 在受支持平台上启用 CI 作为基线
 - 根据需要重构/让 GuC 提交的 CI 恢复健康
- 合并新的并行提交 uAPI
 - bonding uAPI 与 GuC 提交完全不兼容，而且总体来说它有严重的设计问题，这就是为什么
	  无论如何在任何情况下我们都想淘汰它
 - 新的 uAPI 增加了 I915_CONTEXT_ENGINES_EXT_PARALLEL 上下文设置步骤，该步骤用一个
	  包含 N 个上下文的槽位进行配置
 - 在 I915_CONTEXT_ENGINES_EXT_PARALLEL 之后，用户可以在单个 execbuf IOCTL 中向一个
	  槽位提交 N 个批处理，这些批处理在 GPU 上并行运行
 - 最初仅用于 GuC 提交，但如果需要也可以支持 execlists
- 将 i915 转换为使用 DRM 调度器
 - GuC 提交后端与 DRM 调度器完全集成
  - 从后端移除所有请求队列（例如所有背压都在 DRM 调度器中处理）
  - DRM 调度器中的复位/取消钩子
  - DRM 调度器中的看门狗钩子
  - 一旦与 DRM 调度器集成，GuC 后端的许多复杂性都可以抽出（例如状态机变得更简单、
		  锁变得更简单等……）
 - execlists 后端是挂接 DRM 调度器所需的最小实现
  - 传统接口
  - 像时间片/抢占/虚拟引擎这样的特性很难与 DRM 调度器集成，而且这些特性对于 GuC
		  提交不是必需的，因为 GuC 替我们做了这些事情
  - 完全集成到 DRM 调度器的投资回报率（ROI）低
  - 完全集成会给 DRM 调度器增加大量复杂性
 - 在 DRM 调度器中移植 i915 优先级继承/提升特性
  - 用于 i915 页面翻转，对其他 DRM 驱动也可能有用
  - 将成为 DRM 调度器中的一个可选特性
 - 从 DRM 调度器中移除顺序完成假设
  - 即使使用 DRM 调度器，后端也会处理抢占、时间片等等……所以作业有可能乱序完成
 - 抽出 i915 优先级级别并使用 DRM 优先级级别
 - 根据需要优化 DRM 调度器

## GuC 提交上游的待办


- 需要更新 GuC 固件/i915 以启用错误状态捕获
- 开源工具来解码 GuC 日志
- 公开的 GuC 规范

## 基本 GuC 提交的新 uAPI


基本 GuC 提交的 uAPI 不需要重大更改。唯一的更改是一个新的调度器属性：
I915_SCHEDULER_CAP_STATIC_PRIORITY_MAP。该属性表示 2k 个 i915 用户优先级级别被静态
映射为 3 个级别，如下所示：

- -1k 到 -1 低优先级
- 0 中优先级
- 1 到 1k 高优先级

这是需要的，因为 GuC 只有 4 个优先级带。最高优先级带被内核保留。这也与 DRM 调度器的
优先级级别一致。

### 规范参考：

- https://www.khronos.org/registry/EGL/extensions/IMG/EGL_IMG_context_priority.txt
- https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/chap5.html#devsandqueues-priority
- https://spec.oneapi.com/level-zero/latest/core/api.html#ze-command-queue-priority-t

## 新的并行提交 uAPI


现有的 bonding uAPI 在 GuC 提交下完全失效，因为在通过 I915_SUBMIT_FENCE 激活的
execbuf 时间之前，并不知道一次提交是单上下文提交还是并行提交。为了与 GuC 并行提交
多个上下文，上下文必须用 N 个上下文显式注册，并且所有 N 个上下文必须在单个命令中
提交给 GuC。GuC 接口不支持像 bonding uAPI 那样在 N 个上下文之间动态切换。因此需要
一个新的并行提交接口。此外，传统的 bonding uAPI 相当令人困惑且一点也不直观。而且
I915_SUBMIT_FENCE 在设计上是一个未来 fence，因此并不是我们应该继续支持的东西。

新的并行提交 uAPI 由 3 部分组成：

- 导出引擎逻辑映射
- 一个用于为并行提交配置上下文的 'set_parallel' 扩展
- 扩展 execbuf2 IOCTL 以支持在单个 IOCTL 中提交 N 个 BB

### 导出引擎逻辑映射


某些用例要求 BB 按逻辑顺序放置在引擎实例上（例如 gen11+ 上的分帧）。引擎实例的逻辑
映射可能会随着熔断（fusing）而改变。与其让 UMD 了解熔断，不如简单地通过现有的查询
引擎信息 IOCTL 暴露逻辑映射。此外，GuC 提交接口目前只支持按逻辑顺序向引擎提交多个
上下文，这与 execlists 相比是一个新要求。最后，所有当前平台最多只有 2 个引擎实例，
逻辑顺序与 uAPI 顺序相同。这在具有超过 2 个引擎实例的平台上将会改变。

将向 drm_i915_engine_info.flags 添加一个位，指示已返回逻辑实例，并添加一个新字段
drm_i915_engine_info.logical_instance 返回逻辑实例。

### 用于为并行提交配置上下文的 'set_parallel' 扩展


'set_parallel' 扩展为 N 个 BB 的并行提交配置一个槽位。它是必须在使用任何上下文之前
调用的一个设置步骤。请参阅 I915_CONTEXT_ENGINES_EXT_LOAD_BALANCE 或
I915_CONTEXT_ENGINES_EXT_BOND 以获取类似现有示例。一旦一个槽位被配置为并行提交，就
可以调用 execbuf2 IOCTL 在单个 IOCTL 中提交 N 个 BB。最初仅支持 GuC 提交。如果需要，
可以稍后添加 execlists 支持。

添加 I915_CONTEXT_ENGINES_EXT_PARALLEL_SUBMIT 和
drm_i915_context_engines_parallel_submit 到 uAPI 以实现此扩展。


        :functions: i915_context_engines_parallel_submit


### 扩展 execbuf2 IOCTL 以支持在单个 IOCTL 中提交 N 个 BB


配置了 'set_parallel' 扩展的上下文只能在单个 execbuf2 IOCTL 中提交 N 个 BB。这些 BB
要么是 drm_i915_gem_exec_object2 列表中的最后 N 个对象，要么是前 N 个对象（如果设置了
I915_EXEC_BATCH_FIRST）。BB 的数量是隐式的，取决于所提交的槽位以及它被 'set_parallel'
或其他扩展配置的方式。execbuf2 IOCTL 不需要 uAPI 更改。
