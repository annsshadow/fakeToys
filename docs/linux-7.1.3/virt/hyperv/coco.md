
## 鏈哄瘑璁＄畻锛圕onfidential Computing锛塚M


Hyper-V 鍙互鍒涘缓骞惰繍琛屼綔涓烘満瀵嗚绠楋紙Confidential Computing锛孋oCo锛塚M 鐨?Linux 瀹㈡埛鏈恒€傛绫?VM 涓庣墿鐞嗗鐞嗗櫒鍗忎綔锛屼互鏇村ソ鍦颁繚鎶?VM 鍐呭瓨涓暟鎹殑鏈哄瘑鎬у拰瀹屾暣鎬э紝鍗充娇闈㈠鍙兘宸茶鏀荤牬骞惰〃鐜板嚭鎭舵剰琛屼负鐨勭鐞嗙▼搴忥紙hypervisor/VMM锛変篃鏄姝ゃ€侶yper-V 涓婄殑 CoCo VM 鍏变韩 Documentation/security/snp-tdx-threat-model.rst 涓弿杩扮殑閫氱敤 CoCo VM 濞佽儊妯″瀷涓庡畨鍏ㄧ洰鏍囥€傝娉ㄦ剰锛孡inux 涓?Hyper-V 鐗瑰畾鐨勪唬鐮佸皢 CoCo VM 绉颁负"isolated VMs"鎴?isolation VMs"銆?

Hyper-V 涓婄殑 Linux CoCo VM 闇€瑕佷互涓嬮儴鍒嗙殑鍗忎綔涓庝氦浜掞細

- 鏀寔 CoCo VM 鐨勫鐞嗗櫒鎵€鍦ㄧ殑鐗╃悊纭欢

- 杩愯鏀寔 CoCo VM 鐨?Windows/Hyper-V 鐗堟湰鐨勭‖浠?

- 杩愯鏀寔浣滀负 CoCo VM 鐨?Linux 鐗堟湰鐨?VM

鐗╃悊纭欢瑕佹眰濡備笅锛?

- 甯︽湁 SEV-SNP 鐨?AMD 澶勭悊鍣ㄣ€侶yper-V 涓嶄細杩愯浣跨敤 AMD SME銆丼EV 鎴?SEV-ES 鍔犲瘑鐨勫鎴锋満 VM锛屽苟涓旀绫诲姞瀵嗗浜?Hyper-V 涓婄殑 CoCo VM 鏉ヨ骞朵笉鍏呭垎銆?

- 甯︽湁 TDX 鐨?Intel 澶勭悊鍣?

瑕佸垱寤?CoCo VM锛屽繀椤诲湪鍒涘缓 VM 鏃跺悜 Hyper-V 鎸囧畾"Isolated VM"灞炴€с€俈M 涓€鏃﹀垱寤猴紝渚挎棤娉曚粠 CoCo VM 鏇存敼涓烘櫘閫?VM锛屽弽涔嬩害鐒躲€?

### 杩愯妯″紡


Hyper-V CoCo VM 鍙互杩愯浜庝袱绉嶆ā寮忋€傛ā寮忓湪鍒涘缓 VM 鏃堕€夊畾锛屽湪 VM 鐨勭敓鍛藉懆鏈熷唴鏃犳硶鏇存敼銆?

- 瀹屽叏 enlightened锛坒ully-enlightened锛夋ā寮忋€傚湪姝ゆā寮忎笅锛屽鎴锋満鎿嶄綔绯荤粺琚?enlightened锛岃兘澶熺悊瑙ｅ苟绠＄悊浣滀负 CoCo VM 杩愯鐨勫悇涓柟闈€?

- Paravisor 妯″紡銆傚湪姝ゆā寮忎笅锛屼綅浜庡鎴锋満涓庝富鏈轰箣闂寸殑 paravisor 灞傛彁渚涗竴浜涗綔涓?CoCo VM 杩愯鎵€闇€鐨勬搷浣溿€傚鎴锋満鎿嶄綔绯荤粺鎵€闇€鐨?CoCo enlightenment 鍙互灏戜簬 fully-enlightened 鎯呭喌銆?

浠庢蹇典笂璁诧紝fully-enlightened 妯″紡涓?paravisor 妯″紡鍙瑙嗕负涓€涓厜璋变笂鐨勪袱涓偣锛岃鍏夎氨娑电洊浜嗕綔涓?CoCo VM 杩愯鎵€闇€鐨勫鎴锋満 enlightenment 绋嬪害銆俧ully-enlightened 妯″紡鏄厜璋辩殑涓€绔€俻aravisor 妯″紡鐨勫畬鏁村疄鐜版槸鍏夎氨鐨勫彟涓€绔紝鍦ㄩ偅涓€绔紝浣滀负 CoCo VM 杩愯鐨勫悇涓柟闈㈤兘鐢?paravisor 澶勭悊锛屼竴涓鍐呭瓨鍔犲瘑鎴?CoCo VM 鍏朵粬鏂归潰涓€鏃犳墍鐭ョ殑鏅€氬鎴锋満 OS 涔熻兘鎴愬姛杩愯銆傜劧鑰岋紝Hyper-V 瀵?paravisor 妯″紡鐨勫疄鐜板苟鏈蛋鍒拌繖涓€姝ワ紝鑰屾槸澶勪簬鍏夎氨涓棿鐨勬煇涓綅缃€侰oCo VM 鐨勬煇浜涙柟闈㈢敱 Hyper-V paravisor 澶勭悊锛岃€屽鎴锋満 OS 蹇呴』瀵瑰彟涓€鏂归潰杩涜 enlightenment銆傞仐鎲剧殑鏄紝paravisor 涓彲鑳芥彁渚涚殑鍔熻兘/鐗规€ф病鏈夋爣鍑嗗寲鐨勬灇涓撅紝瀹㈡埛鏈?OS 涔熸病鏈夋爣鍑嗗寲鐨勬満鍒跺悜 paravisor 鏌ヨ鍏舵彁渚涚殑鍔熻兘/鐗规€с€俻aravisor 鎻愪緵浠€涔堢殑鐞嗚В鏄‖缂栫爜鍦ㄥ鎴锋満 OS 涓殑銆?

Paravisor 妯″紡涓?`Coconut project`_ 鏈夌浉浼间箣澶勶紝鍚庤€呮棬鍦ㄦ彁渚涗竴涓湁闄愮殑 paravisor锛屼负瀹㈡埛鏈烘彁渚涙湇鍔★紝渚嬪铏氭嫙 TPM銆傜劧鑰岋紝Hyper-V paravisor 閫氬父澶勭悊鐨?CoCo VM 鏂归潰姣旂洰鍓嶄负 Coconut 璁炬兂鐨勬洿澶氾紝鍥犳鏇存帴杩戜簬鍏夎氨涓?鏃犻渶瀹㈡埛鏈?enlightenment"鐨勪竴绔€?


鍦?CoCo VM 濞佽儊妯″瀷涓紝paravisor 澶勪簬瀹㈡埛鏈哄畨鍏ㄥ煙涓紝涓斿繀椤昏瀹㈡埛鏈?OS 淇′换銆傜敱姝ゆ帹璁猴紝hypervisor/VMM 蹇呴』鍍忛槻鑼冩綔鍦ㄦ伓鎰忕殑瀹㈡埛鏈轰竴鏍凤紝闃茶寖娼滃湪鎭舵剰鐨?paravisor銆?

閽堝 fully-enlightened 涓?paravisor 妯″紡鐨勭‖浠舵灦鏋勬柟娉曞洜搴曞眰澶勭悊鍣ㄨ€屽紓銆?

- 瀵逛簬 AMD SEV-SNP 澶勭悊鍣紝鍦?fully-enlightened 妯″紡涓嬪鎴锋満 OS 杩愯浜?VMPL 0锛屽苟瀹屽叏鎺у埗瀹㈡埛鏈轰笂涓嬫枃銆傚湪 paravisor 妯″紡涓嬶紝瀹㈡埛鏈?OS 杩愯浜?VMPL 2锛岃€?paravisor 杩愯浜?VMPL 0銆傝繍琛屼簬 VMPL 0 鐨?paravisor 鎷ユ湁瀹㈡埛鏈?OS锛堣繍琛屼簬 VMPL 2锛夋墍娌℃湁鐨勭壒鏉冦€傛煇浜涙搷浣滆姹傚鎴锋満璋冪敤 paravisor銆傛澶栵紝鍦?paravisor 妯″紡涓嬶紝瀹㈡埛鏈?OS 鎸夌収 SEV-SNP 鏋舵瀯鐨勫畾涔夎繍琛屼簬"virtual Top Of Memory"锛坴TOM锛夋ā寮忋€傚綋浣跨敤 paravisor 鏃讹紝姝ゆā寮忕畝鍖栦簡瀹㈡埛鏈哄鍐呭瓨鍔犲瘑鐨勭鐞嗐€?

- 瀵逛簬 Intel TDX 澶勭悊鍣紝鍦?fully-enlightened 妯″紡涓嬪鎴锋満 OS 杩愯浜?L1 VM銆傚湪 paravisor 妯″紡涓嬶紝浣跨敤 TD 鍒嗗尯銆俻aravisor 杩愯浜?L1 VM锛岃€屽鎴锋満 OS 杩愯浜庡祵濂楃殑 L2 VM銆?

Hyper-V 鍚戝鎴锋満鏆撮湶涓€涓弿杩?CoCo 妯″紡鐨?synthetic MSR銆傝 MSR 鎸囩ず搴曞眰澶勭悊鍣ㄤ娇鐢ㄧ殑鏄?AMD SEV-SNP 杩樻槸 Intel TDX锛屼互鍙婃槸鍚︿娇鐢ㄤ簡 paravisor銆傛瀯寤轰竴涓兘澶熷湪浠讳竴鏋舵瀯涓娿€佷互浠讳竴妯″紡姝ｅ父寮曞骞惰繍琛岀殑鍗曚竴 kernel 鏄犲儚鏄緢鐩存帴鐨勩€?

### Paravisor 褰卞搷


杩愯浜?paravisor 妯″紡浼氬奖鍝嶉€氱敤 Linux kernel CoCo VM 鍔熻兘鐨勪互涓嬫柟闈細

- 鍒濆瀹㈡埛鏈哄唴瀛樿缃€傚湪 paravisor 妯″紡涓嬪垱寤烘柊 VM 鏃讹紝paravisor 鍏堣繍琛岋紝骞跺皢瀹㈡埛鏈虹墿鐞嗗唴瀛樿缃负鍔犲瘑銆傚鎴锋満 Linux 杩涜姝ｅ父鐨勫唴瀛樺垵濮嬪寲锛屽彧鏄樉寮忓湴灏嗛€傚綋鐨勮寖鍥存爣璁颁负宸茶В瀵嗭紙鍏变韩锛夈€傚湪 paravisor 妯″紡涓嬶紝Linux 涓嶆墽琛屽湪 fully-enlightened 妯″紡涓嬮厤鍚?AMD SEV-SNP 鐗瑰埆妫樻墜鐨勬棭鏈熷紩瀵煎唴瀛樿缃楠ゃ€?

- #VC/#VE 寮傚父澶勭悊銆傚湪 paravisor 妯″紡涓嬶紝Hyper-V 灏嗗鎴锋満 CoCo VM 閰嶇疆涓哄皢 #VC 鍜?#VE 寮傚父鍒嗗埆璺敱鍒?VMPL 0 鍜?L1 VM锛岃€屼笉鏄鎴锋満 Linux銆傚洜姝わ紝杩欎簺寮傚父澶勭悊绋嬪簭涓嶅湪瀹㈡埛鏈?Linux 涓繍琛岋紝涔熶笉鏄?paravisor 妯″紡涓?Linux 瀹㈡埛鏈烘墍闇€鐨?enlightenment銆?

- CPUID 鏍囧織銆侫MD SEV-SNP 鍜?Intel TDX 閮藉湪瀹㈡埛鏈轰腑鎻愪緵涓€涓?CPUID 鏍囧織锛屾寚绀鸿 VM 姝ｅ湪浣跨敤鐩稿簲鐨勭‖浠舵敮鎸佽繍琛屻€傝櫧鐒惰繖浜?CPUID 鏍囧織鍦?fully-enlightened CoCo VM 涓彲瑙侊紝浣?paravisor 浼氳繃婊ゆ帀杩欎簺鏍囧織锛屽鎴锋満 Linux 鐪嬩笉鍒板畠浠€傚湪鏁翠釜 Linux kernel 涓紝鏄惧紡娴嬭瘯杩欎簺鏍囧織鐨勫仛娉曞ぇ澶氬凡琚?cc_platform_has() 鍑芥暟鍙栦唬锛岀洰鐨勬槸鎶借薄 SEV-SNP 涓?TDX 涔嬮棿鐨勫樊寮傘€備絾 cc_platform_has() 鎶借薄涔熷厑璁?Hyper-V paravisor 閰嶇疆鍦ㄥ嵆渚挎湭璁剧疆 CPUID 鏍囧織鏃讹紝鏈夐€夋嫨鍦板惎鐢?CoCo VM 鍔熻兘鐨勬煇浜涙柟闈€備緥澶栨槸 SEV-SNP 涓婄殑鏃╂湡寮曞鍐呭瓨璁剧疆锛屽畠浼氭祴璇?CPUID SEV-SNP 鏍囧織銆備絾 Hyper-V paravisor 妯″紡 VM 涓病鏈夎鏍囧織锛屽弽鑰岃揪鍒颁簡涓嶈繍琛?SEV-SNP 鐗瑰畾鏃╂湡寮曞鍐呭瓨璁剧疆鎵€鏈熸湜鐨勬晥鏋溿€?

- 璁惧妯℃嫙銆傚湪 paravisor 妯″紡涓嬶紝Hyper-V paravisor 鎻愪緵瀵?IO-APIC 鍜?TPM 绛夎澶囩殑妯℃嫙銆傜敱浜庢ā鎷熷彂鐢熷湪 paravisor 鐨勫鎴锋満涓婁笅鏂囦腑锛堣€岄潪 hypervisor/VMM 涓婁笅鏂囷級锛屽杩欎簺璁惧鐨?MMIO 璁块棶蹇呴』鏄姞瀵嗗紩鐢紝鑰屼笉鏄?fully-enlightened CoCo VM 涓墍浣跨敤鐨勫凡瑙ｅ瘑寮曠敤銆俖_ioremap_caller() 鍑芥暟宸茶澧炲己锛屼細杩涜涓€娆″洖璋冧互妫€鏌ョ壒瀹氬湴鍧€鑼冨洿鏄惁搴旇瑙嗕负鍔犲瘑锛堢鏈夛級銆傚弬瑙?is_private_mmio"鍥炶皟銆?

- 鍔犲瘑/瑙ｅ瘑鍐呭瓨杞崲銆傚湪 CoCo VM 涓紝鍦ㄥ姞瀵嗕笌瑙ｅ瘑涔嬮棿杞崲瀹㈡埛鏈哄唴瀛橀渶瑕佷笌 hypervisor/VMM 鍗忚皟銆傝繖鏄€氳繃 __set_memory_enc_pgtable() 璋冪敤鐨勫洖璋冨畬鎴愮殑銆傚湪 fully-enlightened 妯″紡涓嬶紝浣跨敤杩欎簺鍥炶皟鐨勬櫘閫?SEV-SNP 鍜?TDX 瀹炵幇銆傚湪 paravisor 妯″紡涓嬶紝浣跨敤 Hyper-V 鐗瑰畾鐨勫洖璋冮泦鍚堛€傝繖浜涘洖璋冭皟鐢?paravisor锛屼互渚?paravisor 鑳藉鍗忚皟杞崲骞跺湪蹇呰鏃堕€氱煡 hypervisor銆傚弬瑙佽缃繖浜涘洖璋冪殑 hv_vtom_init()銆?

- 涓柇娉ㄥ叆銆傚湪 fully enlightened 妯″紡涓嬶紝鎭舵剰 hypervisor 鍙兘鍦ㄨ繚鍙?x86/x64 鏋舵瀯瑙勫垯鐨勬椂鍒诲悜瀹㈡埛鏈?OS 娉ㄥ叆涓柇銆備负浜嗗畬鏁翠繚鎶わ紝瀹㈡埛鏈?OS 搴斿寘鍚娇鐢?CoCo 鑳藉姏澶勭悊鍣ㄦ彁渚涚殑涓柇娉ㄥ叆绠＄悊鐗规€х殑 enlightenment銆傚湪 paravisor 妯″紡涓嬶紝paravisor 涓粙瀵瑰鎴锋満 OS 鐨勪腑鏂敞鍏ワ紝骞剁‘淇濆鎴锋満 OS 鍙湅鍒?鍚堟硶"鐨勪腑鏂€俻aravisor 浣跨敤 CoCo 鑳藉姏鐗╃悊澶勭悊鍣ㄦ彁渚涚殑涓柇娉ㄥ叆绠＄悊鐗规€э紝浠庤€屽皢杩欎簺澶嶆潅鎬у瀹㈡埛鏈?OS 灞忚斀銆?

### Hyper-V 瓒呯骇璋冪敤锛圚ypercalls锛?


鍦?fully-enlightened 妯″紡涓嬶紝Linux 瀹㈡埛鏈哄彂鍑虹殑 hypercall 浼氬儚鍦ㄩ潪 CoCo VM 涓竴鏍风洿鎺ヨ矾鐢卞埌 hypervisor銆備絾鍦?paravisor 妯″紡涓嬶紝鏅€?hypercall 浼氬厛闄峰叆 paravisor锛宲aravisor 杩涜€屽彲鑳借皟鐢?hypervisor銆備絾 paravisor 鍦ㄨ繖鏂归潰鏈夌壒娈婃€э紝Linux 瀹㈡埛鏈哄彂鍑虹殑灏戞暟 hypercall 蹇呴』濮嬬粓鐩存帴璺敱鍒?hypervisor銆傝繖浜?hypercall 璋冪敤鐐逛細妫€娴?paravisor 鏄惁瀛樺湪锛屽苟浣跨敤鐗规畩鐨勮皟鐢ㄥ簭鍒椼€備緥濡傚弬瑙?hv_post_message()銆?

### 瀹㈡埛鏈轰笌 Hyper-V 鐨勯€氫俊


闄や簡 Linux CoCo VM 涓?Linux kernel 瀵瑰唴瀛樺姞瀵嗙殑閫氱敤澶勭悊涔嬪锛孒yper-V 杩樻湁 VMBus 浠ュ強浣跨敤 Linux 瀹㈡埛鏈轰笌涓绘満涔嬮棿鍏变韩鍐呭瓨杩涜閫氫俊鐨?VMBus 璁惧銆傝鍏变韩鍐呭瓨蹇呴』鏍囪涓哄凡瑙ｅ瘑鎵嶈兘鍚敤閫氫俊銆傛澶栵紝鐢变簬濞佽儊妯″瀷鍖呭惈宸查伃鏀荤牬涓旀綔鍦ㄦ伓鎰忕殑涓绘満锛屽鎴锋満蹇呴』闃茶寖閫氳繃姝ゅ叡浜唴瀛樺悜涓绘満娉勯湶浠讳綍闈為鏈熺殑鏁版嵁銆?

杩欎簺 Hyper-V 涓?VMBus 鍐呭瓨椤佃鏍囪涓哄凡瑙ｅ瘑锛?

- VMBus 鐩戣椤碉紙monitor pages锛?

- 鍚堟垚涓柇鎺у埗鍣紙SynIC锛夌浉鍏抽〉锛堥櫎闈炵敱 paravisor 鎻愪緵锛?

- 姣?CPU 鐨?hypercall 杈撳叆鍜岃緭鍑洪〉锛堥櫎闈炰笌 paravisor 涓€璧疯繍琛岋級

- VMBus 鐜舰缂撳啿鍖恒€傜洿鎺ユ槧灏勫湪 __vmbus_establish_gpadl() 涓爣璁颁负宸茶В瀵嗐€傚湪 hv_ringbuffer_init() 涓垱寤虹殑浜岀骇鏄犲皠涔熷繀椤诲寘鍚?decrypted"灞炴€с€?

褰撳鎴锋満鍚戜笌涓绘満鍏变韩鐨勫唴瀛樺啓鍏ユ暟鎹椂锛屽繀椤荤‘淇濆彧鍐欏叆棰勬湡鐨勬暟鎹€傚湪澶嶅埗鍒板叡浜唴瀛樹箣鍓嶏紝濉厖鎴?unused 瀛楁蹇呴』鍒濆鍖栦负闆讹紝浠ュ厤闅忔満 kernel 鏁版嵁琚棤鎰忎腑鎻愪緵缁欎富鏈恒€?

绫讳技鍦帮紝褰撳鎴锋満璇诲彇涓庝富鏈哄叡浜殑鍐呭瓨鏃讹紝蹇呴』鍦ㄥ鐞嗘暟鎹箣鍓嶅鍏惰繘琛岄獙璇侊紝浠ュ厤鎭舵剰涓绘満璇变娇瀹㈡埛鏈烘毚闇查潪棰勬湡鐨勬暟鎹€傝繘琛屾绫婚獙璇佸彲鑳藉緢妫樻墜锛屽洜涓轰富鏈哄嵆浣垮湪楠岃瘉杩涜涓垨涔嬪悗涔熻兘淇敼鍏变韩鍐呭瓨鍖哄煙銆傚浜庡湪 VMBus 鐜舰缂撳啿鍖轰腑浠庝富鏈轰紶閫掔粰瀹㈡埛鏈虹殑娑堟伅锛屼細楠岃瘉娑堟伅闀垮害锛屽苟灏嗘秷鎭鍒跺埌涓存椂锛堝姞瀵嗭級缂撳啿鍖轰互杩涜杩涗竴姝ラ獙璇佸拰澶勭悊銆傚鍒朵細澧炲姞灏戦噺寮€閿€锛屼絾杩欐槸闃茶寖鎭舵剰涓绘満鐨勫敮涓€鏂规硶銆傚弬瑙?hv_pkt_iter_first()銆?

璁稿 VMBus 璁惧鐨勯┍鍔ㄥ凡閫氳繃娣诲姞浠ｇ爜鏉ュ厖鍒嗛獙璇侀€氳繃 VMBus 鎺ユ敹鐨勬秷鎭€?鍔犲浐锛坔ardened锛?锛岃€屼笉鏄亣璁?Hyper-V 鍦ㄥ崗浣滆繍琛屻€傛绫婚┍鍔ㄥ湪 vmbus_devs[] 琛ㄤ腑琚爣璁颁负"allowed_in_isolated"銆侰oCo VM 涓笉闇€瑕佺殑鍏朵粬 VMBus 璁惧椹卞姩灏氭湭鍔犲浐锛屽畠浠笉鍏佽鍦?CoCo VM 涓姞杞姐€傚弬瑙佹帓闄ゆ绫昏澶囩殑 vmbus_is_valid_offer()銆?

涓や釜 VMBus 璁惧渚濊禆 Hyper-V 涓绘満杩涜 DMA 鏁版嵁浼犺緭锛氱敤浜庣鐩?I/O 鐨?storvsc 鍜岀敤浜庣綉缁?I/O 鐨?netvsc銆俿torvsc 浣跨敤鏅€氱殑 Linux kernel DMA API锛屽洜姝ら€氳繃宸茶В瀵?swiotlb 鍐呭瓨鐨勫弽寮圭紦鍐诧紙bounce buffering锛夋槸闅愬紡瀹屾垚鐨勩€俷etvsc 鏈変袱绉嶆暟鎹紶杈撴ā寮忋€傜涓€绉嶆ā寮忕粡杩?netvsc 椹卞姩鏄惧紡鍒嗛厤鐨勫彂閫佸拰鎺ユ敹缂撳啿鍖虹┖闂达紝鐢ㄤ簬澶у鏁拌緝灏忕殑鏁版嵁鍖呫€傝繖浜涘彂閫佸拰鎺ユ敹缂撳啿鍖虹敱 __vmbus_establish_gpadl() 鏍囪涓哄凡瑙ｅ瘑銆傜敱浜?netvsc 椹卞姩鏄惧紡鍦板皢鏁版嵁鍖呭鍒惰繘/鍑鸿繖浜涚紦鍐插尯锛屽姞瀵嗕笌瑙ｅ瘑鍐呭瓨涔嬮棿鐨勫弽寮圭紦鍐茬瓑鏁堟搷浣滃凡缁忔槸鏁版嵁璺緞鐨勪竴閮ㄥ垎銆傜浜岀妯″紡浣跨敤鏅€氱殑 Linux kernel DMA API锛屽苟鍍?storvsc 涓€鏍烽殣寮忓湴閫氳繃 swiotlb 鍐呭瓨杩涜鍙嶅脊缂撳啿銆?

鏈€鍚庯紝VMBus 铏氭嫙 PCI 椹卞姩鍦?CoCo VM 涓渶瑕佺壒娈婂鐞嗐€侺inux PCI 璁惧椹卞姩浣跨敤 Linux PCI 瀛愮郴缁熸彁渚涚殑鏍囧噯 API 璁块棶 PCI 閰嶇疆绌洪棿銆傚湪 Hyper-V 涓婏紝杩欎簺鍑芥暟鐩存帴璁块棶 MMIO 绌洪棿锛岃闂細闄峰叆 Hyper-V 杩涜妯℃嫙銆備絾鍦?CoCo VM 涓紝鍐呭瓨鍔犲瘑闃绘 Hyper-V 璇诲彇瀹㈡埛鏈烘寚浠ゆ祦鏉ユā鎷熻璁块棶銆傚洜姝ゅ湪 CoCo VM 涓紝杩欎簺鍑芥暟蹇呴』鍙戣捣涓€涓?hypercall锛屼互鍙傛暟鏄惧紡鎻忚堪璇ヨ闂€傚弬瑙?_hv_pcifront_read_config() 鍜?_hv_pcifront_write_config() 浠ュ強鎸囩ず浣跨敤 hypercall 鐨?use_calls"鏍囧織銆?

### 鏈哄瘑 VMBus锛圕onfidential VMBus锛?


鏈哄瘑 VMBus 浣挎満瀵嗗鎴锋満鏃犻渶涓庝笉鍙俊鐨勪富鏈哄垎鍖哄拰涓嶅彲淇＄殑 hypervisor 浜や簰銆傜浉鍙嶏紝瀹㈡埛鏈轰緷璧栧彲淇＄殑 paravisor 涓庡鐞嗘晱鎰熸暟鎹殑璁惧閫氫俊銆傜‖浠讹紙SNP 鎴?TDX锛夊瀹㈡満鍐呭瓨鍜屽瘎瀛樺櫒鐘舵€佽繘琛屽姞瀵嗭紝鍚屾椂浣跨敤骞冲彴瀹夊叏澶勭悊鍣ㄥ paravisor 鏄犲儚杩涜搴﹂噺锛屼互纭繚鍙俊涓旀満瀵嗙殑 computing銆?

鏈哄瘑 VMBus 鍦ㄥ鎴锋満涓?paravisor 涔嬮棿鎻愪緵瀹夊叏鐨勯€氫俊閫氶亾锛岀‘淇濇晱鎰熸暟鎹€氳繃鍐呭瓨鍔犲瘑鍜屽瘎瀛樺櫒鐘舵€侀殧绂昏€屽厤鍙?hypervisor 绾у埆鐨勮闂€?

鏈哄瘑 VMBus 鏄満瀵嗚绠楋紙Confidential Computing锛孋oCo锛塚M锛堝湪 Hyper-V 鏈涓張绉?Isolated" VM锛夌殑鎵╁睍銆傛病鏈夋満瀵?VMBus 鏃讹紝瀹㈡埛鏈?VMBus 璁惧椹卞姩锛圴MBus 鏈涓殑"VSC"锛変笌杩愯浜?Hyper-V 涓绘満涓婄殑 VMBus 鏈嶅姟鍣紙VSP锛夐€氫俊銆傞€氫俊蹇呴』閫氳繃宸茶В瀵嗙殑鍐呭瓨锛屼互渚夸富鏈鸿兘澶熻闂€傛湁浜嗘満瀵?VMBus锛屼竴涓垨澶氫釜 VSP 椹荤暀鍦ㄥ鎴锋満 VM 涓彲淇＄殑 paravisor 灞傘€傜敱浜?paravisor 灞備篃杩愯浜庡姞瀵嗗唴瀛樹腑锛屼笌姝ょ被 VSP 閫氫俊鎵€鐢ㄧ殑鍐呭瓨鏃犻渶瑙ｅ瘑骞跺洜姝ゆ毚闇茬粰 Hyper-V 涓绘満銆俻aravisor 璐熻矗鍦ㄥ繀瑕佹椂涓?Hyper-V 涓绘満瀹夊叏鍦伴€氫俊銆?

鏁版嵁鐩存帴鍦?VM 涓?vPCI 璁惧锛堝張绉?PCI pass-thru 璁惧锛屽弬瑙?[vpci](vpci)锛変箣闂翠紶杈擄紝璇ヨ澶囩洿鎺ュ垎閰嶇粰 VTL2 骞舵敮鎸佸姞瀵嗗唴瀛樸€傚湪杩欑鎯呭喌涓嬶紝涓绘満鍒嗗尯鍜?hypervisor 閮芥棤娉曡闂鏁版嵁銆傚鎴锋満鍙渶涓?paravisor 寤虹珛 VMBus 杩炴帴锛岀敤浜庡鐞嗘晱鎰熸暟鎹殑閫氶亾锛岃€?paravisor 灏嗕笌璇ョ壒瀹氳澶囬€氫俊鐨勭粏鑺傛娊璞℃帀锛屽悜瀹㈡埛鏈烘彁渚涘湪 Hyper-V 椹卞姩涓凡鍙楁敮鎸佸崄骞寸殑鎴愮啛 VSP锛圴irtual Service Provider锛夋帴鍙ｃ€?

濡傛灉璁惧涓嶆敮鎸佸姞瀵嗗唴瀛橈紝paravisor 浼氭彁渚涘弽寮圭紦鍐诧紙bounce-buffering锛夛紝铏界劧鏁版嵁鏈姞瀵嗭紝浣嗗悗鍙伴〉涓嶄細閫氳繃 SLAT 鏄犲皠鍒颁富鏈哄垎鍖恒€傚敖绠″苟闈炰笉鍙兘锛屼絾涓庝紶缁?VMBus 杩炴帴锛堜富鏈哄垎鍖哄彲鐩存帴璁块棶鐢ㄤ簬閫氫俊鐨勫唴瀛橈級鐩告瘮锛屼富鏈哄垎鍖烘笚閫忥紙exfiltrate锛夋暟鎹鍥伴毦寰楀銆?

涓嬮潰鏄紶缁?VMBus 杩炴帴鐨勬暟鎹祦锛坄C` 浠ｈ〃瀹㈡埛绔垨 VSC锛宍S` 浠ｈ〃鏈嶅姟绔垨 VSP锛宍DEVICE` 鏄墿鐞嗚澶囷紝鍙兘
```
  +---- GUEST ----+       +----- DEVICE ----+        +----- HOST -----+
  |               |       |                 |        |                |
  |               |       |                 |        |                |
  |               |       |                 ==========                |
  |               |       |                 |        |                |
  |               |       |                 |        |                |
  |               |       |                 |        |                |
  +----- C -------+       +-----------------+        +------- S ------+
         ||                                                   ||
         ||                                                   ||
  +------||------------------ VMBus --------------------------||------+
  |                     Interrupts, MMIO                              |
  +-------------------------------------------------------------------+

```
```
  +---- GUEST --------------- VTL0 ------+               +-- DEVICE --+
  |                                      |               |            |
  | +- PARAVISOR --------- VTL2 -----+   |               |            |
  | |     +-- VMBus Relay ------+    ====+================            |
  | |     |   Interrupts, MMIO  |    |   |               |            |
  | |     +-------- S ----------+    |   |               +------------+
  | |               ||               |   |
  | +---------+     ||               |   |
  | |  Linux  |     ||    OpenHCL    |   |
  | |  kernel |     ||               |   |
  | +---- C --+-----||---------------+   |
  |       ||        ||                   |
  +-------++------- C -------------------+               +------------+
          ||                                             |    HOST    |
          ||                                             +---- S -----+
  +-------||----------------- VMBus ---------------------------||-----+
  |                     Interrupts, MMIO                              |
  +-------------------------------------------------------------------+

```
鎻愪緵鏈哄瘑 VMBus 閫氶亾鐨?VMBus relay 瀹炵幇锛屼綔涓?OpenHCL paravisor 鐨勪竴閮ㄥ垎鍦?OpenVMM 椤圭洰涓彲鐢ㄣ€傛洿澶氫俊鎭鍙傝€?

  - https://openvmm.dev/锛屼互鍙?
  - https://github.com/microsoft/openvmm

浠ヤ簡瑙?OpenHCL paravisor銆?

涓?paravisor 涓€璧疯繍琛岀殑瀹㈡埛鏈哄繀椤诲湪杩愯鏃剁‘瀹氬綋鍓?paravisor 鏄惁鏀寔鏈哄瘑 VMBus銆倄86_64 鐗瑰畾鐨勬柟娉曚緷璧栦簬 CPUID Virtualization Stack leaf锛汚RM64 瀹炵幇鍦ㄨ繍琛?ARM CCA 瀹㈡埛鏈烘椂棰勬湡鏃犳潯浠舵敮鎸佹満瀵?VMBus銆?

鏈哄瘑 VMBus 鏄暣涓?VMBus 杩炴帴浠ュ強鎵€鍒涘缓鐨勬瘡涓?VMBus 閫氶亾鐨勪竴涓壒寰併€傚綋寤虹珛鏈哄瘑 VMBus 杩炴帴鏃讹紝paravisor 鍚戝鎴锋満鎻愪緵鐢ㄤ簬 VMBus 璁惧鍒涘缓鍜屽垹闄ょ殑娑堟伅浼犻€掕矾寰勶紝骞舵彁渚涙瘡 CPU 鐨勫悎鎴愪腑鏂帶鍒跺櫒锛圫ynIC锛夛紝灏卞儚 Hyper-V 涓绘満鎻愪緵鐨?SynIC 涓€鏍枫€傛彁渚涚粰瀹㈡埛鏈虹殑姣忎釜 VMBus 璁惧閮芥寚绀哄叾鍙備笌鏈哄瘑 VMBus 鐨勭▼搴︺€傝 offer 鎸囩ず璁惧鏄惁浣跨敤鍔犲瘑鐜舰缂撳啿鍖猴紝浠ュ強璁惧鏄惁瀵圭幆褰㈢紦鍐插尯涔嬪瀹屾垚鐨?DMA 浣跨敤鍔犲瘑鍐呭瓨銆傚浜庝娇鐢ㄥ悓涓€鏈哄瘑 VMBus 杩炴帴鐨勪笉鍚岃澶囷紝杩欎簺璁剧疆鍙兘涓嶅悓銆?

灏界杩欎簺璁剧疆鏄垎寮€鐨勶紝浣嗗湪瀹炶返涓彧浼氭槸浠呭姞瀵嗙幆褰㈢紦鍐插尯锛屾垨鍚屾椂鍔犲瘑鐜舰缂撳啿鍖哄拰澶栭儴鏁版嵁銆傚鏋滈€氶亾鐢?paravisor 浠ユ満瀵?VMBus 鎻愪緵锛岀幆褰㈢紦鍐插尯鎬绘槸鍙互鍔犲瘑锛屽洜涓哄畠涓ユ牸鐢ㄤ簬 VTL2 paravisor 涓?VTL0 瀹㈡埛鏈轰箣闂寸殑閫氫俊銆傜劧鑰岋紝鍏朵粬鍐呭瓨鍖哄煙甯哥敤浜?DMA 绛夛紝鍥犳瀹冧滑闇€瑕佸簳灞傜‖浠跺彲璁块棶锛屽苟涓斿繀椤绘湭鍔犲瘑锛堥櫎闈炶澶囨敮鎸佸姞瀵嗗唴瀛橈級銆傜洰鍓嶏紝OpenHCL 涓病鏈変换浣曟敮鎸佸姞瀵嗗閮ㄥ唴瀛樼殑 VSP锛屼絾鏈潵鐗堟湰棰勬湡浼氬惎鐢ㄦ鑳藉姏銆?

鐢变簬鏈哄瘑 VMBus 涓婄殑鏌愪簺璁惧鍙兘闇€瑕佸凡瑙ｅ瘑鐨勭幆褰㈢紦鍐插尯鍜?DMA 浼犺緭锛屽鎴锋満蹇呴』涓庝袱涓?SynIC 浜や簰鈥斺€斾竴涓槸 paravisor 鎻愪緵鐨勶紝鍙︿竴涓槸鍦ㄤ笉鎻愪緵鏈哄瘑 VMBus 鏃剁敱 Hyper-V 涓绘満鎻愪緵鐨勩€備腑鏂€绘槸鐢?paravisor SynIC 鍙戝嚭淇″彿锛屼絾瀹㈡埛鏈哄繀椤诲湪涓や釜 SynIC 涓婃鏌ユ秷鎭拰閫氶亾涓柇銆?

鍦ㄦ満瀵?VMBus 鐨勬儏鍐典笅锛屽鎴锋満瀵?SynIC 鐨勫父瑙勮闂細琚?paravisor 鎷︽埅锛堣繖鍖呮嫭鍚勭 MSR锛屽 SIMP 鍜?SIEFP锛屼互鍙婂儚 HvPostMessage 鍜?HvSignalEvent 杩欐牱鐨?hypercall锛夈€傚鏋滃鎴锋満纭疄鎯宠涓?hypervisor 閫氫俊锛屽畠蹇呴』浣跨敤鐗规畩鏈哄埗锛圫NP 涓婄殑 GHCB 椤碉紝鎴?TDX 涓婄殑 tdcall锛夈€傛秷鎭彲浠ユ槸浠讳竴绉嶏細浣跨敤鏈哄瘑 VMBus 鏃讹紝娑堟伅浣跨敤 paravisor SynIC锛涘鏋滃鎴锋満閫夋嫨鐩存帴涓?hypervisor 閫氫俊锛屽垯浣跨敤 hypervisor SynIC銆傚浜庝腑鏂俊鍙凤紝鏌愪簺閫氶亾鍙兘杩愯鍦ㄤ富鏈轰笂锛堥潪鏈哄瘑锛屼娇鐢?VMBus relay锛夊苟浣跨敤 hypervisor SynIC锛屾煇浜涜繍琛屽湪 paravisor 涓婂苟浣跨敤鍏?SynIC銆俁elIDs 鐢?OpenHCL VMBus 鏈嶅姟鍣ㄥ崗璋冿紝鏃犺閫氶亾璧锋簮浜庝富鏈鸿繕鏄?paravisor锛岄兘淇濊瘉鍞竴銆?

### load_unaligned_zeropad()


鍦ㄥ姞瀵嗕笌瑙ｅ瘑涔嬮棿杞崲鍐呭瓨鏃讹紝set_memory_encrypted() 鎴?set_memory_decrypted() 鐨勮皟鐢ㄨ€呰礋璐ｇ‘淇濆唴瀛樻湭琚娇鐢紝涓斿湪杞崲杩涜鏈熼棿涓嶈寮曠敤銆傝浆鎹㈡湁澶氫釜姝ラ锛屽苟鍖呭惈涓?Hyper-V 涓绘満鐨勪氦浜掋€傚湪鍏ㄩ儴姝ラ瀹屾垚涔嬪墠锛屽唴瀛樺浜庝笉涓€鑷寸姸鎬併€傚湪鐘舵€佷笉涓€鑷存椂杩涜寮曠敤鍙兘瀵艰嚧鏃犳硶骞插噣淇鐨勫紓甯搞€?

鐒惰€岋紝kernel 鐨?load_unaligned_zeropad() 鏈哄埗鍙兘浜х敓璋冪敤鑰呮棤娉曢樆姝㈢殑娓哥寮曠敤锛屽洜姝ゅ湪 #VC 鎴?#VE 寮傚父澶勭悊绋嬪簭涓湁鐗瑰畾浠ｇ爜淇姝ょ被鎯呭喌銆備絾鍦?Hyper-V 涓婅繍琛岀殑 CoCo VM 鍙兘琚厤缃负涓?paravisor 涓€璧疯繍琛岋紝涓?#VC 鎴?#VE 寮傚父琚矾鐢卞埌 paravisor銆傛病鏈夋灦鏋勫眰闈㈢殑鏂规硶灏嗚繖浜涘紓甯歌浆鍙戝洖瀹㈡埛鏈?kernel锛屽湪杩欑鎯呭喌涓嬶紝#VC/#VE 澶勭悊绋嬪簭涓殑 load_unaligned_zeropad() 淇浠ｇ爜涓嶄細杩愯銆?

涓洪伩鍏嶆闂锛岀敤浜庨€氱煡 hypervisor 杞崲鍙戠敓鐨?Hyper-V 鐗瑰畾鍑芥暟鍦ㄨ浆鎹㈣繘琛屾湡闂村皢椤垫爣璁颁负"not present"銆傚鏋?load_unaligned_zeropad() 瀵艰嚧娓哥寮曠敤锛屼細鐢熸垚鏅€氶〉閿欒锛坧age fault锛夎€屼笉鏄?#VC 鎴?#VE锛屽苟涓?load_unaligned_zeropad() 鍩轰簬椤甸敊璇殑澶勭悊绋嬪簭浼氫慨澶嶈寮曠敤銆傚綋鍔犲瘑/瑙ｅ瘑杞崲瀹屾垚鏃讹紝椤典細閲嶆柊鏍囪涓?present"銆傚弬瑙?hv_vtom_clear_present() 鍜?hv_vtom_set_host_visibility()銆?
