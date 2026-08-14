
## AMDGPU - Display Contributions


首先，如果你来到这里，你大概是想对显示代码做一些技术贡献，为此我们要说一声：谢谢你 :)

本页汇总了一些你可以帮忙解决的问题；请记住这是一个静态页面，尝试通过 amd-gfx 邮件列表或某位维护者联系开发者总是个好主意。最后，本页遵循 DRM 创建 TODO 列表的方式；更多信息请查看 'Documentation/gpu/todo.rst'。

## Gitlab issues


用户可以在以下地址报告与 AMD GPU 相关的问题：

- https://gitlab.freedesktop.org/drm/amd

通常我们会给所有新工单加上合适的标签以便筛选问题。如果你能复现任何问题，你可以通过补充更多信息或修复该问题来提供帮助。

Level: diverse

## IGT


`IGT`_ 提供了许多可以在你的 GPU 上运行的集成测试。我们总是希望通过大量测试以提升 CI 中的测试覆盖率。如果你想为显示代码做贡献但不确定哪里是好起点，我们建议你运行所有 IGT 测试，并尝试修复你在自己硬件上看到的任何失败。请记住，该失败可能是 IGT 的问题，也可能是内核的问题；需要逐案分析。

Level: diverse


## Compilation


### Fix compilation warnings


在内核编译中启用 W1 或 W2 警告级别，并尝试修复显示一侧的问题。

Level: Starter

### Fix compilation issues when using um architecture


Linux 有一个用户态 Linux（UML）特性，内核可以被编译到 **um** 架构。为 **um** 编译能从测试角度带来多种好处。我们当前在这一领域有一些需要修复的编译问题。

Level: Intermediate

## Code Refactor


### Add prefix to DC functions to improve the debug with ftrace


Ftrace 调试特性（参见 'Documentation/trace/ftrace.rst'）是开发者在理清某个 bug 时代码路径的绝佳方式。Ftrace 提供了一种过滤机制，在开发者对代码的哪部分可能引发问题有所预感时很有用；因此，如果一组函数有合适的前缀，就很容易创建一个好的过滤器。此外，前缀还能提升栈跟踪的可读性。

DC 代码未遵循某些前缀规则，这使 Ftrace 过滤器更加复杂，并降低了栈跟踪的可读性。如果你想从简单的事情开始为显示做贡献，可以为 DC 函数添加前缀提交补丁。要创建这些前缀，请取目标文件名的一部分作为该文件内所有函数的前缀。可参考 `amdgpu_dm_crtc.c` 和 `amdgpu_dm_plane.c` 作为示例。不过，我们强烈建议不要发送大幅改动这些前缀的补丁，否则将难以审阅和测试，可能导致维护者产生顾虑。请采取小步前进的方式；若有疑问，你可以在投入精力之前先询问。我们建议先查看 dceXYZ、dcnXYZ、basics、bios、core、clk_mgr、hwss、resource 和 irq 等目录。

Level: Starter

### Reduce code duplication


AMD 拥有 amdgpu 支持的各种 dGPU 与 APU 的庞大产品组合。为了保持新硬件的发布节奏，DCE/DCN 被设计成模块化结构，使新硬件的引入（bring-up）更快速。多年来，amdgpu 在代码重复方面累积了一些技术债务。对于该任务，寻找一个能发现代码重复（包括模式）的工具并利用它作为减少重复的指引会是个好主意。

Level: Intermediate

### Make atomic_commit_[check|tail] more readable


负责 atomic commit 和 tail 的函数错综复杂且篇幅很长。特别是 `amdgpu_dm_atomic_commit_tail` 是一个很长的函数，如果能拆分成更小的辅助函数会更好。这一领域的改进非常受欢迎，但请记住此处的改动会影响所有 ASIC，这意味着重构需要全面的验证；换句话说，这项工作需要花费一些时间来验证。

Level: Advanced

## Documentation


### Expand kernel-doc


许多 DC 函数没有合适的 kernel-doc；理解一个函数并为其补充文档，是更多地了解 amdgpu 驱动，同时也为整个社区留下杰出贡献的好方法。

Level: Starter

## Beyond AMDGPU


AMDGPU 提供了一些在用户空间尚未启用的特性。本节重点介绍了一些最酷的显示特性，它们可以通过用户空间开发者的协助来启用。

### Enable underlay


AMD 显示有一个称为 underlay（底层叠加）的特性（你可以在 'Documentation/gpu/amdgpu/display/mpo-overview.rst' 中阅读更多内容），它用于在播放视频时节省功耗。其基本思路是把视频放在底部的 underlay 平面上，把桌面放在其上方的平面中并在视频区域挖一个洞。该特性已在 ChromeOS 中启用，根据我们的数据测量，它可以节省功耗。

Level: Unknown

### Adaptive Backlight Modulation (ABM)


ABM 是一个根据所显示图像调节显示面板背光级别和像素值的特性。在系统开始使用电池供电时，这个节能特性会非常有用；由于这会影响显示输出的保真度，最好让用户可以自行开启或关闭该选项。

Level: Unknown


### HDR & Color management & VRR


HDR、色彩管理和 VRR 都是很大的话题，很难把它们浓缩成简洁的 TODO。如果你对此话题感兴趣，我们建议查看社区开发者的一些博客文章，以更好地理解其中的具体挑战和从事该主题的人员。如果有人想着手某个特定部分，我们可以尝试提供一些基础指引。最后请记住，我们在这些领域已经有一些现成的 kernel-doc 了。

Level: Unknown
