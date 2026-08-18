
## 浣跨敤 bnx2fc 杩愯 FCoE


閫氳繃 bnx2fc 瀹炵幇鐨勫崥閫氾紙Broadcom锛塅CoE 鍗歌浇鏄竴绉嶅叏鐘舵€佺‖浠跺嵏杞斤紝瀹冧笌 Linux 鐢熸€佺郴缁熶腑涓?FC/FCoE 鍜?SCSI 鎺у埗鍣ㄦ彁渚涚殑鎵€鏈夋帴鍙ｅ崗鍚屽伐浣溿€傚洜姝わ紝涓€鏃﹀惎鐢紝FCoE 鍔熻兘鍦ㄥ緢澶х▼搴︿笂鏄€忔槑鐨勩€傚湪 SAN 涓婂彂鐜扮殑璁惧浼氳嚜鍔ㄥ悜楂樺眰瀛樺偍灞傛敞鍐屽拰娉ㄩ攢銆?
灏界鍗氶€氱殑 FCoE 鍗歌浇鏄畬鍏ㄥ嵏杞界殑锛屼絾瀹冪‘瀹炰緷璧栦簬缃戠粶鎺ュ彛鐨勮繍琛岀姸鎬併€傚洜姝わ紝涓?FCoE 鍗歌浇鍚姩鍣ㄥ叧鑱旂殑缃戠粶鎺ュ彛锛堜緥濡?eth0锛夊繀椤诲浜?'up' 鐘舵€併€傚缓璁皢缃戠粶鎺ュ彛閰嶇疆涓哄湪鍚姩鏃惰嚜鍔ㄥ惎鐢ㄣ€?
姝ゅ锛屽崥閫?FCoE 鍗歌浇鏂规浼氬垱寤?VLAN 鎺ュ彛锛屼互鏀寔涓?FCoE 鎿嶄綔鍙戠幇鐨?VLAN锛堜緥濡?eth0.1001-fcoe锛夈€備笉瑕佸垹闄ゆ垨绂佺敤杩欎簺鎺ュ彛锛屽惁鍒?FCoE 鎿嶄綔灏嗕腑鏂€?
## 椹卞姩浣跨敤妯″瀷锛?

1. 纭繚宸插畨瑁?fcoe-utils 杞欢鍖呫€?
2. 閰嶇疆 bnx2fc 椹卞姩闇€瑕佽繍琛岀殑鎺ュ彛銆?閰嶇疆姝ラ濡備笅锛?
	a. cd /etc/fcoe
	b. 濡傛灉闇€瑕佸湪 eth5 涓婂惎鐢?FCoE锛屽皢 cfg-ethx 澶嶅埗涓?cfg-eth5銆?	c. 瀵规墍鏈夐渶瑕佸惎鐢?FCoE 鐨勬帴鍙ｉ噸澶嶆鎿嶄綔銆?	d. 缂栬緫鎵€鏈?cfg-eth 鏂囦欢锛屽皢 DCB_REQUIRED** 瀛楁璁句负 "no"锛屽皢
	   AUTO_VLAN 璁句负 "yes"銆?	e. 鍏朵粬閰嶇疆鍙傛暟淇濇寔榛樿鍗冲彲銆?
3. 纭繚 "bnx2fc" 浣嶄簬 /etc/fcoe/config 鐨?SUPPORTED_DRIVERS 鍒楄〃涓€?
4. 鍚姩 fcoe 鏈嶅姟銆傦紙service fcoe start锛夈€傚鏋滅郴缁熶腑瀛樺湪鍗氶€氳澶囷紝bnx2fc 椹卞姩浼氳嚜鍔ㄥ崰鐢ㄨ繖浜涙帴鍙ｏ紝寮€濮?vlan 鍙戠幇骞剁櫥褰曞埌鐩爣銆?
5. 'fcoeadm -i' 杈撳嚭涓殑 "Symbolic Name" 浼氭樉绀?bnx2fc 鏄惁宸插崰鐢ㄨ鎺ュ彛銆?
```

 [root@bh2 ~]# fcoeadm -i
    Description:      NetXtreme II BCM57712 10 Gigabit Ethernet
    Revision:         01
    Manufacturer:     Broadcom Corporation
    Serial Number:    0010186FD558
    Driver:           bnx2x 1.70.00-0
    Number of Ports:  2

        Symbolic Name:     bnx2fc v1.0.5 over eth5.4
        OS Device Name:    host11
        Node Name:         0x10000010186FD559
        Port Name:         0x20000010186FD559
        FabricName:        0x2001000DECB3B681
        Speed:             10 Gbit
        Supported Speed:   10 Gbit
        MaxFrameSize:      2048
        FC-ID (Port ID):   0x0F0377
        State:             Online

```
6. 閫氳繃杩愯 ifconfig 骞剁暀鎰忎細鑷姩鍒涘缓鐨?<INTERFACE>.<VLAN>-fcoe 鎺ュ彛锛岄獙璇佹槸鍚﹀凡鎵ц vlan 鍙戠幇銆?
鏈夊叧 fcoeadm 鎿嶄綔鐢ㄦ潵鍒涘缓/閿€姣佹帴鍙ｆ垨鏄剧ず lun/鐩爣淇℃伅鐨勬洿澶氫俊鎭紝璇峰弬闃?fcoeadm 鎵嬪唽椤点€?
## 娉ㄦ剰

** 鏀寔鍗氶€?FCoE 鐨勮澶囧湪鑺墖涓婂疄鐜颁簡 DCBX/LLDP 瀹㈡埛绔€傛瘡涓帴鍙ｅ彧鍏佽鏈変竴涓?LLDP 瀹㈡埛绔€備负姝ｅ父杩愯锛屽繀椤荤鐢ㄦ墍鏈夊熀浜庝富鏈鸿蒋浠剁殑 DCBX/LLDP 瀹㈡埛绔紙渚嬪 lldpad锛夈€傝绂佺敤 lldpad锛岃浣跨敤
```

	lldptool set-lldp -i <interface_name> adminStatus=disabled

```
