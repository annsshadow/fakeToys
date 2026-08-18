


## 缂栧啓瀹㈡埛绔┍鍔?
API 鏂囨。璇峰弬闃咃細

- [client-api](client-api)

## 姒傝堪

瀹㈡埛绔┍鍔ㄧ殑鎼缓涓昏鏈変袱绉嶆柟寮忥紝鍙栧喅浜庡搴旂殑璁惧鏄浣曟彁渚涚粰绯荤粺鐨勩€傛垜浠壒鍒尯鍒嗕簡閫氳繃甯歌鏂瑰紡锛堜緥濡傞€氳繃 ACPI 浣滀负骞冲彴璁惧锛夊憟鐜扮粰绯荤粺鐨勮澶囷紝浠ュ強涓嶅彲鍙戠幇銆佸洜鑰岄渶瑕侀€氳繃鍏朵粬鏈哄埗鏄惧紡鎻愪緵鐨勮澶囷紝涓嬫枃灏嗗姝よ繘涓€姝ヨ璁恒€?
## 闈?SSAM 瀹㈡埛绔┍鍔?
涓?SAM EC 鐨勬墍鏈夐€氫俊閮介€氳繃浠ｈ〃璇?EC 鐨?|ssam_controller| 澶勭悊锛屽悗鑰呭悜鍐呮牳鍛堢幇璇?EC銆傞潰鍚戦潪 SSAM 璁惧锛堝洜姝ゅ苟闈?|ssam_device_driver|锛夌殑椹卞姩闇€瑕佹樉寮忓湴寤虹珛涓庤鎺у埗鍣ㄧ殑杩炴帴/鍏宠仈銆傝繖鍙互閫氳繃 |ssam_client_bind| 鍑芥暟瀹屾垚銆傝鍑芥暟杩斿洖瀵?SSAM 鎺у埗鍣ㄧ殑寮曠敤锛屼絾鏇撮噸瑕佺殑鏄紝瀹冭繕鍦ㄥ鎴风璁惧涓庢帶鍒跺櫒涔嬮棿寤虹珛浜嗕竴鏉¤澶囬摼鎺ワ紙涔熷彲浠ラ€氳繃 |ssam_client_link| 鍗曠嫭瀹屾垚锛夈€傝繖鏍峰仛寰堥噸瑕侊紝鍥犱负棣栧厛锛屽畠淇濊瘉鍦ㄦ墍杩斿洖鐨勬帶鍒跺櫒鍦ㄥ叾椹卞姩缁戝畾鍒拌澶囨湡闂村璇ュ鎴风椹卞姩濮嬬粓鏈夋晥锛屽嵆椹卞姩浼氬湪鎺у埗鍣ㄥけ鏁堜箣鍓嶅厛琚В缁戯紱鍏舵锛屽畠纭繚浜嗘纭殑鎸傝捣/鎭㈠椤哄簭銆傝繖涓€璁剧疆搴斿綋鍦ㄩ┍鍔ㄧ殑 probe 鍑芥暟涓畬鎴愶紝骞朵笖鍙敤浜庡湪 SSAM 瀛愮郴缁熷皻鏈氨缁椂鎺ㄨ繜鎺㈡祴锛屼緥濡傦細

   static int client_driver_probe(struct platform_device *pdev)
   {
           struct ssam_controller *ctrl;

           ctrl = ssam_client_bind(&pdev->dev);
           if (IS_ERR(ctrl))
                   return PTR_ERR(ctrl) == -ENODEV ? -EPROBE_DEFER : PTR_ERR(ctrl);

           // ...

           return 0;
   }

鎺у埗鍣ㄤ篃鍙互閫氳繃 |ssam_get_controller| 鍗曠嫭鑾峰彇锛屽叾鐢熷懡鍛ㄦ湡鍙€氳繃 |ssam_controller_get| 涓?|ssam_controller_put| 鍔犱互淇濊瘉銆備絾璇锋敞鎰忥紝杩欎簺鍑芥暟閮戒笉鑳戒繚璇佹帶鍒跺櫒涓嶄細琚叧闂垨鎸傝捣銆傝繖浜涘嚱鏁版湰璐ㄤ笂鍙搷浣滃紩鐢紝鍗冲彧淇濊瘉鏈€浣庨檺搴︾殑鍙闂€э紝鑰屽瀹為檯鍙搷浣滄€т笉浣滀换浣曚繚璇併€?
## 娣诲姞 SSAM 璁惧

濡傛灉鏌愪釜璁惧灏氫笉瀛樺湪/灏氭湭閫氳繃甯歌鏂瑰紡鎻愪緵锛屽垯搴旈€氳繃 SSAM 瀹㈡埛绔澶?hub 灏嗗叾浣滀负 |ssam_device| 鎻愪緵銆傚彲浠ラ€氳繃灏嗘柊璁惧鐨?UID 褰曞叆鐩稿簲鐨勬敞鍐岃〃鏉ュ皢鍏舵坊鍔犲埌璇?hub銆係SAM 璁惧涔熷彲浠ユ墜鍔ㄩ€氳繃 |ssam_device_alloc| 鍒嗛厤锛岄殢鍚庡繀椤婚€氳繃 |ssam_device_add| 娣诲姞锛屽苟鏈€缁堥€氳繃 |ssam_device_remove| 绉婚櫎銆傞粯璁ゆ儏鍐典笅锛岃澶囩殑鐖惰澶囪璁句负鐢ㄤ簬鍒嗛厤鐨勬帶鍒跺櫒璁惧锛屼絾杩欏彲浠ュ湪璁惧琚坊鍔犱箣鍓嶆洿鏀广€傛敞鎰忥紝鍦ㄦ洿鏀圭埗璁惧鏃讹紝蹇呴』娉ㄦ剰纭繚榛樿璁剧疆锛堥€氳繃鐖跺瓙鍏崇郴鎻愪緵锛変腑鍏充簬鎺у埗鍣ㄧ敓鍛藉懆鏈熷拰鎸傝捣/鎭㈠椤哄簭鐨勪繚璇佸緱浠ヤ繚鐣欙紱濡傛湁蹇呰锛屽彲浣跨敤 |ssam_client_link|锛屾濡傞潪 SSAM 瀹㈡埛绔┍鍔ㄦ墍鍋氱殑閭ｆ牱锛岃瑙佷笂鏂囥€?
瀹㈡埛绔澶囧繀椤诲缁堢敱娣诲姞璇ヨ澶囩殑閭ｄ竴鏂瑰湪鎺у埗鍣ㄥ叧闂箣鍓嶅皢鍏剁Щ闄ゃ€傝繖绉嶇Щ闄ゅ彲浠ラ€氳繃鎶婃彁渚?SSAM 璁惧鐨勯┍鍔ㄩ€氳繃 |ssam_client_link| 閾炬帴鍒版帶鍒跺櫒鏉ヤ繚璇侊紝浠庤€屼娇鍏跺湪鎺у埗鍣ㄩ┍鍔ㄨВ缁戜箣鍓嶅厛瑙ｇ粦銆備互鎺у埗鍣ㄤ负鐖惰澶囨敞鍐岀殑瀹㈡埛绔澶囦細鍦ㄦ帶鍒跺櫒鍏抽棴鏃惰嚜鍔ㄧЩ闄わ紝浣嗕笉搴斾緷璧栬繖涓€鐐癸紝灏ゅ叾鏄洜涓鸿繖骞朵笉閫傜敤浜庡叿鏈夊叾浠栫埗璁惧鐨勫鎴风璁惧銆?
## SSAM 瀹㈡埛绔┍鍔?
SSAM 瀹㈡埛绔澶囬┍鍔ㄦ湰璐ㄤ笂涓庡叾浠栬澶囬┍鍔ㄧ被鍨嬫病鏈夊尯鍒€傚畠浠€氳繃 |ssam_device_driver| 琛ㄧず锛屽苟閫氳繃鍏?UID锛坄struct ssam_device.uid <ssam_device>`锛夋垚鍛樹互鍙婂尮閰嶈〃锛坄struct ssam_device_driver.match_table <ssam_device_driver>`锛夌粦瀹氬埌 |ssam_device|锛岃繖浜涘簲鍦ㄥ０鏄庨┍鍔ㄧ粨鏋勪綋瀹炰緥鏃惰缃€傚叧浜庡浣曞畾涔夐┍鍔ㄥ尮閰嶈〃鐨勬垚鍛橈紝璇峰弬闃?|SSAM_DEVICE| 瀹忕殑鏂囨。銆?
SSAM 瀹㈡埛绔澶囩殑 UID 鐢?`domain`銆乣category`銆乣target`銆乣instance` 涓?`function` 缁勬垚銆俙domain` 鐢ㄤ簬鍖哄垎鐗╃悊 SAM 璁惧锛坄SSAM_DOMAIN_SERIALHUB <ssam_device_domain>`锛屽嵆閫氳繃 Surface Serial Hub 鍙闂殑璁惧锛変笌铏氭嫙璁惧锛坄SSAM_DOMAIN_VIRTUAL <ssam_device_domain>`锛屼緥濡傚鎴风璁惧 hub锛屽畠浠湪 SAM EC 涓婃病鏈夌湡瀹炶〃绀猴紝浠呯敤浜庡唴鏍?椹卞姩渚э級銆傚浜庣墿鐞嗚澶囷紝`category` 琛ㄧず鐩爣绫诲埆锛宍target` 琛ㄧず鐩爣 ID锛宍instance` 琛ㄧず鐢ㄤ簬璁块棶鐗╃悊 SAM 璁惧鐨勫疄渚?ID銆傛澶栵紝`function` 寮曠敤鐗瑰畾鐨勮澶囧姛鑳斤紝浣嗗 SAM EC 娌℃湁鎰忎箟銆傚鎴风璁惧鐨勶紙榛樿锛夊悕绉版牴鎹叾 UID 鐢熸垚銆?
椹卞姩瀹炰緥鍙互閫氳繃 |ssam_device_driver_register| 娉ㄥ唽锛岄€氳繃 |ssam_device_driver_unregister| 娉ㄩ攢銆備负鏂逛究璧疯锛屽彲浣跨敤 |module_ssam_device_driver| 瀹忔潵瀹氫箟娉ㄥ唽璇ラ┍鍔ㄧ殑妯″潡 init 涓?exit 鍑芥暟銆?
涓?SSAM 瀹㈡埛绔澶囧叧鑱旂殑鎺у埗鍣ㄥ彲鍦ㄥ叾 `struct ssam_device.ctrl <ssam_device>` 鎴愬憳涓壘鍒般€傝寮曠敤淇濊瘉鑷冲皯鍦ㄥ鎴风椹卞姩缁戝畾鏈熼棿鏈夋晥锛屼絾鐞嗚涓婁篃搴斿湪璇ュ鎴风璁惧瀛樺湪鏈熼棿涓€鐩存湁鏁堛€備絾娉ㄦ剰锛屽湪宸茬粦瀹氱殑瀹㈡埛绔┍鍔ㄤ箣澶栬繘琛岃闂椂锛屽繀椤荤‘淇濇帶鍒跺櫒璁惧鍦ㄨ繘琛屼换浣曡姹傛垨锛堟敞閿€锛夋敞鍐屼簨浠堕€氱煡鍣ㄦ椂鏈鎸傝捣锛堝洜姝ら€氬父搴旈伩鍏嶏級銆傚綋鎺у埗鍣ㄤ粠宸茬粦瀹氱殑瀹㈡埛绔┍鍔ㄥ唴閮ㄨ闂椂锛岃繖涓€鐐瑰彲浠ュ緱鍒颁繚璇併€?
## 鍙戣捣鍚屾璇锋眰

鍚屾璇锋眰锛堢洰鍓嶏級鏄富鏈哄彂璧风殑銆佷笌 EC 閫氫俊鐨勪富瑕佸舰寮忋€傛湁澶氱鏂瑰紡鏉ュ畾涔夊苟鎵ц姝ょ被璇锋眰锛屼絾澶у鏈€缁堥兘褰掔粨涓轰笌涓嬮潰绀轰緥绫讳技鐨勫舰寮忋€傝绀轰緥瀹氫箟浜嗕竴涓啓-璇昏姹傦紝鍗宠皟鐢ㄨ€呭悜 SAM EC 鎻愪緵涓€涓弬鏁板苟鏀跺埌涓€涓搷搴斻€傝皟鐢ㄨ€呴渶瑕佺煡閬撳搷搴旇浇鑽风殑锛堟渶澶э級闀垮害骞朵负鍏舵彁渚涗竴涓紦鍐插尯銆?
蹇呴』娉ㄦ剰纭繚浼犵粰 SAM EC 鐨勪换浣曞懡浠よ浇鑽锋暟鎹兘浠ュ皬绔紙little-endian锛夋牸寮忔彁渚涳紝鍚屾牱鍦帮紝浠庡畠鏀跺埌鐨勪换浣曞搷搴旇浇鑽锋暟鎹兘瑕佷粠灏忕杞崲涓轰富鏈哄瓧鑺傚簭銆?
   int perform_request(struct ssam_controller **ctrl, u32 arg, u32 **ret)
   {
           struct ssam_request rqst;
           struct ssam_response resp;
           int status;

           /** Convert request argument to little-endian. **/
           __le32 arg_le = cpu_to_le32(arg);
           __le32 ret_le = cpu_to_le32(0);

           /*
            - Initialize request specification. Replace this with your values.
            - The rqst.payload field may be NULL if rqst.length is zero,
            - indicating that the request does not have any argument.
            *
            - Note: The request parameters used here are not valid, i.e.
            - they do not correspond to an actual SAM/EC request.
            */
           rqst.target_category = SSAM_SSH_TC_SAM;
           rqst.target_id = SSAM_SSH_TID_SAM;
           rqst.command_id = 0x02;
           rqst.instance_id = 0x03;
           rqst.flags = SSAM_REQUEST_HAS_RESPONSE;
           rqst.length = sizeof(arg_le);
           rqst.payload = (u8 *)&arg_le;

           /** Initialize request response. **/
           resp.capacity = sizeof(ret_le);
           resp.length = 0;
           resp.pointer = (u8 *)&ret_le;

           /*
            - Perform actual request. The response pointer may be null in case
            - the request does not have any response. This must be consistent
            - with the SSAM_REQUEST_HAS_RESPONSE flag set in the specification
            - above.
            */
           status = ssam_request_do_sync(ctrl, &rqst, &resp);

           /*
            - Alternatively use
            *
            - ssam_request_do_sync_onstack(ctrl, &rqst, &resp, sizeof(arg_le));
            *
            - to perform the request, allocating the message buffer directly
            - on the stack as opposed to allocation via kzalloc().
            */

           /*
            - Convert request response back to native format. Note that in the
            - error case, this value is not touched by the SSAM core, i.e.
            - 'ret_le' will be zero as specified in its initialization.
            */
           *ret = le32_to_cpu(ret_le);

           return status;
   }

娉ㄦ剰锛寍ssam_request_do_sync| 鏈川涓婃槸瀵规洿浣庡眰璇锋眰鍘熻鐨勫皝瑁咃紝閭ｄ簺鍘熻涔熷彲鐢ㄤ簬鎵ц璇锋眰銆傛洿澶氱粏鑺傝鍙傞槄鍏跺疄鐜颁笌鏂囨。銆?
瀹氫箟姝ょ被鍑芥暟涓紝arguably 涓€绉嶅鐢ㄦ埛鏇村弸濂界殑鏂瑰紡鏄娇鐢ㄥ叾涓竴涓敓鎴愬畯锛屼緥濡傦細

   SSAM_DEFINE_SYNC_REQUEST_W(__ssam_tmp_perf_mode_set, __le32, {
           .target_category = SSAM_SSH_TC_TMP,
           .target_id       = SSAM_SSH_TID_SAM,
           .command_id      = 0x03,
           .instance_id     = 0x00,
   });

璇ョず渚嬪畾涔変簡涓€涓嚱鏁?
   static int __ssam_tmp_perf_mode_set(struct ssam_controller **ctrl, const __le32 **arg);

鐢ㄤ簬鎵ц鎸囧畾鐨勮姹傦紝璋冪敤璇ュ嚱鏁版椂浼犲叆鎺у埗鍣ㄣ€傚湪姝ょず渚嬩腑锛屽弬鏁伴€氳繃 `arg` 鎸囬拡鎻愪緵銆傛敞鎰忥紝鐢熸垚鐨勫嚱鏁颁細鍦ㄦ爤涓婂垎閰嶆秷鎭紦鍐插尯銆傚洜姝わ紝濡傛灉璇锋眰鎻愪緵鐨勫弬鏁拌緝澶э紝搴斿綋閬垮厤浣跨敤杩欑被瀹忋€傝繕瑕佹敞鎰忥紝涓庡墠闈㈤潪瀹忕殑绀轰緥涓嶅悓锛岃鍑芥暟涓嶅仛浠讳綍瀛楄妭搴忚浆鎹紝杩欏繀椤荤敱璋冪敤鑰呭鐞嗐€傞櫎浜嗚繖浜涘樊寮備箣澶栵紝瀹忕敓鎴愮殑鍑芥暟涓庝笂闈㈤潪瀹忕ず渚嬩腑鐨勫嚱鏁扮浉浼笺€?
杩欑被鍑芥暟鐢熸垚瀹忕殑瀹屾暣鍒楄〃涓猴細

- `SSAM_DEFINE_SYNC_REQUEST_N`锛氱敤浜庢棤杩斿洖鍊间笖鏃犲弬鏁扮殑璇锋眰銆?- `SSAM_DEFINE_SYNC_REQUEST_R`锛氱敤浜庢湁杩斿洖鍊间絾鏃犲弬鏁扮殑璇锋眰銆?- `SSAM_DEFINE_SYNC_REQUEST_W`锛氱敤浜庢棤杩斿洖鍊间絾鏈夊弬鏁扮殑璇锋眰銆?
鏇村缁嗚妭璇峰弬闃呭畠浠悇鑷殑鏂囨。銆傚浜庤繖浜涘畯涓殑姣忎竴涓紝閮芥彁渚涗簡涓€涓壒娈婂彉浣擄紝閽堝閫傜敤浜庡悓涓€璁惧绫诲瀷澶氫釜瀹炰緥鐨勮姹傜被鍨嬶細

- `SSAM_DEFINE_SYNC_REQUEST_MD_N`
- `SSAM_DEFINE_SYNC_REQUEST_MD_R`
- `SSAM_DEFINE_SYNC_REQUEST_MD_W`

杩欎簺瀹忎笌涓婅堪鐗堟湰鐨勫尯鍒湪浜庯紝鎵€鐢熸垚鍑芥暟涓澶?target 涓?instance ID 骞堕潪鍥哄畾锛岃€屾槸蹇呴』鐢辫鍑芥暟鐨勮皟鐢ㄨ€呮彁渚涖€?
姝ゅ锛岃繕鎻愪緵浜嗗彲鐩存帴鐢ㄤ簬瀹㈡埛绔澶囷紙鍗?|ssam_device|锛夌殑鍙樹綋銆備緥濡傚彲浠ユ寜濡備笅鏂瑰紡浣跨敤锛?
   SSAM_DEFINE_SYNC_REQUEST_CL_R(ssam_bat_get_sta, __le32, {
           .target_category = SSAM_SSH_TC_BAT,
           .command_id      = 0x01,
   });

瀵硅瀹忕殑杩欐璋冪敤瀹氫箟浜嗕竴涓嚱鏁?
   static int ssam_bat_get_sta(struct ssam_device **sdev, __le32 **ret);

鐢ㄤ簬鎵ц鎸囧畾鐨勮姹傦紝浣跨敤瀹㈡埛绔澶囦腑缁欏嚭鐨勮澶?ID 涓庢帶鍒跺櫒銆傝繖绫荤敤浜庡鎴风璁惧鐨勫畯鐨勫畬鏁村垪琛ㄤ负锛?
- `SSAM_DEFINE_SYNC_REQUEST_CL_N`
- `SSAM_DEFINE_SYNC_REQUEST_CL_R`
- `SSAM_DEFINE_SYNC_REQUEST_CL_W`

## 澶勭悊浜嬩欢

瑕佷粠 SAM EC 鎺ユ敹浜嬩欢锛屽繀椤婚€氳繃 |ssam_notifier_register| 涓烘湡鏈涚殑浜嬩欢娉ㄥ唽涓€涓簨浠堕€氱煡鍣紙notifier锛夈€傚綋涓嶅啀闇€瑕佽閫氱煡鍣ㄦ椂锛屽繀椤婚€氳繃 |ssam_notifier_unregister| 娉ㄩ攢瀹冦€傚浜?|ssam_device| 绫诲瀷鐨勫鎴风锛屽簲褰撲紭鍏堜娇鐢?|ssam_device_notifier_register| 涓?|ssam_device_notifier_unregister| 杩欎袱涓皝瑁呭嚱鏁帮紝鍥犱负瀹冧滑鑳芥纭鐞嗗鎴风璁惧鐨勭儹绉婚櫎銆?
娉ㄥ唽浜嬩欢閫氱煡鍣ㄦ椂锛岃嚦灏戣鎻愪緵锛氭敹鍒颁簨浠舵椂璋冪敤鐨勫洖璋冨嚱鏁般€佹寚瀹氬浣曞惎鐢ㄨ浜嬩欢鐨勬敞鍐岃〃锛坮egistry锛夈€佹寚瀹氬簲涓哄摢涓洰鏍囩被鍒紙浠ュ強鏍规嵁鎵€鐢ㄦ敞鍐岃〃锛屽彲閫夊湴鎸囧畾鍝釜瀹炰緥 ID锛夊惎鐢ㄤ簨浠剁殑浜嬩欢 ID锛屾渶鍚庢槸鎻忚堪 EC 灏嗗浣曞彂閫佽繖浜涗簨浠剁殑鏍囧織銆傚鏋滅壒瀹氱殑娉ㄥ唽琛ㄤ笉鎸夊疄渚?ID 鍚敤浜嬩欢锛屽垯瀹炰緥 ID 蹇呴』璁句负闆躲€傛澶栵紝鍙互涓虹浉搴旈€氱煡鍣ㄦ寚瀹氫竴涓紭鍏堢骇锛屽畠鍐冲畾璇ラ€氱煡鍣ㄧ浉瀵逛簬娉ㄥ唽鍒板悓涓€鐩爣绫诲埆鐨勪换浣曞叾浠栭€氱煡鍣ㄧ殑椤哄簭銆?
榛樿鎯呭喌涓嬶紝浜嬩欢閫氱煡鍣ㄤ細鎺ユ敹鐗瑰畾鐩爣绫诲埆鐨勬墍鏈変簨浠讹紝鏃犺娉ㄥ唽閫氱煡鍣ㄦ椂鎸囧畾鐨勫疄渚?ID 涓轰綍銆傞€氳繃鎻愪緵浜嬩欢鎺╃爜锛堝弬瑙?|ssam_event_mask|锛夛紝鍙互鎸囩ず鏍稿績浠呭湪浜嬩欢鐨勭洰鏍?ID 鎴栧疄渚?ID锛堟垨涓よ€咃級涓庨€氱煡鍣?ID 鎵€闅愬惈鐨勭浉鍖归厤鏃舵墠璋冪敤璇ラ€氱煡鍣紙瀵逛簬鐩爣 ID锛屽嵆娉ㄥ唽琛ㄧ殑鐩爣 ID锛夈€?
涓€鑸€岃█锛屾敞鍐岃〃鐨勭洰鏍?ID 涔熷氨鏄墍鍚敤浜嬩欢鐨勭洰鏍?ID锛堜竴涓樉钁楃殑渚嬪鏄?Surface Laptop 1 鍜?2 涓婄殑閿洏杈撳叆浜嬩欢锛氬畠浠€氳繃鐩爣 ID 涓?1 鐨勬敞鍐岃〃鍚敤锛屼絾鎻愪緵鐨勪簨浠剁洰鏍?ID 涓?2锛夈€?
涓嬮潰鏄竴涓敞鍐屼簨浠堕€氱煡鍣ㄥ苟澶勭悊鏀跺埌浜嬩欢鐨勫畬鏁寸ず渚嬶細

   u32 notifier_callback(struct ssam_event_notifier *nf,
                         const struct ssam_event *event)
   {
           int status = ...

           /** Handle the event here ... **/

           /** Convert return value and indicate that we handled the event. **/
           return ssam_notifier_from_errno(status) | SSAM_NOTIF_HANDLED;
   }

   int setup_notifier(struct ssam_device *sdev,
                      struct ssam_event_notifier *nf)
   {
           /** Set priority wrt. other handlers of same target category. **/
           nf->base.priority = 1;

           /** Set event/notifier callback. **/
           nf->base.fn = notifier_callback;

           /** Specify event registry, i.e. how events get enabled/disabled. **/
           nf->event.reg = SSAM_EVENT_REGISTRY_KIP;

           /** Specify which event to enable/disable **/
           nf->event.id.target_category = sdev->uid.category;
           nf->event.id.instance = sdev->uid.instance;

           /*
            - Specify for which events the notifier callback gets executed.
            - This essentially tells the core if it can skip notifiers that
            - don't have target or instance IDs matching those of the event.
            */
           nf->event.mask = SSAM_EVENT_MASK_STRICT;

           /** Specify event flags. **/
           nf->event.flags = SSAM_EVENT_SEQUENCED;

           return ssam_notifier_register(sdev->ctrl, nf);
   }

鍙互涓哄悓涓€浜嬩欢娉ㄥ唽澶氫釜浜嬩欢閫氱煡鍣ㄣ€備簨浠跺鐞嗘牳蹇冧細鍦ㄩ€氱煡鍣ㄦ敞鍐屽拰娉ㄩ攢鏃惰礋璐ｅ惎鐢ㄥ拰绂佺敤浜嬩欢锛屽叾鏂瑰紡鏄窡韪綋鍓嶄负鏌愪釜鐗瑰畾浜嬩欢锛堢敱娉ㄥ唽琛ㄣ€佷簨浠剁洰鏍囩被鍒€佷簨浠跺疄渚?ID 缁勫悎鑰屾垚锛夋敞鍐屼簡澶氬皯涓€氱煡鍣ㄣ€傝繖鎰忓懗鐫€锛岀壒瀹氫簨浠朵細鍦ㄥ叾绗竴涓€氱煡鍣ㄦ敞鍐屾椂琚惎鐢紝骞跺湪鍏舵渶鍚庝竴涓€氱煡鍣ㄦ敞閿€鏃惰绂佺敤銆傚洜姝わ紝浜嬩欢鏍囧織浠呭湪绗竴涓敞鍐岀殑閫氱煡鍣ㄤ笂鐢熸晥锛涗絾搴旀敞鎰忥紝閽堝鐗瑰畾浜嬩欢鐨勯€氱煡鍣ㄥ簲褰撳缁堜互鐩稿悓鐨勬爣蹇楁敞鍐岋紝鍚﹀垯琚涓轰竴涓己闄凤紙bug锛夈€?