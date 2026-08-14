
## SoC Subsystem


### Overview


SoC 子系统是 SoC 特定代码的聚合之处。该子系统的主要组成部分是：

- 32 位和 64 位 ARM 以及 RISC-V 的设备树（DTS）
- 32 位 ARM 板文件（arch/arm/mach*）
- 32 位和 64 位 ARM 的 defconfig
- 跨架构的 SoC 特定驱动，特别是 32 位和 64 位 ARM、RISC-V 和 Loongarch 的驱动

这些“SoC 特定驱动”不包括有时钟、GPIO 等其他顶级维护者的驱动。drivers/soc/ 目录通常
用于内核内部驱动，这些驱动被其他驱动用来提供 SoC 特定功能，如识别 SoC 版本或与电源域
交互。

SoC 子系统也作为对 drivers/bus、drivers/firmware、drivers/reset 和 drivers/memory 的
变更的中转位置。新平台的加入或现有平台的移除，通常作为覆盖多个子系统的专门分支，经过 SoC
树进行。

主 SoC 树托管在 git.kernel.org 上：
  https://git.kernel.org/pub/scm/linux/kernel/git/soc/soc.git/

### Maintainers


显然，这是一个相当广泛的话题范围，没有任何一个人，甚至一小群人能够维护。相反，SoC 子系统
由许多子维护者（平台维护者）组成，各自负责单个平台和驱动子目录。在这方面，“平台”通常指
某个给定厂商的一系列 SoC，例如 Nvidia 的 Tegra SoC 系列。许多子维护者在厂商层面运作，负责
多个产品线。由于多种原因（包括公司内的收购/不同业务单元），这里的情况差异很大。各个子维护者
记录在 MAINTAINERS 文件中。

这些子维护者大多有自己的树，他们在其中暂存补丁，并向主 SoC 树发送拉取请求（pull request）。
这些树通常（但不总是）列在 MAINTAINERS 中。

然而，SoC 树并非架构特定代码变更的位置。每种架构都有自己的维护者，负责架构细节、CPU 勘误
（errata）等。

#### Submitting Patches for Given SoC


所有典型的平台相关补丁都应通过 SoC 子维护者（平台特定维护者）发送。这也包括对每个平台或
共享 defconfig 的更改。注意 scripts/get_maintainer.pl 可能无法为共享 defconfig 提供正确的
地址，因此请忽略其输出，并根据 MAINTAINERS 文件手动创建抄送（CC）列表，或使用类似
`scripts/get_maintainer.pl -f drivers/soc/FOO/`）的方法。

#### Submitting Patches to the Main SoC Maintainers


主 SoC 维护者仅在以下情况下可通过别名 soc@kernel.org 联系：

1. 没有平台特定维护者。

2. 平台特定维护者无响应。

3. 引入一个全新的 SoC 平台。此类新 SoC 工作应首先发送到由 scripts/get_maintainer.pl 指出的
   公共邮件列表，进行社区评审。在获得积极的社区评审后，工作应作为一个补丁集发送到
   soc@kernel.org，其中包含新的 arch/foo/Kconfig 条目、DTS 文件、MAINTAINERS 文件条目，以及
   可选的初始驱动及其 Devicetree 绑定。MAINTAINERS 文件条目应列出新的平台特定维护者，他们将
   从此负责处理该平台的补丁。

注意，soc@kernel.org 通常不是讨论补丁的地方，因此发送到该地址的工作应已被社区认为可接受。

### Information for (new) Submaintainers


随着新平台的出现，它们往往带来新的子维护者，其中许多人供职于芯片厂商，可能不熟悉该流程。

#### Devicetree ABI Stability


也许最需要强调的一点之一是，dt-bindings 记录了设备树与内核之间的 ABI。请阅读
Documentation/devicetree/bindings/ABI.rst。

如果对 DTS 的更改与旧内核不兼容，在该驱动合入之前（或之后适当的时机）不应应用该 DTS 补丁。
最重要的是，任何不兼容的更改都应在补丁说明和拉取请求中明确指出来，同时说明对现有用户
（如 bootloader 或其他操作系统）的预期影响。

#### Driver Branch Dependencies


一个常见问题是协调设备驱动与设备树文件之间的更改。即使某个更改在两个方向上都兼容，这可能也
需要协调这些更改如何通过不同的维护者树合入。

通常包含驱动更改的分支也会包含对设备树绑定描述的相应更改，以确保它们确实兼容。这意味着设备
树分支最终可能在 `make dtbs_check` 步骤中产生警告。如果设备树更改依赖于 include/dt-bindings/
中某个头文件缺失的新增内容，它将无法通过 `make dtbs` 步骤而不会被合入。

有多种方式处理此问题：

- 避免在 include/dt-bindings/ 中为可从数据手册推导出的硬件常量定义自定义宏——头文件中的
  绑定宏只应作为最后手段使用，当没有自然方式定义绑定时才使用

- 在设备树文件中使用字面值代替宏，即使在需要头文件的情况下，并在后续版本中将其改为具名
  表示

- 将设备树更改推迟到绑定和驱动已经合入之后的版本

- 在一个共享的不可变分支中更改绑定，该分支同时作为驱动更改和设备树更改的基础

- 在设备树文件中添加由 #ifndef 段保护的重复定义，并在后续版本中移除它们

#### Devicetree Naming Convention


设备树文件的一般命名方案如下。在 SoC 级别设定的平台方面（如 CPU 核）包含在名为 $soc.dtsi
的文件中，例如 jh7100.dtsi。因板而异的集成细节在 $soc-$board.dts 中描述。一个例子是
jh7100-beaglev-starlight.dts。通常许多板是同一主题上的变体，并且经常有介于 $soc.dtsi 和
$soc-$board.dts 文件之间的中间文件，如 jh7100-common.dtsi，包含公共硬件的描述。

一些平台还有系统模块（System on Module），其中包含 SoC，然后被集成到多个不同的板中。对于
这些平台，典型的是 $soc-$som.dtsi 和 $soc-$som-$board.dts。

目录通常以纳入时 SoC 厂商的名字命名，这导致树中存在一些历史目录名。

#### Validating Devicetree Files


`make dtbs_check` 可用于验证设备树文件是否符合描述 ABI 的 dt-bindings。有关设备树验证的
更多信息，请阅读 Documentation/devicetree/bindings/writing-schema.rst 的“运行检查”一节。

对于新平台或对现有平台的添加，`make dtbs_check` 不应增加任何新的警告。对于 RISC-V 和
Samsung SoC，要求 `make dtbs_check W=1` 不增加任何新的警告。

#### Branches and Pull Requests


正如主 SoC 树有多个分支一样，期望子维护者也这样做。驱动、defconfig 和设备树的更改都应拆分
为独立分支，并出现在发给 SoC 维护者的独立拉取请求中。每个分支本身应当可用，并避免源自对其他
分支依赖的回归。

小的补丁集也可以作为独立邮件发送到 soc@kernel.org，按相同的类别归类。

如果更改不适合常规模式，可以有额外的顶级分支，例如用于全树重构，或添加新的 SoC 平台（包括
dts 文件和驱动）。

具有大量更改的分支可受益于拆分为独立的主题分支，即使它们最终被合入 SoC 树的同一个分支。这里
的一个例子是：一个分支用于设备树警告修复，一个用于重构，一个用于新添加的板。

另一种常见的拆分更改方式是，在 rc1 到 rc4 之间的某个时间点发送一个包含大部分更改的早期拉取
请求，随后在周期接近尾声时发送一个或多个较小的拉取请求，以添加后期更改或解决测试第一组时
发现的问题。

虽然没有针对后期拉取请求的截止时间，但随着时间越来越接近合并窗口，只发送小分支会更有帮助。

针对当前版本缺陷修复的拉取请求可以随时发送，但同样，拥有多个较小的分支优于试图将过多补丁
合并到一个拉取请求中。

拉取请求的主题行应以“[GIT PULL]”开头，并使用带签名的标签（tag）而非分支来创建。该标签应包含
一段简短描述，总结拉取请求中的更改。有关发送拉取请求的更多细节，请参阅
Documentation/maintainer/pull-requests.rst。

#### Defconfigs purpose


defconfig 主要被内核开发者使用，因为发行版有自己的配置。向 defconfig 添加新 CONFIG 选项的更改
应解释为什么内核开发者总体上会需要这样的选项，例如提供使用该新选项的某个上游支持的
机器/板的名称。这意味着不应接受为未上游机器在 defconfig 中启用选项。
