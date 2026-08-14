
## USB Type-C connector class


### Introduction


typec 绫伙紙class锛夋棬鍦ㄤ互缁熶竴鐨勬柟寮忓悜鐢ㄦ埛绌洪棿鎻忚堪绯荤粺涓殑 USB Type-C 绔彛銆傝绫?
琚璁′负闄ょ敤鎴风┖闂存帴鍙ｇ殑瀹炵幇涔嬪涓嶆彁渚涗换浣曞叾浠栧唴瀹癸紝甯屾湜瀹冭兘琚敖鍙兘澶氱殑骞冲彴鎵€
浣跨敤銆?

鍚勫钩鍙板簲灏嗗畠浠嫢鏈夌殑姣忎釜 USB Type-C 绔彛娉ㄥ唽鍒拌绫汇€傛甯告儏鍐典笅娉ㄥ唽鐢?USB Type-C
鎴?PD PHY 椹卞姩瀹屾垚锛屼絾涔熷彲鑳芥槸涓€涓浐浠舵帴鍙ｏ紙濡?UCSI锛夐┍鍔ㄣ€乁SB PD 鎺у埗鍣ㄩ┍鍔紝鐢氳嚦
Thunderbolt3 鎺у埗鍣ㄩ┍鍔ㄣ€傛湰鏂囨。灏嗗悜璇ョ被娉ㄥ唽 USB Type-C 绔彛鐨勭粍浠剁О涓衡€滅鍙ｉ┍鍔ㄢ€濄€?

闄や簡灞曠ず鑳藉姏锛屽綋绔彛椹卞姩鑳藉鏀寔杩欎簺鐗规€ф椂锛岃绫昏繕鎻愪緵瀵圭敤鎴风┖闂村绔彛銆佷紮浼达紙partner锛?
鍜岀嚎缂嗘彃澶达紙cable plug锛夌殑瑙掕壊涓庢浛浠ｆā寮忥紙Alternate Mode锛夌殑鎺у埗銆?

璇ョ被鎻愪緵浜嗘湰鏂囨。鎵€鎻忚堪鐨勭鍙ｉ┍鍔ㄦ墍鐢ㄧ殑 API銆傝繖浜涘睘鎬у湪
Documentation/ABI/testing/sysfs-class-typec 涓弿杩般€?

### User space interface

姣忎釜绔彛閮戒細浣滀负鑷繁鐨勮澶囧憟鐜板湪 /sys/class/typec/ 涓嬨€傜涓€涓鍙ｅ懡鍚嶄负鈥減ort0鈥濓紝
绗簩涓负鈥減ort1鈥濓紝渚濇绫绘帹銆?

杩炴帴鍚庯紝浼欎即涔熶細浣滀负鑷繁鐨勮澶囧憟鐜板湪 /sys/class/typec/ 涓嬨€備紮浼磋澶囩殑鐖惰澶囧缁堟槸
瀹冩墍杩炴帴鐨勭鍙ｃ€傝繛鎺ュ埌鈥減ort0鈥濈鍙ｇ殑浼欎即灏嗗懡鍚嶄负鈥減ort0-partner鈥濄€傝澶囩殑瀹屾暣璺緞涓?
/sys/class/typec/port0/port0-partner/銆?

绾跨紗鍙婂叾涓婄殑涓や釜鎻掑ご涔熷彲浠ラ€夋嫨鎬у湴浣滀负鑷繁鐨勮澶囧憟鐜板湪 /sys/class/typec/ 涓嬨€傝繛鎺ュ埌
鈥減ort0鈥濈鍙ｇ殑绾跨紗灏嗗懡鍚嶄负 port0-cable锛孲OP Prime 绔殑鎻掑ご锛堣 USB Power Delivery
瑙勮寖绗?2.4 绔狅級鍛藉悕涓衡€減ort0-plug0鈥濓紝SOP Double Prime 绔懡鍚嶄负鈥減ort0-plug1鈥濄€傜嚎缂嗙殑
鐖惰澶囧缁堟槸绔彛锛岀嚎缂嗘彃澶寸殑鐖惰澶囧缁堟槸绾跨紗銆?

濡傛灉绔彛銆佷紮浼存垨绾跨紗鎻掑ご鏀寔鏇夸唬妯″紡锛屾瘡涓彈鏀寔鐨勬浛浠ｆā寮?SVID 閮戒細鏈夎嚜宸辩殑璁惧鏉?
鎻忚堪瀹冦€傛敞鎰忔浛浠ｆā寮忚澶囦笉浼氭寕鎺ュ埌 typec 绫讳笅銆傛浛浠ｆā寮忕殑鐖惰澶囨槸鏀寔瀹冪殑璁惧锛屼緥濡?
port0-partner 鐨勪竴涓浛浠ｆā寮忎細鍛堢幇鍦?/sys/class/typec/port0-partner/ 涓嬨€傛瘡涓彈鏀寔鐨?
妯″紡鍦ㄦ浛浠ｆā寮忚澶囦笅閮芥湁鑷繁鐨勫悕涓衡€渕ode<index>鈥濈殑缁勶紝渚嬪
/sys/class/typec/port0/<alternate mode>/mode1/銆傝繘鍏?閫€鍑烘煇涓ā寮忕殑璇锋眰鍙互閫氳繃璇ョ粍涓?
鐨勨€渁ctive鈥濆睘鎬ф枃浠跺畬鎴愩€?

### Driver API


#### Registering the ports


绔彛椹卞姩浼氱敤 struct typec_capability 鏁版嵁缁撴瀯鎻忚堪瀹冧滑鎵€鎺у埗鐨勬瘡涓?Type-C 绔彛锛屽苟鐢?
浠ヤ笅 API 娉ㄥ唽瀹冧滑锛?

   :functions: typec_register_port typec_unregister_port

娉ㄥ唽绔彛鏃讹紝struct typec_capability 涓殑 prefer_role 鎴愬憳闇€瑕佺壒鍒敞鎰忋€傚鏋滄鍦ㄦ敞鍐岀殑
绔彛娌℃湁鍒濆瑙掕壊鍋忓ソ锛堝嵆璇ョ鍙ｉ粯璁や笉鎵ц Try.SNK 鎴?Try.SRC锛夛紝璇ユ垚鍛樺繀椤诲叿鏈夊€?
TYPEC_NO_PREFERRED_ROLE銆傚惁鍒欙紝濡傛灉绔彛榛樿鎵ц Try.SNK锛岃鎴愬憳蹇呴』鍏锋湁鍊?
TYPEC_DEVICE锛涜嫢鎵ц Try.SRC锛屽垯璇ュ€煎繀椤讳负 TYPEC_HOST銆?

#### Registering Partners


鍦ㄤ紮浼存垚鍔熻繛鎺ュ悗锛岀鍙ｉ┍鍔ㄩ渶瑕佸悜璇ョ被娉ㄥ唽璇ヤ紮浼淬€備紮浼寸殑璇︾粏淇℃伅闇€瑕佸湪 struct
typec_partner_desc 涓弿杩般€傝绫诲湪娉ㄥ唽鏈熼棿浼氬鍒朵紮浼寸殑璇︾粏淇℃伅銆傝绫绘彁渚涗互涓?API 鐢ㄤ簬
娉ㄥ唽/娉ㄩ攢浼欎即銆?

   :functions: typec_register_partner typec_unregister_partner

濡傛灉娉ㄥ唽鎴愬姛锛岃绫讳細鎻愪緵涓€涓寚鍚?struct typec_partner 鐨勫彞鏌勶紝鍚﹀垯涓?NULL銆?

濡傛灉浼欎即鏀寔 USB Power Delivery锛屼笖绔彛椹卞姩鑳藉灞曠ず Discover Identity 鍛戒护鐨勭粨鏋滐紝浼欎即
鎻忚堪绗︾粨鏋勫簲鍖呭惈鎸囧悜 struct usb_pd_identity 瀹炰緥鐨勫彞鏌勩€傝绫婚殢鍚庝細鍦ㄤ紮浼磋澶囦笅涓鸿韩浠?
淇℃伅鍒涘缓涓€涓?sysfs 鐩綍銆侱iscover Identity 鍛戒护鐨勭粨鏋滈殢鍚庡彲閫氳繃浠ヤ笅 API 涓婃姤锛?

   :functions: typec_partner_set_identity

#### Registering Cables


鍦ㄦ敮鎸?USB Power Delivery 缁撴瀯鍖?VDM鈥淒iscover Identity鈥濈殑绾跨紗鎴愬姛杩炴帴鍚庯紝绔彛椹卞姩闇€瑕?
娉ㄥ唽璇ョ嚎缂嗕互鍙婁竴涓垨涓や釜鎻掑ご锛屽叿浣撳彇鍐充簬绾跨紗涓槸鍚﹀瓨鍦?CC Double Prime 鎺у埗鍣ㄣ€傚洜姝わ紝
鏀寔 SOP Prime 閫氫俊浣嗕笉鏀寔 SOP Double Prime 閫氫俊鐨勭嚎缂嗗簲鍙敞鍐屼竴涓彃澶淬€傚叧浜?SOP 閫氫俊
鐨勬洿澶氫俊鎭紝璇烽槄璇绘渶鏂?USB Power Delivery 瑙勮寖涓殑鐩稿叧绔犺妭銆?

鎻掑ご浣滀负鑷繁鐨勮澶囪〃绀恒€傚厛娉ㄥ唽绾跨紗锛岀劧鍚庢敞鍐岀嚎缂嗘彃澶淬€傜嚎缂嗗皢鏄彃澶寸殑鐖惰澶囥€傜嚎缂嗙殑
璇︾粏淇℃伅闇€瑕佸湪 struct typec_cable_desc 涓弿杩帮紝鎻掑ご鐨勮缁嗕俊鎭湪 struct typec_plug_desc
涓弿杩般€傝绫诲湪娉ㄥ唽鏈熼棿浼氬鍒惰繖浜涜缁嗕俊鎭€傝绫绘彁渚涗互涓?API 鐢ㄤ簬娉ㄥ唽/娉ㄩ攢绾跨紗鍙婂叾鎻掑ご锛?

   :functions: typec_register_cable typec_unregister_cable typec_register_plug typec_unregister_plug

濡傛灉娉ㄥ唽鎴愬姛锛岃绫讳細鎻愪緵涓€涓寚鍚?struct typec_cable 鍜?struct typec_plug 鐨勫彞鏌勶紝鍚﹀垯
涓?NULL銆?

濡傛灉绾跨紗鏀寔 USB Power Delivery锛屼笖绔彛椹卞姩鑳藉灞曠ず Discover Identity 鍛戒护鐨勭粨鏋滐紝绾跨紗
鎻忚堪绗︾粨鏋勫簲鍖呭惈鎸囧悜 struct usb_pd_identity 瀹炰緥鐨勫彞鏌勩€傝绫婚殢鍚庝細鍦ㄧ嚎缂嗚澶囦笅涓鸿韩浠?
淇℃伅鍒涘缓涓€涓?sysfs 鐩綍銆侱iscover Identity 鍛戒护鐨勭粨鏋滈殢鍚庡彲閫氳繃浠ヤ笅 API 涓婃姤锛?

   :functions: typec_cable_set_identity

#### Notifications


褰撲紮浼存墽琛屼簡瑙掕壊鍒囨崲锛屾垨鑰呭湪杩炴帴浼欎即鎴栫嚎缂嗘湡闂撮粯璁よ鑹插彂鐢熷彉鍖栨椂锛岀鍙ｉ┍鍔ㄥ繀椤讳娇鐢?
浠ヤ笅 API 灏嗗叾鎶ュ憡缁欒绫伙細

   :functions: typec_set_data_role typec_set_pwr_role typec_set_vconn_role typec_set_pwr_opmode

#### Alternate Modes


USB Type-C 绔彛銆佷紮浼村拰绾跨紗鎻掑ご鍙兘鏀寔鏇夸唬妯″紡銆傛瘡涓浛浠ｆā寮忛兘鏈変竴涓О涓?SVID 鐨?
鏍囪瘑绗︼紝瀹冭涔堟槸 USB-IF 缁欏嚭鐨勬爣鍑?ID锛岃涔堟槸鍘傚晢 ID锛涙瘡涓彈鏀寔鐨?SVID 鍙互鏈?1鈥? 涓?
妯″紡銆傝绫绘彁渚?struct typec_mode_desc 鐢ㄤ簬鎻忚堪涓€涓?SVID 鐨勫崟涓ā寮忥紝浠ュ強 struct
typec_altmode_desc 浣滀负鎵€鏈夊彈鏀寔妯″紡鐨勫鍣ㄣ€?

鏀寔鏇夸唬妯″紡鐨勭鍙ｉ渶瑕佺敤浠ヤ笅 API 娉ㄥ唽瀹冧滑鏀寔鐨勬瘡涓?SVID锛?

   :functions: typec_port_register_altmode

濡傛灉浼欎即鎴栫嚎缂嗘彃澶翠互 USB Power Delivery 缁撴瀯鍖?VDM Discover SVIDs 娑堟伅鍝嶅簲骞舵彁渚涗簡 SVID
鍒楄〃锛屽垯姣忎釜 SVID 閮介渶瑕佹敞鍐屻€?

闈㈠悜浼欎即鐨?API锛?

   :functions: typec_partner_register_altmode

闈㈠悜绾跨紗鎻掑ご鐨?API锛?

   :functions: typec_plug_register_altmode

鎵€浠ョ鍙ｃ€佷紮浼村拰绾跨紗鎻掑ご浼氱敤鑷繁鐨勫嚱鏁版敞鍐屾浛浠ｆā寮忥紝浣嗘敞鍐屾垚鍔熸椂鎬绘槸杩斿洖涓€涓寚鍚?
struct typec_altmode 鐨勫彞鏌勶紝澶辫触鍒欎负 NULL銆傛敞閿€浼氫娇鐢ㄥ悓涓€涓嚱鏁帮細

   :functions: typec_unregister_altmode

濡傛灉浼欎即鎴栫嚎缂嗘彃澶磋繘鍏ユ垨閫€鍑烘煇涓ā寮忥紝绔彛椹卞姩闇€瑕佺敤浠ヤ笅 API 閫氱煡璇ョ被锛?

   :functions: typec_altmode_update_active

#### Multiplexer/DeMultiplexer Switches


USB Type-C 杩炴帴鍣ㄥ悗闈㈠彲鑳芥湁涓€涓垨澶氫釜 mux/demux 寮€鍏炽€傜敱浜庢彃澶村彲浠ユ鎻掓垨鍙嶆彃锛岄渶瑕?
涓€涓紑鍏冲皢鏉ヨ嚜杩炴帴鍣ㄧ殑姝ｇ‘鏁版嵁瀵硅矾鐢卞埌 USB 鎺у埗鍣ㄣ€傚鏋滄敮鎸佹浛浠ｆā寮忔垨閰嶄欢妯″紡锛岃繕闇€瑕?
鍙︿竴涓紑鍏筹紝灏嗚繛鎺ュ櫒涓婄殑寮曡剼璺敱鍒?USB 浠ュ鐨勫叾瀹冪粍浠躲€俇SB Type-C 杩炴帴鍣ㄧ被锛圕onnector
Class锛夋彁渚涙敞鍐岃繖浜涘紑鍏崇殑 API銆?

   :functions: typec_switch_register typec_switch_unregister typec_mux_register typec_mux_unregister

鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝鍚屼竴涓墿鐞?mux 浼氬悓鏃跺鐞嗘柟鍚戯紙orientation锛夊拰妯″紡锛坢ode锛夈€傜劧鑰岋紝鐢变簬
绔彛椹卞姩璐熻矗鏂瑰悜锛岃€屾浛浠ｆā寮忛┍鍔ㄨ礋璐ｆā寮忥紝浜岃€呮€绘槸琚垎绂讳负鍚勮嚜鐨勯€昏緫缁勪欢锛氣€渕ux鈥?瀵瑰簲
妯″紡锛屸€渟witch鈥?瀵瑰簲鏂瑰悜銆?

褰撶鍙ｆ敞鍐屾椂锛孶SB Type-C 杩炴帴鍣ㄧ被浼氳姹傝绔彛鐨?mux 鍜?switch銆傞┍鍔ㄩ殢鍚庡彲浠ョ敤浠ヤ笅 API
鎺у埗瀹冧滑锛?

   :functions: typec_set_orientation typec_set_mode

濡傛灉杩炴帴鍣ㄦ敮鎸佸弻瑙掕壊锛坉ual-role锛夛紝鍙兘杩樻湁涓€涓敤浜庢暟鎹鑹茬殑寮€鍏炽€俇SB Type-C 杩炴帴鍣ㄧ被
娌℃湁涓哄畠浠彁渚涘崟鐙殑 API銆傜鍙ｉ┍鍔ㄥ彲浠ヤ娇鐢?USB Role Class API 鏉ユ搷浣滃畠浠€?

```

                     ------------------------
                     |       Connector      |
                     ------------------------
                            |         |
                     ------------------------
                      \     Orientation    /
                       --------------------
                                |
                       --------------------
                      /        Mode        \
                     ------------------------
                         /              \
      ------------------------        --------------------
      |       Alt Mode       |       /      USB Role      \
      ------------------------      ------------------------
                                         /            \
                     ------------------------      ------------------------
                     |       USB Host       |      |       USB Device     |
                     ------------------------      ------------------------

```