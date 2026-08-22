

## Media 提交者（Committers

### 谁是 Media 提交者？


Media 提交者是一位拥patchwork 访问权限Media Maintainer，其被授予了
鍚?`media-committers <https://gitlab.freedesktop.org/linux-media/media-committers>`_
树推送来自其他开发者及其自身补丁的提交权限
授予这些提交权限是基于对责任的期待：提交者是那些关心 Linux 内核整体以及
Linux 媒体子系统、并希望推进其发展的人。它也建立在其他提交者、维护者以Linux 媒体社区之间的信任关系之上
作为 Media 提交者，你承担以下额外责任：

1. 你创作的补丁必须带有另一Media Maintainer `Signed-off-by`   `Reviewed-by` `Acked-by`2. 如果一个补丁引入了回归，则必须尽快纠正。通常的做法是要么回退该补丁，
   要么提交一个额外的补丁来修复该回归3. 如果补丁修复的是针对已发布内核的缺陷（包括上述的回退），Media 提交   应当添加所需的标签。更多细节请参阅 Media 开发流程4. 所Media 提交者都有责任维   `Patchwork <https://patchwork.linuxtv.org/project/linux-media/list/>`_   更新他们所审阅或合并的补丁的状态

### 成为 Media 提交

现有Media 提交者可以提名一Media Maintainer 以获得提交权限。该 Media
Maintainer 必须拥有 patchwork 访问权限，曾一段时间审阅来自第三方的补丁，已展现出对维护者职责与流程的良好理解
接受被提名提交者的最终责任落Media 子系统维护者身上。被提名的提交者必与所Media 子系统维护者建立信任关系，因为授予你提交权限即意味着他们部分职责被移交给了你
因此，要成为 Media 提交者，需要所Media 子系统维护者之间达成一致

   为了保护那些可能被授予、拒绝或移除提交权限的开发者，以及负有接受   拒绝提交权限任务的子系统维护者，所有与变更提交权限相关的沟通都应尽可能
   在私下进行

### Media 提交者协

一旦被提名的提交者被所Media 子系统维护者接受，他们会询问该开发者是否对
提名感兴趣，并讨论该提交者将负责媒体子系统的哪些领域。这些领域通常与其
本就在维护的领域相同
当开发者接受成为提交者时，新的提交者应当通过media-committers@linuxtv.org
发送一封声明意向的电子邮件，明确接受其 Documentation/ 下描述的 Kernel 开策略，尤其是本文档中的规则：

```
   I, John Doe, would like to change my status to: Committer

   As Media Maintainer I accept commit rights for the following areas of
   the media subsystem:

   ...

   For the purpose of committing patches to the media-committers tree,
   I'll be using my user https://gitlab.freedesktop.org/users/<username>.

```
随后是一份正式声明，表示同意内核开发规则：

```
   I agree to follow the Kernel development rules described at:

   https://www.kernel.org/doc/html/latest/driver-api/media/media-committers.rst

   and to the Linux Kernel development process rules.

   I agree to abide by the Code of Conduct as documented in:
   https://www.kernel.org/doc/html/latest/process/code-of-conduct.rst

   I am aware that I can, at any point of time, retire. In that case, I will
   send an e-mail to notify the Media Subsystem Maintainers for them to revoke
   my commit rights.

   I am aware that the Kernel development rules change over time.
   By doing a new push to media-committers tree, I understand that I agree
   to follow the rules in effect at the time of the commit.

```
该电子邮件应当通过 Kernel Web of trust，使用经其他内核及媒体开发者交叉签PGP 密钥进行签名。如 media-developers-gpg 所述，PGP 签名连同 gitlab 用户
安全性，是确保将media-committers.git 树上发生的合并请求真实性的基本
组成部分
如果内核开发流程发生变更，通过`media-committers tree <https://gitlab.freedesktop.org/linux-media/media-committers>`_
合并新的提交，Media 提交者即隐式声明其同意最新版本的文档化流程，包括本文件的
内容
如果某位 Media 提交者决定退休，及时通知 Media 子系统维护者这一决定该提交者的职责

   1. 对内核媒体开发流程的变更应在 media-committers 邮件列表中公布，并给      合理的审阅期。所有提交者会自动订阅该邮件列表；
   2. 由于内核开发的分布式特性，内核开发流程的变更有可能会最终在 Linux Docs
      Linux Kernel 邮件列表上被审阅/合并，尤其是针对 Documentation/process
      下的内容以及琐碎的拼写修正
### Media Core 鎻愪氦鑰。

Media Core 提交者是一位拥有提交权限的 Media Core Maintainer
Documentation/driver-api/media/maintainer-entry-profile.rst 所述，Media
Core Maintainer 除了驱动之外还维护媒体核心框架，因此被允许修改核心文件以媒体子系统的 Kernel API。Core 提交者权限的范围将由 Media 子系统维护者在提名
Media Core 提交者时详述
现有Media 提交者可以成Media Core 提交者，反之亦然。此类决定将Media
子系统维护者之间协商一致后作出
### Media 提交者规

Media 提交者应尽最大努力避免合并会破坏任何现有驱动的补丁。如果发生了破坏补丁修复或回退应当尽快合并，目标是在报告该缺陷的同一内核周期内完成合并
Media 提交者应按照 Media 子系统维护者所授予的权限行事，特别是在其可以直应用media-committers 树的变更范围方面。该范围可随着 Media 提交者与 Media
子系统维护者之间的相互协议随时间变化
Media 提交者工作流Media 开发流程中描述

### 维持 Media Maintainer Committer 身份


一个共同协作以推动 Linux 内核前进的维护者社区，对于创建成功的、值得投入项目至关重要。如果社区内部存在问题或分歧，通常可以通过健康的讨论与辩论
解决
在不幸的情况下，如果某位 Media Maintainer Committer 持续无视良好的社行为（或主动破坏项目），我们可能需要撤销该人的身份。在此类情况下，如果有人
提出有充分理由的撤销建议，那么在 Media 维护者之间讨论之后，最终决定由 Media
子系统维护者作出
由于成为 Media Maintainer Committer 的决定来Media 子系统维护者之间的
一致意见，只要有一Media 子系统维护者不再信任该 Media Maintainer Committer，就足以撤销其维护者身份、Patchwork 授予或提交权限
提交权限被撤销并不妨碍 Media 维护者继续通过拉取请求或电子邮件工作流向子系统
做出贡献，如 Media 开发流程中所述
如果某位维护者在一个或多个内核周期内处于不活跃状态，维护者会尝试通过电子
邮件联系你。如果无法联系，他们可能会撤销其维护patchwork 和提交者权限，
并相应更MAINTAINERS 文件条目。如果你希望日后作为维护者恢复贡献，请联Media 子系统维护者，询问你的维护者身份、Patchwork 授予和提交权限能否被恢复
### 参考资

本文档大量受以下项目的提交者策略启复制而来
- `Chromium <https://chromium.googlesource.com/chromium/src/+/main/docs/contributing.md>`_锛?- `WebKit <https://webkit.org/commit-and-review-policy/>`_锛?- `Mozilla <https://www.mozilla.org/hacking/committer/>`_銆。