
## Devlink Region


`devlink` region 鏀寔浣跨敤 devlink 璁块棶鐢遍┍鍔ㄥ畾涔夌殑鍦板潃鍖哄煙銆?
姣忎釜璁惧鍙互鍒涘缓骞舵敞鍐屽畠鑷繁鏀寔鐨勫湴鍧€鍖哄煙銆傞殢鍚庡彲閫氳繃 devlink region 鎺ュ彛璁块棶璇ュ尯鍩熴€?
鍖哄煙蹇収鐢遍┍鍔ㄩ噰闆嗭紝骞跺彲閫氳繃 read 鎴?dump 鍛戒护璁块棶銆傝繖鍏佽瀵瑰垱寤虹殑蹇収杩涜鍚庣画鍒嗘瀽銆?鍖哄煙鍙互閫夋嫨鎬у湴鏀寔鎸夐渶瑙﹀彂蹇収銆?
蹇収鏍囪瘑绗︾殑浣滅敤鍩熸槸 devlink 瀹炰緥锛岃€屼笉鏄煇涓尯鍩熴€傚悓涓€ devlink 瀹炰緥涓墍鏈夊叿鏈夌浉鍚屽揩鐓?id 鐨?蹇収瀵瑰簲浜庡悓涓€浜嬩欢銆?
鍒涘缓鍖哄煙鐨勪富瑕佸ソ澶勬槸鎻愪緵瀵瑰唴閮ㄥ湴鍧€鍖哄煙鐨勮闂紝杩欎簺鍖哄煙鍘熸湰瀵圭敤鎴锋槸涓嶅彲璁块棶鐨勩€?
鍖哄煙涔熷彲鐢ㄤ簬鎻愪緵璋冭瘯澶嶆潅閿欒鐘舵€佺殑棰濆鏂瑰紡锛屼絾鍙﹁鍙傝 Documentation/networking/devlink/devlink-health.rst

鍖哄煙鍙互閫夋嫨鎬у湴鏀寔閫氳繃 `DEVLINK_CMD_REGION_NEW` netlink 娑堟伅鎸夐渶鎹曡幏蹇収銆備竴涓笇鏈涘厑璁?璇锋眰蹇収鐨勯┍鍔ㄥ繀椤诲湪瀹冪殑 `devlink_region_ops` 缁撴瀯浣撲腑瀹炵幇 `.snapshot` 鍥炶皟銆傚鏋滃湪
`DEVLINK_CMD_REGION_NEW` 璇锋眰涓病鏈夎缃揩鐓?id锛屽唴鏍稿皢鍒嗛厤涓€涓苟鎶婂揩鐓т俊鎭彂閫佺粰鐢ㄦ埛绌洪棿銆?
鍖哄煙鍙互閫夋嫨鎬у湴鍏佽鍦ㄦ病鏈夊揩鐓х殑鎯呭喌涓嬬洿鎺ヤ粠鍏跺唴瀹硅鍙栥€傜洿鎺ヨ鍙栬姹備笉鏄師瀛愮殑銆傜壒鍒湴锛屽ぇ灏?涓?256 瀛楄妭鎴栨洿澶х殑璇诲彇璇锋眰浼氳鎷嗗垎鎴愬涓潡銆傚鏋滈渶瑕佸師瀛愯闂紝璇蜂娇鐢ㄥ揩鐓с€備竴涓笇鏈涗负姝ゅ惎鐢?鐨勯┍鍔ㄥ簲璇ュ湪 `devlink_region_ops` 缁撴瀯浣撲腑瀹炵幇 `.read` 鍥炶皟銆傜敤鎴风┖闂村彲浠ラ€氳繃浣跨敤
`DEVLINK_ATTR_REGION_DIRECT` 灞炴€ц€屼笉鏄寚瀹氬揩鐓?id 鏉ヨ姹傜洿鎺ヨ鍙栥€?
### 浣跨敤绀轰緥


    $ devlink region help
    $ devlink region show [ DEV/REGION ]
    $ devlink region del DEV/REGION snapshot SNAPSHOT_ID
    $ devlink region dump DEV/REGION [ snapshot SNAPSHOT_ID ]
    $ devlink region read DEV/REGION [ snapshot SNAPSHOT_ID ] address ADDRESS length LENGTH

    # 鏄剧ず鎵€鏈夋毚闇茬殑鍖哄煙鍙婂叾鍖哄煙澶у皬锛?    $ devlink region show
    pci/0000:00:05.0/cr-space: size 1048576 snapshot [1 2] max 8
    pci/0000:00:05.0/fw-health: size 64 snapshot [1 2] max 8

    # 浣跨敤浠ヤ笅鍛戒护鍒犻櫎涓€涓揩鐓э細
    $ devlink region del pci/0000:00:05.0/cr-space snapshot 1

    # 璇锋眰涓€涓嵆鏃跺揩鐓э紙濡傛灉璇ュ尯鍩熸敮鎸侊級
    $ devlink region new pci/0000:00:05.0/cr-space
    pci/0000:00:05.0/cr-space: snapshot 5

    # 杞偍涓€涓揩鐓э細
    $ devlink region dump pci/0000:00:05.0/fw-health snapshot 1
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8
    0000000000000020 0016 0bb8 0016 1720 0000 0000 c00f 3ffc
    0000000000000030 bada cce5 bada cce5 bada cce5 bada cce5

    # 璇诲彇蹇収鐨勭壒瀹氶儴鍒嗭細
    $ devlink region read pci/0000:00:05.0/fw-health snapshot 1 address 0 length 16
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30

    # 鍦ㄦ病鏈夊揩鐓х殑鎯呭喌涓嬩粠鍖哄煙璇诲彇
    $ devlink region read pci/0000:00:05.0/fw-health address 16 length 16
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8

鐢变簬鍖哄煙寰堝彲鑳介潪甯镐緷璧栦簬璁惧鎴栭┍鍔紝鍥犳娌℃湁瀹氫箟閫氱敤鐨勫尯鍩熴€傛湁鍏虫煇涓┍鍔ㄦ敮鎸佺殑鍏蜂綋鍖哄煙鐨勪俊鎭紝
璇峰弬瑙侀┍鍔ㄤ笓鏈夋枃妗ｆ枃浠躲€?