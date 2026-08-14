## Infiniband 鐢ㄦ埛绌洪棿鑳藉姏

   User CAPability锛圲CAP锛夋彁渚涘 Infiniband锛圛B锛夎澶囦腑鐗瑰畾鍥轰欢鐗规€х殑缁嗙矑搴?   鎺у埗銆傜浉姣旂幇鏈夌殑 Linux capabilities锛岃繖绉嶆柟寮忔彁渚涗簡鏇寸粏鍖栫殑鑳藉姏锛岃€岀幇鏈夌殑
   Linux capabilities 瀵逛簬鏌愪簺 FW 鐗规€ц€岃█鍙兘杩囦簬瀹芥硾銆?
   姣忎釜鐢ㄦ埛鑳藉姏閮借〃鐜颁负涓€涓瓧绗﹁澶囷紝浠?root 鍙鍐欍€俽oot 杩涚▼鍙互閫氳繃鍏佽
   璁块棶杩欎簺瀛楃璁惧锛堜緥濡備娇鐢?chown锛夋潵鎺堜簣鐢ㄦ埛鐗规畩鏉冮檺銆?
## 鐢ㄦ硶


   UCAP 鍏佽浣跨敤 UCAP 瀛楃璁惧鐨勬枃浠舵弿杩扮鏉ユ帶鍒?IB 璁惧鐨勭壒瀹氱壒鎬с€傜敤鎴峰惎鐢?   IB 璁惧鐗瑰畾鐗规€х殑鏂瑰紡濡備笅锛?
      - root 杩涚▼鎺堜簣鐢ㄦ埛璁块棶浠ｈ〃杩欎簺鑳藉姏鐨?UCAP 鏂囦欢鐨勬潈闄愶紙渚嬪浣跨敤 chown锛夈€?      - 鐢ㄦ埛鎵撳紑 UCAP 鏂囦欢锛岃幏鍙栨枃浠舵弿杩扮銆?      - 鍦ㄦ墦寮€ IB 璁惧鏃讹紝灏?UCAP 鏂囦欢鎻忚堪绗︽暟缁勪綔涓轰竴涓睘鎬у寘鍚繘鏉ャ€?      - ib_uverbs 椹卞姩璇嗗埆杩欎簺 UCAP 鏂囦欢鎻忚堪绗︼紝骞朵负璇?IB 璁惧鍚敤鐩稿簲鐨勮兘鍔涖€?
## 鍒涘缓 UCAP


   瑕佸垱寤烘柊鐨?UCAP锛岄┍鍔ㄥ繀椤婚鍏堝湪 rdma/ib_ucaps.h 鐨?rdma_user_cap 鏋氫妇涓?   瀹氫箟涓€涓被鍨嬨€俇CAP 瀛楃璁惧鐨勫悕绉板簲娣诲姞鍒?drivers/infiniband/core/ucaps.c
   鐨?ucap_names 鏁扮粍涓€傜劧鍚庯紝椹卞姩鍙互閫氳繃璋冪敤甯︽湁 UCAP 绫诲瀷鐨?ib_create_ucap
   API 鏉ュ垱寤?UCAP 瀛楃璁惧銆?
   涓烘瘡涓?UCAP 瀛樺偍涓€涓紩鐢ㄨ鏁帮紝浠ヨ窡韪?UCAP 璁惧鐨勫垱寤轰笌绉婚櫎銆傚鏋滀互鐩稿悓绫诲瀷
   锛堜緥濡傞拡瀵逛袱涓?IB 璁惧锛夊彂鍑哄娆″垱寤鸿皟鐢紝鍒?UCAP 瀛楃璁惧浼氬湪棣栨璋冪敤鏃?   鍒涘缓锛屽悗缁皟鐢ㄩ€掑寮曠敤璁℃暟銆?
   UCAP 瀛楃璁惧鍒涘缓鍦?/dev/infiniband 涓嬶紝鍏舵潈闄愯璁剧疆涓轰粎鍏佽 root 璇诲啓銆?
## 绉婚櫎 UCAP


   姣忔绉婚櫎閮戒細閫掑噺 UCAP 鐨勫紩鐢ㄨ鏁般€傚彧鏈夊綋寮曠敤璁℃暟鍑忓埌 0 鏃讹紝UCAP 瀛楃璁惧
   鎵嶄細浠庢枃浠剁郴缁熶腑绉婚櫎銆?
## /dev 涓?/sys/class 鏂囦欢


```

      /sys/class/infiniband_ucaps

   is created when the first UCAP character device is created.

   The UCAP character device is created under /dev/infiniband.

   For example, if mlx5_ib adds the rdma_user_cap
   RDMA_UCAP_MLX5_CTRL_LOCAL with name "mlx5_perm_ctrl_local", this will
   create the device node::

      /dev/infiniband/mlx5_perm_ctrl_local


```
