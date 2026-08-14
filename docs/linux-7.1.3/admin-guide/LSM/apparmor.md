## AppArmor


## 什么是 AppArmor？


AppArmor 是 Linux 内核的 MAC（强制访问控制）风格安全扩展。它实现了一个以任务为中心的策略，任务的“配置文件”从用户空间创建并加载。系统上没有为其定义配置文件的任务以无约束（unconfined）状态运行，这等同于标准 Linux DAC 权限。

## 如何启用/禁用


设置 `CONFIG_SECURITY_APPARMOR=y`

```
   CONFIG_DEFAULT_SECURITY_APPARMOR=y
```
CONFIG_LSM 参数管理 LSM 的顺序和选择。在列表中将 apparmor 指定为第一个“主要”模块（例如 AppArmor、SELinux、Smack）。

构建内核

如果 AppArmor 不是默认安全模块，可以通过在内核命令行上传递 `security=apparmor` 来启用。

如果 AppArmor 是默认安全模块，可以通过在内核命令行上传递 `apparmor=0, security=XXXX`（其中 `XXXX` 是有效的安全模块）来禁用。

为了让 AppArmor 强制执行超出标准 Linux DAC 权限之外的任何限制，必须将策略从用户空间加载到内核中（请参阅下方的文档和工具链接）。

## 文档


文档可以在下方链接的 wiki 中找到。

## 链接


邮件列表 - apparmor@lists.ubuntu.com

Wiki - http://wiki.apparmor.net

用户空间工具 - https://gitlab.com/apparmor

内核模块 - git://git.kernel.org/pub/scm/linux/kernel/git/jj/linux-apparmor
