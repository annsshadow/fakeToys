
## mlxsw devlink 鏀寔


鏈枃妗ｆ弿杩颁簡 `mlxsw` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


   - - 鍚嶇О
     - 妯″紡
   - - `fw_load_policy`
     - driverinit

`mlxsw` 椹卞姩杩樺疄鐜颁簡浠ヤ笅椹卞姩鐗瑰畾鐨勫弬鏁般€?
   :widths: 5 5 5 85

   - - 鍚嶇О
     - 绫诲瀷
     - 妯″紡
     - 鎻忚堪
   - - `acl_region_rehash_interval`
     - u32
     - runtime
     - 璁剧疆 ACL 鍖哄煙瀹氭湡閲嶅搱甯岋紙rehash锛夌殑闂撮殧銆備互姣涓哄崟浣嶏紝鏈€灏忎负
       `3000`銆傚€间负 `0` 琛ㄧず瀹屽叏绂佺敤瀹氭湡宸ヤ綔銆傜涓€娆￠噸鍝堝笇灏嗗湪鍊艰璁剧疆鍚?       绔嬪嵆杩愯銆?
`mlxsw` 椹卞姩鏀寔閫氳繃 `DEVLINK_CMD_RELOAD` 閲嶆柊鍔犺浇

## 鐗堟湰淇℃伅


`mlxsw` 椹卞姩鎶ュ憡浠ヤ笅鐗堟湰

   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `hw.revision`
     - fixed
     - 璇ユ澘鐨勭‖浠朵慨璁㈠彿
   - - `fw.psid`
     - fixed
     - 鍥轰欢 PSID
   - - `fw.version`
     - running
     - 涓変綅鍥轰欢鐗堟湰鍙?
## 绾垮崱杈呭姪璁惧鐗堟湰淇℃伅


`mlxsw` 椹卞姩涓虹嚎鍗¤緟鍔╄澶囨姤鍛婁互涓嬬増鏈?
   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `hw.revision`
     - fixed
     - 璇ョ嚎鍗＄殑纭欢淇鍙?   - - `ini.version`
     - running
     - 宸插姞杞界殑绾垮崱 INI 鐗堟湰
   - - `fw.psid`
     - fixed
     - 绾垮崱璁惧 PSID
   - - `fw.version`
     - running
     - 绾垮崱璁惧鐨勪笁浣嶅浐浠剁増鏈彿

## 椹卞姩鐗瑰畾闄烽槺


   :widths: 5 5 90

   - - 鍚嶇О
     - 绫诲瀷
     - 鎻忚堪
   - - `irif_disabled`
     - `drop`
     - 鎹曡幏璁惧鍐冲畾涓㈠純鐨勬暟鎹寘锛屽洜涓哄畠浠渶瑕佷粠宸茬鐢ㄧ殑璺敱鍣ㄦ帴鍙ｏ紙RIF锛?       璺敱銆傝繖鍙兘鍙戠敓鍦?RIF 鎷嗛櫎鏈熼棿锛屽綋 RIF 鍦ㄨ褰诲簳绉婚櫎鍓嶅厛琚鐢ㄦ椂
   - - `erif_disabled`
     - `drop`
     - 鎹曡幏璁惧鍐冲畾涓㈠純鐨勬暟鎹寘锛屽洜涓哄畠浠渶瑕侀€氳繃宸茬鐢ㄧ殑璺敱鍣ㄦ帴鍙ｏ紙RIF锛?       璺敱銆傝繖鍙兘鍙戠敓鍦?RIF 鎷嗛櫎鏈熼棿锛屽綋 RIF 鍦ㄨ褰诲簳绉婚櫎鍓嶅厛琚鐢ㄦ椂
