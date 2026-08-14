
## 姒傝堪


鏈妭浠嬬粛 CXL Type-3 鍐呭瓨璁惧鐨勯厤缃繃绋嬶紝浠ュ強瀹冩渶缁堝浣曚綔涓?`DAX` 璁惧鎴栫粡鐢卞唴鏍?椤靛垎閰嶅櫒鐨勬櫘閫氬唴瀛橀〉鏆撮湶缁欑敤鎴枫€?
浠ラ」鐩鍙锋爣璁扮殑閮ㄥ垎鏄敓鎴愮壒瀹氬唴鏍稿璞＄殑鏃堕棿鐐广€?
1) 鏃╂湡鍚姩

  a) BIOS銆佹瀯寤轰笌鍚姩鍙傛暟

    i) EFI_MEMORY_SP
    ii) CONFIG_EFI_SOFT_RESERVE
    iii) CONFIG_MHP_DEFAULT_ONLINE_TYPE
    iv) nosoftreserve

  b) 鍐呭瓨鏄犲皠鍒涘缓

    i) 閽堝 Soft-Reserved 鏌ラ槄 EFI 鍐呭瓨鏄犲皠 / E820

      - CXL 鍐呭瓨琚鐣欏嚭鏉ョ敱 CXL 椹卞姩澶勭悊

      - 涓?CFMWS 鏉＄洰鍒涘缓 Soft-Reserved IO 璧勬簮

  c) NUMA 鑺傜偣鍒涘缓

    - 浠?ACPI CEDT CFMWS 涓?SRAT 閭昏繎鍩燂紙PXM锛夊垱寤鸿妭鐐?
  d) 鍐呭瓨鍒嗗眰锛圡emory Tier锛夊垱寤?
    - 浣跨敤鎵€鏈夎妭鐐瑰垱寤轰竴涓粯璁ょ殑 memory_tier

  e) 杩炵画鍐呭瓨鍒嗛厤

    - 浠讳綍璇锋眰鐨?CMA 閮戒粠鍦ㄧ嚎鑺傜偣鍒嗛厤

  f) 鍒濆鍖栫粨鏉燂紝椹卞姩寮€濮嬫帰娴?
2) ACPI 涓?PCI 椹卞姩

  a) 妫€娴嬪埌 PCI 璁惧鏄?CXL锛屽皢鍏舵爣璁颁负浜ょ敱 CXL 椹卞姩鎺㈡祴

3) CXL 椹卞姩鎿嶄綔

  a) 鍩虹璁惧鍒涘缓

    - 鍒涘缓 root銆乸ort 涓?memdev 璁惧
    - 鍒涘缓 CEDT CFMWS IO 璧勬簮

  b) 瑙ｇ爜鍣紙Decoder锛夊垱寤?
    - 鍒涘缓 root銆乻witch 涓?endpoint 瑙ｇ爜鍣?
  c) 閫昏緫璁惧鍒涘缓

    - 鍒涘缓 memory_region 涓?endpoint 璁惧

  d) 璁惧鐩镐簰鍏宠仈

    - 濡傛灉鏄?auto-decoder锛圔IOS 缂栫▼鐨勮В鐮佸櫒锛夛紝椹卞姩鍦ㄦ帰娴嬫椂楠岃瘉閰嶇疆銆佸缓绔嬪叧鑱斿苟
      閿佸畾閰嶇疆銆?
    - 濡傛灉鏄敤鎴烽厤缃殑锛岄獙璇佷笌鍏宠仈鍦?decoder-commit 鏃跺缓绔嬨€?
  e) 鍖哄煙浣滀负 DAX 鍖哄煙鍛堢幇

    - 鍒涘缓 dax_region

    - 閫氳繃 DAX 椹卞姩鍒涘缓 DAX 璁惧

4) DAX 椹卞姩鎿嶄綔

  a) DAX 椹卞姩灏?DAX 鍖哄煙浠ヤ袱绉?dax 璁惧妯″紡涔嬩竴鍛堢幇

    - kmem - dax 璁惧琚浆鎹负鐑彃鎷斿唴瀛樺潡

      - 鍒涘缓 DAX kmem IO 璧勬簮

    - hmem - dax 璁惧淇濈暀涓?daxdev锛屼綔涓烘枃浠惰闂€?
      - 濡傛灉鏄?hmem锛屾祦绋嬪湪姝ょ粨鏉熴€?
  b) DAX kmem 灏嗗唴瀛樺尯鍩熷憟鐜扮粰 Memory Hotplug锛屼互浣滀负鈥滈┍鍔ㄧ鐞嗗唴瀛樷€濆姞鍏ラ〉鍒嗛厤鍣?
5) 鍐呭瓨鐑彃鎷旓紙Memory Hotplug锛?
  a) mhp 缁勪欢灏嗕竴涓?dax 璁惧鍐呭瓨鍖哄煙浣滀负澶氫釜鍐呭瓨鍧楀憟鐜扮粰椤靛垎閰嶅櫒

    - 杩欎簺鍧楀嚭鐜板湪 `/sys/bus/memory/devices` 涓紝骞堕摼鎺ュ埌涓€涓?NUMA 鑺傜偣

  b) 杩欎簺鍧楄涓婄嚎鍒版墍璇锋眰鐨勫尯锛圢ORMAL 鎴?MOVABLE锛?
    - 鍐呭瓨琚爣璁颁负鈥淒river Managed鈥濓紝浠ラ伩鍏?kexec 灏嗗叾鐢ㄤ綔鍐呮牳鏇存柊鐨勫尯鍩?