## DRM 鍐呴儴鏈哄埗


鏈珷璁板綍涓庨┍鍔ㄤ綔鑰呬互鍙婇偅浜涗负鐜版湁椹卞姩娣诲姞鏈€鏂扮壒鎬ф敮鎸佺殑寮€鍙戣€呯浉鍏崇殑 DRM 鍐呴儴鏈哄埗銆?
棣栧厛锛屾垜浠洖椤句竴浜涘吀鍨嬬殑椹卞姩鍒濆鍖栬姹傦紝渚嬪寤虹珛鍛戒护缂撳啿鍖恒€佸垱寤哄垵濮嬭緭鍑洪厤缃紝浠ュ強鍒濆鍖栨牳蹇冩湇鍔°€傚悗缁珷鑺傚皢鏇磋缁嗗湴浠嬬粛鏍稿績鍐呴儴鏈哄埗锛屽苟鎻愪緵瀹炵幇璇存槑涓庣ず渚嬨€?
DRM 灞備负鍥惧舰椹卞姩鎻愪緵鑻ュ共鏈嶅姟锛屽叾涓澶氱敱瀹冮€氳繃 libdrm锛堝皝瑁呬簡澶ч儴鍒?DRM ioctl 鐨勫簱锛夋墍鎻愪緵鐨勫簲鐢ㄧ▼搴忔帴鍙ｉ┍鍔ㄣ€傝繖浜涙湇鍔″寘鎷?vblank 浜嬩欢澶勭悊銆佸唴瀛樼鐞嗐€佽緭鍑虹鐞嗐€佸抚缂撳啿绠＄悊銆佸懡浠ゆ彁浜や笌鏍呮爮锛坒encing锛夈€佹寕璧?鎭㈠鏀寔锛屼互鍙?DMA 鏈嶅姟銆?
## 椹卞姩鍒濆鍖?

姣忎釜 DRM 椹卞姩鐨勬牳蹇冮兘鏄竴涓?:c:type:`struct drm_driver <drm_driver>` 缁撴瀯浣撱€傞┍鍔ㄩ€氬父浼氶潤鎬佸垵濮嬪寲涓€涓?drm_driver 缁撴瀯浣擄紝鐒跺悗灏嗗叾浼犵粰 drm_dev_alloc() 浠ュ垎閰嶄竴涓澶囧疄渚嬨€傚湪璁惧瀹炰緥瀹屽叏鍒濆鍖栦箣鍚庯紝灏卞彲浠ヤ娇鐢?drm_dev_register() 灏嗗叾娉ㄥ唽锛堟敞鍐屽悗鐢ㄦ埛绌洪棿鍗冲彲璁块棶锛夈€?
`struct drm_driver <drm_driver>` 缁撴瀯浣撳寘鍚弿杩伴┍鍔ㄥ強鍏舵墍鏀寔鐗规€х殑闈欐€佷俊鎭紝浠ュ強渚?DRM 鏍稿績璋冪敤浠ュ疄鐜?DRM API 鐨勬柟娉曟寚閽堛€傛垜浠皢棣栧厛閫氳 :c:type:`struct drm_driver <drm_driver>` 鐨勯潤鎬佷俊鎭瓧娈碉紝鐒跺悗鍦ㄥ悗缁珷鑺傜敤鍒板悇涓搷浣滄椂鍐嶈缁嗘弿杩板畠浠€?
### 椹卞姩淇℃伅


#### 涓荤増鏈彿銆佹鐗堟湰鍙蜂笌琛ヤ竵绾у埆


int major; int minor; int patchlevel;
DRM 鏍稿績閫氳繃涓€涓富鐗堟湰鍙枫€佹鐗堟湰鍙蜂笌琛ヤ竵绾у埆鐨勪笁鍏冪粍鏉ユ爣璇嗛┍鍔ㄧ増鏈€傝淇℃伅浼氬湪鍒濆鍖栨椂鎵撳嵃鍒板唴鏍告棩蹇楋紝骞堕€氳繃 DRM_IOCTL_VERSION ioctl 浼犵粰鐢ㄦ埛绌洪棿銆?
涓荤増鏈彿涓庢鐗堟湰鍙蜂篃鐢ㄤ簬鏍￠獙浼犵粰 DRM_IOCTL_SET_VERSION 鐨勬墍璇锋眰椹卞姩 API 鐗堟湰銆傚綋椹卞姩 API 鍦ㄤ笉鍚屾鐗堟湰涔嬮棿鍙戠敓鍙樺寲鏃讹紝搴旂敤绋嬪簭鍙互璋冪敤 DRM_IOCTL_SET_VERSION 鏉ラ€夋嫨鏌愪釜鐗瑰畾鐨?API 鐗堟湰銆傚鏋滄墍璇锋眰鐨勪富鐗堟湰鍙蜂笌椹卞姩涓荤増鏈彿涓嶄竴鑷达紝鎴栬€呮墍璇锋眰鐨勬鐗堟湰鍙峰ぇ浜庨┍鍔ㄦ鐗堟湰鍙凤紝DRM_IOCTL_SET_VERSION 璋冪敤灏嗚繑鍥為敊璇€傚惁鍒欏皢浠ユ墍璇锋眰鐨勭増鏈皟鐢ㄩ┍鍔ㄧ殑 set_version() 鏂规硶銆?
#### 鍚嶇О涓庢弿杩?

char \**name; char \**desc; char \*date;
椹卞姩鍚嶇О浼氬湪鍒濆鍖栨椂鎵撳嵃鍒板唴鏍告棩蹇楋紝鐢ㄤ簬 IRQ 娉ㄥ唽锛屽苟閫氳繃 DRM_IOCTL_VERSION 浼犵粰鐢ㄦ埛绌洪棿銆?
椹卞姩鎻忚堪鏄竴涓函淇℃伅鎬х殑瀛楃涓诧紝閫氳繃 DRM_IOCTL_VERSION ioctl 浼犵粰鐢ㄦ埛绌洪棿锛屽唴鏍告湰韬笉鍐嶄娇鐢ㄥ畠銆?
### 妯″潡鍒濆鍖?

   :doc: overview

### 璁惧瀹炰緥涓庨┍鍔ㄥ鐞?

   :doc: driver instance overview

   :internal:

   :internal:

   :export:

### 椹卞姩鍔犺浇


#### 缁勪欢杈呭姪锛圕omponent Helper锛夌敤娉?

   :doc: component helper usage recommendations

#### 鍐呭瓨绠＄悊鍣ㄥ垵濮嬪寲


姣忎釜 DRM 椹卞姩閮介渶瑕佷竴涓唴瀛樼鐞嗗櫒锛屽苟涓斿繀椤诲湪鍔犺浇鏃跺垵濮嬪寲銆侱RM 鐩墠鍖呭惈涓や釜鍐呭瓨绠＄悊鍣細杞崲琛ㄧ鐞嗗櫒锛圱TM锛孴ranslation Table Manager锛変笌鍥惧舰鎵ц绠＄悊鍣紙GEM锛孏raphics Execution Manager锛夈€傛湰鏂囨。浠呮弿杩?GEM 鍐呭瓨绠＄悊鍣ㄧ殑浣跨敤銆傝瑙?? 銆?
#### 鏉傞」璁惧閰嶇疆


鍦ㄩ厤缃?PCI 璁惧鏃讹紝鍙︿竴椤瑰彲鑳介渶瑕佺殑浠诲姟鏄槧灏勮棰?BIOS锛圴BIOS锛夈€傚湪璁稿璁惧涓婏紝VBIOS 鎻忚堪浜嗚澶囬厤缃€丩CD 闈㈡澘鏃跺簭锛堝鏋滄湁锛夛紝骞跺寘鍚寚绀鸿澶囩姸鎬佺殑鏍囧織浣嶃€傛槧灏?BIOS 鍙互浣跨敤 pci_map_rom() 璋冪敤锛岃繖鏄竴涓究鎹峰嚱鏁帮紝璐熻矗鏄犲皠瀹為檯鐨?ROM鈥斺€旀棤璁哄畠鏄褰卞瓙澶嶅埗鍒板唴瀛樹腑锛堥€氬父鍦ㄥ湴鍧€ 0xc0000锛夛紝杩樻槸瀛樺湪浜?PCI 璁惧鐨?ROM BAR 涓€傛敞鎰忥細鍦?ROM 琚槧灏勪笖鎻愬彇浜嗕换浣曞繀瑕佷俊鎭箣鍚庯紝搴斿綋灏嗗叾鍙栨秷鏄犲皠锛涘湪璁稿璁惧涓婏紝ROM 鍦板潃瑙ｇ爜鍣ㄤ笌鍏朵粬 BAR 鍏变韩锛屽洜姝よ鍏朵繚鎸佹槧灏勫彲鑳藉鑷存寕璧锋垨鍐呭瓨鎹熷潖绛変笉鑹涓恒€?
### 鍙楃鐞嗚祫婧?

   :doc: managed resources

   :export:

   :internal:

## 鎵撳紑/鍏抽棴銆佹枃浠舵搷浣滀笌 IOCTL


### 鏂囦欢鎿嶄綔


   :doc: file operations

   :internal:

   :export:

## 鏉傞」宸ュ叿


### 鎵撳嵃鍣?

   :doc: print

   :internal:

   :export:

### 宸ュ叿鍑芥暟


   :doc: drm utils

   :internal:


## 鍗曞厓娴嬭瘯


### KUnit


KUnit锛堝唴鏍稿崟鍏冩祴璇曟鏋讹級涓?Linux 鍐呮牳涓殑鍗曞厓娴嬭瘯鎻愪緵浜嗕竴涓€氱敤妗嗘灦銆傛湰鑺備粙缁?DRM 瀛愮郴缁熺殑鍏蜂綋鍐呭銆傛湁鍏?KUnit 鐨勪竴鑸俊鎭紝璇峰弬闃?Documentation/dev-tools/kunit/start.rst銆?
#### 濡備綍杩愯娴嬭瘯锛?

涓轰究浜庤繍琛屾祴璇曞浠讹紝`drivers/gpu/drm/tests/.kunitconfig` 涓彁渚涗簡涓€涓厤缃枃浠躲€傚畠鍙互鎸夊涓嬫柟寮忚 `kunit.py` 浣跨敤锛?

	$ ./tools/testing/kunit/kunit.py run --kunitconfig=drivers/gpu/drm/tests \
		--kconfig_add CONFIG_VIRTIO_UML=y \
		--kconfig_add CONFIG_UML_PCI_OVER_VIRTIO=y

	`.kunitconfig` 涓寘鍚殑閰嶇疆搴斿綋灏藉彲鑳介€氱敤銆俙CONFIG_VIRTIO_UML` 涓?	`CONFIG_UML_PCI_OVER_VIRTIO` 鏈鍖呭惈鍦ㄥ唴锛屽洜涓哄畠浠粎鐢ㄤ簬鐢ㄦ埛妯″紡
	Linux锛圲ser Mode Linux锛夈€?
#### KUnit 瑕嗙洊瑙勫垯


KUnit 鏀寔姝ｉ€愭鍔犲叆鍒?DRM 妗嗘灦涓庤緟鍔╁嚱鏁颁腑銆傜洰鍓嶅妗嗘灦鍜岃緟鍔╁嚱鏁板苟娌℃湁蹇呴』鎷ユ湁 KUnit 娴嬭瘯鐨勬櫘閬嶈姹傘€備笉杩囷紝濡傛灉鏌愪釜琛ヤ竵褰卞搷鍒板凡琚?KUnit 娴嬭瘯瑕嗙洊鐨勫嚱鏁版垨杈呭姪鍑芥暟锛屼笖鏀瑰姩闇€瑕佺浉搴旀祴璇曪紝鍒欏繀椤绘彁渚涙祴璇曘€?
## 鏃х増鏀寔浠ｇ爜


鏈妭闈炲父绠€瑕佸湴浠嬬粛涓€浜涙棫鐗堟敮鎸佷唬鐮侊紝瀹冧滑浠呰閭ｄ簺瀵瑰簳灞傝澶囧仛浜嗘墍璋?shadow-attach锛堝奖瀛愰檮鍔狅級銆佽€岄潪娉ㄥ唽涓虹湡姝ｉ┍鍔ㄧ殑鏃?DRM 椹卞姩浣跨敤銆傝繖涔熷寘鎷竴浜涙棫鐨勯€氱敤缂撳啿鍖虹鐞嗕笌鍛戒护鎻愪氦浠ｇ爜銆傚湪鏂扮殑鐜颁唬椹卞姩涓笉瑕佷娇鐢ㄥ叾涓换浣曞唴瀹广€?
### 鏃х増鎸傝捣/鎭㈠


DRM 鏍稿績鎻愪緵浜嗕竴浜涙寕璧?鎭㈠浠ｇ爜锛屼絾鎯宠瀹屾暣鎸傝捣/鎭㈠鏀寔鐨勯┍鍔ㄥ簲褰撴彁渚?save() 涓?restore() 鍑芥暟銆傚畠浠細鍦ㄦ寕璧枫€佷紤鐪犳垨鎭㈠鏃惰璋冪敤锛屽苟搴旀墽琛岃澶囧湪璺ㄦ寕璧锋垨浼戠湢鐘舵€佹椂鎵€瑕佹眰鐨勪换浣曠姸鎬佷繚瀛樻垨鎭㈠銆?
int (\**suspend) (struct drm_device \**, pm_message_t state); int
(\**resume) (struct drm_device \**);
杩欎簺鏄棫鐗堟寕璧蜂笌鎭㈠鏂规硶锛?*浠?*涓庢棫鐗?shadow-attach 椹卞姩娉ㄥ唽鍑芥暟閰嶅悎浣跨敤銆傛柊椹卞姩搴斿綋浣跨敤鍏舵€荤嚎绫诲瀷鎵€鎻愪緵鐨勭數婧愮鐞嗘帴鍙ｏ紙閫氬父閫氳繃 `struct device_driver <device_driver>` 鐨?dev_pm_ops锛夛紝骞跺皢杩欎簺鏂规硶璁句负 NULL銆?
### 鏃х増 DMA 鏈嶅姟


杩欓噷搴斿綋浠嬬粛鏍稿績濡備綍鏀寔 DMA 鏄犲皠绛夈€傝繖浜涘嚱鏁板凡琚純鐢紝涓嶅簲浣跨敤銆?