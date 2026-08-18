
### RISC-V 纭欢鎺㈡祴鎺ュ彛


RISC-V 纭欢鎺㈡祴鎺ュ彛鍥寸粫涓€涓崟鐙殑 syscall 鏋勫缓锛屽叾
```

    struct riscv_hwprobe {
        __s64 key;
        __u64 value;
    };

    long sys_riscv_hwprobe(struct riscv_hwprobe *pairs, size_t pair_count,
                           size_t cpusetsize, cpu_set_t *cpus,
                           unsigned int flags);

```
   鍙傛暟鍒嗕负涓夌粍锛氫竴缁勯敭鍊煎鏁扮粍銆佷竴涓?CPU 闆嗗悎锛屼互鍙婁竴浜涙爣蹇椼€傞敭鍊煎甯︽湁鏁伴噺鎻愪緵銆傜敤鎴风┖闂村繀椤讳负姣忎釜鍏冪礌鐨?key 瀛楁棰勫厛濉€硷紝濡傛灉 key 琚瘑鍒紝鍐呮牳浼氬～鍏ュ叾 value銆傚鏋滃唴鏍镐笉璁よ瘑鏌愪釜 key锛屽叾 key 瀛楁浼氳娓呬负 -1锛寁alue 璁句负 0銆侰PU 闆嗗悎鐢?CPU_SET(3) 瀹氫箟锛屽ぇ灏忎负 `cpusetsize` 瀛楄妭銆傚浜庣被鍊肩殑 key锛堜緥濡?vendor銆乤rch銆乮mpl锛夛紝浠呭綋缁欏畾闆嗗悎涓殑鎵€鏈?CPU 鍏锋湁鐩稿悓鍊兼椂锛岃繑鍥炲€兼墠鏈夋晥銆傚惁鍒欏皢杩斿洖 -1銆傚浜庣被甯冨皵鐨?key锛岃繑鍥炲€兼槸鎵€鎸囧畾 CPU 鍊肩殑閫昏緫涓庛€傜敤鎴锋ā寮忓彲浠ュ皢 `cpus` 璁句负 NULL銆佸皢 `cpusetsize` 璁句负 0锛屼綔涓烘墍鏈夊湪绾?CPU 鐨勭畝鍐欍€傚綋鍓嶆敮鎸佺殑鏍囧織濡備笅锛?
- `RISCV_HWPROBE_WHICH_CPUS`锛氳鏍囧織鍩烘湰涓婂弽杞簡 sys_riscv_hwprobe() 鐨勮涓恒€傚畠涓嶆槸涓虹粰瀹氱殑 CPU 闆嗗悎濉厖 key 鐨勫€硷紝鑰屾槸缁欏嚭姣忎釜 key 鐨勫€硷紝骞剁敱 sys_riscv_hwprobe() 灏?CPU 闆嗗悎缂╁噺涓轰粎閭ｄ簺涓庢瘡涓敭鍊煎閮藉尮閰嶇殑 CPU銆傚浣曞尮閰嶅彇鍐充簬 key 鐨勭被鍨嬨€傚浜庣被鍊肩殑 key锛屽尮閰嶆剰鍛崇潃瀹屽叏绛変簬璇ュ€笺€傚浜庣被甯冨皵鐨?key锛屽尮閰嶆剰鍛崇潃璇ュ鐨勫€间笌 CPU 鐨勫€肩殑閫昏緫涓庣粨鏋滃畬鍏ㄧ瓑浜庤瀵圭殑鍊笺€傛澶栵紝褰?`cpus` 涓虹┖闆嗗悎鏃讹紝瀹冭鍒濆鍖栦负鍏朵腑鑳藉绾崇殑鎵€鏈夊湪绾?CPU锛屽嵆杩斿洖鐨?CPU 闆嗗悎鏄敤澶у皬涓?`cpusetsize` 鐨?CPU 闆嗗悎鎵€鑳借〃绀虹殑鎵€鏈夊湪绾?CPU 鐨勭缉鍑忕粨鏋溿€?
鎵€鏈夊叾浠栨爣蹇椾繚鐣欎緵灏嗘潵鍏煎浣跨敤锛屽繀椤讳负闆躲€?
鎴愬姛鏃惰繑鍥?0锛屽け璐ユ椂杩斿洖璐熺殑閿欒鐮併€?
瀹氫箟浜嗕互涓?key锛?
- `RISCV_HWPROBE_KEY_MVENDORID`: 鍖呭惈 `mvendorid` 鐨勫€硷紝瀹氫箟瑙?RISC-V 鐗规潈鏋舵瀯瑙勮寖銆?
- `RISCV_HWPROBE_KEY_MARCHID`: 鍖呭惈 `marchid` 鐨勫€硷紝瀹氫箟瑙?RISC-V 鐗规潈鏋舵瀯瑙勮寖銆?
- `RISCV_HWPROBE_KEY_MIMPID`: 鍖呭惈 `mimpid` 鐨勫€硷紝瀹氫箟瑙?RISC-V 鐗规潈鏋舵瀯瑙勮寖銆?
- `RISCV_HWPROBE_KEY_BASE_BEHAVIOR`: 涓€涓綅鎺╃爜锛屽寘鍚湰鍐呮牳鎵€鏀寔鐨勫熀鏈敤鎴峰彲瑙佽涓恒€傚畾涔変簡浠ヤ笅鍩烘湰鐢ㄦ埛 ABI锛?
  - `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`: 鏀寔 rv32ima 鎴?rv64ima锛屽畾涔夎鐢ㄦ埛 ISA 2.2 鐗堝拰鐗规潈 ISA 1.10 鐗堬紝骞跺叿鏈変互涓嬪凡鐭ヤ緥澶栵紙鍙兘浼氭坊鍔犳洿澶氫緥澶栵紝浣嗗墠鎻愭槸瑕佽兘璇佹槑鐢ㄦ埛 ABI 鏈鐮村潖锛夛細

    - 鐢ㄦ埛绌洪棿绋嬪簭涓嶈兘鐩存帴鎵ц `fence.i` 鎸囦护锛堜粛鍙€氳繃 vDSO 绛夊唴鏍告帶鍒剁殑鏈哄埗鍦ㄧ敤鎴风┖闂存墽琛岋級銆?
- `RISCV_HWPROBE_KEY_IMA_EXT_0`: 涓€涓綅鎺╃爜锛屽寘鍚笌 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`锛氬熀鏈郴缁熻涓哄吋瀹圭殑鎵╁睍銆?
  - `RISCV_HWPROBE_IMA_FD`: 鏀寔 F 鍜?D 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽鐨?commit cd20cee锛?FMIN/FMAX 鐜板湪瀹炵幇 minimumNumber/maximumNumber锛岃€岄潪 minNum/maxNum"锛夈€?
  - `RISCV_HWPROBE_IMA_C`: 鏀寔 C 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽 2.2 鐗堛€?
  - `RISCV_HWPROBE_IMA_V`: 鏀寔 V 鎵╁睍锛屽畾涔夎 RISC-V 鍚戦噺鎵╁睍鎵嬪唽 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZBA`: 鏀寔 Zba 鍦板潃鐢熸垚鎵╁睍锛屽畾涔夎浣嶆搷浣?ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZBB`: 鏀寔 Zbb 鎵╁睍锛屽畾涔夎浣嶆搷浣?ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZBS`: 鏀寔 Zbs 鎵╁睍锛屽畾涔夎浣嶆搷浣?ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZICBOZ`: 鏀寔 Zicboz 鎵╁睍锛屼簬 riscv-CMOs 鐨?commit 3dd606f锛?Create cmobase-v1.0.pdf"锛変腑琚壒鍑嗐€?
  - `RISCV_HWPROBE_EXT_ZBC`: 鏀寔 Zbc 鎵╁睍锛屽畾涔夎浣嶆搷浣?ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZBKB`: 鏀寔 Zbkb 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZBKC`: 鏀寔 Zbkc 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZBKX`: 鏀寔 Zbkx 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZKND`: 鏀寔 Zknd 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZKNE`: 鏀寔 Zkne 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZKNH`: 鏀寔 Zknh 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZKSED`: 鏀寔 Zksed 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZKSH`: 鏀寔 Zksh 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZKT`: 鏀寔 Zkt 鎵╁睍锛屽畾涔夎鏍囬噺鍔犲瘑 ISA 鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVBB`: 鏀寔 Zvbb 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVBC`: 鏀寔 Zvbc 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKB`: 鏀寔 Zvkb 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKG`: 鏀寔 Zvkg 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKNED`: 鏀寔 Zvkned 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKNH... Zvknha`: 鏀寔 Zvknha 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKNHB`: 鏀寔 Zvknhb 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKSED`: 鏀寔 Zvksed 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKSH`: 鏀寔 Zvksh 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVKT`: 鏀寔 Zvkt 鎵╁睍锛屽畾涔夎 RISC-V 鍔犲瘑鎵╁睍 绗簩鍗?1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZFH`: 鏀寔 Zfh 鎵╁睍 1.0 鐗堬紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽銆?
  - `RISCV_HWPROBE_EXT_ZFHMIN`: 鏀寔 Zfhmin 鎵╁睍 1.0 鐗堬紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽銆?
  - `RISCV_HWPROBE_EXT_ZIHINTNTL`: 鏀寔 Zihintntl 鎵╁睍 1.0 鐗堬紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽銆?
  - `RISCV_HWPROBE_EXT_ZVFH`: 鏀寔 Zvfh 鎵╁睍锛屽畾涔夎 RISC-V 鍚戦噺鎵嬪唽锛岃嚜 commit e2ccd0548d6c锛?Remove draft warnings from Zvfh[min]"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZVFHMIN`: 鏀寔 Zvfhmin 鎵╁睍锛屽畾涔夎 RISC-V 鍚戦噺鎵嬪唽锛岃嚜 commit e2ccd0548d6c锛?Remove draft warnings from Zvfh[min]"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZFA`: 鏀寔 Zfa 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit 056b6ff467c7锛?Zfa is ratified"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZTSO`: 鏀寔 Ztso 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit 5618fb5a216b锛?Ztso is now ratified."锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZACAS`: 鏀寔 Zacas 鎵╁睍锛屽畾涔夎鍘熷瓙姣旇緝骞朵氦鎹紙CAS锛夋寚浠ゆ墜鍐岋紝鑷?commit 5059e0ca641c锛?update to ratified"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZICNTR`: 鏀寔 Zicntr 鎵╁睍 2.0 鐗堬紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽銆?
  - `RISCV_HWPROBE_EXT_ZICOND`: 鏀寔 Zicond 鎵╁睍锛屽畾涔夎 RISC-V 鏁存暟鏉′欢锛圸icond锛夋搷浣滄墿灞曟墜鍐岋紝鑷?commit 95cf1f9锛?Add changes requested by Ved during signoff"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZIHINTPAUSE`: 鏀寔 Zihintpause 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit d8ab5c78c207锛?Zihintpause is ratified"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZIHPM`: 鏀寔 Zihpm 鎵╁睍 2.0 鐗堬紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽銆?
  - `RISCV_HWPROBE_EXT_ZVE32X`: 鏀寔鍚戦噺瀛愭墿灞?Zve32x锛屽畾涔夎 RISC-V 鍚戦噺鎵╁睍鎵嬪唽 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVE32F`: 鏀寔鍚戦噺瀛愭墿灞?Zve32f锛屽畾涔夎 RISC-V 鍚戦噺鎵╁睍鎵嬪唽 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVE64X`: 鏀寔鍚戦噺瀛愭墿灞?Zve64x锛屽畾涔夎 RISC-V 鍚戦噺鎵╁睍鎵嬪唽 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVE64F`: 鏀寔鍚戦噺瀛愭墿灞?Zve64f锛屽畾涔夎 RISC-V 鍚戦噺鎵╁睍鎵嬪唽 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZVE64D`: 鏀寔鍚戦噺瀛愭墿灞?Zve64d锛屽畾涔夎 RISC-V 鍚戦噺鎵╁睍鎵嬪唽 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZIMOP`: 鏀寔 Zimop锛堝彲鑳戒负鎿嶄綔锛孧ay-Be-Operations锛夋墿灞曪紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽锛岃嚜 commit 58220614a5f锛?Zimop is ratified/1.0"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZCA`: 鏀寔 Zca 鎵╁睍锛屽畠鏄敤浜庡噺灏忎唬鐮佸ぇ灏忕殑 Zc* 鏍囧噯鎵╁睍鐨勪竴閮ㄥ垎锛屼簬 riscv-code-size-reduction 鐨?commit 8be3419c1c0锛?Zcf doesn't exist on RV64 as it contains no instructions"锛変腑琚壒鍑嗐€?
  - `RISCV_HWPROBE_EXT_ZCB`: 鏀寔 Zcb 鎵╁睍锛屽畠鏄敤浜庡噺灏忎唬鐮佸ぇ灏忕殑 Zc* 鏍囧噯鎵╁睍鐨勪竴閮ㄥ垎锛屼簬 riscv-code-size-reduction 鐨?commit 8be3419c1c0 涓鎵瑰噯銆?
  - `RISCV_HWPROBE_EXT_ZCD`: 鏀寔 Zcd 鎵╁睍锛屽畠鏄敤浜庡噺灏忎唬鐮佸ぇ灏忕殑 Zc* 鏍囧噯鎵╁睍鐨勪竴閮ㄥ垎锛屼簬 riscv-code-size-reduction 鐨?commit 8be3419c1c0 涓鎵瑰噯銆?
  - `RISCV_HWPROBE_EXT_ZCF`: 鏀寔 Zcf 鎵╁睍锛屽畠鏄敤浜庡噺灏忎唬鐮佸ぇ灏忕殑 Zc* 鏍囧噯鎵╁睍鐨勪竴閮ㄥ垎锛屼簬 riscv-code-size-reduction 鐨?commit 8be3419c1c0 涓鎵瑰噯銆?
  - `RISCV_HWPROBE_EXT_ZCMOP`: 鏀寔 Zcmop锛堝彲鑳戒负鎿嶄綔锛夋墿灞曪紝瀹氫箟瑙?RISC-V ISA 鎵嬪唽锛岃嚜 commit c732a4f39a4锛?Zcmop is ratified/1.0"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZAWRS`: 鏀寔 Zawrs 鎵╁睍锛屼簬 riscv-isa-manual 鐨?commit 98918c844281锛?Merge pull request #1217 from riscv/zawrs"锛変腑琚壒鍑嗐€?
  - `RISCV_HWPROBE_EXT_ZAAMO`: 鏀寔 Zaamo 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit e87412e621f1锛?integrate Zaamo and Zalrsc text (#1304)"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZALASR`: 鏀寔 Zalasr 鎵╁睍锛屼簬 riscv-zalasr 鐨?commit 194f0094锛?Version 0.9 for freeze"锛夊鍐荤粨銆?
  - `RISCV_HWPROBE_EXT_ZALRSC`: 鏀寔 Zalrsc 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit e87412e621f1锛?integrate Zaamo and Zalrsc text (#1304)"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_SUPM`: 鏀寔 Supm 鎵╁睍锛屽畾涔夎 RISC-V 鎸囬拡鎺╃爜鎵╁睍 1.0 鐗堛€?
  - `RISCV_HWPROBE_EXT_ZFBFMIN`: 鏀寔 Zfbfmin 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit 4dc23d6229de锛?Added Chapter title to BF16"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZVFBFMIN`: 鏀寔 Zvfbfmin 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit 4dc23d6229de锛?Added Chapter title to BF16"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZVFBFWMA`: 鏀寔 Zvfbfwma 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 commit 4dc23d6229de锛?Added Chapter title to BF16"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZICBOM`: 鏀寔 Zicbom 鎵╁睍锛屼簬 riscv-CMOs 鐨?commit 3dd606f锛?Create cmobase-v1.0.pdf"锛変腑琚壒鍑嗐€?
  - `RISCV_HWPROBE_EXT_ZABHA`: 鏀寔 Zabha 鎵╁睍锛屼簬 riscv-zabha 鐨?commit 49f49c842ff9锛?Update to Rafified state"锛変腑琚壒鍑嗐€?
  - `RISCV_HWPROBE_EXT_ZICBOP`: 鏀寔 Zicbop 鎵╁睍锛屼簬 riscv-CMOs 鐨?commit 3dd606f锛?Create cmobase-v1.0.pdf"锛変腑琚壒鍑嗐€?
  - `RISCV_HWPROBE_EXT_ZILSD`: 鏀寔 Zilsd 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 riscv-isa-manual 鐨?commit f88abf1锛?Integrating load/store pair for RV32 with the main manual"锛夎捣銆?
  - `RISCV_HWPROBE_EXT_ZCLSD`: 鏀寔 Zclsd 鎵╁睍锛屽畾涔夎 RISC-V ISA 鎵嬪唽锛岃嚜 riscv-isa-manual 鐨?commit f88abf1锛?Integrating load/store pair for RV32 with the main manual"锛夎捣銆?
- `RISCV_HWPROBE_KEY_CPUPERF_0`: 宸插純鐢ㄣ€傝繑鍥炰笌 `RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF` 绫讳技鐨勫€硷紝浣嗚 key 琚敊璇湴褰掔被涓轰綅鎺╃爜鑰岄潪鍊笺€?
- `RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF`: 涓€涓灇涓惧€硷紝鎻忚堪鎵€閫夊鐞嗗櫒闆嗗悎涓婃湭瀵归綈鏍囬噺鏈満瀛楄闂殑鎬ц兘銆?
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN`: 鏈榻愭爣閲忚闂殑鎬ц兘鏈煡銆?
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED`: 鏈榻愭爣閲忚闂€氳繃杞欢妯℃嫙锛屾ā鎷熷彂鐢熷湪鍐呮牳涓垨鍐呮牳涔嬩笅銆傝繖浜涜闂€绘槸鏋佹參銆?
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW`: 鏈榻愭爣閲忔湰鏈哄瓧澶у皬鐨勮闂瘮鍚岀瓑鏁伴噺鐨勫瓧鑺傝闂洿鎱€傛湭瀵归綈璁块棶鍙兘鐢辩‖浠剁洿鎺ユ敮鎸侊紝涔熷彲鑳借鎹曡幏骞剁敱杞欢妯℃嫙銆?
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_FAST`: 鏈榻愭爣閲忔湰鏈哄瓧澶у皬鐨勮闂瘮鍚岀瓑鏁伴噺鐨勫瓧鑺傝闂洿蹇€?
  - `RISCV_HWPROBE_MISALIGNED_SCALAR_UNSUPPORTED`: 瀹屽叏涓嶆敮鎸佹湭瀵归綈鏍囬噺璁块棶锛屼細鐢熸垚鏈榻愬湴鍧€閿欒銆?
- `RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE`: 涓€涓棤绗﹀彿鏁存暟锛岃〃绀?Zicboz 鍧楃殑澶у皬锛堜互瀛楄妭涓哄崟浣嶏級銆?
- `RISCV_HWPROBE_KEY_HIGHEST_VIRT_ADDRESS`: 涓€涓棤绗﹀彿闀挎暣鏁帮紝琛ㄧず鍙敤鐨勬渶楂樼敤鎴风┖闂磋櫄鎷熷湴鍧€銆?
- `RISCV_HWPROBE_KEY_TIME_CSR_FREQ`: `time CSR` 鐨勯鐜囷紙鍗曚綅 Hz锛夈€?
- `RISCV_HWPROBE_KEY_MISALIGNED_VECTOR_PERF`: 涓€涓灇涓惧€硷紝鎻忚堪鎵€閫夊鐞嗗櫒闆嗗悎涓婃湭瀵归綈鍚戦噺璁块棶鐨勬€ц兘銆?
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN`: 鏈榻愬悜閲忚闂殑鎬ц兘鏈煡銆?
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_SLOW`: 浣跨敤鍚戦噺瀵勫瓨鍣ㄧ殑 32 浣嶆湭瀵归綈璁块棶姣旈€氳繃鍚戦噺瀵勫瓨鍣ㄧ殑鍚岀瓑鏁伴噺瀛楄妭璁块棶鏇存參銆傛湭瀵归綈璁块棶鍙兘鐢辩‖浠剁洿鎺ユ敮鎸侊紝涔熷彲鑳借鎹曡幏骞剁敱杞欢妯℃嫙銆?
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_FAST`: 浣跨敤鍚戦噺瀵勫瓨鍣ㄧ殑 32 浣嶆湭瀵归綈璁块棶姣旈€氳繃鍚戦噺瀵勫瓨鍣ㄧ殑鍚岀瓑鏁伴噺瀛楄妭璁块棶鏇村揩銆?
  - `RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED`: 瀹屽叏涓嶆敮鎸佹湭瀵归綈鍚戦噺璁块棶锛屼細鐢熸垚鏈榻愬湴鍧€閿欒銆?
- `RISCV_HWPROBE_KEY_VENDOR_EXT_MIPS_0`: 涓€涓綅鎺╃爜锛屽寘鍚笌 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`锛氬熀鏈郴缁熻涓哄吋瀹圭殑 mips 鍘傚晢鎵╁睍銆?
  - MIPS

    - `RISCV_HWPROBE_VENDOR_EXT_XMIPSEXECTL`: 鍦?MIPS ISA 鎵╁睍瑙勮寖涓敮鎸?xmipsexectl 鍘傚晢鎵╁睍銆?
- `RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0`: 涓€涓綅鎺╃爜锛屽寘鍚笌 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`锛氬熀鏈郴缁熻涓哄吋瀹圭殑 thead 鍘傚晢鎵╁睍銆?
  - T-HEAD

    - `RISCV_HWPROBE_VENDOR_EXT_XTHEADVECTOR`: 鍦?T-Head ISA 鎵╁睍瑙勮寖涓敮鎸?xtheadvector 鍘傚晢鎵╁睍锛岃嚜 commit a18c801634锛?Add T-Head VECTOR vendor extension. "锛夎捣銆?
- `RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE`: 涓€涓棤绗﹀彿鏁存暟锛岃〃绀?Zicbom 鍧楃殑澶у皬锛堜互瀛楄妭涓哄崟浣嶏級銆?
- `RISCV_HWPROBE_KEY_VENDOR_EXT_SIFIVE_0`: 涓€涓綅鎺╃爜锛屽寘鍚笌 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`锛氬熀鏈郴缁熻涓哄吋瀹圭殑 sifive 鍘傚晢鎵╁睍銆?
  - SIFIVE

    - `RISCV_HWPROBE_VENDOR_EXT_XSFVQMACCDOD`: 鍦?SiFive Int8 鐭╅樀涔樻硶鎵╁睍瑙勮寖 1.1 鐗堜腑鏀寔 Xsfqmaccdod 鍘傚晢鎵╁睍銆?
    - `RISCV_HWPROBE_VENDOR_EXT_XSFVQMACCQOQ`: 鍦?SiFive Int8 鐭╅樀涔樻硶鎸囦护鎵╁睍瑙勮寖 1.1 鐗堜腑鏀寔 Xsfqmaccqoq 鍘傚晢鎵╁睍銆?
    - `RISCV_HWPROBE_VENDOR_EXT_XSFVFNRCLIPXFQF`: 鍦?SiFive FP32 鍒?int8 鑼冨洿瑁佸壀鎸囦护鎵╁睍瑙勮寖 1.0 鐗堜腑鏀寔 Xsfvfnrclipxfqf 鍘傚晢鎵╁睍銆?
    - `RISCV_HWPROBE_VENDOR_EXT_XSFVFWMACCQQQ`: 鍦ㄧ煩闃典箻绱姞鎸囦护鎵╁睍瑙勮寖 1.0 鐗堜腑鏀寔 Xsfvfwmaccqqq 鍘傚晢鎵╁睍銆?
- `RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE`: 涓€涓棤绗﹀彿鏁存暟锛岃〃绀?Zicbop 鍧楃殑澶у皬锛堜互瀛楄妭涓哄崟浣嶏級銆?
- `RISCV_HWPROBE_KEY_IMA_EXT_1`: 涓€涓綅鎺╃爜锛屽寘鍚笌 `RISCV_HWPROBE_BASE_BEHAVIOR_IMA`锛氬熀鏈郴缁熻涓哄吋瀹圭殑闄勫姞鎵╁睍銆?