## I915 Small BAR RFC 章节

DG2 开始，我们将为设备本地内存（即 I915_MEMORY_CLASS_DEVICE）提供可调整大小BAR 支持，但在某些情况下，最终的 BAR 大小可能仍然小于总的 probed_size。在这种情况下，只有 I915_MEMORY_CLASS_DEVICE 的一部分可被 CPU 访问（例如前 256M），其余部分只能通过 GPU 访问
### I915_GEM_CREATE_EXT_FLAG_NEEDS_CPU_ACCESS 标志

新的 gem_create_ext 标志，用于告诉内核某BO 将需CPU 访问。当将对象放置在 I915_MEMORY_CLASS_DEVICE 中时这一点很重要，因为底层设备的 BAR 较小，意味着其中只有一部分可被 CPU 访问。如果没有该标志，内核会假定不需CPU 访问，并优先使用 I915_MEMORY_CLASS_DEVICE 中不可被 CPU 看到的部分
   :functions: __drm_i915_gem_create_ext

### probed_cpu_visible_size 属
新的 struct __drm_i915_memory_region 属性，返回特定区域中可CPU 访问部分的总大小。这应仅适用I915_MEMORY_CLASS_DEVICE。我们同时报unallocated_cpu_visible_size unallocated_size
Vulkan 需要此属性，作为创建带有 VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT 标志的独VkMemoryHeap 的一部分，以表示 CPU 可见的部分，其中需要知道堆的总大小。它还需要能够大致估计内存可能被分配的情况
   :functions: __drm_i915_memory_region_info

### 错误捕获限制

通过错误捕获我们有两个新的限制：

    1) small BAR 系统上错误捕获是尽力而为的；如果在捕获时页面不可CPU 访问，那么内核可以跳过尝试捕获它们
    2) 在独立式以及较新的集成平台上，我们现在拒绝在可恢复上下文上进行错误捕获。未来内核可能希望在错误捕获期间进行 blit 操作，例如当某个对象当前不可CPU 访问时