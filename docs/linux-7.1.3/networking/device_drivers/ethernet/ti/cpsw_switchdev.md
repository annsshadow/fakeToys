
## 寰峰窞浠櫒锛圱exas Instruments锛塁PSW 鍩轰簬 switchdev 鐨勪互澶綉椹卞姩


:Version: 2.0

## 绔彛閲嶅懡鍚?


鍦ㄨ緝鏃х殑 udev 鐗堟湰涓婏紝灏?ethX 閲嶅懡鍚嶄负 swXpY 涓嶄細鑷姩鏀寔

```

    ip -d link show dev sw0p1 | grep switchid

    SUBSYSTEM=="net", ACTION=="add", ATTR{phys_switch_id}==<switchid>, \
	    ATTR{phys_port_name}!="", NAME="sw0$attr{phys_port_name}"


```
## 鍙?MAC 妯″紡


- 鏂扮殑锛坈psw_new.c锛夐┍鍔ㄩ粯璁や互鍙?emac 妯″紡杩愯锛屽洜姝や綔涓?2 涓嫭绔嬬殑缃戠粶鎺ュ彛宸ヤ綔銆備笌浼犵粺鐨?CPSW 椹卞姩鐨勪富瑕佸尯鍒槸锛?

 - 浼樺寲鐨勬贩鏉傦紙promiscuous锛夋ā寮忥細闄や簡 ALLMULTI锛堝綋鍓嶇鍙ｏ級澶栵紝杩樺惎鐢?P0_UNI_FLOOD锛堜袱涓鍙ｏ級锛岃€屼笉鏄?ALE_BYPASS銆傚洜姝わ紝澶勪簬娣锋潅妯″紡鐨勭鍙ｅ皢淇濈暀 mcast 鍜?vlan 杩囨护鐨勫彲鑳芥€э紝褰撶鍙ｈ鍔犲叆鍚屼竴涓ˉ鎺ワ紙浣嗘湭鍚敤鈥滀氦鎹㈡満鈥濇ā寮忥級鎴栦笉鍚岀殑妗ユ帴鏃讹紝杩欏甫鏉ヤ簡鏄捐憲鐨勫ソ澶勩€?
 - 鍦ㄧ鍙ｄ笂绂佺敤瀛︿範锛坙earning锛夛紝鍥犱负瀵归殧绂荤殑绔彛鎰忎箟涓嶅ぇ鈥斺€旂‖浠朵腑涓嶈繘琛岃浆鍙戙€?
 - 鍚敤浜嗗 devlink 鐨勫熀鏈敮鎸併€?

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
## Devlink 閰嶇疆鍙傛暟


鍙傝 Documentation/networking/devlink/ti-cpsw-switch.rst

## 鍙?MAC 妯″紡涓嬬殑妗ユ帴


鍙?mac 妯″紡闇€瑕佷繚鐣欎袱涓?vid 渚涘唴閮ㄤ娇鐢紝榛樿鎯呭喌涓嬪畠浠瓑浜?CPSW 绔彛鍙枫€傚洜姝わ紝妗ユ帴蹇呴』
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
## 鍚敤鈥滀氦鎹㈡満鈥?


鍙互閫氳繃閰嶇疆 devlink 椹卞姩鍙傛暟鏉ュ惎鐢ㄤ氦鎹㈡満妯″紡
```

	devlink dev param set platform/48484000.switch \
	name switch_mode value 1 cmode runtime

```
杩欏彲浠ヤ笉鍙楃鍙?netdev 璁惧鐘舵€侊紙UP/DOWN锛夌殑褰卞搷鏉ュ畬鎴愶紝浣嗗湪鍔犲叆妗ユ帴涔嬪墠锛岀鍙ｇ殑 netdev 璁惧蹇呴』澶勪簬 UP 鐘舵€侊紝浠ラ伩鍏嶈鐩栨ˉ鎺ラ厤缃紝鍥犱负 CPSW 浜ゆ崲鏈洪┍鍔ㄥ湪绗竴涓鍙ｇ姸鎬佸彉涓?UP 鏃朵細瀹屽叏閲嶆柊鍔犺浇鍏堕厤缃€?

褰撲袱涓帴鍙ｉ兘鍔犲叆妗ユ帴鍚庘€斺€擟PSW 浜ゆ崲鏈洪┍鍔ㄥ皢鍚敤鐢?offload_fwd_mark 鏍囧織鏍囪鏁版嵁鍖咃紝闄ら潪 "ale_bypass=0"

鎵€鏈夐厤缃兘閫氳繃 switchdev API 瀹炵幇銆?

## 妗ユ帴璁剧疆


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
## 寮€鍚?鍏抽棴 STP


```

	ip link set dev BRDEV type bridge stp_state 1/0

```
## VLAN 閰嶇疆


```

  bridge vlan add dev br0 vid 1 pvid untagged self <---- add cpu port to VLAN 1

```
娉ㄦ剰锛氳繖涓€姝ュ浜?bridge/default_pvid 鏄繀闇€鐨勩€?

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
### FDB


FDB 鍦ㄦ娴嬪埌鏃朵細鑷姩娣诲姞鍒扮浉搴旂殑浜ゆ崲鏈虹鍙ｄ笂
```

    bridge fdb add aa:bb:cc:dd:ee:ff dev sw0p1 master vlan 100
    bridge fdb add aa:bb:cc:dd:ee:fe dev sw0p2 master <---- Add on all VLANs

```
### MDB


MDB 鍦ㄦ娴嬪埌鏃朵細鑷姩娣诲姞鍒扮浉搴旂殑浜ゆ崲鏈虹鍙ｄ笂
```

  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent vid 100
  bridge mdb add dev br0 port sw0p1 grp 239.1.1.1 permanent <---- Add on all VLANs

```
## 缁勬挱娉涙椽


CPU 绔彛鐨?mcast_flooding 濮嬬粓寮€鍚?

鍦ㄤ氦鎹㈡満绔彛涓婂紑鍚?鍏抽棴娉涙椽锛?
bridge link set dev sw0p1 mcast_flood on/off

## 鎺ュ叆绔彛涓庝腑缁х鍙ｏ紙Access and Trunk port锛?


```

 bridge vlan add dev sw0p1 vid 100 pvid untagged master
 bridge vlan add dev sw0p2 vid 100 master


 bridge vlan add dev br0 vid 100 self
 ip link add link br0 name br0.100 type vlan id 100

```
娉ㄦ剰锛氬湪妗ユ帴璁惧鑷韩涓婅缃?PVID 浠呭榛樿 VLAN锛坉efault_pvid锛夋湁鏁堛€?

## NFS


NFS 鑳藉宸ヤ綔鐨勫敮涓€鏂瑰紡锛屾槸鍦ㄩ渶瑕佸奖鍝嶈繛閫氭€х殑浜ゆ崲鏈洪厤缃椂锛宑hroot 鍒颁竴涓渶灏忕幆澧冧腑銆傚亣璁句綘鏄€氳繃 eth1 鎺ュ彛鍚姩 NFS锛堣鑴氭湰姣旇緝绮楃硻锛屽彧鏄敤鏉ヨ瘉鏄?NFS 鏄彲琛岀殑锛夈€?

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