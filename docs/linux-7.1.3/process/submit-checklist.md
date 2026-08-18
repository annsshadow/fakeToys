
## Linux 鍐呮牳琛ヤ竵鎻愪氦妫€鏌ユ竻鍗?

濡傛灉寮€鍙戣€呭笇鏈涜嚜宸辩殑鍐呮牳琛ヤ竵鎻愪氦鑳藉琚洿蹇湴鎺ュ彈锛?浠ヤ笅鏄粬浠簲褰撳仛鍒扮殑涓€浜涘熀鏈簨椤广€?
杩欎簺閮借秴鍑轰簡 `Documentation/process/submitting-patches.rst` <submittingpatches>
浠ュ強鍏朵粬鍦版柟鍏充簬鎻愪氦 Linux 鍐呮牳琛ヤ竵鐨勬枃妗ｆ墍鎻愪緵鐨勮姹傘€?
## 瀹℃煡浣犵殑浠ｇ爜


1) 濡傛灉浣犱娇鐢ㄤ簡鏌愪釜鍔熻兘锛坒acility锛夛紝閭ｄ箞璇?`#include` 瀹氫箟/澹版槑
   璇ュ姛鑳芥墍鍦ㄧ殑澶存枃浠躲€備笉瑕佷緷璧栧叾浠栧ご鏂囦欢涓轰綘
   鎷夊叆浣犳墍浣跨敤鐨勫ご鏂囦欢銆?
2) 鎸夌収 `Documentation/process/coding-style.rst` <codingstyle>
   涓殑璇︾粏璇存槑妫€鏌ヨˉ涓佺殑鎬讳綋椋庢牸銆?
3) 鎵€鏈夊唴瀛樺睆闅滐紙渚嬪 `barrier()`銆乣rmb()`銆乣wmb()`锛夐兘闇€瑕佸湪
   婧愪唬鐮佷腑鏈夋敞閲婏紝瑙ｉ噴瀹冧滑姝ｅ湪鍋氫粈涔堜互鍙婁负浠€涔堣繖鏍峰仛鐨勯€昏緫銆?
## 瀹℃煡 Kconfig 鏀瑰姩


1) 浠讳綍鏂板鎴栦慨鏀圭殑 `CONFIG` 閫夐」閮戒笉瑕佸紕涔遍厤缃彍鍗曪紝骞朵笖
   闄ら潪婊¤冻 `Documentation/kbuild/kconfig-language.rst` 涓?   鈥滆彍鍗曞睘鎬э細榛樿鍊尖€濇墍璁板綍鐨勪緥澶栨爣鍑嗭紝鍚﹀垯榛樿搴斾负鍏抽棴锛坥ff锛夈€?
2) 鎵€鏈夋柊澧炵殑 `Kconfig` 閫夐」閮藉簲鏈夊府鍔╂枃鏈紙help text锛夈€?
3) 宸茬粡閽堝鐩稿叧鐨?`Kconfig` 缁勫悎杩涜浜嗕粩缁嗗鏌ャ€傝繖涓€鐐瑰緢闅鹃€氳繃
   娴嬭瘯鍋氬鈥斺€斿湪杩欓噷锛岃剳鍔涙€濊€冿紙brainpower锛夋槸鍊煎緱鐨勩€?
## 鎻愪緵鏂囨。


1) 鍖呭惈 kernel-doc <kernel_doc> 浠ユ枃妗ｅ寲鍏ㄥ眬鍐呮牳 API銆?   锛堥潤鎬佸嚱鏁颁笉瑕佹眰锛屼絾鍦ㄩ偅閲屼篃鍙互銆傦級

2) 鎵€鏈夋柊澧炵殑 `/proc` 鏉＄洰閮藉湪 `Documentation/` 涓嬫湁鏂囨。璇存槑銆?
3) 鎵€鏈夋柊澧炵殑鍐呮牳鍚姩鍙傛暟閮藉湪
   `Documentation/admin-guide/kernel-parameters.rst` 涓湁鏂囨。璇存槑銆?
4) 鎵€鏈夋柊澧炵殑妯″潡鍙傛暟閮界敤 `MODULE_PARM_DESC()` 杩涜鏂囨。璇存槑銆?
5) 鎵€鏈夋柊澧炵殑鐢ㄦ埛绌洪棿鎺ュ彛閮藉湪 `Documentation/ABI/` 涓湁鏂囨。璇存槑銆?   鏈夊叧鏇村淇℃伅锛岃鍙傝 Documentation/admin-guide/abi.rst锛堟垨 `Documentation/ABI/README`锛夈€?   淇敼鐢ㄦ埛绌洪棿鎺ュ彛鐨勮ˉ涓佸簲褰撴妱閫侊紙CC锛夊埌
   linux-api@vger.kernel.org銆?
6) 濡傛灉琛ヤ竵鏂板浜嗕换浣?ioctl锛岄偅涔堜篃瑕佹洿鏂?   `Documentation/userspace-api/ioctl/ioctl-number.rst`銆?
## 鐢ㄥ伐鍏锋鏌ヤ綘鐨勪唬鐮?

1) 鍦ㄦ彁浜や箣鍓嶇敤琛ヤ竵椋庢牸妫€鏌ュ櫒妫€鏌ユ槸鍚﹀瓨鍦ㄧ悙纰庣殑杩濊
   锛坄scripts/checkpatch.pl`锛夈€?   浣犲簲璇ヨ兘澶熶负琛ヤ竵涓畫鐣欑殑鎵€鏈夎繚瑙勭粰鍑哄悎鐞嗚В閲娿€?
2) 鐢?sparse 骞插噣鍦伴€氳繃妫€鏌ャ€?
3) 浣跨敤 `make checkstack` 骞朵慨澶嶅畠鍙戠幇鐨勪换浣曢棶棰樸€?   娉ㄦ剰 `checkstack` 骞朵笉浼氭樉寮忔寚鍑洪棶棰橈紝
   浣嗕换浣曞湪鏍堜笂浣跨敤瓒呰繃 512 瀛楄妭鐨勫嚱鏁伴兘鏄渶瑕佷慨鏀圭殑鍊欓€夊璞°€?
## 鏋勫缓浣犵殑浠ｇ爜


1) 骞插噣鍦版瀯寤猴細

  a) 鍦ㄩ€傜敤鐨勬垨宸蹭慨鏀圭殑 `CONFIG` 閫夐」鍒嗗埆涓?`=y`銆乣=m` 浠ュ強
     `=n` 鏃躲€傛病鏈?`gcc` 璀﹀憡/閿欒锛屾病鏈夐摼鎺ュ櫒璀﹀憡/閿欒銆?
  b) 閫氳繃 `allnoconfig`銆乣allmodconfig`銆?
  c) 鍦ㄤ娇鐢?`O=builddir` 鏃舵瀯寤烘垚鍔熴€?
  d) 浠讳綍 Documentation/ 涓嬬殑鏀瑰姩閮借兘鎴愬姛鏋勫缓锛屼笖涓嶄骇鐢熸柊鐨勮鍛?閿欒銆?     浣跨敤 `make htmldocs` 鎴?`make pdfdocs` 鏉ユ鏌ユ瀯寤哄苟
     淇浠讳綍闂銆?
2) 閫氳繃浣跨敤鏈湴浜ゅ弶缂栬瘧宸ュ叿鎴栨煇涓叾浠栨瀯寤洪泦缇わ紝
   鍦ㄥ绉?CPU 鏋舵瀯涓婃瀯寤恒€?   娉ㄦ剰锛岄拡瀵逛笉鍚屽瓧闀匡紙32 浣嶅拰 64 浣嶏級浠ュ強涓嶅悓瀛楄妭搴?   锛堝ぇ绔拰灏忕锛夌殑鏋舵瀯杩涜娴嬭瘯锛岃兘澶熸湁鏁堝彂鐜扮敱浜庡
   鍙〃绀烘暟閲忚寖鍥淬€佹暟鎹榻愭垨瀛楄妭搴忕瓑鍋氬嚭閿欒鍋囪鑰屽鑷寸殑
   鍚勭鍙Щ妞嶆€ч棶棰樸€?
3) 鏂板鐨勪唬鐮佸凡缁忕敤 `gcc -W`锛堜娇鐢?   `make KCFLAGS=-W`锛夌紪璇戣繃銆傝繖浼氫骇鐢熷ぇ閲忓櫔闊筹紝浣嗘湁鍒╀簬
   鍙戠幇绫讳技鈥渨arning: comparison between signed and unsigned鈥?   杩欐牱鐨?bug銆?
4) 濡傛灉浣犱慨鏀圭殑婧愪唬鐮佷緷璧栨垨浣跨敤浜嗕互涓?`Kconfig` 绗﹀彿鐩稿叧鐨?   浠讳綍鍐呮牳 API 鎴栫壒鎬э紝閭ｄ箞璇风敤鐩稿叧鐨?`Kconfig` 绗﹀彿琚鐢?   鍜?鎴栬涓?`=m`锛堝鏋滆閫夐」鍙敤锛夋潵杩涜澶氭鏋勫缓
   [涓嶉渶瑕佸悓鏃跺叏閮ㄨ缃紝鍙渶瀹冧滑鍚勭闅忔満鐨勭粍鍚圿锛?
   `CONFIG_SMP`銆乣CONFIG_SYSFS`銆乣CONFIG_PROC_FS`銆乣CONFIG_INPUT`銆?   `CONFIG_PCI`銆乣CONFIG_BLOCK`銆乣CONFIG_PM`銆乣CONFIG_MAGIC_SYSRQ`銆?   `CONFIG_NET`銆乣CONFIG_INET=n`锛堝悗鑰呮惌閰?`CONFIG_NET=y`锛夈€?
## 娴嬭瘯浣犵殑浠ｇ爜


1) 宸茬粡鐢?`CONFIG_PREEMPT`銆乣CONFIG_DEBUG_PREEMPT`銆?   `CONFIG_SLUB_DEBUG`銆乣CONFIG_DEBUG_PAGEALLOC`銆乣CONFIG_DEBUG_MUTEXES`銆?   `CONFIG_DEBUG_SPINLOCK`銆乣CONFIG_DEBUG_ATOMIC_SLEEP`銆?   `CONFIG_PROVE_RCU` 浠ュ強 `CONFIG_DEBUG_OBJECTS_RCU_HEAD`
   鍏ㄩ儴鍚屾椂鍚敤杩涜娴嬭瘯銆?
2) 宸茬粡鐢ㄥ拰涓嶇敤 `CONFIG_SMP` 涓?   `CONFIG_PREEMPT` 杩涜浜嗘瀯寤轰笌杩愯鏃舵祴璇曘€?
3) 鎵€鏈変唬鐮佽矾寰勯兘宸插湪鍚敤鎵€鏈?lockdep 鐗规€х殑鎯呭喌涓嬭鎵ц杩囥€?
4) 宸茬粡閫氳繃鑷冲皯娉ㄥ叆 slab 涓庨〉鍒嗛厤
   澶辫触杩涜妫€鏌ャ€傚弬瑙?`Documentation/fault-injection/`銆?   濡傛灉鏂颁唬鐮侀噺寰堝ぇ锛屾坊鍔犲瓙绯荤粺鐗瑰畾鐨勬晠闅滄敞鍏ュ彲鑳芥槸鍚堥€傜殑銆?
5) 宸茬敤 linux-next 鏈€鏂扮殑鏍囩杩涜娴嬭瘯锛屼互纭繚瀹冧粛鑳?   涓庢墍鏈夊叾浠栧凡鎺掗槦鐨勮ˉ涓佷互鍙?VM銆?   VFS 鍜屽叾浠栧瓙绯荤粺涓殑鍚勭鍙樺姩鍗忓悓宸ヤ綔銆?