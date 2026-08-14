
## 媒体子系统概况


### 概述


Linux 媒体社区（又称：LinuxTV 社区）由从事 Linux 内核媒体子系统开发的开发者，以及同样在测试代码方面发挥重要作用的用户组成。

媒体子系统包含用于支持各种媒体相关设备的代码：流捕获、模拟与数字电视流、摄像头、视频编解码器、视频处理（缩放器等）、无线电、遥控器、HDMI CEC 以及媒体流水线控制。

媒体子系统在内核树中由以下目录组成：

  - drivers/media
  - drivers/staging/media
  - include/media
  - Documentation/devicetree/bindings/media/\ [^1^]_
  - Documentation/admin-guide/media
  - Documentation/driver-api/media
  - Documentation/userspace-api/media

       OPEN FIRMWARE AND FLATTENED DEVICE TREE BINDINGS 的维护者
       （参见 MAINTAINERS 文件）。因此，在那里的变更在被合并进媒体子系统的
       开发树之前，必须先由他们审查。

媒体用户空间 API 与内核 API 都已形成文档，且文档必须与 API 变更保持同步。这意味着所有为该子系统添加新特性的补丁也必须同时带来相应 API 文档的变更。

### 媒体维护者


媒体维护者不仅仅是能够编写代码的人，更是那些展现出与团队协作能力、能够让最有经验的专家来审查代码、贡献高质量代码，并跟进修复问题（代码或测试中的）的开发者。

由于媒体子系统规模庞大、涉及面广，需要多层次的维护者，每一位都有其各自的专长领域：

- **媒体驱动维护者（Media Driver Maintainer）**：
    负责媒体子系统中的一个或多个驱动。他们在 MAINTAINERS 文件中被列为这些驱动的维护者。媒体驱动维护者审查这些驱动的补丁，如果补丁不遵循子系统规则、未正确使用媒体内核或用户空间 API，或代码质量较差，则提供反馈。

    如果你是补丁作者，你需要与其他媒体维护者协作，以确保你的补丁得到审查。

    一些媒体驱动维护者还有额外职责。他们已被授予 Patchwork 访问权限，并维持
    `Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
    为最新状态，决定补丁何时可以合并，并为媒体子系统维护者创建拉取请求（Pull Request）以进行合并。

- **媒体核心维护者（Media Core Maintainer）**：
    拥有 Patchwork 访问权限、并且还负责一个或多个媒体核心框架的媒体驱动维护者。

    核心框架的变更由相关的媒体核心维护者通过共识（consensus）完成。如果相关媒体核心维护者已签字认可（sign off），媒体维护者可以在其拉取请求中包含核心框架的变更。

- **媒体子系统维护者（Media Subsystem Maintainers）**：
    同时也对整个子系统负责、能够访问整个子系统的媒体核心维护者。负责合并来自其他媒体维护者的拉取请求。

    用户空间 API/ABI 的变更由媒体子系统维护者之间通过共识做出\ [^2^]_。如果相关媒体子系统维护者全部签字认可，媒体维护者可以在其拉取请求中包含 API/ABI 变更。

所有媒体维护者都应同意 Documentation/process/index.rst 中描述的内核开发流程，以及内核文档中的内核开发规则，包括其行为准则（code of conduct）。

媒体维护者通常可以通过 OFTC 上的 #linux-media IRC 频道联系到。

       非内核代码的也属于 API/ABI 变更。这包括 ioctl 与 sysfs 接口、v4l2
       控件，以及它们的行为。

### Patchwork 访问权限


所有被授予 Patchwork 访问权限的媒体维护者都应确保
`Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
反映当前状态，例如补丁应被委派给正在处理它的媒体维护者，并且补丁状态应根据以下规则更新：

- `Under Review`（审查中）：当补丁需要第二种意见，或当它属于某个拉取请求的一部分时使用；
- `Superseded`（已被取代）：邮件列表上发布了该补丁的更新版本。
- `Duplicated`（重复的）：有其他人做了同样事情的另一份补丁被接受了。
- `Not Applicable`（不适用）：用于那些不会被合并到 media.git 树（例如 drm、dmabuf、上游合并等）但被交叉发布到 linux-media 邮件列表的补丁系列。
- `Accepted`（已接受）：一旦补丁被合并进多提交者（multi-committer）树。只有拥有提交权的媒体维护者才能设置此状态。

如果媒体维护者决定不接受某个补丁，他们应通过电子邮件回复补丁作者，解释不被接受的原因，并相应地将
`Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
更新为以下状态之一：

- `Changes Requested`（已请求变更）：如果请求了新的修订版本；
- `Rejected`（已拒绝）：如果所提议的变更完全不可接受。

   Patchwork 支持几个客户端，可借助其 REST 接口帮助半自动化地更新状态：

   https://patchwork.readthedocs.io/en/latest/usage/clients/

对于属于其职责范围内的补丁，媒体维护者还决定这些补丁何时可以合并，并为媒体子系统维护者创建拉取请求以进行合并。

成为拥有 Patchwork 访问权限的媒体维护者，最重要的方面在于你已经展现出了给出良好代码审查的能力。我们重视你提供透彻、有建设性的代码审查的能力。

因此，潜在的维护者必须从 Linux 媒体社区赢得足够的信誉与信任。为此，开发者应当熟悉开源模式，并在 Linux 内核社区（尤其是媒体子系统）中活跃一段时间。

除了实际进行代码变更之外，你基本上是在证明你的：

- 对项目的投入；
- 与团队协作出良好沟通的能力；
- 对上游以及 Linux 媒体社区如何运作的理解
  （策略、测试流程、代码审查……）
- 对以下内容的合理了解：

  - 内核开发流程：
    Documentation/process/index.rst

  - 媒体开发概况：
    Documentation/driver-api/media/maintainer-entry-profile.rst

- 对项目代码库与编码风格的理解；
- 向补丁作者提供反馈的能力；
- 判断一个补丁何时可能准备好审查并提交的能力；
- 编写良好代码的能力（最后但绝非最不重要）。

希望获得 Patchwork 访问权限的媒体驱动维护者，鼓励参加每年一度的 Linux 媒体峰会（Linux Media Summit），该峰会通常与某个 Linux 相关的会议同期举行。这些峰会在 linux-media 邮件列表上公布。

如果你正在做这类工作并已成为一名受重视的开发者，一位现有的媒体维护者可以将你提名为媒体子系统维护者。

接受被提名维护者的最终责任在于子系统的维护者。被提名的维护者必须与所有媒体子系统维护者建立起信任关系，因为一旦被授予 Patchwork 访问权限，你将接管他们的一部分维护工作。

### 媒体提交者


经验丰富且受信任的媒体维护者可能会被授予提交权（commit rights），允许他们直接将补丁推送到媒体开发树，而无需为媒体子系统维护者发布拉取请求。这有助于减轻媒体子系统维护者的一些工作。

关于媒体提交者角色与职责的更多细节可在此处找到：Media Committers。

### 媒体开发站点


`LinuxTV <https://linuxtv.org/>`_ 网站托管着关于该子系统的新闻，以及：

- `Wiki 页面 <https://www.linuxtv.org/wiki/index.php/Main_Page>`_；
- `Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_；
- `Linux Media 文档 <https://linuxtv.org/docs.php>`_；
- 等等。

媒体子系统使用的主要开发树位于：

- 稳定树（Stable tree）：
  - https://git.linuxtv.org/media.git/

- 媒体提交者树（Media committers tree）：
  - https://gitlab.freedesktop.org/linux-media/media-committers.git

    请注意它可能会被变基（rebase），尽管只作为最后手段。

- 媒体开发树，包括应用与 CI：

  - https://git.linuxtv.org/
  - https://gitlab.freedesktop.org/linux-media/



媒体开发流程
++++++++++++++++++++++++++

媒体子系统的所有变更都应首先作为电子邮件发送到媒体邮件列表，遵循
Documentation/process/index.rst 中记录的流程。

这意味着补丁应仅以纯文本形式通过电子邮件提交到 linux-media@vger.kernel.org（又称：LMML）。虽然订阅不是强制的，但你可以从以下位置找到如何订阅以及查看其归档的详细信息：

  https://subspace.kernel.org/vger.kernel.org.html

包含 HTML 的电子邮件会被邮件服务器自动拒绝。

明智的做法是同时抄送（copy）相关的媒体维护者。你应该使用 `scripts/get_maintainers.pl` 来确定还需要抄送谁。请始终抄送驱动的作者与维护者。

为了尽量减少你的补丁系列产生合并冲突的机会，并使其更容易向后移植（backport）到稳定内核，我们建议你为你的补丁系列使用以下基线：

1. 面向下一个主线（mainline）版本的特性的补丁：

   - 基线应为 `media-committers.git next` 分支；

2. 面向下一个主线版本的缺陷修复：

   - 基线应为 `media-committers.git next` 分支。如果变更依赖于
     `media-committers.git fixes` 分支中的一个修复，那么你可以用它作为基线。

3. 面向当前主线版本（-rcX）的缺陷修复：

   - 基线应为最新的主线 -rcX 版本，或者如果变更依赖于一个尚未合并的主线
     修复，则为 `media-committers.git fixes` 分支；


   有关内核发布类型的概述，请参见 https://www.kernel.org/category/releases.html。

带有修复的补丁应带有：

- 一个指向引入该缺陷的第一个提交的 `Fixes:` 标签；
- 在适用时，一个 `Cc: stable@vger.kernel.org` 标签。

由 linux-media@vger.kernel.org 邮件列表上某人公开报告的缺陷修复补丁应带有：

- 一个 `Reported-by:` 标签，其后紧跟一个 `Closes:` 标签。

变更 API 的补丁应在同一补丁系列中相应地更新文档。

有关电子邮件提交的更多细节，请参见 Documentation/process/index.rst。

一旦提交了补丁，它可能遵循以下两种流程之一：

a. 媒体维护者流程：媒体维护者发布拉取请求，
```

     +-------+   +------------+   +------+   +-------+   +---------------------+
     |e-mail |-->|picked up by|-->|code  |-->|pull   |-->|Subsystem Maintainers|
     |to LMML|   |Patchwork   |   |review|   |request|   |merge in             |
     |       |   |            |   |      |   |       |   |media-committers.git |
     +-------+   +------------+   +------+   +-------+   +---------------------+

   对于此流程，拉取请求由拥有 Patchwork 访问权限的媒体维护者生成。如果你没有
   Patchwork 访问权限，请不要提交拉取请求，因为它们不会被处理。

```
b. 媒体提交者流程：补丁由拥有 Patchwork 访问权限的媒体维护者处理
```

     +-------+   +------------+   +------+   +--------------------------+
     |e-mail |-->|picked up by|-->|code  |-->|Media Committers merge in |
     |to LMML|   |Patchwork   |   |review|   |media-committers.git      |
     +-------+   +------------+   +------+   +--------------------------+

```
当补丁被
`Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
接收，并在 media-committers 合并时，媒体 CI 机器人将检查错误，并可能就补丁问题提供电子邮件反馈。发生这种情况时，补丁提交者必须修复它们，或解释为何这些错误是误报（false positive）。

只有在通过媒体 CI，或者媒体 CI 报告中存在误报的情况下，补丁才会在这两种流程中被推进到下一阶段。

对于两种流程，所有补丁在合并进 `media-committers.git` 之前，都应在
linux-media@vger.kernel.org（LMML）上得到恰当的审查。媒体补丁将由 MAINTAINERS 文件中列出的维护者与审查者及时审查。

媒体维护者应在适用时请求其他媒体维护者与开发者进行审查，即因为那些开发者对被补丁所变更的部分有更多了解。

不应存在来自任何人的未决问题，或未解决、相互冲突的反馈。先将其澄清。如有需要，交由媒体子系统维护者裁决。

邮件提交过程中的失败
+++++++++++++++++++++++++++++++++

媒体的工作流程严重依赖 Patchwork，这意味着一旦提交了补丁，该电子邮件将首先被邮件列表服务器接受，过一段时间后，它应出现在：

   - https://patchwork.linuxtv.org/project/linux-media/list/

如果一段时间后它未自动出现在那里 [^3^]_，那么很可能是你的提交出了问题。在抱怨或再次提交之前，请检查电子邮件是否仅为纯文本\ [^4^]_，以及你的邮件客户端是否没有破坏空白字符。

要排查问题，你应首先通过查看以下位置，检查邮件列表服务器是否已接受你的补丁：

   - https://lore.kernel.org/linux-media/

如果补丁在那里而不在
`Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
，很可能是你的邮件客户端破坏了补丁。Patchwork 内部有逻辑来检查收到的电子邮件是否包含有效的补丁。任何破坏补丁的空白字符与换行破坏都不会被
`Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
识别，这样的补丁会被拒绝。

       邮件服务器可能正忙，因此
       `Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_
       接收一份补丁可能需要更长时间。

       直接丢弃它，不做任何进一步通知。

拉取请求与合并请求的认证
++++++++++++++++++++++++++++++++++++++++++

提交拉取请求与合并请求的开发者的真实性，应使用 Linux 内核信任网络（Web of Trust）通过 PGP 签名在某个时刻进行验证。参见：kernel_org_trust_repository。

对于拉取请求流程，拉取请求应使用 PGP 签名的标签（tag）。

对于提交者流程，这一点在授予合并请求权限给 media-committers.git 树所使用的 gitlab 实例时得到保证，在该实例收到
media-committer-agreement 中记录的电子邮件之后。

有关 PGP 签名的更多细节，请阅读
Documentation/process/maintainer-pgp-guide.rst。

### 维护媒体维护者身份


参见 Maintain Media Status。

### 媒体维护者名单


此处列出的媒体维护者都拥有 Patchwork 访问权限，可以创建拉取请求或拥有提交权。

媒体子系统维护者为：
  - Mauro Carvalho Chehab <mchehab@kernel.org>
  - Hans Verkuil <hverkuil@kernel.org>

媒体核心维护者为：
  - Sakari Ailus <sakari.ailus@linux.intel.com>

    - 媒体控制器驱动
    - 核心媒体控制器框架
    - ISP
    - 传感器驱动
    - v4l2-async 与 v4l2-fwnode 核心框架
    - v4l2-flash-led-class 核心框架

  - Mauro Carvalho Chehab <mchehab@kernel.org>

    - DVB

  - Laurent Pinchart <laurent.pinchart@ideasonboard.com>

    - 媒体控制器驱动
    - 核心媒体控制器框架
    - ISP

  - Hans Verkuil <hverkuil@kernel.org>

    - V4L2 驱动
    - V4L2 与 videobuf2 核心框架
    - HDMI CEC 驱动
    - HDMI CEC 核心框架

  - Sean Young <sean@mess.org>

    - 遥控器（红外）驱动
    - 遥控器（红外）核心框架

负责特定领域的媒体驱动维护者为：
  - Nicolas Dufresne <nicolas.dufresne@collabora.com>

    - 编解码器驱动
    - 未被另行委派的 M2M 驱动

  - Bryan O'Donoghue <bryan.odonoghue@linaro.org>

    - Qualcomm 驱动

### 提交检查清单补充


变更 Open Firmware/Device Tree 绑定的补丁必须由 Device Tree 维护者审查。因此，当这些补丁通过 devicetree@vger.kernel.org 邮件列表提交时，应抄送（Cc）DT 维护者。

在 https://git.linuxtv.org/v4l-utils.git/ 有一组合规工具，应用于检查驱动是否正确实现了媒体 API：

====================	=======================================================
Type			Utility
====================	=======================================================
V4L2 drivers\ [^5^]_	`v4l2-compliance`
V4L2 virtual drivers	`contrib/test/test-media`
CEC drivers		`cec-compliance`
====================	=======================================================

       位于 V4L2 驱动内部。

这些测试必须在补丁进入上游之前通过。

```

	make CF=-D__CHECK_ENDIAN__ CONFIG_DEBUG_SECTION_MISMATCH=y C=1 W=1 CHECK=check_script

```
```

	#!/bin/bash
	/devel/smatch/smatch -p=kernel $@ >&2
	/devel/sparse/sparse $@ >&2

```
请确保不要在没有充分理由的情况下在补丁中引入新的警告。

有关电子邮件提交规则，请参见 `Media development workflow`_。

代码风格清理补丁
+++++++++++++++++++++

当风格清理与将受影响文件中的其他变更一起进行时，我们欢迎这种清理。

我们也可能接受纯独立的风格清理，但它们理想情况下应为整个子系统的一份补丁（如果清理量较小），或至少按目录分组。因此，例如，如果你正在对 drivers/media 下的驱动做一处大的清理变更集，请为 drivers/media/pci 下的所有驱动发送单份补丁，为 drivers/media/usb 发送另一份，依此类推。

编码风格补充
+++++++++++++++++++++

媒体开发使用严格模式（strict mode）下的 `checkpatch.pl` 来验证代码
```

	$ ./scripts/checkpatch.pl --strict --max-line-length=80

```
原则上，补丁应遵循编码风格规则，但如果有充分理由，允许例外。在这种情况下，维护者与审查者可能会质疑未处理 `checkpatch.pl` 的理由。

请注意，这里的目标是提高代码可读性。在少数情况下，`checkpatch.pl` 实际上可能指向看起来更糟的内容。因此，你应当善用判断力。

请注意，单独处理一个 `checkpatch.pl` 问题（任何种类）可能导致行宽超过每行 80 字符。虽然这并非严格禁止，但应努力将每行控制在 80 字符以内。这可以包括使用重构代码以减少缩进、使用更短的变量或函数名，以及——最后但同样重要的——简单地换行。

特别是，我们接受超过 80 列的行：

    - 在字符串上，因为它们不应因行长度限制而被断开；
    - 当函数或变量名需要较长的标识符名称，从而难以遵守 80 列限制时；
    - 在算术表达式上，当断开行会使它们更难阅读时；
    - 当它们避免了以开括号或开方括号结尾的行时。

### 关键周期日期


新的提交可以在任何时候发送，但如果它们打算进入下一个合并窗口（merge window），就应在 -rc5 之前发送，并理想情况下在 -rc6 之前于 linux-media 分支中稳定下来。

### 审查节奏


只要你的补丁已进入
`Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_，
它迟早会得到处理，因此你无需重新提交补丁。

除了重要的缺陷修复之外，我们通常不会在 -rc6 与下一个 -rc1 之间向开发树添加新补丁。

请注意，媒体子系统是一个高流量的子系统，因此我们可能需要一段时间才能审查你的补丁。如果你在几周内没有得到反馈，或者想请其他开发者公开添加 `Reviewed-by:` 标签，更重要的是 `Tested-by:` 标签，请随时 ping 我们。

请注意，我们期望 `Tested-by:` 有详细的描述，指明测试期间使用了哪些开发板（board）以及测试了什么。
