
######## 面向 ARC 处理器的 Linux 内核


# 其他信息来源


以下是一些可以获取关ARC 处理器及相关开源项目更多信息的资源
- `<https://embarc.org>`_ - ARC 上开源软件的社区门户  寻找相关 FOSS 项目、工具链发布、新闻等内容的良好起点
- `<https://github.com/foss-for-synopsys-dwc-arc-processors>`_ -
  ARC 处理器开源项目所有开发活动的所在地。其中一些项目是各种上游项目的分支，
  在提交到上游项目之前，“进行中的工作”会托管于此。其他项目则Synopsys 开发，
  并作为开源提供给社区ARC 处理器上使用
- `Synopsys ARC 处理器官方网  <https://www.synopsys.com/designware-ip/processor-solutions.html>`_ -
  该站点可获取部分 IP 文档（`Programmer's Reference
  Manual，即 ARC HS 处理PRM
  <https://www.synopsys.com/dw/doc.php/ds/cc/programmers-reference-manual-ARC-HS.pdf>`_)
  以及部分商业工具的免费版本（`Free nSIM
  <https://www.synopsys.com/cgi-bin/dwarcnsim/req1.cgi>`_   `MetaWare Light Edition <https://www.synopsys.com/cgi-bin/arcmwtk_lite/reg1.cgi>`_）  但请注意，访问这些文档和工具都需要注册
# 关于 ARC 处理器可配置性的重要说明


ARC 处理器具有高度可配置性，Linux 支持若干可配置选项。其中一些选项对软件是透明（例如缓存几何结构，有些可以在运行时被探测并相应配置和使用），而另一些则需要在
内核的配置工具（即“make menuconfig”）中显式选择或配置
然而，并非所有可配置选项ARC 处理器运Linux 时都受支持。SoC 设计团队应参ARC HS Databook 中的“Appendix E: Configuration for ARC Linux”以获取可配置性指南
遵循这些指南并预先选择有效的配置选项，对于帮助避SoC 启动（bringup）以及软开发过程中任何不必要的问题至关重要
# ARC 处理器构Linux 内核


ARC 处理器构建内核的过程与任何其他架构相同，可通过两种方式完成
- 交叉编译（Cross-compilation）：在处理器架构不同的开发主机（通常x86_64/amd64）上
  ARC 目标进行编译的过程- 本地编译（Native compilation）：在装有完整开发环境（GNU 工具链、dtc、make 等）  ARC 平台（硬件板卡或 QEMU 之类的模拟器）上ARC 进行编译的过程
两种情况下，都需要主机上最新的 ARC GNU 工具链。Synopsys 提供了可用于此目的的预构工具链发布版本，可从以下位置获取
- Synopsys GNU 工具链发布：
  `<https://github.com/foss-for-synopsys-dwc-arc-processors/toolchain/releases>`_

- Linux 内核编译器集合：
  `<https://mirrors.edge.kernel.org/pub/tools/crosstool>`_

- Bootlin 的工具链集合：`<https://toolchains.bootlin.com>`_

工具链安装到系统后，请确保其“bin”文件夹已加入你`PATH` 环境变量。然后设`ARCH=arc` `CROSS_COMPILE=arc-linux`（或与你安装ARC 工具链前缀相匹配的值），接着照常执行
`make defconfig && make`銆。
这将在内核源码树根目录生成可用的“vmlinux”文件，可用于通过 JTAG 加载到目标系统如果你需要一个可用于 U-Boot 引导加载程序的镜像，请执`make uImage``uImage` 将在 `arch/arc/boot` 文件夹中生成