## GPU 璋冭瘯


## 閫氱敤璋冭瘯閫夐」


DebugFS 涓€鑺傛彁渚涗簡鑻ュ共鏂囦欢鐨勬枃妗ｏ紝鐢ㄤ簬杈呭姪璋冭瘯 GPU 涓婄殑闂銆?

## GPUVM 璋冭瘯


涓轰簡杈呭姪璋冭瘯 GPU 铏氭嫙鍐呭瓨鐩稿叧鐨勯棶棰橈紝椹卞姩鏀寔浠ヤ笅鑻ュ共妯″潡鍙傛暟閫夐」锛?
`vm_fault_stop` - 鑻ラ潪 0锛屽垯鍦ㄥ彂鐢?GPU 椤甸敊璇椂鍋滄 GPU 鍐呭瓨鎺у埗鍣ㄣ€?
`vm_update_mode` - 鑻ラ潪 0锛屽垯浣跨敤 CPU 鑰岄潪 GPU 鏉ユ洿鏂?GPU 椤佃〃銆?

## 瑙ｇ爜涓€娆?GPUVM 椤甸敊璇?

濡傛灉浣犲湪鍐呮牳鏃ュ織涓湅鍒颁竴娆?GPU 椤甸敊璇紝鍙互瀵瑰叾杩涜瑙ｇ爜锛屼互寮勬竻浣犵殑搴旂敤绋嬪簭涓彂鐢熶簡浠€涔堥棶棰樸€傚唴鏍告棩蹇椾腑鐨勪竴娆￠〉閿欒鍙兘绫讳技濡備笅鍐呭锛?
```

 [gfxhub0] no-retry page fault (src_id:0 ring:24 vmid:3 pasid:32777, for process glxinfo pid 2424 thread glxinfo:cs0 pid 2425)
   in page starting at address 0x0000800102800000 from IH client 0x1b (UTCL2)
 VM_L2_PROTECTION_FAULT_STATUS:0x00301030
 	Faulty UTCL2 client ID: TCP (0x8)
 	MORE_FAULTS: 0x0
 	WALKER_ERROR: 0x0
 	PERMISSION_FAULTS: 0x3
 	MAPPING_ERROR: 0x0
 	RW: 0x0

```
棣栧厛鏄唴瀛樻灑绾斤紙memory hub锛夛紝鍗?gfxhub 鍜?mmhub銆俫fxhub 鏄敤浜庡浘褰€佽绠楀拰鏌愪簺鑺墖涓?sdma 鐨勫唴瀛樻灑绾姐€俶mhub 鏄敤浜庡濯掍綋鍜屾煇浜涜姱鐗囦笂 sdma 鐨勫唴瀛樻灑绾姐€?
鎺ヤ笅鏉ユ槸 vmid 鍜?pasid銆傚鏋?vmid 涓?0锛屽垯璇ラ敊璇緢鍙兘鐢卞唴鏍搁┍鍔ㄦ垨鍥轰欢寮曡捣銆傚鏋?vmid 闈?0锛屽垯閫氬父鏄敤鎴峰簲鐢ㄧ▼搴忎腑鐨勯敊璇€俻asid 鐢ㄤ簬灏?vmid 鍏宠仈鍒扮郴缁熺殑杩涚▼ id銆傚鏋滈敊璇彂鐢熸椂璇ヨ繘绋嬪浜庢椿鍔ㄧ姸鎬侊紝鍒欎細鎵撳嵃杩涚▼淇℃伅銆?
寮曡捣璇ラ敊璇殑 GPU 铏氭嫙鍦板潃绱ч殢鍏跺悗銆?
瀹㈡埛绔?ID 鎸囨槑浜嗗紩鍙戦敊璇殑 GPU 妯″潡銆備竴浜涘父瑙佺殑瀹㈡埛绔?ID锛?
- CB/DB锛氬浘褰㈢绾夸腑鐨勯鑹?娣卞害鍚庣
- CPF锛氬懡浠ゅ鐞嗗櫒鍓嶇锛圕ommand Processor Frontend锛?- CPC锛氬懡浠ゅ鐞嗗櫒璁＄畻锛圕ommand Processor Compute锛?- CPG锛氬懡浠ゅ鐞嗗櫒鍥惧舰锛圕ommand Processor Graphics锛?- TCP/SQC/SQG锛氱潃鑹插櫒锛圫haders锛?- SDMA锛歋DMA 寮曟搸
- VCN锛氳棰戠紪瑙ｇ爜寮曟搸
- JPEG锛欽PEG 寮曟搸

PERMISSION_FAULTS 鎻忚堪浜嗛亣鍒颁簡鍝簺閿欒锛?
- bit 0锛歅TE 鏃犳晥
- bit 1锛歅TE 璇讳綅鏈缃?- bit 2锛歅TE 鍐欎綅鏈缃?- bit 3锛歅TE 鎵ц浣嶆湭璁剧疆

鏈€鍚庯紝RW 鎸囩ず璇ヨ闂槸璇伙紙0锛夎繕鏄啓锛?锛夈€?
鍦ㄤ笂闈㈢殑绀轰緥涓紝涓€涓潃鑹插櫒锛堝鎴风 id = TCP锛夊 GPU 铏氭嫙鍦板潃 0x0000800102800000 澶勭殑鏃犳晥椤碉紙PERMISSION_FAULTS = 0x3锛夊彂璧蜂簡涓€娆¤璁块棶锛圧W = 0x0锛夈€傞殢鍚庣敤鎴峰彲浠ユ鏌ュ叾鐫€鑹插櫒浠ｇ爜鍜岃祫婧愭弿杩扮鐘舵€侊紝浠ョ‘瀹氭槸浠€涔堝鑷翠簡璇?GPU 椤甸敊璇€?
## UMR


`umr <https://gitlab.freedesktop.org/tomstdenis/umr>`_ 鏄竴涓€氱敤鐨?GPU 璋冭瘯涓庤瘖鏂伐鍏枫€傛湁鍏冲叾鑳藉姏鐨勬洿澶氫俊鎭紝璇峰弬瑙?umr 鐨?`鏂囨。 <https://umr.readthedocs.io/en/main/>`_銆?
## 璋冭瘯鑳屽厜浜害

榛樿鑳屽厜浜害搴旂粡鐢卞浐浠舵墍閫氬憡鐨勭瓥鐣ユ潵璁剧疆銆傚浐浠堕€氬父浼氫负浜ゆ祦锛圓C锛夋垨鐩存祦锛圖C锛変緵鐢垫彁渚涗笉鍚岀殑榛樿鍊笺€傛澶栵紝鏌愪簺鐢ㄦ埛绌洪棿杞欢浼氬湪涓婁竴娆″惎鍔ㄦ椂淇濆瓨鑳屽厜浜害锛屽苟灏濊瘯鎭㈠瀹冦€?
鏌愪簺鍥轰欢杩樻敮鎸佷竴椤圭О涓衡€淐ustom Backlight Curves锛堣嚜瀹氫箟鑳屽厜鏇茬嚎锛夆€濈殑鍔熻兘锛屽湪璇ュ姛鑳戒腑灏嗕寒搴﹁緭鍏ュ€兼部涓€鏉′笌鏄剧ず鐗规€ф洿鍖归厤鐨勪寒搴﹀€肩嚎鎬ф彃鍊兼洸绾胯繘琛屾槧灏勩€?
鍦ㄨ儗鍏夊嚭鐜伴棶棰樻椂锛屾湁涓€涓彲鍦ㄥ惎鍔ㄦ椂鍚敤鐨?trace 浜嬩欢锛岀敤浜庤褰曟瘡涓€娆′寒搴﹀彉鏇磋姹傘€傝繖鏈夊姪浜庡畾浣嶉棶棰樻墍鍦ㄣ€傝鍚敤璇?trace 浜嬩欢锛岃鍦ㄥ懡浠よ涓坊鍔犲涓嬪唴瀹癸細

  tp_printk trace_event=amdgpu_dm:amdgpu_dm_brightness:mod:amdgpu trace_buf_size=1M
