## NIC SR-IOV API


寮虹儓寤鸿鐜颁唬 NIC 鑱氱劍浜庡疄鐜?`switchdev` 妯″瀷锛堝弬瑙?switchdev锛夛紝浠ラ厤缃?SR-IOV 鍔熻兘鐨勮浆鍙戜笌瀹夊叏鎬с€?

## 浼犵粺 API


鏃х殑 SR-IOV API 鍦?`rtnetlink` Netlink 鏃忎腑瀹炵幇锛屼綔涓?`RTM_GETLINK` 鍜?`RTM_SETLINK` 鍛戒护鐨勪竴閮ㄥ垎銆傚湪椹卞姩渚э紝瀹冪敱鑻ュ共 `ndo_set_vf_**` 鍜?`ndo_get_vf_**` 鍥炶皟缁勬垚銆?

鐢变簬浼犵粺 API 涓庡崗璁爤鍏朵綑閮ㄥ垎鐨勯泦鎴愪笉浣筹紝璇?API 琚涓哄喕缁撶姸鎬侊紱涓嶄細鎺ュ彈浠讳綍鏂板姛鑳芥垨鎵╁睍銆傛柊鐨勯┍鍔ㄤ笉搴斿疄鐜伴偅浜涗笉甯歌鐨勫洖璋冿紱鍗充互涓嬪洖璋冨湪闄愬埗涔嬪锛堜笉寰椾娇鐢級锛?

 - `ndo_get_vf_port`
 - `ndo_set_vf_port`
 - `ndo_set_vf_rss_query_en`
