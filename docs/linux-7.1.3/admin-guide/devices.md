
## Linux allocated 设备 (4.x+ 版本)


此 列出 是 the Linux 设备 列出, the official registry 的 allocated
设备 numbers 和 `/dev` directory nodes 用于 the Linux operating
系统.

The 版本 的 此 document 在 lanana.org 是 无 longer maintained.  此
版本 在 the mainline Linux 内核 是 the master document.  Updates
shall 为 sent 作为 patches 到 the 内核 maintainers (参见 the
Documentation/进程/submitting-patches.rst <submittingpatches> document).
Specifically explore the sections titled "CHAR 和 MISC 驱动", 和
"块 LAYER" 在 the MAINTAINERS 文件 到 find the right maintainers
到 involve 用于 character 和 块 设备.

此 document 是 included 由 参考 进入 the 文件系统 Hierarchy
标准 (FHS).	 The FHS 是 可用 来自 https://www.pathname.com/fhs/.

Allocations marked (68k/Amiga) apply 到 Linux/68k 在 the Amiga
platform 仅.	Allocations marked (68k/Atari) apply 到 Linux/68k 在
the Atari platform 仅.

此 document 是 在 the 公共 domain.	The authors requests, 然而,
该 semantically altered versions 是 不 distributed 无
permission 的 the authors, assuming the authors 可 为 contacted 无
一个 unreasonable effort.



  设备 驱动 AUTHORS 请 读取 此

  Linux 现在 具有 extensive 支持 用于 动态 分配 的 设备 numbering
  和 可 使用 `sysfs` 和 `udev` (`systemd`) 到 handle the naming needs.
  存在 仍然 一些 exceptions 在 the 串行 和 boot 设备 area. 之前
  asking   用于 一个 设备 数字 确保 您 actually 需要 one.

  到 具有 一个 主要 数字 allocated, 或 一个 次要 数字 在 situations
  何处 该 applies (e.g. busmice), 请 submit 一个 patch 和 send 到
  the authors 作为 indicated 上文.

  Keep the description 的 the 设备 *在 the 相同 格式
  作为 此 列出*. The reason 用于 这是 该 它是 the 仅 way 我们 具有
  found 到 ensure 我们 具有 全部 the requisite information 到 publish 您的
  设备 和 avoid conflicts.

  Finally, 有时 我们 具有 到 play "namespace police."  请 don't 为
  offended.  我们 通常 get submissions 用于 `/dev` names 该 将会 为 bound
  到 cause conflicts down the road.  我们 是 trying 到 avoid getting 在 一个
  situation 何处 我们 将会 具有 到 suffer 一个 incompatible forward
  change.  因此, 请 consult 与 us **之前** 您 make 您的
  设备 names 和 numbers 在 任何 way 公共, 至少 到 the point
  何处 它 将会 为 在 全部 difficult 到 get them changed.

  您的 cooperation 是 appreciated.

   :literal:

### 额外 ``/dev/`` directory 条目


此 section details 额外 条目 该 应当 或 可 exist 在
the /dev directory.  它是 preferred 该 symbolic links 使用 the 相同
form (absolute 或 relative) 作为 是 indicated 此处.  Links 是
classified 作为 "hard" 或 "symbolic" depending 在 the preferred 类型 的
link; 若 可能, the indicated 类型 的 link 应当 为 使用.

Compulsory links
++++++++++++++++

这些 links 应当 exist 在 全部 系统:

=============== =============== =============== ===============================
/dev/fd		/proc/self/fd	symbolic	文件 描述符
/dev/stdin	fd/0		symbolic	stdin 文件 描述符
/dev/stdout	fd/1		symbolic	stdout 文件 描述符
/dev/stderr	fd/2		symbolic	stderr 文件 描述符
/dev/nfsd	socksys		symbolic	必需 由 iBCS-2
/dev/X0R	null		symbolic	必需 由 iBCS-2
=============== =============== =============== ===============================

注意: `/dev/X0R` 是 <letter X>-<digit 0>-<letter R>.

Recommended links
+++++++++++++++++

它是 recommended 该 这些 links exist 在 全部 系统:


=============== =============== =============== ===============================
/dev/核心	/proc/kcore	symbolic	Backward compatibility
/dev/ramdisk	ram0		symbolic	Backward compatibility
/dev/ftape	qft0		symbolic	Backward compatibility
/dev/bttv0	视频0		symbolic	Backward compatibility
/dev/radio	radio0		symbolic	Backward compatibility
/dev/i2o**	/dev/i2o/**	symbolic	Backward compatibility
=============== =============== =============== ===============================

Suggested 更早 `/dev/scd?` alternative names 用于 `/dev/sr?`
CD-ROM 和 其他 optical drives (使用 SCSI 命令) 曾是 removed
在 `udev` 版本 174 该 曾是 released 在 2011.

Locally 定义 links
+++++++++++++++++++++

The 以下 links 可 为 established locally 到 conform 到 the
配置 的 the 系统.  这是 merely 一个 tabulation 的 existing
practice, 和 执行 不 constitute 一个 recommendation.  然而, 若 它们
exist, 它们 应当 具有 the 以下 uses.

=============== =============== =============== ===============================
/dev/鼠标	鼠标 端口	symbolic	电流 鼠标 设备
/dev/tape	tape 设备	symbolic	电流 tape 设备
/dev/cdrom	CD-ROM 设备	symbolic	电流 CD-ROM 设备
/dev/扫描仪	扫描仪		symbolic	电流 扫描仪 设备
/dev/modem	modem 端口	symbolic	电流 dialout 设备
/dev/root	root 设备	symbolic	电流 root 文件系统
/dev/swap	swap 设备	symbolic	电流 swap 设备
=============== =============== =============== ===============================

`/dev/modem` 应当 不 为 使用 用于 一个 modem 其 supports dialin 作为
well 作为 dialout, 作为 它 tends 到 cause 锁 文件 problems.  若 它
exists, `/dev/modem` 应当 point 到 the appropriate primary TTY 设备
(the 使用 的 the alternate callout 设备 是 已废弃).

用于 SCSI 设备, `/dev/tape` 和 `/dev/cdrom` 应当 point 到 the
**cooked** 设备 (`/dev/st**` 和 `/dev/sr**`, respectively), whereas
`/dev/scanner` 应当 point 到 the appropriate generic
SCSI 设备 (`/dev/sg*`).

`/dev/mouse` 可 point 到 一个 primary 串行 TTY 设备, 一个 硬件 鼠标
设备, 或 一个 套接字 用于 一个 鼠标 驱动 program (e.g. `/dev/gpmdata`).

Sockets 和 pipes
+++++++++++++++++

Non-transient sockets 和 named pipes 可 exist 在 /dev.  通用 条目 是:

=============== =============== ===============================================
/dev/打印机	套接字		lpd 本地 套接字
/dev/log	套接字		syslog 本地 套接字
/dev/gpmdata	套接字		gpm 鼠标 multiplexer
=============== =============== ===============================================

Mount points
++++++++++++

The 以下 names 是 reserved 用于 mounting 特殊 文件系统
在…下 /dev.  这些 特殊 文件系统 提供 内核 interfaces 该
cannot 为 provided 与 标准 设备 nodes.

=============== =============== ===============================================
/dev/pts	devpts		PTY slave 文件系统
/dev/shm	tmpfs		POSIX shared 内存 maintenance access
=============== =============== ===============================================

### Terminal 设备


Terminal, 或 TTY 设备 是 一个 特殊 类 的 character 设备.  一个
terminal 设备 是 任何 设备 该 可以 act 作为 一个 controlling terminal
用于 一个 会话; 此 包含 虚拟 consoles, 串行 ports, 和
pseudoterminals (PTYs).

全部 terminal 设备 share 一个 通用 set 的 capabilities known 作为 line
disciplines; 这些 包含 the 通用 terminal line discipline 作为 well
作为 SLIP 和 PPP modes.

全部 terminal 设备 是 named similarly; 此 section explains the
naming 和 使用 的 the 各种 types 的 TTYs.  注意 该 the naming
conventions 包含 若干 historical warts; 一些 的 这些 是
Linux-specific, 一些 曾是 inherited 来自 其他 系统, 和 一些
reflect Linux outgrowing 一个 borrowed convention.

一个 hash mark (`#`) 在 一个 设备 name 是 使用 此处 到 indicate 一个 decimal
数字 无 leading zeroes.

虚拟 consoles 和 the console 设备
+++++++++++++++++++++++++++++++++++++++

虚拟 consoles 是 full-screen terminal displays 在 the 系统 视频
监视器.  虚拟 consoles 是 named `/dev/tty#`, 与 numbering
starting 在 `/dev/tty1`; `/dev/tty0` 是 the 电流 虚拟 console.
`/dev/tty0` 是 the 设备 该 应当 为 使用 到 access the 系统 视频
卡 在 那些 architectures 用于 其 the 帧 缓冲区 设备
(`/dev/fb*`) 是 不 applicable. 执行 不 使用 `/dev/console`
用于 此 purpose.

The console 设备, `/dev/console`, 是 the 设备 到 其 系统
messages 应当 为 sent, 和 在 其 logins 应当 为 permitted 在
single-user 模式.  Starting 与 Linux 2.1.71, `/dev/console` 是 managed
由 the 内核; 用于 前一个 versions 它 应当 为 一个 symbolic link 到
任一个 `/dev/tty0`, 一个 特定 虚拟 console 例如 `/dev/tty1`, 或 到
一个 串行 端口 primary (`tty**`, 不 `cu**`) 设备, depending 在 the
配置 的 the 系统.

串行 ports
++++++++++++

串行 ports 是 RS-232 串行 ports 和 任何 设备 其 simulates
one, 任一个 在 硬件 (例如 内部 modems) 或 在 软件 (此类
作为 the ISDN 驱动.)  在…下 Linux, 每个 串行 ports 具有 two 设备
names, the primary 或 callin 设备 和 the alternate 或 callout one.
每个 kind 的 设备 是 indicated 由 一个 不同 letter.	 用于 任何
letter X, the names 的 the 设备 是 `/dev/ttyX#` 和 `/dev/cux#`,
respectively; 用于 historical reasons, `/dev/ttyS#` 和 `/dev/ttyC#`
correspond 到 `/dev/cua#` 和 `/dev/cub#`. 在 the future, 它 应当 为
expected 该 多个 letters 将 为 使用; 全部 letters 将 为 upper
case 用于 the "tty" 设备 (e.g. `/dev/ttyDP#`) 和 lower case 用于 the
"cu" 设备 (e.g. `/dev/cudp#`).

The names `/dev/ttyQ#` 和 `/dev/cuq#` 是 reserved 用于 本地 使用.

The alternate 设备 提供 用于 kernel-based exclusion 和 somewhat
不同 defaults 比 the primary 设备.  它们的 主要 purpose 是 到
允许 the 使用 的 串行 ports 与 programs 与 无 inherent 或 broken
支持 用于 串行 ports.  它们的 使用 是 已废弃, 和 它们 可 为
removed 来自 一个 future 版本 的 Linux.

Arbitration 的 串行 ports 是 provided 由 the 使用 的 锁 文件 与
the names `/var/lock/LCK..ttyX#`. The contents 的 the 锁 文件 应当
为 the PID 的 the locking 进程 作为 一个 ASCII 数字.

它是 通用 practice 到 install links 例如 /dev/modem
其 point 到 串行 ports.  为了 ensure proper locking 在 the
presence 的 这些 links, 它是 recommended 该 软件 chase
symlinks 和 锁 全部 可能 names; additionally, 它是 recommended
该 一个 锁 文件 为 installed 与 the corresponding alternate
设备.	 为了 avoid deadlocks, 它是 recommended 该 the 锁
是 acquired 在 the 以下 order, 和 released 在 the reverse:

 1. The symbolic link name, 若 任何 (`/var/lock/LCK..modem`)
 2. The "tty" name (`/var/lock/LCK..ttyS2`)
 3. The alternate 设备 name (`/var/lock/LCK..cua2`)

在该情况下 的 nested symbolic links, the 锁 文件 应当 为
installed 在 the order the symlinks 是 resolved.

在…下 无 circumstances 应当 一个 应用程序 hold 一个 锁 同时 waiting
用于 another 到 为 released.  此外, applications 其 attempt
到 创建 锁 文件 用于 the corresponding alternate 设备 names
应当 take 进入 account the possibility 的 正在 使用 在 一个 non-serial
端口 TTY, 用于 其 无 alternate 设备 将会 exist.

Pseudoterminals (PTYs)
++++++++++++++++++++++

Pseudoterminals, 或 PTYs, 是 使用 到 创建 login sessions 或 提供
其他 capabilities requiring 一个 TTY line discipline (including SLIP 或
PPP capability) 到 arbitrary data-generation 进程.	 每个 PTY 具有
一个 master side, named `/dev/pty[p-za-e][0-9a-f]`, 和 一个 slave side, named
`/dev/tty[p-za-e][0-9a-f]`.  The 内核 arbitrates the 使用 的 PTYs 由
allowing 每个 master side 到 为 opened 仅 一旦.

一旦 the master side 具有 已经 opened, the corresponding slave 设备
可 为 使用 在 the 相同 manner 作为 任何 TTY 设备.  The master 和
slave 设备 是 connected 由 the 内核, generating the equivalent
的 一个 bidirectional pipe 与 TTY capabilities.

Recent versions 的 the Linux kernels 和 GNU libc 包含 支持 用于
the 系统 V/Unix98 naming scheme 用于 PTYs, 其 assigns 一个 通用
设备, `/dev/ptmx`, 到 全部 the masters (opening 它 将 automatically
give 您 一个 previously unassigned PTY) 和 一个 subdirectory, `/dev/pts`,
用于 the slaves; the slaves 是 named 与 decimal integers (`/dev/pts/#`
在 我们的 notation).  此 removes the problem 的 exhausting the
namespace 和 enables the 内核 到 automatically 创建 the 设备
nodes 用于 the slaves 在 demand 使用 the "devpts" 文件系统.
