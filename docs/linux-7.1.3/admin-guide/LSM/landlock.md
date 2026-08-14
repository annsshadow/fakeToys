
## Landlock：系统级管理


:Author: Mickaël Salaün
:Date: January 2026

Landlock 可以利用审计（audit）框架来记录事件。

用户空间文档可在此处找到：Documentation/userspace-api/landlock.rst。

## 审计


如果启用了 `audit`，那么被沙箱化的程序所发起的被拒绝访问请求会默认记录日志。
这一默认行为可以通过 sys_landlock_restrict_self() 的标志来更改（参见
Documentation/userspace-api/landlock.rst）。Landlock 日志也可以通过审计规则来
屏蔽。Landlock 可以生成 2 种审计记录类型。

### 记录类型


AUDIT_LANDLOCK_ACCESS
    该记录类型标识一次对被拒绝的内核资源访问请求。`domain` 字段指示阻止了该
    请求的域（domain）的 ID。`blockers` 字段指示该拒绝的原因（以逗号分隔），
    其余字段标识内核对象（类似于 SELinux）。每个审计事件中可能有不止一条该
    类型的记录。

```

        domain=195ba459b blockers=fs.refer path="/usr/bin" dev="vda2" ino=351
        domain=195ba459b blockers=fs.make_reg,fs.refer path="/usr/local" dev="vda2" ino=365


    ``blockers`` 字段使用以句点分隔的前缀来表示导致拒绝的限制类型：

    **fs.*** - 文件系统访问权限（ABI 1+）：
        - fs.execute、fs.write_file、fs.read_file、fs.read_dir
        - fs.remove_dir、fs.remove_file
        - fs.make_char、fs.make_dir、fs.make_reg、fs.make_sock
        - fs.make_fifo、fs.make_block、fs.make_sym
        - fs.refer（ABI 2+）
        - fs.truncate（ABI 3+）
        - fs.ioctl_dev（ABI 5+）

    **net.*** - 网络访问权限（ABI 4+）：
        - net.bind_tcp - TCP 端口绑定被拒绝
        - net.connect_tcp - TCP 连接被拒绝

    **scope.*** - IPC 范围限制（ABI 6+）：
        - scope.abstract_unix_socket - 抽象 UNIX 套接字连接被拒绝
        - scope.signal - 信号发送被拒绝

    当缺少多项访问权限时，多个 blockers 可能出现在同一条事件中（以逗号分隔）。
    例如，在一个既缺少 ``make_reg`` 又缺少 ``refer`` 权限的目录中创建普通文件，
    会显示 ``blockers=fs.make_reg,fs.refer``。

    对象标识字段（文件系统对应的是 path、dev、ino；信号对应的是 opid、ocomm）
    取决于被阻止的访问类型，并提供关于拒绝所涉及资源的上下文。

```
AUDIT_LANDLOCK_DOMAIN
    该记录类型描述一个 Landlock 域的状态。`status` 字段可以是 `allocated` 或
    `deallocated`。

    `allocated` 状态属于同一个审计事件的一部分，并跟在某个域首次记录的
    `AUDIT_LANDLOCK_ACCESS` 记录之后。它标识在调用 sys_landlock_restrict_self()
    时该 Landlock 域的信息，包含以下字段：

    - 域（domain）ID
    - 强制（enforcement）`mode`
    - 域创建者的 `pid`
    - 域创建者的 `uid`
    - 域创建者的可执行文件路径（`exe`）
    - 域创建者的命令行（`comm`）

```

        domain=195ba459b status=allocated mode=enforcing pid=300 uid=0 exe="/root/sandboxer" comm="sandboxer"

    ``deallocated`` 状态是一个独立的事件，它标识一次 Landlock 域的释放。在此
    事件之后，可以保证在系统生命周期内相关域 ID 绝不会被复用。``domain`` 字段
    指示被释放域的 ID，``denials`` 字段指示被拒绝访问请求的总数，其中部分请求
    可能由于审计规则和 sys_landlock_restrict_self() 的标志而未被记录。

    Example::

        domain=195ba459b status=deallocated denials=3

```
### 事件示例


下面是两个日志记录事件的示例（参见序列号）。

在此示例中，一个被沙箱化的程序（`kill`）试图向 init 进程发送信号，该请求因
信号范围限制而被拒绝。
```

  $ LL_FS_RO=/ LL_FS_RW=/ LL_SCOPED=s LL_FORCE_LOG=1 ./sandboxer kill 1

```
该命令生成两个事件，每个事件都带有一个跟随时间戳的唯一序列号
（`msg=audit(1729738800.268:30)`）。第一个事件（序列号 `30`）包含 4 条记录。
第一条记录（`type=LANDLOCK_ACCESS`）显示一个被域 `1a6fdc66f` 拒绝的访问。
该拒绝的原因是信号范围限制（`blockers=scope.signal`）。本将接收该信号的进程是
init 进程（`opid=1 ocomm="systemd"`）。

第二条记录（`type=LANDLOCK_DOMAIN`）描述（`status=allocated`）域 `1a6fdc66f`。
该域由进程 `286` 执行 root 用户启动的 `/root/sandboxer` 程序所创建。

第三条记录（`type=SYSCALL`）描述该 syscall、其提供的参数、其结果
（`success=no exit=-1`）以及调用它的进程。

第四条记录（`type=PROCTITLE`）以十六进制值显示命令名。可以用
``python -c 'print(bytes.fromhex("6B696C6C0031"))'`` 来转换它。

最后，最后一条记录（`type=LANDLOCK_DOMAIN`）也是第二个事件（序列号 `31`）中
唯一的记录。它并不对应于某个直接的用户空间动作，而是一个异步动作，用于释放
与某个 Landlock 域相关的资源（`status=deallocated`）。这有助于了解后续日志
将不再涉及域 `1a6fdc66f`。该记录还汇总了该域拒绝的请求数量（`denials=1`），
无论它们是否被记录。

```
  type=LANDLOCK_ACCESS msg=audit(1729738800.268:30): domain=1a6fdc66f blockers=scope.signal opid=1 ocomm="systemd"
  type=LANDLOCK_DOMAIN msg=audit(1729738800.268:30): domain=1a6fdc66f status=allocated mode=enforcing pid=286 uid=0 exe="/root/sandboxer" comm="sandboxer"
  type=SYSCALL msg=audit(1729738800.268:30): arch=c000003e syscall=62 success=no exit=-1 [..] ppid=272 pid=286 auid=0 uid=0 gid=0 [...] comm="kill" [...]
  type=PROCTITLE msg=audit(1729738800.268:30): proctitle=6B696C6C0031
  type=LANDLOCK_DOMAIN msg=audit(1729738800.324:31): domain=1a6fdc66f status=deallocated denials=1

```

  $ LL_FS_RO=/ LL_FS_RW=/tmp LL_FORCE_LOG=1 ./sandboxer sh -c "echo > /etc/passwd"

```
相关的审计日志包含来自 3 个不同事件（序列号 33、
```

  type=LANDLOCK_ACCESS msg=audit(1729738800.221:33): domain=1a6fdc679 blockers=fs.write_file path="/dev/tty" dev="devtmpfs" ino=9
  type=LANDLOCK_DOMAIN msg=audit(1729738800.221:33): domain=1a6fdc679 status=allocated mode=enforcing pid=289 uid=0 exe="/root/sandboxer" comm="sandboxer"
  type=SYSCALL msg=audit(1729738800.221:33): arch=c000003e syscall=257 success=no exit=-13 [...] ppid=272 pid=289 auid=0 uid=0 gid=0 [...] comm="sh" [...]
  type=PROCTITLE msg=audit(1729738800.221:33): proctitle=7368002D63006563686F203E202F6574632F706173737764
  type=LANDLOCK_ACCESS msg=audit(1729738800.221:34): domain=1a6fdc679 blockers=fs.write_file path="/etc/passwd" dev="vda2" ino=143821
  type=SYSCALL msg=audit(1729738800.221:34): arch=c000003e syscall=257 success=no exit=-13 [...] ppid=272 pid=289 auid=0 uid=0 gid=0 [...] comm="sh" [...]
  type=PROCTITLE msg=audit(1729738800.221:34): proctitle=7368002D63006563686F203E202F6574632F706173737764
  type=LANDLOCK_DOMAIN msg=audit(1729738800.261:35): domain=1a6fdc679 status=deallocated denials=2

### 事件过滤


如果你被与 Landlock 相关的审计日志刷屏，这要么是一次攻击尝试，要么是安全
策略中的 bug。我们可以通过两种互补的方式来设置一些过滤器以限制噪音：

- 如果我们能修复被沙箱化的程序，可以使用 sys_landlock_restrict_self() 的标志；
- 或使用审计规则（参见 `auditctl(8)`）。

## 补充文档


- `Linux Audit Documentation`_
- Documentation/userspace-api/landlock.rst
- Documentation/security/landlock.rst
- https://landlock.io

   https://github.com/linux-audit/audit-documentation/wiki
