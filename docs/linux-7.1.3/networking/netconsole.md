
## Netconsole


started by Ingo Molnar <mingo@redhat.com>, 2001.09.17

2.6 port and netpoll api by Matt Mackall <mpm@selenic.com>, Sep 9 2003

IPv6 support by Cong Wang <xiyou.wangcong@gmail.com>, Jan 1 2013

Extended console support by Tejun Heo <tj@kernel.org>, May 1 2015

Release prepend support by Breno Leitao <leitao@debian.org>, Jul 7 2023

Userdata append support by Matthew Wood <thepacketgeek@gmail.com>, Jan 22 2024

Sysdata append support by Breno Leitao <leitao@debian.org>, Jan 15 2025

## 简介：


该模块通过 UDP 记录内核 printk 消息，便于在磁盘日志失败且串口控制台不实用的情况下进行调试
它既可以作为内置功能使用，也可以作为模块使用。作为内置功能时，netconsole 在网卡之后立即初始化，并尽快启用指定的接口。虽然这无法捕获早期的内panic，但它确实能捕获大部分的启动过程
## 发送方与接收方配置

它接受一个字符串配置参数 "netconsole"，格式如下：
```
 netconsole=[+][r][src-port]@[src-ip]/[<dev>],[tgt-port]@<tgt-ip>/[tgt-macaddr]

   where
	+             if present, enable extended console support
	r             if present, prepend kernel version (release) to the message
	src-port      source for UDP packets (defaults to 6665)
	src-ip        source IP to use (interface address)
	dev           network interface name (eth0) or MAC address
	tgt-port      port for logging agent (6666)
	tgt-ip        IP address for logging agent
	tgt-macaddr   ethernet MAC address for logging agent (broadcast)
```
```
 linux netconsole=4444@10.0.0.1/eth1,9353@10.0.0.2/12:34:56:78:9a:bc
```
```
 insmod netconsole netconsole=@/,@10.0.0.2/
```
```
 insmod netconsole netconsole=@/,@fd00:1:2:3::1/
```
```
 linux netconsole=4444@10.0.0.1/22:33:44:55:66:77,9353@10.0.0.2/12:34:56:78:9a:bc
```
它还支持通过用分号分隔多个代理的参数，把日志发送到多个远程代理```
 modprobe netconsole netconsole="@/,@10.0.0.2/;@/eth1,6892@10.0.0.3/"
```
内置netconsole TCP 协议栈初始化后立即启动，并尝试在所提供的地址上启用所提供dev
远程主机有几种接收内核消息的方式，例如：

1) syslogd

2) netcat

   在使用基BSD netcat 版本（例Fedora、openSUSE Ubuntu）的发行版上，必须以不带以下形式的方式指定监听端```
	nc -u -l -p <port>' / 'nc -u -l <port>

   or::

	netcat -u -l -p <port>' / 'netcat -u -l <port>
```
3) socat

```
   socat udp-recv:<port> -
```
## 动态重配置

动态可重配置是 netconsole 的一个有用补充，它使远程日志目标能够通过基于 configfs 的用户空间接口在运行时被动态添加、移除或其参数被重新配置
要包含此特性，请在构建 netconsole 模块（或内核，如netconsole 是内置的）时选择 CONFIG_NETCONSOLE_DYNAMIC
以下是一些示例（其中 configfs 挂载/sys/kernel/config 挂载点）
```
 cd /sys/kernel/config/netconsole/
 mkdir target1
```
请注意，新创建的目标具有默认的参数值（如上所述），并且默认是禁用的——它们必须首先通过"1" 写入 "enabled" 属性（通常在相应地设置参数之后）来启用，如下所述
```
 rmdir /sys/kernel/config/netconsole/othertarget/
```
该接口向用户空间暴露netconsole 目标的以下参数：

	=============== =================================       ============
	enabled		该目标当前是否已启用	（可读写	extended	是否启用扩展模式			（可读写	release		在消息前加上内核版本（release（可读写	dev_name	本地网络接口名称			（可读写	local_port	要使用的UDP 端口			（可读写	remote_port	远程代理UDP 端口			（可读写	local_ip	要使用的IP 地址			（可读写	remote_ip	远程代理IP 地址			（可读写	local_mac	本地接口MAC 地址			（只读）
	remote_mac	远程代理MAC 地址			（可读写	transmit_errors	数据包发送错误次		（只读）
	=============== =================================       ============

"enabled" 属性还用于控制能否更新目标的参数——你只能修改已禁用目标（"enabled" 0）的参数
```
 cat enabled				# check if enabled is 1
 echo 0 > enabled			# disable the target (if required)
 echo eth2 > dev_name			# set local interface
 echo 10.0.0.4 > remote_ip		# update some parameter
 echo cb:a9:87:65:43:21 > remote_mac	# update more parameters
 echo 1 > enabled			# enable target again
```
你也可以动态地更新本地接口。如果你想要使用新近启动（且netconsole 加载/初始化时可能还不存在）的接口，这尤其有用
在引导时（或模块加载时）通过 `netconsole=` 参数定义的目标会被赋予名`cmdline<index>`。例如，参数中的第一个目标被命名`cmdline0`。你可以通过创建同名 configfs 目录来控制和修改这些目标
```
 netconsole=4444@10.0.0.1/eth1,9353@10.0.0.2/12:34:56:78:9a:bc;4444@10.0.0.1/eth1,9353@10.0.0.3/12:34:56:78:9a:bc
```
```
 mkdir cmdline0
 cat cmdline0/remote_ip
 10.0.0.2

 mkdir cmdline1
 cat cmdline1/remote_ip
 10.0.0.3
```
### 追加用户数据


在启用了 netconsole 动态配置的情况下，可以将自定义用户数据追加到消息的末尾。用户数据条目可以在不更改目"enabled" 属性的情况下被修改
位于 `userdata` 下的目录（键）长度限制为 53 个字符，并且
```
 cd /sys/kernel/config/netconsole && mkdir cmdline0
 cd cmdline0
 mkdir userdata/foo
 echo bar > userdata/foo/value
 mkdir userdata/qux
 echo baz > userdata/qux/value
```
```
 echo "This is a message" > /dev/kmsg
```
```
 12,607,22085407756,-;This is a message
  foo=bar
  qux=baz
```
```
 cd /sys/kernel/config/netconsole/cmdline0/userdata
 for f in `ls userdata`; do echo $f=$(cat userdata/$f/value); done
```
如果创建`userdata` 条目但没有向 `value` 文件写入数据```
 cd /sys/kernel/config/netconsole && mkdir cmdline0
 cd cmdline0
 mkdir userdata/foo
 echo bar > userdata/foo/value
 mkdir userdata/qux
```
```
 echo "This is a message" > /dev/kmsg
 12,607,22085407756,-;This is a message
  foo=bar
```
```
 rmdir /sys/kernel/config/netconsole/cmdline0/userdata/qux
```
   向用户数据值写入字符串时，输入会按行拆```
     mkdir userdata/testing
     printf "val1\nval2" > userdata/testing/value
     # userdata store value is called twice, first with "val1\n" then "val2"
     # so "val2" is stored, being the last value stored
     cat userdata/testing/value
     val2

   建议不要写入带有换行符的用户数据值```
### userdata 中自动填充任务名


netconsole configfs 层级中，有一个名`taskname_enabled` 的文件，位于 `userdata` 目录下。该文件用于启用或禁用自动任务名填充特性。该特性会自动填充当前正在负责发送消息的 CPU 上被调度的任务的名称
```
  echo 1 > /sys/kernel/config/netconsole/target1/userdata/taskname_enabled
```
当启用该选项后，netconsole 消息会在 userdata 字段中包含一行额外内容，格式`taskname=<任务`。这使得 netconsole 消息的接收方能够轻松找出生成该消息时当前被调度的应用程序，从而为内核消息提供额外的上下文并有助于对其分类
```
  echo "This is a message" > /dev/kmsg
  12,607,22085407756,-;This is a message
   taskname=echo
```
在此示例中，该消息是"echo" 作为当前被调度进程时生成的
### userdata 中自动填充内核版本（release

netconsole configfs 层级中，有一个名`release_enabled` 的文件，位于 `userdata` 目录下。该文件控制内核版本（release）自动填充特性，它会将内核版本信息追加到所发送每条消息的 userdata 字典中
```
  echo 1 > /sys/kernel/config/netconsole/target1/userdata/release_enabled
```
```
  echo "This is a message" > /dev/kmsg
  12,607,22085407756,-;This is a message
   release=6.14.0-rc6-01219-g3c027fbd941d
```
   该特性提供的数据"release prepend" 特性相同。不过，在这种情况下，版本信息是被追加到 userdata 字典中，而不是包含在消息头里
### userdata 中自动填CPU 编号


netconsole configfs 层级中，有一个名`cpu_nr` 的文件，位于 `userdata` 目录下。该文件用于启用或禁CPU 编号自动填充特性。该特性会自动填充正在发送消息的 CPU 的编号
```
  echo 1 > /sys/kernel/config/netconsole/target1/userdata/cpu_nr
```
当启用该选项后，netconsole 消息会在 userdata 字段中包含一行额外内容，格式`cpu=<cpu_number>`。这使得 netconsole 消息的接收方能够轻松区分和解复用来自不同 CPU 的消息，在处理并行日志输出时尤其有用
```
  echo "This is a message" > /dev/kmsg
  12,607,22085407756,-;This is a message
   cpu=42
```
在此示例中，该消息由 CPU 42 发送
   如果用户已在 userdata 字典中设置了一个冲突的 `cpu` 键，两个键都会被报告，其中内核填充的条目出现在其```
     # User-defined CPU entry
     mkdir -p /sys/kernel/config/netconsole/target1/userdata/cpu
     echo "1" > /sys/kernel/config/netconsole/target1/userdata/cpu/value

   Output might look like::

     12,607,22085407756,-;This is a message
      cpu=1
      cpu=42    # kernel-populated value
```
### userdata 中自动填充消ID


netconsole configfs 层级中，有一个名`msgid_enabled` 的文件，位于 `userdata` 目录下。该文件控制消息 ID 自动填充特性，它会为发送到给定目标的每条消息分配一个数ID，并将该 ID 追加到所发送每条消息的 userdata 字典中
消息 ID 使用每个目标一个的 32 位计数器生成，每向该目标发送一条消息就递增一次。请注意，该计数器在达到 uint32_t 最大值后会回绕，因此消息 ID 在长时间范围内并非全局唯一。不过，目标仍然可以利用它，通过识别 ID 序列中的间隙来检测消息是否在到达目标之前被丢弃
区分消息 ID 与消息的 <sequnum> 字段很重要。某些内核消息可能永远不会到netconsole（例如由printk 限速）。因此，<sequnum> 中的间隙不能单独用来指示消息在传输过程中被丢弃，因为它可能从未通过 netconsole 发送过。另一方面，消ID 只分配给实际通过 netconsole 传输的消息
```
  echo "This is message #1" > /dev/kmsg
  echo "This is message #2" > /dev/kmsg
  13,434,54928466,-;This is message #1
   msgid=1
  13,435,54934019,-;This is message #2
   msgid=2
```
## 扩展控制台：


如果配置行前缀'+'，或"extended" 配置文件被设1，则启用扩展控制台支持。一个引```
 linux netconsole=+4444@10.0.0.1/eth1,9353@10.0.0.2/12:34:56:78:9a:bc
```
日志消息会以扩展元数据头的形式传```
 <level>,<sequnum>,<timestamp>,<contflag>;<message text>
```
如果启用'r'（release）特性，则会在消息中包含内核版本```
 6.4.0,6,444,501151268,-;netconsole: network logging started
```
<message text> 中的不可打印字符使用 "\xff" 记法进行转义。如果消息包含可选的字典，则使用原样的换行符作为分隔符
如果一条消息无法放入一定数量的字节（当前为 1000）中，netconsole 会将其拆分为多个分片。这```
 ncfrag=<byte-offset>/<total-bytes>
```
例如，假设分块大小小得多，消"the first
```
 6,416,1758426,-,ncfrag=0/31;the first chunk,
 6,416,1758426,-,ncfrag=16/31; the 2nd chunk.
```
## 杂项说明

   默认目标的以太网设置使用广播以太网地址来发送数据包，这会导致同一以太网段上其他系统的负载增加

   某些 LAN 交换机可能被配置为抑制以太网广播，因此建议通过传给 netconsole 的配置参数显式指定远程代理的 MAC 地址

```
	ping -c 1 10.0.0.2 ; /sbin/arp -n | grep 10.0.0.2
```
   如果远程日志代理与发送方位于不同LAN 子网，建议尝试将默认网关MAC 地址（你可以使用 /sbin/route -n 查到）指定为远程 MAC 地址

   网络设备（上述例子中eth1）可以运行任何类型的其他网络流量，netconsole 不会造成干扰。如果内核消息量很大，netconsole 可能会导致其他流量出现轻微延迟，但不应产生其他影响

   如果你发现远程日志代理没有接收或打印出发送方的所有消息，很可能是因为你将发送方上的 "console_loglevel" 参数设置得只发送高
```
	dmesg -n 8

   or by specifying "debug" on the kernel command line at boot, to send
   all kernel messages to the console. A specific value for this parameter
   can also be set using the "loglevel" kernel boot option. See the
   dmesg(8) man page and Documentation/admin-guide/kernel-parameters.rst
   for details.
```
Netconsole 被设计为尽可能即时，以便能够记录即使是最关键的内bug。它也可以在 IRQ 上下文中工作，并且在发送数据包时不启用中断。由于这些独特的需求，配置无法更加自动化，并且一些基本限制将长期存在：仅支持 IP 网络、UDP 数据包和以太网设备