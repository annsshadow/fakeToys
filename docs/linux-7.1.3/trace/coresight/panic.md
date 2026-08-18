## 浣跨敤 Coresight 搴斿鍐呮牳 panic 涓庣湅闂ㄧ嫍澶嶄綅


### 绠€浠?
鏈枃妗ｄ粙缁嶅浣曚娇鐢?Linux coresight 璺熻釜鏀寔鏉ヨ皟璇曞唴鏍?panic 涓庣湅闂ㄧ嫍澶嶄綅鍦烘櫙銆?
### 鍐呮牳 panic 鏈熼棿鐨?Coresight 璺熻釜

浠?coresight 椹卞姩鐨勮搴︽潵鐪嬶紝澶勭悊鍐呮牳 panic 鎯呭舰鏈夊洓涓富瑕侀渶姹傘€?
a. 鏀寔浠庝繚鐣欏唴瀛樺尯鍩熷垎閰嶈窡韪紦鍐插尯椤点€傚钩鍙板彲閫氳繃鍦ㄧ浉鍏?coresight 鑺傜偣涓婃柊澧炵殑 device tree 灞炴€ф潵澹版槑杩欎竴鐐广€?
b. 鏀寔鍦ㄥ唴鏍?panic 鏃跺仠姝?coresight 妯″潡

c. 浠ユ寚瀹氭牸寮忎繚瀛樻墍闇€鐨勫厓鏁版嵁

d. 鏀寔璇诲彇鍐呮牳 panic 鏃舵崟鑾风殑璺熻釜鏁版嵁

#### 浠庝繚鐣?RAM 鍒嗛厤璺熻釜缂撳啿鍖洪〉

涓€涓柊鐨勫彲閫?device tree 灞炴€?"memory-region" 琚姞鍏ュ埌 Coresight TMC 璁惧鑺傜偣涓紝鐢ㄤ簬缁欏嚭璺熻釜缂撳啿鍖虹殑鍩哄湴鍧€涓庡ぇ灏忋€?
璺熻釜缂撳啿鍖虹殑闈欐€佸垎閰嶅彲纭繚 IOMMU 鍚敤涓庣鐢ㄤ袱绉嶆儏鍐甸兘琚鐞嗐€傛澶栵紝鏀寔鎸佷箙 RAM 鐨勫钩鍙板厑璁哥敤鎴峰湪鍚庣画鍚姩涓鍙栬窡韪暟鎹紝鑰屾棤闇€鍚姩 crashdump 鍐呮牳銆?
娉ㄦ剰锛?瀵逛簬 ETR sink 璁惧锛岃淇濈暀鍖哄煙灏嗗悓鏃剁敤浜庤窡韪崟鑾蜂笌璺熻釜鏁版嵁璇诲彇銆?瀵逛簬 ETF sink 璁惧锛屽皢浣跨敤鍐呴儴 SRAM 杩涜璺熻釜鎹曡幏锛屽苟鍚屾鍒颁繚鐣欏尯鍩熶互渚涜鍙栥€?

#### 鍦ㄥ唴鏍?panic 鏃剁鐢?coresight 妯″潡

涓轰簡閬垮厤鍐呮牳 panic 鍚庝涪澶辩浉鍏宠窡韪暟鎹紝鏈€濂藉湪鍐呮牳 panic 鏃跺仠姝?coresight 妯″潡銆?
杩欏彲浠ラ€氳繃閰嶇疆 comparator銆丆TI 涓?sink 鏉ュ疄鐜帮細

```
           Trigger on panic
    Comparator --->External out --->CTI -->External In---->ETR/ETF stop

```
#### 鍦ㄥ唴鏍?panic 鏃朵繚瀛樺厓鏁版嵁

Coresight 鍏冩暟鎹寘鍚櫎璺熻釜鏁版嵁澶栵紝鎴愬姛杩涜璺熻釜瑙ｇ爜鎵€闇€鐨勬墍鏈夐檮鍔犳暟鎹€傝繖鍖呮嫭 ETR/ETF/ETB 瀵勫瓨鍣ㄥ揩鐓х瓑銆?
涓烘锛屼竴涓柊鐨勫彲閫夎澶囧睘鎬?"memory-region" 琚姞鍏ュ埌 ETR/ETF/ETB 璁惧鑺傜偣涓€?
#### 璇诲彇鍐呮牳 panic 鏃舵崟鑾风殑璺熻釜鏁版嵁

鍐呮牳 panic 鏃舵崟鑾风殑璺熻釜鏁版嵁锛屽彲閫氳繃鐗规畩鐨勮澶囨枃浠?/dev/crash_tmc_xxx 浠庨噸鍚悗鐨勫唴鏍告垨 crashdump 鍐呮牳涓鍙栥€傝璁惧鏂囦欢浠呭湪瀛樺湪鏈夋晥 crashdata 鏃舵墠浼氳鍒涘缓銆?
#### 鍐呮牳 panic 鎯呭喌涓嬬殑璺熻釜鎹曡幏涓庤В鐮佷竴鑸祦绋?
1. 閫氳繃 sysfs 鎺ュ彛鍦ㄦ墍鏈夋牳涓婁娇鑳芥簮涓?sink銆侲TR sink 搴旈€氳繃浠?sysfs 閫夋嫨 "resrv" 缂撳啿鍖烘ā寮忥紝浠庝繚鐣欏唴瀛樺垎閰嶈窡韪紦鍐插尯銆?
2. 杩愯鐩稿叧娴嬭瘯銆?
3. 鍙戠敓鍐呮牳 panic 鏃讹紝鎵€鏈?coresight 妯″潡琚鐢紝蹇呰鐨勫厓鏁版嵁鐢卞唴鏍?panic 澶勭悊鍑芥暟鍚屾銆?
   绯荤粺鏈€缁堝皢閲嶅惎鎴栧惎鍔?crashdump 鍐呮牳銆?
4. 瀵逛簬鏀寔 crashdump 鍐呮牳鐨勫钩鍙帮紝鍙娇鐢?coresight sysfs 鎺ュ彛鐩存帴浠?crashdump 鍐呮牳杞偍鍘熷璺熻釜鏁版嵁銆傛绉嶆儏鍐典笅鏃犻渶鎸佷箙 RAM銆?
5. 瀵逛簬鏀寔鎸佷箙 RAM 鐨勫钩鍙帮紝鍙湪闅忓悗鐨?Linux 鍚姩涓€氳繃 coresight sysfs 鎺ュ彛杞偍璺熻釜鏁版嵁銆傛绉嶆儏鍐典笅鏃犻渶 crashdump 鍐呮牳銆傛寔涔?RAM 鍙‘淇濊窡韪暟鎹湪閲嶅惎鍚庝繚鎸佸畬鏁淬€?
### 鐪嬮棬鐙楀浣嶆湡闂寸殑 Coresight 璺熻釜

澶勭悊鐪嬮棬鐙楀浣嶄笌鍐呮牳 panic 鎯呭喌鐨勪富瑕佸尯鍒涓嬶細

a. 淇濆瓨 coresight 鍏冩暟鎹渶鐢?SCP锛堢郴缁熸帶鍒跺鐞嗗櫒锛夊浐浠舵寜鎸囧畾鏍煎紡璐熻矗锛岃€岄潪鍐呮牳銆?
b. 鍥轰欢涓鸿窡韪紦鍐插尯涓庡厓鏁版嵁鎻愪緵鐨勪繚鐣欏唴瀛樺尯鍩熷繀椤讳綅浜庢寔涔?RAM 涓€?   娉ㄦ剰锛氳繖鏄湅闂ㄧ嫍澶嶄綅鎯呭喌涓嬬殑瑕佹眰锛屼絾鍦ㄥ唴鏍?panic 鎯呭喌涓嬩负鍙€夐」銆?
鐪嬮棬鐙楀浣嶄粎鑳藉湪婊¤冻涓婅堪涓ら」瑕佹眰鐨勫钩鍙颁笂寰楀埌鏀寔銆?
### 浣跨敤 ETR sink 娴嬭瘯鍐呮牳 panic 鎯呭喌鐨勭ず渚嬪懡浠?

1. 鍦ㄥ唴鏍?bootargs 涓姞鍏?"crash_kexec_post_notifiers" 鍚姩 Linux 鍐呮牳銆傝嫢鐢ㄦ埛甯屾湜浠?crashdump 鍐呮牳璇诲彇璺熻釜鏁版嵁锛岃繖鏄繀闇€鐨勩€?
```

    #echo 1 > /sys/kernel/config/cs-syscfg/configurations/panicstop/enable

```
```

    #./cti_setup.sh

    #cat cti_setup.sh


    cd /sys/bus/coresight/devices/

    ap_cti_config () {
      #ETM trig out[0] trigger to Channel 0
      echo 0 4 > channels/trigin_attach
    }

    etf_cti_config () {
      #ETF Flush in trigger from Channel 0
      echo 0 1 > channels/trigout_attach
      echo 1 > channels/trig_filter_enable
    }

    etr_cti_config () {
      #ETR Flush in from Channel 0
      echo 0 1 > channels/trigout_attach
      echo 1 > channels/trig_filter_enable
    }

    ctidevs=`find . -name "cti*"`

    for i in $ctidevs
    do
            cd $i

            connection=`find . -name "ete*"`
            if [ ! -z "$connection" ]
            then
                    echo "AP CTI config for $i"
                    ap_cti_config
            fi

            connection=`find . -name "tmc_etf*"`
            if [ ! -z "$connection" ]
            then
                    echo "ETF CTI config for $i"
                    etf_cti_config
            fi

            connection=`find . -name "tmc_etr*"`
            if [ ! -z "$connection" ]
            then
                    echo "ETR CTI config for $i"
                    etr_cti_config
            fi

            cd ..
    done

```
娉細CTI 杩炴帴鏄?SoC 鐩稿叧鐨勶紝鍥犳涓婇潰鐨勮剼鏈粎渚涘弬鑰冦€?
```

    #echo "resrv" > /sys/bus/coresight/devices/tmc_etr0/buf_mode_preferred

```
```

    #echo 1 > /sys/bus/coresight/devices/tmc_etr0/stop_on_flush

```
6. 浣跨敤 sysfs 鎺ュ彛鍦ㄦ牳 1 涓庢牳 2 涓婂惎鍔?Coresight 璺熻釜

```

    #taskset -c 1 dd if=/dev/urandom of=/dev/null &

```
```

    #echo 1 > /proc/sys/kernel/panic
    #taskset -c 2 echo c > /proc/sysrq-trigger

```
```

    #dd if=/dev/crash_tmc_etr0 of=/trace/cstrace.bin

```
10. 杩愯 opencsd 瑙ｇ爜鍣ㄥ伐鍏?鑴氭湰鏉ョ敓鎴愭寚浠よ窡韪€?
#### 鎸囦护璺熻釜杞偍绀轰緥


```

    A                                  etm4_enable_hw: ffff800008ae1dd4
    CONTEXT EL2                        etm4_enable_hw: ffff800008ae1dd4
    I                                  etm4_enable_hw: ffff800008ae1dd4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1dd8:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1ddc:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de0:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de8:
    d503233f   paciasp
    I                                  etm4_enable_hw: ffff800008ae1dec:
    a9be7bfd   stp     x29, x30, [sp, #-32]!
    I                                  etm4_enable_hw: ffff800008ae1df0:
    910003fd   mov     x29, sp
    I                                  etm4_enable_hw: ffff800008ae1df4:
    a90153f3   stp     x19, x20, [sp, #16]
    I                                  etm4_enable_hw: ffff800008ae1df8:
    2a0003f4   mov     w20, w0
    I                                  etm4_enable_hw: ffff800008ae1dfc:
    900085b3   adrp    x19, ffff800009b95000 <reserved_mem+0xc48>
    I                                  etm4_enable_hw: ffff800008ae1e00:
    910f4273   add     x19, x19, #0x3d0
    I                                  etm4_enable_hw: ffff800008ae1e04:
    f8747a60   ldr     x0, [x19, x20, lsl #3]
    E                                  etm4_enable_hw: ffff800008ae1e08:
    b4000140   cbz     x0, ffff800008ae1e30 <etm4_starting_cpu+0x50>
    I    149.039572921                 etm4_enable_hw: ffff800008ae1e30:
    a94153f3   ldp     x19, x20, [sp, #16]
    I    149.039572921                 etm4_enable_hw: ffff800008ae1e34:
    52800000   mov     w0, #0x0                        // #0
    I    149.039572921                 etm4_enable_hw: ffff800008ae1e38:
    a8c27bfd   ldp     x29, x30, [sp], #32

    ..snip

        149.052324811           chacha_block_generic: ffff800008642d80:
    9100a3e0   add     x0,
    I    149.052324811           chacha_block_generic: ffff800008642d84:
    b86178a2   ldr     w2, [x5, x1, lsl #2]
    I    149.052324811           chacha_block_generic: ffff800008642d88:
    8b010803   add     x3, x0, x1, lsl #2
    I    149.052324811           chacha_block_generic: ffff800008642d8c:
    b85fc063   ldur    w3, [x3, #-4]
    I    149.052324811           chacha_block_generic: ffff800008642d90:
    0b030042   add     w2, w2, w3
    I    149.052324811           chacha_block_generic: ffff800008642d94:
    b8217882   str     w2, [x4, x1, lsl #2]
    I    149.052324811           chacha_block_generic: ffff800008642d98:
    91000421   add     x1, x1, #0x1
    I    149.052324811           chacha_block_generic: ffff800008642d9c:
    f100443f   cmp     x1, #0x11



```
```

    A                                  etm4_enable_hw: ffff800008ae1dd4
    CONTEXT EL2                        etm4_enable_hw: ffff800008ae1dd4
    I                                  etm4_enable_hw: ffff800008ae1dd4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1dd8:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1ddc:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de0:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de4:
    d503201f   nop
    I                                  etm4_enable_hw: ffff800008ae1de8:
    d503233f   paciasp
    I                                  etm4_enable_hw: ffff800008ae1dec:
    a9be7bfd   stp     x29, x30, [sp, #-32]!
    I                                  etm4_enable_hw: ffff800008ae1df0:
    910003fd   mov     x29, sp
    I                                  etm4_enable_hw: ffff800008ae1df4:
    a90153f3   stp     x19, x20, [sp, #16]
    I                                  etm4_enable_hw: ffff800008ae1df8:
    2a0003f4   mov     w20, w0
    I                                  etm4_enable_hw: ffff800008ae1dfc:
    900085b3   adrp    x19, ffff800009b95000 <reserved_mem+0xc48>
    I                                  etm4_enable_hw: ffff800008ae1e00:
    910f4273   add     x19, x19, #0x3d0
    I                                  etm4_enable_hw: ffff800008ae1e04:
    f8747a60   ldr     x0, [x19, x20, lsl #3]
    E                                  etm4_enable_hw: ffff800008ae1e08:
    b4000140   cbz     x0, ffff800008ae1e30 <etm4_starting_cpu+0x50>
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e30:
    a94153f3   ldp     x19, x20, [sp, #16]
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e34:
    52800000   mov     w0, #0x0                        // #0
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e38:
    a8c27bfd   ldp     x29, x30, [sp], #32
    I    149.046243445                 etm4_enable_hw: ffff800008ae1e3c:
    d50323bf   autiasp
    E    149.046243445                 etm4_enable_hw: ffff800008ae1e40:
    d65f03c0   ret
    A                                ete_sysreg_write: ffff800008adfa18

    ..snip

    I     149.05422547                          panic: ffff800008096300:
    a90363f7   stp     x23, x24, [sp, #48]
    I     149.05422547                          panic: ffff800008096304:
    6b00003f   cmp     w1, w0
    I     149.05422547                          panic: ffff800008096308:
    3a411804   ccmn    w0, #0x1, #0x4, ne  // ne = any
    N     149.05422547                          panic: ffff80000809630c:
    540001e0   b.eq    ffff800008096348 <panic+0xe0>  // b.none
    I     149.05422547                          panic: ffff800008096310:
    f90023f9   str     x25, [sp, #64]
    E     149.05422547                          panic: ffff800008096314:
    97fe44ef   bl      ffff8000080276d0 <panic_smp_self_stop>
    A                                           panic: ffff80000809634c
    I     149.05422547                          panic: ffff80000809634c:
    910102d5   add     x21, x22, #0x40
    I     149.05422547                          panic: ffff800008096350:
    52800020   mov     w0, #0x1                        // #1
    E     149.05422547                          panic: ffff800008096354:
    94166b8b   bl      ffff800008631180 <bust_spinlocks>
    N    149.054225518                 bust_spinlocks: ffff800008631180:
    340000c0   cbz     w0, ffff800008631198 <bust_spinlocks+0x18>
    I    149.054225518                 bust_spinlocks: ffff800008631184:
    f000a321   adrp    x1, ffff800009a98000 <pbufs.0+0xbb8>
    I    149.054225518                 bust_spinlocks: ffff800008631188:
    b9405c20   ldr     w0, [x1, #92]
    I    149.054225518                 bust_spinlocks: ffff80000863118c:
    11000400   add     w0, w0, #0x1
    I    149.054225518                 bust_spinlocks: ffff800008631190:
    b9005c20   str     w0, [x1, #92]
    E    149.054225518                 bust_spinlocks: ffff800008631194:
    d65f03c0   ret
    A                                           panic: ffff800008096358

```
### 鍩轰簬 Perf 鐨勬祴璇?

#### 鍚姩 perf 浼氳瘽

```

    perf record -e cs_etm/panicstop,@tmc_etf1/ -C 1
    perf record -e cs_etm/panicstop,@tmc_etf2/ -C 2

```
```

    perf record -e cs_etm/panicstop,@tmc_etr0/ -C 1,2

```
#### panic 鍚庤鍙栬窡韪暟鎹?
涓婃枃浠嬬粛鐨勭浉鍚岀殑鍩轰簬 sysfs 鐨勬柟娉曪紝鍙敤浜庡湪鍐呮牳 panic 閲嶅惎鍚庤幏鍙栧苟瑙ｇ爜璺熻釜鏁版嵁銆?