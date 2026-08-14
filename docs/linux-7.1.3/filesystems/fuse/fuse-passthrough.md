
## FUSE 直通（Passthrough）


## 简介


FUSE（Filesystem in Userspace，用户空间文件系统）直通是一项旨在提升 FUSE 文件系统在 I/O 操作方面性能的特性。通常，FUSE 操作涉及内核与用户空间 FUSE 守护进程之间的通信，这会带来开销。直通允许对 FUSE 文件执行的某些操作绕过用户空间守护进程，由内核直接在一个底层的“后备文件（backing file）”上执行。

这一机制的实现方式是：FUSE 守护进程向 FUSE 内核模块注册一个文件描述符（指向底层文件系统上的后备文件）。随后内核会为该已注册的后备文件收到一个标识符（`backing_id`）。当随后打开一个 FUSE 文件时，FUSE 守护进程可以在其对 `OPEN` 请求的响应中，包含该 `backing_id` 并设置 `FOPEN_PASSTHROUGH` 标志。这样就为特定的操作建立了直接链接。

目前，直通支持 `read(2)`/`write(2)`（通过 `read_iter`/`write_iter`）、`splice(2)` 以及 `mmap(2)` 等操作。

## 启用直通


要使用 FUSE 直通：

  1. FUSE 文件系统必须在编译时启用 `CONFIG_FUSE_PASSTHROUGH`。
  2. FUSE 守护进程必须在 `FUSE_INIT` 握手期间协商 `FUSE_PASSTHROUGH` 能力，并指定其期望的 `max_stack_depth`。
  3. （特权）FUSE 守护进程在其连接文件描述符（例如 `/dev/fuse`）上使用 `FUSE_DEV_IOC_BACKING_OPEN` ioctl 来注册一个后备文件描述符，并获得一个 `backing_id`。
  4. 在处理 FUSE 文件的 `OPEN` 或 `CREATE` 请求时，守护进程在 `fuse_open_out::open_flags` 中设置 `FOPEN_PASSTHROUGH` 标志，并在 `fuse_open_out::backing_id` 中提供相应的 `backing_id`。
  5. FUSE 守护进程最终应当使用 `FUSE_DEV_IOC_BACKING_CLOSE` 配合 `backing_id` 来释放内核对该后备文件的引用，前提是直通设置不再需要它。

## 权限要求


设置直通功能目前要求 FUSE 守护进程具备 `CAP_SYS_ADMIN` 能力。这一要求的来源涉及若干安全和资源管理方面的考量，目前仍在积极讨论与改进中。实施此限制的主要原因详见下文。

### 资源记账与可见性


直通的核心机制是 FUSE 守护进程打开一个指向后备文件的文件描述符，并通过 `FUSE_DEV_IOC_BACKING_OPEN` ioctl 将其注册到 FUSE 内核模块。该 ioctl 返回一个与内核内部的 `struct fuse_backing` 对象关联的 `backing_id`，后者持有对后备 `struct file` 的引用。

一个显著的隐患在于：FUSE 守护进程可以在注册后关闭其自身指向后备文件的文件描述符。然而，只要该 `struct file` 仍与一个 `backing_id` 关联（或者随后与一个处于直通模式的已打开 FUSE 文件关联），内核就会通过 `struct fuse_backing` 对象继续持有对它的引用。

这种行为会给非特权 FUSE 守护进程带来两个主要问题：

  1. **对 lsof 及其他检查工具不可见**：一旦 FUSE 守护进程关闭其文件描述符，内核持有的已打开后备文件就变得“隐藏”。像 `lsof` 这类通常会检查进程文件描述符表的常规工具，将无法识别出该文件仍由系统代表 FUSE 文件系统打开。这使得系统管理员难以追踪资源使用情况，或调试与已打开文件相关的问题（例如阻止卸载）。

  2. **绕过 RLIMIT_NOFILE**：FUSE 守护进程进程受到资源限制约束，包括最大打开文件描述符数量（`RLIMIT_NOFILE`）。如果非特权守护进程能够注册后备文件然后关闭自身的文件描述符，就可能导致内核持有数量不受限制的已打开 `struct file` 引用，且这些引用不计入该守护进程的 `RLIMIT_NOFILE`。这可能通过耗尽系统范围的文件资源而导致拒绝服务（DoS）。

`CAP_SYS_ADMIN` 要求起到了防范这些问题的作用，将此强大能力限制在受信任的进程范围内。

**注意**：`io_uring` 通过暴露其“固定文件（fixed files）”解决了类似问题，这些文件可通过 `fdinfo` 查看，并计入注册用户所属的 `RLIMIT_NOFILE`。

### 文件系统堆叠与关闭循环


另一个隐患与潜在地创建复杂且有问题的文件系统堆叠场景有关，如果非特权用户能够设置直通的话。一个 FUSE 直通文件系统可能使用一个位于以下位置的后备文件：

  - 位于**同一个** FUSE 文件系统上。
  - 位于另一个文件系统（如 OverlayFS）上，而该文件系统自身的上层或下层可能又是一个 FUSE 文件系统。

这些配置可能在文件系统关闭或卸载过程中创建依赖循环，导致死锁或系统不稳定。这在概念上类似于与 `LOOP_SET_FD` ioctl 相关的风险，该 ioctl 同样需要 `CAP_SYS_ADMIN`。

为缓解此问题，FUSE 直通已经引入了基于文件系统堆叠深度（`sb->s_stack_depth` 和 `fc->max_stack_depth`）的检查。例如，在 `FUSE_INIT` 握手期间，FUSE 守护进程可以协商其支持的 `max_stack_depth`。当通过 `FUSE_DEV_IOC_BACKING_OPEN` 注册一个后备文件时，内核会检查该后备文件的文件系统堆叠深度是否在允许的限制之内。

`CAP_SYS_ADMIN` 要求提供了额外一层安全保障，确保只有特权用户才能创建这些可能复杂的堆叠结构。

### 总体安全姿态


作为一项通用原则，对于允许用户空间基于其所提供的文件描述符来指示内核代为执行直接操作的新内核特性，从较高的权限要求（例如 `CAP_SYS_ADMIN`）起步是一种保守且常见的安全实践。这样可以让该特性在被使用与测试的同时，进一步的安全影响得到评估与处理。
