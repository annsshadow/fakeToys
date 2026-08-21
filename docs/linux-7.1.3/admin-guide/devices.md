
## Linux allocated 设备 (4.x+ 版本)


列出 the Linux 设备 列出, the official registry allocated
设备 numbers `/dev` directory nodes 用于 the Linux operating
系统.

The 版本 document lanana.org longer maintained.  
版本 the mainline Linux 内核 the master document.  Updates
shall sent 作为 patches the 内核 maintainers (参见 the
Documentation/进程/submitting-patches.rst <submittingpatches> document).
Specifically explore the sections titled "CHAR 鍜?MISC 椹卞姩", 鍜。
"LAYER" the MAINTAINERS 文件 find the right maintainers
involve 用于 character 设备.

document included 参进入 the 文件系统 Hierarchy
标准 (FHS).	 The FHS 可用 来自 https://www.pathname.com/fhs/.

Allocations marked (68k/Amiga) apply 鍒?Linux/68k 鍦?the Amiga
platform 浠?	Allocations marked (68k/Atari) apply 鍒?Linux/68k 鍦。
the Atari platform 浠。

document the 公共 domain.	The authors requests, 然
璇?semantically altered versions 鏄，涓?distributed 鏃。
permission 鐨?the authors, assuming the authors 鍙，涓?contacted 鏃。
一unreasonable effort.



  设备 驱动 AUTHORS 读取 

  Linux 现在 具有 extensive 支持 用于 动分配 设备 numbering
  使用 `sysfs` `udev` (`systemd`) handle the naming needs.
  存在 仍然 一exceptions the 串行 boot 设备 area. 之前
  asking   用于 一设备 数字 确保 actually 需one.

  具有 一主要 数字 allocated, 一次要 数字 situations
  何处 applies (e.g. busmice), submit 一patch send 
  the authors 作为 indicated 上文.

  Keep the description the 设备 *the 相同 格式
  作为 列出*. The reason 用于 这是 它是 the way 我们 具有
  found ensure 我们 具有 全部 the requisite information publish 您的
  设备 avoid conflicts.

  Finally, 有时 我们 具有 play "namespace police."  don't 
  offended.  我们 通常 get submissions 用于 `/dev` names 将会 bound
  cause conflicts down the road.  我们 trying avoid getting 一
  situation 何处 我们 将会 具有 suffer 一incompatible forward
  change.  因此, consult us **之前** make 您的
  设备 names numbers 任何 way 公共, 至少 the point
  何处 将会 全部 difficult get them changed.

  您的 cooperation appreciated.

   :literal:

### 额外 ``/dev/`` directory 条目


section details 额外 条目 应当 exist 
the /dev directory.  它是 preferred symbolic links 使用 the 相同
form (absolute relative) 作为 indicated 此处.  Links 
classified 作为 "hard" "symbolic" depending the preferred 类型 
link; 可能, the indicated 类型 link 应当 使用.

Compulsory links
++++++++++++++++

这些 links 应当 exist 全部 系统:

=============== =============== =============== ===============================
/dev/fd		/proc/self/fd	symbolic	文件 描述
/dev/stdin	fd/0		symbolic	stdin 文件 描述
/dev/stdout	fd/1		symbolic	stdout 文件 描述
/dev/stderr	fd/2		symbolic	stderr 文件 描述
/dev/nfsd	socksys		symbolic	必需 iBCS-2
/dev/X0R	null		symbolic	必需 iBCS-2
=============== =============== =============== ===============================

注意: `/dev/X0R` <letter X>-<digit 0>-<letter R>.

Recommended links
+++++++++++++++++

它是 recommended 这些 links exist 全部 系统:


=============== =============== =============== ===============================
/dev/核心	/proc/kcore	symbolic	Backward compatibility
/dev/ramdisk	ram0		symbolic	Backward compatibility
/dev/ftape	qft0		symbolic	Backward compatibility
/dev/bttv0	视频0		symbolic	Backward compatibility
/dev/radio	radio0		symbolic	Backward compatibility
/dev/i2o**	/dev/i2o/**	symbolic	Backward compatibility
=============== =============== =============== ===============================

Suggested 更早 `/dev/scd?` alternative names 用于 `/dev/sr?`
CD-ROM 其他 optical drives (使用 SCSI 命令) 曾是 removed
`udev` 版本 174 曾是 released 2011.

Locally 定义 links
+++++++++++++++++++++

The 以下 links established locally conform the
配置 the 系统.  这是 merely 一tabulation existing
practice, 执行 constitute 一recommendation.  然 它们
exist, 它们 应当 具有 the 以下 uses.

=============== =============== =============== ===============================
/dev/鼠标	鼠标 端口	symbolic	电流 鼠标 设备
/dev/tape	tape 设备	symbolic	电流 tape 设备
/dev/cdrom	CD-ROM 设备	symbolic	电流 CD-ROM 设备
/dev/扫描扫描	symbolic	电流 扫描设备
/dev/modem	modem 端口	symbolic	电流 dialout 设备
/dev/root	root 设备	symbolic	电流 root 文件系统
/dev/swap	swap 设备	symbolic	电流 swap 设备
=============== =============== =============== ===============================

`/dev/modem` 应当 使用 用于 一modem supports dialin 作为
well 作为 dialout, 作为 tends cause 文件 problems.  
exists, `/dev/modem` 应当 point the appropriate primary TTY 设备
(the 使用 the alternate callout 设备 已废.

用于 SCSI 设备, `/dev/tape` `/dev/cdrom` 应当 point the
**cooked** 设备 (`/dev/st**` `/dev/sr**`, respectively), whereas
`/dev/scanner` 应当 point the appropriate generic
SCSI 设备 (`/dev/sg*`).

`/dev/mouse` point 一primary 串行 TTY 设备, 一硬件 鼠标
设备, 一套接用于 一鼠标 驱动 program (e.g. `/dev/gpmdata`).

Sockets 鍜?pipes
+++++++++++++++++

Non-transient sockets named pipes exist /dev.  通用 条目 

=============== =============== ===============================================
/dev/打印套接	lpd 本地 套接
/dev/log	套接	syslog 本地 套接
/dev/gpmdata	濂楁帴瀛?	gpm 榧犳爣 multiplexer
=============== =============== ===============================================

Mount points
++++++++++++

The 以下 names reserved 用于 mounting 特殊 文件系统
在…下 /dev.  这些 特殊 文件系统 提供 内核 interfaces 
cannot provided 标准 设备 nodes.

=============== =============== ===============================================
/dev/pts	devpts		PTY slave 文件系统
/dev/shm	tmpfs		POSIX shared 内存 maintenance access
=============== =============== ===============================================

### Terminal 设备


Terminal, TTY 设备 一特殊 character 设备.  一
terminal 设备 任何 设备 可以 act 作为 一controlling terminal
用于 一会话; 包含 虚拟 consoles, 串行 ports, 
pseudoterminals (PTYs).

全部 terminal 设备 share 一通用 set capabilities known 作为 line
disciplines; 这些 包含 the 通用 terminal line discipline 作为 well
作为 SLIP PPP modes.

全部 terminal 设备 named similarly; section explains the
naming 使用 the 各种 types TTYs.  注意 the naming
conventions 包含 若干 historical warts; 一这些 
Linux-specific, 一曾是 inherited 来自 其他 系统, 一
reflect Linux outgrowing 一borrowed convention.

一hash mark (`#`) 一设备 name 使用 此处 indicate 一decimal
鏁板瓧 鏃?leading zeroes.

虚拟 consoles the console 设备
+++++++++++++++++++++++++++++++++++++++

虚拟 consoles full-screen terminal displays the 系统 视频
监视  虚拟 consoles named `/dev/tty#`, numbering
starting `/dev/tty1`; `/dev/tty0` the 电流 虚拟 console.
`/dev/tty0` the 设备 应当 使用 access the 系统 视频
那些 architectures 用于 the 缓冲设备
(`/dev/fb*`) applicable. 执行 使用 `/dev/console`
用于 purpose.

The console 设备, `/dev/console`, the 设备 系统
messages 应当 sent, logins 应当 permitted 
single-user 模式.  Starting Linux 2.1.71, `/dev/console` managed
the 内核; 用于 前一versions 应当 一symbolic link 
任一`/dev/tty0`, 一特定 虚拟 console 例如 `/dev/tty1`, 
一串行 端口 primary (`tty**`, `cu**`) 设备, depending the
配置 the 系统.

串行 ports
++++++++++++

串行 ports RS-232 串行 ports 任何 设备 simulates
one, 任一硬件 (例如 内部 modems) 软件 (此类
作为 the ISDN 驱动.)  在…下 Linux, 每个 串行 ports 具有 two 设备
names, the primary callin 设备 the alternate callout one.
每个 kind 设备 indicated 一不同 letter.	 用于 任何
letter X, the names the 设备 `/dev/ttyX#` `/dev/cux#`,
respectively; 用于 historical reasons, `/dev/ttyS#` `/dev/ttyC#`
correspond `/dev/cua#` `/dev/cub#`. the future, 应当 
expected 多个 letters 使用; 全部 letters upper
case 用于 the "tty" 设备 (e.g. `/dev/ttyDP#`) lower case 用于 the
"cu" 设备 (e.g. `/dev/cudp#`).

The names `/dev/ttyQ#` `/dev/cuq#` reserved 用于 本地 使用.

The alternate 设备 提供 用于 kernel-based exclusion somewhat
不同 defaults the primary 设备.  它们主要 purpose 
允许 the 使用 串行 ports programs inherent broken
支持 用于 串行 ports.  它们使用 已废 它们 
removed 来自 一future 版本 Linux.

Arbitration 串行 ports provided the 使用 文件 
the names `/var/lock/LCK..ttyX#`. The contents the 文件 应当
the PID the locking 进程 作为 一ASCII 数字.

它是 通用 practice install links 例如 /dev/modem
point 串行 ports.  为了 ensure proper locking the
presence 这些 links, 它是 recommended 软件 chase
symlinks 全部 可能 names; additionally, 它是 recommended
一文件 installed the corresponding alternate
设备.	 为了 avoid deadlocks, 它是 recommended the 
acquired the 以下 order, released the reverse:

 1. The symbolic link name, 任何 (`/var/lock/LCK..modem`)
 2. The "tty" name (`/var/lock/LCK..ttyS2`)
 3. The alternate 设备 name (`/var/lock/LCK..cua2`)

在该情况nested symbolic links, the 文件 应当 
installed 鍦?the order the symlinks 鏄?resolved.

在…下 circumstances 应当 一应用程序 hold 一同时 waiting
用于 another released.  此外, applications attempt
创建 文件 用于 the corresponding alternate 设备 names
应当 take 进入 account the possibility 正在 使用 一non-serial
端口 TTY, 用于 alternate 设备 将会 exist.

Pseudoterminals (PTYs)
++++++++++++++++++++++

Pseudoterminals, PTYs, 使用 创建 login sessions 提供
其他 capabilities requiring 一TTY line discipline (including SLIP 
PPP capability) arbitrary data-generation 进程.	 每个 PTY 具有
一master side, named `/dev/pty[p-za-e][0-9a-f]`, 一slave side, named
`/dev/tty[p-za-e][0-9a-f]`.  The 内核 arbitrates the 使用 PTYs 
allowing 每个 master side opened 一

一the master side 具有 已经 opened, the corresponding slave 设备
使用 the 相同 manner 作为 任何 TTY 设备.  The master 
slave 设备 connected the 内核, generating the equivalent
一bidirectional pipe TTY capabilities.

Recent versions the Linux kernels GNU libc 包含 支持 用于
the 系统 V/Unix98 naming scheme 用于 PTYs, assigns 一通用
设备, `/dev/ptmx`, 全部 the masters (opening automatically
give 一previously unassigned PTY) 一subdirectory, `/dev/pts`,
用于 the slaves; the slaves named decimal integers (`/dev/pts/#`
我们notation).  removes the problem exhausting the
namespace enables the 内核 automatically 创建 the 设备
nodes 用于 the slaves demand 使用 the "devpts" 文件系统.
