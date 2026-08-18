
## Boot Interrupts


:Author: - Sean V Kelley <sean.v.kelley@linux.intel.com>

## Overview


鍦?PCI Express 涓婏紝涓柇鐢?MSI 鎴栧叆绔欎腑鏂秷鎭紙Assert_INTx/Deassert_INTx锛夎〃绀恒€傜粰瀹?Core IO 涓殑闆嗘垚 IO-APIC 灏嗘潵鑷?PCI Express 鐨勪紶缁熶腑鏂秷鎭浆鎹负 MSI 涓柇銆傚鏋?IO-APIC 琚鐢紙閫氳繃 IO-APIC 琛ㄩ」涓殑鎺╃爜浣嶏級锛岃繖浜涙秷鎭細琚矾鐢卞埌浼犵粺鐨?PCH銆傝繖绉嶅甫鍐咃紙in-band锛変腑鏂満鍒朵紶缁熶笂瀵逛簬涓嶆敮鎸?IO-APIC 鐨勭郴缁熶互鍙婂惎鍔紙boot锛夋槸蹇呰鐨勩€侷ntel 杩囧幓浣跨敤鏈鈥渂oot interrupts鈥濇潵鎻忚堪杩欑鏈哄埗銆傛澶栵紝PCI Express 鍗忚鎻忚堪浜嗚繖绉嶅甫鍐呬紶缁熺嚎缂嗕腑鏂?INTx 鏈哄埗锛屼緵 I/O 璁惧鍙戝嚭 PCI 椋庢牸鐨勭數骞充腑鏂€傚悗缁钀芥弿杩颁簡 Core IO 鍦ㄥ鐞?INTx 娑堟伅璺敱鍒?PCH 鏃剁殑闂锛屼互鍙?BIOS 鍜屾搷浣滅郴缁熷唴鐨勭紦瑙ｆ帾鏂姐€?

## Issue


褰撳甫鍐呬紶缁?INTx 娑堟伅琚浆鍙戝埌 PCH 鏃讹紝瀹冧滑鍙嶈繃鏉ヤ細瑙﹀彂涓€涓柊鐨勪腑鏂紝鑰屾搷浣滅郴缁熷緢鍙兘缂哄皯瀵瑰簲鐨勫鐞嗙▼搴忋€傚綋涓柇闀挎椂闂存湭琚鐞嗘椂锛孡inux 鍐呮牳浼氬皢鍏朵綔涓轰吉涓柇锛圫purious Interrupt锛夎窡韪€傚綋璇?IRQ 杈惧埌鐗瑰畾璁℃暟鏃讹紝Linux 鍐呮牳浼氫互 "nobody cared" 閿欒绂佺敤瀹冦€傝繖涓绂佺敤鐨?IRQ 鐜板湪闃绘浜嗘伆濂藉叡浜
```

  irq 19: nobody cared (try booting with the "irqpoll" option)
  CPU: 0 PID: 2988 Comm: irq/34-nipalk Tainted: 4.14.87-rt49-02410-g4a640ec-dirty #1
  Hardware name: National Instruments NI PXIe-8880/NI PXIe-8880, BIOS 2.1.5f1 01/09/2020
  Call Trace:

  <IRQ>
   ? dump_stack+0x46/0x5e
   ? __report_bad_irq+0x2e/0xb0
   ? note_interrupt+0x242/0x290
   ? nNIKAL100_memoryRead16+0x8/0x10 [nikal]
   ? handle_irq_event_percpu+0x55/0x70
   ? handle_irq_event+0x4f/0x80
   ? handle_fasteoi_irq+0x81/0x180
   ? handle_irq+0x1c/0x30
   ? do_IRQ+0x41/0xd0
   ? common_interrupt+0x84/0x84
  </IRQ>

  handlers:
  irq_default_primary_handler threaded usb_hcd_irq
  Disabling IRQ #19


```
## Conditions


浣跨敤绾跨▼鍖栦腑鏂紙threaded interrupts锛夋槸褰撲粖鏈€鏈夊彲鑳借Е鍙戞闂鐨勬潯浠躲€傜嚎绋嬪寲涓柇鍦?IRQ 澶勭悊绋嬪簭鍞ら啋鍚庡彲鑳戒笉浼氳閲嶆柊鍚敤銆傝繖浜涒€滀竴娆℃€р€濓紙one shot锛夋潯浠舵剰鍛崇潃绾跨▼鍖栦腑鏂渶瑕佸湪绾跨▼澶勭悊绋嬪簭杩愯涔嬪墠涓€鐩翠繚鎸佷腑鏂嚎琚睆钄姐€傜壒鍒槸鍦ㄥ鐞嗛珮鏁版嵁閫熺巼涓柇鏃讹紝绾跨▼闇€瑕佽繍琛屽埌瀹屾垚锛涘惁鍒欎竴浜涘鐞嗙▼搴忔渶缁堜細瀵艰嚧鏍堟孩鍑猴紝鍥犱负鍙戝嚭璁惧鐨勪腑鏂粛澶勪簬娲诲姩鐘舵€併€?
## Affected Chipsets


浼犵粺鐨勭粓绔腑鏂浆鍙戞満鍒跺浠婂瓨鍦ㄤ簬璁稿璁惧涓紝鍖呮嫭浣嗕笉闄愪簬鏉ヨ嚜 AMD/ATI銆丅roadcom 鍜?Intel 鐨勮姱鐗囩粍銆傞€氳繃涓嬮潰缂撹В鎺柦鎵€鍋氱殑鏇存敼宸插簲鐢ㄥ埌 drivers/pci/quirks.c銆?
浠?ICX 寮€濮嬶紝Core IO 鐨勮澶囦腑涓嶅啀鏈変换浣?IO-APIC銆侷O-APIC 浠呭湪 PCH 涓€傝繛鎺ュ埌 Core IO 鐨?PCIe Root Port 鐨勮澶囧皢浣跨敤鍘熺敓鐨?MSI/MSI-X 鏈哄埗銆?
## Mitigations


缂撹В鎺柦閲囧彇 PCI quirks 鐨勫舰寮忋€備紭鍏堝仛娉曟槸棣栧厛璇嗗埆骞跺埄鐢ㄤ竴绉嶇鐢ㄥ埌 PCH 璺敱鐨勬柟娉曘€傚湪杩欑鎯呭喌涓嬶紝鍙互娣诲姞涓€涓鐢?boot 涓柇鐢熸垚鐨?quirk銆俒^1^]_

Intel庐 6300ESB I/O Controller Hub
  Alternate Base Address Register锛?   BIE: Boot Interrupt Enable

	  ==  ===========================
	  0   Boot interrupt is enabled.
	  1   Boot interrupt is disabled.
	  ==  ===========================

Intel庐 Sandy Bridge 鍒?Sky Lake 鐨?Xeon 鏈嶅姟鍣細
  Coherent Interface Protocol Interrupt Control
   dis_intx_route2pch/dis_intx_route2ich/dis_intx_route2dmi2锛?	  褰撹浣嶈璁剧疆鏃躲€備粠 Intel庐 Quick Data DMA/PCI Express 绔彛鏀跺埌鐨勬湰鍦?INTx 娑堟伅涓嶄細琚矾鐢卞埌浼犵粺
	  PCH鈥斺€斿畠浠涔堥€氳繃闆嗘垚鐨?IO-APIC 杞崲涓?MSI锛堝鏋滅浉搴旇〃椤逛腑鐨?IO-APIC 鎺╃爜浣嶄负娓呴櫎锛夛紝
	  瑕佷箞涓嶅紩鍙戣繘涓€姝ュ姩浣滐紙褰撴帺鐮佷綅琚缃椂锛?
鍦ㄦ棤娉曠洿鎺ョ鐢ㄨ矾鐢辩殑鎯呭喌涓嬶紝鍙︿竴绉嶆柟娉曟槸鍒╃敤 PCI 涓柇寮曡剼鍒?INTx 鐨勮矾鐢辫〃锛屼互渚块粯璁ゅ皢涓柇澶勭悊绋嬪簭閲嶅畾鍚戝埌閲嶆柊璺敱鐨勪腑鏂嚎銆傚洜姝わ紝鍦ㄦ棤娉曠鐢ㄦ INTx 璺敱鐨勮姱鐗囩粍涓婏紝Linux 鍐呮牳浼氭妸鏈夋晥鐨勪腑鏂噸鏂拌矾鐢卞埌鍏朵紶缁熶腑鏂€傝繖绉嶅鐞嗙▼搴忕殑閲嶅畾鍚戝皢闃叉鍑虹幇浼腑鏂娴嬶紝鍚﹀垯璇ユ娴嬩細鍥犺繃澶氱殑鏈鐞嗚鏁拌€岀鐢?IRQ 绾裤€俒^2^]_

閰嶇疆閫夐」 X86_REROUTE_FOR_BROKEN_BOOT_IRQS 鐢ㄤ簬鍚敤锛堟垨绂佺敤锛夊皢涓柇澶勭悊绋嬪簭閲嶅畾鍚戝埌 PCH 涓柇绾裤€傝閫夐」鍙互琚?pci=ioapicreroute 鎴?pci=noioapicreroute 瑕嗙洊銆俒^3^]_


## More Documentation


鍦ㄤ竴浜涙暟鎹墜鍐岋紙涓嬮潰鐨?6300ESB 鍜?6700PXH锛変腑鏈夊叧浜庝紶缁熶腑鏂鐞嗙殑姒傝堪銆傝櫧鐒跺ぇ浣撶浉鍚岋紝浣嗗畠鎻ず浜嗗叾澶勭悊闅忚姱鐗囩粍鐨勬紨杩涖€?
### Example of disabling of the boot interrupt


      - Intel庐 6300ESB I/O Controller Hub (Document # 300641-004US)
	5.7.3 Boot Interrupt
	https://www.intel.com/content/dam/doc/datasheet/6300esb-io-controller-hub-datasheet.pdf

      - Intel庐 Xeon庐 Processor E5-1600/2400/2600/4600 v3 Product Families
	Datasheet - Volume 2: Registers (Document # 330784-003)
	6.6.41 cipintrc Coherent Interface Protocol Interrupt Control
	https://www.intel.com/content/dam/www/public/us/en/documents/datasheets/xeon-e5-v3-datasheet-vol-2.pdf

### Example of handler rerouting


      - Intel庐 6700PXH 64-bit PCI Hub (Document # 302628)
	2.15.2 PCI Express Legacy INTx Support and Boot Interrupt
	https://www.intel.com/content/dam/doc/datasheet/6700pxh-64-bit-pci-hub-datasheet.pdf


濡傛灉浣犳湁浠讳綍鏈В绛旂殑浼犵粺 PCI 涓柇闂锛岃鍙戦偖浠剁粰鎴戙€?
Cheers,
    Sean V Kelley
    sean.v.kelley@linux.intel.com
