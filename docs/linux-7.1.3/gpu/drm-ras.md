
## 鍩轰簬 Generic Netlink 鐨?DRM RAS


DRM RAS锛圧eliability, Availability, Serviceability锛屽彲闈犳€с€佸彲鐢ㄦ€с€佸彲鏈嶅姟鎬э級鎺ュ彛涓?GPU/鍔犻€熷櫒
椹卞姩鎻愪緵浜嗕竴绉嶆爣鍑嗗寲鐨勬柟寮忥紝閫氳繃 Generic Netlink 鍚戠敤鎴风┖闂存毚闇查敊璇鏁板櫒鍙婂叾瀹冨彲闈犳€ц妭鐐广€?杩欎娇寰楄瘖鏂伐鍏枫€佺洃鎺у畧鎶よ繘绋嬫垨娴嬭瘯鍩虹璁炬柦鑳藉浠ョ粺涓€鐨勬柟寮忚法涓嶅悓 DRM 椹卞姩鏌ヨ纭欢鍋ュ悍鐘舵€併€?
涓昏鐩爣锛?
- 涓?GPU 涓庡姞閫熷櫒椹卞姩鎻愪緵鏍囧噯鍖栫殑 RAS 瑙ｅ喅鏂规锛屼互鏀寔鏁版嵁涓績鐩戞帶涓庡彲闈犳€ц繍缁淬€?- 瀹炵幇涓€涓崟涓€鐨?drm-ras Generic Netlink 绯诲垪锛屼互婊¤冻鐜颁唬 Netlink YAML 瑙勮寖锛屽苟灏嗘墍鏈?RAS 鐩稿叧
  閫氫俊闆嗕腑鍒板悓涓€鍛藉悕绌洪棿涓€?- 鏀寔鍩虹鐨勯敊璇鏁板櫒鎺ュ彛锛屾弧瓒冲綋鍓嶇揣杩笖蹇呰鐨勭洃鎺ч渶姹傘€?- 鎻愪緵鐏垫椿銆侀潰鍚戞湭鏉ョ殑鎺ュ彛锛屾湭鏉ュ彲鎵╁睍浠ユ敮鎸佸叾瀹冪被鍨嬬殑 RAS 鏁版嵁銆?- 鍏佽姣忎釜椹卞姩鎷ユ湁澶氫釜鑺傜偣锛屼娇椹卞姩鑳藉涓轰笉鍚岀殑 IP 鍧椼€佸瓙鍧楁垨鍏跺畠閫傜敤鐨勯€昏緫缁嗗垎鍗曞厓娉ㄥ唽鐙珛鐨?  鑺傜偣銆?
## 鑺傜偣


鑺傜偣鏄〃绀鸿澶囧唴閮ㄩ敊璇被鍨嬫垨閿欒鏉ユ簮鐨勯€昏緫鎶借薄銆傜洰鍓嶄粎鏀寔閿欒璁℃暟鍣ㄨ妭鐐广€?
椹卞姩璐熻矗閫氳繃 `drm_ras_node_register()` 涓?`drm_ras_node_unregister()` API 娉ㄥ唽鍜屾敞閿€鑺傜偣銆?
### 鑺傜偣绠＄悊


   :doc: DRM RAS Node Management
   :internal:

## Generic Netlink 鐢ㄦ硶


璇ユ帴鍙ｅ疄鐜颁负涓€涓悕涓?`drm-ras` 鐨?Generic Netlink 绯诲垪銆傜敤鎴风┖闂村伐鍏峰彲浠ワ細

- 浣跨敤 `list-nodes` 鍛戒护鍒楀嚭宸叉敞鍐岀殑鑺傜偣銆?- 浣跨敤 `get-error-counter` 鍛戒护锛屽苟浠?`node-id` 浣滀负鍙傛暟锛屽垪鍑烘煇涓妭鐐逛腑鐨勬墍鏈夐敊璇鏁板櫒銆?- 浣跨敤 `get-error-counter` 鍛戒护锛屽悓鏃朵互 `node-id` 涓?`error-id` 浣滀负鍙傛暟锛屾煡璇㈢壒瀹氱殑閿欒璁℃暟鍣ㄥ€笺€?
### 鍩轰簬 YAML 鐨勬帴鍙?

璇ユ帴鍙ｇ敱涓€涓?YAML 瑙勮寖 `Documentation/netlink/specs/drm_ras.yaml` 鎻忚堪銆?
姝?YAML 閫氳繃 `tools/net/ynl/pyynl/ynl_gen_c.py` 鑷姩鐢熸垚鐢ㄦ埛绌洪棿缁戝畾锛屽苟椹卞姩 netlink 灞炴€т笌
鎿嶄綔鐨勭粨鏋勩€?
### 浣跨敤璇存槑


- 鐢ㄦ埛绌洪棿蹇呴』棣栧厛鏋氫妇鑺傜偣浠ヨ幏鍙栧叾 ID銆?- 鑺傜偣 ID 鎴栬妭鐐瑰悕鍙敤浜庢墍鏈夊悗缁煡璇紝渚嬪閿欒璁℃暟鍣ㄣ€?- 閿欒璁℃暟鍣ㄥ彲浠ラ€氳繃閿欒 ID 鎴栭敊璇悕鏌ヨ銆?- 鏌ヨ鍙傛暟搴斿畾涔変负 uAPI 鐨勪竴閮ㄥ垎锛屼互纭繚鐢ㄦ埛鎺ュ彛鐨勭ǔ瀹氭€с€?- 璇ユ帴鍙ｆ敮鎸侀€氳繃娣诲姞鏂扮殑鑺傜偣绫诲瀷涓庨澶栧睘鎬ф潵鎵╁睍銆?
绀轰緥锛氫娇鐢?ynl 鍒楀嚭鑺傜偣


    sudo ynl --family drm_ras --dump list-nodes
    [{'device-name': '0000:03:00.0',
    'node-id': 0,
    'node-name': 'correctable-errors',
    'node-type': 'error-counter'},
    {'device-name': '0000:03:00.0',
     'node-id': 1,
     'node-name': 'uncorrectable-errors',
     'node-type': 'error-counter'}]

绀轰緥锛氫娇鐢?ynl 鍒楀嚭鎵€鏈夐敊璇鏁板櫒


    sudo ynl --family drm_ras --dump get-error-counter --json '{"node-id":0}'
    [{'error-id': 1, 'error-name': 'error_name1', 'error-value': 0},
    {'error-id': 2, 'error-name': 'error_name2', 'error-value': 0}]

绀轰緥锛氭煡璇㈡煇涓粰瀹氳妭鐐圭殑閿欒璁℃暟鍣?

    sudo ynl --family drm_ras --do get-error-counter --json '{"node-id":0, "error-id":1}'
    {'error-id': 1, 'error-name': 'error_name1', 'error-value': 0}
