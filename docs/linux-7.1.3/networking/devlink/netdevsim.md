## netdevsim devlink 鏀寔


鏈枃妗ｆ弿杩颁簡 `netdevsim` 璁惧椹卞姩鏀寔鐨?`devlink` 鐗规€с€?
## 鍙傛暟


   - - Name
     - Mode
   - - `max_macs`
     - driverinit

`netdevsim` 椹卞姩杩樺疄鐜颁簡浠ヤ笅椹卞姩鐗瑰畾鐨勫弬鏁般€?
   :widths: 5 5 5 85

   - - Name
     - Type
     - Mode
     - Description
   - - `test1`
     - Boolean
     - driverinit
     - 鐢ㄤ簬灞曠ず椹卞姩鐗瑰畾鐨?devlink 鍙傛暟濡備綍瀹炵幇鐨勬祴璇曞弬鏁般€?
`netdevsim` 椹卞姩鏀寔閫氳繃 `DEVLINK_CMD_RELOAD` 閲嶆柊鍔犺浇銆?
## 鍖哄煙锛圧egions锛?

`netdevsim` 椹卞姩瀵煎嚭涓€涓?`dummy` 鍖哄煙锛屼綔涓?devlink-region 鎺ュ彛濡備綍宸ヤ綔鐨勭ず渚嬨€傛瘡褰撳悜
`take_snapshot` debugfs 鏂囦欢鍐欏叆鏃讹紝灏变細鑾峰彇涓€娆″揩鐓с€?
## 璧勬簮


`netdevsim` 椹卞姩瀵煎嚭璧勬簮浠ユ帶鍒堕┍鍔ㄥ皢鍏佽鐨?FIB 鏉＄洰銆丗IB 瑙勫垯鏉＄洰鍜?nexthops 鐨勬暟閲忋€?

    $ devlink resource set netdevsim/netdevsim0 path /IPv4/fib size 96
    $ devlink resource set netdevsim/netdevsim0 path /IPv4/fib-rules size 16
    $ devlink resource set netdevsim/netdevsim0 path /IPv6/fib size 64
    $ devlink resource set netdevsim/netdevsim0 path /IPv6/fib-rules size 16
    $ devlink resource set netdevsim/netdevsim0 path /nexthops size 16
    $ devlink dev reload netdevsim/netdevsim0

## 閫熺巼瀵硅薄


`netdevsim` 椹卞姩鏀寔閫熺巼瀵硅薄绠＄悊锛屽寘鎷細

- 涓烘瘡涓?VF devlink 绔彛娉ㄥ唽/娉ㄩ攢鍙跺瓙锛坙eaf锛夐€熺巼瀵硅薄锛?- 鍒涘缓/鍒犻櫎鑺傜偣閫熺巼瀵硅薄锛?- 涓轰换鎰忛€熺巼瀵硅薄绫诲瀷璁剧疆 tx_share 鍜?tx_max 閫熺巼鍊硷紱
- 涓轰换鎰忛€熺巼瀵硅薄绫诲瀷璁剧疆鐖惰妭鐐广€?
閫熺巼鑺傜偣鍙婂叾鍙傛暟鍦?`netdevsim` debugfs 涓互鍙锛圧O锛夋ā寮忓鍑恒€備緥濡傚垱寤虹殑鍚嶄负 `some_group`
鐨勯€熺巼鑺傜偣锛?

    $ ls /sys/kernel/debug/netdevsim/netdevsim0/rate_groups/some_group
    rate_parent  tx_max  tx_share

鐩稿悓鐨勫弬鏁板湪鐩稿簲绔彛鐩綍涓嬩负鍙跺瓙瀵硅薄瀵煎嚭銆備緥濡傦細


    $ ls /sys/kernel/debug/netdevsim/netdevsim0/ports/1
    dev  ethtool  rate_parent  tx_max  tx_share

## 椹卞姩鐗瑰畾鐨?Traps


   :widths: 5 5 90

   - - Name
     - Type
     - Description
   - - `fid_miss`
     - `exception`
     - 褰撴暟鎹寘杩涘叆璁惧鏃讹紝浼氬熀浜庡叆绔彛鍜?VLAN 灏嗗叾鍒嗙被鍒颁竴涓繃婊ゆ爣璇嗙锛團ID锛夈€?       璇?trap 鐢ㄤ簬鎹曡幏鏃犳硶鎵惧埌 FID 鐨勬暟鎹寘
