
## DRM 子系统的自动化测试


## 简介


当需要测试大量不同的硬件配置时，确保对核心或驱动的修改不会引入回归可能会非常耗时。此外，对于每个有意进行此类测试的人来说，去获取并维护可能相当可观的硬件设备是不切实际的。

同时，开发者最好能够自行检查其代码中的回归，而不是依赖维护者去发现它们再回馈报告。

gitlab.freedesktop.org 上提供了用于自动测试 Mesa 的设施，同样可用于测试 DRM 子系统。本文档说明了有意进行测试的人如何利用这套共享的基础设施，从而节省相当多的时间和精力。


## 相关文件


### drivers/gpu/drm/ci/gitlab-ci.yml


这是 GitLab CI 的根配置文件。除其他不太重要的内容外，它还指定了要使用的脚本的具体版本。有一些变量可以修改以改变流水线的行为：

DRM_CI_PROJECT_PATH
    包含 CI 所用 Mesa 软件基础设施的代码仓库

DRM_CI_COMMIT_SHA
    要从该代码仓库使用的特定修订版本

UPSTREAM_REPO
    包含目标分支的 git 仓库的 URL

TARGET_BRANCH
    本分支将要合并到的目标分支

IGT_VERSION
    所使用的 igt-gpu-tools 的修订版本，来自
    https://gitlab.freedesktop.org/drm/igt-gpu-tools

### drivers/gpu/drm/ci/testlist.txt


要在所有驱动上运行的 IGT 测试（除非在某个驱动的 \*-skips.txt 文件中有所提及，见下文）。

### drivers/gpu/drm/ci/${DRIVER_NAME}-${HW_REVISION}-fails.txt


列出某个驱动在某一特定硬件修订版本上的已知失败项。

### drivers/gpu/drm/ci/${DRIVER_NAME}-${HW_REVISION}-flakes.txt


列出某个驱动在某一特定硬件修订版本上已知行为不可靠的测试。无论结果如何，这些测试都不会导致作业失败。它们仍会被运行。

每个新的 flake 条目都必须关联一个指向邮件的链接，该邮件向受影响驱动的作者或相关 GitLab issue 报告了该缺陷。该条目还必须包含板卡名称或设备树名称、首个受影响的内核版本、用于测试的 IGT 版本，以及失败率的近似值。

```

  # Bug Report: $LORE_URL_OR_GITLAB_ISSUE
  # Board Name: broken-board.dtb
  # Linux Version: 6.6-rc1
  # IGT Version: 1.28-gd2af13d9f
  # Failure Rate: 100
  flaky-test

```
使用下方相应的链接来创建一个 GitLab issue：
amdgpu driver: https://gitlab.freedesktop.org/drm/amd/-/issues
i915 driver: https://gitlab.freedesktop.org/drm/i915/kernel/-/issues
msm driver: https://gitlab.freedesktop.org/drm/msm/-/issues
xe driver: https://gitlab.freedesktop.org/drm/xe/kernel/-/issues

### drivers/gpu/drm/ci/${DRIVER_NAME}-${HW_REVISION}-skips.txt


列出某个驱动在某一特定硬件修订版本上不会被运行的测试。这些通常是会因挂起机器、导致 OOM、耗时过长等原因而干扰测试列表运行的测试。


## 如何在你自己的代码树上启用自动化测试


1. 如果你还没有的话，在 https://gitlab.freedesktop.org/ 上创建一个 Linux 代码树

2. 在你的内核仓库的配置中（例如
   https://gitlab.freedesktop.org/janedoe/linux/-/settings/ci_cd），将
   CI/CD 配置文件从 .gitlab-ci.yml 改为
   drivers/gpu/drm/ci/gitlab-ci.yml。

3. 请求被添加到 drm/ci-ok 组，使你的用户拥有在
   https://gitlab.freedesktop.org/drm/ci-ok 上运行 CI 所需的权限

4. 下次你推送到该代码仓库时，你将看到一条 CI 流水线被创建（例如
   https://gitlab.freedesktop.org/janedoe/linux/-/pipelines）

5. 各项作业将会运行，当流水线结束时，除非发现了回归，否则所有作业都应当是绿色的。

6. 流水线中的警告表明，在测试期间检测到了 lockdep
   （参见 Documentation/locking/lockdep-design.rst）问题。


## 如何更新测试期望


如果你对代码的修改修复了某些测试，你将需要从
drivers/gpu/drm/ci/${DRIVER_NAME}_*_fails.txt 中受该修改影响的每个测试平台对应的文件中，
删除一行或多行。


## 如何扩展测试覆盖


如果你的代码修改使得可以运行更多测试（例如通过解决可靠性问题），你可以从 flakes 和/或 skips 列表中移除测试，以及（如果存在已知失败）相应的预期结果。

如果需要更新所使用的 IGT 版本（也许你向其中添加了更多测试），请更新 gitlab-ci.yml 文件顶部的 IGT_VERSION 变量。


## 如何测试你对脚本的修改


为了测试对 drm-ci 仓库中脚本的修改，请将
drivers/gpu/drm/ci/gitlab-ci.yml 中的 DRM_CI_PROJECT_PATH 和 DRM_CI_COMMIT_SHA 变量改为与你的项目分支（例如 janedoe/drm-ci）相匹配。该分支需要位于 https://gitlab.freedesktop.org/。


## 如何在测试中引入外部修复


通常，其他代码树中的回归会阻止对当前被测代码树中本地修改的测试。这些修复会在构建作业期间从目标代码树中一个名为
${TARGET_BRANCH}-external-fixes 的分支自动合并进来。

如果流水线不在合并请求中，并且本地代码树中存在同名的分支，那么该分支中的提交也会被合并进来。


## 如何处理可能宕机的自动化测试实验室


如果某个硬件农场宕机，从而导致本应通过却使流水线失败，可以通过编辑
https://gitlab.freedesktop.org/gfx-ci/lab-status/-/blob/main/lab-status.yml 处的文件，
来禁用所有将被提交到该硬件农场的作业。
