## Ramoops oops/panic 璁板綍鍣?

Sergiu Iordache <sergiu@chromium.org>

Updated: 10 Feb 2021

### 绠€浠?

Ramoops 鏄竴涓?oops/panic 璁板綍鍣紝鍦ㄧ郴缁熷穿婧冨墠灏嗗叾鏃ュ織鍐欏叆 RAM銆傚畠閫氳繃鍦ㄧ幆褰?缂撳啿鍖轰腑璁板綍 oops 鍜?panic 鏉ュ伐浣溿€俁amoops 闇€瑕佺郴缁熷叿澶囨寔涔?RAM锛屼互渚胯鍖哄煙鐨?鍐呭鍦ㄩ噸鍚悗鑳藉淇濈暀銆?
### Ramoops 姒傚康


Ramoops 浣跨敤棰勫畾涔夌殑鍐呭瓨鍖哄煙鏉ュ瓨鍌ㄨ浆鍌ㄣ€傝鍐呭瓨鍖哄煙鐨勮捣濮嬨€佸ぇ灏忓拰绫诲瀷閫氳繃浠ヤ笅
涓変釜鍙橀噺璁剧疆锛?
  - `mem_address` 琛ㄧず璧峰鍦板潃
  - `mem_size` 琛ㄧず澶у皬銆傚唴瀛樺ぇ灏忎細鍚戜笅鍙栨暣涓?2 鐨勫箓銆?  - `mem_type` 鐢ㄤ簬鎸囧畾鍐呭瓨绫诲瀷锛堥粯璁ゆ槸 pgprot_writecombine锛夈€?  - `mem_name` 鐢ㄤ簬鎸囧畾鐢?`reserve_mem` 鍛戒护琛屽弬鏁板畾涔夌殑鍐呭瓨鍖哄煙銆?
閫氬父搴斾娇鐢?`mem_type=0` 鐨勯粯璁ゅ€硷紝鍥犱负杩欎細灏?pstore 鏄犲皠璁剧疆涓?pgprot_writecombine銆傝缃?`mem_type=1` 浼氬皾璇曚娇鐢?`pgprot_noncached`锛岃繖浠呭湪
鏌愪簺骞冲彴涓婃湁鏁堛€傝繖鏄洜涓?pstore 渚濊禆浜庡師瀛愭搷浣溿€傝嚦灏戝湪 ARM 涓婏紝pgprot_noncached
浼氫娇鍐呭瓨琚槧灏勪负寮烘湁搴忥紙strongly ordered锛夛紝鑰屽寮烘湁搴忓唴瀛樼殑鍘熷瓙鎿嶄綔鏄緷瀹炵幇
鑰屽畾鐨勶紝骞朵笖鍦ㄨ澶?ARM锛堝 omaps锛変笂鏃犳硶宸ヤ綔銆傝缃?`mem_type=2` 浼氬皾璇曞皢璇ュ唴瀛?鍖哄煙褰撲綔鏅€氬唴瀛樺鐞嗭紝浠庤€屽惎鐢ㄥ叾涓婄殑瀹屾暣缂撳瓨銆傝繖鍙互鎻愬崌鎬ц兘銆?
璇ュ唴瀛樺尯鍩熻鍒掑垎涓?`record_size` 澶у皬鐨勫潡锛堝悓鏍峰悜涓嬪彇鏁翠负 2 鐨勫箓锛夛紝姣忔 kmesg
杞偍浼氬啓鍏ヤ竴涓?`record_size` 澶у皬鐨勪俊鎭潡銆?
鍙互閫氳繃 `max_reason` 鍊兼潵闄愬埗瀛樺偍鍝簺绫诲瀷鐨?kmsg 杞偍锛岃鍊煎畾涔変簬
include/linux/kmsg_dump.h 鐨?`enum kmsg_dump_reason`銆備緥濡傦紝瑕佸悓鏃跺瓨鍌?Oops 鍜?Panic锛宍max_reason` 搴旇涓?2锛圞MSG_DUMP_OOPS锛夛紱瑕佸彧瀛樺偍 Panic锛宍max_reason`
搴旇涓?1锛圞MSG_DUMP_PANIC锛夈€傚皢鍏惰涓?0锛圞MSG_DUMP_UNDEF锛夋椂锛屽師鍥犺繃婊ゅ皢鐢?`printk.always_kmsg_dump` 鍚姩鍙傛暟鎺у埗锛氳嫢鏈缃紝鍒欎负 KMSG_DUMP_OOPS锛屽惁鍒?涓?KMSG_DUMP_MAX銆?
璇ユā鍧椾娇鐢ㄤ竴涓鏁板櫒鏉ヨ褰曞娆¤浆鍌紝浣嗚鏁板櫒浼氬湪閲嶅惎鏃堕噸缃紙鍗抽噸鍚悗鐨勬柊杞偍
浼氳鐩栨棫鐨勶級銆?
Ramoops 杩樻敮鎸佸鎸佷箙鍐呭瓨鍖哄煙鐨勮蒋浠?ECC 淇濇姢銆傚綋浣跨敤纭欢澶嶄綅浣挎満鍣ㄦ仮澶嶏紙渚嬪
鐪嬮棬鐙楄Е鍙戯級鏃讹紝杩欏彲鑳藉緢鏈夌敤銆傚湪杩欑鎯呭喌涓嬶紝RAM 鍙兘鐣ユ湁鎹熷潖锛屼絾閫氬父鍙互鎭㈠銆?
### 璁剧疆鍙傛暟


璁剧疆 ramoops 鍙傛暟鏈夊嚑绉嶄笉鍚岀殑鏂瑰紡锛?
```
 A. 浣跨敤妯″潡鍙傛暟锛堝叾鍚嶇О鍗冲墠杩板彉閲忓悕锛夈€備负浜嗗揩閫熻皟璇曪紝鎮ㄤ篃鍙互鍦ㄥ惎鍔ㄦ湡闂翠繚鐣? 閮ㄥ垎鍐呭瓨锛岀劧鍚庡皢淇濈暀鐨勫唴瀛樼敤浜?ramoops銆備緥濡傦紝鍋囪涓€鍙板唴瀛樺ぇ浜?128 MB 鐨勬満鍣紝
 浠ヤ笅鍐呮牳鍛戒护琛屽皢鍛婅瘔鍐呮牳鍙娇鐢ㄥ墠 128 MB 鍐呭瓨锛屽苟灏?ECC 淇濇姢鐨?
	mem=128M ramoops.mem_address=0x8000000 ramoops.ecc=1

 B. 浣跨敤璁惧鏍戠粦瀹氾紝濡?``Documentation/devicetree/bindings/reserved-memory/ramoops.yaml``
 鎵€杩般€備緥濡傦細

	reserved-memory {
		#address-cells = <2>;
		#size-cells = <2>;
		ranges;

		ramoops@8f000000 {
			compatible = "ramoops";
			reg = <0 0x8f000000 0 0x100000>;
			record-size = <0x4000>;
			console-size = <0x4000>;
		};
	};

 C. 浣跨敤骞冲彴璁惧骞惰缃钩鍙版暟鎹€傜劧鍚庡彲浠ラ€氳繃璇ュ钩鍙版暟鎹缃弬鏁般€傜ず渚嬪涓嬶細

 .. code-block:: c

  #include <linux/pstore_ram.h>
  [...]

  static struct ramoops_platform_data ramoops_data = {
        .mem_size               = <...>,
        .mem_address            = <...>,
        .mem_type               = <...>,
        .record_size            = <...>,
        .max_reason             = <...>,
        .ecc                    = <...>,
  };

  static struct platform_device ramoops_dev = {
        .name = "ramoops",
        .dev = {
                .platform_data = &ramoops_data,
        },
  };

  [... inside a function ...]
  int ret;

  ret = platform_device_register(&ramoops_dev);
  if (ret) {
	printk(KERN_ERR "unable to register platform device\n");
	return ret;
  }

 D. 浣跨敤閫氳繃 ``reserve_mem`` 鍛戒护琛屽弬鏁颁繚鐣欑殑鍐呭瓨鍖哄煙銆傚湴鍧€鍜屽ぇ灏忕敱 ``reserve_mem``
 鍙傛暟瀹氫箟銆傝娉ㄦ剰锛宍`reserve_mem`` 涓嶄竴瀹氭€绘槸鍦ㄥ悓涓€浣嶇疆鍒嗛厤鍐呭瓨锛屽洜姝や笉鍙緷璧栥€? 闇€瑕佽繘琛屾祴璇曪紝骞朵笖瀹冨彲鑳藉苟闈炲湪姣忓彴鏈哄櫒鎴栨瘡涓唴鏍镐笂閮芥湁鏁堛€傝灏嗘瑙嗕负"灏藉姏鑰屼负"
 鐨勬柟寮忋€俙`reserve_mem`` 閫夐」鎺ュ彈澶у皬銆佸榻愬拰鍚嶇О浣滀负鍙傛暟銆傝鍚嶇О鐢ㄤ簬灏嗗唴瀛樻槧灏? 鍒颁竴涓爣绛撅紝ramoops 鍙嵁姝ゆ绱€?
	reserve_mem=2M:4096:oops  ramoops.mem_name=oops
```
鎮ㄥ彲浠ユ寚瀹?RAM 鎴栧璁剧殑鍐呭瓨銆備絾鏄紝褰撴寚瀹?RAM 鏃讹紝璇峰姟蹇呴€氳繃鍙戝嚭 memblock_reserve()

```
	#include <linux/memblock.h>

	memblock_reserve(ramoops_data.mem_address, ramoops_data.mem_size);

```
### 杞偍鏍煎紡


鏁版嵁杞偍浠ヤ竴涓ご閮ㄥ紑濮嬶紝褰撳墠瀹氫箟涓?`====`锛屽悗璺熸椂闂存埑鍜屾崲琛岀銆傞殢鍚庢槸瀹為檯鏁版嵁銆?
### 璇诲彇鏁版嵁


杞偍鏁版嵁鍙互浠?pstore 鏂囦欢绯荤粺璇诲彇銆傝繖浜涙枃浠剁殑鏍煎紡涓?`dmesg-ramoops-N`锛屽叾涓?N 鏄唴瀛樹腑鐨勮褰曞彿銆傝浠?RAM 涓垹闄ゅ凡瀛樺偍鐨勮褰曪紝鍙渶鍙栨秷閾炬帴鐩稿簲鐨?pstore 鏂囦欢銆?
### 鎸佷箙鍑芥暟璺熻釜


鎸佷箙鍑芥暟璺熻釜鍙兘鏈夊姪浜庤皟璇曚笌杞欢鎴栫‖浠剁浉鍏崇殑鎸傝捣銆傚嚱鏁拌皟鐢ㄩ摼鏃ュ織瀛樺偍鍦?`ftrace-ramoops`

```
 # mount -t debugfs debugfs /sys/kernel/debug/
 # echo 1 > /sys/kernel/debug/pstore/record_ftrace
 # reboot -f
 [...]
 # mount -t pstore pstore /mnt/
 # tail /mnt/ftrace-ramoops
 0 ffffffff8101ea64  ffffffff8101bcda  native_apic_mem_read <- disconnect_bsp_APIC+0x6a/0xc0
 0 ffffffff8101ea44  ffffffff8101bcf6  native_apic_mem_write <- disconnect_bsp_APIC+0x86/0xc0
 0 ffffffff81020084  ffffffff8101a4b5  hpet_disable <- native_machine_shutdown+0x75/0x90
 0 ffffffff81005f94  ffffffff8101a4bb  iommu_shutdown_noop <- native_machine_shutdown+0x7b/0x90
 0 ffffffff8101a6a1  ffffffff8101a437  native_machine_emergency_restart <- native_machine_restart+0x37/0x40
 0 ffffffff811f9876  ffffffff8101a73a  acpi_reboot <- native_machine_emergency_restart+0xaa/0x1e0
 0 ffffffff8101a514  ffffffff8101a772  mach_reboot_fixups <- native_machine_emergency_restart+0xe2/0x1e0
 0 ffffffff811d9c54  ffffffff8101a7a0  __const_udelay <- native_machine_emergency_restart+0x110/0x1e0
 0 ffffffff811d9c34  ffffffff811d9c80  __delay <- __const_udelay+0x30/0x40
 0 ffffffff811d9d14  ffffffff811d9c3f  delay_tsc <- __delay+0xf/0x20

```
