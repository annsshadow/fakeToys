## UCAN 鍗忚


UCAN 鏄熀浜庡井鎺у埗鍣ㄧ殑 USB-CAN 閫傞厤鍣ㄦ墍浣跨敤鐨勫崗璁紝璇ラ€傞厤鍣ㄩ泦鎴愬湪 Theobroma Systems 鐨?System-on-Module 涓婏紝涔熷彲浣滀负鐙珛鐨?USB 妫掕幏寰椼€?
UCAN 鍗忚琚璁′负涓庣‖浠舵棤鍏炽€傚畠绱у瘑鍦版ā浠夸簡 Linux 鍐呴儴琛ㄧず CAN 璁惧鐨勬柟寮忋€傛墍鏈夊瀛楄妭鏁存暟閮界紪鐮佷负 Little Endian銆?
鏈枃妗ｄ腑鎻愬埌鐨勬墍鏈夌粨鏋勯兘瀹氫箟鍦?`drivers/net/can/usb/ucan.c`銆?
## USB 绔偣


UCAN 璁惧浣跨敤涓変釜 USB 绔偣锛?
CONTROL 绔偣
  椹卞姩鍦ㄦ绔偣涓婂彂閫佽澶囩鐞嗗懡浠?
IN 绔偣
  璁惧鍙戦€?CAN 鏁版嵁甯у拰 CAN 閿欒甯?
OUT 绔偣
  椹卞姩鍦?OUT 绔偣涓婂彂閫?CAN 鏁版嵁甯?
## 鎺у埗娑堟伅锛圕ONTROL Messages锛?

UCAN 璁惧閫氳繃浣跨敤鎺у埗绠￠亾锛坈ontrol pipe锛変笂鐨勫巶鍟嗚姹傦紙vendor request锛夎繘琛岄厤缃€?
涓轰簡鏀寔鍗曚釜 USB 璁惧涓殑澶氫釜 CAN 鎺ュ彛锛屾墍鏈夐厤缃懡浠ら兘鎸囧悜 USB 鎻忚堪绗︿腑鐩稿簲鐨勬帴鍙ｃ€?
椹卞姩浣跨敤 `ucan_ctrl_command_in/out` 鍜?`ucan_device_request_in` 鍚戣澶囦紶閫掑懡浠ゃ€?
### 寤虹珛鍖咃紙Setup Packet锛?

=================  =====================================================
`bmRequestType`  鏂瑰悜 | 鍘傚晢锛圴endor锛?| 锛堟帴鍙ｆ垨璁惧锛?`bRequest`       鍛戒护缂栧彿
`wValue`         瀛愬懡浠ょ紪鍙凤紙16 浣嶏級锛岃嫢鏈娇鐢ㄥ垯涓?0
`wIndex`         USB 鎺ュ彛绱㈠紩锛堣澶囧懡浠や负 0锛?`wLength`        * 涓绘満鍒拌澶?- 瑕佸彂閫佺殑瀛楄妭鏁?                   - 璁惧鍒颁富鏈?- 瑕佹帴鏀剁殑鏈€澶у瓧鑺傛暟
                     濡傛灉璁惧鍙戦€佽緝灏戯紝鍒欎娇鐢ㄥ父瑙佺殑 ZLP 璇箟銆?=================  =====================================================

### 閿欒澶勭悊


璁惧閫氳繃闃诲锛坰tall锛夎绠￠亾鏉ユ寚绀哄け璐ョ殑鎺у埗鍛戒护銆?
### 璁惧鍛戒护


#### UCAN_DEVICE_GET_FW_STRING


**Dev2Host锛涘彲閫?*

璇锋眰璁惧鍥轰欢瀛楃涓层€?
### 鎺ュ彛鍛戒护


#### UCAN_COMMAND_START


**Host2Dev锛涘繀闇€**

鍚姩 CAN 鎺ュ彛銆?
Payload 鏍煎紡
  `ucan_ctl_payload_t.cmd_start`

====  ============================
mode  `UCAN_MODE_*` 鐨勬垨鎺╃爜
====  ============================

#### UCAN_COMMAND_STOP


**Host2Dev锛涘繀闇€**

鍋滄 CAN 鎺ュ彛

Payload 鏍煎紡
  **绌?*

#### UCAN_COMMAND_RESET


**Host2Dev锛涘繀闇€**

澶嶄綅 CAN 鎺у埗鍣紙鍖呮嫭閿欒璁℃暟鍣級

Payload 鏍煎紡
  **绌?*

#### UCAN_COMMAND_GET


**Host2Dev锛涘繀闇€**

浠庤澶囪幏鍙栦俊鎭?
##### 瀛愬懡浠?

UCAN_COMMAND_GET_INFO
  璇锋眰璁惧淇℃伅缁撴瀯 `ucan_ctl_payload_t.device_info`銆?
  缁嗚妭鍙傝 `device_info` 瀛楁锛屼互鍙?  `uapi/linux/can/netlink.h` 涓 `can_bittiming 瀛楁` 鐨勮鏄庛€?
  Payload 鏍煎紡
    `ucan_ctl_payload_t.device_info`

UCAN_COMMAND_GET_PROTOCOL_VERSION

  璇锋眰璁惧鍗忚鐗堟湰
  `ucan_ctl_payload_t.protocol_version`銆傚綋鍓嶅崗璁増鏈负 3銆?
  Payload 鏍煎紡
    `ucan_ctl_payload_t.protocol_version`

          protocol version 1

#### UCAN_COMMAND_SET_BITTIMING


**Host2Dev锛涘繀闇€**

閫氳繃鍙戦€佺粨鏋?`ucan_ctl_payload_t.cmd_set_bittiming`锛堢粏鑺傝 `struct bittiming`锛夋潵璁剧疆浣嶆椂搴忥紙bittiming锛?
Payload 鏍煎紡
  `ucan_ctl_payload_t.cmd_set_bittiming`銆?
#### UCAN_SLEEP/WAKE


**Host2Dev锛涘彲閫?*

閰嶇疆鐫＄湢鍜屽敜閱掓ā寮忋€傞┍鍔ㄥ皻涓嶆敮鎸併€?
#### UCAN_FILTER


**Host2Dev锛涘彲閫?*

璁剧疆纭欢 CAN 杩囨护鍣ㄣ€傞┍鍔ㄥ皻涓嶆敮鎸併€?
### 鍏佽鐨勬帴鍙ｅ懡浠?

==================  ===================  ==================
鍚堟硶璁惧鐘舵€?        鍛戒护                 鏂拌澶囩姸鎬?==================  ===================  ==================
stopped             SET_BITTIMING        stopped
stopped             START                started
started             STOP or RESET        stopped
stopped             STOP or RESET        stopped
started             RESTART              started
any                 GET                  **鏃犲彉鍖?*
==================  ===================  ==================

## IN 娑堟伅鏍煎紡


USB IN 绔偣涓婄殑鏁版嵁鍖呭寘鍚竴涓垨澶氫釜 `ucan_message_in` 鍊笺€傚鏋滃涓秷鎭鎵瑰鐞嗗湪涓€涓?USB 鏁版嵁鍖呬腑锛宍len` 瀛楁鍙敤浜庤烦鍒颁笅涓€涓?`ucan_message_in` 鍊硷紙娉ㄦ剰瀵?`len` 鍊煎仛鍋ュ叏鎬ф鏌ワ紝浠ュ鐓у疄闄呮暟鎹ぇ灏忥級銆?
### ``len`` 瀛楁


姣忎釜 `ucan_message_in` 蹇呴』瀵归綈鍒?4 瀛楄妭杈圭晫锛堢浉瀵逛簬鏁版嵁缂撳啿鍖鸿捣濮嬬殑浣嶇疆锛夈€傝繖鎰忓懗鐫€鍦ㄥ涓?`ucan_message_in` 鍊间箣闂村彲鑳芥湁濉厖瀛楄妭锛?
    +----------------------------+ < 0
    |                            |
    |   struct ucan_message_in   |
    |                            |
    +----------------------------+ < len
              [padding]
    +----------------------------+ < round_up(len, 4)
    |                            |
    |   struct ucan_message_in   |
    |                            |
    +----------------------------+
                [...]

### ``type`` 瀛楁


`type` 瀛楁鎸囧畾娑堟伅鐨勭被鍨嬨€?
#### UCAN_IN_RX


`subtype`
  zero

浠?CAN 鎬荤嚎鎺ユ敹鍒扮殑鏁版嵁锛圛D + 杞借嵎锛夈€?
#### UCAN_IN_TX_COMPLETE


`subtype`
  zero

CAN 璁惧宸插悜 CAN 鎬荤嚎鍙戦€佷簡涓€鏉℃秷鎭€傚畠鐢ㄤ竴涓厓缁勫垪琛?<echo-ids, flags> 浣滀负搴旂瓟銆?
echo-id 鏍囪瘑浜嗘潵鑷紙鍥炴樉浜嗗厛鍓?UCAN_OUT_TX 娑堟伅鐨?id锛夌殑甯с€俧lag 鎸囩ず浼犺緭鐨勭粨鏋溿€傚叾涓紝缃綅鐨?Bit 0 琛ㄧず鎴愬姛銆傛墍鏈夊叾浠栦綅淇濈暀骞惰涓洪浂銆?
### 娴佹帶


鎺ユ敹 CAN 娑堟伅鏃讹紝USB 缂撳啿鍖轰笂娌℃湁娴佹帶銆傞┍鍔ㄥ繀椤昏冻澶熷揩鍦板鐞嗗叆绔欐秷鎭互閬垮厤涓㈠寘銆傚鏋滆澶囩紦鍐插尯婧㈠嚭锛岃鐘跺喌浼氶€氳繃鍙戦€佺浉搴旂殑閿欒甯ф潵鎶ュ憡锛堝弬瑙?can_ucan_error_handling锛夈€?
## OUT 娑堟伅鏍煎紡


USB OUT 绔偣涓婄殑鏁版嵁鍖呭寘鍚竴涓垨澶氫釜 ``struct ucan_message_out`` 鍊笺€傚鏋滃涓秷鎭鎵瑰鐞嗗埌涓€涓暟鎹寘涓紝璁惧浣跨敤 `len` 瀛楁璺冲埌涓嬩竴涓?ucan_message_out 鍊笺€傛瘡涓?ucan_message_out 蹇呴』瀵归綈鍒?4 瀛楄妭锛堢浉瀵逛簬鏁版嵁缂撳啿鍖鸿捣濮嬬殑浣嶇疆锛夈€傝鏈哄埗涓?can_ucan_in_message_len 涓弿杩扮殑涓€鏍枫€?
    +----------------------------+ < 0
    |                            |
    |   struct ucan_message_out  |
    |                            |
    +----------------------------+ < len
              [padding]
    +----------------------------+ < round_up(len, 4)
    |                            |
    |   struct ucan_message_out  |
    |                            |
    +----------------------------+
                [...]

### ``type`` 瀛楁


鍦ㄥ崗璁増鏈?3 涓彧瀹氫箟浜?`UCAN_OUT_TX`锛屽叾浠栫殑浠呯敱鏃ц澶囷紙鍗忚鐗堟湰 1锛変娇鐢ㄣ€?
#### UCAN_OUT_TX

`subtype`
  瑕佸湪 CAN_IN_TX_COMPLETE 娑堟伅涓簲绛旂殑 echo id

鍙戦€佷竴涓?CAN 甯с€傦紙鍙傛暟锛歚id`銆乣data`锛?
### 娴佹帶


褰撹澶囧嚭绔欑紦鍐插尯婊℃椂锛屽畠寮€濮嬪湪 **OUT** 绠￠亾涓婂彂閫?**NAK**锛岀洿鍒版湁鏇村缂撳啿鍖哄彲鐢ㄣ€傚綋鏈畬鎴愬嚭绔欏寘杈惧埌涓€瀹氶槇鍊兼椂锛岄┍鍔ㄥ仠姝㈤槦鍒椼€?
## CAN 閿欒澶勭悊


濡傛灉寮€鍚簡閿欒鎶ュ憡锛岃澶囦細鎶婇敊璇紪鐮佷负 CAN 閿欒甯э紙鍙傝 `uapi/linux/can/error.h`锛夊苟閫氳繃 IN 绔偣鍙戦€併€傞┍鍔ㄦ洿鏂板叾閿欒缁熻骞惰浆鍙戝畠銆?
灏界 UCAN 璁惧鍙互瀹屽叏鎶戝埗閿欒甯э紝浣嗗湪 Linux 涓┍鍔ㄦ€绘槸鎰熷叴瓒ｇ殑銆傚洜姝わ紝璁惧鎬绘槸浠ヨ缃簡 `UCAN_MODE_BERR_REPORT` 鐨勬柟寮忓惎鍔ㄣ€備负 user space 杩囨护杩欎簺娑堟伅鐢遍┍鍔ㄥ畬鎴愩€?
### 鎬荤嚎鍏抽棴锛圔us OFF锛?

- 璁惧涓嶄細鑷姩浠庢€荤嚎鍏抽棴涓仮澶嶃€?- 鎬荤嚎鍏抽棴鐢遍敊璇抚鎸囩ず锛堝弬瑙?`uapi/linux/can/error.h`锛?- 鎬荤嚎鍏抽棴鎭㈠鐢?`UCAN_COMMAND_RESTART` 鍚姩
- 涓€鏃︽€荤嚎鍏抽棴鎭㈠瀹屾垚锛岃澶囧彂閫佷竴涓敊璇抚锛屾寚绀哄叾澶勪簬 ERROR-ACTIVE 鐘舵€併€?- 鍦ㄦ€荤嚎鍏抽棴鏈熼棿锛岃澶囦笉鍙戦€佷换浣曞抚銆?- 鍦ㄦ€荤嚎鍏抽棴鏈熼棿锛屾潵鑷富鏈虹殑浼犺緭璇锋眰浼氱珛鍗冲畬鎴愶紝涓旀垚鍔熶綅淇濇寔鏈疆浣嶃€?
## 绀轰緥浼氳瘽


#) 璁惧杩炴帴鍒?USB
#) 涓绘満鍙戦€佸懡浠?`UCAN_COMMAND_RESET`锛宻ubcmd 0
#) 涓绘満鍙戦€佸懡浠?`UCAN_COMMAND_GET`锛宻ubcmd `UCAN_COMMAND_GET_INFO`
#) 璁惧鍙戦€?`UCAN_IN_DEVICE_INFO`
#) 涓绘満鍙戦€佸懡浠?`UCAN_OUT_SET_BITTIMING`
#) 涓绘満鍙戦€佸懡浠?`UCAN_COMMAND_START`锛宻ubcmd 0锛宮ode `UCAN_MODE_BERR_REPORT`
