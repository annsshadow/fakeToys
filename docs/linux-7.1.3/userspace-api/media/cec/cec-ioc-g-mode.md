

######## ioctls CEC_G_MODE 涓?CEC_S_MODE

CEC_G_MODE, CEC_S_MODE - 鑾峰彇鎴栬缃 CEC 閫傞厤鍣ㄧ殑鐙崰浣跨敤

## 姒傝

`int ioctl(int fd, CEC_G_MODE, __u32 *argp)`

`int ioctl(int fd, CEC_S_MODE, __u32 *argp)`

## 鍙傛暟

`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 CEC 妯″紡鐨勬寚閽堛€?
## 鎻忚堪

榛樿鎯呭喌涓嬶紝浠讳綍鏂囦欢鍙ユ焺锛坒ilehandle锛夐兘鍙互浣跨敤 CEC_TRANSMIT锛屼絾涓轰簡闃绘鍚勪釜搴旂敤绋嬪簭浜掔浉骞叉壈锛屽繀椤昏兘澶熻幏鍙栧 CEC 閫傞厤鍣ㄧ殑鐙崰璁块棶銆傝 ioctl 灏嗘枃浠跺彞鏌勮缃负 initiator锛堝彂璧疯€咃級鍜?鎴?follower锛堣窡闅忚€咃級妯″紡锛屽叿浣撳彇鍐充簬鎵€閫夋嫨鐨勬ā寮忥紝骞朵笖鍙互鏄嫭鍗犵殑銆俰nitiator 鏄敤浜庡彂璧锋秷鎭殑鏂囦欢鍙ユ焺锛屽嵆瀹冨懡浠ゅ叾浠?CEC 璁惧銆俧ollower 鏄帴鏀跺彂寰€ CEC 閫傞厤鍣ㄧ殑娑堟伅骞跺鐞嗗畠浠殑鏂囦欢鍙ユ焺銆傚悓涓€涓枃浠跺彞鏌勫彲浠ユ棦鏄?initiator 鍙堟槸 follower锛屼篃鍙互鐢变袱涓笉鍚岀殑鏂囦欢鍙ユ焺鍒嗗埆鎷呬换杩欎袱涓鑹层€?
褰撴帴鏀跺埌涓€鏉?CEC 娑堟伅鏃讹紝CEC 妗嗘灦浼氬喅瀹氬浣曞鐞嗗畠銆傚鏋滆繖鏉℃秷鎭槸瀵规棭鍏堝彂鍑虹殑娑堟伅鐨勫簲绛旓紝閭ｄ箞璇ュ簲绛斾細琚€佸洖姝ｅ湪绛夊緟瀹冪殑鏂囦欢鍙ユ焺銆傛澶栵紝CEC 妗嗘灦涔熶細澶勭悊瀹冦€?
濡傛灉杩欐潯娑堟伅涓嶆槸搴旂瓟锛岄偅涔?CEC 妗嗘灦浼氬厛澶勭悊瀹冦€傚鏋滄病鏈?follower锛岄偅涔堣娑堟伅浼氳鐩存帴涓㈠純锛屽苟涓斿鏋滄鏋舵棤娉曞鐞嗗畠锛屽垯浼氬悜 initiator 鍙戝洖涓€涓?feature abort锛堝姛鑳芥嫆缁濓級銆傚鏋滄湁 follower锛屽垯璇ユ秷鎭細琚紶閫掔粰 follower锛宖ollower 灏嗕娇鐢?ioctl CEC_RECEIVE <CEC_RECEIVE> 灏嗚繖鏉℃柊娑堟伅鍑洪槦銆傛鏋舵湡鏈?follower 鍋氬嚭姝ｇ‘鐨勫喅绛栥€?
闄ら潪 follower 鍙︽湁瑕佹眰锛屽惁鍒?CEC 妗嗘灦浼氬鐞嗘牳蹇冩秷鎭€俧ollower 鍙互鍚敤 passthrough锛堥€忎紶锛夋ā寮忋€傚湪杩欑鎯呭喌涓嬶紝CEC 妗嗘灦浼氬皢澶у鏁版牳蹇冩秷鎭洿鎺ヤ紶閫掕繃鍘昏€屼笉澶勭悊瀹冧滑锛岃€?follower 蹇呴』鑷瀹炵幇杩欎簺娑堟伅銆傛湁浜涙秷鎭槸鏍稿績濮嬬粓閮戒細澶勭悊鐨勶紝鏃犺閫忎紶妯″紡濡備綍銆傝瑙?cec-core-processing銆?
濡傛灉娌℃湁 initiator锛岄偅涔堜换浣?CEC 鏂囦欢鍙ユ焺閮藉彲浠ヤ娇鐢?ioctl CEC_TRANSMIT <CEC_TRANSMIT>銆傚鏋滃瓨鍦ㄤ竴涓嫭鍗犵殑 initiator锛岄偅涔堝彧鏈夎 initiator 鍙互璋冪敤 CEC_TRANSMIT銆傚綋鐒讹紝follower 濮嬬粓鍙互璋冪敤 ioctl CEC_TRANSMIT <CEC_TRANSMIT>銆?
鍙敤鐨?initiator 妯″紡鏈夛細



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-MODE-NO-INITIATOR`:

      - `CEC_MODE_NO_INITIATOR`
      - 0x0
      - 杩欎笉鏄竴涓?initiator锛屽嵆瀹冧笉鑳藉彂閫?CEC 娑堟伅锛屼篃涓嶈兘瀵?CEC 閫傞厤鍣ㄥ仛浠讳綍鍏朵粬鏇存敼銆?    - .. _`CEC-MODE-INITIATOR`:

      - `CEC_MODE_INITIATOR`
      - 0x1
      - 杩欐槸涓€涓?initiator锛堣澶囨墦寮€鏃剁殑榛樿鍊硷級锛屽畠鍙互鍙戦€?CEC 娑堟伅骞跺 CEC 閫傞厤鍣ㄨ繘琛屾洿鏀癸紝闄ら潪瀛樺湪涓€涓嫭鍗犵殑 initiator銆?    - .. _`CEC-MODE-EXCL-INITIATOR`:

      - `CEC_MODE_EXCL_INITIATOR`
      - 0x2
      - 杩欐槸涓€涓嫭鍗犵殑 initiator锛岃鏂囦欢鎻忚堪绗︽槸鍞竴鑳藉鍙戦€?CEC 娑堟伅骞跺 CEC 閫傞厤鍣ㄨ繘琛屾洿鏀圭殑鍙ユ焺銆傚鏋滃凡缁忔湁鍏朵粬浜烘垚涓虹嫭鍗犵殑 initiator锛岄偅涔堝皾璇曟垚涓虹嫭鍗?initiator 灏嗚繑鍥?`EBUSY` 閿欒鐮併€?
鍙敤鐨?follower 妯″紡鏈夛細




    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 16

    - .. _`CEC-MODE-NO-FOLLOWER`:

      - `CEC_MODE_NO_FOLLOWER`
      - 0x00
      - 杩欎笉鏄竴涓?follower锛堣澶囨墦寮€鏃剁殑榛樿鍊硷級銆?    - .. _`CEC-MODE-FOLLOWER`:

      - `CEC_MODE_FOLLOWER`
      - 0x10
      - 杩欐槸涓€涓?follower锛屽畠浼氭帴鏀?CEC 娑堟伅锛岄櫎闈炲瓨鍦ㄤ竴涓嫭鍗犵殑 follower銆傚鏋滄湭璁剧疆 CEC_CAP_TRANSMIT <CEC-CAP-TRANSMIT>锛屾垨鑰呮寚瀹氫簡 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR>锛屽垯涓嶈兘鎴愪负 follower锛岃繖绉嶆儏鍐典笅浼氳繑鍥?`EINVAL` 閿欒鐮併€?    - .. _`CEC-MODE-EXCL-FOLLOWER`:

      - `CEC_MODE_EXCL_FOLLOWER`
      - 0x20
      - 杩欐槸涓€涓嫭鍗犵殑 follower锛屽彧鏈夎鏂囦欢鎻忚堪绗︿細鎺ユ敹 CEC 娑堟伅杩涜澶勭悊銆傚鏋滃凡缁忔湁鍏朵粬浜烘垚涓虹嫭鍗犵殑 follower锛岄偅涔堝皾璇曟垚涓虹嫭鍗?follower 灏嗚繑鍥?`EBUSY` 閿欒鐮併€傚鏋滄湭璁剧疆 CEC_CAP_TRANSMIT <CEC-CAP-TRANSMIT>锛屾垨鑰呮寚瀹氫簡 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR>锛屽垯涓嶈兘鎴愪负 follower锛岃繖绉嶆儏鍐典笅浼氳繑鍥?`EINVAL` 閿欒鐮併€?    - .. _`CEC-MODE-EXCL-FOLLOWER-PASSTHRU`:

      - `CEC_MODE_EXCL_FOLLOWER_PASSTHRU`
      - 0x30
      - 杩欐槸涓€涓嫭鍗犵殑 follower锛屽彧鏈夎鏂囦欢鎻忚堪绗︿細鎺ユ敹 CEC 娑堟伅杩涜澶勭悊銆傛澶栵紝瀹冧細灏?CEC 璁惧缃簬 passthrough 妯″紡锛屼粠鑰屽厑璁哥嫭鍗?follower 鏉ュ鐞嗗ぇ澶氭暟鏍稿績娑堟伅锛岃€屼笉蹇呬緷璧?CEC 妗嗘灦銆傚鏋滃凡缁忔湁鍏朵粬浜烘垚涓虹嫭鍗?follower锛岄偅涔堝皾璇曟垚涓虹嫭鍗?follower 灏嗚繑鍥?`EBUSY` 閿欒鐮併€傚鏋滄湭璁剧疆 CEC_CAP_TRANSMIT <CEC-CAP-TRANSMIT>锛屾垨鑰呮寚瀹氫簡 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR>锛屽垯涓嶈兘鎴愪负 follower锛岃繖绉嶆儏鍐典笅浼氳繑鍥?`EINVAL` 閿欒鐮併€?    - .. _`CEC-MODE-MONITOR-PIN`:

      - `CEC_MODE_MONITOR_PIN`
      - 0xd0
      - 灏嗘枃浠舵弿杩扮缃簬寮曡剼鐩戣妯″紡銆傚彧鑳戒笌 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR> 缁勫悎浣跨敤锛屽惁鍒欎細杩斿洖 `EINVAL` 閿欒鐮併€傝妯″紡瑕佹眰璁剧疆浜?CEC_CAP_MONITOR_PIN <CEC-CAP-MONITOR-PIN> 鑳藉姏锛屽惁鍒欎細杩斿洖 `EINVAL` 閿欒鐮併€傚湪寮曡剼鐩戣妯″紡涓嬶紝璇ユ枃浠舵弿杩扮鍙互鎺ユ敹 `CEC_EVENT_PIN_CEC_LOW` 鍜?`CEC_EVENT_PIN_CEC_HIGH` 浜嬩欢锛屼互瑙傚療搴曞眰鐨?CEC 寮曡剼鐘舵€佸彉鍖栥€傝繖瀵逛簬璋冭瘯闈炲父鏈夌敤銆傝妯″紡浠呭湪杩涚▼鎷ユ湁 `CAP_NET_ADMIN` 鑳藉姏鏃舵墠琚厑璁搞€傚鏋滄湭璁剧疆璇ヨ兘鍔涳紝鍒欒繑鍥?`EPERM` 閿欒鐮併€?    - .. _`CEC-MODE-MONITOR`:

      - `CEC_MODE_MONITOR`
      - 0xe0
      - 灏嗘枃浠舵弿杩扮缃簬鐩戣妯″紡銆傚彧鑳戒笌 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR> 缁勫悎浣跨敤锛屽惁鍒欎細杩斿洖 `EINVAL` 閿欒鐮併€傚湪鐩戣妯″紡涓嬶紝璇?CEC 璁惧鍙戦€佺殑鎵€鏈夋秷鎭互鍙婂畠鎺ユ敹鐨勬墍鏈夋秷鎭紙鍖呮嫭骞挎挱娑堟伅浠ュ強鍙戝線鍏舵煇涓€昏緫鍦板潃鐨勫畾鍚戞秷鎭級閮戒細琚姤鍛娿€傝繖瀵逛簬璋冭瘯闈炲父鏈夌敤銆傝妯″紡浠呭湪杩涚▼鎷ユ湁 `CAP_NET_ADMIN` 鑳藉姏鏃舵墠琚厑璁搞€傚鏋滄湭璁剧疆璇ヨ兘鍔涳紝鍒欒繑鍥?`EPERM` 閿欒鐮併€?    - .. _`CEC-MODE-MONITOR-ALL`:

      - `CEC_MODE_MONITOR_ALL`
      - 0xf0
      - 灏嗘枃浠舵弿杩扮缃簬鈥滅洃瑙嗗叏閮ㄢ€濇ā寮忋€傚彧鑳戒笌 CEC_MODE_NO_INITIATOR <CEC-MODE-NO-INITIATOR> 缁勫悎浣跨敤锛屽惁鍒欎細杩斿洖 `EINVAL` 閿欒鐮併€傚湪鈥滅洃瑙嗗叏閮ㄢ€濇ā寮忎笅锛岃 CEC 璁惧鍙戦€佺殑鎵€鏈夋秷鎭互鍙婂畠鎺ユ敹鐨勬墍鏈夋秷鎭紝鍖呮嫭鍙戠粰鍏朵粬 CEC 璁惧鐨勫畾鍚戞秷鎭紝閮戒細琚姤鍛娿€傝繖瀵逛簬璋冭瘯闈炲父鏈夌敤锛屼絾骞堕潪鎵€鏈夎澶囬兘鏀寔姝ゆā寮忋€傝妯″紡瑕佹眰璁剧疆浜?CEC_CAP_MONITOR_ALL <CEC-CAP-MONITOR-ALL> 鑳藉姏锛屽惁鍒欎細杩斿洖 `EINVAL` 閿欒鐮併€傝妯″紡浠呭湪杩涚▼鎷ユ湁 `CAP_NET_ADMIN` 鑳藉姏鏃舵墠琚厑璁搞€傚鏋滄湭璁剧疆璇ヨ兘鍔涳紝鍒欒繑鍥?`EPERM` 閿欒鐮併€?
鏍稿績娑堟伅澶勭悊缁嗚妭锛?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 8

    - .. _`CEC-MSG-GET-CEC-VERSION`:

      - `CEC_MSG_GET_CEC_VERSION`
      - 鏍稿績浼氳繑鍥為€氳繃 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 璁剧疆鐨?CEC 鐗堟湰锛岄€忎紶妯″紡闄ゅ銆傚湪閫忎紶妯″紡涓嬶紝鏍稿績涓嶅仛浠讳綍澶勭悊锛岃娑堟伅蹇呴』鐢?follower 鏉ュ鐞嗐€?    - .. _`CEC-MSG-GIVE-DEVICE-VENDOR-ID`:

      - `CEC_MSG_GIVE_DEVICE_VENDOR_ID`
      - 鏍稿績浼氳繑鍥為€氳繃 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 璁剧疆鐨勫巶鍟?ID锛岄€忎紶妯″紡闄ゅ銆傚湪閫忎紶妯″紡涓嬶紝鏍稿績涓嶅仛浠讳綍澶勭悊锛岃娑堟伅蹇呴』鐢?follower 鏉ュ鐞嗐€?    - .. _`CEC-MSG-ABORT`:

      - `CEC_MSG_ABORT`
      - 鎸夌収瑙勮寖锛屾牳蹇冧細杩斿洖涓€鏉?reason 涓衡€淔eature Refused鈥濓紙鍔熻兘琚嫆缁濓級鐨?Feature Abort 娑堟伅锛岄€忎紶妯″紡闄ゅ銆傚湪閫忎紶妯″紡涓嬶紝鏍稿績涓嶅仛浠讳綍澶勭悊锛岃娑堟伅蹇呴』鐢?follower 鏉ュ鐞嗐€?    - .. _`CEC-MSG-GIVE-PHYSICAL-ADDR`:

      - `CEC_MSG_GIVE_PHYSICAL_ADDR`
      - 鏍稿績浼氭姤鍛婂綋鍓嶇殑鐗╃悊鍦板潃锛岄€忎紶妯″紡闄ゅ銆傚湪閫忎紶妯″紡涓嬶紝鏍稿績涓嶅仛浠讳綍澶勭悊锛岃娑堟伅蹇呴』鐢?follower 鏉ュ鐞嗐€?    - .. _`CEC-MSG-GIVE-OSD-NAME`:

      - `CEC_MSG_GIVE_OSD_NAME`
      - 鏍稿績浼氭姤鍛婇€氳繃 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 璁剧疆鐨勫綋鍓?OSD 鍚嶇О锛岄€忎紶妯″紡闄ゅ銆傚湪閫忎紶妯″紡涓嬶紝鏍稿績涓嶅仛浠讳綍澶勭悊锛岃娑堟伅蹇呴』鐢?follower 鏉ュ鐞嗐€?    - .. _`CEC-MSG-GIVE-FEATURES`:

      - `CEC_MSG_GIVE_FEATURES`
      - 濡傛灉 CEC 鐗堟湰浣庝簬 2.0锛屾牳蹇冧笉鍋氫换浣曞鐞嗭紱鍚﹀垯瀹冧細鎶ュ憡閫氳繃 ioctl CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 璁剧疆鐨勫綋鍓嶇壒鎬э紝閫忎紶妯″紡闄ゅ銆傚湪閫忎紶妯″紡涓嬶紝鏍稿績涓嶅仛浠讳綍澶勭悊锛堝浜庝换浣?CEC 鐗堟湰锛夛紝璇ユ秷鎭繀椤荤敱 follower 鏉ュ鐞嗐€?    - .. _`CEC-MSG-USER-CONTROL-PRESSED`:

      - `CEC_MSG_USER_CONTROL_PRESSED`
      - 濡傛灉璁剧疆浜?CEC_CAP_RC <CEC-CAP-RC>锛屽苟涓旇缃簡 CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU <CEC-LOG-ADDRS-FL-ALLOW-RC-PASSTHRU>锛屽垯鐢熸垚涓€涓仴鎺ф寜閿寜涓嬩簨浠躲€傝娑堟伅濮嬬粓浼氳浼犻€掔粰 follower(s)銆?    - .. _`CEC-MSG-USER-CONTROL-RELEASED`:

      - `CEC_MSG_USER_CONTROL_RELEASED`
      - 濡傛灉璁剧疆浜?CEC_CAP_RC <CEC-CAP-RC>锛屽苟涓旇缃簡 CEC_LOG_ADDRS_FL_ALLOW_RC_PASSTHRU <CEC-LOG-ADDRS-FL-ALLOW-RC-PASSTHRU>锛屽垯鐢熸垚涓€涓仴鎺ф寜閿噴鏀句簨浠躲€傝娑堟伅濮嬬粓浼氳浼犻€掔粰 follower(s)銆?    - .. _`CEC-MSG-REPORT-PHYSICAL-ADDR`:

      - `CEC_MSG_REPORT_PHYSICAL_ADDR`
      - CEC 妗嗘灦浼氳褰曟墍鎶ュ憡鐨勭墿鐞嗗湴鍧€锛岀劧鍚庣洿鎺ュ皢娑堟伅浼犻€掔粰 follower(s)銆?
## 杩斿洖鍊?
鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
ioctl CEC_S_MODE <CEC_S_MODE> 鍙互杩斿洖浠ヤ笅閿欒鐮侊細

EINVAL
    鎵€璇锋眰鐨勬ā寮忔棤鏁堛€?
EPERM
    璇锋眰浜嗙洃瑙嗘ā寮忥紝浣嗚繘绋嬫湭鎷ユ湁 `CAP_NET_ADMIN` 鑳藉姏銆?
EBUSY
    宸茬粡鏈夊叾浠栬繘绋嬫垚涓虹嫭鍗犵殑 follower 鎴?initiator銆?