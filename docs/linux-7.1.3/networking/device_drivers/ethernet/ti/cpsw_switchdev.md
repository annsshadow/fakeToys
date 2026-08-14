
## 德州仪器（Texas Instruments）CPSW 基于 switchdev 的以太网驱动


:Version: 2.0

## 端口重命名


在较旧的 udev 版本上，将 ethX 重命名为 swXpY 不会自动支持

```

    ip -d link show dev sw0p1 | grep switchid

    SUBSYSTEM=="net", ACTION=="add", ATTR{phys_switch_id}==<switchid>, \
	    ATTR{phys_port_name}!="", NAME="sw0$attr{phys_port_name}"


```
## 双 MAC 模式


- 新的（cpsw_new.c）驱动默认以双 emac 模式运行，因此作为 2 个独立的网络接口工作。与传统的 CPSW 驱动的主要区别是：

 - 优化的混杂（promiscuous）模式：除了 ALLMULTI（当前端口）外，还启用 P0_UNI_FLOOD（两个端口），而不是 ALE_BYPASS。因此，处于混杂模式的端口将保留 mcast 和 vlan 过滤的可能性，当端口被加入同一个桥接（但未启用“交换机”模式）或不同的桥接时，这带来了显著的好处。
 - 在端口上禁用学习（learning），因为对隔离的端口意义不大——硬件中不进行转发。
 - 启用了对 devlink 的基本支持。

```

	devlink dev show
		platform/48484000.switch

	devlink dev param show
	platform/48484000.switch:
	name switch_mode type driver-specific
	values:
		cmode runtime value false
	name ale_bypass type driver-specific
	values:
		cmode runtime value false

```
## Devlink 配置参数


参见 Documentation/networking/devlink/ti-cpsw-switch.rst

## 双 MAC 模式下的桥接


双 mac 模式需要保留两个 vid 供内部使用，默认情况下它们等于 CPSW 端口号。因此，桥接必须
```

	ip link add name br0 type bridge
	ip link set dev br0 type bridge vlan_filtering 0
	echo 0 > /sys/class/net/br0/bridge/default_pvid
	ip link set dev sw0p1 master br0
	ip link set dev sw0p2 master br0

```
```

	ip link add name br0 type bridge
	ip link set dev br0 type bridge vlan_filtering 0
	echo 100 > /sys/class/net/br0/bridge/default_pvid
	ip link set dev br0 type bridge vlan_filtering 1
	ip link set dev sw0p1 master br0
	ip link set dev sw0p2 master br0

```
## 启用“交换机”


可以通过配置 devlink 驱动参数来启用交换机模式
```

	devlink dev param set platform/48484000.switch \
	name switch_mode value 1 cmode runtime

```
这可以不受端口 netdev 设备状态（UP/DOWN）的影响来完成，但在加入桥接之前，端口的 netdev 设备必须处于 UP 状态，以避免覆盖桥接配置，因为 CPSW 交换机驱动在第一个端口状态变为 UP 时会完全重新加载其配置。

当两个接口都加入桥接后——CPSW 交换机驱动将启用用 offload_fwd_mark 标志标记数据包，除非 "ale_bypass=0"

所有配置都通过 switchdev API 实现。

## 桥接设置


```

	devlink dev param set platform/48484000.switch \
	name switch_mode value 1 cmode runtime

	ip link add name br0 type bridge
	ip link set dev br0 type bridge ageing_time 1000
	ip link set dev sw0p1 up
	ip link set dev sw0p2 up
	ip link set dev sw0p1 master br0
	ip link set dev sw0p2 master br0

	[*] bridge vlan add dev br0 vid 1 pvid untagged self

	[*] if vlan_filtering=1. where default_pvid=1

	Note. Steps [*] are mandatory.


```
## 开启/关闭 STP


```

	ip link set dev BRDEV type bridge stp_state 1/0

```
## VLAN 配置


```

  bridge vlan add dev br0 vid 1 pvid untagged self <---- add cpu port to VLAN 1

```
注意：这一步对于 bridge/default_pvid 是必需的。

## 添加额外的 VLAN


```

	bridge vlan add dev sw0p1 vid 100 pvid untagged master
	bridge vlan add dev sw0p2 vid 100 pvid untagged master
	bridge vlan add dev br0 vid 100 pvid untagged self <---- Add cpu port to VLAN100

 2. tagged::

	bridge vlan add dev sw0p1 vid 100 master
	bridge vlan add dev sw0p2 vid 100 master
	bridge vlan add dev br0 vid 100 pvid tagged self <---- Add cpu port to VLAN100

```
### FDB


FDB 在检测到时会自动添加到相应的交换机端口上
```

    bridge fdb add aa:bb:cc:dd:ee:ff dev sw0p1 master vlan 100
    bridge fdb add aa:bb:cc:dd:ee:fe dev sw0p2 master <---- Add on all VLANs

```
### MDB


MDB 在检测到时会自动添加到相应的交换机端口上
```

  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent vid 100
  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent <---- Add on all VLANs

```
## 组播泛洪


CPU 端口的 mcast_flooding 始终开启

在交换机端口上开启/关闭泛洪：
bridge link set dev sw0p1 mcast_flood on/off

## 接入端口与中继端口（Access and Trunk port）


```

 bridge vlan add dev sw0p1 vid 100 pvid untagged master
 bridge vlan add dev sw0p2 vid 100 master


 bridge vlan add dev br0 vid 100 self
 ip link add link br0 name br0.100 type vlan id 100

```
注意：在桥接设备自身上设置 PVID 仅对默认 VLAN（default_pvid）有效。

## NFS


NFS 能够工作的唯一方式，是在需要影响连通性的交换机配置时，chroot 到一个最小环境中。假设你是通过 eth1 接口启动 NFS（该脚本比较粗糙，只是用来证明 NFS 是可行的）。

```

	#!/bin/sh
	mkdir proc
	mount -t proc none /proc
	ifconfig br0  > /dev/null
	if [ $? -ne 0 ]; then
		echo "Setting up bridge"
		ip link add name br0 type bridge
		ip link set dev br0 type bridge ageing_time 1000
		ip link set dev br0 type bridge vlan_filtering 1

		ip link set eth1 down
		ip link set eth1 name sw0p1
		ip link set dev sw0p1 up
		ip link set dev sw0p2 up
		ip link set dev sw0p2 master br0
		ip link set dev sw0p1 master br0
		bridge vlan add dev br0 vid 1 pvid untagged self
		ifconfig sw0p1 0.0.0.0
		udhchc -i br0
	fi
	umount /proc

```
```

	#!/bin/sh
	mkdir /tmp/root/bin -p
	mkdir /tmp/root/lib -p

	cp -r /lib/ /tmp/root/
	cp -r /bin/ /tmp/root/
	cp /sbin/ip /tmp/root/bin
	cp /sbin/bridge /tmp/root/bin
	cp /sbin/ifconfig /tmp/root/bin
	cp /sbin/udhcpc /tmp/root/bin
	cp /path/to/setup.sh /tmp/root/bin
	chroot /tmp/root/ busybox sh /bin/setup.sh

	run ./run_nfs.sh

```