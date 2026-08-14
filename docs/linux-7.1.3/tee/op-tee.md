## OP-TEE锛堝紑鏀惧彲绉绘鍙俊鎵ц鐜锛孫pen Portable Trusted Execution Environment锛?

OP-TEE 椹卞姩澶勭悊鍩轰簬 OP-TEE [^1^] 鐨?TEE銆傜洰鍓嶄粎鏀寔鍩轰簬 ARM TrustZone 鐨?OP-TEE 鏂规銆?
涓?OP-TEE 閫氫俊鐨勬渶浣庡眰鏋勫缓鍦?ARM SMC 璋冪敤绾﹀畾锛圫MCCC锛塠^2^] 涔嬩笂锛屽畠鏄?OP-TEE 鐨?SMC 鎺ュ彛 [^3^] 鐨勫熀纭€锛岃鎺ュ彛鐢遍┍鍔ㄥ湪鍐呴儴浣跨敤銆傚湪姝や箣涓婂彔鏀剧殑鏄?OP-TEE 娑堟伅鍗忚 [^4^]銆?
OP-TEE SMC 鎺ュ彛鎻愪緵 SMCCC 鎵€闇€鐨勫熀鏈姛鑳戒互鍙?OP-TEE 鐗规湁鐨勪竴浜涢檮鍔犲姛鑳姐€傛渶鏈夋剰鎬濈殑鍔熻兘鏄細

- OPTEE_SMC_FUNCID_CALLS_UID锛圫MCCC 鐨勪竴閮ㄥ垎锛夎繑鍥炵増鏈俊鎭紝闅忓悗鐢?TEE_IOC_VERSION 杩斿洖

- OPTEE_SMC_CALL_GET_OS_UUID 杩斿洖鐗瑰畾鐨?OP-TEE 瀹炵幇锛岀敤浜庡尯鍒嗭紝渚嬪锛孴rustZone OP-TEE 涓庤繍琛屽湪鐙珛瀹夊叏鍗忓鐞嗗櫒涓婄殑 OP-TEE銆?
- OPTEE_SMC_CALL_WITH_ARG 椹卞姩 OP-TEE 娑堟伅鍗忚

- OPTEE_SMC_GET_SHM_CONFIG 璁╅┍鍔ㄥ拰 OP-TEE 灏?Linux 涓?OP-TEE 涔嬮棿鐢ㄤ簬鍏变韩鍐呭瓨鐨勫唴瀛樿寖鍥磋揪鎴愪竴鑷淬€?
GlobalPlatform TEE Client API [^5^] 瀹炵幇鍦ㄩ€氱敤 TEE API 涔嬩笂銆?
涓嶅悓缁勪欢涔嬮棿鍏崇郴鐨勭ず鎰忓浘锛?```
      User space                  Kernel                   Secure world
      ~~~~~~~~~~                  ~~~~~~                   ~~~~~~~~~~~~
   +--------+                                             +-------------+
   | Client |                                             | Trusted     |
   +--------+                                             | Application |
      /\                                                  +-------------+
      || +----------+                                           /\
      || |tee-      |                                           ||
      || |supplicant|                                           \/
      || +----------+                                     +-------------+
      \/      /\                                          | TEE Internal|
   +-------+  ||                                          | API         |
   + TEE   |  ||            +--------+--------+           +-------------+
   | Client|  ||            | TEE    | OP-TEE |           | OP-TEE      |
   | API   |  \/            | subsys | driver |           | Trusted OS  |
   +-------+----------------+----+-------+----+-----------+-------------+
   |      Generic TEE API        |       |     OP-TEE MSG               |
   |      IOCTL (TEE_IOC_*)      |       |     SMCCC (OPTEE_SMC_CALL_*) |
   +-----------------------------+       +------------------------------+
```
RPC锛堣繙绋嬭繃绋嬭皟鐢紝Remote Procedure Call锛夋槸鏉ヨ嚜瀹夊叏涓栫晫瀵瑰唴鏍搁┍鍔ㄦ垨 tee-supplicant 鐨勮姹傘€備竴涓?RPC 鐢?OPTEE_SMC_CALL_WITH_ARG 杩斿洖鐨勪竴缁勭壒娈婅寖鍥寸殑 SMCCC 杩斿洖鍊兼爣璇嗐€傛棬鍦ㄥ彂缁欏唴鏍哥殑 RPC 娑堟伅鐢卞唴鏍搁┍鍔ㄥ鐞嗐€傚叾浠?RPC 娑堟伅灏嗚杞彂缁?tee-supplicant锛岄┍鍔ㄤ笉鍐嶈繘涓€姝ュ弬涓庯紝闄ら潪鍒囨崲鍏变韩鍐呭瓨缂撳啿鍖虹殑琛ㄧず銆?
### OP-TEE 璁惧鏋氫妇锛圤P-TEE device enumeration锛?

OP-TEE 鎻愪緵浜嗕竴涓吉鍙俊搴旂敤绋嬪簭锛歞rivers/tee/optee/device.c锛屼互鏀寔璁惧鏋氫妇銆傛崲鍙ヨ瘽璇达紝OP-TEE 椹卞姩璋冪敤璇ュ簲鐢ㄧ▼搴忔潵妫€绱㈠彲浣滀负璁惧娉ㄥ唽鍒?TEE 鎬荤嚎涓婄殑鍙俊搴旂敤绋嬪簭鍒楄〃銆?
### OP-TEE 閫氱煡锛圤P-TEE notifications锛?

瀹夊叏涓栫晫鍙互浣跨敤涓ょ被閫氱煡锛屼娇鏅€氫笘鐣岀煡鏅撴煇涓簨浠躲€?
1. 閫氳繃 `OPTEE_RPC_CMD_NOTIFICATION` 閰嶅悎 `OPTEE_RPC_NOTIFICATION_SEND` 鍙傛暟浼犻€掔殑鍚屾閫氱煡銆?2. 閫氳繃闈炲畨鍏ㄧ殑杈规部瑙﹀彂涓柇涓庨潪瀹夊叏涓柇澶勭悊绋嬪簭涓殑蹇€熻皟鐢ㄧ粍鍚堜紶閫掔殑寮傛閫氱煡銆?
鍚屾閫氱煡鍙楅檺浜庝緷璧?RPC 鏉ユ姇閫掞紝杩欎粎鍦ㄤ娇鐢?`OPTEE_SMC_CALL_WITH_ARG` 鐨?yielding 璋冪敤杩涘叆瀹夊叏涓栫晫鏃跺彲鐢ㄣ€傝繖灏嗗叾鎺掗櫎鍦ㄥ畨鍏ㄤ笘鐣屼腑鏂鐞嗙▼搴忎箣澶栥€?
寮傛閫氱煡閫氳繃娉ㄥ唽鍦?OP-TEE 椹卞姩涓殑闈炲畨鍏ㄨ竟娌胯Е鍙戜腑鏂姇閫掔粰涓柇澶勭悊绋嬪簭銆傚疄闄呯殑閫氱煡鍊奸€氳繃蹇€熻皟鐢?`OPTEE_SMC_GET_ASYNC_NOTIF_VALUE` 鑾峰彇銆傝娉ㄦ剰锛屼竴涓腑鏂彲浠ヤ唬琛ㄥ涓€氱煡銆?
閫氱煡鍊?`OPTEE_SMC_ASYNC_NOTIF_VALUE_DO_BOTTOM_HALF` 鍏锋湁鐗规畩鍚箟銆傚綋鎺ユ敹鍒拌鍊兼椂锛屾剰鍛崇潃鏅€氫笘鐣屽簲褰撳彂璧蜂竴涓?yielding 璋冪敤 `OPTEE_MSG_CMD_DO_BOTTOM_HALF`銆傝璋冪敤鐢卞崗鍔╀腑鏂鐞嗙▼搴忕殑绾跨▼鍙戝嚭銆傝繖鏄畨鍏ㄤ笘鐣屼腑鐨?OP-TEE OS 瀹炵幇璁惧椹卞姩涓婂崐閮?涓嬪崐閮ㄩ鏍肩殑涓€涓瀯寤烘ā鍧椼€?
### OPTEE_INSECURE_LOAD_IMAGE Kconfig 閫夐」


OPTEE_INSECURE_LOAD_IMAGE Kconfig 閫夐」鍚敤浜嗗湪鍐呮牳鍚姩鍚庝粠鍐呮牳鍔犺浇 BL32 OP-TEE 闀滃儚鐨勮兘鍔涳紝鑰屼笉鏄湪鍐呮牳鍚姩鍓嶄粠鍥轰欢鍔犺浇銆傝繖杩橀渶瑕佸湪 Arm 鐨?Trusted Firmware 涓惎鐢ㄧ浉搴旂殑閫夐」銆侫rm 鐨?Trusted Firmware 鏂囨。 [^6^] 瑙ｉ噴浜嗗惎鐢ㄦ閫夐」鎵€甯︽潵鐨勫畨鍏ㄥ▉鑳侊紝浠ュ強鍥轰欢鍜屽钩鍙板眰闈㈢殑缂撹В鎺柦銆?
浣跨敤璇ラ€夐」鏃讹紝杩樺瓨鍦ㄥ簲褰撹В鍐崇殑銆侀拡瀵瑰唴鏍哥殑棰濆鏀诲嚮鍚戦噺/缂撹В鎺柦銆?
1. 鍚姩閾惧畨鍏ㄣ€?
   - 鏀诲嚮鍚戦噺锛氭浛鎹?rootfs 涓殑 OP-TEE OS 闀滃儚浠ヨ幏鍙栧绯荤粺鐨勬帶鍒舵潈銆?
   - 缂撹В锛氬繀椤绘湁楠岃瘉鍐呮牳鍜?rootfs 鐨勫惎鍔ㄩ摼瀹夊叏锛屽惁鍒欐敾鍑昏€呭彲浠ラ€氳繃淇敼 rootfs 涓殑鍐呭鏉ヤ慨鏀瑰凡鍔犺浇鐨?OP-TEE 浜岃繘鍒舵枃浠躲€?
2. 澶囩敤鍚姩妯″紡銆?
   - 鏀诲嚮鍚戦噺锛氫娇鐢ㄥ鐢ㄥ惎鍔ㄦā寮忥紙鍗虫仮澶嶆ā寮忥級鏃讹紝OP-TEE 椹卞姩涓嶄細琚姞杞斤紝浠庤€岀暀涓?SMC 婕忔礊銆?
   - 缂撹В锛氬鏋滃瓨鍦ㄥ鐢ㄥ惎鍔ㄨ澶囩殑鏂规硶锛堜緥濡傛仮澶嶆ā寮忥級锛屽簲纭繚鍦ㄩ偅绉嶆ā寮忎笅搴旂敤鐩稿悓鐨勭紦瑙ｆ帾鏂姐€?
3. SMC 璋冪敤涔嬪墠鐨勬敾鍑汇€?
   - 鏀诲嚮鍚戦噺锛氬湪鍙戝嚭鐢ㄤ簬鍔犺浇 OP-TEE 鐨?SMC 璋冪敤涔嬪墠鎵ц鐨勪唬鐮佸彲鑳借鍒╃敤锛屼粠鑰屽姞杞戒竴涓浛鎹㈢殑 OS 闀滃儚銆?
   - 缂撹В锛歄P-TEE 椹卞姩蹇呴』鍦ㄤ换浣曟綔鍦ㄧ殑鏀诲嚮鍚戦噺琚墦寮€涔嬪墠鍔犺浇銆傝繖搴斿寘鎷寕杞戒换浣曞彲淇敼鐨勬枃浠剁郴缁熴€佹墦寮€缃戠粶绔彛鎴栦笌澶栭儴璁惧锛堜緥濡?USB锛夐€氫俊銆?
4. 闃绘鍔犺浇 OP-TEE 鐨?SMC 璋冪敤銆?
   - 鏀诲嚮鍚戦噺锛氶樆姝㈤┍鍔ㄨ鎺㈡祴锛坧robe锛夛紝浠庤€屼娇鍔犺浇 OP-TEE 鐨?SMC 璋冪敤鍦ㄦ湡鏈涙椂鏈兘鎵ц锛屼娇鍏朵繚鎸佸紑鏀句互渚垮悗缁墽琛屽苟鍔犺浇琚慨鏀圭殑 OS銆?
   - 缂撹В锛氬缓璁皢 OP-TEE 椹卞姩鏋勫缓涓哄唴寤猴紙builtin锛夐┍鍔紝鑰岄潪妯″潡锛屼互闃叉鍙兘瀵艰嚧妯″潡涓嶈鍔犺浇鐨勬紡娲炲埄鐢ㄣ€?
## 鍙傝€冿紙References锛?

[^1^] https://github.com/OP-TEE/optee_os

[^2^] http://infocenter.arm.com/help/topic/com.arm.doc.den0028a/index.html

[^3^] drivers/tee/optee/optee_smc.h

[^4^] drivers/tee/optee/optee_msg.h

[^5^] http://www.globalplatform.org/specificationsdevice.asp look for
    "TEE Client API Specification v1.0" 骞剁偣鍑讳笅杞姐€?
[^6^] https://trustedfirmware-a.readthedocs.io/en/latest/threat_model/threat_model.html
