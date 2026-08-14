
## 瀹屾暣鎬х瓥鐣ュ己鍒舵墽琛岋紙Integrity Policy Enforcement锛孖PE锛?


   This is the documentation for admins, system builders, or individuals
   attempting to use IPE. If you're looking for more developer-focused
   documentation about IPE please see [IPE 鐨勫紑鍙戞枃妗?</security/ipe>](the design docs </security/ipe>).

### 姒傝堪


瀹屾暣鎬х瓥鐣ュ己鍒舵墽琛岋紙Integrity Policy Enforcement锛孖PE锛夋槸涓€涓?Linux 瀹夊叏妯″潡
锛圠SM锛夛紝閲囩敤涓庝紶缁熻闂帶鍒朵簰琛ョ殑鏂瑰紡杩涜璁块棶鎺у埗銆備笌渚濊禆鏍囩鍜岃矾寰勫仛鍐崇瓥鐨?浼犵粺璁块棶鎺у埗鏈哄埗涓嶅悓锛孖PE 鑱氱劍浜庣郴缁熺粍浠舵墍鍥烘湁鐨勩€佷笉鍙彉鐨勫畨鍏ㄥ睘鎬с€傝繖浜涘睘鎬?鏄郴缁熺粍浠剁殑鍩烘湰灞炴€ф垨鐗瑰緛锛屾棤娉曡鏇存敼锛屼粠鑰屼负瀹夊叏鍐崇瓥鎻愪緵浜嗕竴鑷翠笖鍙潬鐨?鍩虹銆?
鍏蜂綋鑰岃█锛屽湪 IPE 鐨勮澧冧腑锛岀郴缁熺粍浠朵富瑕佹寚鏂囦欢鎴栬繖浜涙枃浠舵墍鍦ㄧ殑璁惧銆備笉杩囷紝杩?鍙槸涓€涓捣鐐广€傜郴缁熺粍浠剁殑姒傚康鏄伒娲荤殑锛屽彲浠ラ殢鐫€绯荤粺婕旇繘鑰屾墿灞曚互绾冲叆鏂扮殑鍏冪礌銆?涓嶅彲鍙樺睘鎬у寘鎷枃浠剁殑鏉ユ簮锛屽畠鍦ㄦ椂闂翠笂淇濇寔鎭掑畾涓斾笉鍙洿鏀广€備緥濡傦紝鍙互缂栧啓 IPE
绛栫暐鏉ヤ俊浠绘簮鑷?initramfs 鐨勬枃浠躲€傜敱浜?initramfs 閫氬父鐢卞紩瀵煎姞杞界▼搴忛獙璇侊紝鍏?鏂囦欢琚涓哄彲淇★紱鈥滄枃浠舵潵鑷?initramfs鈥濆湪 IPE 鐨勮€冮噺涓嬫垚涓轰竴涓笉鍙彉灞炴€с€?
涓嶅彲鍙樺睘鎬х殑姒傚康杩樺欢浼稿埌鏂囦欢鏉ユ簮涓婃墍鍚敤鐨勫畨鍏ㄧ壒鎬э紝渚嬪 dm-verity 鎴?fs-verity锛屽畠浠彁渚涗簡涓€灞傚畬鏁存€т笌淇′换淇濋殰銆備緥濡傦紝IPE 鍏佽瀹氫箟淇′换鏉ヨ嚜
dm-verity 淇濇姢璁惧鐨勬枃浠剁殑绛栫暐銆俤m-verity 閫氳繃鎻愪緵鍏跺唴瀹瑰彲楠岃瘉涓斾笉鍙彉鐨?鐘舵€佹潵纭繚鏁翠釜璁惧鐨勫畬鏁存€с€傜被浼煎湴锛宖s-verity 鎻愪緵鏂囦欢绯荤粺绾у埆鐨勫畬鏁存€ф鏌ワ紝
浣?IPE 鑳藉寮哄埗鎵ц淇′换鍙?fs-verity 淇濇姢鐨勬枃浠剁殑绛栫暐銆傝繖涓や釜鐗规€т竴鏃﹀缓绔嬪氨
鏃犳硶鍏抽棴锛屽洜姝ゅ畠浠瑙嗕负涓嶅彲鍙樺睘鎬с€傝繖浜涚ず渚嬪睍绀轰簡 IPE 濡備綍鍒╃敤涓嶅彲鍙樺睘鎬?锛堜緥濡傛枃浠剁殑鏉ユ簮鍙婂叾瀹屾暣鎬т繚鎶ゆ満鍒讹級鏉ュ仛鍑鸿闂帶鍒跺喅绛栥€?
鍏蜂綋鑰岃█锛屽浜?IPE 绛栫暐锛屽畠鍏峰閫氳繃灏嗗畨鍏ㄥ睘鎬т笌绛栫暐涓畾涔夌殑鍙傝€冨€艰繘琛屾瘮瀵规潵
寮哄埗瀹炴柦涓ユ牸璁块棶鎺у埗鐨勮兘鍔涖€傝繖绉嶈瘎浼板彲浠ュ熀浜庡畨鍏ㄥ睘鎬х殑瀛樺湪锛堜緥濡傦紝楠岃瘉鏌愪釜
鏂囦欢鏄惁婧愯嚜 initramfs锛夛紝鎴栬€呰瘎浼版煇涓笉鍙彉瀹夊叏灞炴€х殑鍐呴儴鐘舵€併€傚悗鑰呭寘鎷鏌?dm-verity 淇濇姢璁惧鐨?roothash銆佺‘瀹?dm-verity 鏄惁鎷ユ湁鏈夋晥鐨勭鍚嶃€佽瘎浼?fs-verity 淇濇姢鏂囦欢鐨?digest锛屾垨鑰呯‘瀹?fs-verity 鏄惁鎷ユ湁鏈夋晥鐨勫唴寤虹鍚嶃€傝繖绉?缁嗚嚧鐨勭瓥鐣ュ己鍒舵墽琛屾柟娉曞疄鐜颁簡楂樺害瀹夊叏涓斿彲瀹氬埗銆佸苟閽堝鐗瑰畾瀹夊叏闇€姹備笌淇′换妯″瀷
閲忚韩鎵撻€犵殑绯荤粺闃插尽鏈哄埗銆?
瑕佸惎鐢?IPE锛岃纭繚 `CONFIG_SECURITY_IPE`锛堜綅浜?`Security -> Integrity Policy Enforcement (IPE)`锛夐厤缃€夐」宸插惎鐢ㄣ€?
### 浣跨敤鍦烘櫙


IPE 鍦ㄥ浐瀹氬姛鑳借澶囷紙fixed-function devices锛変笂琛ㄧ幇鏈€浣筹細鍗抽偅浜涚敤閫旇鏄庣‘瀹氫箟
涓斾笉搴旇鏇存敼鐨勮澶囷紙渚嬪鏁版嵁涓績涓殑缃戠粶闃茬伀澧欒澶囥€両oT 璁惧绛夛級锛屽叾涓墍鏈?杞欢鍜岄厤缃兘鐢辩郴缁熸墍鏈夎€呮瀯寤轰笌鎻愪緵銆?
IPE 璺濈鐢ㄤ簬閫氱敤璁＄畻杩樺緢閬ヨ繙锛歀inux 绀惧尯鏁翠綋涓婂€惧悜浜庨伒寰幓涓績鍖栫殑淇′换妯″瀷
锛堝嵆浼楁墍鍛ㄧ煡鐨勪俊浠荤綉锛寃eb of trust锛夛紝鑰?IPE 鐩墠杩樹笉鏀寔瀹冦€傜浉鍙嶏紝IPE 鏀寔
PKI锛堝叕閽ュ熀纭€璁炬柦锛宲ublic key infrastructure锛夛紝瀹冮€氬父鎸囧畾涓€缁勬彁渚涙煇绉嶇粷瀵逛俊浠?鐨勫彲淇″疄浣撱€?
姝ゅ锛岃櫧鐒跺浠婂ぇ澶氭暟杞欢鍖呴兘缁忚繃绛惧悕锛屼絾杞欢鍖呭唴閮ㄧ殑鏂囦欢锛堜緥濡傚彲鎵ц鏂囦欢锛?寰€寰€鏈粡绛惧悕銆傝繖浣垮緱鍦ㄤ笉瀵瑰寘绠＄悊鍣ㄥ強鍏惰儗鍚庣殑鐢熸€佺郴缁熷仛閲嶅ぇ鏀瑰姩鐨勬儏鍐典笅锛屽緢闅?鍦ㄦ湡鏈涘寘绠＄悊鍣ㄥ彲鐢ㄧ殑绯荤粺涓埄鐢?IPE銆?
digest_cache LSM [#digest_cache_lsm]_ 鏄竴涓笌 IPE 缁撳悎浣跨敤鏃讹紝鍙敤浜庡惎鐢ㄥ苟鏀寔
閫氱敤璁＄畻浣跨敤鍦烘櫙鐨勭郴缁熴€?
### 宸茬煡闄愬埗


IPE 鏃犳硶楠岃瘉鍖垮悕鍙墽琛屽唴瀛樼殑瀹屾暣鎬э紝渚嬪鐢?gcc 闂寘鍜?libffi锛?3.4.2锛夊垱寤虹殑
trampoline锛屾垨 JIT 鐢熸垚鐨勪唬鐮併€傞仐鎲剧殑鏄紝鐢变簬杩欐槸鍔ㄦ€佺敓鎴愮殑浠ｇ爜锛孖PE 鏃犳硶纭繚
杩欎簺浠ｇ爜鐨勫畬鏁存€т互褰㈡垚淇′换鍩虹銆?
褰撹В閲婂瀷璇█缂栧啓鐨勭▼搴忛€氳繃灏嗙▼搴忔枃浠朵紶閫掔粰瑙ｉ噴鍣ㄦ潵璋冪敤鏃讹紝IPE 鏃犳硶楠岃瘉杩欎簺
绋嬪簭鐨勫畬鏁存€с€傝繖鏄洜涓鸿В閲婂櫒鎵ц杩欎簺鏂囦欢鐨勬柟寮忥細鑴氭湰鏈韩骞舵湭閫氳繃 IPE 鐨勬煇涓?閽╁瓙浣滀负鍙墽琛屼唬鐮佽璇勪及锛岃€屼粎浠呮槸琚鍙栫殑鏂囨湰鏂囦欢锛堜笌缂栬瘧鍚庣殑鍙墽琛屾枃浠剁浉瀵癸級銆?鐒惰€岋紝闅忕潃 `AT_EXECVE_CHECK` 鏍囧織鐨勫紩鍏ワ紙[AT_EXECVE_CHECK </userspace-api/check_exec>](AT_EXECVE_CHECK </userspace-api/check_exec>)锛夛紝
瑙ｉ噴鍣ㄥ彲浠ュ埄鐢ㄥ畠鍚戝唴鏍稿彂鍑轰俊鍙凤紝琛ㄦ槑鏌愪釜鑴氭湰鏂囦欢灏嗚鎵ц锛屽苟璇锋眰鍐呮牳瀵瑰叾鎵ц
LSM 瀹夊叏妫€鏌ャ€?
IPE 鐨?EXECUTE 鎿嶄綔寮哄埗鍦ㄧ紪璇戝悗鐨勫彲鎵ц鏂囦欢涓庤В閲婂瀷鑴氭湰涔嬮棿瀛樺湪宸紓锛氬浜庣紪璇?鍚庣殑鍙墽琛屾枃浠讹紝褰撳姞杞藉彲鎵ц鍐呭鏃讹紝寮哄埗鐢卞唴鏍稿湪 `execve()`銆乣execveat()`銆?`mmap()` 鍜?`mprotect()` 绯荤粺璋冪敤鏈熼棿鑷姩瑙﹀彂銆傚浜庤В閲婂瀷鑴氭湰锛屽己鍒堕渶瑕佽В閲婂櫒
浣跨敤甯?`AT_EXECVE_CHECK` 鏍囧織鐨?`execveat()` 杩涜鏄惧紡闆嗘垚銆備笌 IPE 鍦ㄦ墽琛岃繃绋嬩腑
鎷︽埅鐨?exec 绯荤粺璋冪敤涓嶅悓锛岃鏈哄埗闇€瑕佽В閲婂櫒涓诲姩閲囧彇琛屽姩锛岃€岀幇鏈夌殑瑙ｉ噴鍣ㄩ櫎闈炴坊鍔犱簡
璇ヤ俊鍙疯皟鐢紝鍚﹀垯涓嶄細琚嚜鍔ㄦ敮鎸併€?
### 濞佽儊妯″瀷


IPE 涓撻棬鐬勫噯鍐呮牳鍒濇鍚姩鍚庯紝瀵圭敤鎴风┖闂村彲鎵ц浠ｇ爜锛堝寘鎷€氳繃 `modprobe` 鎴?`insmod` 浠庣敤鎴风┖闂村姞杞界殑鍐呮牳妯″潡锛夎繘琛岀鏀圭殑椋庨櫓銆?
涓句緥鏉ヨ锛岃€冭檻杩欐牱涓€绉嶅満鏅細涓€涓彲鑳芥伓鎰忋€佷笉鍙椾俊浠荤殑浜岃繘鍒舵枃浠惰繛鍚屾墍鏈夊繀瑕佺殑
渚濊禆锛堝寘鎷姞杞藉櫒鍜?libc锛変竴璧疯涓嬭浇銆侷PE 鍦ㄦ璇涓嬬殑涓昏浣滅敤鏄樆姝㈡绫讳簩杩涘埗
鏂囦欢鍙婂叾渚濊禆鐨勬墽琛屻€?
IPE 閫氳繃鍏佽瀹冧滑杩愯涔嬪墠楠岃瘉鎵€鏈夊彲鎵ц浠ｇ爜鐨勫畬鏁存€т笌鐪熷疄鎬ф潵瀹炵幇杩欎竴鐐广€傚畠浼?杩涜褰诲簳鐨勬鏌ワ紝浠ョ‘淇濅唬鐮佺殑瀹屾暣鎬у畬濂斤紝骞朵笖瀹冧滑绗﹀悎鎵€瀹氫箟绛栫暐涓巿鏉冪殑鍙傝€冨€?锛坉igest銆佺鍚嶇瓑锛夈€傚鏋滀竴涓簩杩涘埗鏂囦欢鏈兘閫氳繃姝ら獙璇佽繃绋嬧€斺€旀棤璁烘槸鐢变簬鍏跺畬鏁存€?宸茶鐮村潖锛岃繕鏄笉婊¤冻鎺堟潈鏍囧噯鈥斺€擨PE 閮藉皢鎷掔粷鍏舵墽琛屻€傛澶栵紝IPE 浼氱敓鎴愬璁℃棩蹇楋紝
鍙敤浜庢娴嬪拰鍒嗘瀽鍥犵瓥鐣ヨ繚瑙勫鑷寸殑澶辫触銆?
绡℃敼濞佽儊鍦烘櫙鍖呮嫭鐢变竴绯诲垪鍙備笌鑰呭鍙墽琛屼唬鐮佽繘琛岀殑淇敼鎴栨浛鎹紝鍖呮嫭锛?
- 鑳藉鐗╃悊鎺ヨЕ纭欢鐨勫弬涓庤€?- 鑳藉鏈湴缃戠粶璁块棶绯荤粺鐨勫弬涓庤€?- 鑳藉璁块棶閮ㄧ讲绯荤粺鐨勫弬涓庤€?- 鍙楀閮ㄦ帶鍒剁殑琚敾鐮村唴閮ㄧ郴缁?- 绯荤粺鐨勬伓鎰忔渶缁堢敤鎴?- 琚敾鐮寸殑绯荤粺鏈€缁堢敤鎴?- 瀵圭郴缁熺殑杩滅▼锛堝閮級鏀荤牬

IPE 涓嶇紦瑙ｆ潵鑷伓鎰忎絾宸叉巿鏉冨紑鍙戣€咃紙鎷ユ湁绛惧悕璇佷功璁块棶鏉冮檺锛夌殑濞佽儊锛屼篃涓嶇紦瑙ｄ粬浠?鎵€浣跨敤鐨勮鏀荤牬鐨勫紑鍙戝伐鍏凤紙鍗抽潰鍚戣繑鍥炵紪绋嬫敾鍑伙紝return-oriented programming
attacks锛夈€傛澶栵紝IPE 鍦ㄧ敤鎴风┖闂翠笌鍐呮牳绌洪棿涔嬮棿鍒掑嚭浜嗕弗鏍肩殑瀹夊叏杈圭晫銆傚洜姝わ紝鍐呮牳
绾ф紡娲炲埄鐢ㄨ瑙嗕负瓒呭嚭 IPE 鐨勮寖鍥达紝缂撹В宸ヤ綔鐣欑粰鍏跺畠鏈哄埗銆?
### 绛栫暐


IPE 绛栫暐鏄竴绉嶇函鏂囨湰 [#devdoc]_ 绛栫暐锛岀敱璺ㄥ琛岀殑澶氭潯璇彞缁勬垚銆傚湪绛栫暐椤堕儴鏈変竴琛?蹇呴渶琛岋紝鎸囨槑绛栫暐鍚嶇О鍜岀瓥鐣ョ増鏈紝鐢ㄤ簬
```

   policy_name=Ex_Policy policy_version=0.0.0

```
绛栫暐鍚嶇О鏄竴涓敮涓€閿紝浠ヤ汉绫诲彲璇荤殑鍚嶇О鏍囪瘑姝ょ瓥鐣ャ€傚畠鐢ㄤ簬鍦?securityfs 涓嬪垱寤?鑺傜偣锛屽苟鍞竴鏍囪瘑绛栫暐浠ラ儴缃叉柊绛栫暐鎴栨洿鏂扮幇鏈夌瓥鐣ャ€?
绛栫暐鐗堟湰鎸囩ず绛栫暐鐨勫綋鍓嶇増鏈紙鑰岄潪绛栫暐璇硶鐗堟湰锛夈€傚畠鐢ㄤ簬闃叉灏嗙瓥鐣ュ洖婊氬埌鍙兘
涓嶅畨鍏ㄧ殑鏃х増鏈€?
IPE 绛栫暐鐨勪笅涓€閮ㄥ垎鏄鍒欙紙rules锛夈€傝鍒欑敱 key=value 瀵癸紙绉颁负灞炴€э紝properties锛?鏋勬垚銆侷PE 瑙勫垯闇€瑕佷袱涓睘鎬э細`action`锛屽畠鍐冲畾 IPE 鍦ㄥ尮閰嶅埌璇ヨ鍒欐椂鍋氫粈涔堬紱浠ュ強
`op`锛屽畠鍐冲畾搴斿湪浣曟椂璇勪及璇ヨ鍒欍€傞『搴忔槸鏈夋剰涔夌殑锛岃鍒欏繀椤讳互 `op` 寮€澶达紝骞朵互
```

   op=EXECUTE action=ALLOW

```
缁撳熬銆傛绀轰緥灏嗗厑璁镐换浣曟墽琛屻€傞澶栫殑灞炴€х敤浜庤瘎浼拌璇勪及鏂囦欢鐨勪笉鍙彉瀹夊叏灞炴€с€?杩欎簺灞炴€ф棬鍦ㄦ弿杩板唴鏍镐腑鑳藉鎻愪緵鏌愮瀹屾暣鎬ч獙璇佺殑绯荤粺锛屼娇寰?IPE 鑳藉鍩轰簬灞炴€х殑鍊?鏉ョ‘瀹氳祫婧愮殑淇′换搴︺€?
瑙勫垯鑷笂鑰屼笅璇勪及銆傚洜姝わ紝浠讳綍鎾ら攢瑙勫垯鎴栨嫆缁濊鍒欓兘搴旀斁鍦ㄦ枃浠堕潬鍓嶇殑浣嶇疆锛屼互纭繚
杩欎簺瑙勫垯鍦ㄥ甫鏈?`action=ALLOW` 鐨勮鍒欎箣鍓嶈璇勪及銆?
IPE 绛栫暐鏀寔娉ㄩ噴銆傚瓧绗?'#' 灏嗕綔涓烘敞閲婏紝蹇界暐 '#' 鍙充晶鐩村埌鎹㈣绗︿箣鍓嶇殑鎵€鏈夊瓧绗︺€?
IPE 璇勪及鐨勯粯璁よ涓轰篃鍙互鍦ㄧ瓥鐣ヤ腑閫氳繃 `DEFAULT` 璇彞鏉ヨ〃杈俱€傝繖鍙互鍦ㄥ叏灞€绾у埆瀹屾垚锛?```

   # Global
   DEFAULT action=ALLOW

   # Operation Specific
   DEFAULT op=EXECUTE action=ALLOW

```
蹇呴』涓?IPE 涓墍鏈夊凡鐭ユ搷浣滆缃粯璁ゅ€笺€傚鏋滀綘鎯充繚鎸佽緝鏃х瓥鐣ヤ笌鍙兘寮曞叆鏂版搷浣滅殑杈冩柊
鍐呮牳鍏煎锛岃璁剧疆涓€涓叏灞€榛樿鍊?`ALLOW`锛岀劧鍚庢寜鎿嶄綔閫愪釜瑕嗙洊榛樿鍊硷紙濡備笂鎵€绀猴級銆?
瀵逛簬鍙厤缃殑鍩轰簬绛栫暐鐨?LSM锛屽湪鍚姩鏃跺己鍒舵墽琛屽彲閰嶇疆绛栫暐銆佸洿缁曡鍙栧拰瑙ｆ瀽绛栫暐瀛樺湪
鑻ュ共闂锛?
1. 鍐呮牳**涓嶅簲**浠庣敤鎴风┖闂磋鍙栨枃浠讹紝鍥犳鐩存帴璇诲彇绛栫暐鏂囦欢鏄绂佹鐨勩€?2. 鍐呮牳鍛戒护琛屾湁瀛楃鏁伴檺鍒讹紝涓€涓唴鏍告ā鍧椾笉搴斾负鍏惰嚜韬厤缃繚鐣欐暣涓瓧绗﹂檺鍒躲€?3. 鍐呮牳鐢熸€佺郴缁熶腑鏈夊悇绉嶅悇鏍风殑寮曞鍔犺浇绋嬪簭锛屽洜姝や氦浠樹竴涓唴瀛樺潡灏嗘槸浠ｄ环楂樻槀銆侀毦浠?   缁存姢鐨勩€?
鍥犳锛孖PE 閫氳繃涓€涓О涓衡€滃惎鍔ㄧ瓥鐣モ€濓紙boot policy锛夌殑姒傚康瑙ｅ喅浜嗚繖涓棶棰樸€傚惎鍔ㄧ瓥鐣?鏄紪璇戣繘鍐呮牳鐨勬渶灏忕瓥鐣ャ€傝绛栫暐鏃ㄥ湪灏嗙郴缁熷甫鍏ョ敤鎴风┖闂村凡灏辩华銆佸彲浠ユ帴鏀跺懡浠ょ殑鐘舵€侊紝
姝ゆ椂鍙互閫氳繃 securityfs 閮ㄧ讲鏇村鏉傜殑绛栫暐銆傚惎鍔ㄧ瓥鐣ュ彲浠ラ€氳繃 `SECURITY_IPE_BOOT_POLICY`
閰嶇疆閫夐」鎸囧畾锛屽畠鎺ュ彈涓€涓寚鍚戣搴旂敤鐨?IPE 绛栫暐绾枃鏈増鏈殑璺緞銆傝绛栫暐灏嗚缂栬瘧杩?鍐呮牳銆傚鏋滄湭鎸囧畾锛孖PE 灏嗚绂佺敤锛岀洿鍒伴€氳繃 securityfs 閮ㄧ讲骞舵縺娲绘煇涓瓥鐣ャ€?
#### 閮ㄧ讲绛栫暐


绛栫暐鍙互閫氳繃 securityfs 浠庣敤鎴风┖闂撮儴缃层€傝繖浜涚瓥鐣ラ€氳繃 PKCS#7 娑堟伅鏍煎紡杩涜绛惧悕锛?浠ュ己鍒跺疄鐜版煇绉嶇▼搴︾殑绛栫暐鎺堟潈锛堢姝㈡敾鍑昏€呰幏寰椾笉鍙楃害鏉熺殑 root 鏉冮檺骞堕儴缃蹭竴涓?鈥渁llow all鈥濈瓥鐣ワ級銆傝繖浜涚瓥鐣ュ繀椤荤敱閾炬帴鍒?`SYSTEM_TRUSTED_KEYRING` 鐨勮瘉涔︾鍚嶏紝
鎴栬€呪€斺€斿鏋滃垎鍒惎鐢ㄤ簡 `CONFIG_IPE_POLICY_SIG_SECONDARY_KEYRING` 鍜?鎴?`CONFIG_IPE_POLICY_SIG_PLATFORM_KEYRING`鈥斺€旂敱娆＄骇鍜?鎴栧钩鍙板瘑閽ョ幆绛惧悕銆?```

   openssl smime -sign \
      -in "$MY_POLICY" \
      -signer "$MY_CERTIFICATE" \
      -inkey "$MY_PRIVATE_KEY" \
      -noattr \
      -nodetach \
      -nosmimecap \
      -outform der \
      -out "$MY_POLICY.p7b"

```
閮ㄧ讲绛栫暐鏄€氳繃 securityfs 鐨?`new_policy` 鑺傜偣瀹屾垚鐨勩€傝閮ㄧ讲绛栫暐锛屽彧闇€灏嗘枃浠?cat 鍒?```

   cat "$MY_POLICY.p7b" > /sys/kernel/security/ipe/new_policy

```
鎴愬姛鍚庯紝杩欏皢鍦?`/sys/kernel/security/ipe/policies/` 涓嬪垱寤轰竴涓瓙鐩綍銆傝瀛愮洰褰?灏嗘槸鎵€閮ㄧ讲绛栫暐鐨?`policy_name` 瀛楁锛屽洜姝ゅ浜庝笂闈㈢殑绀轰緥锛岀洰褰曞皢鏄?`/sys/kernel/security/ipe/policies/Ex_Policy`銆傝鐩綍涓皢鏈変竷涓枃浠讹細`pkcs7`銆?`policy`銆乣name`銆乣version`銆乣active`銆乣update` 鍜?`delete`銆?
`pkcs7` 鏂囦欢鏄彧璇荤殑銆傝鍙栧畠浼氳繑鍥炴彁渚涚粰鍐呮牳鐨勩€佷唬琛ㄨ绛栫暐鐨勫師濮?PKCS#7 鏁版嵁銆?濡傛灉璇诲彇鐨勭瓥鐣ユ槸鍚姩绛栫暐锛岀敱浜庡畠鏈粡绛惧悕锛岃繖灏嗚繑鍥?`ENOENT`銆?
`policy` 鏂囦欢鏄彧璇荤殑銆傝鍙栧畠浼氳繑鍥炵瓥鐣ョ殑 PKCS#7 鍐呴儴鍐呭锛屽嵆绾枃鏈瓥鐣ャ€?
`active` 鏂囦欢鐢ㄤ簬灏嗘煇涓瓥鐣ヨ缃负褰撳墠娲诲姩绛栫暐銆傝鏂囦欢鏄彲璇诲啓鐨勶紙rw锛夛紝鎺ュ彈鍊?`"1"` 浠ュ皢璇ョ瓥鐣ヨ涓烘椿鍔ㄣ€傜敱浜庡悓涓€鏃跺埢鍙兘鏈変竴涓瓥鐣ュ浜庢椿鍔ㄧ姸鎬侊紝鎵€鏈夊叾瀹冪瓥鐣?閮藉皢琚爣璁颁负涓嶆椿鍔ㄣ€傝鏍囪涓烘椿鍔ㄧ殑绛栫暐蹇呴』鍏锋湁澶т簬鎴栫瓑浜庡綋鍓嶈繍琛岀増鏈殑绛栫暐鐗堟湰銆?
`update` 鏂囦欢鐢ㄤ簬鏇存柊宸茬粡瀛樺湪浜庡唴鏍镐腑鐨勭瓥鐣ャ€傝鏂囦欢鏄彧鍐欑殑锛屾帴鍙椾竴涓?PKCS#7 绛惧悕鐨?绛栫暐銆傚皢濮嬬粓瀵规绛栫暐鎵ц涓ら」妫€鏌ワ細绗竴锛宍policy_names` 蹇呴』涓庢洿鏂扮増鏈拰鐜版湁鐗堟湰
鍖归厤銆傜浜岋紝鏇存柊鍚庣殑绛栫暐蹇呴』鍏锋湁澶т簬褰撳墠杩愯鐗堟湰鐨勭瓥鐣ョ増鏈€傝繖鏄负浜嗛槻姝㈠洖婊氭敾鍑汇€?
`delete` 鏂囦欢鐢ㄤ簬绉婚櫎涓嶅啀闇€瑕佺殑绛栫暐銆傝鏂囦欢鏄彧鍐欑殑锛屾帴鍙楀€?`1` 浠ュ垹闄よ绛栫暐銆?鍒犻櫎鏃讹紝浠ｈ〃璇ョ瓥鐣ョ殑 securityfs 鑺傜偣灏嗚绉婚櫎銆備笉杩囷紝鍒犻櫎褰撳墠娲诲姩绛栫暐鏄笉鍏佽鐨勶紝
浼氳繑鍥炴搷浣滀笉琚厑璁哥殑閿欒銆?
绫讳技鍦帮紝鍚?`update` 鍜?`new_policy` 鍐欏叆閮藉彲鑳藉鑷村潖娑堟伅锛堢瓥鐣ヨ娉曢敊璇級鎴栨枃浠?宸插瓨鍦ㄩ敊璇€傚悗鑰呭彂鐢熷湪灏濊瘯閮ㄧ讲涓€涓甫鏈?`policy_name` 鐨勭瓥鐣ワ紝鑰屽唴鏍稿凡缁忔湁涓€涓?鍏锋湁鐩稿悓 `policy_name` 鐨勫凡閮ㄧ讲绛栫暐鏃躲€?
閮ㄧ讲绛栫暐**涓嶄細**瀵艰嚧 IPE 寮€濮嬪己鍒舵墽琛岃绛栫暐銆侷PE 鍙細寮哄埗鎵ц琚爣璁颁负娲诲姩鐨勭瓥鐣ャ€?璇锋敞鎰忥紝鍚屼竴鏃跺埢鍙兘鏈変竴涓瓥鐣ュ浜庢椿鍔ㄧ姸鎬併€?
閮ㄧ讲鎴愬姛鍚庯紝鍙互閫氳繃鍐欏叆鏂囦欢
`/sys/kernel/security/ipe/policies/$policy_name/active` 鏉ユ縺娲昏绛栫暐銆?```

   echo 1 > "/sys/kernel/security/ipe/policies/Ex_Policy/active"

```
浠庝互涓婃椂鍒昏捣锛宍Ex_Policy` 鐜板湪灏辨垚涓虹郴缁熶笂琚己鍒舵墽琛岀殑绛栫暐銆?
IPE 涔熸彁渚涘垹闄ょ瓥鐣ョ殑鏂瑰紡銆傝繖鍙互閫氳繃 `delete` securityfs 鑺傜偣瀹屾垚锛?`/sys/kernel/security/ipe/policies/$policy_name/delete`銆?```

   echo 1 > "/sys/kernel/security/ipe/policies/$policy_name/delete"

```
鍒犻櫎绛栫暐鍙湁涓€涓姹傦細琚垹闄ょ殑绛栫暐蹇呴』澶勪簬涓嶆椿鍔ㄧ姸鎬併€?

   If a traditional MAC system is enabled (SELinux, apparmor, smack), all
   writes to ipe's securityfs nodes require `CAP_MAC_ADMIN`.

#### 妯″紡


IPE 鏀寔涓ょ杩愯妯″紡锛氬瀹规ā寮忥紙permissive锛岀被浼间簬 SELinux 鐨?permissive 妯″紡锛?鍜屽己鍒舵ā寮忥紙enforced锛夈€傚湪瀹藉妯″紡涓嬶紝鎵€鏈変簨浠堕兘浼氳妫€鏌ワ紝绛栫暐杩濊浼氳璁板綍锛屼絾
绛栫暐瀹為檯涓婂苟鏈寮哄埗鎵ц銆傝繖璁╃敤鎴疯兘澶熷湪寮哄埗鎵ц涔嬪墠娴嬭瘯绛栫暐銆?
榛樿妯″紡鏄己鍒讹紙enforce锛夛紝鍙互閫氳繃鍐呮牳鍛戒护琛屽弬鏁?`ipe.enforce=(0|1)`锛屾垨
securityfs 鑺傜偣 `/sys/kernel/security/ipe/enforce` 鏉ユ洿鏀广€?

   If a traditional MAC system is enabled (SELinux, apparmor, smack, etcetera),
   all writes to ipe's securityfs nodes require `CAP_MAC_ADMIN`.

#### 瀹¤浜嬩欢


##### 1420 AUDIT_IPE_ACCESS

```

   type=1420 audit(1653364370.067:61): ipe_op=EXECUTE ipe_hook=MMAP enforcing=1 pid=2241 comm="ld-linux.so" path="/deny/lib/libc.so.6" dev="sda2" ino=14549020 rule="DEFAULT action=DENY"
   type=1300 audit(1653364370.067:61): SYSCALL arch=c000003e syscall=9 success=no exit=-13 a0=7f1105a28000 a1=195000 a2=5 a3=812 items=0 ppid=2219 pid=2241 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=2 comm="ld-linux.so" exe="/tmp/ipe-test/lib/ld-linux.so" subj=unconfined key=(null)
   type=1327 audit(1653364370.067:61): 707974686F6E3300746573742F6D61696E2E7079002D6E00

   type=1420 audit(1653364735.161:64): ipe_op=EXECUTE ipe_hook=MMAP enforcing=1 pid=2472 comm="mmap_test" path=? dev=? ino=? rule="DEFAULT action=DENY"
   type=1300 audit(1653364735.161:64): SYSCALL arch=c000003e syscall=9 success=no exit=-13 a0=0 a1=1000 a2=4 a3=21 items=0 ppid=2219 pid=2472 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=2 comm="mmap_test" exe="/root/overlake_test/upstream_test/vol_fsverity/bin/mmap_test" subj=unconfined key=(null)
   type=1327 audit(1653364735.161:64): 707974686F6E3300746573742F6D61696E2E7079002D6E00

```
姝や簨浠惰〃绀?IPE 鍋氬嚭浜嗕竴涓闂帶鍒跺喅绛栵紱IPE 鐗瑰畾鐨勮褰曪紙1420锛夋€绘槸涓庝竴鏉?`AUDITSYSCALL` 璁板綍涓€璧峰彂鍑恒€?
鍙互閫氳繃 `AUDITSYSCALL` 璁板綍鐨?`success` 灞炴€у拰閫€鍑虹爜鏉ュ垽鏂?IPE 澶勪簬瀹藉妯″紡杩樻槸
寮哄埗妯″紡銆?

瀛楁鎻忚堪锛?
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| Field     | Value Type | Optional? | Description of Value                                                            |
+===========+============+===========+=================================================================================+
| ipe_op    | string     | No        | The IPE operation name associated with the log                                  |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| ipe_hook  | string     | No        | The name of the LSM hook that triggered the IPE event                           |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| enforcing | integer    | No        | The current IPE enforcing state 1 is in enforcing mode, 0 is in permissive mode |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| pid       | integer    | No        | The pid of the process that triggered the IPE event.                            |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| comm      | string     | No        | The command line program name of the process that triggered the IPE event       |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| path      | string     | Yes       | The absolute path to the evaluated file                                         |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| ino       | integer    | Yes       | The inode number of the evaluated file                                          |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| dev       | string     | Yes       | The device name of the evaluated file, e.g. vda                                 |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| rule      | string     | No        | The matched policy rule                                                         |
+-----------+------------+-----------+---------------------------------------------------------------------------------+

##### 1421 AUDIT_IPE_CONFIG_CHANGE


```

   type=1421 audit(1653425583.136:54): old_active_pol_name="Allow_All" old_active_pol_version=0.0.0 old_policy_digest=sha256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855 new_active_pol_name="boot_verified" new_active_pol_version=0.0.0 new_policy_digest=sha256:820EEA5B40CA42B51F68962354BA083122A20BB846F26765076DD8EED7B8F4DB auid=4294967295 ses=4294967295 lsm=ipe res=1
   type=1300 audit(1653425583.136:54): SYSCALL arch=c000003e syscall=1 success=yes exit=2 a0=3 a1=5596fcae1fb0 a2=2 a3=2 items=0 ppid=184 pid=229 auid=4294967295 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=4294967295 comm="python3" exe="/usr/bin/python3.10" key=(null)
   type=1327 audit(1653425583.136:54): PROCTITLE proctitle=707974686F6E3300746573742F6D61696E2E7079002D66002E2

```
姝や簨浠惰〃绀?IPE 灏嗘椿鍔ㄧ瓥鐣ヤ粠鏌愪竴涓垏鎹㈠埌浜嗗彟涓€涓紝骞堕檮甯︿袱涓瓥鐣ョ殑鐗堟湰涓庡搱甯?digest銆傛敞鎰?IPE 鍚屼竴鏃跺埢鍙兘鏈変竴涓椿鍔ㄧ瓥鐣ワ紝鎵€鏈夎闂喅绛栬瘎浼伴兘鍩轰簬褰撳墠娲诲姩绛栫暐銆?閮ㄧ讲鏂扮瓥鐣ョ殑姝ｅ父娴佺▼鏄厛灏嗗緟閮ㄧ讲绛栫暐鍔犺浇杩涘唴鏍革紝鐒跺悗鍐嶅皢娲诲姩绛栫暐鍒囨崲鍒板畠銆?
姝よ褰曟€绘槸涓?`write` 绯荤粺璋冪敤鐨?`AUDITSYSCALL` 璁板綍涓€璧峰彂鍑恒€?
瀛楁鎻忚堪锛?
+------------------------+------------+-----------+---------------------------------------------------+
| Field                  | Value Type | Optional? | Description of Value                              |
+========================+============+===========+===================================================+
| old_active_pol_name    | string     | Yes       | The name of previous active policy                |
+------------------------+------------+-----------+---------------------------------------------------+
| old_active_pol_version | string     | Yes       | The version of previous active policy             |
+------------------------+------------+-----------+---------------------------------------------------+
| old_policy_digest      | string     | Yes       | The hash of previous active policy                |
+------------------------+------------+-----------+---------------------------------------------------+
| new_active_pol_name    | string     | No        | The name of current active policy                 |
+------------------------+------------+-----------+---------------------------------------------------+
| new_active_pol_version | string     | No        | The version of current active policy              |
+------------------------+------------+-----------+---------------------------------------------------+
| new_policy_digest      | string     | No        | The hash of current active policy                 |
+------------------------+------------+-----------+---------------------------------------------------+
| auid                   | integer    | No        | The login user ID                                 |
+------------------------+------------+-----------+---------------------------------------------------+
| ses                    | integer    | No        | The login session ID                              |
+------------------------+------------+-----------+---------------------------------------------------+
| lsm                    | string     | No        | The lsm name associated with the event            |
+------------------------+------------+-----------+---------------------------------------------------+
| res                    | integer    | No        | The result of the audited operation(success/fail) |
+------------------------+------------+-----------+---------------------------------------------------+

##### 1422 AUDIT_IPE_POLICY_LOAD


```

   type=1422 audit(1653425529.927:53): policy_name="boot_verified" policy_version=0.0.0 policy_digest=sha256:820EEA5B40CA42B51F68962354BA083122A20BB846F26765076DD8EED7B8F4DB auid=4294967295 ses=4294967295 lsm=ipe res=1 errno=0
   type=1300 audit(1653425529.927:53): arch=c000003e syscall=1 success=yes exit=2567 a0=3 a1=5596fcae1fb0 a2=a07 a3=2 items=0 ppid=184 pid=229 auid=4294967295 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=4294967295 comm="python3" exe="/usr/bin/python3.10" key=(null)
   type=1327 audit(1653425529.927:53): PROCTITLE proctitle=707974686F6E3300746573742F6D61696E2E7079002D66002E2E

```
姝よ褰曡〃绀轰竴涓柊绛栫暐宸茶繛鍚岀瓥鐣ュ悕绉般€佺瓥鐣ョ増鏈拰绛栫暐鍝堝笇琚姞杞借繘鍐呮牳銆?
姝よ褰曟€绘槸涓?`write` 绯荤粺璋冪敤鐨?`AUDITSYSCALL` 璁板綍涓€璧峰彂鍑恒€?
瀛楁鎻忚堪锛?
+----------------+------------+-----------+-------------------------------------------------------------+
| Field          | Value Type | Optional? | Description of Value                                        |
+================+============+===========+=============================================================+
| policy_name    | string     | Yes       | The policy_name                                             |
+----------------+------------+-----------+-------------------------------------------------------------+
| policy_version | string     | Yes       | The policy_version                                          |
+----------------+------------+-----------+-------------------------------------------------------------+
| policy_digest  | string     | Yes       | The policy hash                                             |
+----------------+------------+-----------+-------------------------------------------------------------+
| auid           | integer    | No        | The login user ID                                           |
+----------------+------------+-----------+-------------------------------------------------------------+
| ses            | integer    | No        | The login session ID                                        |
+----------------+------------+-----------+-------------------------------------------------------------+
| lsm            | string     | No        | The lsm name associated with the event                      |
+----------------+------------+-----------+-------------------------------------------------------------+
| res            | integer    | No        | The result of the audited operation(success/fail)           |
+----------------+------------+-----------+-------------------------------------------------------------+
| errno          | integer    | No        | Error code from policy loading operations (see table below) |
+----------------+------------+-----------+-------------------------------------------------------------+

绛栫暐閿欒鐮侊紙errno锛夛細

浠ヤ笅琛ㄦ牸鍒楀嚭浜嗗湪鍔犺浇鎴栨洿鏂扮瓥鐣ユ椂鍙兘鍑虹幇鍦?errno 瀛楁涓殑閿欒鐮侊細

+----------------+--------------------------------------------------------+
| Error Code     | Description                                            |
+================+========================================================+
| 0              | Success                                                |
+----------------+--------------------------------------------------------+
| -EPERM         | Insufficient permission                                |
+----------------+--------------------------------------------------------+
| -EEXIST        | Same name policy already deployed                      |
+----------------+--------------------------------------------------------+
| -EBADMSG       | Policy is invalid                                      |
+----------------+--------------------------------------------------------+
| -ENOMEM        | Out of memory (OOM)                                    |
+----------------+--------------------------------------------------------+
| -ERANGE        | Policy version number overflow                         |
+----------------+--------------------------------------------------------+
| -EINVAL        | Policy version parsing error                           |
+----------------+--------------------------------------------------------+
| -ENOKEY        | Key used to sign the IPE policy not found in keyring   |
+----------------+--------------------------------------------------------+
| -EKEYREJECTED  | Policy signature verification failed                   |
+----------------+--------------------------------------------------------+
| -ESTALE        | Attempting to update an IPE policy with older version  |
+----------------+--------------------------------------------------------+
| -ENOENT        | Policy was deleted while updating                      |
+----------------+--------------------------------------------------------+

##### 1404 AUDIT_MAC_STATUS


```

   type=1404 audit(1653425689.008:55): enforcing=0 old_enforcing=1 auid=4294967295 ses=4294967295 enabled=1 old-enabled=1 lsm=ipe res=1
   type=1300 audit(1653425689.008:55): arch=c000003e syscall=1 success=yes exit=2 a0=1 a1=55c1065e5c60 a2=2 a3=0 items=0 ppid=405 pid=441 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=)
   type=1327 audit(1653425689.008:55): proctitle="-bash"

   type=1404 audit(1653425689.008:55): enforcing=1 old_enforcing=0 auid=4294967295 ses=4294967295 enabled=1 old-enabled=1 lsm=ipe res=1
   type=1300 audit(1653425689.008:55): arch=c000003e syscall=1 success=yes exit=2 a0=1 a1=55c1065e5c60 a2=2 a3=0 items=0 ppid=405 pid=441 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=)
   type=1327 audit(1653425689.008:55): proctitle="-bash"

```
姝よ褰曟€绘槸涓?`write` 绯荤粺璋冪敤鐨?`AUDITSYSCALL` 璁板綍涓€璧峰彂鍑恒€?
瀛楁鎻忚堪锛?
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| Field         | Value Type | Optional? | Description of Value                                                                            |
+===============+============+===========+=================================================================================================+
| enforcing     | integer    | No        | The enforcing state IPE is being switched to, 1 is in enforcing mode, 0 is in permissive mode   |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| old_enforcing | integer    | No        | The enforcing state IPE is being switched from, 1 is in enforcing mode, 0 is in permissive mode |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| auid          | integer    | No        | The login user ID                                                                               |
+---------------+------------+-----------+---------------------------------------------------------------------------------------------------+
| ses           | integer    | No        | The login session ID                                                                            |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| enabled       | integer    | No        | The new TTY audit enabled setting                                                               |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| old-enabled   | integer    | No        | The old TTY audit enabled setting                                                               |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| lsm           | string     | No        | The lsm name associated with the event                                                          |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| res           | integer    | No        | The result of the audited operation(success/fail)                                               |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+


##### 鎴愬姛瀹¤


IPE 鏀寔鎴愬姛瀹¤锛坰uccess auditing锛夈€傚惎鐢ㄥ悗锛屾墍鏈夐€氳繃 IPE 绛栫暐涓旀湭琚樆姝㈢殑浜嬩欢
閮戒細鍙戝嚭涓€鏉″璁′簨浠躲€傞粯璁ゆ儏鍐典笅姝ゅ姛鑳借绂佺敤锛屽彲浠ラ€氳繃鍐呮牳鍛戒护琛?`ipe.success_audit=(0|1)` 鎴?`/sys/kernel/security/ipe/success_audit`
securityfs 鏂囦欢鍚敤銆?
杩?*闈炲父**鍢堟潅锛屽洜涓?IPE 浼氭鏌ョ郴缁熶笂鐨勬瘡涓€涓敤鎴风┖闂翠簩杩涘埗鏂囦欢锛屼絾瀵硅皟璇曠瓥鐣?寰堟湁鐢ㄣ€?

   If a traditional MAC system is enabled (SELinux, apparmor, smack, etcetera),
   all writes to ipe's securityfs nodes require `CAP_MAC_ADMIN`.

### 灞炴€?

濡備笂鎵€杩帮紝IPE 灞炴€ф槸鍦?IPE 绛栫暐涓〃杈剧殑 `key=value` 瀵广€傛湁涓や釜灞炴€у唴寤猴紙built-in锛?浜庣瓥鐣ヨВ鏋愬櫒锛歚op` 鍜?`action`銆傚叾瀹冨睘鎬х敤浜庨檺鍒惰璇勪及鏂囦欢鐨勪笉鍙彉瀹夊叏灞炴€с€?鐩墠杩欎簺灞炴€ф槸锛?`boot_verified`'銆乣'dmverity_signature'`銆乣'dmverity_roothash'`銆?`'fsverity_signature'`銆乣'fsverity_digest'`銆侷PE 鏀寔鐨勬墍鏈夊睘鎬х殑鎻忚堪濡備笅锛?
#### op


鎸囩ず瑙勫垯鎵€閫傜敤鐨勬搷浣溿€傚繀椤讳綔涓烘瘡涓鍒欑殑绗竴涓爣璁板嚭鐜般€侷PE 鏀寔浠ヤ笅鎿嶄綔锛?
   `EXECUTE`

      涓庝换浣曡瘯鍥捐鎵ц銆佹垨浣滀负鍙墽琛屾枃浠跺姞杞界殑鏂囦欢鐩稿叧銆?
   `FIRMWARE`锛?
      涓庨€氳繃 firmware_class 鎺ュ彛鍔犺浇鐨勫浐浠剁浉鍏炽€傝繖鍚屾椂娑电洊棰勫垎閰嶇殑缂撳啿鍖哄拰
      鍥轰欢鏂囦欢鏈韩銆?
   `KMODULE`锛?
      涓庨€氳繃 `modprobe` 鎴?`insmod` 鍔犺浇鍐呮牳妯″潡鐩稿叧銆?
   `KEXEC_IMAGE`锛?
      涓庨€氳繃 `kexec` 鍔犺浇鍐呮牳鏄犲儚鐩稿叧銆?
   `KEXEC_INITRAMFS`

      涓庨€氳繃 `kexec --initrd` 鍔犺浇 initrd 鏄犲儚鐩稿叧銆?
   `POLICY`锛?
      閫氳繃鍐呮牳绌洪棿鍙戣捣鐨勮鍙栨潵鎺у埗绛栫暐鍔犺浇銆?
      姝ょ被鐨勪竴涓緥瀛愭槸閫氳繃灏嗙瓥鐣ユ枃浠惰矾寰勫啓鍏?`$securityfs/ima/policy` 鏉ュ姞杞?      IMA 绛栫暐銆?
   `X509_CERT`锛?
      閫氳繃 Kconfig `CONFIG_IMA_X509_PATH` 鍜?`CONFIG_EVM_X509_PATH` 鎺у埗
      鍔犺浇 IMA 璇佷功銆?
#### action


   鍐冲畾褰撹鍒欏尮閰嶆椂 IPE 搴旇鍋氫粈涔堛€傚繀椤讳綔涓烘瘡涓鍒欑殑鏈€鍚庝竴涓瓙鍙ュ嚭鐜般€傚彲浠?   鏄互涓嬩箣涓€锛?
   `ALLOW`锛?
      濡傛灉瑙勫垯鍖归厤锛屾樉寮忓厑璁哥户缁闂璧勬簮锛屼笉鍐嶆墽琛屾洿澶氳鍒欍€?
   `DENY`锛?
      濡傛灉瑙勫垯鍖归厤锛屾樉寮忕姝㈢户缁闂璧勬簮锛屼笉鍐嶆墽琛屾洿澶氳鍒欍€?
#### boot_verified


   姝ゅ睘鎬у彲鐢ㄤ簬鎺堟潈鏉ヨ嚜 initramfs 鐨勬枃浠躲€?```

         boot_verified=(TRUE|FALSE)


   .. WARNING::

      This property will trust files from initramfs(rootfs). It should
      only be used during early booting stage. Before mounting the real
      rootfs on top of the initramfs, initramfs script will recursively
      remove all files and directories on the initramfs. This is typically
      implemented by using switch_root(8) [#switch_root]_. Therefore the
      initramfs will be empty and not accessible after the real
      rootfs takes over. It is advised to switch to a different policy
      that doesn't rely on the property after this point.
      This ensures that the trust policies remain relevant and effective
      throughout the system's operation.

```
#### dmverity_roothash


   姝ゅ睘鎬у彲鐢ㄤ簬鎺堟潈鎴栨挙閿€鐗瑰畾鐨?dm-verity 鍗凤紝閫氳繃瀹冧滑鐨?root hash 杩涜鏍囪瘑銆?   瀹冧緷璧栦簬 DM_VERITY 妯″潡銆傛灞炴€х敱 `IPE_PROP_DM_VERITY` 閰嶇疆閫夐」鎺у埗锛屽綋
   `SECURITY_IPE` 鍜?`DM_VERITY` 閮藉惎鐢ㄦ椂浼氳嚜鍔ㄨ閫変腑銆?```

      dmverity_roothash=DigestName:HexadecimalString

   The supported DigestNames for dmverity_roothash are [#dmveritydigests]_

      + blake2b-512
      + blake2s-256
      + sha256
      + sha384
      + sha512
      + sha3-224
      + sha3-256
      + sha3-384
      + sha3-512
      + sm3
      + rmd160

```
#### dmverity_signature


   姝ゅ睘鎬у彲鐢ㄤ簬鎺堟潈鎵€鏈夋嫢鏈夌敱 dm-verity 閰嶇疆鎸囧畾鐨勫瘑閽ョ幆锛堣涔堟槸绯荤粺鍙俊瀵嗛挜鐜紝
   瑕佷箞鏄绾у瘑閽ョ幆锛夐獙璇佽繃鐨勭鍚?root hash 鐨?dm-verity 鍗枫€傚畠渚濊禆浜?   `DM_VERITY_VERIFY_ROOTHASH_SIG` 閰嶇疆閫夐」锛屽苟鐢?`IPE_PROP_DM_VERITY_SIGNATURE`
   閰嶇疆閫夐」鎺у埗锛屽綋 `SECURITY_IPE`銆乣DM_VERITY` 鍜?   `DM_VERITY_VERIFY_ROOTHASH_SIG` 閮藉惎鐢ㄦ椂浼氳嚜鍔ㄨ閫変腑銆?```

      dmverity_signature=(TRUE|FALSE)

```
#### fsverity_digest


   姝ゅ睘鎬у彲鐢ㄤ簬鎺堟潈鐗瑰畾鐨勩€佸惎鐢ㄤ簡 fs-verity 鐨勬枃浠讹紝閫氳繃瀹冧滑鐨?fs-verity digest
   杩涜鏍囪瘑銆傚畠渚濊禆浜?`FS_VERITY` 閰嶇疆閫夐」锛屽苟鐢?`IPE_PROP_FS_VERITY` 閰嶇疆閫夐」
   鎺у埗锛屽綋 `SECURITY_IPE` 鍜?`FS_VERITY` 閮藉惎鐢ㄦ椂浼氳嚜鍔ㄨ閫変腑銆?```

      fsverity_digest=DigestName:HexadecimalString

   The supported DigestNames for fsverity_digest are [#fsveritydigest]_

      + sha256
      + sha512

```
#### fsverity_signature


   姝ゅ睘鎬х敤浜庢巿鏉冩墍鏈夌敱 fs-verity 鍐呭缓绛惧悕鏈哄埗楠岃瘉杩囩殑銆佸惎鐢ㄤ簡 fs-verity 鐨勬枃浠躲€?   绛惧悕楠岃瘉渚濊禆浜庡瓨鍌ㄥ湪 ".fs-verity" 瀵嗛挜鐜腑鐨勫瘑閽ャ€傚畠渚濊禆浜?   `FS_VERITY_BUILTIN_SIGNATURES` 閰嶇疆閫夐」锛屽苟鐢?`IPE_PROP_FS_VERITY` 閰嶇疆閫夐」
   鎺у埗锛屽綋 `SECURITY_IPE`銆乣FS_VERITY` 鍜?`FS_VERITY_BUILTIN_SIGNATURES` 閮?   鍚敤鏃朵細鑷姩琚€変腑銆?```

      fsverity_signature=(TRUE|FALSE)

```
### 绛栫暐绀轰緥


#### 鍏佽鍏ㄩ儴


```

   policy_name=Allow_All policy_version=0.0.0
   DEFAULT action=ALLOW

```
#### 浠呭厑璁?initramfs


```

   policy_name=Allow_Initramfs policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE boot_verified=TRUE action=ALLOW

```
#### 鍏佽浠讳綍宸茬鍚嶄笖宸查獙璇佺殑 dm-verity 鍗蜂互鍙?initramfs


```

   policy_name=Allow_Signed_DMV_And_Initramfs policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE boot_verified=TRUE action=ALLOW
   op=EXECUTE dmverity_signature=TRUE action=ALLOW

```
#### 绂佹浠庣壒瀹氱殑 dm-verity 鍗锋墽琛?

```

   policy_name=Deny_DMV_By_Roothash policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE dmverity_roothash=sha256:cd2c5bae7c6c579edaae4353049d58eb5f2e8be0244bf05345bc8e5ed257baff action=DENY

   op=EXECUTE boot_verified=TRUE action=ALLOW
   op=EXECUTE dmverity_signature=TRUE action=ALLOW

```
#### 浠呭厑璁哥壒瀹氱殑 dm-verity 鍗?

```

   policy_name=Allow_DMV_By_Roothash policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE dmverity_roothash=sha256:401fcec5944823ae12f62726e8184407a5fa9599783f030dec146938 action=ALLOW

```
#### 鍏佽浠讳綍甯︽湁鏈夋晥鍐呭缓绛惧悕鐨?fs-verity 鏂囦欢


```

   policy_name=Allow_Signed_And_Validated_FSVerity policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE fsverity_signature=TRUE action=ALLOW

```
#### 鍏佽鎵ц鐗瑰畾鐨?fs-verity 鏂囦欢


```

   policy_name=ALLOW_FSV_By_Digest policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE fsverity_digest=sha256:fd88f2b8824e197f850bf4c5109bea5cf0ee38104f710843bb72da796ba5af9e action=ALLOW

```
### 闄勫姞淇℃伅


- `Github Repository <https://github.com/microsoft/ipe>`_
- [IPE 鐨勫紑鍙戜笌璁捐鏂囨。 </security/ipe>](Developer and design docs for IPE </security/ipe>)

### 甯歌闂锛團AQ锛?

Q:
   涓庡叾瀹冩彁渚涙煇绉嶅熀浜庝俊浠荤殑璁块棶鎺у埗鐨?LSM 鐩告瘮锛屽尯鍒湪鍝噷锛?
A:

   涓€鑸€岃█锛岃繕鏈夊彟澶栦袱涓?LSM 鑳芥彁渚涚被浼煎姛鑳斤細IMA 鍜?Loadpin銆?
   IMA 涓?IPE 鍦ㄥ姛鑳戒笂闈炲父鐩镐技銆備袱鑰呬箣闂寸殑鏄捐憲鍖哄埆鍦ㄤ簬绛栫暐銆俒#devdoc]_

   Loadpin 涓?IPE 鐨勫樊寮傜浉褰撳ぇ锛屽洜涓?Loadpin 鍙鐩?IPE 鐨勫唴鏍歌鍙栨搷浣滐紝鑰?IPE
   鑳藉鍦ㄥ唴鏍歌鍙栦箣涓婃帶鍒舵墽琛屻€備俊浠绘ā鍨嬩篃涓嶅悓锛汱oadpin 灏嗗叾淇′换鏍规浜庡垵濮嬭秴绾у潡
   锛坰uper-block锛夛紝鑰?IPE 鐨勪俊浠绘簮鑷唴鏍歌嚜韬紙閫氳繃 `SYSTEM_TRUSTED_KEYS`锛夈€?
-----------


             this topic.

                      the Linux crypto API; IPE does not impose any
                      restrictions on the digest algorithm itself;
                      thus, this list may be out of date.

                     kernel's fsverity support; IPE does not impose any
                     restrictions on the digest algorithm itself;
                     thus, this list may be out of date.
