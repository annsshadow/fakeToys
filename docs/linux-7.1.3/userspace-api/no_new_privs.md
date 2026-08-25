## 禁止新权限标志（No New Privileges Flag

execve 系统调用可以授予新启动的程序其父进程所没有的权限。最明显的例子是 setuid/setgid 程序和文件能力（file capabilities）。为了防止父进程也获得这些权限，内核与用户态代码必须小心，避免父进程做出任何可能破坏子进程的事情。例如：

 - 动态加载器在处`LD_*` 环境变量时，若程序为 setuid，则会采用不同的方式
 - chroot 对非特权进程是禁止的，因为它会让chroot 继承而来的进程视角下替换`/etc/passwd`
 - exec 代码ptrace 有专门的处理
这些都是临时性的修补。自 Linux 3.5 起，`no_new_privs` 位是一种新的、通用的机制，用于在跨 execve 持续生效的方式下，让进程安全地修改其执行环境。任何任务都可以设置 `no_new_privs`。一旦该位被设置，它会在 fork、clone execve 之间继承，且无法被清除。在设置`no_new_privs` 的情况下，`execve()` 承诺不会授予任何在没execve 调用时无法完成的权限。例如，setuid setgid 位将不再改变 uid gid；文件能力不会加入许可集（permitted set），LSM 也不会在 execve 之后放宽约束
```

    prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);

```
不过要注意：`no_new_privs` 模式下，LSM 也可能不会在 exec 时收紧约束。（这意味着，建立一个通用的服务启动器、在 exec 守护进程之前设置 `no_new_privs`，可能会干扰基于 LSM 的沙箱隔离。）

请注意，`no_new_privs` 并不会阻止不涉及 `execve()` 的权限变更。一个具有适当权限的任务仍然可以调`setuid(2)` 并接SCM_RIGHTS 数据报
到目前为止，`no_new_privs` 有两个主要用例：

 - seccomp 模式 2 沙箱安装的过滤器会在 execve 之间持续存在，并可以改变新执行程序的行为。因此，只有在设置了 `no_new_privs` 的情况下，才允许非特权用户安装此类过滤器
 - 仅凭自身，`no_new_privs` 就可用于缩减非特权用户可用的攻击面。如果以某个给定 uid 运行的所有进程都设置`no_new_privs`，那么该 uid 将无法通过直接攻击 setuid、setgid 及使fcap 的二进制文件来提升其权限；它必须先攻破某个未设置 `no_new_privs` 位的目标
将来，如果设置了 `no_new_privs`，其他潜在危险的内核特性也可能对非特权任务开放。原则上，`unshare(2)` `clone(2)` 的若干选项在设置了 `no_new_privs` 时是安全的，而且 `no_new_privs` + `chroot` 的危险性远低于单独使用 chroot