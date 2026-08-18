## Linux Devlink 鏂囨。


devlink 鏄竴涓?API锛岀敤浜庢毚闇蹭笌浠讳綍璁惧绫绘病鏈夌洿鎺ュ叧绯荤殑璁惧淇℃伅涓庤祫婧愶紝渚嬪鑺墖绾?浜ゆ崲鏈?ASIC 绾х殑閰嶇疆銆?
### 閿?

椹卞姩闈㈠悜鐨?API 鐩墠姝ｅ湪杩囨浮鍒板厑璁告洿鏄惧紡鐨勫姞閿併€傞┍鍔ㄥ彲浠ヤ娇鐢ㄧ幇鏈夌殑 `devlink_*` 涓€缁?API锛屾垨
浠?`devl_*` 涓哄墠缂€鐨勬柊 API銆傝緝鏃х殑 API 鍦?devlink 鏍稿績涓鐞嗘墍鏈夊姞閿侊紝浣嗕笉鍏佽鍦ㄤ富 devlink 瀵硅薄
鑷韩娉ㄥ唽鍚庢敞鍐屽ぇ澶氭暟瀛愬璞°€傝緝鏂扮殑 `devl_*` API 鍋囧畾 devlink 瀹炰緥閿佸凡缁忚鎸佹湁銆傞┍鍔ㄥ彲浠ラ€氳繃璋冪敤
`devl_lock()` 鑾峰彇瀹炰緥閿併€傚湪鎵€鏈?devlink netlink 鍛戒护鐨勫洖璋冧腑涔熶細鎸佹湁瀹冦€?
榧撳姳椹卞姩涓鸿嚜宸辩殑闇€姹備娇鐢?devlink 瀹炰緥閿併€?
椹卞姩鍦ㄥ悓鏃惰幏鍙?devlink 瀹炰緥閿佷笌鑾峰彇 RTNL 閿佹椂闇€瑕佽皑鎱庛€傞渶瑕佸厛鑾峰彇 devlink 瀹炰緥閿侊紝鍙湁鍦ㄦ涔嬪悗鎵嶈兘
鑾峰彇 RTNL 閿併€?
### 宓屽瀹炰緥


鏌愪簺瀵硅薄锛屽绾垮崱锛坙inecard锛夋垨绔彛鍔熻兘锛坧ort function锛夛紝鍏朵笅鍙兘浼氬垱寤哄彟涓€涓?devlink 瀹炰緥銆傚湪閭ｇ
鎯呭喌涓嬶紝椹卞姩搴旂‘淇濋伒瀹堜互涓嬭鍒欙細

 - 搴斾繚鎸佸姞閿侀『搴忋€傚鏋滈┍鍔ㄩ渶瑕佸悓鏃惰幏鍙栧祵濂楀疄渚嬩笌鐖舵瘝瀹炰緥鐨勫疄渚嬮攣锛屽簲鍏堣幏鍙栫埗姣嶅疄渚嬬殑 devlink 瀹炰緥閿侊紝
   鐒跺悗鎵嶈兘鑾峰彇宓屽瀹炰緥鐨勫疄渚嬮攣銆? - 椹卞姩搴斾娇鐢ㄥ璞＄壒瀹氱殑杈呭姪鍑芥暟鏉ュ缓绔嬪祵濂楀叧绯伙細

   - `devl_nested_devlink_set()` - 璋冪敤浠ュ缓绔?devlink -> 宓屽 devlink 鍏崇郴锛堝彲鐢ㄤ簬澶氫釜宓屽瀹炰緥锛夈€?   - `devl_port_fn_devlink_set()` - 璋冪敤浠ュ缓绔嬬鍙ｅ姛鑳?-> 宓屽 devlink 鍏崇郴銆?   - `devlink_linecard_nested_dl_set()` - 璋冪敤浠ュ缓绔嬬嚎鍗?-> 宓屽 devlink 鍏崇郴銆?
宓屽 devlink 淇℃伅閫氳繃 devlink netlink 鐨勫璞＄壒瀹氬睘鎬ф毚闇茬粰鐢ㄦ埛绌洪棿銆?
### 鎺ュ彛鏂囨。


浠ヤ笅椤甸潰涓€鑸湴鎻忚堪浜嗛€氳繃 devlink 鍙敤鐨勫悇绉嶆帴鍙ｃ€?
- [devlink-dpipe](devlink-dpipe)
- [devlink-eswitch-attr](devlink-eswitch-attr)
- [devlink-flash](devlink-flash)
- [devlink-health](devlink-health)
- [devlink-info](devlink-info)
- [devlink-linecard](devlink-linecard)
- [devlink-params](devlink-params)
- [devlink-port](devlink-port)
- [devlink-region](devlink-region)
- [devlink-reload](devlink-reload)
- [devlink-resource](devlink-resource)
- [devlink-selftests](devlink-selftests)
- [devlink-trap](devlink-trap)
- [devlink-shared](devlink-shared)

### 椹卞姩涓撴湁鏂囨。


姣忎釜瀹炵幇浜?`devlink` 鐨勯┍鍔ㄩ兘搴旇褰曞畠鏀寔鐨勫弬鏁般€佷俊鎭増鏈互鍙婂叾浠栫壒鎬с€?
- [am65-nuss-cpsw-switch](am65-nuss-cpsw-switch)
- [bnxt](bnxt)
- [etas_es58x](etas_es58x)
- [hns3](hns3)
- [i40e](i40e)
- [ice](ice)
- [ionic](ionic)
- [iosm](iosm)
- [ixgbe](ixgbe)
- [kvaser_pciefd](kvaser_pciefd)
- [kvaser_usb](kvaser_usb)
- [mlx4](mlx4)
- [mlx5](mlx5)
- [mlxsw](mlxsw)
- [mv88e6xxx](mv88e6xxx)
- [netdevsim](netdevsim)
- [nfp](nfp)
- [octeontx2](octeontx2)
- [prestera](prestera)
- [qed](qed)
- [sfc](sfc)
- [stmmac](stmmac)
- [ti-cpsw-switch](ti-cpsw-switch)
- [zl3073x](zl3073x)
