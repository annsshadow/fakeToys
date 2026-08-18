## 绠€浠?

Linux DRM 灞傚寘鍚棬鍦ㄦ敮鎸佸鏉傚浘褰㈣澶囬渶姹傜殑浠ｇ爜锛岃繖绫昏澶囬€氬父鍖呭惈闈炲父閫傚悎 3D 鍥惧舰鍔犻€熺殑鍙紪绋嬫祦姘寸嚎銆?鍐呮牳涓殑鍥惧舰椹卞姩鍙互浣跨敤 DRM 鍑芥暟鏉ョ畝鍖栧唴瀛樼鐞嗐€佷腑鏂鐞嗗拰 DMA 绛変换鍔★紝骞跺悜搴旂敤绋嬪簭鎻愪緵缁熶竴鐨勬帴鍙ｃ€?
鍏充簬鐗堟湰鐨勮鏄庯細鏈寚鍗楁兜鐩?DRM 鏍戜腑鐨勭壒鎬э紝鍖呮嫭 TTM 鍐呭瓨绠＄悊鍣ㄣ€佽緭鍑洪厤缃笌妯″紡璁剧疆锛屼互鍙婃柊鐨?vblank 鍐呴儴鏈哄埗锛?姝ゅ杩樺寘鎷綋鍓嶅唴鏍镐腑鎵€鏈夌殑甯歌鐗规€с€?
[鍦ㄦ鎻掑叆鍏稿瀷 DRM 鍗忚鏍堝浘]

## 椋庢牸鎸囧崡


涓轰繚鎸佷竴鑷存€э紝鏈枃妗ｄ娇鐢ㄧ編寮忚嫳璇€傜缉鍐欏叏閮ㄤ互澶у啓瀛楁瘝涔﹀啓锛屼緥濡傦細DRM銆並MS銆両OCTL銆丆RTC 绛夈€?涓轰究浜庨槄璇伙紝鏂囨。鍏呭垎鍒╃敤 kerneldoc 鎻愪緵鐨勬爣璁板瓧绗︼細@parameter 琛ㄧず鍑芥暟鍙傛暟锛孈member 琛ㄧず缁撴瀯浣撴垚鍛橈紙鍚屼竴缁撴瀯浣撳唴锛夛紝
&struct 琛ㄧず寮曠敤缁撴瀯浣擄紝function() 琛ㄧず鍑芥暟銆傚鏋滆寮曠敤瀵硅薄鐨?kerneldoc 瀛樺湪锛岃繖浜涢兘浼氳嚜鍔ㄧ敓鎴愯秴閾炬帴銆?褰撳紩鐢ㄥ嚱鏁拌櫄琛紙浠ュ強涓€鑸殑缁撴瀯浣撴垚鍛橈級涓殑鏉＄洰鏃讹紝璇蜂娇鐢?&vtable_name.vfunc銆傞仐鎲剧殑鏄紝杩欑洰鍓嶅皻涓嶈兘鐢熸垚鎸囧悜璇ユ垚鍛樼殑鐩存帴閾炬帴锛屽彧鑳芥寚鍚戠粨鏋勪綋銆?
闄ょ壒娈婃儏鍐靛锛堢敤浜庡尯鍒嗗姞閿佷笌涓嶅姞閿佺殑鍙樹綋锛夛紝鍑芥暟鐨勫姞閿佽姹傚湪 kerneldoc 涓苟涓嶈杞姐€?鐩稿弽锛屽姞閿佸簲鍦ㄨ繍琛屾椂閫氳繃渚嬪 `WARN_ON(!mutex_is_locked(...));` 杩涜妫€鏌ャ€傜敱浜庢枃妗ｆ瘮杩愯鏃跺憡璀︽洿瀹规槗琚拷鐣ワ紝
杩欐牱鍋氭洿鏈変环鍊笺€傝€屼笖杩愯鏃舵鏌ュ湪鍔犻攣瑙勫垯鏀瑰彉鏃剁‘瀹為渶瑕佹洿鏂帮紝浠庤€屾彁楂樹簡鍏舵纭€с€傚湪鏂囨。涓紝
鍔犻攣瑙勫垯搴斿湪鐩稿叧缁撴瀯浣撲腑璇存槑锛氭棦鍙互鍦ㄩ攣鐨勬敞閲婁腑瑙ｉ噴鍏朵繚鎶や粈涔堬紝涔熷彲浠ヤ负鏁版嵁瀛楁娣诲姞鍏充簬鍝釜閿佷繚鎶ゅ畠鐨勮鏄庯紝鎴栦袱鑰呭吋鏈夈€?
鍏锋湁闈?`void` 杩斿洖鍊肩殑鍑芥暟搴旀湁涓€涓悕涓衡€淩eturns鈥濈殑灏忚妭锛岃鏄庝笉鍚屾儏鍐典笅鐨勯鏈熻繑鍥炲€煎強鍏跺惈涔夈€?鐩墠瀵逛簬璇ュ皬鑺傚悕绉版槸鍚﹀簲鍏ㄩ儴澶у啓銆佹槸鍚﹀簲浠ュ啋鍙风粨灏惧皻鏃犵粺涓€瀹氳銆傝閬靛惊鏂囦欢鏈湴鐨勯鏍笺€?鍏朵粬甯歌鐨勫皬鑺傚悕绉板寘鎷€淣otes鈥濓紙鍗遍櫓鎴栨鎵嬭竟鐣屾儏鍐电殑淇℃伅锛変互鍙娾€淔IXME鈥濓紙鎺ュ彛鍙竻鐞嗕箣澶勶級銆?
鍙﹁闃呰闈㈠悜鍐呮牳鏁翠綋鐨勬枃妗ｆ寚鍗?<doc_guide>銆?
### kAPI 鐨勬枃妗ｈ姹?

鎵€鏈夊鍑虹粰鍏朵粬妯″潡鐨?kernel API 閮藉繀椤荤紪鍐欐枃妗ｏ紝鍖呮嫭鍏舵暟鎹粨鏋勶紝浠ュ強鑷冲皯涓€涓畝鐭殑寮曡█灏忚妭鏉ヨВ閲婃暣浣撴蹇点€?鏂囨。搴斿敖鍙兘鏀惧湪浠ｇ爜鏈韩涓紝閲囩敤 kerneldoc 娉ㄩ噴鐨勫舰寮忋€?
涓嶈鐩茬洰鍦颁负鎵€鏈夊唴瀹圭紪鍐欐枃妗ｏ紝鑰屽彧璁板綍瀵归┍鍔ㄤ綔鑰呯浉鍏崇殑鍐呭锛歞rm.ko 鐨勫唴閮ㄥ嚱鏁颁互鍙婄‘瀹氭槸闈欐€佺殑鍑芥暟涓嶅簲鍏锋湁姝ｅ紡鐨?kerneldoc 娉ㄩ噴銆?濡傛灉璁や负闇€瑕佹敞閲婏紝璇蜂娇鐢ㄦ櫘閫氱殑 C 娉ㄩ噴銆備綘鍙互鍦ㄦ敞閲婁腑浣跨敤 kerneldoc 璇硶锛屼絾瀹冧笉搴斾互 /** kerneldoc 鏍囪寮€澶淬€?鏁版嵁缁撴瀯绫讳技锛岃鎸夌収鏂囨。鎸囧崡鐢?`/** private: **/` 娉ㄩ噴鏍囨敞瀹屽叏绉佹湁鐨勫唴瀹广€?
## 鍏ラ棬


娆㈣繋鏈夋剰鍙備笌 DRM 瀛愮郴缁熷紑鍙戠殑寮€鍙戣€呫€備汉浠粡甯镐細閽堝 checkpatch 鎴?sparse 鎶ュ憡鐨勫悇绉嶉棶棰樻彁浜よˉ涓併€傛垜浠杩庢绫昏础鐚€?
鎯宠鏇磋繘涓€姝ョ殑浜哄彲浠ュ湪 TODO 鍒楄〃 <todo> 涓婃壘鍒颁竴浠芥竻鐞嗕换鍔℃竻鍗曘€?
## 璐＄尞娴佺▼


DRM 瀛愮郴缁熷ぇ浣撲笂涓庡叾浠栧唴鏍稿瓙绯荤粺宸ヤ綔鏂瑰紡鐩稿悓锛屽弬瑙?:ref:`涓绘祦绋嬫寚鍗椾笌鏂囨。 <process_index>` 浜嗚В杩愪綔鏂瑰紡銆?姝ゅ鎴戜滑浠呰褰?GPU 瀛愮郴缁熺殑涓€浜涚壒娈婁箣澶勩€?
### 鐗规€у悎骞舵埅姝㈡椂闂?

鎵€鏈夌壒鎬у伐浣滃繀椤诲湪褰撳墠鍙戝竷鍛ㄦ湡鐨?-rc6 鐗堟湰鍓嶈繘鍏?linux-next 鏍戯紝鍚﹀垯蹇呴』鎺ㄨ繜锛屾棤娉曡繘鍏ヤ笅涓€涓悎骞剁獥鍙ｃ€?鎵€鏈夎ˉ涓佹渶杩熷繀椤诲湪 -rc7 鍓嶈繘鍏?drm-next 鏍戯紝浣嗗鏋滀綘鐨勫垎鏀笉鍦?linux-next 涓紝鍒欒繖蹇呴』鍦?-rc6 鍓嶅凡缁忓彂鐢熴€?
姝ゅ悗鍙厑璁哥己闄蜂慨澶嶏紙濡傚悓涓婃父鍚堝苟绐楀彛闅?-rc1 鍙戝竷鑰屽叧闂箣鍚庨偅鏍凤級銆備笉鍏佽鏂板骞冲彴鏀寔鎴栨柊鐨勯┍鍔ㄣ€?
杩欐剰鍛崇潃瀛樺湪涓€涓害涓€涓湀鐨勭壒鎬у伐浣滄棤娉曞悎骞剁殑灏佺鏈熴€傛帹鑽愮殑搴斿鏂瑰紡鏄淮鎶や竴涓缁堝紑鏀剧殑 -next 鏍戯紝
浣嗙‘淇濆湪灏佺鏈熷唴涓嶆妸瀹冨杺鍏?linux-next銆備緥濡?drm-misc 灏辨槸杩欐牱宸ヤ綔鐨勩€?
### 琛屼负鍑嗗垯


浣滀负 freedesktop.org 椤圭洰锛宒ri-devel 浠ュ強 DRM 绀惧尯閬靛惊璐＄尞鑰呭叕绾︼紙Contributor Covenant锛夛紝鍦板潃涓猴細
https://www.freedesktop.org/wiki/CodeOfConduct

鍦ㄩ偖浠跺垪琛ㄣ€両RC 鎴栫己闄疯窡韪櫒涓婁笌绀惧尯鎴愬憳浜ゆ祦鏃讹紝璇蜂繚鎸佸皧閲嶄笌鏂囨槑鐨勪妇姝€傜ぞ鍖轰唬琛ㄧ潃鏁翠釜椤圭洰锛?椤圭洰涓嶅蹇?abusive 鎴栨鍑岃涓恒€?
## 鍙敤浣滅ず渚嬬殑绠€鍗?DRM 椹卞姩


DRM 瀛愮郴缁熷寘鍚ぇ閲忚緟鍔╁嚱鏁帮紝浠ョ畝鍖栦负绠€鍗曞浘褰㈣澶囩紪鍐欓┍鍔ㄧ殑宸ヤ綔銆備緥濡傦紝`drivers/gpu/drm/tiny/` 鐩綍涓湁涓€缁?瓒冲绠€鍗曘€佸彲浠ョ敤鍗曚釜婧愭枃浠跺疄鐜扮殑椹卞姩銆倀iny DRM 椹卞姩鏄悊瑙?DRM 椹卞姩搴旀槸浠€涔堟牱瀛愮殑濂戒緥瀛愩€傜敱浜庡彧鏈夊嚑鐧捐浠ｇ爜锛屽畠浠浉褰撴槗璇汇€?
## 澶栭儴鍙傝€?

棣栨娣卞叆涓€涓?Linux 鍐呮牳瀛愮郴缁熷彲鑳芥槸涓€绉嶄护浜轰笉鐭ユ墍鎺殑浣撻獙锛岄渶瑕佺啛鎮夋墍鏈夋蹇靛苟浜嗚В璇ュ瓙绯荤粺鐨勫唴閮ㄦ満鍒剁瓑璇稿缁嗚妭銆?
涓轰簡骞崇紦瀛︿範鏇茬嚎锛屾湰鑺傚垪鍑轰竴浠藉彲鐢ㄤ簬瀛︿範 DRM/KMS 浠ュ強鍥惧舰涓€鑸煡璇嗙殑婕旇鍜屾枃妗ｆ竻鍗曘€?
浜轰滑鎯充簡瑙?DRM 鐨勫師鍥犲悇涓嶇浉鍚岋細绉绘鐜版湁鐨?fbdev 椹卞姩銆佷负鏂扮‖浠剁紪鍐?DRM 椹卞姩銆佷慨澶嶅湪澶勭悊鍥惧舰鐢ㄦ埛绌洪棿鍗忚鏍堟椂鍙兘閬囧埌鐨勭己闄风瓑銆?鍥犳锛屽涔犳潗鏂欐兜鐩栦簡 Linux 鍥惧舰鍗忚鏍堢殑璁稿鏂归潰锛屼粠鍐呮牳涓庣敤鎴风┖闂村崗璁爤鐨勬瑙堝埌闈炲父鍏蜂綋鐨勪富棰樸€?
娓呭崟鎸夋椂闂村€掑簭鎺掑垪锛屼互浣挎渶鏂扮殑鏉愭枡浣嶄簬椤堕儴銆備絾瀹冧滑閮藉寘鍚湁鐢ㄧ殑淇℃伅锛屾祻瑙堣緝鏃х殑鏉愭枡鏈夊姪浜庣悊瑙?DRM 瀛愮郴缁熸墍鍋氬彉鏇寸殑缂樼敱鍜岃儗鏅€?
### 浼氳婕旇


- `An Overview of the Linux and Userspace Graphics Stack <https://www.youtube.com/watch?v=wjAJmqwg47k>`_ - Paul Kocialkowski (2020)
- `Getting pixels on screen on Linux: introduction to Kernel Mode Setting <https://www.youtube.com/watch?v=haes4_Xnc5Q>`_ - Simon Ser (2020)
- `Everything Great about Upstream Graphics <https://www.youtube.com/watch?v=kVzHOgt6WGE>`_ - Simona Vetter (2019)
- `An introduction to the Linux DRM subsystem <https://www.youtube.com/watch?v=LbDOCJcDRoo>`_ - Maxime Ripard (2017)
- `Embrace the Atomic (Display) Age <https://www.youtube.com/watch?v=LjiB_JeDn2M>`_ - Simona Vetter (2016)
- `Anatomy of an Atomic KMS Driver <https://www.youtube.com/watch?v=lihqR9sENpc>`_ - Laurent Pinchart (2015)
- `Atomic Modesetting for Drivers <https://www.youtube.com/watch?v=kl9suFgbTc8>`_ - Simona Vetter (2015)
- `Anatomy of an Embedded KMS Driver <https://www.youtube.com/watch?v=Ja8fM7rTae4>`_ - Laurent Pinchart (2013)

### 骞荤伅鐗囦笌鏂囩珷


- `The Linux graphics stack in a nutshell, part 1 <https://lwn.net/Articles/955376/>`_ - Thomas Zimmermann (2023)
- `The Linux graphics stack in a nutshell, part 2 <https://lwn.net/Articles/955708/>`_ - Thomas Zimmermann (2023)
- `Understanding the Linux Graphics Stack <https://bootlin.com/doc/training/graphics/graphics-slides.pdf>`_ - Bootlin (2022)
- `DRM KMS overview <https://wiki.st.com/stm32mpu/wiki/DRM_KMS_overview>`_ - STMicroelectronics (2021)
- `Linux graphic stack <https://studiopixl.com/2017-05-13/linux-graphic-stack-an-overview>`_ - Nathan Gau毛r (2017)
- `Atomic mode setting design overview, part 1 <https://lwn.net/Articles/653071/>`_ - Simona Vetter (2015)
- `Atomic mode setting design overview, part 2 <https://lwn.net/Articles/653466/>`_ - Simona Vetter (2015)
- `The DRM/KMS subsystem from a newbie鈥檚 point of view <https://bootlin.com/pub/conferences/2014/elce/brezillon-drm-kms/brezillon-drm-kms.pdf>`_ - Boris Brezillon (2014)
- `A brief introduction to the Linux graphics stack <https://blogs.igalia.com/itoral/2014/07/29/a-brief-introduction-to-the-linux-graphics-stack/>`_ - Iago Toral (2014)
- `The Linux Graphics Stack <https://blog.mecheye.net/2012/06/the-linux-graphics-stack/>`_ - Jasper St. Pierre (2012)
