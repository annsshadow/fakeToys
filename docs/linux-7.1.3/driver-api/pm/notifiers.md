
## 鎸傝捣/浼戠湢閫氱煡鍣紙Notifiers锛?


:Copyright: |copy| 2016 Intel Corporation

:Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>


鏌愪簺瀛愮郴缁熸垨椹卞姩鍙兘甯屾湜鍦ㄤ紤鐪?鎸傝捣涔嬪墠鎴栨仮澶?鍞ら啋涔嬪悗鎵ц涓€浜涙搷浣滐紝浣嗗畠浠姹傜郴缁熷畬鍏ㄥ彲鐢紝鍥犳椹卞姩鐨勪笌瀛愮郴缁熺殑 `->suspend()` 鍜?`->resume()` 鐢氳嚦 `->prepare()` 鍜?`->complete()` 鍥炶皟閮戒笉閫傚悎姝ょ洰鐨勩€?

渚嬪锛岃澶囬┍鍔ㄥ彲鑳藉笇鏈涘湪鍞ら啋/鎭㈠涔嬪悗鍚戝畠浠殑璁惧涓婁紶鍥轰欢锛屼絾瀹冧滑鏃犳硶浠?`->resume()` 鎴?`->complete()` 鍥炶皟渚嬬▼涓皟鐢?`request_firmware()`锛堟鏃剁敤鎴锋€佽繘绋嬪凡琚喕缁擄級銆傝В鍐虫柟妗堝彲鑳芥槸鍦ㄨ繘绋嬭鍐荤粨涔嬪墠灏嗗浐浠跺姞杞藉埌鍐呭瓨涓紝骞跺湪 `->resume()` 渚嬬▼涓粠閭ｉ噷涓婁紶銆備负姝ゅ彲浠ヤ娇鐢ㄦ寕璧?浼戠湢閫氱煡鍣ㄣ€?

鏈夋绫婚渶姹傜殑瀛愮郴缁熸垨椹卞姩鍙互娉ㄥ唽鎸傝捣閫氱煡鍣紝瀹冧滑灏嗗湪浠ヤ笅浜嬩欢鏃惰 PM 鏍稿績璋冪敤锛?

`PM_HIBERNATION_PREPARE`
	绯荤粺灏嗚浼戠湢锛屼换鍔″皢绔嬪嵆琚喕缁撱€傝繖涓庝笅闈㈢殑 `PM_SUSPEND_PREPARE` 涓嶅悓锛屽洜涓哄湪杩欑鎯呭喌涓嬶紝閫氱煡鍣ㄤ笌閽堝鈥滃喕缁撯€濊浆鎹㈢殑 PM 鍥炶皟璋冪敤涔嬮棿浼氬畬鎴愰澶栫殑宸ヤ綔銆?

`PM_POST_HIBERNATION`
	绯荤粺鍐呭瓨鐘舵€佸凡浠庝紤鐪犻暅鍍忔仮澶嶏紝鎴栧湪浼戠湢鏈熼棿鍙戠敓浜嗛敊璇€傝澶囨仮澶嶅洖璋冨凡鎵ц锛屼换鍔″凡瑙ｅ喕銆?

`PM_RESTORE_PREPARE`
	绯荤粺灏嗚鎭㈠涓€涓紤鐪犻暅鍍忋€傚鏋滀竴鍒囬『鍒╋紝鎭㈠鍚庣殑闀滃儚鍐呮牳灏嗗彂鍑?`PM_POST_HIBERNATION` 閫氱煡銆?

`PM_POST_RESTORE`
	浠庝紤鐪犳仮澶嶆湡闂村彂鐢熶簡閿欒銆傝澶囨仮澶嶅洖璋冨凡鎵ц锛屼换鍔″凡瑙ｅ喕銆?

`PM_SUSPEND_PREPARE`
	绯荤粺姝ｅ湪鍑嗗鎸傝捣銆?

`PM_POST_SUSPEND`
	绯荤粺鍒氬垰鍞ら啋锛屾垨鍦ㄦ寕璧锋湡闂村彂鐢熶簡閿欒銆傝澶囧敜閱掑洖璋冨凡鎵ц锛屼换鍔″凡瑙ｅ喕銆?

閫氬父鍋囧畾锛岄€氱煡鍣ㄤ负 `PM_HIBERNATION_PREPARE` 鎵€鍋氱殑浠讳綍浜嬫儏锛岄兘搴斿湪 `PM_POST_HIBERNATION` 涓挙閿€銆傜被浼煎湴锛屼负 `PM_SUSPEND_PREPARE` 鎵ц鐨勬搷浣滃簲鍦?`PM_POST_SUSPEND` 涓弽鍚戞墽琛屻€?

姝ゅ锛屽鏋滄煇涓€氱煡鍣ㄥ湪 `PM_HIBERNATION_PREPARE` 鎴?`PM_SUSPEND_PREPARE` 浜嬩欢涓婂け璐ワ紝閭ｄ箞宸茬粡涓鸿浜嬩欢鎴愬姛杩囩殑閫氱煡鍣ㄥ皢鍒嗗埆琚互 `PM_POST_HIBERNATION` 鎴?`PM_POST_SUSPEND` 璋冪敤銆?

浼戠湢涓庢寕璧烽€氱煡鍣ㄥ湪鎸佹湁 :c:`pm_mutex` 鐨勬儏鍐典笅琚皟鐢ㄣ€傚畠浠互閫氬父鐨勬柟寮忓畾涔夛紝浣嗗畠浠殑鏈€鍚庝竴涓弬鏁版棤鎰忎箟锛堝缁堜负 NULL锛夈€?

瑕佹敞鍐屽拰/鎴栨敞閿€鎸傝捣閫氱煡鍣紝鍒嗗埆浣跨敤 `register_pm_notifier()` 涓?`unregister_pm_notifier()`锛堜簩鑰呴兘瀹氫箟鍦?`include/linux/suspend.h` 涓級銆傚鏋滀綘涓嶉渶瑕佹敞閿€閫氱煡鍣紝涔熷彲浠ヤ娇鐢?`include/linux/suspend.h` 涓畾涔夌殑 `pm_notifier()` 瀹忋€?
