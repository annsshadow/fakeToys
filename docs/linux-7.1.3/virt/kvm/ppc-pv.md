
## PPC KVM 鍗婅櫄鎷熷寲锛坧aravirtual锛夋帴鍙?


KVM on PowerPC 鐨勫熀鏈繍琛屽師鐞嗘槸锛氬唴鏍告€佷唬鐮佷互 PR=1锛堢敤鎴锋€侊級鏂瑰紡杩愯浜庡鎴锋満涓€傜壒鏉冩寚浠ょ敱姝や互鐩稿簲鐨勬柟寮忛櫡鍏ワ紙trap锛夊苟琚ā鎷熴€?

浣嗕笉骞哥殑鏄紝杩欎篃鏈夊叾缂洪櫡銆傜浉褰撲竴閮ㄥ垎鐗规潈鎸囦护鍗充究鏈彲浠ヤ笉鍚屾柟寮忓鐞嗭紝涔熶細涓嶅繀瑕佸湴杩斿洖鍒版垜浠殑 hypervisor銆?

PPC PV 鎺ュ彛姝ｆ槸鐢ㄤ簬瑙ｅ喅杩欎竴闂銆傚畠灏嗙壒鏉冩寚浠よ浆鎹负闈炵壒鏉冩寚浠ゆ潵杈呭姪 hypervisor锛屼粠鑰屽皢铏氭嫙鍖栧紑閿€鍦ㄦ垜鐨勫熀鍑嗘祴璇曚腑闄嶄綆浜嗙害 50%銆?

璇ユ帴鍙ｇ殑浠ｇ爜浣嶄簬 `arch/powerpc/kernel/kvm*`銆?


## 鏌ヨ瀛樺湪鎬э紙Querying existence锛?

瑕佸垽鏂嚜宸辨槸鍚﹁繍琛屼簬 KVM 涔嬩笂锛屽彲鍒╃敤璁惧鏍戯紙device tree锛夈€傚湪杩愯浜?KVM 鐨?Linux 涓紝浼氬瓨鍦?`/hypervisor` 鑺傜偣銆傝鑺傜偣鍖呭惈涓€涓€间负 "linux,kvm" 鐨?`compatible` 灞炴€с€?

涓€鏃︾‘瀹氳嚜宸辫繍琛屼簬鏀寔 PV 鐨?KVM 涔嬩笂锛屽嵆鍙娇鐢ㄤ笅鏂囨弿杩扮殑 hypercall銆?


## KVM hypercalls

鍦ㄨ澶囨爲鐨?`/hypervisor` 鑺傜偣涓紝鏈変竴涓悕涓?`hypercall-instructions` 鐨勫睘鎬с€傝灞炴€у寘鍚瀯鎴愪竴娆?hypercall 鐨勬渶澶?4 鏉?opcode銆傝鍙戣捣 hypercall锛屽彧闇€鎵ц杩欎簺鎸囦护鍗冲彲銆?

鍙傛暟绾﹀畾濡備笅锛?

========	================	================
瀵勫瓨鍣?IN OUT
========	================	================
r0 - volatile
r3 1st 鍙傛暟 杩斿洖 code
r4 2nd 鍙傛暟 1st output 鍊?
r5 3rd 鍙傛暟 2nd output 鍊?
r6 4th 鍙傛暟 3rd output 鍊?
r7 5th 鍙傛暟 4th output 鍊?
r8 6th 鍙傛暟 5th output 鍊?
r9 7th 鍙傛暟 6th output 鍊?
r10 8th 鍙傛暟 7th output 鍊?
r11 hypercall 缂栧彿	8th output 鍊?
r12 - volatile
========	================	================

hypercall 鐨勫畾涔夊湪閫氱敤浠ｇ爜涓叡浜紝x86 涓?powerpc 浣跨敤鐩稿悓鐨?hypercall 缂栧彿锛涘紓甯告儏鍐垫槸锛孠VM hypercall 闇€瑕佷笌 KVM vendor code锛?2 << 16锛夊仛鎸変綅鎴栥€?

杩斿洖鐮佺害瀹氬涓嬶細

==== =========================
Code Meaning
==== =========================
0 Success
12 Hypercall implemented
<0 閿欒
==== =========================


## magic 椤?

涓哄惎鐢?guest 涓?hypervisor 涔嬮棿鐨勯€氫俊锛屽紩鍏ヤ簡涓€椤垫柊鐨勫叡浜唴瀛橈紝鍏朵腑鍖呭惈閮ㄥ垎浠?supervisor 鍙鐨勫瘎瀛樺櫒鐘舵€併€俫uest 鍙€氳繃 KVM hypercall `KVM_HC_PPC_MAP_MAGIC_PAGE` 鏄犲皠璇ュ叡浜〉銆?

璇?hypercall 鐢?guest 鍙戣捣鍚庯紝鎬绘槸浼氭妸 magic 椤垫槧灏勫埌鏈熸湜鐨勪綅缃€傜涓€涓弬鏁拌〃绀哄惎鐢?MMU 鏃剁殑鏈夋晥鍦板潃锛坋ffective address锛夛紱绗簩涓弬鏁拌〃绀哄疄妯″紡锛坮eal mode锛変笅鐨勫湴鍧€锛岄€傜敤浜庣浉搴旂洰鏍囥€傜洰鍓嶏紝magic 椤垫€绘槸鏄犲皠鍒?-4096 澶勩€傝繖鏍蜂究鍙娇鐢ㄧ粷瀵瑰姞杞?瀛樺偍鍑芥暟鏉ヨ闂€備緥濡傦細

```
	ld	rX, -4096(0)
```

璇ユ帴鍙ｈ璁捐涓哄彲鎵╁睍鐨勶紝浠ヤ究鏃ュ悗鍚?magic 椤垫坊鍔犳洿澶氬瘎瀛樺櫒銆傚悜 magic 椤垫坊鍔犲瓧娈垫椂锛屽簲瀹氫箟鏂扮殑 hypercall 鐗规€т綅鏉ユ寚绀?host 鎻愪緵浜嗘洿澶氬瘎瀛樺櫒銆傝嫢 host 鏀寔璇ラ檮鍔犵壒鎬э紝鍗冲彲鍔犱互鍒╃敤銆?

magic 椤电殑甯冨眬鐢?`arch/powerpc/include/uapi/asm/kvm_para.h` 涓殑缁撴瀯浣?`kvm_vcpu_arch_shared` 鎻忚堪銆?


## Magic 椤?鐗规€?

鏄犲皠 magic 椤典娇鐢?KVM hypercall `KVM_HC_PPC_MAP_MAGIC_PAGE`锛屽叾绗簩涓繑鍥炲€间細浼犵粰 guest銆傜浜屼釜杩斿洖鍊煎寘鍚竴涓綅鍥撅紝鎸囩ず magic 椤靛唴鍙敤鐨勭壒鎬с€?

鐩墠 magic 椤靛彲鐢ㄧ殑澧炲己鐗规€у涓嬶細

============================ =======================================
KVM_MAGIC_FEAT_SR Maps SR 瀵勫瓨鍣?r/w magic 椤?
KVM_MAGIC_FEAT_MAS0_TO_SPRG7	Maps MASn, ESR, PIR high SPRGs
============================ =======================================

瑕佸惎鐢?magic 椤电殑澧炲己鐗规€э紝璇峰厛妫€鏌ヨ鐗规€ф槸鍚﹀瓨鍦紙浣跨敤鐩稿簲鐨勭壒鎬т綅锛夛紒


## Magic 椤?鏍囧織

闄や簡鎸囩ず host 鏄惁鏀寔鏌愪釜鐗瑰畾鐗规€х殑"鐗规€?浣嶄箣澶栵紝杩樺瓨鍦ㄤ竴绉?guest 鍛婄煡 host "鑷繁涔熸敮鎸佹煇鑳藉姏"鐨勯€氶亾锛岀О涓?鏍囧織"銆?

鏍囧織閫氳繃鏈夋晥鍦板潃锛圗ffective address锛夌殑浣?12 浣嶄紶缁?host銆?

鐩墠 guest 鍙毚闇茬殑鏍囧織濡備笅锛?

MAGIC_PAGE_FLAG_NOT_MAPPED_NX Guest 鑳芥纭鐞?magic 椤电殑 NX 浣?


## MSR 浣?

MSR 涓寘鍚竴浜涢渶瑕?hypervisor 浠嬪叆鐨勪綅锛屼互鍙婁竴浜涢渶瑕佺洿鎺ョ敱 hypervisor 瑙ｉ噴銆佸湪杩涘叆 guest 鏃朵笉褰卞搷 hypervisor 琛屼负鐨勪綅銆?

浠ヤ笅浣嶅彲鍦?guest 鍐呭畨鍏ㄨ缃細

- MSR_EE
- MSR_RI

瀵?MSR 鐨勪綅杩涜淇敼鏃讹紝浠嶈浣跨敤 `mtmsr(d)`銆?


## Patched instructions锛堣ˉ涓佸寲鎸囦护锛?

"ld" 涓?"std" 鎸囦护鍒嗗埆琚浆鎹负 "lwz" 涓?"stw" 鎸囦护锛堝湪 32 浣嶇郴缁熶笂锛屽苟鍔犱笂鍋忕Щ閲?4 浠ラ€傚簲澶х搴忥級銆?

浠ヤ笅鏄?Linux 鍐呮牳鍦?guest 杩愯鏃舵墍鎵ц鐨勬槧灏勩€傚疄鐜拌繖浜涙槧灏勬槸鍙€夌殑鈥斺€旇嫢鎸囦护闄峰叆锛屼粛浼氭寜鍏变韩椤垫柟寮忓鐞嗭紱璋冪敤鐗规潈鎸囦护鍚屾牱鍙銆?

======================= ================================
mfmsr	rX ld	rX, magic_page->msr
mfsprg	rX, 0 ld	rX, magic_page->sprg0
mfsprg	rX, 1 ld	rX, magic_page->sprg1
mfsprg	rX, 2 ld	rX, magic_page->sprg2
mfsprg	rX, 3 ld	rX, magic_page->sprg3
mfsrr0	rX ld	rX, magic_page->srr0
mfsrr1	rX ld	rX, magic_page->srr1
mfdar	rX ld	rX, magic_page->dar
mfdsisr	rX lwz	rX, magic_page->dsisr
mtmsr	rX std	rX, magic_page->msr
mtsprg	0, rX std	rX, magic_page->sprg0
mtsprg	1, rX std	rX, magic_page->sprg1
mtsprg	2, rX std	rX, magic_page->sprg2
mtsprg	3, rX std	rX, magic_page->sprg3
mtsrr0	rX std	rX, magic_page->srr0
mtsrr1	rX std	rX, magic_page->srr1
mtdar	rX std	rX, magic_page->dar
mtdsisr	rX stw	rX, magic_page->dsisr
tlbsync nop
mtmsrd	rX, 0 b	<special mtmsr 绔犺妭>
mtmsr	rX b	<special mtmsr 绔犺妭>
mtmsrd	rX, 1 b	<special mtmsrd 绔犺妭>
[Book3S ]
mtsrin	rX, rY b	<special mtsrin 绔犺妭>
[BookE ]
wrteei	[0|1] b	<special wrteei 绔犺妭>
======================= ================================

瀵逛簬閭ｄ簺闇€瑕佹洿澶氶€昏緫鏉ュ垽鏂槸鍔犺浇杩樻槸瀛樺偍鎸囦护琚氦浠樼殑鎸囦护锛屽惎鐢ㄨˉ涓侊紙patching锛夊悗锛屼細鍦ㄥ疄鏃剁炕璇戞寚浠ょ殑 RAM 鍛ㄥ洿淇濈暀绌洪棿銆傚叾杩囩▼濡備笅锛?

1) 灏嗘ā鎷熶唬鐮佸鍒跺埌鍐呭瓨
2) 琛ヤ竵鍖栦唬鐮佷互閫傞厤琚ā鎷熺殑鎸囦护
3) 琛ヤ竵鍖栦唬鐮佷娇鍏惰繑鍥炲師濮?pc + 4
4) 灏嗚琛ヤ竵鍖栫殑鍘熷鎸囦护鍒嗘敮鍒版柊浠ｇ爜

鐢辨锛屽彲鐢ㄤ换鎰忔暟閲忕殑浠ｇ爜鏇挎崲鍗曟潯鎸囦护銆備緥濡傦紝杩欏厑璁告垜浠€氳繃璁剧疆 EE=1 鏉ユ鏌ユ寕璧风殑涓柇銆?


## Hypercall ABIs锛圞VM PowerPC锛?

1) KVM hypercalls (ePAPR)

绗﹀悎 ePAPR 鐨?hypercall 瀹炵幇锛堝鍓嶆墍杩帮級銆傚嵆渚块€氱敤 hypercall 宸插疄鐜帮紙濡?ePAPR idle hcall锛夛紝涔熷彲鐢ㄣ€傞€傜敤浜庣浉搴?targets銆?

2) PAPR hypercalls

杩愯 server PowerPC PAPR guest锛坄-M pseries` QEMU锛夐渶瑕?PAPR hypercall銆傝繖浜?hypercall 涓?pHyp锛圥OWER hypervisor锛夊疄鐜扮殑鐩稿悓銆備竴閮ㄥ垎鐢卞唴鏍稿鐞嗭紝涓€閮ㄥ垎鐢辩敤鎴风┖闂村鐞嗐€傚彲鐢ㄤ簬 book3s_64銆?

3) OSI hypercalls

Mac-on-Linux 鐢ㄦ埛涓?KVM PowerPC 鎻愪緵浜嗚嚜宸辩殑 hypercall锛堟部鐢ㄨ嚜鏃╂湡鐨?KVM锛夈€備负淇濇寔鍏煎鎬ц€屾敮鎸佽繖浜?hypercall銆傚畠浠細琚浆鍙戝埌鐢ㄦ埛绌洪棿銆傚 book3s_32 鏈夌敤锛屽悓鏍烽€傜敤浜?book3s_64銆?
