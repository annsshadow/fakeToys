
## Reset controller API锛堝浣嶆帶鍒跺櫒 API锛?

## 绠€浠嬶紙Introduction锛?

澶嶄綅鎺у埗鍣紙reset controller锛夋槸鎺у埗澶氫釜澶栬澶嶄綅淇″彿鐨勪腑澶崟鍏冦€?澶嶄綅鎺у埗鍣?API 鍒嗕负涓ら儴鍒嗭細
`consumer driver interface <#consumer-driver-interface>`__锛坄API 鍙傝€?<#reset-consumer-api>`__锛夛紝鐢ㄤ簬璁╁璁鹃┍鍔ㄨ姹傛帶鍒跺叾澶嶄綅杈撳叆淇″彿锛?浠ュ強 `reset controller driver interface
<#reset-controller-driver-interface>`__锛坄API 鍙傝€?<#reset-controller-driver-api>`__锛夛紝渚涘浣嶆帶鍒跺櫒璁惧鐨勯┍鍔ㄧ敤鏉ユ敞鍐屽叾澶嶄綅
鎺у埗锛屼粠鑰屾彁渚涚粰浣跨敤鑰咃紙consumer锛変娇鐢ㄣ€?
铏界劧鏌愪簺澶嶄綅鎺у埗鍣ㄧ‖浠跺崟鍏冧篃瀹炵幇浜嗙郴缁熼噸鍚姛鑳斤紝浣嗛噸鍚鐞嗙▼搴忎笉鍦?澶嶄綅鎺у埗鍣?API 鐨勮寖鐣翠箣鍐呫€?
### 鏈琛紙Glossary锛?

澶嶄綅鎺у埗鍣?API 瀵逛互涓嬫湳璇湁鐗瑰畾鍚箟锛?
Reset line锛堝浣嶇嚎锛?
    浠庡浣嶆帶鍒跺櫒纭欢鍗曞厓杩炴帴鍒板璁炬ā鍧楃殑銆佹壙杞藉浣嶄俊鍙风殑鐗╃悊澶嶄綅绾裤€?
Reset control锛堝浣嶆帶鍒讹級

    鍐冲畾涓€鏉℃垨澶氭潯澶嶄綅绾跨姸鎬佺殑鎺у埗鏂规硶銆傛渶甯歌鐨勫舰寮忔槸鍦ㄥ浣嶆帶鍒跺櫒
    瀵勫瓨鍣ㄧ┖闂翠腑鐨勪竴涓崟鐙綅锛屽畠瑕佷箞鍏佽鐩存帴鎺у埗澶嶄綅绾跨殑鐗╃悊鐘舵€侊紝
    瑕佷箞鏄嚜娓呴浂鐨勶紝鍙敤浜庡湪澶嶄綅绾夸笂瑙﹀彂涓€涓瀹氱殑鑴夊啿銆?    鍦ㄦ洿涓哄鏉傜殑澶嶄綅鎺у埗涓紝涓€娆¤Е鍙戝姩浣滃彲浠ュ惎鍔ㄥ鏉″浣嶇嚎涓婁竴缁?    缁忚繃绮剧‘璁℃椂鐨勮剦鍐插簭鍒椼€?
Reset controller锛堝浣嶆帶鍒跺櫒锛?
    涓€涓‖浠舵ā鍧楋紝鎻愪緵鑻ュ共澶嶄綅鎺у埗浠ユ帶鍒惰嫢骞插浣嶇嚎銆?
Reset consumer锛堝浣嶄娇鐢ㄨ€咃級

    鐢卞浣嶇嚎涓婄殑淇″彿缃叆澶嶄綅鐘舵€佺殑澶栬妯″潡鎴栧閮?IC銆?
## Consumer driver interface锛堜娇鐢ㄨ€呴┍鍔ㄦ帴鍙ｏ級


璇ユ帴鍙ｆ彁渚涚殑 API 绫讳技浜庡唴鏍告椂閽熸鏋讹紙clock framework锛夈€備娇鐢ㄨ€呴┍鍔?浣跨敤 get 鍜?put 鎿嶄綔鏉ヨ幏鍙栧拰閲婃斁澶嶄綅鎺у埗銆?杩樻彁渚涗簡鐢ㄤ簬鏂█锛坅ssert锛夊拰瑙ｉ櫎鏂█锛坉eassert锛夋墍鎺у埗鐨勫浣嶇嚎銆佽Е鍙?澶嶄綅鑴夊啿浠ュ強鏌ヨ澶嶄綅绾跨姸鎬佺殑鍑芥暟銆?
鍦ㄨ姹傚浣嶆帶鍒舵椂锛屼娇鐢ㄨ€呭彲浠ヤ娇鐢ㄥ叾澶嶄綅杈撳叆鐨勭鍙峰悕绉帮紝鐢辨牳蹇冨皢鍏舵槧灏勫埌
鏌愪釜宸叉湁澶嶄綅鎺у埗鍣ㄨ澶囦笂鐨勫疄闄呭浣嶆帶鍒躲€?
褰撳浣嶆帶鍒跺櫒妗嗘灦鏈浣跨敤鏃讹紝浼氭彁渚涗竴涓 API 鐨勬々锛坰tub锛夌増鏈紝浠ュ敖閲?鍑忓皯瀵?ifdef 鐨勪娇鐢ㄩ渶姹傘€?
### Shared and exclusive resets锛堝叡浜笌鐙崰澶嶄綅锛?

澶嶄綅鎺у埗鍣?API 鎻愪緵寮曠敤璁℃暟鐨勮В闄?鏂█锛屾垨鑰呯洿鎺ャ€佺嫭鍗犵殑鎺у埗銆?鍏变韩锛坰hared锛変笌鐙崰锛坋xclusive锛夊浣嶆帶鍒剁殑鍖哄垎鍦ㄨ姹傚浣嶆帶鍒舵椂鍋氬嚭锛?鏃㈠彲浠ラ€氳繃 devm_reset_control_get_shared()锛屼篃鍙互閫氳繃
devm_reset_control_get_exclusive()銆?杩欎竴閫夋嫨鍐冲畾浜嗕娇鐢ㄨ澶嶄綅鎺у埗鏃?API 璋冪敤鐨勮涓恒€?
鍏变韩澶嶄綅鐨勮涓虹被浼间簬鍐呮牳鏃堕挓妗嗘灦涓殑鏃堕挓銆?瀹冧滑鎻愪緵寮曠敤璁℃暟鐨勮В闄ゆ柇瑷€锛氬彧鏈夌涓€娆¤В闄ゆ柇瑷€锛堝皢瑙ｉ櫎鏂█寮曠敤璁℃暟
澧炲姞鍒?1锛夊拰鏈€鍚庝竴娆℃柇瑷€锛堝皢瑙ｉ櫎鏂█寮曠敤璁℃暟鍑忓洖鍒?0锛夋墠浼氬澶嶄綅绾夸骇鐢?瀹為檯鐨勭墿鐞嗗奖鍝嶃€?
鑰岀嫭鍗犲浣嶅垯淇濊瘉鐩存帴鎺у埗銆備篃灏辨槸璇达紝涓€娆℃柇瑷€浼氳澶嶄綅绾跨珛鍗宠鏂█锛?涓€娆¤В闄ゆ柇瑷€浼氳澶嶄綅绾跨珛鍗宠瑙ｉ櫎鏂█銆?
### Assertion and deassertion锛堟柇瑷€涓庤В闄ゆ柇瑷€锛?

浣跨敤鑰呴┍鍔ㄤ娇鐢?reset_control_assert() 鍜?reset_control_deassert() 鍑芥暟
鏉ユ柇瑷€鍜岃В闄ゆ柇瑷€澶嶄綅绾裤€傚浜庡叡浜浣嶆帶鍒讹紝瀵硅繖涓や釜鍑芥暟鐨勮皟鐢ㄥ繀椤讳繚鎸佸钩琛°€?
娉ㄦ剰锛岀敱浜庡涓娇鐢ㄨ€呭彲鑳戒細浣跨敤鍚屼竴涓叡浜浣嶆帶鍒讹紝鍥犳鏃犳硶淇濊瘉鍦ㄥ叡浜?澶嶄綅鎺у埗涓婅皟鐢?reset_control_assert() 灏变竴瀹氫細浣垮浣嶇嚎琚柇瑷€銆?浣跨敤鍏变韩澶嶄綅鎺у埗鐨勬秷璐硅€呴┍鍔ㄥ簲褰撳亣瀹氬浣嶇嚎鍙兘濮嬬粓淇濇寔瑙ｉ櫎鏂█鐘舵€併€?璇?API 浠呬繚璇侊細鍙鏈変换浣曚娇鐢ㄨ€呰姹傚皢鍏惰В闄ゆ柇瑷€锛屽浣嶇嚎灏变笉浼氳鏂█銆?
### Triggering锛堣Е鍙戯級


浣跨敤鑰呴┍鍔ㄤ娇鐢?reset_control_reset() 鍦ㄨ嚜瑙ｉ櫎鏂█鐨勫浣嶆帶鍒朵笂瑙﹀彂涓€涓?澶嶄綅鑴夊啿銆備竴鑸€岃█锛岃繖浜涘浣嶄笉鑳藉湪澶氫釜浣跨敤鑰呬箣闂村叡浜紝鍥犱负浠讳綍涓€涓娇鐢ㄨ€?椹卞姩璇锋眰鑴夊啿閮戒細澶嶄綅鎵€鏈夌浉杩炵殑澶栬銆?
澶嶄綅鎺у埗鍣?API 鍏佽灏嗚嚜瑙ｉ櫎鏂█鐨勫浣嶆帶鍒朵綔涓哄叡浜姹傦紝浣嗗浜庢绫绘帶鍒讹紝
鍙湁绗竴娆¤Е鍙戣姹傛墠浼氬湪澶嶄綅绾夸笂鐪熸鍙戝嚭涓€涓剦鍐层€?鍦ㄨ澶嶄綅鎺у埗鐨勬墍鏈変娇鐢ㄨ€呴兘璋冪敤 reset_control_rearm() 涔嬪墠锛屽璇ュ嚱鏁扮殑
鍚庣画璋冪敤鍧囨棤鏁堛€傚浜庡叡浜浣嶆帶鍒讹紝瀵硅繖涓や釜鍑芥暟鐨勮皟鐢ㄥ繀椤讳繚鎸佸钩琛°€?杩欎娇寰楅偅浜涘彧闇€瑕佸湪椹卞姩鎺㈡祴鎴栨仮澶嶄箣鍓嶇殑浠绘剰鏃跺埢杩涜涓€娆″垵濮嬪浣嶇殑璁惧
鍙互鍏变韩涓€鏉¤剦鍐插紡澶嶄綅绾裤€?
### Querying锛堟煡璇級


鍙湁閮ㄥ垎澶嶄綅鎺у埗鍣ㄦ敮鎸侀€氳繃 reset_control_status() 鏌ヨ澶嶄綅绾跨殑褰撳墠鐘舵€併€?鑻ユ敮鎸侊紝褰撶粰瀹氱殑澶嶄綅绾垮浜庢柇瑷€鐘舵€佹椂锛岃鍑芥暟杩斿洖姝ｇ殑闈為浂鍊笺€?reset_control_status() 鍑芥暟涓嶆帴鍙?`reset control array <#reset-control-arrays>`__
鍙ユ焺浣滀负鍏惰緭鍏ュ弬鏁般€?
### Optional resets锛堝彲閫夊浣嶏級


澶栬甯稿父鍦ㄦ煇浜涘钩鍙颁笂闇€瑕佸浣嶇嚎锛岃€屽湪鍙︿竴浜涘钩鍙颁笂鍒欎笉闇€瑕併€?涓烘锛屽彲浠ヤ娇鐢?devm_reset_control_get_optional_exclusive() 鎴?devm_reset_control_get_optional_shared() 灏嗗浣嶆帶鍒朵綔涓哄彲閫夋潵璇锋眰銆?褰撴墍璇锋眰鐨勫浣嶆帶鍒跺湪璁惧鏍戜腑鏈寚瀹氭椂锛岃繖浜涘嚱鏁拌繑鍥炰竴涓?NULL 鎸囬拡鑰屼笉鏄?閿欒銆傚皢 NULL 鎸囬拡浼犵粰澶嶄綅鎺у埗鍑芥暟浼氫娇瀹冧滑瀹夐潤鍦拌繑鍥炶€屼笉浜х敓閿欒銆?
### Reset control arrays锛堝浣嶆帶鍒舵暟缁勶級


鏌愪簺椹卞姩闇€瑕佷互浠绘剰椤哄簭鏂█涓€缁勫浣嶇嚎銆俤evm_reset_control_array_get()
杩斿洖涓€涓笉閫忔槑鐨勫浣嶆帶鍒跺彞鏌勶紝鍙敤浜庝竴娆℃€ф柇瑷€銆佽В闄ゆ柇瑷€鎴栬Е鍙戞墍鏈夋寚瀹氱殑
澶嶄綅鎺у埗銆傚浣嶆帶鍒?API 涓嶄繚璇佸叾涓悇涓帶鍒惰澶勭悊鐨勯『搴忋€?
## Reset controller driver interface锛堝浣嶆帶鍒跺櫒椹卞姩鎺ュ彛锛?

澶嶄綅鎺у埗鍣ㄦā鍧楃殑椹卞姩鎻愪緵鏂█鎴栬В闄ゆ柇瑷€澶嶄綅淇″彿銆佸湪澶嶄綅绾夸笂瑙﹀彂澶嶄綅鑴夊啿鎴?鏌ヨ鍏跺綋鍓嶇姸鎬佹墍闇€鐨勫姛鑳姐€傛墍鏈夊嚱鏁伴兘鏄彲閫夌殑銆?
### Initialization锛堝垵濮嬪寲锛?

椹卞姩鍦ㄦ帰娴嬶紙probe锛夊嚱鏁颁腑濉厖涓€涓?struct `reset_controller_dev` 缁撴瀯浣擄紝
骞堕€氳繃 reset_controller_register() 娉ㄥ唽瀹冦€傚疄闄呯殑鍔熻兘閫氳繃 struct
`reset_control_ops` 涓殑鍥炶皟鍑芥暟瀹炵幇銆?
## API reference锛圓PI 鍙傝€冿級


澶嶄綅鎺у埗鍣?API 鍦ㄦ鍒嗕负涓ら儴鍒嗚繘琛岃鏄庯細
`reset consumer API <#reset-consumer-api>`__ 鍜?`reset controller
driver API <#reset-controller-driver-api>`__銆?
### Reset consumer API锛堜娇鐢ㄨ€?API锛?

澶嶄綅浣跨敤鑰呭彲浠ヤ娇鐢ㄤ竴涓笉閫忔槑鐨勫浣嶆帶鍒跺彞鏌勬潵鎺у埗澶嶄綅绾匡紝璇ュ彞鏌勫彲鐢?devm_reset_control_get_exclusive() 鎴?devm_reset_control_get_shared() 鑾峰緱銆?寰楀埌澶嶄綅鎺у埗鍚庯紝浣跨敤鑰呭彲浠ヨ皟鐢?reset_control_assert() 鍜?reset_control_deassert()锛屼娇鐢?reset_control_reset() 瑙﹀彂澶嶄綅鑴夊啿锛屾垨浣跨敤
reset_control_status() 鏌ヨ澶嶄綅绾跨姸鎬併€?
   :internal:

   :functions: reset_control_reset
               reset_control_assert
               reset_control_deassert
               reset_control_status
               reset_control_acquire
               reset_control_release
               reset_control_rearm
               reset_control_put
               of_reset_control_get_count
               devm_reset_control_array_get
               reset_control_get_count

### Reset controller driver API锛堟帶鍒跺櫒椹卞姩 API锛?

澶嶄綅鎺у埗鍣ㄩ┍鍔ㄥ簲褰撳湪 static 甯搁噺缁撴瀯浣?`reset_control_ops` 涓疄鐜版墍闇€鐨?鍑芥暟锛屽垎閰嶅苟濉厖涓€涓?struct `reset_controller_dev`锛屽苟閫氳繃
devm_reset_controller_register() 娉ㄥ唽瀹冦€?
   :internal:

   :functions: of_reset_simple_xlate
               reset_controller_register
               reset_controller_unregister
               devm_reset_controller_register
