
## 鐢ㄤ簬 USB Type-C 浜ゆ浛妯″紡锛圓lternate Mode锛夐┍鍔ㄧ殑 API


### 绠€浠?

浜ゆ浛妯″紡闇€瑕佷娇鐢?USB Type-C 涓?USB Power Delivery 瑙勮寖涓畾涔夌殑渚涘簲鍟嗗畾涔夋秷鎭紙VDM锛変笌瀵圭閫氫俊銆?璇ラ€氫俊鏄?SVID锛圫tandard 鎴?Vendor ID锛屾爣鍑嗘垨渚涘簲鍟?ID锛夌壒瀹氱殑锛屽嵆瀵规瘡涓氦鏇挎ā寮忛兘鏄壒瀹氱殑锛屽洜姝?姣忎釜浜ゆ浛妯″紡閮介渶瑕佷竴涓嚜瀹氫箟椹卞姩銆?
USB Type-C 鎬荤嚎鍏佽閫氳繃浣跨敤 SVID 鍜屾ā寮忓彿锛屽皢椹卞姩缁戝畾鍒拌鍙戠幇鐨勫绔氦鏇挎ā寮忋€?
USB Type-C Connector Class <typec> 涓虹鍙ｆ敮鎸佺殑姣忎釜浜ゆ浛妯″紡鎻愪緵涓€涓澶囷紝骞朵负瀵圭鏀寔鐨勬瘡涓氦鏇挎ā寮?鎻愪緵鍗曠嫭鐨勮澶囥€備氦鏇挎ā寮忕殑椹卞姩琚粦瀹氬埌瀵圭浜ゆ浛妯″紡璁惧锛岃€岀鍙ｄ氦鏇挎ā寮忚澶囧繀椤荤敱绔彛椹卞姩澶勭悊銆?
褰撲竴涓柊鐨勫绔氦鏇挎ā寮忚澶囪娉ㄥ唽鏃讹紝瀹冧細琚摼鎺ュ埌瀹冩墍杩炴帴绔彛鐨勩€佸叿鏈夊尮閰?SVID 鍜屾ā寮忕殑浜ゆ浛妯″紡璁惧銆?绔彛椹卞姩涓庝氦鏇挎ā寮忛┍鍔ㄤ箣闂寸殑閫氫俊灏嗕娇鐢ㄧ浉鍚岀殑 API 杩涜銆?
绔彛浜ゆ浛妯″紡璁惧琚敤浣滃绔笌浜ゆ浛妯″紡椹卞姩涔嬮棿鐨勪唬鐞嗭紝鍥犳绔彛椹卞姩鍙渶灏嗘潵鑷氦鏇挎ā寮忛┍鍔ㄧ殑銆丼VID 鐗瑰畾鐨?鍛戒护浼犻€掔粰瀵圭锛屼互鍙婂皢瀵圭鐨勫懡浠や紶閫掔粰浜ゆ浛妯″紡椹卞姩銆傜鍙ｉ┍鍔ㄤ笉闇€瑕佷换浣曠洿鎺ョ殑 SVID 鐗瑰畾閫氫俊锛屼絾绔彛椹卞姩
闇€瑕佹彁渚涚鍙ｄ氦鏇挎ā寮忚澶囩殑鎿嶄綔鍥炶皟锛屽氨鍍忎氦鏇挎ā寮忛┍鍔ㄩ渶瑕佷负瀵圭浜ゆ浛妯″紡璁惧鎻愪緵瀹冧滑涓€鏍枫€?
### 鐢ㄦ硶锛?

#### 涓€鑸?

榛樿鎯呭喌涓嬶紝浜ゆ浛妯″紡椹卞姩璐熻矗杩涘叆璇ユā寮忋€備篃鍙互灏嗚繘鍏ユā寮忕殑鍐崇瓥鐣欑粰鐢ㄦ埛绌洪棿锛堝弬瑙?`Documentation/ABI/testing/sysfs-class-typec`锛夈€傜鍙ｉ┍鍔ㄤ笉搴旇嚜琛岃繘鍏ヤ换浣曟ā寮忋€?
`->vdm` 鏄搷浣滃洖璋冨悜閲忎腑鏈€閲嶈鐨勫洖璋冦€傚畠灏嗙敤浜庢妸鏉ヨ嚜瀵圭鐨勩€佹墍鏈?SVID 鐗瑰畾鐨勫懡浠や紶閫掔粰浜ゆ浛妯″紡椹卞姩锛?瀵逛簬绔彛椹卞姩鍒欏弽涔嬨€傞┍鍔ㄤ箣闂翠娇鐢?`typec_altmode_vdm()` 浜掔浉鍙戦€?SVID 鐗瑰畾鐨勫懡浠ゃ€?
濡傛灉浣跨敤 SVID 鐗瑰畾鐨勫懡浠や笌瀵圭閫氫俊鐨勭粨鏋滈渶瑕侀噸鏂伴厤缃繛鎺ュ櫒涓婄殑寮曡剼锛屼氦鏇挎ā寮忛┍鍔ㄩ渶瑕佷娇鐢?`typec_altmode_notify()` 閫氱煡鎬荤嚎銆傞┍鍔ㄥ皢鍗忓晢寰楀埌鐨?SVID 鐗瑰畾寮曡剼閰嶇疆鍊间綔涓哄弬鏁颁紶閫掔粰璇ュ嚱鏁般€傛€荤嚎椹卞姩
闅忓悗灏嗕娇鐢ㄨ鍊间綔涓哄璺鐢ㄥ櫒锛坢ux锛夌殑鐘舵€佸€硷紝鏉ラ厤缃繛鎺ュ櫒鍚庨潰鐨勫璺鐢ㄥ櫒銆?
娉ㄦ剰锛歋VID 鐗瑰畾鐨勫紩鑴氶厤缃€煎繀椤诲缁堜粠 `TYPEC_STATE_MODAL` 寮€濮嬨€俇SB Type-C 瑙勮寖涓鸿繛鎺ュ櫒瀹氫箟浜嗕袱涓?榛樿鐘舵€侊細`TYPEC_STATE_USB` 鍜?`TYPEC_STATE_SAFE`銆傝繖浜涘€艰鎬荤嚎淇濈暀涓虹姸鎬佺殑鍓嶅嚑涓彲鑳藉€笺€傚綋杩涘叆浜ゆ浛妯″紡鏃讹紝
鎬荤嚎浼氬湪鍙戦€?USB Type-C 瑙勮寖瀹氫箟鐨?Enter 鎴?Exit Mode 鍛戒护涔嬪墠锛屽皢杩炴帴鍣ㄧ疆浜?`TYPEC_STATE_SAFE`锛屽苟鍦?妯″紡閫€鍑哄悗灏嗚繛鎺ュ櫒鏀惧洖 `TYPEC_STATE_USB`銆?
涓€涓?SVID 鐗瑰畾寮曡剼閰嶇疆鐨勫彲琛屽畾涔夌ず渚嬩负
```

    enum {
        ALTMODEX_CONF_A = TYPEC_STATE_MODAL,
        ALTMODEX_CONF_B,
        ...
    };

```
```

```
#define ALTMODEX_CONF_A = TYPEC_MODAL_STATE(0);
#define ALTMODEX_CONF_B = TYPEC_MODAL_STATE(1);

#### 绾跨紗鎻掑ご浜ゆ浛妯″紡


浜ゆ浛妯″紡椹卞姩涓嶄細琚粦瀹氬埌绾跨紗鎻掑ご浜ゆ浛妯″紡璁惧锛屽彧缁戝畾鍒板绔氦鏇挎ā寮忚澶囥€傚鏋滆浜ゆ浛妯″紡鏀寔鎴栬姹備竴鏉?鍝嶅簲 SOP Prime锛堜互鍙婂彲閫夌殑 SOP Double Prime锛夋秷鎭殑绾跨紗锛岃浜ゆ浛妯″紡鐨勯┍鍔ㄥ繀椤讳娇鐢?`typec_altmode_get_plug()`
璇锋眰绾跨紗鎻掑ご浜ゆ浛妯″紡鐨勫鐞嗗彞鏌勶紝骞舵帴绠″畠浠殑鎺у埗銆?
### 椹卞姩 API


#### 浜ゆ浛妯″紡缁撴瀯浣?

   :functions: typec_altmode_driver typec_altmode_ops

#### 浜ゆ浛妯″紡椹卞姩鐨勬敞鍐?娉ㄩ攢


   :functions: typec_altmode_register_driver typec_altmode_unregister_driver

#### 浜ゆ浛妯″紡椹卞姩鎿嶄綔


   :functions: typec_altmode_enter typec_altmode_exit typec_altmode_attention typec_altmode_vdm typec_altmode_notify

#### 鐢ㄤ簬绔彛椹卞姩鐨?API


   :functions: typec_match_altmode

#### 绾跨紗鎻掑ご鎿嶄綔


   :functions: typec_altmode_get_plug typec_altmode_put_plug
