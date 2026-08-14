
## Linux 以太网 Bonding 驱动使用手册（HOWTO）


最近更新：2011 年 4 月 27 日

初次发布：Thomas Davis <tadavis at lbl.gov>

修订与高可用扩展：2000/10/03-15：

  - Willy Tarreau <willy at meta-x.org>
  - Constantine Gavrilov <const-g at xpert.com>
  - Chad N. Tindel <ctindel at ieee dot org>
  - Janice Girouard <girouard at us dot ibm dot com>
  - Jay Vosburgh <fubar at us dot ibm dot com>

由 Jay Vosburgh 于 2005 年 2 月重新组织并更新
新增 Sysfs 信息：2006/04/24

  - Mitch Williams <mitch.a.williams at intel.com>

## 简介


Linux bonding 驱动提供了一种将多个网络接口聚合为单一逻辑
“bonded”接口的方法。bonded 接口的行为取决于所使用的模式；一般
而言，各种模式提供热备份或负载均衡服务。此外，还可以执行链路
完整性监控。

bonding 驱动最初来源于 Donald Becker 为 2.0 内核编写的
beowulf 补丁。此后它已经发生了很大变化，来自 extreme-linux 和
beowulf 站点的原始工具将无法与此版本的驱动一起工作。

有关驱动的新版本、更新后的用户态工具，以及向谁寻求帮助，请
参见本文件末尾的链接。


   1. Bonding 驱动安装

   2. Bonding 驱动选项

   3. 配置 Bonding 设备
   3.1	使用 Sysconfig 支持进行配置
   3.1.1		在 Sysconfig 中使用 DHCP
   3.1.2		在 Sysconfig 中配置多个 Bond
   3.2	使用 Initscripts 支持进行配置
   3.2.1		在 Initscripts 中使用 DHCP
   3.2.2		在 Initscripts 中配置多个 Bond
   3.3	使用 Ifenslave 手动配置 Bonding
   3.3.1		手动配置多个 Bond
   3.4	通过 Sysfs 手动配置 Bonding
   3.5	使用 Interfaces 支持进行配置
   3.6	特殊情况下的配置覆盖
   3.7 以更安全的方式为 802.3ad 模式配置 LACP

   4. 查询 Bonding 配置
   4.1	Bonding 配置
   4.2	网络配置

   5. 交换机配置

   6. 802.1q VLAN 支持

   7. 链路监控
   7.1	ARP 监控工作机制
   7.2	配置多个 ARP 目标
   7.3	MII 监控工作机制

   8. 潜在的故障来源
   8.1	路由方面的坑
   8.2	以太网设备重命名
   8.3	Miimon 检测链路失败极慢或完全失效

   9. SNMP 代理

   10. 混杂模式

   11. 为 High Availability 配置 Bonding
   11.1	单交换机拓扑中的高可用
   11.2	多交换机拓扑中的高可用
   11.2.1		多交换机拓扑的 HA Bonding 模式选择
   11.2.2		多交换机拓扑的 HA 链路监控选择

   12. 为最大吞吐量配置 Bonding
   12.1	单交换机拓扑中的最大吞吐量
   12.1.1		单交换机拓扑的 MT Bonding 模式选择
   12.1.2		单交换机拓扑的 MT 链路监控
   12.2	多交换机拓扑中的最大吞吐量
   12.2.1		多交换机拓扑的 MT Bonding 模式选择
   12.2.2		多交换机拓扑的 MT 链路监控

   13. 交换机行为问题
   13.1	链路建立与故障切换延迟
   13.2	重复的入站数据包

   14. 硬件相关注意事项
   14.1	IBM BladeCenter

   15. 常见问题解答

   16. 资源与链接


## 1. Bonding 驱动安装


大多数流行的发行版内核都附带 bonding 驱动，已经作为模块
提供。如果你的发行版没有附带，或者你需要从源码编译 bonding（例如，
配置并安装来自 kernel.org 的 mainline 内核），则需要执行以下步骤：

### 1.1 配置并构建带 bonding 的内核


最新版本的 bonding 驱动位于最近的内核源码的
drivers/net/bonding 子目录中（可在 http://kernel.org 获取）。大多数
“自行构建”的用户会希望使用来自 kernel.org 的最新内核。

使用 "make menuconfig"（或 "make xconfig" 或
"make config"）配置内核，然后在 "Network
device support" 一节中选择 "Bonding driver support"。建议将驱动
配置为模块，因为这是目前向驱动传递参数或配置多个 bonding 设备的
唯一方式。

构建并安装新的内核与模块。

### 1.2 Bonding 控制工具


建议通过 iproute2（netlink）或 sysfs 配置 bonding，旧的
ifenslave 控制工具已经过时。

## 2. Bonding 驱动选项


bonding 驱动的选项在加载时作为 bonding 模块的参数提供，或
通过 sysfs 指定。

模块选项可以作为命令行参数传给 insmod 或 modprobe 命令，但
通常写在 `/etc/modprobe.d/*.conf` 配置文件中，或写在某个发行版
特定的配置文件中（其中部分会在下一节详细介绍）。

有关 bonding 对 sysfs 的支持详见下面的
“通过 Sysfs 手动配置 Bonding”一节。

可用的 bonding 驱动参数如下所列。如果未指定某个参数，则使用其
默认值。在初次配置 bond 时，建议在一个独立窗口中运行
"tail -f /var/log/messages" 以观察 bonding 驱动的错误消息。

务必指定 miimon 或 arp_interval 与 arp_ip_target 参数中的
至少一个，否则在链路故障期间会出现严重的网络性能下降。很少有设备
不支持至少 miimon，因此实在没有理由不使用它。

带有文本值的选项既接受文本名称，也（为了向后兼容）接受选项
数值。例如，"mode=802.3ad" 与 "mode=4" 设置的是同一个模式。

参数如下：

active_slave

	指定支持该选项的模式的新的 active slave（active-backup、
	balance-alb 和 balance-tlb）。可能的值为当前任一被 enslaved 的
	接口名称，或空字符串。如果给定了名称，则该 slave 及其链路必须
	处于 up 状态才能被选中为新的 active slave。如果指定了空字符串，
	则清除当前的 active slave，并自动选择一个新的 active slave。

	注意，此选项仅通过 sysfs 接口提供。不存在以此命名的模块参数。

	此选项的正常值为当前 active slave 的名称，或者在没有
	active slave 或当前模式不使用 active slave 时为空字符串。

ad_actor_sys_prio

	在 AD 系统中，此参数指定系统优先级。允许的范围是
	1 - 65535。如果未指定该值，则默认取 65535。

	此参数仅在 802.3ad 模式下生效，并且通过 SysFs 接口提供。

actor_port_prio

	在 AD 系统中，此参数指定端口优先级。允许的范围是
	1 - 65535。如果未指定该值，则默认取 255。

	此参数仅在 802.3ad 模式下生效，并且通过 netlink 接口提供。

ad_actor_system

	在 AD 系统中，此参数指定 actor 在协议报文交换（LACPDU）中
	使用的 mac 地址。该值不能是组播地址。如果指定了全零 MAC，
	bonding 将在内部使用 bond 自身的 MAC。建议为此 mac 设置
	local-admin 位，但驱动并不强制要求。如果未给出该值，则系统
	默认使用 master 的 mac 地址作为 actor 的系统地址。

	此参数仅在 802.3ad 模式下生效，并且通过 SysFs 接口提供。

ad_select

	指定要使用的 802.3ad 聚合选择逻辑。可能的值及其效果如下：

	stable 或 0

		活动聚合器由最大的聚合带宽选出。

		仅当活动聚合器的所有 slave 都已 down，或活动聚合器
		已经没有 slave 时，才会重新选择活动聚合器。

		这是默认值。

	bandwidth 或 1

		活动聚合器由最大的聚合带宽选出。在以下情况会
		发生重新选择：

  - 向 bond 中增加或从 bond 中移除某个 slave

  - 任意 slave 的链路状态发生变化

  - 任意 slave 的 802.3ad 关联状态发生变化

  - bond 的管理状态变为 up

	count 或 2

		活动聚合器由最多的端口数（slave）选出。重新选择的
		发生方式如上文 “bandwidth” 设置所述。

	actor_port_prio 或 3

		活动聚合器由其活动端口上 actor 端口优先级总和最高
		者选出。注意此优先级是 actor_port_prio，而非每个端口
		的优先级（后者用于 primary reselect）。

	bandwidth、count 和 actor_port_prio 选择策略允许在活动聚合器
	发生部分故障时进行 802.3ad 聚合的故障切换。这保证了具有最高
	可用性的聚合器（无论是带宽、端口数还是端口优先级总值）始终
	处于活动状态。

	此选项添加于 bonding 版本 3.4.0。

ad_user_port_key

	在 AD 系统中，port-key 由如下三部分组成 -

	   =====  ============
	   Bits   用途
	   =====  ============
	   00     双工
	   01-05  速率
	   06-15  用户定义
	   =====  ============

	此参数定义 port key 的高 10 位。取值可以为 0 - 1023。如果未给出，
	则系统默认取 0。

	此参数仅在 802.3ad 模式下生效，并且通过 SysFs 接口提供。

all_slaves_active

	指定应丢弃（0）还是交付（1）重复帧（在 inactive 端口上接收到的）。

	通常情况下，bonding 会丢弃重复帧（在 inactive 端口上接收到的），
	这对大多数用户而言是期望的行为。但有时允许交付重复帧会更好。

	默认值为 0（丢弃在 inactive 端口上接收到的重复帧）。

arp_interval

	指定 ARP 链路监控的频率，单位为毫秒。

	ARP 监控通过定期检查 slave 设备来确定它们最近是否发送或
	接收过流量（确切的判定标准取决于 bonding 模式以及 slave 的
	状态）。常规流量由针对 arp_ip_target 选项所指定地址发出的
	ARP 探测报文产生。

	此行为可由下面的 arp_validate 选项修改。

	如果在 etherchannel 兼容模式（模式 0 和 2）下使用 ARP 监控，
	则交换机应配置为能够均匀地将数据包分布到所有链路上的模式。如果
	交换机配置为以 XOR 方式分发数据包，则来自 ARP 目标的全部应答
	都会在同一个链路上收到，这可能导致其他组成员失败。ARP 监控不应
	与 miimon 配合使用。取值为 0 时禁用 ARP 监控。默认值为 0。

arp_ip_target

	指定在 arp_interval 大于 0 时用作 ARP 监控对等体的 IP 地址。
	这些地址是用于确定到各目标的链路健康状况的 ARP 请求的目标。
	以 ddd.ddd.ddd.ddd 格式指定这些值。多个 IP 地址必须以逗号
	分隔。必须至少给出一个 IP 地址，ARP 监控才能正常工作。可以指定的
	目标最大数量为 16。默认值为无 IP 地址。

ns_ip6_target

	指定在 arp_interval 大于 0 时用作 IPv6 监控对等体的 IPv6 地址。
	这些地址是用于确定到各目标的链路健康状况的 NS 请求的目标。
	以 ffff:ffff::ffff:ffff 格式指定这些值。多个 IPv6 地址必须以
	逗号分隔。必须至少给出一个 IPv6 地址，NS/NA 监控才能正常工作。
	可以指定的目标最大数量为 16。默认值为无 IPv6 地址。

arp_validate

	指定在任何支持 arp 监控的模式下，是否应校验 ARP 探测与应答，
	或者是否应过滤（忽略）非 ARP 流量用于链路监控目的。

	可能的值如下：

	none 或 0

		不执行任何校验或过滤。

	active 或 1

		仅对 active slave 执行校验。

	backup 或 2

		仅对 backup slave 执行校验。

	all 或 3

		对所有 slave 执行校验。

	filter 或 4

		对所有 slave 应用过滤。不执行任何校验。

	filter_active 或 5

		对所有 slave 应用过滤，仅对 active slave 执行校验。

	filter_backup 或 6

		对所有 slave 应用过滤，仅对 backup slave 执行校验。

	校验：

	启用校验会使 ARP 监控检查入站的 ARP 请求与应答，并且仅当某个
	slave 正在接收相应的 ARP 流量时才认为它处于 up 状态。

	对于 active slave，校验会检查 ARP 应答，以确认它们是由某个
	arp_ip_target 生成的。由于 backup slave 通常不会接收到这些应答，
	因此对 backup slave 执行的校验是针对经由 active slave 发出的
	广播 ARP 请求的。某些交换机或网络配置可能导致 backup slave
	收不到 ARP 请求的情况；在这种情况下，必须禁用对 backup slave 的
	校验。

	对 backup slave 的 ARP 请求校验主要是为了帮助 bonding 在 active
	slave 发生故障时判断哪些 slave 更可能正常工作，它并不能真正保证
	该 backup slave 在被选为下一个 active slave 时一定能工作。

	校验在网络配置中有多个 bonding 主机同时向一个或多个超出公共
	交换机的目标发出 ARP 时很有用。如果交换机与目标之间的链路失败
	（但交换机本身未失败），多个 bonding 实例产生的探测流量会欺骗
	标准的 ARP 监控，使其认为链路仍然 up。使用校验可以解决这个问题，
	因为 ARP 监控只会考虑与其自身 bonding 实例相关的 ARP 请求与应答。

	过滤：

	启用过滤会使 ARP 监控仅使用入站的 ARP 数据包来判断链路可用性。
	到达的非 ARP 数据包会照常交付，但在判断某个 slave 是否可用时
	不被计入。

	过滤仅考虑在判断链路可用性时是否接收到了 ARP 数据包（任意 ARP
	数据包，无论源或目的）。

	过滤在以下网络配置中很有用：大量第三方的广播流量会欺骗标准的
	ARP 监控，使其认为链路仍然 up。使用过滤可以解决这个问题，因为
	只有 ARP 流量才会被用于判断链路可用性。

	此选项添加于 bonding 版本 3.1.0。

arp_all_targets

	指定为了使 ARP 监控认为某个 slave 处于 up 状态，必须可达的
	arp_ip_target 的数量。此选项仅影响启用了 arp_validation 的
	active-backup 模式下的 slave。

	可能的值如下：

	any 或 0

		仅当任意一个 arp_ip_target 可达时才认为该 slave 处于 up 状态

	all 或 1

		仅当所有 arp_ip_target 都可达时才认为该 slave 处于 up 状态

arp_missed_max

	指定必须有多少次 arp_interval 监控检查失败，接口才会被 ARP 监控
	标记为 down。

	为了提供有序的故障切换语义，backup 接口被允许多一次监控检查
	（即它们必须失败 arp_missed_max + 1 次才会被标记为 down）。

	默认值为 2，允许的范围是 1 - 255。

coupled_control

    指定 802.3ad 模式下的 LACP 状态机的 MUX 是否应具有独立的 Collecting
    与 Distributing 状态。

    这是通过除了现有的 coupled control 状态机之外，还实现遵循
    IEEE 802.1AX-2008 5.4.15 的独立控制状态机来完成的。

    默认值为 1。此设置不会分离 Collecting 与 Distributing 状态，
    使 bond 保持在 coupled control 状态。

downdelay

	指定在检测到链路故障后，禁用某个 slave 之前等待的时间，单位为
	毫秒。此选项仅对 miimon 链路监控有效。downdelay 的值应为 miimon
	值的整数倍；否则将被向下取整为最接近的整数倍。默认值为 0。

fail_over_mac

	指定 active-backup 模式是否应在 enslavement 时将全部 slave 设置为
	相同的 MAC 地址（传统行为），或者在启用时根据所选策略对 bond 的
	MAC 地址执行特殊处理。

	可能的值如下：

	none 或 0

		此设置禁用 fail_over_mac，并使 bonding 在 enslavement 时
		将 active-backup bond 的所有 slave 设置为相同的 MAC 地址。
		这是默认值。

	active 或 1

		“active” fail_over_mac 策略表示 bond 的 MAC 地址应始终为
		当前 active slave 的 MAC 地址。slave 的 MAC 地址不会被改变；
		相反，bond 的 MAC 地址会在故障切换期间发生变化。

		此策略适用于那些永远不能更改其 MAC 地址的设备，或者拒绝
		接收以其自身源 MAC 为目的的入站广播的设备（这会干扰 ARP 监控）。

		此策略的缺点在于，网络上的每个设备都必须通过免费 ARP 来更新，
		而传统方法通常只需更新一个或多个交换机（如果交换机会侦听
		入站流量以更新其表项，则这通常对任何其他流量、而不仅仅是 ARP
		流量都会发生）。如果免费 ARP 丢失，通信可能会中断。

		当此策略与 mii 监控配合使用时，那些在真正能够收发之前就断言
		链路 up 的设备特别容易丢失免费 ARP，因此可能需要设置合适的
		updelay。

	follow 或 2

		“follow” fail_over_mac 策略使 bond 的 MAC 地址以常规方式
		选出（通常为加入 bond 的第一个 slave 的 MAC 地址）。但是，
		第二及后续的 slave 在处于 backup 角色时不会被设置为该 MAC 地址；
		slave 会在故障切换时被写入 bond 的 MAC 地址（而原先的 active
		slave 则收到新 active slave 的 MAC 地址）。

		此策略适用于那些在多个端口被设置为相同 MAC 地址时会感到困惑
		或产生性能下降的多端口设备。

	默认策略为 none，除非第一个 slave 无法更改其 MAC 地址，此时默认
	选择 active 策略。

	此选项仅可在 bond 中没有 slave 时通过 sysfs 修改。

	此选项添加于 bonding 版本 3.2.0。其中 “follow” 策略添加于 bonding
	版本 3.3.0。

lacp_active
	指定是否定期发送 LACPDU 帧的选项。

	off 或 0
		LACPDU 帧的行为类似于“被问才说”（speak when spoken to）。

	on 或 1
		LACPDU 帧会沿着配置好的链路定期发送。更多细节参见 lacp_rate。

	默认为 on。

lacp_rate

	指定在 802.3ad 模式下我们请求链路对端发送 LACPDU 数据包的速率的选项。
	可能的值如下：

	slow 或 0
		请求对端每 30 秒发送一次 LACPDU

	fast 或 1
		请求对端每 1 秒发送一次 LACPDU

	默认为 slow。

broadcast_neighbor

	指定是否向所有 active slave 广播 ARP/ND 数据包的选项。此选项在
	802.3ad 模式之外的其他模式中不起作用。默认为 off（0）。

max_bonds

	指定为此 bonding 驱动实例创建多少个 bonding 设备。例如，如果
	max_bonds 为 3，且 bonding 驱动尚未加载，则将创建 bond0、bond1
	和 bond2。默认值为 1。指定值为 0 会加载 bonding，但不会创建任何
	设备。

miimon

	指定 MII 链路监控的频率，单位为毫秒。它决定了每个 slave 的链路
	状态被检查以发现链路故障的频率。取值为 0 会禁用 MII 链路监控。
	取值为 100 是一个不错的起始值。

	如果未设置 arp_interval，则默认值为 100。

min_links

	指定在断言 carrier 之前必须处于 active 状态的最小链路数。它类似于
	Cisco EtherChannel 的 min-links 特性。这允许设置在将 bond 设备
	标记为 up（carrier on）之前必须处于 up（链路 up 状态）状态的成员
	端口的最小数量。这对于集群等高层服务希望在切换之前确保有最少数量
	的低带宽链路处于活动状态的情况很有用。此选项仅影响 802.3ad 模式。

	默认值为 0。这将在 802.3ad 模式下，只要存在活动聚合器，无论其中
	可用链路数量多少，都会导致 carrier 被断言。注意，由于聚合器在没有
	至少一个可用链路的情况下不可能处于活动状态，因此将此选项设为 0
	或 1 的效果完全相同。

mode

	指定 bonding 策略之一。默认值为 balance-rr（round robin）。可能的值
	如下：

	balance-rr 或 0

		轮询策略：按顺序从第一个可用 slave 到最后一个 slave 依次
		发送数据包。此模式提供负载均衡与容错能力。

	active-backup 或 1

		active-backup 策略：bond 中只有一个 slave 处于活动状态。
		当且仅当 active slave 失败时，另一个 slave 才会变为活动状态。
		bond 的 MAC 地址只在同一个端口（网络适配器）上对外可见，
		以避免让交换机感到困惑。

		在 bonding 版本 2.6.2 或更高版本中，当 active-backup 模式发生
		故障切换时，bonding 会在新的 active slave 上发出一个或多个
		免费 ARP。会为 bonding master 接口及其上配置的每个 VLAN 接口
		各发出一个免费 ARP，前提是这些接口至少配置了一个 IP 地址。
		为 VLAN 接口发出的免费 ARP 会带有相应的 VLAN id 标记。

		此模式提供容错能力。下面文档化的 primary 选项会影响此模式的
		行为。

	balance-xor 或 2

		XOR 策略：基于所选的发送哈希策略发送。默认策略为一个简单的
		[（源 MAC 地址 XOR 目的 MAC 地址 XOR 数据包类型 ID）取模
		slave 数]。可以通过下面文档化的 xmit_hash_policy 选项选择
		其他发送策略。

		此模式提供负载均衡与容错能力。

	broadcast 或 3

		广播策略：在所有 slave 接口上发送一切内容。此模式提供容错能力。

	802.3ad 或 4

		IEEE 802.3ad 动态链路聚合。创建共享相同速率与双工设置的聚合组。
		根据 802.3ad 规范，将所有位于活动聚合器中的 slave 都利用起来。

		出站流量的 slave 选择根据发送哈希策略进行，该策略可以通过下面
		文档化的 xmit_hash_policy 选项从默认的简单 XOR 策略更改。
		注意，并非所有发送策略都符合 802.3ad，特别是在 802.3ad 标准
		第 43.2.4 节关于数据包乱序的要求方面。不同的对端实现对不合规
		行为的容忍度各不相同。

		先决条件：

  1. 基础驱动中支持 Ethtool，能够获取每个 slave 的速率与双工。

  2. 支持 IEEE 802.3ad 动态链路聚合的交换机。

		大多数交换机需要某种配置才能启用 802.3ad 模式。

	balance-tlb 或 5

		自适应发送负载均衡：不需要任何特殊交换机支持的信道绑定。

		在 tlb_dynamic_lb=1 模式下，出站流量根据每个 slave 上的当前负载
		（相对于速率计算）进行分发。

		在 tlb_dynamic_lb=0 模式下，基于当前负载的负载均衡被禁用，流量
		仅使用哈希分发。

		入站流量由当前 slave 接收。如果接收 slave 失败，另一个 slave 会
		接管失败接收 slave 的 MAC 地址。

		先决条件：

		基础驱动中支持 Ethtool，能够获取每个 slave 的速率。

	balance-alb 或 6

		自适应负载均衡：包含 balance-tlb，外加针对 IPv4 流量的接收负载
		均衡（rlb），并且不需要任何特殊交换机支持。接收负载均衡通过
		ARP 协商实现。bonding 驱动拦截本地系统发出途中的 ARP 应答，并将
		源硬件地址改写为 bond 中某个 slave 的唯一硬件地址，使得不同的
		对等体使用不同的硬件地址来访问服务器。

		由服务器创建的连接的接收流量也会被均衡。当本地系统发送 ARP
		请求时，bonding 驱动会从 ARP 数据包中复制并保存对等体的 IP
		信息。当来自对等体的 ARP 应答到达时，其硬件地址被取出，bonding
		驱动向该对等体发起一个 ARP 应答，将其分配给 bond 中的某个 slave。
		使用 ARP 协商进行均衡的一个问题是，每次广播 ARP 请求时都会使用
		bond 的硬件地址。因此，对等体学习到的是 bond 的硬件地址，接收
		流量的均衡就坍缩到当前 slave 上。这通过向所有对等体发送更新
		（ARP 应答，带有各自分配的硬件地址）来解决，从而使流量被重新
		分发。当新的 slave 加入 bond，或某个 inactive slave 被重新激活时，
		接收流量也会重新分发。接收负载在 bond 中最高速率的一组 slave 间
		顺序（轮询）分发。

		当链路重新连接或有新的 slave 加入 bond 时，接收流量会通过向每个
		客户端发起带有所选 MAC 地址的 ARP 应答，在 bond 中的所有 active
		slave 间重新分发。updelay 参数（详见下文）必须设置为大于或等于
		交换机的转发延迟的值，以便发给对等体的 ARP 应答不会被交换机
		阻塞。

		先决条件：

  1. 基础驱动中支持 Ethtool，能够获取每个 slave 的速率。

  2. 基础驱动支持在设备处于打开状态时设置其硬件地址。这是必需的，
		以保证始终有一个 slave 使用 bond 硬件地址（curr_active_slave），
		同时为 bond 中的每个 slave 提供唯一的硬件地址。如果 curr_active_slave
		失败，其硬件地址会与选出的新 curr_active_slave 交换。

num_grat_arp,
num_unsol_na

	指定在故障切换事件后发出的对等体通知（免费 ARP 与主动发出的 IPv6
	邻居通告）的数量。一旦新 slave 上的链路 up（可能立即），就会在
	bonding 设备及其每个 VLAN 子设备上发送一个对等体通知。如果数量
	大于 1，则按 peer_notif_delay 指定的速率重复发送。

	有效范围是 0 - 255；默认值为 1。这些选项影响 active-backup 或
	802.3ad（启用 broadcast_neighbor 时）模式。这些选项分别添加于
	bonding 版本 3.3.0 与 3.4.0。

	从 Linux 3.0 与 bonding 版本 3.7.1 起，这些通知由 ipv4 与 ipv6 代码
	生成，重复次数无法独立设置。

packets_per_slave

	指定在切换到下一个 slave 之前，通过一个 slave 发送的数据包数量。
	当设为 0 时，则随机选择一个 slave。

	有效范围是 0 - 65535；默认值为 1。此选项仅在 balance-rr 模式下生效。

peer_notif_delay

	指定在故障切换事件后发出的每个对等体通知（免费 ARP 与主动发出的
	IPv6 邻居通告）之间的延迟，单位为毫秒。此延迟应为 MII 链路监控
	间隔（miimon）的整数倍。

	有效范围是 0 - 300000。默认值为 0，表示与 MII 链路监控间隔的值一致。

prio
	slave 优先级。数值越大表示优先级越高。primary slave 具有最高优先级。
	此选项也遵循 primary_reselect 规则。

	此选项只能通过 netlink 配置，并且仅对 active-backup(1)、balance-tlb (5)
	和 balance-alb (6) 模式有效。有效值范围为有符号 32 位整数。

	默认值为 0。

primary

	一个字符串（eth0、eth2 等），指定哪个 slave 是 primary 设备。只要该
	设备可用，它就始终是 active slave。仅当 primary 离线时才会使用备用
	设备。当某个 slave 优于另一个 slave 时（例如，某个 slave 的吞吐量
	高于另一个），这很有用。

	primary 选项仅对 active-backup(1)、balance-tlb (5) 和 balance-alb (6)
	模式有效。

primary_reselect

	指定 primary slave 的重新选择策略。它影响在 active slave 失败或
	primary slave 恢复时，如何选择 primary slave 成为 active slave。此选项
	旨在防止 primary slave 与其他 slave 之间反复横跳。可能的值如下：

	always 或 0（默认）

		primary slave 一旦恢复就立即成为 active slave。

	better 或 1

		primary slave 在其恢复且速率与双工优于当前 active slave 时，
		成为 active slave。

	failure 或 2

		primary slave 仅在当前 active slave 失败且 primary slave 处于 up
		状态时，才成为 active slave。

	在两种情况下会忽略 primary_reselect 设置：

		如果没有 slave 处于活动状态，则第一个恢复的 slave 会被设为
		active slave。

		在初次被 enslaved 时，primary slave 总是被设为 active slave。

	通过 sysfs 更改 primary_reselect 策略会导致根据新策略立即选择最佳的
	active slave。根据具体情况，这可能会也可能不会导致 active slave 发生变化。

	此选项添加于 bonding 版本 3.6.0。

tlb_dynamic_lb

	指定在 tlb 或 alb 模式下是否启用流的动态重排。该值对其他任何模式
	都不起作用。

	tlb 模式的默认行为是在该间隔内基于负载跨 slave 重排活动流。这带来了
	不错的 lb 特性，但可能导致数据包重排序。如果重排序是个问题，可使用
	此变量禁用流重排，仅依赖哈希分发提供的负载均衡。xmit-hash-policy
	可用于为设置选择合适的哈希。

	sysfs 项可用于按 bond 设备更改此设置，其初始值取自模块参数。sysfs
	项仅允许在 bond 设备处于 down 状态时更改。

	默认值为 "1"，即启用流重排；值为 "0" 时禁用它。此选项添加于 bonding
	驱动 3.7.1 版本。

updelay

	指定在检测到链路恢复后，启用某个 slave 之前等待的时间，单位为毫秒。
	此选项仅对 miimon 链路监控有效。updelay 的值应为 miimon 值的整数倍；
	否则将被向下取整为最接近的整数倍。默认值为 0。

use_carrier

	一个过时的选项，以前用于在 MII / ETHTOOL ioctl 与 netif_carrier_ok()
	之间选择以判断链路状态。

	现在所有链路状态检查都通过 netif_carrier_ok() 完成。

	为了向后兼容，此选项的值可以被检查或设置。唯一有效的设置是 1。

xmit_hash_policy

	选择用于 balance-xor、802.3ad 和 tlb 模式中 slave 选择的发送哈希策略。
	可能的值如下：

	layer2

		使用硬件 MAC 地址与数据包类型 ID 字段的 XOR 来生成哈希。公式为

		hash = 源 MAC[^5^] XOR 目的 MAC[^5^] XOR 数据包类型 ID
		slave 编号 = hash 取模 slave 数

		此算法会将到某个特定网络对等体的所有流量放在同一个 slave 上。

		此算法符合 802.3ad。

	layer2+3

		此策略结合使用 layer2 与 layer3 的协议信息来生成哈希。

		使用硬件 MAC 地址与 IP 地址的 XOR 来生成哈希。公式为

		hash = 源 MAC[^5^] XOR 目的 MAC[^5^] XOR 数据包类型 ID
		hash = hash XOR 源 IP XOR 目的 IP
		hash = hash XOR (hash RSHIFT 16)
		hash = hash XOR (hash RSHIFT 8)
		然后 hash 取模 slave 数。

		如果协议为 IPv6，则源地址与目的地址首先使用 ipv6_addr_hash 进行
		哈希。

		此算法会将到某个特定网络对等体的所有流量放在同一个 slave 上。
		对于非 IP 流量，公式与 layer2 发送哈希策略相同。

		此策略旨在提供比单独 layer2 更均衡的流量分布，尤其是在需要
		通过 layer3 网关设备才能到达大多数目的地的环境中。

		此算法符合 802.3ad。

	layer3+4

		此策略在可用时使用上层协议信息来生成哈希。这允许到某个特定
		网络对等体的流量跨多个 slave，尽管单个连接不会跨多个 slave。

		对于未分片的 TCP 与 UDP 数据包，公式为

		hash = 源端口, 目的端口（如头部所示）
		hash = hash XOR 源 IP XOR 目的 IP
		hash = hash XOR (hash RSHIFT 16)
		hash = hash XOR (hash RSHIFT 8)
		hash = hash RSHIFT 1
		然后 hash 取模 slave 数。

		如果协议为 IPv6，则源地址与目的地址首先使用 ipv6_addr_hash 进行
		哈希。

		对于分片的 TCP 或 UDP 数据包，以及所有其他 IPv4 与 IPv6 协议流量，
		省略源端口与目的端口信息。对于非 IP 流量，公式与 layer2 发送哈希
		策略相同。

		此算法并不完全符合 802.3ad。一个同时包含分片与未分片数据包的
		单个 TCP 或 UDP 会话，会看到数据包被条带化到两个接口上。这可能导致
		乱序交付。大多数流量类型不会满足此条件，因为 TCP 很少对流量分片，
		且大多数 UDP 流量不涉及长时间的会话。其他 802.3ad 实现可能容忍也可能
		不容忍此不合规行为。

	encap2+3

		此策略使用与 layer2+3 相同的公式，但它依赖 skb_flow_dissect 来获取
		头部字段，在使用封装协议时可能会使用内层头部。例如，这将提升隧道
		用户的性能，因为数据包会根据封装后的流进行分发。

	encap3+4

		此策略使用与 layer3+4 相同的公式，但它依赖 skb_flow_dissect 来获取
		头部字段，在使用封装协议时可能会使用内层头部。例如，这将提升隧道
		用户的性能，因为数据包会根据封装后的流进行分发。

	vlan+srcmac

		此策略使用非常基础的 vlan ID 与源 mac 哈希，按 vlan 进行负载均衡，
		并在某条链路失败时提供故障切换。预期的用例是供多个虚拟机共享的
		bond 使用，这些虚拟机都配置为使用自己的 vlan，以在没有 lacp 能力的
		交换硬件的情况下提供类似 lacp 的功能。

	哈希公式很简单：

		hash = (vlan ID) XOR (源 MAC 厂商) XOR (源 MAC 设备)

	默认值为 layer2。此选项添加于 bonding 版本 2.6.3。在更早的 bonding 版本中，
	此参数不存在，layer2 是唯一策略。layer2+3 值添加于 bonding 版本 3.2.2。

resend_igmp

	指定在故障切换事件后发出的 IGMP 成员报告的数量。故障切换后立即发出
	一份成员报告，后续数据包在每个 200ms 间隔发送。

	有效范围是 0 - 255；默认值为 1。值为 0 时阻止因故障切换事件而发出
	IGMP 成员报告。

	此选项对 balance-rr (0)、active-backup (1)、balance-tlb (5) 和
	balance-alb (6) 模式很有用，在这些模式中，故障切换可能将 IGMP 流量从
	一个 slave 切换到另一个。因此需要发出一份新的 IGMP 报告，以促使交换机
	通过新选出的 slave 转发入站 IGMP 流量。

	此选项添加于 bonding 版本 3.7.0。

lp_interval

	指定 bonding 驱动向每个 slave 的对端交换机发送学习数据包的间隔秒数。

	有效范围是 1 - 0x7fffffff；默认值为 1。此选项仅在 balance-tlb 与
	balance-alb 模式下生效。

## 3. 配置 Bonding 设备


你可以使用发行版的网络初始化脚本，或者手动使用 iproute2 或
sysfs 接口来配置 bonding。发行版通常使用三个包之一来提供网络初始化
脚本：initscripts、sysconfig 或 interfaces。这些包的较新版本支持
bonding，而较旧版本不支持。

我们将首先描述针对使用 initscripts、sysconfig 和 interfaces（完全或
部分支持 bonding）的发行版配置 bonding 的选项，然后提供在不依赖网络
初始化脚本（即较旧版本的 initscripts 或 sysconfig）的情况下启用 bonding
的信息。

如果你不确定你的发行版使用的是 sysconfig、initscripts 还是 interfaces，
或者不知道它是否够新，不用担心。判断这一点相当直接。

首先，查找 /etc/network 目录下名为 interfaces 的文件。如果
你的系统中存在此文件，则你的系统使用 interfaces。参见“使用 Interfaces
支持进行配置”。

```

	$ rpm -qf /sbin/ifup

```

它会响应一行以 "initscripts" 或 "sysconfig" 开头、后接一些数字的文本。
这就是提供你的网络初始化脚本的包。

接下来，要判断你的安装是否支持 bonding，

```

    $ grep ifenslave /sbin/ifup

```

如果返回任何匹配项，则你的 initscripts 或 sysconfig 支持 bonding。

### 3.1 使用 Sysconfig 支持进行配置


本节适用于使用带 bonding 支持的 sysconfig 版本的发行版，例如
SuSE Linux Enterprise Server 9。

SuSE SLES 9 的网络配置系统确实支持 bonding，但在撰写本文时，YaST
系统配置前端并未提供任何处理 bonding 设备的方法。不过，bonding 设备
可以手动管理，如下所示。

首先，如果尚未配置，请配置 slave 设备。在 SLES 9 上，最简单的方法是
运行 yast2 sysconfig 配置工具。目标是为每个 slave 设备创建一个
ifcfg-id 文件。完成此操作最简单的方式是将设备配置为 DHCP（这只是为了
创建 ifcfg-id 文件；关于 DHCP 的一些问题见下文）。该

```

    ifcfg-id-xx:xx:xx:xx:xx:xx

```

其中 "xx" 部分将被设备的永久 MAC 地址中的数字替换。

一旦创建了一组 ifcfg-id-xx:xx:xx:xx:xx:xx 文件，就需要编辑这些 slave
设备（其 MAC 地址对应于 slave 设备）的配置文件。在编辑之前，文件将包含
多行，看起来

```

	BOOTPROTO='dhcp'
	STARTMODE='on'
	USERCTL='no'
	UNIQUE='XNzu.WeZGOGF+4wE'
	_nm_name='bus-pci-0001:61:01.0'

```
```
	BOOTPROTO='none'
	STARTMODE='off'

```
不要更改 UNIQUE 或 _nm_name 行。删除其他所有行（USERCTL 等）。

一旦 ifcfg-id-xx:xx:xx:xx:xx:xx 文件被修改完，就到了为 bonding 设备
自身创建配置文件的时候。该文件命名为 ifcfg-bondX，其中 X 是要创建的
bonding 设备的编号，从 0 开始。第一个这样的文件是 ifcfg-bond0，第二个是
ifcfg-bond1，依此类推。sysconfig 网络配置系统能够正确启动 bonding 的多个
实例。

```

	BOOTPROTO="static"
	BROADCAST="10.0.2.255"
	IPADDR="10.0.2.10"
	NETMASK="255.255.0.0"
	NETWORK="10.0.2.0"
	REMOTE_IPADDR=""
	STARTMODE="onboot"
	BONDING_MASTER="yes"
	BONDING_MODULE_OPTS="mode=active-backup miimon=100"
	BONDING_SLAVE0="eth0"
	BONDING_SLAVE1="bus-pci-0000:06:08.1"

```
用适合你网络的相应值替换示例中的 BROADCAST、IPADDR、NETMASK 和 NETWORK
取值。

STARTMODE 指定设备何时上线。可能的值如下：

	======== ======================================================
	onboot	 设备在启动时启动。如果你不确定，这大概就是
		 你想要的。

	manual	 设备仅在手动调用 ifup 时启动。如果你出于某种
		 原因不希望它们在开机时自动启动，bonding 设备
		 可以这样配置。

	hotplug  设备由热插拔事件启动。这对 bonding 设备而言不是
		 一个有效选择。

	off 或  设备的配置被忽略。
	ignore
	======== ======================================================

BONDING_MASTER='yes' 这一行表明该设备是一个 bonding master 设备。唯一
有用的值是 "yes"。

BONDING_MODULE_OPTS 的内容会提供给此设备的 bonding 模块实例。在此指定
bonding 模式、链路监控等选项。不要包含 max_bonds bonding 参数；如果你有
多个 bonding 设备，这会混淆配置系统。

最后，为每个 slave 提供一个 BONDING_SLAVEn="slave device"。其中 "n" 是一个
递增的值，每个 slave 对应一个。"slave device" 可以是一个接口名称，例如
"eth0"，也可以是网络设备的设备描述符。接口名称更容易查找，但 ethN 名称在
启动时可能会变化，例如，序列中靠前的某个设备发生了故障。设备描述符
（上例中的 bus-pci-0000:06:08.1）指定的是物理网络设备，除非设备的总线位置
发生变化（例如它被从一块 PCI 插槽移到另一块），否则不会改变。上面的例子为
演示目的各用了一个类型；大多数配置会为所有 slave 设备统一选择其中一种类型。

当所有配置文件都已修改或创建完成后，必须重启网络才能使配置更改生效

```

	# /etc/init.d/network restart

```

注意，网络控制脚本（/sbin/ifdown）会作为网络关闭处理的一部分移除 bonding
模块，因此，例如当模块参数发生变化时，没有必要手工移除该模块。

此外，在撰写本文时，YaST/YaST2 不会管理 bonding 设备（它们在其网络设备
列表中不显示 bonding 接口）。要更改 bonding 配置，必须手工编辑配置文件。

ifcfg 文件的其它通用选项与细节见

```

	/etc/sysconfig/network/ifcfg.template

```

注意，该模板并未记录上面描述的各种 `BONDING_*` 设置，但确实描述了许多其他
选项。

### 3.1.1 在 Sysconfig 中使用 DHCP

在 sysconfig 下，将设备配置为 BOOTPROTO='dhcp' 会导致它向 DHCP 查询其 IP
地址信息。在撰写本文时，这对 bonding 设备不起作用；脚本会尝试在添加任何
slave 设备之前就从 DHCP 获取设备地址。没有 active slave，DHCP 请求就不会被
发送到网络上。

### 3.1.2 在 Sysconfig 中配置多个 Bond

sysconfig 网络初始化系统能够处理多个 bonding 设备。只需要为每个 bonding
实例准备一个适当配置的 ifcfg-bondX 文件（如上所述）。不要向任何 bonding 实例
指定 "max_bonds" 参数，因为这会混淆 sysconfig。如果你需要多个带有相同参数的
bonding 设备，请创建多个 ifcfg-bondX 文件。

由于 sysconfig 脚本在 ifcfg-bondX 文件中提供 bonding 模块选项，因此没有必要
将它们添加到系统的 `/etc/modules.d/*.conf` 配置文件中。

### 3.2 使用 Initscripts 支持进行配置

本节适用于使用带 bonding 支持的较新版本 initscripts 的发行版，例如 Red Hat
Enterprise Linux 3 或更高版本、Fedora 等。在这些系统上，网络初始化脚本了解
bonding，并且可以被配置为控制 bonding 设备。注意，较旧版本的 initscripts 包
对 bonding 的支持程度较低；在适用处会加以说明。

这些发行版不会自动加载网络适配器驱动，除非 ethX 设备配置了 IP 地址。由于
这一限制，用户必须为所有将成为 bondX 链路成员的物理适配器手工配置一个
network-script 文件。network script 文件位于目录：

/etc/sysconfig/network-scripts

文件名必须以 "ifcfg-eth" 为前缀，并以适配器的物理适配器编号作为后缀。例如，
eth0 的脚本应命名为 /etc/sysconfig/network-scripts/ifcfg-eth0。

```

	DEVICE=eth0
	USERCTL=no
	ONBOOT=yes
	MASTER=bond0
	SLAVE=yes
	BOOTPROTO=none

```
每个 ethX 设备的 DEVICE= 行都不同，并且必须与文件名相对应，即 ifcfg-eth1
必须具有 DEVICE=eth1 的设备行。MASTER= 行的设置也取决于为你的 bond 选择的最终
bonding 接口名称。与其他网络设备一样，它们通常从 0 开始，每个设备递增 1，即
第一个 bonding 实例是 bond0，第二个是 bond1，依此类推。

接下来，创建一个 bond 网络脚本。此脚本的文件名将是
/etc/sysconfig/network-scripts/ifcfg-bondX，其中 X 是 bond 的编号。对于 bond0，
该文件名为 "ifcfg-bond0"，对于 bond1，文件名为 "ifcfg-bond1"，依此类推。在该
文件内，

```

	DEVICE=bond0
	IPADDR=192.168.1.1
	NETMASK=255.255.255.0
	NETWORK=192.168.1.0
	BROADCAST=192.168.1.255
	ONBOOT=yes
	BOOTPROTO=none
	USERCTL=no

```
务必更改特定于网络的行（IPADDR、NETMASK、NETWORK 和 BROADCAST）以匹配你的
网络配置。

对于较新版本的 initscripts，例如 Fedora 7（或更高）和 Red Hat Enterprise Linux
5（或更高），可以在 ifcfg-bond0 中指定 bonding 选项，这不仅是可行的，而且

```

  BONDING_OPTS="mode=active-backup arp_interval=60 arp_ip_target=192.168.1.254"

```
将以指定的选项配置 bond。BONDING_OPTS 中指定的选项与 bonding 模块参数相同，
除了在早于 8.57（Fedora 8）和 8.45.19（Red Hat Enterprise Linux 5.2）的
initscripts 版本中 arp_ip_target 字段的情况。使用较旧版本时，每个目标应作为
单独的选项包含，并以 '+' 作为前缀，表示应将其添加到目标列表

```

    arp_ip_target=+192.168.1.1 arp_ip_target=+192.168.1.2

```
这是指定多个目标的正确语法。当通过 BONDING_OPTS 指定选项时，没有必要编辑
`/etc/modprobe.d/*.conf`。

对于不支持 BONDING_OPTS 的更旧的 initscripts 版本，需要编辑
/etc/modprobe.d/*.conf（取决于你的发行版），以便在 bond0 接口 up 时以你所需的
选项加载 bonding 模块。/etc/modprobe.d/*.conf 中的以下行将加载 bonding 模块，
并选择其选项：

	alias bond0 bonding
	options bond0 mode=balance-alb miimon=100

用适合你配置的一组选项替换示例参数。

最后以 root 身份运行 "/etc/rc.d/init.d/network restart"。这将重启网络子系统，
你的 bond 链路现在应该已经 up 并运行。

### 3.2.1 在 Initscripts 中使用 DHCP

较新版本的 initscripts（据报告，随 Fedora Core 3 和 Red Hat Enterprise Linux 4
或更高版本提供的版本可用）支持通过 DHCP 为 bonding 设备分配 IP 信息。

要为 DHCP 配置 bonding，请按上述方法配置，但将 "BOOTPROTO=none" 行替换为
"BOOTPROTO=dhcp"，并添加一行 "TYPE=Bonding"。注意 TYPE 的值是大小写敏感的。

### 3.2.2 在 Initscripts 中配置多个 Bond

随 Fedora 7 和 Red Hat Enterprise Linux 5 提供的 Initscripts 包支持通过简单地
在 ifcfg-bondX（X 为 bond 的编号）中指定适当的 BONDING_OPTS= 来支持多个 bonding
接口。此支持需要内核中的 sysfs 支持，以及版本 3.0.0 或更高的 bonding 驱动。
其他配置可能不支持这种指定多个 bonding 接口的方法；对于这些情况，请参见下面的
“手动配置多个 Bond”一节。

### 3.3 使用 iproute2 手动配置 Bonding

本节适用于那些网络初始化脚本（sysconfig 或 initscripts 包）不具备 bonding
专门知识的发行版。其中一个这样的发行版是 SuSE Linux Enterprise Server 8。

这些系统的通用方法是将 bonding 模块参数放入 /etc/modprobe.d/ 中的一个配置文件
（适合所安装的发行版），然后将 modprobe 和/或 `ip link` 命令添加到系统的全局
init 脚本中。全局 init 脚本的名称不同；对于 sysconfig，它是 /etc/init.d/boot.local，
对于 initscripts，它是 /etc/rc.d/rc.local。

例如，如果你想创建一个由两个 e100 设备（假定为 eth0 和 eth1）组成的简单 bond，
并且让它在重启后保持存在，请编辑相应的文件（/etc/init.d/boot.local 或

```

	modprobe bonding mode=balance-alb miimon=100
	modprobe e100
	ifconfig bond0 192.168.1.1 netmask 255.255.255.0 up
	ip link set eth0 master bond0
	ip link set eth1 master bond0

```
用适合你配置的值替换示例中的 bonding 模块参数和 bond0 网络配置（IP 地址、netmask
等）。

遗憾的是，此方法不会为 bond 设备提供 ifup 和 ifdown 脚本的支持。要重新加载
bonding

```

	# /etc/init.d/boot.local

```
```

	# /etc/rc.d/rc.local

```
在这种情况下，可能希望创建一个单独的脚本，它只初始化 bonding 配置，然后从
boot.local 中调用该单独脚本。这样就无需重新运行整个全局 init 脚本即可启用
bonding。

要关闭 bonding 设备，必须首先将 bonding 设备本身标记为 down，然后移除相应的
设备驱动模块。对于我们上面的例子，你可以执行

```

	# ifconfig bond0 down
	# rmmod bonding
	# rmmod e100

```
同样，为方便起见，可能希望创建一个包含这些命令的脚本。


### 3.3.1 手动配置多个 Bond

本节包含为那些网络初始化脚本不支持配置多个 bond 的系统配置带有不同选项的
多个 bonding 设备的信息。

如果你需要多个 bonding 设备，但所有选项都相同，你可能希望使用上面文档化的
"max_bonds" 模块参数。

要创建带有不同选项的多个 bonding 设备，最好使用 sysfs 导出的 bonding 参数，
详见下面一节。

对于没有 sysfs 支持的 bonding 版本，提供带有不同选项的多个 bonding 实例的唯一
方法是多次加载 bonding 驱动。注意，当前版本的 sysconfig 网络初始化脚本会自动
处理这一点；如果你的发行版使用这些脚本，则无需特殊操作。如果你不确定你的网络
初始化脚本，请参见上面的“配置 Bonding 设备”一节。

要加载模块的多个实例，必须为每个实例指定不同的名称（模块加载系统要求每个已
加载的模块，即使是同一模块的多个实例，都具有唯一的名称）。这可以通过提供多个

```

	alias bond0 bonding
	options bond0 -o bond0 mode=balance-rr miimon=100

	alias bond1 bonding
	options bond1 -o bond1 mode=balance-alb miimon=50

```
将加载 bonding 模块两次。第一个实例命名为 "bond0"，以 balance-rr 模式、miimon
为 100 创建 bond0 设备。第二个实例命名为 "bond1"，以 balance-alb 模式、miimon
为 50 创建 bond1 设备。

在某些情况下（通常是较旧的发行版），上述方法不起作用，第二个 bonding 实例永远
看不到它的选项。在这种情况下，可以用第二行 options 替换为

```

	install bond1 /sbin/modprobe --ignore-install bonding -o bond1 \
				     mode=balance-alb miimon=50

```
这可以重复任意多次，为后续每个实例指定一个新的唯一名称来替换 bond1。

据观察，某些 Red Hat 提供的内核无法在加载时重命名模块（即 "-o bond1" 部分）。
尝试将该选项传给 modprobe 会产生 "Operation not permitted" 错误。这在某些 Fedora
Core 内核上已有报告，在 RHEL 4 上也见到过。在出现此问题的内核上，将无法配置
带有不同参数的多个 bond（因为它们是较旧的内核，并且也缺少 sysfs 支持）。

### 3.4 通过 Sysfs 手动配置 Bonding

从版本 3.0.0 起，Channel Bonding 可以通过 sysfs 接口配置。该接口允许在不卸载
模块的情况下动态配置系统中的所有 bond。它还允许在运行时添加和移除 bond。Ifenslave
不再需要，尽管仍然受支持。

使用 sysfs 接口可以让你使用具有不同配置的多个 bond，而无需重新加载模块。当你
将 bonding 编译进内核时，它同样允许你使用多个配置不同的 bond。

你必须挂载了 sysfs 文件系统才能以这种方式配置 bonding。本文档中的示例假定你
使用的是 sysfs 的标准挂载点，例如 /sys。如果你的 sysfs 文件系统挂载在其他位置，
你需要相应地调整示例路径。

### 创建与销毁 Bond

```

	# echo +foo > /sys/class/net/bonding_masters

```
```

	# echo -bar > /sys/class/net/bonding_masters

```
```

	# cat /sys/class/net/bonding_masters

```

   由于 sysfs 文件有 4K 大小限制，如果你有数百个以上的 bond，此列表可能会被
   截断。在正常操作条件下这不太可能发生。

### 添加与移除 Slave

可以使用文件 /sys/class/net/<bond>/bonding/slaves 将接口 enslave 到某个 bond。
该文件的语义与 bonding_masters 文件相同。

```

	# ifconfig bond0 up
	# echo +eth0 > /sys/class/net/bond0/bonding/slaves

```
```

	# echo -eth0 > /sys/class/net/bond0/bonding/slaves

```
当某个接口被 enslave 到某个 bond 时，会在 sysfs 文件系统中创建两者之间的符号
链接。在这种情况下，你会得到 /sys/class/net/bond0/slave_eth0 指向
/sys/class/net/eth0，以及 /sys/class/net/eth0/master 指向 /sys/class/net/bond0。

这意味着你可以通过查找 master 符号链接来快速判断某个接口是否被 enslave。因此：
# echo -eth0 > /sys/class/net/eth0/master/bonding/slaves
将把 eth0 从它被 enslave 的任何 bond 中释放，无论 bond 接口的名称是什么。

### 更改 Bond 的配置

可以通过操作位于 /sys/class/net/<bond name>/bonding 中的文件来单独配置每个 bond。

这些文件的名称与本文件中别处描述的命令行参数直接对应，并且除 arp_ip_target 外，
它们接受相同的值。要查看当前设置，只需 cat 相应的文件。

此处给出几个示例；有关每个参数的具体使用指南，请参见本文档中的相应章节。

```

	# ifconfig bond0 down
	# echo 6 > /sys/class/net/bond0/bonding/mode
	- 或 -
	# echo balance-alb > /sys/class/net/bond0/bonding/mode

```

   在更改模式之前，bond 接口必须处于 down 状态。

```

	# echo 1000 > /sys/class/net/bond0/bonding/miimon

```

   如果启用了 ARP 监控，则当启用 MII 监控时它将被禁用，反之亦然。

```

	# echo +192.168.0.100 > /sys/class/net/bond0/bonding/arp_ip_target
	# echo +192.168.0.101 > /sys/class/net/bond0/bonding/arp_ip_target

```

   最多可以指定 16 个目标地址。

```

	# echo -192.168.0.100 > /sys/class/net/bond0/bonding/arp_ip_target

```
```

	# echo 12 > /sys/class/net/bond0/bonding/lp_interval

```

   lp_interval 是 bonding 驱动向每个 slave 的对端交换机发送学习数据包的间隔秒数。
   默认间隔为 1 秒。

### 示例配置

我们从第 3.3 节中展示的同一个例子开始，使用 sysfs 执行，并且不使用 ifenslave。

要创建一个由两个 e100 设备（假定为 eth0 和 eth1）组成的简单 bond，并让它在重启后
保持存在，请编辑相应的文件（/etc/init.d/boot.local 或 /etc/rc.d/rc.local），并
添加

```

	modprobe bonding
	modprobe e100
	echo balance-alb > /sys/class/net/bond0/bonding/mode
	ifconfig bond0 192.168.1.1 netmask 255.255.255.0 up
	echo 100 > /sys/class/net/bond0/bonding/miimon
	echo +eth0 > /sys/class/net/bond0/bonding/slaves
	echo +eth1 > /sys/class/net/bond0/bonding/slaves

```
要添加第二个 bond，带有两个 e1000 接口，使用 active-backup 模式，并启用 ARP 监控，
请向

```

	modprobe e1000
	echo +bond1 > /sys/class/net/bonding_masters
	echo active-backup > /sys/class/net/bond1/bonding/mode
	ifconfig bond1 192.168.2.1 netmask 255.255.255.0 up
	echo +192.168.2.100 /sys/class/net/bond1/bonding/arp_ip_target
	echo 2000 > /sys/class/net/bond1/bonding/arp_interval
	echo +eth2 > /sys/class/net/bond1/bonding/slaves
	echo +eth3 > /sys/class/net/bond1/bonding/slaves

```
### 3.5 使用 Interfaces 支持进行配置

本节适用于那些使用 /etc/network/interfaces 文件来描述网络接口配置的发行版，最
著名的是 Debian 及其派生发行版。

Debian 上的 ifup 和 ifdown 命令默认不支持 bonding。应安装 ifenslave-2.6 包以
提供 bonding 支持。一旦安装，该包将提供 `bond-*` 选项，供在 /etc/network/interfaces
中使用。

注意，ifenslave-2.6 包会加载 bonding 模块，并在适当的时候使用 ifenslave 命令。

### 示例配置


在 /etc/network/interfaces 中，以下节将配置 bond0，使用

```

	auto bond0
	iface bond0 inet dhcp
		bond-slaves eth0 eth1
		bond-mode active-backup
		bond-miimon 100
		bond-primary eth0 eth1

```
如果上述配置不起作用，你可能使用的是 upstart 进行系统启动。最近的一些 Ubuntu 版本
尤其如此。/etc/network/interfaces 中的以下节将

```

	auto bond0
	iface bond0 inet dhcp
		bond-slaves none
		bond-mode active-backup
		bond-miimon 100

	auto eth0
	iface eth0 inet manual
		bond-master bond0
		bond-primary eth0 eth1

	auto eth1
	iface eth1 inet manual
		bond-master bond0
		bond-primary eth0 eth1

```
有关 /etc/network/interfaces 中受支持的 `bond-*` 选项完整列表，以及针对你的特定
发行版定制的一些更高级示例，请参见 /usr/share/doc/ifenslave-2.6 中的文件。

### 3.6 特殊情况下的配置覆盖


使用 bonding 驱动时，发送某个帧的物理端口通常由 bonding 驱动选择，对用户或系统
管理员而言并不重要。输出端口只是使用所选 bonding 模式的策略来选择。不过，有时
将某些类别的流量引导到特定的物理输出接口以实施稍微复杂一些的策略是有帮助的。
例如，要通过一个 bonded 接口访问一台 Web 服务器，其中 eth0 连接到私有网络，而
eth1 通过公共网络连接，可能希望偏置该 bond，优先通过 eth0 发送此类流量，仅在回退
时才使用 eth1，而所有其他流量可以安全地通过任一接口发送。此类配置可以使用 linux
固有的流量控制工具来实现。

默认情况下，bonding 驱动是多队列感知的，并在驱动初始化时创建 16 个队列（详见
Documentation/networking/multiqueue.rst）。如果需要更多或更少的队列，可以使用模块
参数 tx_queues 来更改此值。由于没有 sysfs 参数可用（因为分配是在模块初始化时
完成的）。

文件 /proc/net/bonding/bondX 的输出已经改变，因此输出队列

```

	Bonding Mode: fault-tolerance (active-backup)
	Primary Slave: None
	Currently Active Slave: eth0
	MII Status: up
	MII Polling Interval (ms): 0
	Up Delay (ms): 0
	Down Delay (ms): 0

	Slave Interface: eth0
	MII Status: up
	Link Failure Count: 0
	Permanent HW addr: 00:1a:a0:12:8f:cb
	Slave queue ID: 0

	Slave Interface: eth1
	MII Status: up
	Link Failure Count: 0
	Permanent HW addr: 00:1a:a0:12:8f:cc
	Slave queue ID: 2

```
```

	# echo "eth1:2" > /sys/class/net/bond0/bonding/queue_id

```
任何需要设置 queue_id 的接口都应通过类似上面那样的多次调用来设置，直到为所有
接口设置了适当的优先级。在允许通过 initscripts 配置的发行版上，可以向 BONDING_OPTS
添加多个 'queue_id' 参数来设置所有需要的 slave 队列。

这些 queue id 可以与 tc 工具配合使用，配置多队列 qdisc 和过滤器，以将某些流量偏置
到特定的 slave 设备上发送。例如，假设我们想在上述配置中，强制所有发往 192.168.1.100
的流量使用 bond 中的 eth1 作为其输出

```

	# tc qdisc add dev bond0 handle 1 root multiq

	# tc filter add dev bond0 protocol ip parent 1: prio 1 u32 match ip \
		dst 192.168.1.100 action skbedit queue_mapping 2

```
这些命令告诉内核在 bond0 接口上附加一个多队列队列规则，并过滤入队的流量，使得
dst ip 为 192.168.1.100 的数据包其输出队列映射值被覆盖为 2。该值随后被传入驱动，
导致正常的输出路径选择策略被覆盖，转而选择 qid 2，即映射到 eth1。

注意，qid 值从 1 开始。Qid 0 保留用于向驱动表明应进行正常的输出策略选择。将 slave
的 qid 简单地保留为 0 的一个好处是，现在 bonding 驱动中存在的多队列感知能力。这种
感知允许将 tc 过滤器放在 slave 设备以及 bond 设备上，并且 bonding 驱动将简单地充当
透传，用于在 slave 设备上选择输出队列，而不是选择输出端口。

此特性首次出现在 bonding 驱动版本 3.7.0 中，并且对输出 slave 选择的支持仅限于
round-robin 和 active-backup 模式。

### 3.7 以更安全的方式为 802.3ad 模式配置 LACP

当使用 802.3ad bonding 模式时，Actor（主机）与 Partner（交换机）会交换 LACPDU。
这些 LACPDU 无法被嗅探，因为它们发往链路本地 mac 地址（交换机/网桥不应转发这些
地址）。然而，大多数值很容易预测，或者干脆就是机器的 MAC 地址（同一 L2 中的其他
所有主机都轻易知道）。这意味着 L2 域中的其他机器可以从其他主机向交换机 spoof LACPDU
数据包，并可能通过加入（从交换机的角度看）另一台机器的聚合而造成混乱，从而接收到
该主机入站流量的一部分，和/或自己 spoof 来自该主机的流量（甚至可能成功终止该主机的
部分数据流）。虽然这不太可能发生，但可以通过简单地配置几个 bonding 参数来避免这种
可能性：

   (a) ad_actor_system：你可以设置一个随机的 mac 地址，用于这些 LACPDU 交换。该
       值不能是 NULL 或组播地址。此外，最好设置 local-admin 位。以下 shell 代码

```

	      # sys_mac_addr=$(printf '%02x:%02x:%02x:%02x:%02x:%02x' \
				       $(( (RANDOM & 0xFE) | 0x02 )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )) \
				       $(( RANDOM & 0xFF )))
	      # echo $sys_mac_addr > /sys/class/net/bond0/bonding/ad_actor_system

   (b) ad_actor_sys_prio：随机化系统优先级。默认值为 65535，但系统可以取 1 - 65535
       之间的值。以下 shell 代码生成随机优先级并设置它：：

	    # sys_prio=$(( 1 + RANDOM + RANDOM ))
	    # echo $sys_prio > /sys/class/net/bond0/bonding/ad_actor_sys_prio

   (c) ad_user_port_key：使用 port-key 的用户部分。默认保留为空。这些是 port-key
       的高 10 位，取值范围为 0 - 1023。以下 shell 代码生成这 10 位并设置它：：

	    # usr_port_key=$(( RANDOM & 0x3FF ))
	    # echo $usr_port_key > /sys/class/net/bond0/bonding/ad_user_port_key


```
## 4 查询 Bonding 配置


### 4.1 Bonding 配置


每个 bonding 设备都有一个只读文件，位于 /proc/net/bonding 目录中。文件内容包含
关于 bonding 配置、选项以及每个 slave 状态的信息。

例如，在驱动以 mode=0 和 miimon=1000 的参数加载后，/proc/net/bonding/bond0 的内容
为

```

	Ethernet Channel Bonding Driver: 2.6.1 (October 29, 2004)
	Bonding Mode: load balancing (round-robin)
	Currently Active Slave: eth0
	MII Status: up
	MII Polling Interval (ms): 1000
	Up Delay (ms): 0
	Down Delay (ms): 0

	Slave Interface: eth1
	MII Status: up
	Link Failure Count: 1

	Slave Interface: eth0
	MII Status: up
	Link Failure Count: 1

```
确切的格式与内容会根据 bonding 配置、状态以及 bonding 驱动的版本而变化。

### 4.2 网络配置


可以使用 ifconfig 命令检查网络配置。Bonding 设备会设置 MASTER 标志；Bonding slave
设备会设置 SLAVE 标志。ifconfig 输出不包含哪些 slave 与哪些 master 相关联的信息。

在下面的例子中，bond0 接口是 master（MASTER），而 eth0 和 eth1 是 slave（SLAVE）。
注意，对于所有模式，bond0 的所有 slave 都具有与 bond0 相同的 MAC 地址（HWaddr），
但

```

  # /sbin/ifconfig
  bond0     Link encap:Ethernet  HWaddr 00:C0:F0:1F:37:B4
	    inet addr:XXX.XXX.XXX.YYY  Bcast:XXX.XXX.XXX.255  Mask:255.255.252.0
	    UP BROADCAST RUNNING MASTER MULTICAST  MTU:1500  Metric:1
	    RX packets:7224794 errors:0 dropped:0 overruns:0 frame:0
	    TX packets:3286647 errors:1 dropped:0 overruns:1 carrier:0
	    collisions:0 txqueuelen:0

  eth0      Link encap:Ethernet  HWaddr 00:C0:F0:1F:37:B4
	    UP BROADCAST RUNNING SLAVE MULTICAST  MTU:1500  Metric:1
	    RX packets:3573025 errors:0 dropped:0 overruns:0 frame:0
	    TX packets:1643167 errors:1 dropped:0 overruns:1 carrier:0
	    collisions:0 txqueuelen:100
	    Interrupt:10 Base address:0x1080

  eth1      Link encap:Ethernet  HWaddr 00:C0:F0:1F:37:B4
	    UP BROADCAST RUNNING SLAVE MULTICAST  MTU:1500  Metric:1
	    RX packets:3651769 errors:0 dropped:0 overruns:0 frame:0
	    TX packets:1643480 errors:0 dropped:0 overruns:0 carrier:0
	    collisions:0 txqueuelen:100
	    Interrupt:9 Base address:0x1400

```
## 5. 交换机配置


在本节中，“交换机”指的是 bonded 设备直接连接到的任何系统（即网线的另一端插到的
地方）。它可能是一个真正的专用交换机设备，也可能是另一个普通系统（例如，另一台
运行 Linux 的计算机），

active-backup、balance-tlb 和 balance-alb 模式不需要交换机的任何特定配置。

802.3ad 模式要求交换机将适当的端口配置为 802.3ad 聚合。用于配置此功能的具体方法
因交换机而异，但例如，Cisco 3550 系列交换机要求首先将适当的端口组合在单个
etherchannel 实例中，然后将该 etherchannel 设置为 "lacp" 模式以启用 802.3ad（而不是
标准的 EtherChannel）。

balance-rr、balance-xor 和 broadcast 模式通常要求交换机将适当的端口组合在一起。
这种组的命名因交换机而异，它可称为 "etherchannel"（如上面的 Cisco 示例）、"trunk
group" 或其它类似的叫法。对于这些模式，每个交换机还会有自己针对交换机到 bond 的
发送策略的配置选项。典型的选择包括 MAC 或 IP 地址的 XOR。两个对端的发送策略不需要
匹配。对于这三种模式，bonding 模式实际上是为一个 EtherChannel 组选择了一个发送策略；
这三种模式都将与另一个 EtherChannel 组互操作。


## 6. 802.1q VLAN 支持


可以使用 8021q 驱动在 bond 接口之上配置 VLAN 设备。然而，默认情况下，只有来自
8021q 驱动并经过 bonding 的数据包才会被打上标签。自身生成的数据包，例如 bonding 的
学习数据包，或者由 ALB 模式或 ARP 监控机制生成的 ARP 数据包，由 bonding 自身在内部
打标签。因此，bonding 必须“学习”配置在它之上的 VLAN ID，并使用这些 ID 来为其自身
生成的数据包打标签。

出于简化以及支持能够进行 VLAN 硬件加速卸载的适配器的原因，bonding 接口声明自己
完全具备硬件卸载能力，它获取 add_vid/kill_vid 通知以收集必要的信息，并将这些动作
传播到 slave。在混合适配器类型的情况下，本应经过不具备卸载能力的适配器的硬件加速
带标签数据包，会被 bonding 驱动“去加速”，使 VLAN 标签位于常规位置。

VLAN 接口**必须**在至少 enslave 一个 slave 之后，才能添加到 bonding 接口之上。在
添加第一个 slave 之前，bonding 接口的硬件地址为 00:00:00:00:00:00。如果在第一次
enslavement 之前创建了 VLAN 接口，它将取得全零硬件地址。一旦第一个 slave 被附加到
bond，bond 设备自身将取得该 slave 的硬件地址，该地址随后可用于 VLAN 设备。

另外，请注意，如果从仍然在其上有一个或多个 VLAN 接口的 bond 中释放所有 slave，也会
发生类似的问题。当添加一个新的 slave 时，bonding 接口将从第一个 slave 获取其硬件
地址，这可能不匹配 VLAN 接口的硬件地址（后者最终是从更早的 slave 复制而来）。

如果在所有 slave 都从 bond 接口移除的情况下，要确保 VLAN 设备以正确的硬件地址运行，
有两种方法：

1. 移除所有 VLAN 接口，然后重新创建它们

2. 设置 bonding 接口的硬件地址，使其与 VLAN 接口的硬件地址匹配

注意，更改 VLAN 接口的 HW 地址会将底层设备——即 bonding 接口——设置为混杂模式，这
可能并不是你想要的。


## 7. 链路监控


bonding 驱动目前支持两种监控 slave 设备链路状态的方案：ARP 监控与 MII 监控。

目前，由于 bonding 驱动自身的实现限制，无法同时启用 ARP 与 MII 监控。

### 7.1 ARP 监控工作机制


ARP 监控如其名称所示运作：它向网络上一个或多个指定的对端系统发送 ARP 查询，并
使用响应作为链路正在运行的指示。这提供了一定的保证，即流量确实在与本地网络上的
一个或多个对端之间流动。

### 7.2 配置多个 ARP 目标

虽然 ARP 监控可以仅使用一个目标完成，但在高可用设置中，拥有多个目标进行监控会很
有用。在只有一个目标的情况下，目标本身可能宕机或出现问题，从而无法响应 ARP 请求。
拥有额外的目标（或几个）可提高 ARP 监控的可靠性。

```

 # example options for ARP monitoring with three targets
 alias bond0 bonding
 options bond0 arp_interval=60 arp_ip_target=192.168.0.1,192.168.0.3,192.168.0.9

```
```

    # example options for ARP monitoring with one target
    alias bond0 bonding
    options bond0 arp_interval=60 arp_ip_target=192.168.0.100


```
### 7.3 MII 监控工作机制


MII 监控只监控本地网络接口的 carrier 状态。它通过以下三种方式之一完成：依赖设备
驱动维护其 carrier 状态、查询设备的 MII 寄存器，或对设备发起 ethtool 查询。

MII 监控依赖驱动获取 carrier 状态信息（通过 netif_carrier 子系统）。

## 8. 潜在的故障来源


### 8.1 路由方面的坑


配置 bonding 时，重要的是 slave 设备不要拥有凌驾于 master 路由之上的路由（或者一般
来说，根本不要有路由）。例如，假设 bonding 设备 bond0 有两个 slave，eth0 和 eth1，
并且路由表是

```

  Kernel IP routing table
  Destination     Gateway         Genmask         Flags   MSS Window  irtt Iface
  10.0.0.0        0.0.0.0         255.255.0.0     U        40 0          0 eth0
  10.0.0.0        0.0.0.0         255.255.0.0     U        40 0          0 eth1
  10.0.0.0        0.0.0.0         255.255.0.0     U        40 0          0 bond0
  127.0.0.0       0.0.0.0         255.0.0.0       U        40 0          0 lo

```
这种路由配置可能仍会更新驱动中的接收/发送时间（ARP 监控所需），但可能会绕过 bonding
驱动（因为在本例中，发往网络 10 上另一台主机的出站流量会在 bond0 之前使用 eth0 或
eth1）。

ARP 监控（以及 ARP 本身）可能会被这种配置搞糊涂，因为 ARP 请求（由 ARP 监控生成）
将在一个接口（bond0）上发送，但相应的应答会到达另一个接口（eth0）。对于 ARP 而言，
此应答看起来像是未经请求的 ARP 应答（因为 ARP 是基于接口来匹配应答的），因而被丢弃。
MII 监控不受路由表状态的影响。

这里的解决办法很简单：确保 slave 没有自己的路由，如果由于某种原因必须有，那些路由
也不要凌驾于其 master 的路由之上。通常情况应如此，但不寻常的配置或错误的人工或自动
静态路由添加可能会引发问题。

### 8.2 以太网设备重命名


在那些网络配置脚本不会将物理设备直接与网络接口名称关联（即同一个物理设备始终具有
相同的 "ethX" 名称）的系统上，可能有必要向 /etc/modprobe.d/ 中的配置文件添加一些
特殊逻辑。

```

	alias bond0 bonding
	options bond0 mode=some-mode miimon=50
	alias eth0 tg3
	alias eth1 tg3
	alias eth2 e1000
	alias eth3 e1000

```
如果 eth0 和 eth1 都不是 bond0 的 slave，那么当 bond0 接口 up 时，设备最终可能会
重新排序。发生这种情况是因为先加载 bonding，然后才加载其 slave 设备的驱动。由于尚未
加载其他驱动，当 e1000 驱动加载时，它将为其设备取得 eth0 和 eth1，但 bonding 配置
试图 enslave eth2 和 eth3（这之后可能会被分配给 tg3 设备）。

```

	add above bonding e1000 tg3

```
会导致 modprobe 在加载 bonding 时按此顺序先加载 e1000 再加载 tg3。此命令在
modules.conf 手册页中有完整文档。

在使用 modprobe 的系统上也可能出现类似问题。在这种情况下，可以向配置文件添加以下内容

```

	softdep bonding pre: tg3 e1000

```
这将在加载 bonding 之前先加载 tg3 和 e1000 模块。有关此内容的完整文档可在 modprobe.d
与 modprobe 手册页中找到。

## 9. SNMP 代理

如果运行 SNMP 代理，bonding 驱动应在任何参与 bond 的网络驱动之前加载。此要求是因为
接口索引（ipAdEntIfIndex）与找到的具有给定 IP 地址的第一个接口相关联。也就是说，每个
IP 地址只有一个 ipAdEntIfIndex。例如，如果 eth0 和 eth1 是 bond0 的 slave，并且 eth0
的驱动在 bonding 驱动之前加载，则该 IP 地址的接口将关联到 eth0 接口。此配置如下所示，
IP 地址 192.168.1.1 的接口索引为 2，它在 ifDescr 表中索引到 eth0（ifDescr.2）。

```

     interfaces.ifTable.ifEntry.ifDescr.1 = lo
     interfaces.ifTable.ifEntry.ifDescr.2 = eth0
     interfaces.ifTable.ifEntry.ifDescr.3 = eth1
     interfaces.ifTable.ifEntry.ifDescr.4 = eth2
     interfaces.ifTable.ifEntry.ifDescr.5 = eth3
     interfaces.ifTable.ifEntry.ifDescr.6 = bond0
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.10.10.10 = 5
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.192.168.1.1 = 2
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.74.20.94 = 4
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.127.0.0.1 = 1

```
通过在任何参与 bond 的网络驱动之前加载 bonding 驱动，可以避免此问题。下面是先加载
bonding 驱动的例子，IP 地址 192.168.1.1 正确地关联到 ifDescr.2。

     interfaces.ifTable.ifEntry.ifDescr.1 = lo
     interfaces.ifTable.ifEntry.ifDescr.2 = bond0
     interfaces.ifTable.ifEntry.ifDescr.3 = eth0
     interfaces.ifTable.ifEntry.ifDescr.4 = eth1
     interfaces.ifTable.ifEntry.ifDescr.5 = eth2
     interfaces.ifTable.ifEntry.ifDescr.6 = eth3
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.10.10.10 = 6
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.192.168.1.1 = 2
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.10.74.20.94 = 5
     ip.ipAddrTable.ipAddrEntry.ipAdEntIfIndex.127.0.0.1 = 1

虽然某些发行版可能在 ifDescr 中不报告接口名称，但 IP 地址与 IfIndex 之间的关联仍然
存在，并且 SNMP 功能（如 Interface_Scan_Next）会报告该关联。

## 10. 混杂模式


运行网络监控工具（例如 tcpdump）时，通常会启用设备上的混杂模式，以便看到所有流量
（而不是只看到发往本地主机的流量）。bonding 驱动处理对 bonding master 设备（例如
bond0）的混杂模式更改，并将该设置传播到 slave
设备。

对于 balance-rr、balance-xor、broadcast 和 802.3ad 模式，混杂模式设置会传播到
所有 slave。

对于 active-backup、balance-tlb 和 balance-alb 模式，混杂模式设置仅传播到 active
slave。

对于 balance-tlb 模式，active slave 是当前正在接收入站流量的 slave。

对于 balance-alb 模式，active slave 是作为“primary”使用的 slave。该 slave 用于
模式特定的控制流量，用于向未分配的对端发送流量，或在负载不均衡时发送。

对于 active-backup、balance-tlb 和 balance-alb 模式，当 active slave 发生变化时
（例如由于链路故障），混杂设置将被传播到新的 active slave。

## 11. 为 High Availability 配置 Bonding


高可用指的是通过拥有冗余或备份设备、链路或交换机（位于主机与外界之间）来提供
最大网络可用性的配置。目标是提供最大的网络连接可用性（即网络始终可用），即使
其他配置可能提供更高的吞吐量。

### 11.1 单交换机拓扑中的高可用


如果两个主机（或一台主机与单个交换机）通过多条物理链路直接相连，那么优化为最大
带宽不会带来可用性损失。在这种情况下，只有一个交换机（或对端），因此如果它失败，
就没有可故障切换到的备用接入。此外，bonding 负载均衡模式支持对其成员的链路监控，
因此如果个别链路失败，负载将在剩余的设备上重新均衡。

有关使用单个对端设备配置 bonding 的信息，请参见第 12 节“为最大吞吐量配置 Bonding”。

### 11.2 多交换机拓扑中的高可用


有了多个交换机，bonding 与网络的配置会发生巨大变化。在多交换机拓扑中，网络可用性
与可用带宽之间存在权衡。

下面是一个配置为最大化

```

		|                                     |
		|port3                           port3|
	  +-----+----+                          +-----+----+
	  |          |port2       ISL      port2|          |
	  | switch A +--------------------------+ switch B |
	  |          |                          |          |
	  +-----+----+                          +-----++---+
		|port1                           port1|
		|             +-------+               |
		+-------------+ host1 +---------------+
			 eth0 +-------+ eth1

```
在此配置中，两个交换机之间有一条链路（ISL，即交换机间链路），并有多个端口连接到
外部世界（每个交换机上的 “port3”）。从技术上讲，没有理由不能将其扩展到第三个交换机。

### 11.2.1 多交换机拓扑的 HA Bonding 模式选择


在像上面这样的拓扑中，当优化可用性时，active-backup 和 broadcast 模式是唯一有用的
bonding 模式；其他模式要求所有链路都终止于同一个对端，才能合理地工作。

active-backup:
	这通常是首选模式，特别是如果交换机具有 ISL 并且配合良好。如果
	网络配置使得某个交换机被明确指定为备份交换机（例如，容量较低、
	成本较高等），则可以使用 primary 选项来确保首选链路在可用时总是
	被使用。

broadcast:
	此模式确实是一个特殊用途模式，仅适用于非常特定的需求。例如，
	如果两个交换机未连接（无 ISL），并且它们之外的网络完全独立。在
	这种情况下，如果某些特定的单向流量有必要到达两个独立的网络，那么
	broadcast 模式可能适用。

### 11.2.2 多交换机拓扑的 HA 链路监控选择


链路监控的选择最终取决于你的交换机。如果交换机能够可靠地在响应其他故障时将端口
置为失败，那么 MII 或 ARP 监控都应该可用。例如，在上面的例子中，如果 "port3" 链路
在远端失败，MII 监控没有直接手段检测这一点。ARP 监控可以配置 port3 远端的一个目标，
从而无需交换机支持即可检测该故障。

然而，一般来说，在多交换机拓扑中，ARP 监控在检测端到端连通性故障（可能由任何单个
组件因任何原因无法传递流量而引起）方面可以提供更高等级的可靠性。此外，ARP 监控应
配置多个目标（网络中每个交换机至少一个）。这将确保，无论哪个交换机处于活动状态，
ARP 监控都有一个合适的目标可供查询。

另外还要注意，近来许多交换机支持一种通常称为 “trunk failover” 的功能。这是交换机的
一个特性，当另一个交换机端口的状态变为 down（或 up）时，会使某个特定交换机端口的
链路状态被置为 down（或 up）。其目的是将链路故障从逻辑上“外部”的端口传播到 bonding
能够通过 miimon 监控的逻辑上“内部”的端口。trunk failover 的可用性与配置因交换机而异，
但在使用合适的交换机时，它可以作为 ARP 监控的一个可行替代方案。

## 12. 为最大吞吐量配置 Bonding


### 12.1 单交换机拓扑中的最大吞吐量


在单交换机配置中，最大化吞吐量的最佳方法取决于应用与网络环境。各种负载均衡模式在
不同环境下各有优缺点，详述如下。

在本文讨论中，我们将拓扑分为两类。根据大多数流量的目的地，我们将它们归类为
“gatewayed（经网关）”或 “local（本地）”配置。

在 gatewayed 配置中，“交换机”主要充当路由器，大多数流量经过此路由器到达

```

     +----------+                     +----------+
     |          |eth0            port1|          | to other networks
     | Host A   +---------------------+ router   +------------------->
     |          +---------------------+          | Hosts B and C are out
     |          |eth1            port2|          | here somewhere
     +----------+                     +----------+

```
路由器可以是一个专用的路由器设备，或是充当网关的另一台主机。在本文讨论中，重点是
Host A 的大多数流量在到达其最终目的地之前，都会经过路由器到达某个其他网络。

在 gatewayed 网络配置中，虽然 Host A 可能与许多其他系统通信，但其所有流量都将通过
本地网络上的另一个对端——路由器——发送和接收。

注意，两台系统通过多条物理链路直接相连的情况，就配置 bonding 而言，与 gatewayed
配置相同。在这种情况下，碰巧所有流量都发往“网关”本身，而不是网关之外的某个其他网络。

在 local 配置中，“交换机”主要充当交换机，大多数流量经过此交换机到达同一网络上的
其他站点。例如

```

    +----------+            +----------+       +--------+
    |          |eth0   port1|          +-------+ Host B |
    |  Host A  +------------+  switch  |port3  +--------+
    |          +------------+          |                  +--------+
    |          |eth1   port2|          +------------------+ Host C |
    +----------+            +----------+port4             +--------+


```
同样，交换机可以是一个专用的交换机设备，或是充当网关的另一台主机。在本文讨论中，
重点是 Host A 的大多数流量都发往同一本地网络上的其他主机（上例中的 Hosts B 和 C）。

总之，在 gatewayed 配置中，往返 bonded 设备的流量都将发往网络上同一个 MAC 层级的对端
（网关本身，即路由器），无论其最终目的地如何。在 local 配置中，流量直接在最终目的地
之间流动，因此每个目的地（Host B、Host C）都将由其各自的 MAC 地址直接寻址。

gatewayed 与 local 网络配置之间的这种区别很重要，因为许多可用的负载均衡模式都使用
本地网络源和目的的 MAC 地址来做出负载均衡决策。每种模式的行为如下所述。


### 12.1.1 单交换机拓扑的 MT Bonding 模式选择


此配置最容易搭建和理解，尽管你将不得不决定哪种 bonding 模式最适合你的需求。每种模式
的权衡详述如下：

balance-rr:
	此模式是唯一允许单个 TCP/IP 连接将流量条带化到多个接口的模式。因此，
	它也是唯一允许单个 TCP/IP 流利用超过一个接口的吞吐量的模式。但这
	是有代价的：条带化通常会导致对端系统收到乱序的数据包，从而引发
	TCP/IP 的拥塞控制系统介入，通常表现为重传段。

	可以通过修改 net.ipv4.tcp_reordering sysctl 参数来调整 TCP/IP 的拥塞
	限制。通常的默认值为 3。但请记住，TCP 栈在检测到重排序时能够自动
	增大此值。

	注意，将被乱序交付的数据包比例高度可变，不太可能为零。重排序的程度
	取决于多种因素，包括网络接口、交换机以及配置的拓扑。一般来说，速率
	更高的网卡会产生更多重排序（由于数据包合并等因素），并且 “多对多”
	拓扑比较 “多慢对一快” 的配置会以更高比率重排序。

	许多交换机不支持任何条带化流量的模式（而是基于 IP 或 MAC 层级地址
	选择端口）；对于这些设备，流经交换机到达 balance-rr bond 的特定
	连接的流量将无法利用超过一个接口的带宽。

	如果你使用的是 TCP/IP 之外的协议（例如 UDP），并且你的应用能够容忍
	乱序交付，那么此模式可以实现接近线性的单流数据报性能扩展，随着
	接口被加入到 bond 中。

	此模式要求交换机将适当的端口配置为 “etherchannel” 或 “trunking”。

active-backup:
	在这种网络拓扑中，active-backup 模式没有太大优势，因为不活动的备份
	设备都与 primary 连接到同一个对端。在这种情况下，负载均衡模式（带
	链路监控）将提供相同等级的网络可用性，但具有更高的可用带宽。好的一面是，
	active-backup 模式不需要对交换机进行任何配置，因此如果可用的硬件不支持
	任何负载均衡模式，它可能仍有价值。

balance-xor:
	此模式会限制流量，使发往特定对端的数据包总是通过同一个接口发送。由于
	目的地由所涉及的 MAC 地址决定，此模式在 “local” 网络配置（如上所述）
	中效果最佳，且所有目的地都在同一本地网络上。如果你的所有流量都经过单个
	路由器（即如上所述的 “gatewayed” 网络配置），此模式可能次优。

	与 balance-rr 一样，交换机端口需要配置为 “etherchannel” 或 “trunking”。

broadcast:
	与 active-backup 类似，在这种类型的网络拓扑中，此模式没有太大优势。

802.3ad:
	此模式对于这种类型的网络拓扑是一个不错的选择。802.3ad 模式是一个 IEEE
	标准，因此所有实现 802.3ad 的对端都应该能良好互操作。802.3ad 协议包含
	聚合的自动配置，因此只需要对交换机进行最少的手动配置（通常只是指定某组
	设备可用于 802.3ad）。802.3ad 标准还要求帧按顺序交付（在一定限度内），因此
	一般来说单连接不会看到数据包乱序。802.3ad 模式确实有一些缺点：标准要求在
	聚合中的所有设备以相同的速率和双工运行。此外，与除 balance-rr 之外的所有
	bonding 负载均衡模式一样，没有任何单个连接能够利用超过一个接口的带宽。

	此外，linux bonding 的 802.3ad 实现按对端分发流量（使用 MAC 地址与数据包
	类型 ID 的 XOR），因此在 “gatewayed” 配置中，所有出站流量通常都会使用同一个
	设备。入站流量也可能最终落在单个设备上，但这取决于对端 802.3ad 实现的均衡
	策略。在 “local” 配置中，流量将分布在 bond 中的各个设备上。

	最后，802.3ad 模式强制使用 MII 监控，因此在此模式下 ARP 监控不可用。

balance-tlb:
	balance-tlb 模式按对端均衡出站流量。由于均衡是根据 MAC 地址进行的，在
	“gatewayed” 配置（如上所述）中，此模式会通过单个设备发送所有流量。然而，在
	“local” 网络配置中，此模式以一种模糊智能的方式（不是像 balance-xor 或
	802.3ad 模式中那样简单的 XOR）在设备间均衡多个本地网络对端，使得数学上不走运
	的 MAC 地址（即 XOR 到相同值的那些）不会全部“聚集”在单个接口上。

	与 802.3ad 不同，接口可以具有不同的速率，并且不需要特殊的交换机配置。不利的一面是，
	在此模式下所有入站流量都通过单个接口到达，此模式要求 slave 接口的网络设备驱动
	具备特定的 ethtool 支持，并且 ARP 监控不可用。

balance-alb:
	此模式就是 balance-tlb 的一切，甚至更多。它具备 balance-tlb 的所有特性（和
	限制），并且还会均衡来自本地网络对端的入站流量（如上面的 Bonding 模块选项一节
	所述）。

	此模式唯一额外的不利之处是，网络设备驱动必须支持在设备处于打开状态时更改硬件
	地址。

### 12.1.2 单交换机拓扑的 MT 链路监控


链路监控的选择可能在很大程度上取决于你选择使用的模式。更高级的负载均衡模式不支持
使用 ARP 监控，因此仅限于使用 MII 监控（它提供的端到端保证不如 ARP 监控高）。

### 12.2 多交换机拓扑中的最大吞吐量


当多个交换机作为隔离网络的一部分并行配置时，可以利用它们来优化吞吐量

```

		       +-----------+
		       |  Host A   |
		       +-+---+---+-+
			 |   |   |
		+--------+   |   +---------+
		|            |             |
	 +------+---+  +-----+----+  +-----+----+
	 | Switch A |  | Switch B |  | Switch C |
	 +------+---+  +-----+----+  +-----+----+
		|            |             |
		+--------+   |   +---------+
			 |   |   |
		       +-+---+---+-+
		       |  Host B   |
		       +-----------+

```
在此配置中，交换机彼此隔离。采用这种拓扑的一个原因是，对于一个拥有许多主机的隔离网络
（例如，为高性能配置的集群），使用多个较小的交换机可能比单个较大的交换机更具成本效益，
例如，在一个有 24 台主机的网络上，三台 24 端口交换机可能比单台 72 端口交换机便宜得多。

如果需要访问网络之外的资源，可以为单个主机配备一个连接到外部网络的额外网络设备；
该主机随后也充当网关。

### 12.2.1 多交换机拓扑的 MT Bonding 模式选择


在实际中，此类配置通常采用的 bonding 模式是 balance-rr。从历史上看，在这种网络配置中，
关于数据包乱序交付的通常告诫会通过使用不进行任何数据包合并的网络适配器（通过使用
NAPI，或因为设备本身在若干数据包到达之前不产生中断）来缓解。以这种方式使用时，
balance-rr 模式允许两台主机之间的单个连接有效地利用超过一个接口的带宽。

### 12.2.2 多交换机拓扑的 MT 链路监控


同样，在实际中，此配置中最常使用的是 MII 监控，因为性能优先于可用性。ARP 监控在此
拓扑中能够工作，但随着涉及系统数量的增长，所需探测的数量会削弱其相对于 MII 监控的优势
（请记住，网络中的每台主机都配置了 bonding）。

## 13. 交换机行为问题


### 13.1 链路建立与故障切换延迟


某些交换机在链路 up 与 down 上报的时机方面表现出不良行为。

首先，当链路 up 时，某些交换机可能指示链路已 up（carrier 可用），但在一段时间内不通过
接口传递流量。这种延迟通常是由于某种类型的自动协商或路由协议，但也可能发生在交换机
初始化期间（例如，在交换机故障恢复期间）。如果你发现这是个问题，请为 updelay bonding
模块选项指定一个适当的值，以延迟使用相关接口。

其次，某些交换机可能在链路状态改变时将其“抖动”一次或多次。这最常见于交换机初始化期间。
同样，一个适当的 updelay 值可能会有所帮助。

注意，当 bonding 接口没有活动链路时，驱动将立即重用第一个 up 的链路，即使指定了 updelay
参数（在这种情况下 updelay 被忽略）。如果有 slave 接口正在等待 updelay 超时到期，则
最先进入该状态的接口将被立即重用。如果 updelay 的值被高估，这会减少网络的停机时间，并且
由于这种情况只发生在没有连通性时，忽略 updelay 不会带来额外的惩罚。

除了对交换机时序的担忧之外，如果你的交换机需要很长时间才能进入备份模式，可能希望在链路
down 后不要立即激活备份接口。可以通过 downdelay bonding 模块选项延迟故障切换。

### 13.2 重复的入站数据包


注意：从版本 3.0.2 起，bonding 驱动具有抑制重复数据包的逻辑，这应该能基本消除此问题。
以下描述保留以供参考。

在首次使用 bonding 设备后，或在它闲置一段时间后，观察到短暂的重复流量突发并不罕见。这
最容易通过观察向网络上另一台主机发出 “ping” 并注意 ping 的输出标记了重复项（通常每个
slave 一个）来观察。

例如，在一个有五个 slave 的 active-backup 模式 bond 上

```

	# ping -n 10.0.4.2
	PING 10.0.4.2 (10.0.4.2) from 10.0.3.10 : 56(84) bytes of data.
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.7 ms
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=1 ttl=64 time=13.8 ms (DUP!)
	64 bytes from 10.0.4.2: icmp_seq=2 ttl=64 time=0.216 ms
	64 bytes from 10.0.4.2: icmp_seq=3 ttl=64 time=0.267 ms
	64 bytes from 10.0.4.2: icmp_seq=4 ttl=64 time=0.222 ms

```
这并不是 bonding 驱动的错误，而是许多交换机更新其 MAC 转发表方式的一个副作用。最初，
交换机不会将数据包中的 MAC 地址与特定交换机端口关联，因此可能会将流量发送到所有端口，
直到其 MAC 转发表更新。由于附加到 bond 的接口可能占据单个交换机上的多个端口，当交换机
（临时）将流量泛洪到所有端口时，bond 设备会收到同一数据包的多个副本（每个 slave 设备
一个）。

重复数据包的行为取决于交换机，有些交换机会出现这种行为，有些则不会。在表现出这种行为
的交换机上，可以通过清除 MAC 转发表来引发它（在大多数 Cisco 交换机上，特权命令
“clear mac address-table dynamic” 可以实现这一点）。

## 14. 硬件相关注意事项


本节包含在特定硬件平台上配置 bonding，或将 bonding 与特定交换机或其他设备对接的附加信息。

### 14.1 IBM BladeCenter


这适用于 JS20 及类似系统。

在 JS20 刀片上，bonding 驱动仅支持 balance-rr、active-backup、balance-tlb 和 balance-alb
模式。这在很大程度上是由于 BladeCenter 内部的网络拓扑，详见下文。

### JS20 网络适配器信息


所有 JS20 都配有集成在 planar（IBM 术语中的“主板”）上的两个 Broadcom 千兆以太网端口。在
BladeCenter 机箱中，所有 JS20 刀片的 eth0 端口都硬连线到 I/O 模块 #1；类似地，所有 eth1
端口都连线到 I/O 模块 #2。可以在 JS20 上安装一个附加的 Broadcom 子卡，以提供另外两个千兆
以太网端口。这些端口 eth2 和 eth3 分别连线到 I/O 模块 3 和 4。

每个 I/O 模块可以包含一个交换机或一个直通模块（允许端口直接连接到外部交换机）。某些
bonding 模式需要特定的 BladeCenter 内部网络拓扑才能工作；这些详述如下。

更多 BladeCenter 特定的网络信息可以在两本 IBM Redbook（www.ibm.com/redbooks）中找到：

- "IBM eServer BladeCenter Networking Options"
- "IBM eServer BladeCenter Layer 2-7 Network Switching"

### BladeCenter 网络配置


由于 BladeCenter 可以以非常多的方式配置，本讨论将仅限于描述基本配置。

通常，以太网交换模块（ESM）用于 I/O 模块 1 和 2。在此配置中，JS20 的 eth0 和 eth1 端口
将连接到不同的内部交换机（在各自的 I/O 模块中）。

直通模块（OPM 或 CPM，光口或铜口直通模块）将 I/O 模块直接连接到外部交换机。通过在 I/O
模块 #1 和 #2 中使用 PM，JS20 的 eth0 和 eth1 接口可以被重定向到外部世界，并连接到共同的
外部交换机。

根据 ESM 和 PM 的组合，网络对 bonding 而言要么表现为单交换机拓扑（全部为 PM），要么表现为
多交换机拓扑（一个或多个 ESM，零个或多个 PM）。也可以将 ESM 互连，从而产生一个非常类似于
上面“多交换机拓扑中的高可用”示例的配置。

### 特定模式的要求


balance-rr 模式要求 bond 中的设备使用直通模块，全部连接到一个共同的外部交换机。该交换机必须
在适当的端口上配置为 “etherchannel” 或 “trunking”，这是 balance-rr 的通常要求。

balance-alb 和 balance-tlb 模式可以同时使用交换机模块或直通模块（或混合）。这些模式唯一
特定的要求是，所有网络接口必须能够到达通过 bonding 设备发送的流量的所有目的地（即网络必须
在 BladeCenter 之外的某个点汇聚）。

active-backup 模式没有额外要求。

### 链路监控问题


当以太网交换模块就位时，只有 ARP 监控能可靠地检测到与外部交换机的链路丢失。这没什么异常，
但检查 BladeCenter 机柜会让人以为“外部”网络端口就是系统的以太网端口，而事实上在这些“外部”
端口与 JS20 系统本身上的设备之间有一个交换机。MII 监控只能检测 ESM 与 JS20 系统之间的链路
故障。

当直通模块就位时，MII 监控确实能检测到“外部”端口的故障，该端口随后直接连接到 JS20 系统。

### 其他注意事项


Serial Over LAN（SoL）链路仅建立在主以太网（eth0）上，因此，任何到 eth0 的链路丢失都将导致
你失去 SoL 连接。它不会与其他网络流量一起故障切换，因为 SoL 系统超出了 bonding 驱动的控制
范围。

可能希望禁用交换机（无论是内部以太网交换模块还是外部交换机）上的生成树，以避免在使用 bonding
时出现故障切换延迟问题。


## 15. 常见问题解答


### 1.  它是否 SMP 安全？


是的。旧的 2.0.xx channel bonding 补丁不是 SMP 安全的。新的驱动从一开始设计就是 SMP 安全的。

### 2.  哪些类型的网卡可以与它一起工作？


任何以太网类型的网卡（你甚至可以混用网卡——例如一块 Intel EtherExpress PRO/100 和一块 3com
3c905b）。对于大多数模式，设备不需要具有相同的速率。

从版本 3.2.1 起，bonding 还支持 active-backup 模式下的 Infiniband slave。

### 3.  我可以有多少个 bonding 设备？


没有限制。

### 4.  一个 bonding 设备可以有多少个 slave？


这仅受 Linux 支持的网络接口数量，和/或你可以在系统中放入的网络卡数量的限制。

### 5.  当 slave 链路死掉时会发生什么？


如果启用了链路监控，则失败的设备将被禁用。active-backup 模式将故障切换到备份链路，其他模式
将忽略失败的链路。该链路将继续被监控，如果它恢复，它将以适合该模式的方式重新加入 bond。有关
更多信息，请参见高可用一节以及每种模式的文档。

链路监控可以通过 miimon 或 arp_interval 参数（如上模块参数一节所述）启用。一般来说，miimon
监控底层网络设备感知到的 carrier 状态，而 arp 监控（arp_interval）监控与本地网络上另一台主机的
连通性。

如果未配置链路监控，bonding 驱动将无法检测链路故障，并将假定所有链路始终可用。这很可能会导致
丢包，以及随之而来的性能下降。确切的性能损失取决于 bonding 模式与网络配置。

### 6.  bonding 能用于高可用吗？


可以。详见高可用一节。

### 7.  它适用于哪些交换机/系统？


对此的完整答案取决于所需的模式。

在基本均衡模式（balance-rr 和 balance-xor）中，它适用于任何支持 etherchannel（也称为 trunking）
的系统。目前大多数受管交换机都有此类支持，许多非受管交换机也有。

高级均衡模式（balance-tlb 和 balance-alb）没有特殊的交换机要求，但需要支持特定功能的设备驱动
（在上面的模块参数下的相应小节中描述）。

在 802.3ad 模式中，它适用于支持 IEEE 802.3ad 动态链路聚合的系统。目前大多数受管以及许多非受管
交换机都支持 802.3ad。

active-backup 模式应该适用于任何二层（Layer-II）交换机。

### 8.  bonding 设备的 MAC 地址从哪里来？


当使用具有固定 MAC 地址的 slave 设备，或启用了 fail_over_mac 选项时，bonding 设备的 MAC 地址是
active slave 的 MAC 地址。

对于其他配置，如果未显式配置（使用 ifconfig 或 ip link），bonding 设备的 MAC 地址取自其第一个
slave 设备。该 MAC 地址随后被传递给所有后续的 slave，并保持持久（即使第一个 slave 被移除），直到
bonding 设备被 down 或重新配置。

如果你想更改 MAC 地址，可以使用

```

	# ifconfig bond0 hw ether 00:11:22:33:44:55

	# ip link set bond0 address 66:77:88:99:aa:bb

```
MAC 地址也可以通过将 bond 设备 down/up 来更改

```

	# ifconfig bond0 down ; modprobe -r bonding
	# ifconfig bond0 .... up
	# ifenslave bond0 eth...

```
此方法将自动从接下来加入的 slave 获取地址。

要恢复你的 slave 的 MAC 地址，你需要将它们从 bond 上分离（`ifenslave -d bond0 eth0`）。bonding
驱动随后将恢复这些 slave 在被 enslave 之前拥有的 MAC 地址。

### 9.  哪些 bonding 模式支持原生 XDP？


  - balance-rr (0)
  - active-backup (1)
  - balance-xor (2)
  - 802.3ad (4)

注意，vlan+srcmac 哈希策略不支持原生 XDP。对于其他 bonding 模式，XDP 程序必须以通用模式加载。

## 16. 资源与链接


bonding 驱动的最新版本可以在 linux 内核的最新版本中找到，位于 http://kernel.org

本文档的最新版本可以在最新内核源码中找到（名为 Documentation/networking/bonding.rst）。

有关 bonding 驱动开发的讨论发生在主要的 Linux 网络邮件列表上，托管于 vger.kernel.org。该列表的
地址为：

netdev@vger.kernel.org

管理界面（用于订阅或退订）可以在以下位置找到：

http://vger.kernel.org/vger-lists.html#netdev
