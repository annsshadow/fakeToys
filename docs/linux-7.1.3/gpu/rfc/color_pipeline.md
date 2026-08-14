
## Linux 棰滆壊绠＄嚎 API


## 鎴戜滑瑕佽В鍐充粈涔堥棶棰橈紵


鎴戜滑甯屾湜鍦ㄦ樉绀烘帶鍒跺櫒纭欢涓敮鎸佹贩鍚堝墠涓庢贩鍚堝悗鐨勫鏉傞鑹插彉鎹紝浠ヤ究鏀寔鐢辩‖浠跺疄鐜扮殑 HDR 鐢ㄤ緥锛屽苟涓洪鑹茬鐞嗗簲鐢紙濡傝棰戞垨鍥惧儚缂栬緫鍣級鎻愪緵鏀寔銆?

鍦ㄦ敮鎸?Colorspace 涓?HDR Metadata drm_connector 灞炴€х殑纭欢涓婏紝鏄湁鍙兘鏀寔 HDR 杈撳嚭鐨勶紝浣嗛偅闇€瑕佸悎鎴愬櫒锛坈ompositor锛夋垨搴旂敤灏嗗唴瀹规覆鏌撳苟鍚堟垚涓轰竴涓渶缁堢敤浜庢樉绀虹殑缂撳啿鍖恒€傝繖鏍峰仛浠ｄ环楂樻槀銆?

澶у鏁扮幇浠ｆ樉绀虹‖浠舵彁渚涘悇绉?1D LUT銆?D LUT銆佺煩闃典互鍙婂叾浠栨搷浣滄潵鏀寔棰滆壊鍙樻崲銆傝繖浜涙搷浣滈€氬父瀹炵幇鍦ㄥ浐瀹氬姛鑳斤紙fixed-function锛夌‖浠朵腑锛屽洜姝ゆ瘮閫氳繃鐫€鑹插櫒鎴?CPU 鎵ц绫讳技鎿嶄綔瑕佽妭鑳藉緱澶氥€?

鎴戜滑甯屾湜鍒╃敤杩欑纭欢鍔熻兘锛屼互闆舵垨鏈€灏忕殑 CPU 鎴栫潃鑹插櫒璐熻浇鏀寔澶嶆潅鐨勯鑹插彉鎹€傚湪鍥哄畾鍔熻兘纭欢鍧椾笌鐫€鑹插櫒/CPU 涔嬮棿鐨勫垏鎹㈠繀椤绘槸鏃犵紳鐨勶紝褰撲换浣曟椂鍒婚渶瑕佸洖閫€鍒扮潃鑹插櫒/CPU 鏃讹紝閮戒笉搴旀湁鍙鐨勫樊寮傘€?


## 鍏朵粬鎿嶄綔绯荤粺鏄浣曡В鍐宠繖涓棶棰樼殑锛?


鏈€骞挎硾鏀寔鐨勭敤渚嬫秹鍙?HDR 鍐呭锛屾棤璁烘槸瑙嗛杩樻槸娓告垙銆?

澶у鏁版搷浣滅郴缁熶細鍚戦┍鍔ㄦ寚瀹氭簮鍐呭鏍煎紡锛堣壊鍩熴€佺紪鐮佷紶閫掑嚱鏁帮紝浠ュ強鍏朵粬鍏冩暟鎹紝濡傛渶澶т笌骞冲潎浜害绛夌骇锛夈€傞┍鍔ㄩ殢鍚庝細鐩稿簲鍦扮紪绋嬪叾鍥哄畾鍔熻兘纭欢锛屼互浠庢簮鍐呭缂撳啿鍖虹殑鑹插僵绌洪棿鏄犲皠鍒版樉绀哄櫒鐨勮壊褰╃┖闂淬€?

褰撳浐瀹氬姛鑳界‖浠朵笉鍙敤鏃讹紝鍚堟垚鍣ㄤ細缁勮涓€涓潃鑹插櫒锛岃姹?GPU 鎵ц浠庢簮鍐呭鏍煎紡鍒版樉绀哄櫒鏍煎紡鐨勫彉鎹€?

鍚堟垚鍣ㄧ殑鏄犲皠鍑芥暟涓庨┍鍔ㄥ櫒鐨勬槧灏勫嚱鏁伴€氬父鏄袱涓畬鍏ㄧ嫭绔嬬殑姒傚康銆傚湪纭欢鍘傚晢鏃犳硶浜嗚В闂簮鍚堟垚鍣ㄤ唬鐮佺殑閭ｄ簺鎿嶄綔绯荤粺涓婏紝鍘傚晢浼氳皟鏁村叾棰滆壊绠＄悊浠ｇ爜锛屼娇鍏跺湪瑙嗚涓婂尮閰嶅悎鎴愬櫒鐨勬晥鏋溿€傚湪鍏朵粬鎿嶄綔绯荤粺涓婏紝褰撲袱涓槧灏勫嚱鏁板瀹炵幇鑰呴兘寮€鏀炬椂锛屼粬浠細纭繚涓や釜鏄犲皠鐩稿尮閰嶃€?

杩欏鑷存槧灏勭畻娉曡閿佸畾锛屾剰鍛崇潃娌℃湁浜鸿兘澶熷崟鐙瘯楠屾垨寮曞叆鏂扮殑鏄犲皠绠楁硶锛屽苟鏃犺閲囩敤鍝瀹炵幇璺緞閮借幏寰椾竴鑷寸殑缁撴灉銆?

## 涓轰粈涔?Linux 涓嶅悓锛?


涓庡叾浠栨搷浣滅郴缁熶笂鈥滀竴涓┍鍔ㄥ搴斾竴涓悎鎴愬櫒銆佹垨涓€涓┍鍔ㄥ搴斿涓悎鎴愬櫒鈥濅笉鍚岋紝鍦?Linux 涓婃垜浠湁鐨勬槸澶氬澶氱殑鍏崇郴銆傝澶氬悎鎴愬櫒锛涜澶氶┍鍔ㄣ€傛澶栵紝姣忎釜鍚堟垚鍣ㄥ巶鍟嗘垨绀惧尯瀵逛簬棰滆壊绠＄悊搴斿綋濡備綍鍋氶兘鏈夎嚜宸辩殑鐪嬫硶銆傝繖姝ｆ槸 Linux 涔嬬編鎵€鍦ㄣ€?

杩欐剰鍛崇潃纭欢鍘傚晢鐜板湪涓嶈兘鍐嶆妸涓€涓┍鍔ㄨ皟鏁村埌涓庢煇涓€涓悎鎴愬櫒鍖归厤锛屽洜涓鸿皟鏁村埌鏌愪竴涓彲鑳借瀹冪湅璧锋潵涓庡彟涓€涓悎鎴愬櫒鐨勯鑹叉槧灏勭浉褰撲笉鍚屻€?

鎴戜滑闇€瑕佷竴涓洿濂界殑瑙ｅ喅鏂规銆?


## 鎻忚堪寮?API


涓€涓弿杩版簮涓庣洰鏍囪壊褰╃┖闂寸殑 API 鏄弿杩板紡锛坉escriptive锛堿PI銆傚畠鎻忚堪杈撳叆鍜岃緭鍑虹殑鑹插僵绌洪棿锛屼絾涓嶆弿杩板畠浠簲褰撳浣曡绮剧‘鏄犲皠銆傝繖鏍风殑鏄犲皠鍖呭惈璁稿缁嗗井鐨勮璁″喅绛栵紝浼氭瀬澶у湴褰卞搷鏈€缁堢粨鏋滅殑澶栬銆?

瑕佺敤瓒冲鐨勭粏鑺傛潵鎻忚堪杩欐牱鐨勬槧灏勩€佷互纭繚姣忎釜瀹炵幇寰楀埌鐩稿悓鐨勭粨鏋滐紝鏄笉鐜板疄鐨勩€備簨瀹炰笂锛岃繖浜涙槧灏勬槸涓€涓潪甯告椿璺冪殑鐮旂┒棰嗗煙銆?


## 瑙勫畾寮?API


瑙勫畾寮忥紙prescriptive锛堿PI 鎻忚堪鐨勪笉鏄簮涓庣洰鏍囪壊褰╃┖闂淬€傜浉鍙嶏紝瀹冭瀹氫簡涓€涓浣曞鐞嗗儚绱犲€间互寰楀埌鏈熸湜缁撴灉鐨勯厤鏂广€?

杩欎釜閰嶆柟閫氬父鏄竴涓湁搴忕殑绠€鍗曟搷浣滃垪琛紝鍏锋湁娓呮櫚鐨勬暟瀛﹀畾涔夛紝渚嬪 1D LUT銆?D LUT銆佺煩闃碉紝鎴栧叾浠栬兘澶熶互绮剧‘鏂瑰紡鎻忚堪鐨勬搷浣溿€?


## 棰滆壊绠＄嚎 API


纭欢棰滆壊绠＄悊绠＄嚎鍦ㄧ‖浠跺潡鐨勫彲鐢ㄦ€с€侀『搴忎笌鑳藉姏涓婏紝鍙兘鍦ㄧ‖浠跺巶鍟嗕箣闂存樉钁椾笉鍚屻€傝繖浣垮緱瀵归鑹茬鐞嗗潡鍙婂叾椤哄簭鐨勫叡鍚屽畾涔夊嚑涔庝笉鍙兘銆傚洜姝わ紝鎴戜滑瀹氫箟鐨勬槸涓€涓厑璁哥敤鎴风┖闂翠互閫氱敤鐨勬柟寮忋€佷笌鐗瑰畾椹卞姩鍜岀‖浠舵棤鍏冲湴鍙戠幇纭欢鑳藉姏鐨?API銆?


## drm_colorop 瀵硅薄


涓轰簡鏀寔棰滆壊绠＄嚎鐨勫畾涔夛紝鎴戜滑瀹氫箟浜?DRM 鏍稿績瀵硅薄绫诲瀷 drm_colorop銆傚悇涓?drm_colorop 瀵硅薄灏嗛€氳繃 drm_colorop 鐨?NEXT 灞炴€ч摼鎺ヨ捣鏉ワ紝鏋勬垚涓€涓鑹茬绾裤€傛瘡涓?drm_colorop 瀵硅薄鏄敮涓€鐨勶紝鍗筹紝鍗充娇澶氫釜棰滆壊绠＄嚎鎷ユ湁鐩稿悓鐨勬搷浣滐紝瀹冧滑涔熶笉浼氬叡浜悓涓€涓?drm_colorop 瀵硅薄鏉ユ弿杩拌鎿嶄綔銆?

娉ㄦ剰锛岄┍鍔ㄥ苟涓嶈鏈熸湜灏?drm_colorop 瀵硅薄闈欐€佹槧灏勫埌鐗瑰畾鐨勭‖浠跺潡銆俤rm_colorop 瀵硅薄鐨勬槧灏勫畬鍏ㄦ槸椹卞姩鍐呴儴鐨勭粏鑺傦紝鍙互濡傞┍鍔ㄦ墍闇€閭ｆ牱鍔ㄦ€佹垨闈欐€併€傝瑙佷笅鏂団€滈┍鍔ㄥ疄鐜拌€呮寚鍗椻€濅竴鑺傘€?

姣忎釜 drm_colorop 鏈変笁涓牳蹇冨睘鎬э細

TYPE锛氫竴涓灇涓惧睘鎬э紝瀹氫箟鍙樻崲鐨勭被鍨嬶紝渚嬪
- 鏋氫妇鏇茬嚎
- 鑷畾涔夛紙鍧囧寑锛?D LUT
- 3x3 鐭╅樀
- 3x4 鐭╅樀
- 3D LUT
- 绛夌瓑

鏍规嵁鍙樻崲绫诲瀷鐨勪笉鍚岋紝鍏朵粬灞炴€т細鎻忚堪鏇村缁嗚妭銆?

BYPASS锛氫竴涓竷灏斿睘鎬э紝鍙敤浜庤交鏉惧湴灏嗕竴涓潡缃簬鏃佽矾锛坆ypass锛夋ā寮忋€侭YPASS 灞炴€у colorop 涓嶆槸蹇呴』鐨勶紝鍙閫氳繃灏嗕竴涓?plane 涓婄殑 COLOR_PIPELINE 璁剧疆涓?'0' 鍙互鏃佽矾鏁翠釜绠＄嚎鍗冲彲銆?

NEXT锛氶鑹茬绾夸腑涓嬩竴涓?drm_colorop 鐨?ID锛屽鏋滆 drm_colorop 鏄摼涓殑鏈€鍚庝竴涓紝鍒欎负 0銆?

```
    /* 1D 鏋氫妇鏇茬嚎 */
    Color operation 42
    鈹溾攢 "TYPE": immutable enum {1D enumerated curve, 1D LUT, 3x3 matrix, 3x4 matrix, 3D LUT, etc.} = 1D enumerated curve
    鈹溾攢 "BYPASS": bool {true, false}
    鈹溾攢 "CURVE_1D_TYPE": enum {sRGB EOTF, sRGB inverse EOTF, PQ EOTF, PQ inverse EOTF, 鈥
    鈹斺攢 "NEXT": immutable color operation ID = 43

    /* 鑷畾涔?4k 鏉＄洰 1D LUT */
    Color operation 52
    鈹溾攢 "TYPE": immutable enum {1D enumerated curve, 1D LUT, 3x3 matrix, 3x4 matrix, 3D LUT, etc.} = 1D LUT
    鈹溾攢 "BYPASS": bool {true, false}
    鈹溾攢 "SIZE": immutable range = 4096
    鈹溾攢 "DATA": blob
    鈹斺攢 "NEXT": immutable color operation ID = 0

    /* 17^3 3D LUT */
    Color operation 72
    鈹溾攢 "TYPE": immutable enum {1D enumerated curve, 1D LUT, 3x3 matrix, 3x4 matrix, 3D LUT, etc.} = 3D LUT
    鈹溾攢 "BYPASS": bool {true, false}
    鈹溾攢 "SIZE": immutable range = 17
    鈹溾攢 "DATA": blob
    鈹斺攢 "NEXT": immutable color operation ID = 73
```
### drm_colorop 鍙墿灞曟€?


涓庣幇鏈夌殑 DRM 鏍稿績瀵硅薄锛堝 &drm_plane锛変笉鍚岋紝drm_colorop 涓嶅彲鎵╁睍銆傝繖绠€鍖栦簡瀹炵幇锛屽苟灏嗙鐞?&drm_colorop 瀵硅薄鐨勬墍鏈夊姛鑳戒繚鐣欏湪 DRM 鏍稿績涓€?

濡傛灉鏈夐渶瑕侊紝鏈潵鍙互寮曞叆涓€涓畝鍗曠殑 &drm_colorop_funcs 鍑芥暟琛紝渚嬪鐢ㄦ潵鏀寔 &drm_colorop 涓婄殑 IN_FORMATS 灞炴€с€?

濡傛灉椹卞姩闇€瑕佸垱寤洪┍鍔ㄧ壒瀹氱殑 colorop 瀵硅薄锛屼粬浠皢闇€瑕佹坊鍔?&drm_colorop func 琛ㄦ敮鎸侊紝骞舵敮鎸侀€氬父鐨勫嚱鏁帮紝濡?destroy銆乤tomic_duplicate_state 涓?atomic_destroy_state銆?


## COLOR_PIPELINE 骞抽潰灞炴€?


棰滆壊绠＄嚎鐢遍┍鍔ㄥ垱寤猴紝骞堕€氳繃姣忎釜骞抽潰锛坧lane锛変笂鐨勪竴涓柊 COLOR_PIPELINE 鏋氫妇灞炴€ф潵閫氬憡銆傝灞炴€х殑鍊煎缁堝寘鍚璞?id 0锛屽畠鏄粯璁ゅ€硷紝琛ㄧず绂佺敤鎵€鏈夐鑹插鐞嗐€傞澶栫殑鍊煎皢鏄绾夸腑绗竴涓?drm_colorop 鐨勫璞?ID銆備竴涓┍鍔ㄥ彲浠ュ垱寤哄苟閫氬憡闆朵釜銆佷竴涓垨鏇村鍙兘鐨勯鑹茬绾裤€備竴涓?DRM 瀹㈡埛绔皢閫氳繃鎶?COLOR PIPELINE 璁剧疆涓虹浉搴旂殑鍊兼潵閫夋嫨涓€鏉￠鑹茬绾裤€?

娉ㄦ剰锛氳澶?DRM 瀹㈡埛绔細閫氳繃瀛楃涓插€兼潵璁剧疆鏋氫妇灞炴€э紝甯稿父鏄‖缂栫爜鐨勩€傜敱浜庤繖涓灇涓炬槸鍩轰簬 colorop 瀵硅薄 ID 鐢熸垚鐨勶紝鍥犳鎵ц涓嬫枃鎻忚堪鐨勯鑹茬绾垮彂鐜帮紙Color Pipeline Discovery锛夛紝鑰屼笉鏄‖缂栫爜棰滆壊绠＄嚎鐨勫垎閰嶏紝鏄緢閲嶈鐨勩€傞┍鍔ㄥ彲鑳戒細鍔ㄦ€佺敓鎴愭灇涓惧瓧绗︿覆銆傜‖缂栫爜鐨勫瓧绗︿覆鍙兘鍙鐗瑰畾纭欢涓婄殑鐗瑰畾椹卞姩鏈夋晥銆傚彧瑕侀┍鍔ㄥ疄鐜颁簡鎵€闇€鐨勯鑹叉搷浣滐紝棰滆壊绠＄嚎鍙戠幇灏辫兘鏅亶宸ヤ綔銆?

COLOR_PIPELINE 灞炴€т粎鍦ㄨ缃簡 DRM_CLIENT_CAP_PLANE_COLOR_PIPELINE 鏃舵墠鏆撮湶銆傚綋璁剧疆浜嗘鑳藉姏鏃讹紝椹卞姩搴斿拷鐣ヤ换浣曞凡鏈夌殑娣峰悎鍓嶉鑹叉搷浣滐紝渚嬪 COLOR_RANGE 涓?COLOR_ENCODING銆傚鏋滈┍鍔ㄥ笇鏈涘湪棰滆壊绠＄嚎瀹㈡埛绔兘鍔涜璁剧疆鏃舵敮鎸?COLOR_RANGE 鎴?COLOR_ENCODING 鍔熻兘锛屼粬浠簲褰撻€氳繃鍦ㄧ绾夸腑鏆撮湶 colorop 鏉ュ厑璁哥浉搴旂殑棰滆壊鍙樻崲銆?

浠呭綋璁剧疆浜嗘瀹㈡埛绔兘鍔涚殑鐢ㄦ埛绌洪棿鎵嶅厑璁歌缃?COLOR_PIPELINE 骞抽潰灞炴€ф垨 drm_colorop 灞炴€с€?

```
    Plane 10
    鈹溾攢 "TYPE": immutable enum {Overlay, Primary, Cursor} = Primary
    鈹溾攢 鈥?
    鈹斺攢 "COLOR_PIPELINE": enum {0, 42, 52} = 0
```
## 棰滆壊绠＄嚎鍙戠幇


涓€涓笇鏈涘湪鏌?drm_plane 涓婅繘琛岄鑹茬鐞嗙殑 DRM 瀹㈡埛绔皢锛?

1. 鑾峰彇璇ュ钩闈㈢殑 COLOR_PIPELINE 灞炴€?
2. 閬嶅巻鎵€鏈?COLOR_PIPELINE 鏋氫妇鍊?
3. 瀵规瘡涓灇涓惧€兼部棰滆壊绠＄嚎閬嶅巻锛堥€氳繃 NEXT 鎸囬拡锛夛紝鏌ョ湅鍙敤鐨勯鑹叉搷浣滄槸鍚﹂€傚悎鏈熸湜鐨勯鑹茬鐞嗘搷浣?

濡傛灉鐢ㄦ埛鍦ㄥ彂鐜拌繃绋嬩腑閬囧埌鏈煡鎴栦笉鍚堥€傜殑棰滆壊鎿嶄綔锛屽畠鏃犻渶鐩存帴鎷掔粷鏁存潯棰滆壊绠＄嚎锛屽彧瑕佽鏈煡鎴栦笉鍚堥€傜殑 colorop 鏈変竴涓?鈥淏YPASS鈥?灞炴€с€傞┍鍔ㄥ皢纭繚琚梺璺殑鍧椾笉浼氫骇鐢熶换浣曟晥鏋溿€?

涓€涓敤浜庡畾涔?AMD 娣峰悎鍓嶉鑹茬绾跨殑閾惧紡灞炴€хず渚?
```
    Plane 10
    鈹溾攢 "TYPE" (immutable) = Primary
    鈹斺攢 "COLOR_PIPELINE": enum {0, 44} = 0

    Color operation 44
    鈹溾攢 "TYPE" (immutable) = 1D enumerated curve
    鈹溾攢 "BYPASS": bool
    鈹溾攢 "CURVE_1D_TYPE": enum {sRGB EOTF, PQ EOTF} = sRGB EOTF
    鈹斺攢 "NEXT" (immutable) = 45

    Color operation 45
    鈹溾攢 "TYPE" (immutable) = 3x4 Matrix
    鈹溾攢 "BYPASS": bool
    鈹溾攢 "DATA": blob
    鈹斺攢 "NEXT" (immutable) = 46

    Color operation 46
    鈹溾攢 "TYPE" (immutable) = 1D enumerated curve
    鈹溾攢 "BYPASS": bool
    鈹溾攢 "CURVE_1D_TYPE": enum {sRGB Inverse EOTF, PQ Inverse EOTF} = sRGB EOTF
    鈹斺攢 "NEXT" (immutable) = 47

    Color operation 47
    鈹溾攢 "TYPE" (immutable) = 1D LUT
    鈹溾攢 "SIZE": immutable range = 4096
    鈹溾攢 "DATA": blob
    鈹斺攢 "NEXT" (immutable) = 48

    Color operation 48
    鈹溾攢 "TYPE" (immutable) = 3D LUT
    鈹溾攢 "DATA": blob
    鈹斺攢 "NEXT" (immutable) = 49

    Color operation 49
    鈹溾攢 "TYPE" (immutable) = 1D enumerated curve
    鈹溾攢 "BYPASS": bool
    鈹溾攢 "CURVE_1D_TYPE": enum {sRGB EOTF, PQ EOTF} = sRGB EOTF
    鈹斺攢 "NEXT" (immutable) = 0
```
## 棰滆壊绠＄嚎缂栫▼


涓€鏃︿竴涓?DRM 瀹㈡埛绔壘鍒颁簡鍚堥€傜殑绠＄嚎锛屽畠灏嗭細

1. 灏?COLOR_PIPELINE 鏋氫妇鍊艰缃负鎸囧悜鏈熸湜绠＄嚎鐨勭涓€涓?drm_colorop 瀵硅薄鐨勯偅涓€?
2. 灏嗙绾夸腑鎵€鏈?drm_colorop 瀵硅薄鐨勫睘鎬ц缃负鏈熸湜鍊硷紝瀵规湭浣跨敤鐨?drm_colorop 鍧楀皢 BYPASS 璁句负 true锛屽鍚敤鐨?drm_colorop 鍧楄涓?false
3. 涓庡畠甯屾湜鏀瑰彉鐨勬墍鏈夊叾浠?KMS 鐘舵€佷竴璧锋墽琛岋紙TEST_ONLY 鎴栧惁锛夊師瀛愭彁浜わ紙atomic commit锛?

涓轰簡灏嗙绾块厤缃负 HDR10 PQ 骞抽潰骞跺湪绾挎€х┖闂存贩鍚堬紝涓€涓悎鎴愬櫒鍙兘浼氭墽琛屽涓嬪師瀛愭彁浜わ細
```
    Plane 10
    鈹斺攢 "COLOR_PIPELINE" = 42

    Color operation 42
    鈹斺攢 "BYPASS" = true

    Color operation 44
    鈹斺攢 "BYPASS" = true

    Color operation 45
    鈹斺攢 "BYPASS" = true

    Color operation 46
    鈹斺攢 "BYPASS" = true

    Color operation 47
    鈹溾攢 "DATA" = Gamut mapping + tone mapping + night mode
    鈹斺攢 "BYPASS" = false

    Color operation 48
    鈹溾攢 "CURVE_1D_TYPE" = PQ EOTF
    鈹斺攢 "BYPASS" = false
```
## 椹卞姩瀹炵幇鑰呮寚鍗?


杩欎竴鍒囧椹卞姩瀹炵幇鎰忓懗鐫€浠€涔堬紵濡備笂鎵€杩帮紝colorop 鍙互鐩存帴鏄犲皠鍒扮‖浠讹紝浣嗕笉闇€瑕佽繖鏍峰仛銆傝繖閲屾湁涓€浜涘叧浜庡浣曟€濊€冨垱寤轰綘鐨勯鑹茬绾跨殑寤鸿锛?

- 灏濊瘯鏆撮湶浣跨敤宸插畾涔?colorop 鐨勭绾匡紝鍗充究浣犵殑纭欢绠＄嚎鍒掑垎鏂瑰紡涓嶅悓銆傝繖璁╃幇鏈夌殑鐢ㄦ埛绌洪棿鑳藉绔嬪嵆鍒╃敤纭欢銆?

- 姝ゅ锛屽皾璇曞皢浣犲疄闄呯殑纭欢鍧椾綔涓?colorop 鏆撮湶鍑烘潵銆傚湪浣犺涓哄鏋滅敤鎴风┖闂村浼氱紪绋嬪畠浠氨鑳藉甫鏉ユ樉钁楀ソ澶勭殑鍦版柟锛屽畾涔夋柊鐨?colorop 绫诲瀷銆?

- 閬垮厤涓鸿寖鍥撮潪甯哥獎鐨勫鍚堟搷浣滃畾涔夋柊鐨?colorop銆傚鏋滀綘鏈変竴涓棤娉曡繘涓€姝ユ媶鍒嗙殑鐗规畩鎿嶄綔鐨勭‖浠跺潡锛屼綘鍙互灏嗗叾浣滀负涓€涓柊鐨?colorop 绫诲瀷鏆撮湶銆備絾鏄紝灏濊瘯涓嶈涓衡€滅敤渚嬧€濆畾涔?colorop锛屽挨鍏舵槸褰撳畠浠姹備綘缁勫悎澶氫釜纭欢鍧楁椂銆?

- 灏嗘柊鐨?colorop 璁捐涓鸿瀹氬紡鐨勮€岄潪鎻忚堪寮忕殑锛涗緷鎹暟瀛﹀叕寮忥紝鑰岄潪鍋囧畾鐨勮緭鍏ヤ笌杈撳嚭銆?

涓€涓凡瀹氫箟鐨?colorop 绫诲瀷蹇呴』鏄‘瀹氭€х殑銆俢olorop 鐨勭‘鍒囪涓哄繀椤昏瀹屾暣璁板綍锛屾棤璁烘槸閫氳繃鏁板鍏紡杩樻槸鍏朵粬鏌愮鎻忚堪銆傚畠鐨勬搷浣滃彧鑳戒緷璧栦簬瀹冪殑灞炴€у拰杈撳叆锛岃€屼笉渚濊禆鍏朵粬浠讳綍涓滆タ锛堝厑璁哥殑璇樊瀹归檺闄ゅ锛夈€?


## 椹卞姩鍓嶅悜/鍚庡悜鍏煎鎬?


鐢变簬杩欐槸 uAPI锛岄┍鍔ㄤ笉鑳戒娇宸茬粡涓虹粰瀹氱‖浠朵唬寮曞叆鐨勯鑹茬绾垮彂鐢熼€€鍖栵紙regress锛夈€傛柊鐨勭‖浠朵唬鍙互鑷敱鍦版姏寮冧负鍓嶄唬閫氬憡鐨勯鑹茬绾裤€備笉杩囷紝寤剁画瀵圭幇鏈夐鑹茬绾跨殑鏀寔鍙兘鏄湁鐩婄殑锛屽洜涓哄畠浠緢鍙兘宸茬粡鍦?DRM 瀹㈡埛绔腑鎷ユ湁鏀寔銆?

鍚戜竴鏉＄绾垮紩鍏ユ柊鐨?colorop 鏄彲浠ョ殑锛屽彧瑕佸畠浠彲浠ヨ鏃佽矾锛屾垨绾补鏄俊鎭€х殑銆傚疄鐜颁簡璇ョ绾挎敮鎸佺殑 DRM 瀹㈡埛绔€绘槸鍙互璺宠繃鏈煡灞炴€э紝鍙瀹冧滑鑳藉纭俊杩欐牱鍋氫笉浼氬鑷撮潪棰勬湡鐨勭粨鏋溿€?

濡傛灉涓€涓柊鐨?colorop 涓嶅睘浜庝笂杩扮被鍒箣涓€锛堝彲鏃佽矾鎴栦俊鎭€э級锛岄偅涔堜慨鏀瑰悗鐨勭绾垮鐢ㄦ埛绌洪棿灏嗘槸涓嶅彲鐢ㄧ殑銆傚湪杩欑鎯呭喌涓嬪簲褰撳畾涔変竴鏉℃柊鐨勭绾裤€?


## 鍙傝€冭祫鏂?


1. https://lore.kernel.org/dri-devel/QMers3awXvNCQlyhWdTtsPwkp5ie9bze_hD5nAccFW7a_RXlWjYB7MoUW_8CKLT2bSQwIXVi5H6VULYIxCdgvryZoAoJnC5lZgyK1QWn488=@emersion.fr/
