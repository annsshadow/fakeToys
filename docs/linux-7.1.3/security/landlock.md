
## Landlock LSM: 内核文档


:Author: Mickaël Salaün
:Date: March 2026

Landlock 的目标是创建作用域化的访问控制（即沙箱隔离）。为了使整个系统更加坚固，该功能应当对所有进程（包括非特权进程）可用。由于此类进程可能已被攻陷或植入后门（即不可信），Landlock 的功能从内核和其他进程的角度来看必须是安全可用的。因此，Landlock 的接口必须暴露最小的攻击面。

Landlock 的设计目标是对非特权进程可用，同时遵循由其他访问控制机制（例如 DAC、LSM）强制执行的系统安全策略。一条 Landlock 规则不应干扰系统上强制实施的其他访问控制，而只是增加更多限制。

任何用户都可以在自己的进程上强制实施 Landlock 规则集。它们会与继承的规则集合并并进行评估，确保只能添加更多约束。

用户空间文档可在此处找到：
Documentation/userspace-api/landlock.rst。

## 安全访问控制的设计原则


- Landlock 规则应聚焦于内核对象的访问控制，而不是系统调用过滤（即系统调用参数），后者是 seccomp-bpf 的目的。
- 为了避免多种侧信道攻击（例如安全策略泄露、基于 CPU 的攻击），Landlock 规则不能以编程方式与用户空间通信。
- 内核的访问检查不应拖慢来自未沙箱化进程的访问请求。
- 与 Landlock 操作相关的计算（例如强制实施规则集）应只影响请求它们的进程。
- 由沙箱化进程直接从内核获取的资源（例如文件描述符）应保留其作用域化的访问权限（在获取资源时确定），无论由哪个进程使用。参见 `File descriptor access rights`_。
- 访问拒绝应按照系统和 Landlock 域的配置进行记录。日志条目必须包含拒绝原因以及相关安全策略的所有者信息。此类日志的生成对允许的请求应产生可忽略的性能和内存影响。

## 设计选择


### Inode 访问权限


所有访问权限都与一个 inode 以及可通过它访问的内容绑定。读取目录内容并不意味着被允许读取所列 inode 的内容。实际上，文件名是相对于其父目录局部的，而一个 inode 可以通过（硬）链接被多个文件名引用。能够取消链接（unlink）一个文件只会对目录产生直接影响，而不会影响被取消链接的 inode。正因如此，`LANDLOCK_ACCESS_FS_REMOVE_FILE` 或 `LANDLOCK_ACCESS_FS_REFER` 不允许绑定到文件，而只能绑定到目录。

### 文件描述符访问权限


访问权限在打开时会被检查并绑定到文件描述符。其底层原则是：当在相同的 Landlock 域下执行时，等价的操作序列应产生相同的结果。

以 `LANDLOCK_ACCESS_FS_TRUNCATE` 权限为例，如果相关文件层级未授予该访问权限，则可能允许以写入方式打开一个文件，却不允许对生成的文件描述符执行 `ftruncate`。以下操作序列具有相同的语义，因此应当产生相同的结果：

- `truncate(path);`
- `int fd = open(path, O_WRONLY); ftruncate(fd); close(fd);`

与文件访问模式（例如 `O_RDWR`）类似，附加到文件描述符上的 Landlock 访问权限即使在不同进程间传递（例如通过 Unix 域套接字）也会保留。因此，即使接收进程没有被 Landlock 沙箱化，这些访问权限仍会被强制执行。事实上，这是保持整个系统访问控制一致性所必需的，并且可以避免通过文件描述符传递而产生的意外绕过（即混淆代理攻击，confused deputy attack）。


### 作用域标志与其他访问权限之间的交互


`scoped` 标志（位于 &struct landlock_ruleset_attr 中）会限制所创建的 Landlock 域内**出站** IPC 的使用，同时允许其连接到该 Landlock 域**内部**的 IPC 端点。

未来，作用域标志**可能**会与其他访问权限交互，例如允许按名称将抽象 UNIX 套接字加入允许列表，或者允许按信号编号或目标进程将信号加入允许列表。

在引入 `LANDLOCK_ACCESS_FS_RESOLVE_UNIX` 时，我们将其定义为隐式具有与 `LANDLOCK_SCOPE_PATHNAME_UNIX_SOCKET` 标志相同的范围语义：连接到同一域内的 UNIX 套接字（使用 `LANDLOCK_ACCESS_FS_RESOLVE_UNIX` 的地方）是无条件允许的。

其理由如下：

- 与其他 IPC 机制一样，连接到同一域内的具名 UNIX 套接字应当是预期且无害的。（如有需要，用户可以通过嵌套域或限制 `LANDLOCK_ACCESS_FS_MAKE_SOCK` 来进一步细化其 Landlock 策略。）
- 我们保留了在未来仍然引入 `LANDLOCK_SCOPE_PATHNAME_UNIX_SOCKET` 的选项。（如果我们希望有一条 Landlock 规则允许访问其他 Landlock 域的 IPC，这将很有用。）
- 但我们可以将用户需要处理在用户空间 API 中可见的两个相互交互标志的时间点推迟。（特别是，实践中可能并不需要它，在这种情况下我们可以完全避免引入第二个标志。）
- 如果我们**确实**在未来引入了 `LANDLOCK_SCOPE_PATHNAME_UNIX_SOCKET`，在规则集中设置该作用域标志并**不会**减少限制，因为同一作用域内的访问已经基于 `LANDLOCK_ACCESS_FS_RESOLVE_UNIX` 被允许。

## 测试


用于向后兼容性、ptrace 限制以及文件系统支持的用户空间测试可在此处找到：`tools/testing/selftests/landlock/`_。

## 内核结构体


### Object


    :identifiers:

### Filesystem


    :identifiers:

### Process credential


    :identifiers:

### Ruleset 与 domain


一个域（domain）是一个绑定到一组主体（即任务的凭证 credentials）的只读规则集。每次在任务上强制实施规则集时，当前域会被复制，该规则集会作为新规则层导入到新域中。实际上，一旦进入某个域，每条规则都绑定到一个层级（layer level）。要授予对某个对象的访问权限，每个层级中至少必须有一条规则允许对该对象的请求操作。因此，一个任务只能转换到新域，该新域是当前域的约束与任务提供的规则集约束的交集。

对于一个对自己进行沙箱化的任务，其主体（subject）的定义是隐式的，这使得推理更加容易，并有助于避免陷阱。

    :identifiers:

    :identifiers:

## 补充文档


- Documentation/userspace-api/landlock.rst
- Documentation/admin-guide/LSM/landlock.rst
- https://landlock.io

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/landlock/
