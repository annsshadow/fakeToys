## GPU RFC 章节


对于复杂的工作，尤其是新uapi，通常最好先确定高层
设计问题，以免陷入代码细节。本节用
承载此类文档

- 每个 RFC 应作为本文件中的一个小节，说明目标与主要设
  考量。特别是对于 uapi，请确保Cc 抄送给所有相关的项目
  以及 dri-devel 之外涉及的相关人员

- 对于 uapi 结构体，请向本目录添加一个文件，然后像真正的 uapi 头文件那样将
  像真正的 uapi 头文件那样将 kerneldoc 引入

- 代码合入后，请将所有文档移至主核心、辅助或驱动章节的相应位置，位于
  主核心、辅助或驱动章节的相应位置

- [gpusvm.rst](gpusvm.rst)

- [i915_gem_lmem.rst](i915_gem_lmem.rst)

- [i915_scheduler.rst](i915_scheduler.rst)

- [i915_small_bar.rst](i915_small_bar.rst)

- [i915_vm_bind.rst](i915_vm_bind.rst)

- [color_pipeline.rst](color_pipeline.rst)

