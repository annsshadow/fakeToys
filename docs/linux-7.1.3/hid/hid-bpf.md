
## HID-BPF


HID 鏄緭鍏ヨ澶囩殑鏍囧噯鍗忚锛屼絾鏌愪簺璁惧鍙兘闇€瑕佸畾鍒跺寲璋冩暣锛屼紶缁熶笂閫氳繃鍐呮牳椹卞姩淇鏉ュ畬鎴愩€傛敼鐢?eBPF 鑳藉姏鍙互鍔犻€熷紑鍙戯紝骞朵负鐜版湁 HID 鎺ュ彛澧炴坊鏂拌兘鍔涖€?
    :local:
    :depth: 2


## 浣曟椂锛堜互鍙婁负浣曪級浣跨敤 HID-BPF


鍦ㄤ互涓嬭嫢骞插満鏅笅锛屼娇鐢?HID-BPF 浼樹簬鏍囧噯鐨勫唴鏍搁┍鍔ㄤ慨澶嶏細

### 娓告垙鎵嬫焺鐨勬鍖?
鍋囪浣犵殑鎵嬫焺宸茬粡姣旇緝鑰佹棫锛屽緢瀹规槗鐪嬪埌瀹冨湪涓珛鐐归檮杩戞姈鍔ㄣ€傝繖閫氬父鍦ㄥ簲鐢ㄥ眰閫氳繃涓鸿鐗瑰畾杞存坊鍔?*姝诲尯**鏉ヨ繃婊ゃ€?
鍊熷姪 HID-BPF锛屾垜浠彲浠ョ洿鎺ュ湪鍐呮牳涓繘琛岃繖绫昏繃婊わ紝杩欐牱褰撹緭鍏ユ帶鍒跺櫒涓婃病鏈夊叾瀹冧簨浠跺彂鐢熸椂锛屽氨涓嶄細鍞ら啋鐢ㄦ埛绌洪棿銆?
褰撶劧锛岀敱浜庤姝诲尯鏄拡瀵瑰叿浣撴煇涓澶囩殑锛屾垜浠棤娉曚负鎵€鏈夊悓鍨嬪彿鎵嬫焺鍒涘缓涓€涓€氱敤鐨勪慨澶嶃€備负姝ゆ柊澧炰竴涓唴鏍?API锛堜緥濡傛柊澧炰竴涓?sysfs 椤癸級骞朵笉鑳戒繚璇佽繖涓柊鐨勫唴鏍?API 浼氳骞挎硾閲囩敤鍜岀淮鎶ゃ€?
HID-BPF 鍏佽鐢ㄦ埛绌洪棿绋嬪簭鑷鍔犺浇璇ョ▼搴忥紝纭繚鎴戜滑鍙湪鏈夌敤鎴锋椂鎵嶅姞杞借鑷畾涔?API銆?
### 鎶ュ憡鎻忚堪绗︾殑绠€鍗曚慨姝?
鍦?HID 浠ｇ爜鏍戜腑锛屾湁涓€鍗婄殑椹卞姩浠呬粎鏄负浜嗕慨姝ｆ姤鍛婃弿杩扮涓殑涓€涓寜閿垨涓€涓瓧鑺傘€傝繖浜涗慨澶嶉兘闇€瑕佸唴鏍歌ˉ涓侊紝骞堕殢鍚庣粡鍘嗚繘鍏ュ彂琛岀増鐨勬极闀胯繃绋嬶紝瀵圭敤鎴疯€岃█鏃㈡极闀垮張鐥涜嫤銆?
鎴戜滑鍙互鏀逛负鎻愪緵涓€涓?eBPF 绋嬪簭鏉ュ噺杞昏繖绉嶈礋鎷呫€備竴鏃﹁绋嬪簭琚敤鎴烽獙璇侀€氳繃锛屾垜浠氨鍙互鎶婃簮浠ｇ爜宓屽叆鍐呮牳鏍戯紝鐩存帴闅忓唴鏍稿彂甯冨苟鍔犺浇璇?eBPF 绋嬪簭锛岃€屾棤闇€涓哄畠鍔犺浇鐗瑰畾鐨勫唴鏍告ā鍧椼€?
娉ㄦ剰锛歟BPF 绋嬪簭鐨勫垎鍙戝強鍏剁撼鍏ュ唴鏍稿皻鏈畬鍏ㄥ疄鐜般€?
### 鏂板闇€瑕佹柊鍐呮牳 API 鐨勭壒鎬?
杩欑被鐗规€х殑涓€涓緥瀛愭槸 USI锛圲niversal Stylus Interface锛岄€氱敤瑙︽帶绗旀帴鍙ｏ級瑙︽帶绗斻€傚熀鏈笂锛孶SI 瑙︽帶绗旈渶瑕佷竴涓柊鐨勫唴鏍?API锛屽洜涓哄畠鏈夋垜浠殑 HID 涓庤緭鍏ュ崗璁爤涓嶆敮鎸佺殑鏂伴€氫俊閫氶亾銆備笌鍏朵娇鐢?hidraw銆佸垱寤烘柊鐨?sysfs 椤规垨 ioctl锛屾垜浠彲浠ヤ緷璧?eBPF锛屼娇鍐呮牳 API 鐢变娇鐢ㄨ€呮帶鍒讹紝骞朵笖涓嶄細鍥犳瘡娆′簨浠堕兘鍞ら啋鐢ㄦ埛绌洪棿鑰屽奖鍝嶆€ц兘銆?
### 灏嗚澶囧彉褰负鍏朵粬褰㈡€佸苟浠庣敤鎴风┖闂存帶鍒?
鍐呮牳灏?HID 椤瑰埌 evdev 浣嶇殑鏄犲皠鐩稿闈欐€併€傚畠鏃犳硶鍔ㄦ€佸湴鎶婃煇涓粰瀹氳澶囪浆鎹㈡垚鍏朵粬褰㈡€侊紝鍥犱负瀹冪己灏戞墍闇€鐨勪笂涓嬫枃锛岃€屼笖浠讳綍杩欐牱鐨勮浆鎹㈤兘鏃犳硶琚敤鎴风┖闂存挙閿€锛堢敋鑷冲彂鐜帮級銆?
鐒惰€岋紝鏌愪簺璁惧鍦ㄨ繖绉嶉潤鎬佸畾涔夋柟寮忎笅姣棤鐢ㄥ銆備緥濡傦紝Microsoft Surface Dial 鏄竴涓甫瑙﹁鍙嶉鐨勬寜閽紝鐩墠鍑犱箮涓嶅彲鐢ㄣ€?
鍊熷姪 eBPF锛岀敤鎴风┖闂村彲浠ユ妸璇ヨ澶囧彉褰负榧犳爣锛屽苟灏嗘嫧鐩樹簨浠惰浆鎹负婊氳疆浜嬩欢銆傛澶栵紝鐢ㄦ埛绌洪棿绋嬪簭鍙互鏍规嵁涓婁笅鏂囪缃?鍙栨秷瑙﹁鍙嶉銆備緥濡傦紝濡傛灉灞忓箷涓婂彲瑙佹煇涓彍鍗曪紝鎴戜滑鍙兘闇€瑕佸湪姣?15 搴︿骇鐢熶竴娆¤Е瑙夌偣鍑伙紱鑰屽湪缃戦〉涓粴鍔ㄦ椂锛岃澶囧湪鏈€楂樺垎杈ㄧ巼涓嬪彂鍑轰簨浠朵細甯︽潵鏇村ソ鐨勭敤鎴蜂綋楠屻€?
### 闃茬伀澧?
濡傛灉鎴戜滑鎯抽樆姝㈠叾浠栫敤鎴疯闂澶囩殑鏌愪釜鐗瑰畾鍔熻兘鎬庝箞鍔烇紵锛堟兂鎯虫煇涓彲鑳藉凡鎹熷潖鐨勫浐浠舵洿鏂板叆鍙ｇ偣銆傦級

鍊熷姪 eBPF锛屾垜浠彲浠ユ嫤鎴彂寰€璁惧鐨勪换浣?HID 鍛戒护锛屽苟鍐冲畾鏄惁鏀捐銆?
杩欎篃鍏佽鍦ㄧ敤鎴风┖闂翠笌鍐呮牳/BPF 绋嬪簭涔嬮棿鍚屾鐘舵€侊紝鍥犱负鎴戜滑鍙互鎷︽埅浠讳綍浼犲叆鐨勫懡浠ゃ€?
### 杩借釜

鏈€鍚庣殑鐢ㄩ€旀槸杩借釜浜嬩欢锛屼互鍙婃垜浠€熷姪 BPF 瀵逛簨浠惰繘琛屾眹鎬诲拰鍒嗘瀽鎵€鑳藉仛鐨勭绉嶆湁瓒ｄ箣浜嬨€?
鐩墠锛岃拷韪緷璧?hidraw銆傚畠宸ヤ綔鑹ソ锛屼絾瀛樺湪鍑犱釜闂锛?
1. 濡傛灉椹卞姩娌℃湁瀵煎嚭 hidraw 鑺傜偣锛屾垜浠氨鏃犳硶杩借釜浠讳綍鍐呭锛坋BPF 鍦ㄩ偅閲屽皢澶勪簬鈥滀笂甯濇ā寮忊€濓紝杩欏彲鑳戒細寮曡捣涓€浜涗汉鐨勯【铏戯級銆?2. hidraw 鏃犳硶鎹曡幏鍏朵粬杩涚▼瀵硅澶囩殑璇锋眰锛岃繖鎰忓懗鐫€鎴戜滑鏈夋椂闇€瑕佸湪鍐呮牳涓坊鍔?printk 鎵嶈兘寮勬竻妤氬彂鐢熶簡浠€涔堛€?
## HID-BPF 鐨勯珮灞傝鍥?

HID-BPF 鑳屽悗鐨勬牳蹇冩€濇兂鏄畠鍦ㄥ瓧鑺傛暟缁勫眰闈㈠伐浣溿€傚洜姝わ紝瀵?HID 鎶ュ憡鍜?HID 鎶ュ憡鎻忚堪绗︾殑鎵€鏈夎В鏋愰兘蹇呴』鍦ㄥ姞杞?eBPF 绋嬪簭鐨勭敤鎴风┖闂寸粍浠朵腑瀹炵幇銆?
渚嬪锛屽湪涓婇潰鎻愬埌鐨勬鍖烘墜鏌勪腑锛岄渶瑕佹妸鏁版嵁娴佷腑鐨勫摢浜涘瓧娈电疆涓?`0` 蹇呴』鐢辩敤鎴风┖闂磋绠楀嚭鏉ャ€?
鐢辨鎺ㄨ锛孒ID-BPF 涓嶄簡瑙ｅ唴鏍镐腑鍏跺畠瀛愮郴缁熺殑瀛樺湪銆?浣犳棤娉曚粠 eBPF 涓洿鎺ラ€氳繃杈撳叆 API 鍙戝嚭杈撳叆浜嬩欢*銆?
褰撴煇涓?BPF 绋嬪簭闇€瑕佸彂鍑鸿緭鍏ヤ簨浠舵椂锛屽畠闇€瑕佷笌 HID 鍗忚浜や簰锛屽苟渚濊禆 HID 鍐呮牳澶勭悊灏?HID 鏁版嵁杞崲涓鸿緭鍏ヤ簨浠躲€?
## 鍐呮牳鏍戝唴鐨?HID-BPF 绋嬪簭涓?``udev-hid-bpf``


瀹樻柟鐨勮澶囦慨澶嶄互鍐呮牳婧愮爜鐨勫舰寮忛殢鍐呮牳鏍戜竴璧峰彂甯冿紝浣嶄簬 `drivers/hid/bpf/progs` 鐩綍銆傝繖浣垮緱鎴戜滑鍙互鍦?`tools/testing/selftests/hid` 涓负瀹冧滑娣诲姞鑷祴璇曘€?
涓嶈繃锛岃繖浜涚洰鏍囩殑缂栬瘧涓嶅睘浜庡父瑙勫唴鏍哥紪璇戠殑涓€閮ㄥ垎锛屽洜涓哄畠浠渶瑕佷竴涓閮ㄥ伐鍏锋墠鑳藉姞杞姐€傝宸ュ叿鐩墠鏄?`udev-hid-bpf <https://libevdev.pages.freedesktop.org/udev-hid-bpf/index.html>`_銆?
涓烘柟渚胯捣瑙侊紝璇ュ閮ㄤ粨搴撳皢杩欓噷鐨?`drivers/hid/bpf/progs` 鏂囦欢澶嶅埗鍒拌嚜宸辩嫭绔嬬殑 `src/bpf/stable` 鐩綍涓€傝繖浣垮緱鍙戣鐗堟棤闇€鎷夊彇鏁翠釜鍐呮牳婧愮爜鏍戝氨鑳藉彂甯冨苟鎵撳寘杩欎簺 HID-BPF 淇銆俙udev-hid-bpf` 杩樺叿澶囨牴鎹敤鎴锋墍杩愯鍐呮牳鏉ュ鐞嗗涓洰鏍囨枃浠剁殑鑳藉姏銆?
## 鍙敤鐨勭▼搴忕被鍨?

HID-BPF 鏋勫缓鍦?BPF 鈥滀箣涓娾€濓紝杩欐剰鍛崇潃鎴戜滑浣跨敤 bpf struct_ops 鏂规硶鏉ュ０鏄庣▼搴忋€?
HID-BPF 鎻愪緵浠ヤ笅鍙敤鐨勯檮鍔犵被鍨嬶細

1. 浜嬩欢澶勭悊/杩囨护锛屼娇鐢?libbpf 涓殑 `SEC("struct_ops/hid_device_event")`銆?2. 鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勬搷浣滐紝浣跨敤 libbpf 涓殑 `SEC("syscall")`銆?3. 鎶ュ憡鎻忚堪绗︾殑鏇存敼锛屼娇鐢?libbpf 涓殑 `SEC("struct_ops/hid_rdesc_fixup")` 鎴?`SEC("struct_ops.s/hid_rdesc_fixup")`銆?
`hid_device_event` 鍦ㄤ粠璁惧鏀跺埌浜嬩欢鏃惰皟鐢ㄤ竴涓?BPF 绋嬪簭銆傚洜姝わ紝鎴戜滑澶勪簬 IRQ 涓婁笅鏂囷紝鍙互瀵规暟鎹繘琛屾搷浣滄垨閫氱煡鐢ㄦ埛绌洪棿銆傚苟涓旂敱浜庢垜浠浜?IRQ 涓婁笅鏂囷紝鎴戜滑鏃犳硶涓庤澶囧洖璇濄€?
`syscall` 琛ㄧず鐢ㄦ埛绌洪棿璋冪敤浜?`BPF_PROG_RUN` 绯荤粺璋冪敤璁炬柦銆傝繖涓€娆℃垜浠彲浠ユ墽琛?HID-BPF 鍏佽鐨勪换浣曟搷浣滐紝骞朵笖鍏佽涓庤澶囧璇濄€?
鏈€鍚庯紝`hid_rdesc_fixup` 涓庡叾瀹冪被鍨嬩笉鍚岋紝鍥犱负璇ョ被鍨嬪彧鑳芥湁涓€涓?BPF 绋嬪簭銆傚畠鍦ㄩ┍鍔ㄧ殑 `probe` 鏃惰璋冪敤锛屽苟鍏佽閫氳繃 BPF 绋嬪簭鏇存敼鎶ュ憡鎻忚堪绗︺€備竴鏃?`hid_rdesc_fixup` 绋嬪簭琚姞杞斤紝闄ら潪鎻掑叆瀹冪殑绋嬪簭閫氳繃鍥哄畾锛坧in锛夎绋嬪簭骞跺叧闂墍鏈夋寚鍚戝畠鐨?fd 鏉ュ厑璁革紝鍚﹀垯鏃犳硶瑕嗙洊瀹冦€?
娉ㄦ剰锛宍hid_rdesc_fixup` 鍙互澹版槑涓哄彲浼戠湢鐨勶紙`SEC("struct_ops.s/hid_rdesc_fixup")`锛夈€?
## 寮€鍙戣€?API锛?

### HID-BPF 鍙敤鐨?``struct_ops``锛?

   :identifiers: hid_bpf_ops


### 绋嬪簭涓彲鐢ㄧ殑鐢ㄦ埛 API 鏁版嵁缁撴瀯锛?

   :identifiers: hid_bpf_ctx

### 鎵€鏈?HID-BPF struct_ops 绋嬪簭閮藉彲浣跨敤鐨?API锛?

   :identifiers: hid_bpf_get_data

### syscall 绫?HID-BPF 绋嬪簭鎴栧彲浼戠湢鐨?HID-BPF struct_ops 绋嬪簭閮藉彲浣跨敤鐨?API锛?

   :identifiers: hid_bpf_hw_request hid_bpf_hw_output_report hid_bpf_input_report hid_bpf_try_input_report hid_bpf_allocate_context hid_bpf_release_context

## HID-BPF 绋嬪簭鐨勯€氱敤姒傝


### 璁块棶闄勫姞鍒颁笂涓嬫枃鐨勬暟鎹?

`struct hid_bpf_ctx` 涓嶄細鐩存帴瀵煎嚭 `data` 瀛楁锛岃璁块棶瀹冿紝BPF 绋嬪簭闇€瑕佸厛璋冪敤 `hid_bpf_get_data`銆?
`offset` 鍙互鏄换鎰忔暣鏁帮紝浣?`size` 蹇呴』鏄父閲忥紝鍦ㄧ紪璇戞椂宸茬煡銆?
杩欐牱灏卞厑璁镐互涓嬫儏鍐碉細

1. 瀵逛簬缁欏畾璁惧锛屽鏋滄垜浠煡閬撴姤鍛婇暱搴﹀缁堜负鏌愪釜鍥哄畾鍊硷紝鎴戜滑鍙互璇锋眰 `data` 鎸囬拡鎸囧悜瀹屾暣鐨勬姤鍛婇暱搴︺€?
   鍐呮牳浼氱‘淇濇垜浠娇鐢ㄦ纭殑澶у皬鍜屽亸绉伙紝鑰?eBPF 浼氱‘淇濓細

```
     __u8 *data = hid_bpf_get_data(ctx, 0 /* offset */, 256 /* size */);

     if (!data)
         return 0; /* ensure data is correct, now the verifier knows we
                    * have 256 bytes available */

     bpf_printk("hello world: %02x %02x %02x", data[0], data[128], data[255]);
```

2. 濡傛灉鎶ュ憡闀垮害鍙彉锛屼絾鎴戜滑鐭ラ亾 `X` 鐨勫€煎缁堟槸涓€涓?16 浣嶅€硷細

```
      __u16 *x = hid_bpf_get_data(ctx, offset, sizeof(*x));

      if (!x)
          return 0; /* something went wrong */

      *x += 1; /* increment X by one */
```

### HID-BPF 绋嬪簭鐨勬晥鏋?

瀵逛簬鎵€鏈?HID-BPF 闄勫姞绫诲瀷锛堥櫎浜?`hid_rdesc_fixup`锛夛紝鍙互鏈夊涓?eBPF 绋嬪簭闄勫姞鍒板悓涓€璁惧銆傚鏋滄煇涓?HID-BPF struct_ops 甯︽湁 `hid_rdesc_fixup`锛岃€屽彟涓€涓凡闄勫姞鍒拌璁惧锛屽唴鏍稿湪闄勫姞璇?struct_ops 鏃朵細杩斿洖 `-EINVAL`銆?
闄ら潪鍦ㄩ檮鍔犵▼搴忔椂鍚?flags 娣诲姞浜?`BPF_F_BEFORE`锛屽惁鍒欐柊绋嬪簭浼氳杩藉姞鍒板垪琛ㄦ湯灏俱€俙BPF_F_BEFORE` 浼氭妸鏂扮▼搴忔彃鍏ュ埌鍒楄〃寮€澶达紝杩欏渚嬪杩借釜鍦烘櫙寰堟湁鐢ㄢ€斺€旀垜浠渶瑕佽幏鍙栨潵鑷澶囩殑鏈鐞嗕簨浠躲€?
娉ㄦ剰锛屽鏋滄湁澶氫釜绋嬪簭浣跨敤浜?`BPF_F_BEFORE` 鏍囧織锛屽疄闄呬笂鍙湁鏈€杩戝姞杞界殑閭ｄ竴涓墠鍦ㄥ垪琛ㄩ浣嶃€?
#### ``SEC("struct_ops/hid_device_event")``


姣忓綋鏈夊尮閰嶇殑浜嬩欢琚Е鍙戯紝eBPF 绋嬪簭浼氫緷娆¤璋冪敤锛屽苟涓斿畠浠搷浣滅殑鏄悓涓€浠芥暟鎹紦鍐插尯銆?
濡傛灉鏌愪釜绋嬪簭鏇存敼浜嗕笌涓婁笅鏂囧叧鑱旂殑鏁版嵁锛屼笅涓€涓▼搴忓皢鐪嬪埌淇敼鍚庣殑鏁版嵁锛屼絾瀹冨皢**鏃犱粠鐭ユ檽**鍘熷鏁版嵁鏄粈涔堛€?
涓€鏃︽墍鏈夌▼搴忛兘杩愯瀹屾瘯骞惰繑鍥?`0` 鎴栨鍊硷紝HID 鍗忚鏍堢殑鍏朵綑閮ㄥ垎灏嗗淇敼鍚庣殑鏁版嵁杩涜澶勭悊锛屾渶鍚庝竴涓?hid_bpf_ctx 鐨?`size` 瀛楁鍗充负杈撳叆鏁版嵁娴佺殑鏂板ぇ灏忋€?
杩斿洖璐熼敊璇殑 BPF 绋嬪簭浼氫涪寮冭浜嬩欢锛屽嵆璇ヤ簨浠朵笉浼氳 HID 鍗忚鏍堝鐞嗐€傚鎴风锛坔idraw銆乮nput銆丩ED锛夊皢**涓嶄細**鐪嬪埌璇ヤ簨浠躲€?
#### ``SEC("syscall")``


`syscall` 骞朵笉闄勫姞鍒版煇涓壒瀹氳澶囥€備负浜嗘寚鏄庢垜浠鍦ㄥ鐞嗙殑鏄摢涓澶囷紝鐢ㄦ埛绌洪棿闇€瑕侀€氳繃璁惧鐨勫敮涓€绯荤粺 ID锛坰ysfs 璺緞涓殑鏈€鍚?4 涓暟瀛楋細`/sys/bus/hid/devices/xxxx:yyyy:zzzz:0000`锛夋潵寮曠敤瀹冦€?
涓轰簡鑾峰彇涓庤璁惧鍏宠仈鐨勪笂涓嬫枃锛岀▼搴忓繀椤昏皟鐢?hid_bpf_allocate_context()锛屽苟鍦ㄨ繑鍥炲墠鐢?hid_bpf_release_context() 閲婃斁瀹冦€備竴鏃﹁幏鍙栦簡涓婁笅鏂囷紝涔熷彲浠ョ敤 hid_bpf_get_data() 璇锋眰涓€涓寚鍚戝唴鏍稿唴瀛樼殑鎸囬拡銆傝繖鍧楀唴瀛樿冻澶熷ぇ锛屽彲浠ユ敮鎸佽缁欏畾璁惧鐨勬墍鏈夎緭鍏?杈撳嚭/鐗规€ф姤鍛娿€?
#### ``SEC("struct_ops/hid_rdesc_fixup")``


`hid_rdesc_fixup` 绋嬪簭鐨勫伐浣滄柟寮忎笌 `struct hid_driver` 鐨?`.report_fixup` 绫讳技銆?
褰撹澶囪鎺㈡祴鏃讹紝鍐呮牳浼氱敤鎶ュ憡鎻忚堪绗︾殑鍐呭濉厖涓婁笅鏂囩殑鏁版嵁缂撳啿鍖恒€備笌璇ョ紦鍐插尯鍏宠仈鐨勫唴瀛樹负 `HID_MAX_DESCRIPTOR_SIZE`锛堝綋鍓嶄负 4kB锛夈€?
eBPF 绋嬪簭鍙互闅忔剰淇敼鏁版嵁缂撳啿鍖猴紝鍐呮牳浼氭妸淇敼鍚庣殑鍐呭涓庡ぇ灏忎綔涓烘姤鍛婃弿杩扮浣跨敤銆?
姣忓綋涓€涓寘鍚?`SEC("struct_ops/hid_rdesc_fixup")` 绋嬪簭鐨?struct_ops 琚檮鍔狅紙濡傛灉涔嬪墠娌℃湁绋嬪簭琚檮鍔狅級锛屽唴鏍镐細绔嬪嵆鏂紑璇?HID 璁惧骞堕噸鏂版帰娴嬨€?
鍚屾牱鍦帮紝褰撹 struct_ops 琚垎绂绘椂锛屽唴鏍镐細瀵硅澶囧彂鍑烘柇寮€杩炴帴銆?
HID-BPF 涓病鏈?`detach` 璁炬柦銆傚垎绂讳竴涓▼搴忓彂鐢熷湪鎵€鏈夋寚鍚戞煇涓?HID-BPF struct_ops 閾炬帴鐨勭敤鎴风┖闂存枃浠舵弿杩扮閮借鍏抽棴鏃躲€傚洜姝わ紝濡傛灉鎴戜滑闇€瑕佹浛鎹㈡煇涓姤鍛婃弿杩扮淇绋嬪簭锛岄渶瑕佸師濮嬫姤鍛婃弿杩扮淇绋嬪簭鐨勬墍鏈夎€呴厤鍚堛€傚厛鍓嶇殑鎵€鏈夎€呭緢鍙兘浼氭妸璇?struct_ops 閾炬帴鍥哄畾鍒?bpffs 涓紝涔嬪悗鎴戜滑灏卞彲浠ラ€氳繃鏅€氱殑 bpf 鎿嶄綔鏉ユ浛鎹㈠畠銆?
## 灏?bpf 绋嬪簭闄勫姞鍒拌澶?

鎴戜滑鐜板湪浣跨敤閫氳繃 `bpf_map__attach_struct_ops()` 鐨勬爣鍑?struct_ops 闄勫姞鏂瑰紡銆備絾鐢变簬鎴戜滑闇€瑕佸皢 struct_ops 闄勫姞鍒颁竴涓笓鐢ㄧ殑 HID 璁惧锛岃皟鐢ㄨ€呭繀椤诲湪灏嗙▼搴忓姞杞借繘鍐呮牳涔嬪墠锛屽湪 struct_ops map 涓缃?`hid_id`銆?
`hid_id` 鏄?HID 璁惧鐨勫敮涓€绯荤粺 ID锛坰ysfs 璺緞涓殑鏈€鍚?4 涓暟瀛楋細`/sys/bus/hid/devices/xxxx:yyyy:zzzz:0000`锛夈€?
涔熷彲浠ヨ缃?`flags`锛屽叾绫诲瀷涓?`enum hid_bpf_attach_flags`銆?
鎴戜滑鏃犳硶渚濊禆 hidraw 鏉ユ妸 BPF 绋嬪簭缁戝畾鍒?HID 璁惧銆俬idraw 鏄?HID 璁惧澶勭悊杩囩▼鐨勪骇鐗╋紝骞朵笉绋冲畾銆傛煇浜涢┍鍔ㄧ敋鑷充細绂佺敤瀹冿紝浠庤€屽湪杩欎簺璁惧涓婂け鍘讳簡杩借釜鑳藉姏锛堣€岃幏鍙栭潪 hidraw 鐨勮拷韪俊鎭伆鎭板緢鏈夋剰涔夛級銆?
鍙︿竴鏂归潰锛宍hid_id` 鍦?HID 璁惧鐨勬暣涓敓鍛藉懆鏈熷唴閮芥槸绋冲畾鐨勶紝鍗充究鎴戜滑鏇存敼浜嗗畠鐨勬姤鍛婃弿杩扮銆?
閴翠簬 hidraw 鍦ㄨ澶囨柇寮€/閲嶈繛鏃跺苟涓嶇ǔ瀹氾紝鎴戜滑寤鸿閫氳繃 sysfs 璁块棶璁惧褰撳墠鐨勬姤鍛婃弿杩扮銆傚畠鍦?`/sys/bus/hid/devices/BUS:VID:PID.000N/report_descriptor` 澶勪綔涓轰竴涓簩杩涘埗娴佹彁渚涖€?
瑙ｆ瀽鎶ュ憡鎻忚堪绗︽槸 BPF 缂栫▼鑰呮垨鍔犺浇 eBPF 绋嬪簭鐨勭敤鎴风┖闂寸粍浠剁殑璐ｄ换銆?
## 涓€涓紙鍑犱箮锛夊畬鏁寸殑 BPF 澧炲己 HID 璁惧绀轰緥


**鍓嶈█锛氬湪澶у鏁版儏鍐典笅锛岃繖涔熷彲浠ョ敤鍐呮牳椹卞姩鏉ュ疄鐜?*

璁炬兂鎴戜滑鏈変竴涓柊鐨勫钩鏉胯澶囷紝鍏锋湁涓€浜涜Е瑙夎兘鍔涳紝鍙互妯℃嫙鐢ㄦ埛姝ｅ湪鍏朵笂涔﹀啓鐨勮〃闈€傝璁惧杩樻湁涓€涓壒瀹氱殑 3 妗ｅ紑鍏筹紝鐢ㄤ簬鍦?*閾呯瑪鍦ㄧ焊涓?*銆?*澧欎笂鐨勮湣绗?*鍜?*鐢荤瑪鍦ㄧ敾甯冧笂**涔嬮棿鍒囨崲銆備负浜嗛敠涓婃坊鑺憋紝鎴戜滑杩樺彲浠ラ€氳繃涓€涓壒鎬ф姤鍛婃潵鎺у埗璇ュ紑鍏崇殑鐗╃悊浣嶇疆銆?
褰撶劧锛岃寮€鍏充緷璧栨煇涓敤鎴风┖闂寸粍浠舵潵鎺у埗璁惧鑷韩鐨勮Е瑙夌壒鎬с€?
### 杩囨护浜嬩欢


绗竴姝ユ槸瀵规潵鑷澶囩殑浜嬩欢杩涜杩囨护銆傜敱浜庡紑鍏充綅缃疄闄呬笂鏄湪瑙︽帶绗斾簨浠舵祦涓姤鍛婄殑锛屼娇鐢?hidraw 鏉ュ疄鐜拌繖绉嶈繃婊ゆ剰鍛崇潃姣忎釜浜嬩欢閮戒細鍞ら啋鐢ㄦ埛绌洪棿銆?
杩欏 libinput 鏉ヨ娌￠棶棰橈紝浣嗚涓€涓彧鍏冲績鎶ュ憡涓竴涓瓧鑺傜殑澶栭儴搴撳幓鎵挎媴杩欑鍞ら啋锛屽氨涓嶅お鐞嗘兂浜嗐€?
```
  #include "vmlinux.h"
  #include <bpf/bpf_helpers.h>
  #include <bpf/bpf_tracing.h>

  /* HID programs need to be GPL */
  char _license[] SEC("license") = "GPL";

  /* HID-BPF kfunc API definitions */
  extern __u8 *hid_bpf_get_data(struct hid_bpf_ctx *ctx,
			      unsigned int offset,
			      const size_t __sz) __ksym;

  struct {
	__uint(type, BPF_MAP_TYPE_RINGBUF);
	__uint(max_entries, 4096 * 64);
  } ringbuf SEC(".maps");

  __u8 current_value = 0;

  SEC("struct_ops/hid_device_event")
  int BPF_PROG(filter_switch, struct hid_bpf_ctx *hid_ctx)
  {
	__u8 *data = hid_bpf_get_data(hid_ctx, 0 /* offset */, 192 /* size */);
	__u8 *buf;

	if (!data)
		return 0; /* EPERM check */

	if (current_value != data[152]) {
		buf = bpf_ringbuf_reserve(&ringbuf, 1, 0);
		if (!buf)
			return 0;

		*buf = data[152];

		bpf_ringbuf_commit(buf, 0);

		current_value = data[152];
	}

	return 0;
  }

  SEC(".struct_ops.link")
  struct hid_bpf_ops haptic_tablet = {
  	.hid_device_event = (void *)filter_switch,
  };
```

```
  static int attach_filter(struct hid *hid_skel, int hid_id)
  {
  	int err, link_fd;

  	hid_skel->struct_ops.haptic_tablet->hid_id = hid_id;
  	err = hid__load(skel);
  	if (err)
  		return err;

  	link_fd = bpf_map__attach_struct_ops(hid_skel->maps.haptic_tablet);
  	if (!link_fd) {
  		fprintf(stderr, "can not attach HID-BPF program: %m\n");
  		return -1;
  	}

  	return link_fd; /* the fd of the created bpf_link */
  }
```

鎴戜滑鐨勭敤鎴风┖闂寸▼搴忕幇鍦ㄥ彲浠ョ洃鍚幆褰㈢紦鍐插尯涓婄殑閫氱煡锛屽苟涓斾粎褰撳€煎彂鐢熷彉鍖栨椂鎵嶄細琚敜閱掋€?
褰撶敤鎴风┖闂寸▼搴忎笉鍐嶉渶瑕佺洃鍚簨浠舵椂锛屽畠鍙互绠€鍗曞湴鍏抽棴 `attach_filter` 杩斿洖鐨?bpf 閾炬帴锛岃繖浼氶€氱煡鍐呮牳灏嗚绋嬪簭浠?HID 璁惧涓婂垎绂汇€?
褰撶劧锛屽湪鍏朵粬浣跨敤鍦烘櫙涓紝鐢ㄦ埛绌洪棿绋嬪簭涔熷彲浠ュ儚浠讳綍 bpf_link 涓€鏍凤紝閫氳繃璋冪敤 `bpf_obj_pin` 鎶婅 fd 鍥哄畾鍒?BPF 鏂囦欢绯荤粺銆?
### 鎺у埗璁惧


涓轰簡鑳藉鏇存敼骞虫澘鐨勮Е瑙夊弽棣堬紝鐢ㄦ埛绌洪棿绋嬪簭闇€瑕佸悜璁惧鑷韩鍙戝嚭涓€涓壒鎬ф姤鍛娿€?
鎴戜滑涓嶅繀涓烘浣跨敤 hidraw锛屽彲浠ュ垱寤轰竴涓?`SEC("syscall")` 绋嬪簭锛?
```
  /* some more HID-BPF kfunc API definitions */
  extern struct hid_bpf_ctx *hid_bpf_allocate_context(unsigned int hid_id) __ksym;
  extern void hid_bpf_release_context(struct hid_bpf_ctx *ctx) __ksym;
  extern int hid_bpf_hw_request(struct hid_bpf_ctx *ctx,
			      __u8* data,
			      size_t len,
			      enum hid_report_type type,
			      enum hid_class_request reqtype) __ksym;


  struct hid_send_haptics_args {
	/* data needs to come at offset 0 so we can do a memcpy into it */
	__u8 data[10];
	unsigned int hid;
  };

  SEC("syscall")
  int send_haptic(struct hid_send_haptics_args *args)
  {
	struct hid_bpf_ctx *ctx;
	int ret = 0;

	ctx = hid_bpf_allocate_context(args->hid);
	if (!ctx)
		return 0; /* EPERM check */

	ret = hid_bpf_hw_request(ctx,
				 args->data,
				 10,
				 HID_FEATURE_REPORT,
				 HID_REQ_SET_REPORT);

	hid_bpf_release_context(ctx);

	return ret;
  }
```

```
  static int set_haptic(struct hid *hid_skel, int hid_id, __u8 haptic_value)
  {
	int err, prog_fd;
	int ret = -1;
	struct hid_send_haptics_args args = {
		.hid = hid_id,
	};
	DECLARE_LIBBPF_OPTS(bpf_test_run_opts, tattrs,
		.ctx_in = &args,
		.ctx_size_in = sizeof(args),
	);

	args.data[0] = 0x02; /* report ID of the feature on our device */
	args.data[1] = haptic_value;

	prog_fd = bpf_program__fd(hid_skel->progs.set_haptic);

	err = bpf_prog_test_run_opts(prog_fd, &tattrs);
	return err;
  }
```

鐜板湪鎴戜滑鐨勭敤鎴风┖闂寸▼搴忎簡瑙ｄ簡瑙﹁鐘舵€佸苟鑳藉鎺у埗瀹冦€傝绋嬪簭鍙互鎶婅繖涓姸鎬佽繘涓€姝ユ彁渚涚粰鍏跺畠鐢ㄦ埛绌洪棿绋嬪簭锛堜緥濡傞€氳繃 DBus API锛夈€?
杩欓噷鏈夎叮鐨勪竴鐐规槸锛屾垜浠苟娌℃湁涓烘鍒涘缓鏂扮殑鍐呮牳 API銆傝繖鎰忓懗鐫€濡傛灉鎴戜滑瀹炵幇涓湁 bug锛屾垜浠彲浠ラ殢鎰忔洿鏀逛笌鍐呮牳涔嬮棿鐨勬帴鍙ｏ紝鍥犱负鐢ㄦ埛绌洪棿搴旂敤绋嬪簭瑕佸鑷繁鐨勪娇鐢ㄨ礋璐ｃ€?