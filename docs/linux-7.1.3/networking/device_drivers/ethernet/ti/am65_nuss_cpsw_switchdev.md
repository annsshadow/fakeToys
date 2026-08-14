
## Texas Instruments K3 AM65 CPSW NUSS 鍩轰簬 switchdev 鐨勪互澶綉椹卞姩


:Version:1.0

## 绔彛閲嶅懡鍚?

```

    ip -d link show dev sw0p1 | grep switchid

    SUBSYSTEM=="net", ACTION=="add", ATTR{phys_switch_id}==<switchid>, \
	    ATTR{phys_port_name}!="", NAME="sw0$attr{phys_port_name}"

```

## 澶?MAC 妯″紡


- 椹卞姩榛樿浠ュ MAC 妯″紡杩愯锛屽洜姝よ〃鐜颁负 N 涓嫭绔嬬殑缃戠粶鎺ュ彛銆?
## Devlink 閰嶇疆鍙傛暟


鍙傝 Documentation/networking/devlink/am65-nuss-cpsw-switch.rst

## 鍚敤 "switch" 妯″紡


Switch 妯″紡鍙€氳繃閰嶇疆 devlink 椹卞姩鍙傛暟鏉ュ惎鐢細

```

        devlink dev param set platform/c000000.ethernet \
        name switch_mode value true cmode runtime

```

鏃犺绔彛鐨勭綉缁滄帴鍙ｅ浜?UP 杩樻槸 DOWN 鐘舵€佸潎鍙繘琛岋紱褰撶鍙ｇ殑缃戠粶鎺ュ彛澶勪簬 UP
鐘舵€佸苟鍔犲叆缃戞ˉ鏃讹紝CPSW switch 椹卞姩浼氬畬鍏ㄩ噸鏂板姞杞藉叾閰嶇疆锛屼互閬垮厤瑕嗙洊缃戞ˉ閰嶇疆銆?璇ラ厤缃€氳繃 switchdev API 瀹炵幇銆?
## 缃戞ˉ閰嶇疆


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

## STP 寮€鍚?鍏抽棴


```

	ip link set dev BRDEV type bridge stp_state 1/0

```

## VLAN 閰嶇疆


```

  bridge vlan add dev br0 vid 1 pvid untagged self <---- add cpu port to VLAN 1

```

璇存槑锛氳姝ラ瀵逛簬缃戞ˉ/榛樿 PVID锛坉efault_pvid锛変负蹇呴渶銆?
## 娣诲姞棰濆鐨?VLAN


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


FDB 浼氭牴鎹浉搴旂殑浜ゆ崲鏈虹鍙ｆ娴嬬粨鏋滆嚜鍔ㄦ坊鍔犮€?
```

    bridge fdb add aa:bb:cc:dd:ee:ff dev sw0p1 master vlan 100
    bridge fdb add aa:bb:cc:dd:ee:fe dev sw0p2 master <---- Add on all VLANs

```

### MDBs


MDB 浼氭牴鎹浉搴旂殑浜ゆ崲鏈虹鍙ｆ娴嬬粨鏋滆嚜鍔ㄦ坊鍔犮€?
```

  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent vid 100
  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent <---- Add on all VLANs

```

## 缁勬挱娉涙椽


CPU 绔彛鐨?mcast_flooding 濮嬬粓寮€鍚€?
鍦ㄤ氦鎹㈡満绔彛涓婂紑鍚?鍏抽棴娉涙椽锛?bridge link set dev sw0p1 mcast_flood on/off

## 璁块棶 Trunk 绔彛


```

 bridge vlan add dev sw0p1 vid 100 pvid untagged master
 bridge vlan add dev sw0p2 vid 100 master


 bridge vlan add dev br0 vid 100 self
 ip link add link br0 name br0.100 type vlan id 100

```

璇存槑锛氬湪缃戞ˉ璁惧鑷韩涓婅缃?PVID 閫傜敤浜庨粯璁?VLAN锛坉efault_pvid锛夈€?