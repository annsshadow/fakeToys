## x86 IOMMU 鏀寔


鏋舵瀯瑙勮寖鍙互浠庡巶鍟嗙綉绔欒幏鍙栥€傛悳绱互涓嬫枃妗ｄ互鑾峰彇鏈€鏂扮増鏈細

- Intel锛欼ntel Virtualization Technology for Directed I/O Architecture Specification锛圛D: D51397锛?- AMD锛欰MD I/O Virtualization Technology (IOMMU) Specification锛圛D: 48882锛?
鏈寚鍗椾负涓€浜涘熀鏈悊瑙ｆ彁渚涘揩閫熷蹇樺崟銆?
### 鍩虹鍐呭


ACPI 鏋氫妇骞跺垪鍑哄钩鍙颁笂涓嶅悓鐨?IOMMU锛屼互鍙婅澶囦笌鍝釜 IOMMU 鎺у埗瀹冧滑涔嬮棿鐨?璁惧浣滅敤鍩燂紙device scope锛夊叧绯汇€?
涓€浜?ACPI 鍏抽敭瀛楋細

- DMAR - Intel DMA 閲嶆槧灏勮〃锛圖MA Remapping table锛?- DRHD - Intel DMA 閲嶆槧灏勭‖浠跺崟鍏冨畾涔夛紙DMA Remapping Hardware Unit Definition锛?- RMRR - Intel 淇濈暀鍐呭瓨鍖哄煙鎶ュ憡缁撴瀯锛圧eserved Memory Region Reporting Structure锛?- IVRS - AMD I/O 铏氭嫙鍖栨姤鍛婄粨鏋勶紙I/O Virtualization Reporting Structure锛?- IVDB - AMD I/O 铏氭嫙鍖栧畾涔夊潡锛圛/O Virtualization Definition Block锛?- IVHD - AMD I/O 铏氭嫙鍖栫‖浠跺畾涔夛紙I/O Virtualization Hardware Definition锛?
##### 浠€涔堟槸 Intel RMRR锛?

鏈変竴浜涜澶囩敱 BIOS 鎺у埗锛屼緥濡?USB 璁惧鐢ㄤ簬鎵ц PS2 浠跨湡銆傜敤浜庤繖浜涜澶囩殑鍐呭瓨
鍖哄煙鍦?e820 鏄犲皠涓鏍囪涓轰繚鐣欍€傚綋鎴戜滑寮€鍚?DMA 杞崲鏃讹紝瀵硅繖浜涘尯鍩熺殑 DMA 灏?澶辫触銆傚洜姝?BIOS 浣跨敤 RMRR 鏉ユ寚瀹氳繖浜涘尯鍩熶互鍙婇渶瑕佽闂繖浜涘尯鍩熺殑璁惧銆侽S 搴斿綋
涓鸿繖浜涘尯鍩熻缃粺涓€鏄犲皠锛坲nity mapping锛夛紝浠ヤ究杩欎簺璁惧璁块棶杩欎簺鍖哄煙銆?
##### 浠€涔堟槸 AMD IVRS锛?

璇ユ灦鏋勫畾涔変簡涓€涓О涓?I/O 铏氭嫙鍖栨姤鍛婄粨鏋勶紙IVRS锛夌殑 ACPI 鍏煎鏁版嵁缁撴瀯锛岀敤浜庡悜
绯荤粺杞欢浼犺揪涓?I/O 铏氭嫙鍖栫浉鍏崇殑淇℃伅銆侷VRS 鎻忚堪浜嗗钩鍙颁腑鍖呭惈鐨?IOMMU 鐨勯厤缃笌
鑳藉姏锛屼互鍙婃瘡涓?IOMMU 铏氭嫙鍖栫殑璁惧鐨勪俊鎭€?
IVRS 鎻愪緵浠ヤ笅鍏充簬浠ヤ笅鏂归潰鐨勪俊鎭細

- 骞冲彴涓瓨鍦ㄧ殑 IOMMU锛屽寘鎷畠浠殑鑳藉姏涓庢纭厤缃?- 涓庢瘡涓?IOMMU 鐩稿叧鐨勭郴缁?I/O 鎷撴墤
- 鏃犳硶浠ュ叾浠栨柟寮忔灇涓剧殑澶栬
- SMI/SMM銆佸钩鍙板浐浠朵笌骞冲彴纭欢浣跨敤鐨勫唴瀛樺尯鍩熴€傝繖浜涢€氬父鏄渶瑕佺敱绯荤粺杞欢閰嶇疆鐨?  鎺掗櫎鑼冨洿銆?
### 濡備綍鐢熸垚 I/O 铏氭嫙鍦板潃锛圛OVA锛夛紵


琛屼负鑹ソ鐨勯┍鍔ㄥ湪鍙戦€侀渶瑕佹墽琛?DMA 鐨勫懡浠ゅ埌璁惧涔嬪墠璋冪敤 dma_map_*() 璋冪敤銆備竴鏃?DMA 瀹屾垚涓斾笉鍐嶉渶瑕佹槧灏勶紝椹卞姩鎵ц dma_unmap_*() 璋冪敤浠ュ彇娑堟槧灏勮鍖哄煙銆?
### Intel 鐗瑰畾璇存槑


##### 鍥惧舰闂锛?

濡傛灉浣犻亣鍒板浘褰㈣澶囩殑闂锛屽彲浠ュ皾璇曟坊鍔犻€夐」 intel_iommu=igfx_off 鏉ュ叧闂泦鎴?鍥惧舰寮曟搸銆傚鏋滆繖淇浜嗕换浣曢棶棰橈紝璇风‘淇濅綘鎻愪氦涓€涓?bug 鎶ュ憡璇ラ棶棰樸€?
##### IOVA 鐨勪竴浜涗緥澶?

涓柇鑼冨洿涓嶈鍦板潃杞崲锛?xfee00000 - 0xfeefffff锛夈€傚绛夛紙peer to peer锛変簨鍔′篃
鍚屾牱濡傛銆傚洜姝ゆ垜浠繚鐣欐潵鑷?PCI MMIO 鑼冨洿鐨勫湴鍧€锛屼娇瀹冧滑涓嶈鍒嗛厤缁?IOVA 鍦板潃銆?
### AMD 鐗瑰畾璇存槑


##### 鍥惧舰闂锛?

濡傛灉浣犻亣鍒伴泦鎴愬浘褰㈣澶囩殑闂锛屽彲浠ュ皾璇曞湪鍐呮牳鍛戒护琛屼笂娣诲姞閫夐」 iommu=pt锛屽
IOMMU 浣跨敤 1:1 鏄犲皠銆傚鏋滆繖淇浜嗕换浣曢棶棰橈紝璇风‘淇濅綘鎻愪氦涓€涓?bug 鎶ュ憡璇ラ棶棰樸€?
### 鏁呴殰鎶ュ憡


褰撴姤鍛婇敊璇椂锛孖OMMU 閫氳繃涓柇鍙戝嚭淇″彿銆傚鑷存晠闅滅殑鍘熷洜鍜岃ō鍌欎細鎵撳嵃鍦ㄦ帶鍒跺彴涓娿€?
### 鍐呮牳鏃ュ織鏍蜂緥


##### Intel 鍚姩娑堟伅


浼氭墦鍗扮被浼间互涓嬪唴瀹癸紝鎸囩ず ACPI 涓瓨鍦?DMAR 琛細

```

	ACPI: DMAR (v001 A M I  OEMDMAR  0x00000001 MSFT 0x00000097) @ 0x000000007f5b5ef0

```
褰?DMAR 琚?ACPI 澶勭悊骞跺垵濮嬪寲鏃讹紝鎵撳嵃 DMAR 浣嶇疆浠ュ強浠讳綍宸插鐞嗙殑 RMRR锛?
```

	ACPI DMAR:Host address width 36
	ACPI DMAR:DRHD (flags: 0x00000000)base: 0x00000000fed90000
	ACPI DMAR:DRHD (flags: 0x00000000)base: 0x00000000fed91000
	ACPI DMAR:DRHD (flags: 0x00000001)base: 0x00000000fed93000
	ACPI DMAR:RMRR base: 0x00000000000ed000 end: 0x00000000000effff
	ACPI DMAR:RMRR base: 0x000000007f600000 end: 0x000000007fffffff

```
褰?DMAR 琚惎鐢ㄤ娇鐢ㄦ椂锛屼綘浼氭敞鎰忓埌锛?
```

	PCI-DMA: Using DMAR IOMMU

```
##### Intel 鏁呴殰鎶ュ憡

```

	DMAR:[DMA Write] Request device [00:02.0] fault addr 6df084000
	DMAR:[fault reason 05] PTE Write access is not set
	DMAR:[DMA Write] Request device [00:02.0] fault addr 6df084000
	DMAR:[fault reason 05] PTE Write access is not set

```
##### AMD 鍚姩娑堟伅


浼氭墦鍗扮被浼间互涓嬪唴瀹癸紝鎸囩ず IOMMU 鐨勫瓨鍦細

```

	iommu: Default domain type: Translated
	iommu: DMA domain TLB invalidation policy: lazy mode

```
##### AMD 鏁呴殰鎶ュ憡

```

	AMD-Vi: Event logged [IO_PAGE_FAULT domain=0x0007 address=0xffffc02000 flags=0x0000]
	AMD-Vi: Event logged [IO_PAGE_FAULT device=07:00.0 domain=0x0007 address=0xffffc02000 flags=0x0000]

```
