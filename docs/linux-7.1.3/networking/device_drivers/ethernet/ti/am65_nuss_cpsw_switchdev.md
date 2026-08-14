
## Texas Instruments K3 AM65 CPSW NUSS 基于 switchdev 的以太网驱动


:Version:1.0

## 端口重命名


```

    ip -d link show dev sw0p1 | grep switchid

    SUBSYSTEM=="net", ACTION=="add", ATTR{phys_switch_id}==<switchid>, \
	    ATTR{phys_port_name}!="", NAME="sw0$attr{phys_port_name}"

```

## 多 MAC 模式


- 驱动默认以多 MAC 模式运行，因此表现为 N 个独立的网络接口。

## Devlink 配置参数


参见 Documentation/networking/devlink/am65-nuss-cpsw-switch.rst

## 启用 "switch" 模式


Switch 模式可通过配置 devlink 驱动参数来启用：

```

        devlink dev param set platform/c000000.ethernet \
        name switch_mode value true cmode runtime

```

无论端口的网络接口处于 UP 还是 DOWN 状态均可进行；当端口的网络接口处于 UP
状态并加入网桥时，CPSW switch 驱动会完全重新加载其配置，以避免覆盖网桥配置。
该配置通过 switchdev API 实现。

## 网桥配置


```

        devlink dev param set platform/c000000.ethernet \
        name switch_mode value true cmode runtime

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

## STP 开启/关闭


```

	ip link set dev BRDEV type bridge stp_state 1/0

```

## VLAN 配置


```

  bridge vlan add dev br0 vid 1 pvid untagged self <---- add cpu port to VLAN 1

```

说明：该步骤对于网桥/默认 PVID（default_pvid）为必需。

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

### FDBs


FDB 会根据相应的交换机端口检测结果自动添加。

```

    bridge fdb add aa:bb:cc:dd:ee:ff dev sw0p1 master vlan 100
    bridge fdb add aa:bb:cc:dd:ee:fe dev sw0p2 master <---- Add on all VLANs

```

### MDBs


MDB 会根据相应的交换机端口检测结果自动添加。

```

  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent vid 100
  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent <---- Add on all VLANs

```

## 组播泛洪


CPU 端口的 mcast_flooding 始终开启。

在交换机端口上开启/关闭泛洪：
bridge link set dev sw0p1 mcast_flood on/off

## 访问 Trunk 端口


```

 bridge vlan add dev sw0p1 vid 100 pvid untagged master
 bridge vlan add dev sw0p2 vid 100 master


 bridge vlan add dev br0 vid 100 self
 ip link add link br0 name br0.100 type vlan id 100

```

说明：在网桥设备自身上设置 PVID 适用于默认 VLAN（default_pvid）。
