
## 维护者条目档案（Maintainer Entry Profile

维护者条目档案是对顶层流程文（submitting-patches、提交驱动程序……）的补充，包含
子系设备驱动本地的约定，以及有关补丁
提交流程生命周期的细节。贡献者使用本文档来调其预期并避免常见错误；维护者可以使用这些档跨子系统寻找机会，以在通用实践上达成一致
### 概述


介绍该子系统如何运作。虽MAINTAINERS 告诉贡献哪些文件对应的补丁应发送到哪里，但它并没有传达
有助于开发的其他子系统本地基础设施与机制
需要考虑的示例问题：

- 当补丁被应用到本地树，或合并到上游时，是否有通知- 该子系统是否patchwork 实例？patchwork 状态的变化是否会被通知- 是否有任何机器人CI 基础设施监视邮件列表，或该子系统
  用于把关接受的自动化测试反馈- 有哪些分支会被拉-next- 贡献者应当针对哪个分支提交？
- 指向任何其他维护者条目档案的链接？例如，设备驱动
  可能指向其父子系统的条目。这让贡献者意识到维护  在提交流程链中对其他维护者可能承担的义务
### 提交检查清单补充（Submit Checklist Addendum

列出除通用“submit-checklist”之外，一份补丁要被认为足健康以引起维护者关注所必须满足的强制与建议标准例如：“通过 checkpatch.pl，无错误或警告。通过 $URI 详述的单元测试”
提交检查清单补充也可以包含有关相关硬件规范
状态的细节。例如，该子系统是否要求在考虑补丁之前
提供某个修订版本的已发布规范
### 关键周期日期（Key Cycle Dates

提交者最常见的误解之一是，补丁可以在合并窗口关闭之前的
任何时间发送，并且仍可被考虑纳入下一-rc1。现实情况是大多数补丁需要在合并窗口开启之前，提前linux-next 沉淀（soaking）。向提交者阐明关键日期（-rc 发布周计），
即补丁可能被考虑合并的时间，以及补丁需等待下一-rc 的时间。至少应包括
- 新特性提交的最-rc
  针对下一个合并窗口的新特性提交，其首次发布以被考虑
  应在此时间点之前。在此时间点之后提交的补丁应当明  它们针对的是 NEXT+1 合并窗口，或者应附带充分的理  说明为何应按加急时间表考虑它们。一般准则是，与贡献  约定新特性提交应-rc5 之前出现
- 合并特性的最-rc：合并决策的截止日期

  向贡献者指明，尚未应用的补丁集在此时间点之后将需  等待 NEXT+1 合并窗口。当然，绝没有义务必须接受任何给定的补丁集，
  但如果到此时审查尚未结束，预期是贡献者应等待  为下一个合并窗口重新提交
可选：

- 概述一节中所列开发基线分支被认为准备好接受新提交  第一-rc
### 审查节奏（Review Cadence

导致贡献者焦虑的最大来源之一，是在补丁集发布未收到任何反馈的情况下，应多快去催问（ping）除了规定重新提交前需要等待多久之外，本节也可以指偏好的更新风格，例如重新发送整个系列，或私下发提醒邮件。本节也可以列出该代码区域的审查如何运作以及不直接来自维护者的获取反馈的方法
### 现有档案


目前，现有维护者档案列于此；在不久的将我们可能会希望采取不同的做法
- [../doc-guide/maintainer-profile](../doc-guide/maintainer-profile)
- [../nvdimm/maintainer-entry-profile](../nvdimm/maintainer-entry-profile)
- [../arch/riscv/patch-acceptance](../arch/riscv/patch-acceptance)
- [../process/maintainer-soc](../process/maintainer-soc)
- [../process/maintainer-soc-clean-dts](../process/maintainer-soc-clean-dts)
- [../driver-api/media/maintainer-entry-profile](../driver-api/media/maintainer-entry-profile)
- [../process/maintainer-netdev](../process/maintainer-netdev)
- [../driver-api/vfio-pci-device-specific-driver-acceptance](../driver-api/vfio-pci-device-specific-driver-acceptance)
- [../nvme/feature-and-quirk-policy](../nvme/feature-and-quirk-policy)
- [../filesystems/nfs/nfsd-maintainer-entry-profile](../filesystems/nfs/nfsd-maintainer-entry-profile)
- [../filesystems/xfs/xfs-maintainer-entry-profile](../filesystems/xfs/xfs-maintainer-entry-profile)
- [../mm/damon/maintainer-profile](../mm/damon/maintainer-profile)
