## PM 鏈嶅姟璐ㄩ噺锛圦oS锛夋帴鍙?

璇ユ帴鍙ｆ彁渚涗簡涓€涓唴鏍告ā寮忓拰鐢ㄦ埛妯″紡鎺ュ彛锛岀敤浜庨┍鍔ㄣ€佸瓙绯荤粺浠ュ強鐢ㄦ埛绌洪棿搴旂敤绋嬪簭灏?鏌愪釜鍙傛暟娉ㄥ唽鎬ц兘鏈熸湜銆?
鏈変袱绉嶄笉鍚岀殑 PM QoS 妗嗘灦鍙敤锛? - CPU 寤惰繜 QoS銆? - 姣忚澶?PM QoS 妗嗘灦鎻愪緵浜嗙敤浜庣鐞嗘瘡璁惧寤惰繜绾︽潫鍜?PM QoS 鏍囧織鐨?API銆?
PM QoS 妗嗘灦涓娇鐢ㄧ殑寤惰繜鍗曚綅鏄井绉掞紙usec锛夈€?

## 1. PM QoS 妗嗘灦


涓€涓?CPU 寤惰繜 QoS 璇锋眰鐨勫叏灞€鍒楄〃涓庤仛鍚堬紙鏈夋晥锛夌洰鏍囧€间竴璧疯缁存姢銆傝仛鍚堢洰鏍囧€间細闅忕潃
璇锋眰鍒楄〃鎴栧叾鍏冪礌鐨勫彉鏇磋€屾洿鏂般€傚浜?CPU 寤惰繜 QoS锛岃仛鍚堢洰鏍囧€煎氨鏄垪琛ㄤ腑鍚勫厓绱犳墍鎸佹湁
璇锋眰鍊肩殑鏈€灏忓€笺€?
娉ㄦ剰锛氳仛鍚堢洰鏍囧€艰瀹炵幇涓轰竴涓師瀛愬彉閲忥紝鍥犳璇诲彇鑱氬悎鍊间笉闇€瑕佷换浣曞姞閿佹満鍒躲€?
鍦ㄥ唴鏍哥┖闂达紝璇ユ帴鍙ｇ殑浣跨敤寰堢畝鍗曪細

void cpu_latency_qos_add_request(handle, target_value):
  灏嗕竴涓厓绱犱互鐩爣鍊兼彃鍏ュ埌 CPU 寤惰繜 QoS 鍒楄〃涓€?  涓€鏃﹁鍒楄〃鍙戠敓鍙樺寲锛屽氨浼氶噸鏂拌绠楁柊鐩爣锛屽苟涓斾粎褰撶洰鏍囧€煎彂鐢熸敼鍙樻椂鎵嶄細璋冪敤浠讳綍宸?  娉ㄥ唽鐨?notifier銆?  PM QoS 鐨勫鎴风闇€瑕佷繚瀛樿繑鍥炵殑澶勭悊鍙ユ焺锛屼互渚垮湪鍏朵粬 PM QoS API 鍑芥暟涓悗缁娇鐢ㄣ€?
void cpu_latency_qos_update_request(handle, new_target_value):
  浼氫娇鐢ㄨ鏂扮洰鏍囧€兼洿鏂扮敱鍙ユ焺鎸囧悜鐨勫垪琛ㄥ厓绱狅紝骞堕噸鏂拌绠楁柊鐨勮仛鍚堢洰鏍囷紝濡傛灉鐩爣鍙戠敓
  鍙樺寲鍒欒皟鐢ㄩ€氱煡鏍戙€?
void cpu_latency_qos_remove_request(handle):
  浼氱Щ闄よ鍏冪礌銆傜Щ闄や箣鍚庯紝濡傛灉绉婚櫎璇ヨ姹傚鑷寸洰鏍囧彂鐢熷彉鍖栵紝瀹冧細鏇存柊鑱氬悎鐩爣骞惰皟鐢?  閫氱煡鏍戙€?
int cpu_latency_qos_limit():
  杩斿洖 CPU 寤惰繜 QoS 鐨勮仛鍚堝€笺€?
int cpu_latency_qos_request_active(handle):
  杩斿洖璇ヨ姹傛槸鍚︿粛鐒跺浜庢椿鍔ㄧ姸鎬侊紝鍗冲畠鏄惁灏氭湭浠?CPU 寤惰繜 QoS 鍒楄〃涓Щ闄ゃ€?

浠庣敤鎴风┖闂达細

璇ュ熀纭€璁炬柦鏆撮湶涓や釜鐙珛鐨勮澶囪妭鐐癸紝/dev/cpu_dma_latency 鐢ㄤ簬 CPU 寤惰繜 QoS锛?/dev/cpu_wakeup_latency 鐢ㄤ簬 CPU 绯荤粺鍞ら啋寤惰繜 QoS銆?
鍙湁杩涚▼鍙互娉ㄥ唽涓€涓?PM QoS 璇锋眰銆備负浜嗘敮鎸佽繘绋嬬殑鑷姩娓呯悊锛岃鎺ュ彛瑕佹眰杩涚▼鎸夊涓?鏂瑰紡娉ㄥ唽鍏跺弬鏁拌姹傘€?
瑕佹敞鍐?CPU 寤惰繜 QoS 鐨勯粯璁?PM QoS 鐩爣锛岃繘绋嬪繀椤绘墦寮€ /dev/cpu_dma_latency銆傝娉ㄥ唽
涓€涓?CPU 绯荤粺鍞ら啋 QoS 闄愬埗锛岃繘绋嬪繀椤绘墦寮€ /dev/cpu_wakeup_latency銆?
鍙璇ヨ澶囪妭鐐逛繚鎸佹墦寮€锛岃杩涚▼灏卞湪杩欎釜鍙傛暟涓婃嫢鏈変竴涓凡娉ㄥ唽鐨勮姹傘€?
瑕佹洿鏀规墍璇锋眰鐨勭洰鏍囧€硷紝杩涚▼闇€瑕佸悜鎵撳紑鐨勮澶囪妭鐐瑰啓鍏ヤ竴涓?s32 鍊笺€傛垨鑰咃紝瀹冨彲浠ヤ娇鐢?10 涓瓧绗﹂暱鐨勬牸寮忥紙渚嬪 "0x12345678"锛夊啓鍏ヨ鍊肩殑鍗佸叚杩涘埗瀛楃涓层€?
瑕佺Щ闄ら拡瀵规煇涓洰鏍囧€肩殑鐢ㄦ埛妯″紡璇锋眰锛屽彧闇€鍏抽棴璇ヨ澶囪妭鐐广€?

## 2. 姣忚澶?PM QoS 寤惰繜涓庢爣蹇楁鏋?

瀵逛簬姣忎釜璁惧锛屾湁涓変釜 PM QoS 璇锋眰鍒楄〃銆傚叾涓袱涓笌鎭㈠寤惰繜锛坮esume latency锛夊拰
娲诲姩鐘舵€佸欢杩熷蹇嶅害锛坅ctive state latency tolerance锛屽崟浣嶄负寰锛夌殑鑱氬悎鐩爣涓€璧疯
缁存姢锛岀涓変釜鐢ㄤ簬 PM QoS 鏍囧織銆傝繖浜涘€间細闅忕潃璇锋眰鍒楄〃鐨勫彉鍖栬€屾洿鏂般€?
鎭㈠寤惰繜鍜屾椿鍔ㄧ姸鎬佸欢杩熷蹇嶅害鐨勭洰鏍囧€硷紝灏辨槸鍙傛暟鍒楄〃鍏冪礌鎵€鎸佹湁璇锋眰鍊肩殑鏈€灏忓€笺€侾M QoS
鏍囧織鐨勮仛鍚堝€兼槸鎵€鏈夊垪琛ㄥ厓绱犲€肩殑鑱氶泦锛堟寜浣?OR锛夈€傜洰鍓嶅畾涔変簡涓€涓澶?PM QoS 鏍囧織锛?PM_QOS_FLAG_NO_POWER_OFF銆?
娉ㄦ剰锛氳仛鍚堢洰鏍囧€肩殑瀹炵幇鏂瑰紡浣垮緱璇诲彇鑱氬悎鍊间笉闇€瑕佷换浣曞姞閿佹満鍒躲€?

鍦ㄥ唴鏍告ā寮忥紝璇ユ帴鍙ｇ殑浣跨敤濡備笅锛?
int dev_pm_qos_add_request(device, handle, type, value):
  灏嗕互鐩爣鍊兼妸涓€涓厓绱犳彃鍏ュ埌鎵€鏍囪瘑璁惧鐨勫垪琛ㄤ腑銆備竴鏃﹁鍒楄〃鍙戠敓鍙樺寲锛屽氨浼氶噸鏂拌绠?  鏂扮洰鏍囷紝骞朵笖浠呭綋鐩爣鍊煎彂鐢熸敼鍙樻椂鎵嶄細璋冪敤浠讳綍宸叉敞鍐岀殑 notifier銆俤ev_pm_qos 鐨?  瀹㈡埛绔渶瑕佷繚瀛樿鍙ユ焺锛屼互渚垮湪鍏朵粬 dev_pm_qos API 鍑芥暟涓悗缁娇鐢ㄣ€?
int dev_pm_qos_update_request(handle, new_value):
  浼氫娇鐢ㄨ鏂扮洰鏍囧€兼洿鏂扮敱鍙ユ焺鎸囧悜鐨勫垪琛ㄥ厓绱狅紝骞堕噸鏂拌绠楁柊鐨勮仛鍚堢洰鏍囷紝濡傛灉鐩爣鍙戠敓
  鍙樺寲鍒欒皟鐢ㄩ€氱煡鏍戙€?
int dev_pm_qos_remove_request(handle):
  浼氱Щ闄よ鍏冪礌銆傜Щ闄や箣鍚庯紝濡傛灉绉婚櫎璇ヨ姹傚鑷寸洰鏍囧彂鐢熷彉鍖栵紝瀹冧細鏇存柊鑱氬悎鐩爣骞惰皟鐢?  閫氱煡鏍戙€?
s32 dev_pm_qos_read_value(device, type):
  杩斿洖缁欏畾璁惧绾︽潫鍒楄〃鐨勮仛鍚堝€笺€?
enum pm_qos_flags_status dev_pm_qos_flags(device, mask)
  鏍规嵁缁欏畾鏍囧織鎺╃爜妫€鏌ョ粰瀹氳澶囩殑 PM QoS 鏍囧織銆傝繑鍥炲€肩殑鍚箟濡備笅锛?
	PM_QOS_FLAGS_ALL:
		鎺╃爜涓殑鎵€鏈夋爣蹇楅兘宸茶缃?	PM_QOS_FLAGS_SOME:
		鎺╃爜涓殑鏌愪簺鏍囧織宸茶缃?	PM_QOS_FLAGS_NONE:
		鎺╃爜涓殑鏍囧織鍧囨湭璁剧疆
	PM_QOS_FLAGS_UNDEFINED:
		璇ヨ澶囩殑 PM QoS 缁撴瀯灏氭湭鍒濆鍖栵紝鎴栬姹傚垪琛ㄤ负绌恒€?
int dev_pm_qos_add_ancestor_request(dev, handle, type, value)
  涓虹粰瀹氳澶囩殑绗竴涓洿鎺ョ鍏堟坊鍔犱竴涓?PM QoS 璇锋眰锛岃绁栧厛鐨?power.ignore_children
  鏍囧織鏈缃紙瀵逛簬 DEV_PM_QOS_RESUME_LATENCY 璇锋眰锛夛紝鎴栧叾
  power.set_latency_tolerance 鍥炶皟鎸囬拡涓嶄负 NULL锛堝浜?  DEV_PM_QOS_LATENCY_TOLERANCE 璇锋眰锛夈€?
int dev_pm_qos_expose_latency_limit(device, value)
  鍚戣澶囩殑鎭㈠寤惰繜绾︽潫 PM QoS 鍒楄〃娣诲姞涓€涓姹傦紝骞跺湪璁惧 power 鐩綍涓嬪垱寤?sysfs
  灞炴€?pm_qos_resume_latency_us锛屽厑璁哥敤鎴风┖闂存搷浣滆璇锋眰銆?
void dev_pm_qos_hide_latency_limit(device)
  浠庤澶囩殑鎭㈠寤惰繜绾︽潫 PM QoS 鍒楄〃涓Щ闄ょ敱 dev_pm_qos_expose_latency_limit() 娣诲姞鐨?  璇锋眰锛屽苟浠庤澶?power 鐩綍涓Щ闄?sysfs 灞炴€?pm_qos_resume_latency_us銆?
int dev_pm_qos_expose_flags(device, value)
  鍚戣澶囩殑鏍囧織 PM QoS 鍒楄〃娣诲姞涓€涓姹傦紝骞跺湪璁惧 power 鐩綍涓嬪垱寤?sysfs 灞炴€?  pm_qos_no_power_off锛屽厑璁哥敤鎴风┖闂存洿鏀?PM_QOS_FLAG_NO_POWER_OFF 鏍囧織鐨勫€笺€?
void dev_pm_qos_hide_flags(device)
  浠庤澶囩殑鏍囧織 PM QoS 鍒楄〃涓Щ闄ょ敱 dev_pm_qos_expose_flags() 娣诲姞鐨勮姹傦紝骞朵粠璁惧
  power 鐩綍涓Щ闄?sysfs 灞炴€?pm_qos_no_power_off銆?
閫氱煡鏈哄埗锛?
姣忚澶?PM QoS 妗嗘灦鏈変竴涓瘡璁惧鐨勯€氱煡鏍戙€?
int dev_pm_qos_add_notifier(device, notifier, type):
  涓鸿澶囨坊鍔犱竴涓拡瀵圭壒瀹氳姹傜被鍨嬬殑閫氱煡鍥炶皟鍑芥暟銆?
  褰撹澶囩害鏉熷垪琛ㄧ殑鑱氬悎鍊煎彂鐢熷彉鍖栨椂浼氳皟鐢ㄨ鍥炶皟銆?
int dev_pm_qos_remove_notifier(device, notifier, type):
  绉婚櫎璁惧鐨勯€氱煡鍥炶皟鍑芥暟銆?

##### 娲诲姩鐘舵€佸欢杩熷蹇嶅害


璇ヨ澶?PM QoS 绫诲瀷鐢ㄤ簬鏀寔閭ｄ簺纭欢鍙互鍔ㄦ€佸垏鎹㈠埌鑺傝兘杩愯妯″紡鐨勭郴缁熴€傚湪杩欑被绯荤粺涓紝
濡傛灉纭欢鎵€閫夋嫨鐨勮繍琛屾ā寮忎互杩囧害婵€杩涚殑鏂瑰紡鑺傜渷鑳借€楋紝鍙兘浼氫娇杞欢鍙鐨勫欢杩熻繃澶э紝瀵艰嚧
鍏堕敊杩囨煇浜涘崗璁姹傛垨鐩爣甯х巼銆侀噰鏍风巼绛夈€?
濡傛灉杞欢鍙互浣跨敤缁欏畾璁惧鐨勬煇涓欢杩熷蹇嶅害鎺у埗鏈哄埗锛屽垯搴斿綋濉厖璇ヨ澶?dev_pm_info
缁撴瀯涓殑 .set_latency_tolerance 鍥炶皟銆傚畠鎵€鎸囧悜鐨勪緥绋嬪簲褰撳疄鐜板皢鏈夋晥闇€姹傚€间紶閫掔粰纭欢
鎵€闇€鐨勪换浣曟搷浣溿€?
姣忓綋璁惧鐨勬湁鏁堝欢杩熷蹇嶅害鍙戠敓鍙樺寲鏃讹紝鍏?.set_latency_tolerance() 鍥炶皟灏变細琚墽琛岋紝
骞跺皢鏈夋晥鍊间紶閫掔粰瀹冦€傚鏋滆鍊间负璐熸暟锛屾剰鍛崇潃璇ヨ澶囩殑寤惰繜瀹瑰繊搴﹂渶姹傚垪琛ㄤ负绌猴紝鍒欐湡鏈?璇ュ洖璋冨湪鍙鏃跺皢搴曞眰纭欢寤惰繜瀹瑰繊搴︽帶鍒舵満鍒跺垏鎹㈠埌鑷富锛坅utonomous锛夋ā寮忋€傚弽涔嬶紝濡傛灉
璇ュ€间负 PM_QOS_LATENCY_ANY锛屽苟涓旂‖浠舵敮鎸佷竴绉嶇壒娈婄殑"鏃犻渶姹?璁剧疆锛屽垯鏈熸湜璇ュ洖璋冧娇鐢?瀹冦€傝繖鏍疯蒋浠跺彲浠ラ槻姝㈢‖浠跺湪鍝嶅簲鐢垫簮鐘舵€佸彉鍖栵紙渚嬪浠?D3cold 杞崲鍒?D0 鏈熼棿锛夋椂鑷姩
鏇存柊璁惧鐨勫欢杩熷蹇嶅害锛岃€岃繖閫氬父鏄湪鑷富寤惰繜瀹瑰繊搴︽帶鍒舵ā寮忎笅瀹屾垚鐨勩€?
濡傛灉璁惧瀛樺湪 .set_latency_tolerance()锛屽垯璁惧鐨?power 鐩綍涓細鍑虹幇 sysfs 灞炴€?pm_qos_latency_tolerance_us銆傜劧鍚庯紝鐢ㄦ埛绌洪棿鍙互浣跨敤璇ュ睘鎬ф潵鎸囧畾鍏跺璁惧鐨勫欢杩?瀹瑰繊搴﹂渶姹傦紙濡傛灉鏈夛級銆傚悜鍏跺啓鍏?"any" 琛ㄧず"鏃犻渶姹傦紝浣嗕笉瑕佽纭欢鎺у埗寤惰繜瀹瑰繊搴?锛?鍚戝叾鍐欏叆 "auto" 鍒欏厑璁稿湪娌℃湁鏉ヨ嚜鍐呮牳渚х殑鍏朵粬闇€姹傛椂锛屽皢纭欢鍒囨崲鍒拌嚜涓绘ā寮忋€?
鍐呮牳浠ｇ爜鍙互浣跨敤涓婅堪鍑芥暟锛岄厤鍚?DEV_PM_QOS_LATENCY_TOLERANCE 璁惧 PM QoS 绫诲瀷锛屾潵
涓鸿澶囨坊鍔犮€佺Щ闄ゅ拰鏇存柊寤惰繜瀹瑰繊搴﹂渶姹傘€?