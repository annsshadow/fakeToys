
## 澶?PF 缃戠粶璁惧


## 鐩綍


- `鑳屾櫙`_
- `姒傝堪`_
- `mlx5 瀹炵幇`_
- `閫氶亾鍒嗛厤`_
- `鍙娴嬫€_
- `娴佸鍚慲_
- `浜掓枼鍔熻兘`_

## 鑳屾櫙


澶?PF NIC 鎶€鏈娇澶氳矾鏈嶅姟鍣ㄤ腑鐨勫涓?CPU 鑳藉閫氳繃鍚勮嚜鐨勪笓鐢?PCIe 鎺ュ彛鐩存帴杩炴帴鍒?缃戠粶锛屾棦鍙互閫氳繃鍦ㄤ袱寮犲崱涔嬮棿鍒嗗壊 PCIe 閫氶亾鐨勮繛鎺ョ嚎鏉燂紝涔熷彲浠ラ€氳繃涓哄崟寮犲崱鍒嗗弶
PCIe 鎻掓Ы鏉ュ疄鐜般€傝繖娑堥櫎浜嗚法鎻掓Ы鍐呴儴鎬荤嚎浼犺緭鐨勭綉缁滄祦閲忥紝鏄捐憲闄嶄綆浜嗗紑閿€鍜屽欢杩燂紝
鍚屾椂鍑忓皯浜?CPU 鍒╃敤鐜囧苟鎻愰珮浜嗙綉缁滃悶鍚愰噺銆?
## 姒傝堪


璇ョ壒鎬ф敮鎸佸湪澶?PF 鐜涓皢鍚屼竴绔彛鐨勫涓?PF 鍚堝苟鍒颁竴涓?netdev 瀹炰緥涔嬩笅銆傚畠瀹炵幇浜?netdev 灞傘€傚簳灞傚疄渚嬶紙濡?pci 鍔熻兘銆乻ysfs 鏉＄洰鍜?devlink锛変繚鎸佺嫭绔嬨€?閫氳繃涓嶅悓鐨勮澶囷紙灞炰簬涓嶅悓鐨?NUMA 鎻掓Ы锛変紶杈撴祦閲忥紝鍙互鑺傜渷璺?NUMA 娴侀噺锛屽苟鍏佽
杩愯浜庡悓涓€ netdev 涓娿€佹潵鑷笉鍚?NUMA 鐨勫簲鐢ㄧ▼搴忎粛鐒舵劅鍙楀埌涓庤澶囩殑閭昏繎鎬э紝浠庤€岃幏寰?鏀瑰杽鐨勬€ц兘銆?
## mlx5 瀹炵幇


mlx5 涓殑澶?PF 鎴?Socket-direct 鏄€氳繃灏嗗睘浜庡悓涓€ NIC 涓斿惎鐢ㄤ簡 socket-direct 灞炴€х殑
PF 鍒嗙粍鍦ㄤ竴璧锋潵瀹炵幇鐨勶紝涓€鏃︽墍鏈?PF 閮借鎺㈡祴鍒帮紝鎴戜滑灏卞垱寤轰竴涓崟涓€鐨?netdev 鏉?浠ｈ〃瀹冧滑鍏ㄩ儴锛涘绉板湴锛屽綋浠讳綍涓€涓?PF 琚Щ闄ゆ椂锛屾垜浠攢姣佽 netdev銆?
netdev 鐨勭綉缁滈€氶亾琚垎閰嶅埌鎵€鏈夎澶囦箣闂达紝姝ｇ‘鐨勯厤缃細鍦ㄥ鐞嗘煇涓簲鐢?CPU 鏃朵娇鐢?姝ｇ‘鐨勯偦杩?NUMA 鑺傜偣銆?
鎴戜滑閫夋嫨涓€涓?PF 浣滀负涓?PF锛堥瀵艰€咃級锛屽畠鎵挎媴鐗规畩瑙掕壊銆傚叾瀹冭澶囷紙浠庡睘锛夊湪鑺墖灞傞潰
涓庣綉缁滄柇寮€锛堣涓洪潤榛樻ā寮忥級銆傚湪闈欓粯妯″紡涓嬶紝浠庡睘 PF 涔嬮棿娌℃湁鍗?<-> 鍖楁祦閲忕洿鎺ユ祦杩囥€?瀹冮渶瑕佸湪棰嗗鑰?PF锛堜笢 <-> 瑗挎祦閲忥級鐨勫崗鍔╀笅鎵嶈兘宸ヤ綔銆傛墍鏈?Rx/Tx 娴侀噺閮介€氳繃涓?PF
鍚戜粠灞?PF 瀵煎悜鎴栦粠涓祦鍑恒€?
鐩墠锛屾垜浠皢鏀寔闄愬埗涓轰粎 PF锛屼笖鏈€澶氫袱涓?PF锛堟彃妲斤級銆?
## 閫氶亾鍒嗛厤


鎴戜滑鍦ㄤ笉鍚岀殑 PF 涔嬮棿鍒嗛厤閫氶亾锛屼互鍦ㄥ涓?NUMA 鑺傜偣涓婂疄鐜版湰鍦?NUMA 鑺傜偣鎬ц兘銆?
姣忎釜缁勫悎閫氶亾閽堝涓€涓壒瀹氱殑 PF 宸ヤ綔锛岄拡瀵瑰畠鍒涘缓鍏舵墍鏈夌殑鏁版嵁璺緞闃熷垪銆傛垜浠互
杞绛栫暐灏嗛€氶亾鍒嗛厤缁?PF銆?
```

        Example for 2 PFs and 5 channels:
        +--------+--------+
        | ch idx | PF idx |
        +--------+--------+
        |    0   |    0   |
        |    1   |    1   |
        |    2   |    0   |
        |    3   |    1   |
        |    4   |    0   |
        +--------+--------+


```
鎴戜滑鍊惧悜浜庤疆璇㈢殑鍘熷洜鏄紝瀹冭緝灏戝彈鍒伴€氶亾鏁伴噺鍙樺寲鐨勫奖鍝嶃€傞€氶亾绱㈠紩涓?PF 涔嬮棿鐨勬槧灏?鏄浐瀹氱殑锛屾棤璁虹敤鎴烽厤缃簡澶氬皯閫氶亾銆傜敱浜庨€氶亾缁熻鍦ㄩ€氶亾鍏抽棴鏈熼棿鏄寔涔呯殑锛屾瘡娆￠兘
鏀瑰彉鏄犲皠浼氫娇绱Н缁熻涓嶈兘寰堝ソ鍦颁唬琛ㄩ€氶亾鐨勫巻鍙层€?
杩欐槸閫氳繃鍦ㄦ瘡涓€氶亾涓娇鐢ㄦ纭殑鏍稿績璁惧瀹炰緥锛坢dev锛夛紝鑰屼笉鏄叏閮ㄤ娇鐢?"priv->mdev"
涓嬬殑鍚屼竴瀹炰緥鏉ュ疄鐜扮殑銆?
## 鍙娴嬫€?```

  $ ./tools/net/ynl/pyynl/cli.py --spec Documentation/netlink/specs/netdev.yaml --dump queue-get --json='{"ifindex": 13}'
  [{'id': 0, 'ifindex': 13, 'napi-id': 539, 'type': 'rx'},
   {'id': 1, 'ifindex': 13, 'napi-id': 540, 'type': 'rx'},
   {'id': 2, 'ifindex': 13, 'napi-id': 541, 'type': 'rx'},
   {'id': 3, 'ifindex': 13, 'napi-id': 542, 'type': 'rx'},
   {'id': 4, 'ifindex': 13, 'napi-id': 543, 'type': 'rx'},
   {'id': 0, 'ifindex': 13, 'napi-id': 539, 'type': 'tx'},
   {'id': 1, 'ifindex': 13, 'napi-id': 540, 'type': 'tx'},
   {'id': 2, 'ifindex': 13, 'napi-id': 541, 'type': 'tx'},
   {'id': 3, 'ifindex': 13, 'napi-id': 542, 'type': 'tx'},
   {'id': 4, 'ifindex': 13, 'napi-id': 543, 'type': 'tx'}]

  $ ./tools/net/ynl/pyynl/cli.py --spec Documentation/netlink/specs/netdev.yaml --dump napi-get --json='{"ifindex": 13}'
  [{'id': 543, 'ifindex': 13, 'irq': 42},
   {'id': 542, 'ifindex': 13, 'irq': 41},
   {'id': 541, 'ifindex': 13, 'irq': 40},
   {'id': 540, 'ifindex': 13, 'irq': 39},
   {'id': 539, 'ifindex': 13, 'irq': 36}]

```
```

  $ ls /proc/irq/{36,39,40,41,42}/mlx5* -d -1
  /proc/irq/36/mlx5_comp0@pci:0000:08:00.0
  /proc/irq/39/mlx5_comp0@pci:0000:09:00.0
  /proc/irq/40/mlx5_comp1@pci:0000:08:00.0
  /proc/irq/41/mlx5_comp1@pci:0000:09:00.0
  /proc/irq/42/mlx5_comp2@pci:0000:08:00.0

```
## 娴佸鍚?

浠庡睘 PF 琚涓?闈欓粯"妯″紡锛屾剰鍛崇潃瀹冧滑涓庣綉缁滄柇寮€銆?
鍦?Rx 涓紝娴佸鍚戣〃浠呭睘浜庝富 PF锛岀敱鍏惰礋璐ｉ€氳繃璺?vhca 娴佸鍚戣兘鍔涘皢 incoming 娴侀噺
鍒嗗彂鍒板叾瀹?PF銆備粛鐒剁淮鎶や竴涓崟涓€鐨勯粯璁?RSS 琛紝瀹冭兘澶熸寚鍚戜笉鍚?PF 鐨勬帴鏀堕槦鍒椼€?
鍦?Tx 涓紝涓?PF 鍒涘缓涓€涓柊鐨?Tx 娴佽〃锛岀敱浠庡睘 PF 鍒悕寮曠敤锛屼互渚垮畠浠彲浠ラ€氳繃瀹?鍑哄幓鍒扮綉缁溿€?
姝ゅ锛屾垜浠缃粯璁ょ殑 XPS 閰嶇疆锛屽畠鍩轰簬 CPU 閫夋嫨灞炰簬涓庤 CPU 鍚屼竴鑺傜偣鐨?PF 鐨?SQ銆?
XPS 榛樿閰嶇疆绀轰緥锛?
NUMA node(s):          2
NUMA node0 CPU(s):     0-11
NUMA node1 CPU(s):     12-23

PF0 鍦?node0 涓婏紝PF1 鍦?node1 涓娿€?
- /sys/class/net/eth2/queues/tx-0/xps_cpus:000001
- /sys/class/net/eth2/queues/tx-1/xps_cpus:001000
- /sys/class/net/eth2/queues/tx-2/xps_cpus:000002
- /sys/class/net/eth2/queues/tx-3/xps_cpus:002000
- /sys/class/net/eth2/queues/tx-4/xps_cpus:000004
- /sys/class/net/eth2/queues/tx-5/xps_cpus:004000
- /sys/class/net/eth2/queues/tx-6/xps_cpus:000008
- /sys/class/net/eth2/queues/tx-7/xps_cpus:008000
- /sys/class/net/eth2/queues/tx-8/xps_cpus:000010
- /sys/class/net/eth2/queues/tx-9/xps_cpus:010000
- /sys/class/net/eth2/queues/tx-10/xps_cpus:000020
- /sys/class/net/eth2/queues/tx-11/xps_cpus:020000
- /sys/class/net/eth2/queues/tx-12/xps_cpus:000040
- /sys/class/net/eth2/queues/tx-13/xps_cpus:040000
- /sys/class/net/eth2/queues/tx-14/xps_cpus:000080
- /sys/class/net/eth2/queues/tx-15/xps_cpus:080000
- /sys/class/net/eth2/queues/tx-16/xps_cpus:000100
- /sys/class/net/eth2/queues/tx-17/xps_cpus:100000
- /sys/class/net/eth2/queues/tx-18/xps_cpus:000200
- /sys/class/net/eth2/queues/tx-19/xps_cpus:200000
- /sys/class/net/eth2/queues/tx-20/xps_cpus:000400
- /sys/class/net/eth2/queues/tx-21/xps_cpus:400000
- /sys/class/net/eth2/queues/tx-22/xps_cpus:000800
- /sys/class/net/eth2/queues/tx-23/xps_cpus:800000

## 浜掓枼鍔熻兘


澶?PF 鐨勬湰璐ㄦ槸涓嶅悓閫氶亾涓庝笉鍚?PF 閰嶅悎宸ヤ綔锛岃繖涓庣姸鎬佺淮鎶ゅ湪鍏朵腑涓€涓?PF 涓殑鏈夌姸鎬?鍔熻兘鐩稿啿绐併€備緥濡傦紝鍦?TLS 璁惧鍗歌浇鍔熻兘涓紝浼氫负姣忎釜杩炴帴鍒涘缓鐗规畩鐨勪笂涓嬫枃瀵硅薄骞剁淮鎶?鍦?PF 涓€傚湪涓嶅悓 RQ/SQ 涔嬮棿鍒囨崲浼氱牬鍧忚鍔熻兘銆傚洜姝わ紝鎴戜滑鏆傛椂绂佺敤浜嗚繖绉嶇粍鍚堛€?