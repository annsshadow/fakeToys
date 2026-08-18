## Coresight CPU 璋冭瘯妯″潡


   :Author:   Leo Yan <leo.yan@linaro.org>
   :Date:     April 5th, 2017

### 绠€浠?

Coresight CPU 璋冭瘯妯″潡瀹氫箟浜?ARMv8-a 鏋舵瀯鍙傝€冩墜鍐岋紙ARM DDI 0487A.k锛夌殑
鈥淧art H: External debug鈥?绔犺妭锛孋PU 鍙泦鎴愯皟璇曟ā鍧楋紝涓昏鐢ㄤ簬涓ょ妯″紡锛?self-hosted debug锛堣嚜鎵樼璋冭瘯锛夊拰 external debug锛堝閮ㄨ皟璇曪級銆傞€氬父 external
debug 妯″紡骞夸负浜虹煡锛屽嵆澶栭儴璋冭瘯鍣ㄩ€氳繃 JTAG 绔彛杩炴帴鍒?SoC锛涘彟涓€鏂归潰锛岀▼搴?涔熷彲渚濊禆 self-hosted debug 妯″紡鏉ユ帰绱㈣皟璇曟柟娉曪紝鏈枃妗ｉ噸鐐瑰叧娉ㄨ繖涓€閮ㄥ垎銆?
璇ヨ皟璇曟ā鍧楁彁渚涘熀浜庨噰鏍风殑鎬ц兘鍓栨瀽锛坧rofiling锛夋墿灞曪紝鍙敤浜庡 CPU 绋嬪簭璁℃暟鍣ㄣ€?瀹夊叏鐘舵€佸拰寮傚父绾у埆绛夎繘琛岄噰鏍凤紱閫氬父姣忎釜 CPU 閮芥湁涓€涓笓鐢ㄧ殑璋冭瘯妯″潡涓庝箣杩炴帴銆?鍩轰簬 self-hosted debug 鏈哄埗锛孡inux 鍐呮牳鍙湪鍐呮牳鍙戠敓 panic 鏃朵粠 mmio 鍖哄煙
璁块棶杩欎簺鐩稿叧瀵勫瓨鍣ㄣ€傚唴鏍?panic 鐨勫洖璋冮€氱煡鍣紙callback notifier锛変細涓烘瘡涓?CPU
杞偍鐩稿叧瀵勫瓨鍣紱杩欐渶缁堟湁鍔╀簬瀵?panic 杩涜杈呭姪鍒嗘瀽銆?

### 瀹炵幇


- 鍦ㄩ┍鍔ㄦ敞鍐屾湡闂达紝瀹冧娇鐢?EDDEVID 鍜?EDDEVID1 杩欎袱涓澶?ID 瀵勫瓨鍣ㄦ潵鍒ゆ柇鏄惁
  瀹炵幇浜嗗熀浜庨噰鏍风殑鎬ц兘鍓栨瀽銆傚湪鏌愪簺骞冲彴涓婏紝璇ョ‖浠剁壒鎬ц瀹屽叏鎴栭儴鍒嗗疄鐜帮紱鑻ヤ笉
  鏀寔璇ョ壒鎬э紝鍒欐敞鍐屽皢澶辫触銆?
- 鍦ㄧ紪鍐欐湰鏂囨。鏃讹紝璋冭瘯椹卞姩涓昏渚濊禆鍐呮牳 panic 鍥炶皟閫氱煡鍣ㄤ粠涓変釜閲囨牱瀵勫瓨鍣ㄦ敹闆?  鐨勪俊鎭細EDPCSR銆丒DVIDSR 鍜?EDCIDSR锛氫粠 EDPCSR 鍙幏鍙栫▼搴忚鏁板櫒锛汦DVIDSR
  鍖呭惈瀹夊叏鐘舵€併€佸紓甯哥骇鍒€佷綅瀹界瓑淇℃伅锛汦DCIDSR 鏄笂涓嬫枃 ID 鍊硷紝鍖呭惈
  CONTEXTIDR_EL1 鐨勯噰鏍峰€笺€?
- 璇ラ┍鍔ㄦ敮鎸佽繍琛屼簬 AArch64 鎴?AArch32 妯″紡鐨?CPU銆備袱鑰呭瘎瀛樺櫒鍛藉悕绾﹀畾鐣ユ湁涓嶅悓锛?  AArch64 浣跨敤 'ED' 浣滀负瀵勫瓨鍣ㄥ墠缂€锛圓RM DDI 0487A.k锛孒9.1 绔狅級锛孉Arch32 浣跨敤
  'DBG' 浣滀负鍓嶇紑锛圓RM DDI 0487A.k锛孏5.1 绔狅級銆傞┍鍔ㄧ粺涓€閲囩敤 AArch64 鍛藉悕绾﹀畾銆?
- ARMv8-a锛圓RM DDI 0487A.k锛夊拰 ARMv7-a锛圓RM DDI 0406C.b锛夌殑瀵勫瓨鍣ㄤ綅瀹氫箟涓嶅悓銆?  鍥犳椹卞姩鏁村悎浜嗕袱鑰呯殑宸紓锛?
  鑻?PCSROffset=0b0000锛屽湪 ARMv8-a 涓?EDPCSR 鐗规€ф湭瀹炵幇锛涗絾 ARMv7-a 瀹氫箟涓?  鈥淧CSR 閲囨牱鍊间細鏍规嵁鎸囦护闆嗙姸鎬佸亸绉讳竴涓€尖€濄€傚浜?ARMv7-a锛岄┍鍔ㄨ繘涓€姝ユ鏌?CPU
  杩愯浜?ARM 杩樻槸 thumb 鎸囦护闆嗭紝骞跺 PCSR 鍊艰繘琛屾牎鍑嗭紝鍏充簬鍋忕Щ鐨勮缁嗚鏄庤
  ARMv7-a ARM锛圓RM DDI 0406C.b锛塁11.11.34 绔?鈥淒BGPCSR, Program Counter
  Sampling Register鈥濄€?
  鑻?PCSROffset=0b0010锛孉RMv8-a 瀹氫箟涓衡€滃凡瀹炵幇鐨?EDPCSR锛岄噰鏍蜂笉搴旂敤鍋忕Щ锛屼笖涓?  鍦?AArch32 鐘舵€佷笅閲囨牱鎸囦护闆嗙姸鎬佲€濄€傚洜姝ゅ湪 ARMv8 涓婏紝鑻?EDDEVID1.PCSROffset
  涓?0b0010 涓?CPU 杩愯浜?AArch32 鐘舵€侊紝鍒欎笉瀵?EDPCSR 閲囨牱锛涘綋 CPU 杩愯浜?  AArch64 鐘舵€佹椂锛孍DPCSR 琚噰鏍蜂笖涓嶅簲鐢ㄥ亸绉汇€?

### 鏃堕挓涓庣數婧愬煙


鍦ㄨ闂皟璇曞瘎瀛樺櫒涔嬪墠锛屽簲纭繚鏃堕挓鍜岀數婧愬煙宸叉纭娇鑳姐€傚湪 ARMv8-a ARM
锛圓RM DDI 0487A.k锛夌殑 'H9.1 Debug registers' 绔犺妭涓紝璋冭瘯瀵勫瓨鍣ㄥ垎甯冨湪涓や釜鍩熶腑锛?debug 鍩熷拰 CPU 鍩熴€?
```

                                +---------------+
                                |               |
                                |               |
                     +----------+--+            |
        dbg_clock -->|          |**|            |<-- cpu_clock
                     |    Debug |**|   CPU      |
 dbg_power_domain -->|          |**|            |<-- cpu_power_domain
                     +----------+--+            |
                                |               |
                                |               |
                                +---------------+

```
瀵逛簬 debug 鍩燂紝鐢ㄦ埛浣跨敤 DT binding锛堣澶囨爲缁戝畾锛夆€渃locks鈥?鍜?鈥減ower-domains鈥?鏉ヤ负璋冭瘯閫昏緫鎸囧畾鐩稿簲鐨勬椂閽熸簮鍜岀數婧愩€傞┍鍔ㄦ寜闇€璋冪敤 pm_runtime_{put|get} 鎿嶄綔鏉?澶勭悊璋冭瘯鐢垫簮鍩熴€?
瀵逛簬 CPU 鍩燂紝涓嶅悓鐨?SoC 璁捐鏈変笉鍚岀殑鐢垫簮绠＄悊鏂规锛屾渶缁堜細涓ラ噸褰卞搷 external
debug 妯″潡銆傚洜姝ゅ彲鍒嗕负浠ヤ笅鍑犵鎯呭喌锛?
- 鍦ㄥ叿鏈夊悎鐞嗙數婧愭帶鍒跺櫒銆佽兘姝ｇ‘澶勭悊 CPU 鐢垫簮鍩熺殑绯荤粺涓紝CPU 鐢垫簮鍩熷彲鐢遍┍鍔ㄤ腑鐨?  EDPRCR 瀵勫瓨鍣ㄦ帶鍒躲€傞┍鍔ㄩ鍏堝啓 EDPRCR.COREPURQ 浣嶄负 CPU 涓婄數锛岀劧鍚庡啓
  EDPRCR.CORENPDRQ 浣嶄互妯℃嫙 CPU 鎺夌數銆傝繖鏍峰彲浠ョ‘淇?CPU 鐢垫簮鍩熷湪璁块棶璋冭瘯鐩稿叧
  瀵勫瓨鍣ㄦ湡闂磋姝ｇ‘涓婄數锛?
- 鏌愪簺璁捐鍦ㄩ泦缇や腑鎵€鏈?CPU 鎺夌數鏃朵細鍏抽棴鏁翠釜闆嗙兢鈥斺€斿寘鎷湰搴斿湪 debug 鐢垫簮鍩熶腑
  淇濇寔渚涚數鐨勮皟璇曞瘎瀛樺櫒閮ㄥ垎銆傝繖浜涙儏鍐典笉浼氶伒寰?EDPRCR 涓殑浣嶏紝鍥犳杩欎簺璁捐鏃犳硶
  浠?CoreSight / Debug 璁捐鑰呴鏈熺殑鏂瑰紡鏀寔鎺夌數璋冭瘯銆傝繖鎰忓懗鐫€鍗充娇妫€鏌?EDPRSR锛?  鑻ョ洰鏍囧瘎瀛樺櫒鏈笂鐢碉紝涔熷彲鑳藉鑷存€荤嚎鎸傝捣锛坆us hang锛夈€?
  鍦ㄨ繖绉嶆儏鍐典笅锛屽湪璋冭瘯瀵勫瓨鍣ㄦ湭涓婄數鏃惰闂畠浠棤寮備簬鐏鹃毦锛涘洜姝ゆ垜浠渶瑕佸湪鍚姩鏃跺氨
  闃绘 CPU 浣庡姛鑰楃姸鎬侊紝鎴栧湪鐢ㄦ埛杩愯鏃跺惎鐢ㄦā鍧楁椂闃绘銆傝缁嗙敤娉曡鍙傝
  鈥淗ow to use the module鈥?绔犺妭銆?

### 璁惧鏍戠粦瀹?

鏈夊叧璇︾粏淇℃伅锛岃鍙傞槄 Documentation/devicetree/bindings/arm/arm,coresight-cpu-debug.yaml銆?

### 濡備綍浣跨敤璇ユā鍧?

鑻ヨ鍦ㄥ惎鍔ㄦ椂灏卞惎鐢ㄨ皟璇曞姛鑳斤紝鍙湪鍐呮牳鍛戒护琛屽弬鏁颁腑娣诲姞 鈥渃oresight_cpu_debug.enable=1鈥濄€?
璇ラ┍鍔ㄤ篃鍙綔涓烘ā鍧楀伐浣滐紝鍥犳鍙湪 insmod 鏃跺惎鐢ㄨ皟璇?
```

  # insmod coresight_cpu_debug.ko debug=1

```
鑻ュ湪鍚姩鎴?insmod 妯″潡鏃舵湭鍚敤璋冭瘯锛岄┍鍔ㄤ細浣跨敤 debugfs 鏂囦欢绯荤粺鎻愪緵涓€涓棆閽紝
鐢ㄤ簬鍔ㄦ€佸惎鐢ㄦ垨绂佺敤璋冭瘯锛?
```

  # echo 1 > /sys/kernel/debug/coresight_cpu_debug/enable

```

```

  # echo 0 > /sys/kernel/debug/coresight_cpu_debug/enable

```
濡?鈥淐lock and power domain鈥?绔犺妭鎵€杩帮紝鑻ヤ綘浣跨敤鐨勫钩鍙板叿鏈変細鍏抽棴璋冭瘯閫昏緫鐨勭┖闂?鐘舵€侊紝涓旂數婧愭帶鍒跺櫒鏃犳硶寰堝ソ鍦板搷搴旀潵鑷?EDPRCR 鐨勮姹傦紝鍒欏簲鍦ㄥ惎鐢?CPU 璋冭瘯鍔熻兘
涔嬪墠鍏堥檺鍒?CPU 绌洪棽鐘舵€侊紱杩欐牱鎵嶈兘纭繚瀵硅皟璇曢€昏緫鐨勮闂€?
鑻ヨ鍦ㄥ惎鍔ㄦ椂灏遍檺鍒剁┖闂茬姸鎬侊紝鍙湪鍐呮牳鍛戒护琛屼腑浣跨敤 鈥渘ohlt鈥?鎴?鈥渃puidle.off=1鈥濄€?
鍦ㄨ繍琛屾椂锛屽彲閫氳繃浠ヤ笅鏂规硶绂佺敤绌洪棽鐘舵€侊細

鍙互閫氳繃 PM QoS 瀛愮郴缁熺鐢?CPU 绌洪棽鐘舵€侊紝鏇村叿浣撳湴璇存槸浣跨敤 鈥?dev/cpu_dma_latency鈥?鎺ュ彛锛堣瑙?Documentation/power/pm_qos_interface.rst锛夈€傚 PM QoS 鏂囨。鎵€杩帮紝鎵€
璇锋眰鐨勫弬鏁板皢涓€鐩寸敓鏁堬紝鐩村埌鏂囦欢鎻忚堪绗﹁閲婃斁銆?
```

  # exec 3<> /dev/cpu_dma_latency; echo 0 >&3
  ...
  Do some work...
  ...
  # exec 3<>-

```
鍚屾牱鐨勬搷浣滀篃鍙粠搴旂敤绋嬪簭涓畬鎴愩€?
閫氳繃 cpuidle sysfs 绂佺敤鐗瑰畾 CPU 鐨勭壒瀹氱┖闂茬姸鎬侊紙鍙傝

```

  # echo 1 > /sys/devices/system/cpu/cpu$cpu/cpuidle/state$state/disable

```

### 杈撳嚭鏍煎紡


```

  ARM external debug module:
  coresight-cpu-debug 850000.debug: CPU[0]:
  coresight-cpu-debug 850000.debug:  EDPRSR:  00000001 (Power:On DLK:Unlock)
  coresight-cpu-debug 850000.debug:  EDPCSR:  handle_IPI+0x174/0x1d8
  coresight-cpu-debug 850000.debug:  EDCIDSR: 00000000
  coresight-cpu-debug 850000.debug:  EDVIDSR: 90000000 (State:Non-secure Mode:EL1/0 Width:64bits VMID:0)
  coresight-cpu-debug 852000.debug: CPU[1]:
  coresight-cpu-debug 852000.debug:  EDPRSR:  00000001 (Power:On DLK:Unlock)
  coresight-cpu-debug 852000.debug:  EDPCSR:  debug_notifier_call+0x23c/0x358
  coresight-cpu-debug 852000.debug:  EDCIDSR: 00000000
  coresight-cpu-debug 852000.debug:  EDVIDSR: 90000000 (State:Non-secure Mode:EL1/0 Width:64bits VMID:0)

```
