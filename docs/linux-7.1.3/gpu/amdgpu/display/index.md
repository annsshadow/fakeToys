
## drm/amd/display - 显示核心（DC）


AMD 显示引擎与其他操作系统部分共享；因此，我们的显示核心驱动分为两部分：

#. **显示核心（DC）** 包含与操作系统无关的组件。硬件编程与资源管理在此处理。
#. **显示管理器（DM）** 包含与操作系统相关的组件。与 amdgpu 基础驱动及 DRM 的钩子
   在此实现。例如，可以查看 display/amdgpu_dm/ 目录。

### DC 代码验证


在多个操作系统间维护同一套代码库，需要在各仓库间进行大量同步工作，并进行详尽的
验证。在 DC 的案例中，我们维护一棵树来集中来自不同部分的代码。共享仓库与我们的内部
Linux CI 集群有集成测试，并在多种 AMD GPU/APU（主要是较新的 dGPU 与 APU）上运行一套
全面的 IGT 测试。我们的 CI 还会检查开启与关闭 DCN 时 ARM64/32、PPC64/32 与 x86_64/32
的编译。

当我们向上游提交一个新特性或一些补丁时，我们会将它们打包为一个带有前缀
**DC Patches for <DATE>** 的补丁集，该补丁集基于最新的
`amd-staging-drm-next <https://gitlab.freedesktop.org/agd5f/linux>`_ 创建。所有这些补丁
都在如下测试的某个 DC 版本下：

- 确保每个补丁都能编译，并且整个系列在不同的硬件上通过我们的 IGT 测试集。
- 为我们的验证团队准备一个包含这些补丁的分支。如果出现错误，开发者会尽快调试；通常
  只需在该系列中做一次简单的二分（bisect）就能定位到一处坏改动，随之产生两种可能的
  动作：修复问题或丢弃补丁。如果不易修复，则丢弃该坏补丁。
- 最后，在合并该系列之前，开发者会等待几天以收集社区反馈。

需要着重强调的是，测试阶段是我们极其重视的事情，我们绝不会合并任何未通过我们验证的
内容。下面是我们测试集的概览：

#. 手动测试
    - 使用 DP 与 HDMI 的多次热插拔。
    - 通过用户界面进行多种显示配置变更的压测。
    - 验证 VRR 行为。
    - 检查 PSR。
    - 在播放视频时验证 MPO。
    - 测试同时连接两个以上的显示器。
    - 检查挂起/恢复。
    - 验证 FPO。
    - 检查 MST。
#. 自动测试
    - 在支持 DCN 与 DCE 的 GPU 与 APU 集群中运行 IGT 测试。
    - 使用 LTS 发行版中最新的 GCC 与 Clang 进行编译验证。
    - 针对 PowerPC 64/32、ARM 64/32 与 x86 32 进行交叉编译。

在 CI 与手动测试的环境搭建方面，我们通常使用：

#. 最新的 Ubuntu LTS。
#. 在用户空间方面，我们只使用发行版官方包管理器提供的、完全更新的开源组件。
#. 关于 IGT，我们使用上游的最新代码。
#. 大多数手动测试在 GNome 下进行，但我们也使用 KDE。

注意，我们测试团队的某位成员总会以测试报告回复封面信（cover letter）。

### DC 信息


显示流水线（display pipe）负责将渲染好的帧从 GPU 内存（也称为 VRAM、FrameBuffer 等）
“扫描输出（scan out）”到显示器。换句话说，它会：

#. 从内存读取帧信息；
#. 执行所需的变换；
#. 将像素数据发送给接收端设备。

如果你想进一步了解我们的驱动细节，请查看下面的目录：

- [display-manager.rst](display-manager.rst)
- [dcn-overview.rst](dcn-overview.rst)
- [dcn-blocks.rst](dcn-blocks.rst)
- [programming-model-dcn.rst](programming-model-dcn.rst)
- [mpo-overview.rst](mpo-overview.rst)
- [dc-debug.rst](dc-debug.rst)
- [display-contributing.rst](display-contributing.rst)
- [dc-glossary.rst](dc-glossary.rst)
