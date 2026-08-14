
## 不可执行 mfd 的引入


:Author:
    Daniel Verkamp <dverkamp@chromium.org>
    Jeff Xu <jeffxu@chromium.org>

:Contributor:
	Aleksa Sarai <cyphar@cyphar.com>

自 Linux 引入 memfd 特性以来，memfd 始终带有执行位，而 memfd_create() 系统调用不允许以不同方式设置它。

然而，在一个默认安全（secure-by-default）的系统（如 ChromeOS，其中所有可执行文件都应来自受验证启动保护的根文件系统）中，memfd 的这种可执行特性为 NoExec 绕过打开了大门，并促成了“混淆代理攻击（confused deputy attack）”。例如，在 VRP 缺陷 [^1^] 中：cros_vm 进程创建了一个 memfd 来与外部进程共享内容，但该 memfd 被覆写并用于执行任意代码与提权。[^2^] 列出了更多此类 VRP。

另一方面，可执行的 memfd 有其合法用途：runc 使用 memfd 的 seal 与可执行特性来复制二进制的内容然后执行它们。对于这样的系统，我们需要一种方案来区分 runc 对可执行 memfd 的使用与攻击者的使用 [^3^]。

为了解决上述问题：
 - 让 memfd_create() 在创建时设置 X 位。
 - 当设置 NX 时，让 memfd 被 seal 以禁止修改 X 位。
 - 新增一个 pid namespace sysctl：vm.memfd_noexec，以帮助应用程序迁移并强制使用不可执行 MFD。

## 用户 API


`int memfd_create(const char *name, unsigned int flags)`

`MFD_NOEXEC_SEAL`
	当 `flags` 中设置了 MFD_NOEXEC_SEAL 位时，memfd 以 NX 创建。F_SEAL_EXEC 被设置，且 memfd 之后不能被修改为添加 X。同时隐含 MFD_ALLOW_SEALING。
	这是应用程序使用 memfd 最常见的情况。

`MFD_EXEC`
	当 `flags` 中设置了 MFD_EXEC 位时，memfd 以 X 创建。

注意：
	`MFD_NOEXEC_SEAL` 隐含 `MFD_ALLOW_SEALING`。若应用程序不希望 seal，它可以在创建后添加 F_SEAL_SEAL。

## Sysctl：


`pid namespaced sysctl vm.memfd_noexec`

新的 pid namespaced sysctl vm.memfd_noexec 有 3 个值：

 - 0: MEMFD_NOEXEC_SCOPE_EXEC
	不带 MFD_EXEC 也不带 MFD_NOEXEC_SEAL 的 memfd_create() 表现得如同设置了 MFD_EXEC。

 - 1: MEMFD_NOEXEC_SCOPE_NOEXEC_SEAL
	不带 MFD_EXEC 也不带 MFD_NOEXEC_SEAL 的 memfd_create() 表现得如同设置了 MFD_NOEXEC_SEAL。

 - 2: MEMFD_NOEXEC_SCOPE_NOEXEC_ENFORCED
	不带 MFD_NOEXEC_SEAL 的 memfd_create() 将被拒绝。

该 sysctl 允许对不设置执行位的旧软件进行更精细的 memfd_create 控制；例如，一个 vm.memfd_noexec=1 的容器意味着旧软件默认将创建不可执行的 memfd，而新软件可以通过设置 MFD_EXEC 创建可执行的 memfd。

vm.memfd_noexec 的值在创建时传递给子命名空间。此外，该设置是分层的，即在 memfd_create 期间，我们将从当前 ns 搜索到根 ns，并使用最严格的设置。

[^1^] https://crbug.com/1305267

[^2^] https://bugs.chromium.org/p/chromium/issues/list?q=type%3Dbug-security%20memfd%20escalation&can=1

[^3^] https://lwn.net/Articles/781013/
