
## LoongArch 绠€浠?

LoongArch 鏄竴绉嶆柊鐨?RISC 鎸囦护闆嗘灦鏋勶紙ISA锛夛紝鏈夌偣绫讳技浜?MIPS 鎴?RISC-V銆傜洰鍓嶆湁 3 绉嶅彉浣擄細绮剧畝 32 浣嶇増鏈紙LA32R锛夈€佹爣鍑?32 浣嶇増鏈紙LA32S锛夊拰 64 浣嶇増鏈紙LA64锛夈€侺oongArch 涓畾涔変簡 4 涓壒鏉冪骇锛圥LV锛夛細PLV0~PLV3锛屼粠楂樺埌浣庢帓鍒椼€傚唴鏍歌繍琛屽湪 PLV0锛岃€屽簲鐢ㄧ▼搴忚繍琛屽湪 PLV3銆傛湰鏂囨。浠嬬粛 LoongArch 鐨勫瘎瀛樺櫒銆佸熀鏈寚浠ら泦銆佽櫄鎷熷唴瀛樹互鍙婂叾浠栦竴浜涗富棰樸€?
## 瀵勫瓨鍣?

LoongArch 鐨勫瘎瀛樺櫒鍖呮嫭閫氱敤瀵勫瓨鍣紙GPR锛夈€佹诞鐐瑰瘎瀛樺櫒锛團PR锛夈€佸悜閲忓瘎瀛樺櫒锛圴R锛夛紝浠ュ強鐢ㄤ簬鐗规潈妯″紡锛圥LV0锛夌殑鎺у埗鐘舵€佸瘎瀛樺櫒锛圕SR锛夈€?
### 閫氱敤瀵勫瓨鍣紙GPR锛?

LoongArch 鏈?32 涓€氱敤瀵勫瓨鍣紙GPR锛夛紙`$r0` ~ `$r31`锛夛紱鍦?LA32 涓瘡涓负 32 浣嶅锛屽湪 LA64 涓负 64 浣嶅銆俙$r0` 琚‖杩炵嚎涓洪浂锛屽叾浠栧瘎瀛樺櫒鍦ㄦ灦鏋勪笂娌℃湁鐗规畩涔嬪銆傦紙`$r1` 闄ゅ锛屽畠琚‖杩炵嚎涓?BL 鎸囦护鐨勯摼鎺ュ瘎瀛樺櫒銆傦級

鍐呮牳浣跨敤浜?LoongArch 瀵勫瓨鍣ㄧ害瀹氱殑涓€涓彉浣擄紝濡傚弬鑰冭祫鏂?<loongarch-references> 涓殑 LoongArch ELF psABI 瑙勮寖鎵€杩帮細

================= =============== =================== ============
鍚嶇О              鍒悕            鐢ㄩ€?               璺ㄨ皟鐢ㄤ繚鐣?================= =============== =================== ============
`$r0`           `$zero`       甯搁噺闆?            鏈娇鐢?`$r1`           `$ra`         杩斿洖鍦板潃           鍚?`$r2`           `$tp`         TLS/绾跨▼鎸囬拡       鏈娇鐢?`$r3`           `$sp`         鏍堟寚閽?            鏄?`$r4`-`$r11`  `$a0`-`$a7` 鍙傛暟瀵勫瓨鍣?        鍚?`$r4`-`$r5`   `$v0`-`$v1` 杩斿洖鍊?            鍚?`$r12`-`$r20` `$t0`-`$t8` 涓存椂瀵勫瓨鍣?        鍚?`$r21`          `$u0`         姣?CPU 鍩哄潃        鏈娇鐢?`$r22`          `$fp`         甯ф寚閽?            鏄?`$r23`-`$r31` `$s0`-`$s8` 闈欐€佸瘎瀛樺櫒         鏄?================= =============== =================== ============

    瀵勫瓨鍣?`$r21` 鍦?ELF psABI 涓繚鐣欙紝浣嗚 Linux 鍐呮牳鐢ㄤ簬瀛樺偍姣?CPU 鍩哄潃銆傚畠閫氬父娌℃湁 ABI 鍚嶇О锛屽湪鍐呮牳涓О涓?`$u0`銆備綘涔熷彲鑳藉湪涓€浜涙棫浠ｇ爜涓湅鍒?`$v0` 鎴?`$v1`锛屼絾瀹冧滑鍒嗗埆鏄?`$a0` 鍜?`$a1` 宸插簾寮冪殑鍒悕銆?
### 娴偣瀵勫瓨鍣紙FPR锛?

褰撳瓨鍦?FPU 鏃讹紝LoongArch 鏈?32 涓诞鐐瑰瘎瀛樺櫒锛團PR锛夛紙`$f0` ~ `$f31`锛夈€傚湪 LA64 鏍稿績涓婃瘡涓负 64 浣嶅銆?
娴偣瀵勫瓨鍣ㄧ害瀹氫笌 LoongArch ELF psABI 瑙勮寖涓墍杩扮浉鍚岋細

================= ================== =================== ============
鍚嶇О              鍒悕              鐢ㄩ€?               璺ㄨ皟鐢ㄤ繚鐣?================= ================== =================== ============
`$f0`-`$f7`   `$fa0`-`$fa7`  鍙傛暟瀵勫瓨鍣?         鍚?`$f0`-`$f1`   `$fv0`-`$fv1`  杩斿洖鍊?             鍚?`$f8`-`$f23`  `$ft0`-`$ft15` 涓存椂瀵勫瓨鍣?         鍚?`$f24`-`$f31` `$fs0`-`$fs7`  闈欐€佸瘎瀛樺櫒          鏄?================= ================== =================== ============

    浣犲彲鑳戒細鍦ㄤ竴浜涙棫浠ｇ爜涓湅鍒?`$fv0` 鎴?`$fv1`锛屼絾瀹冧滑鍒嗗埆鏄?`$fa0` 鍜?`$fa1` 宸插簾寮冪殑鍒悕銆?
### 鍚戦噺瀵勫瓨鍣紙VR锛?

鐩墠 LoongArch 鏈?2 绉嶅悜閲忔墿灞曪細

- LSX锛堥緳鑺?SIMD 鎵╁睍锛孡oongson SIMD eXtension锛夛紝鍚戦噺涓?128 浣嶏紝
- LASX锛堥緳鑺珮绾?SIMD 鎵╁睍锛孡oongson Advanced SIMD eXtension锛夛紝鍚戦噺涓?256 浣嶃€?
LSX 鎻愪緵 `$v0` ~ `$v31`锛岃€?LASX 鎻愪緵 `$x0` ~ `$x31` 浣滀负鍚戦噺瀵勫瓨鍣ㄣ€?
VR 涓?FPR 閲嶅彔锛氫緥濡傦紝鍦ㄥ疄鐜?LSX 鍜?LASX 鐨勬牳蹇冧笂锛宍$x0` 鐨勪綆 128 浣嶄笌 `$v0` 鍏变韩锛宍$v0` 鐨勪綆 64 浣嶄笌 `$f0` 鍏变韩锛涘叾浠栨墍鏈?VR 涔熷悓鐞嗐€?
### 鎺у埗鐘舵€佸瘎瀛樺櫒锛圕SR锛?

CSR 鍙兘浠庣壒鏉冩ā寮忥紙PLV0锛夎闂細

================= ===================================== ==============
鍦板潃              鍏ㄧО                                  缂╁啓鍚?================= ===================================== ==============
0x0               褰撳墠妯″紡淇℃伅                           CRMD
0x1               寮傚父鍓嶆ā寮忎俊鎭?                        PRMD
0x2               鎵╁睍鍗曞厓浣胯兘                           EUEN
0x3               鏉傞」鎺у埗                               MISC
0x4               寮傚父閰嶇疆                               ECFG
0x5               寮傚父鐘舵€?                              ESTAT
0x6               寮傚父杩斿洖鍦板潃                           ERA
0x7               閿欒锛堟晠闅滐級铏氭嫙鍦板潃                   BADV
0x8               閿欒锛堟晠闅滐級鎸囦护瀛?                    BADI
0xC               寮傚父鍏ュ彛鍦板潃                           EENTRY
0x10              TLB 绱㈠紩                              TLBIDX
0x11              TLB 琛ㄩ」楂樹綅                           TLBEHI
0x12              TLB 琛ㄩ」浣庝綅 0                         TLBELO0
0x13              TLB 琛ㄩ」浣庝綅 1                         TLBELO1
0x18              鍦板潃绌洪棿鏍囪瘑绗?                        ASID
0x19              涓嬪崐鍦板潃绌洪棿鐨勯〉鍏ㄥ眬鐩綍鍦板潃           PGDL
0x1A              涓婂崐鍦板潃绌洪棿鐨勯〉鍏ㄥ眬鐩綍鍦板潃           PGDH
0x1B              椤靛叏灞€鐩綍鍦板潃                         PGD
0x1C              涓嬪崐鍦板潃绌洪棿鐨勯〉娓歌蛋鎺у埗               PWCL
0x1D              涓婂崐鍦板潃绌洪棿鐨勯〉娓歌蛋鎺у埗               PWCH
0x1E              STLB 椤靛ぇ灏?                          STLBPS
0x1F              缂╁噺铏氭嫙鍦板潃閰嶇疆                       RVACFG
0x20              CPU 鏍囪瘑绗?                           CPUID
0x21              鐗规潈璧勬簮閰嶇疆 1                         PRCFG1
0x22              鐗规潈璧勬簮閰嶇疆 2                         PRCFG2
0x23              鐗规潈璧勬簮閰嶇疆 3                         PRCFG3
0x30+n (0鈮鈮?5)   淇濆瓨鏁版嵁瀵勫瓨鍣?                        SAVEn
0x40              瀹氭椂鍣ㄦ爣璇嗙                           TID
0x41              瀹氭椂鍣ㄩ厤缃?                            TCFG
0x42              瀹氭椂鍣ㄥ€?                              TVAL
0x43              瀹氭椂鍣ㄨ鏁拌ˉ鍋?                        CNTC
0x44              瀹氭椂鍣ㄤ腑鏂竻闄?                        TICLR
0x60              LLBit 鎺у埗                            LLBCTL
0x80              瀹炵幇鐩稿叧鎺у埗 1                         IMPCTL1
0x81              瀹炵幇鐩稿叧鎺у埗 2                         IMPCTL2
0x88              TLB 閲嶅～寮傚父鍏ュ彛鍦板潃                   TLBRENTRY
0x89              TLB 閲嶅～寮傚父閿欒锛堟晠闅滐級铏氭嫙鍦板潃       TLBRBADV
0x8A              TLB 閲嶅～寮傚父杩斿洖鍦板潃                   TLBRERA
0x8B              TLB 閲嶅～寮傚父淇濆瓨鏁版嵁瀵勫瓨鍣?            TLBRSAVE
0x8C              TLB 閲嶅～寮傚父鍏ュ彛浣庝綅 0                 TLBRELO0
0x8D              TLB 閲嶅～寮傚父鍏ュ彛浣庝綅 1                 TLBRELO1
0x8E              TLB 閲嶅～寮傚父鍏ュ彛楂樹綅                   TLBEHI
0x8F              TLB 閲嶅～寮傚父寮傚父鍓嶆ā寮忎俊鎭?            TLBRPRMD
0x90              鏈哄櫒閿欒鎺у埗                           MERRCTL
0x91              鏈哄櫒閿欒淇℃伅 1                         MERRINFO1
0x92              鏈哄櫒閿欒淇℃伅 2                         MERRINFO2
0x93              鏈哄櫒閿欒寮傚父鍏ュ彛鍦板潃                   MERRENTRY
0x94              鏈哄櫒閿欒寮傚父杩斿洖鍦板潃                   MERRERA
0x95              鏈哄櫒閿欒寮傚父淇濆瓨鏁版嵁瀵勫瓨鍣?            MERRSAVE
0x98              缂撳瓨 TAG                               CTAG
0x180+n (0鈮鈮?)   鐩存帴鏄犲皠閰嶇疆绐楀彛 n                     DMWn
0x200+2n (0鈮鈮?1) 鎬ц兘鐩戣鍣ㄩ厤缃?n                       PMCFGn
0x201+2n (0鈮鈮?1) 鎬ц兘鐩戣鍣ㄦ€昏鏁板櫒 n                   PMCNTn
0x300             鍐呭瓨鍔犺浇/瀛樺偍瑙傚療鐐规€讳綋鎺у埗            MWPC
0x301             鍐呭瓨鍔犺浇/瀛樺偍瑙傚療鐐规€讳綋鐘舵€?           MWPS
0x310+8n (0鈮鈮?)  鍐呭瓨鍔犺浇/瀛樺偍瑙傚療鐐?n 閰嶇疆 1           MWPnCFG1
0x311+8n (0鈮鈮?)  鍐呭瓨鍔犺浇/瀛樺偍瑙傚療鐐?n 閰嶇疆 2           MWPnCFG2
0x312+8n (0鈮鈮?)  鍐呭瓨鍔犺浇/瀛樺偍瑙傚療鐐?n 閰嶇疆 3           MWPnCFG3
0x313+8n (0鈮鈮?)  鍐呭瓨鍔犺浇/瀛樺偍瑙傚療鐐?n 閰嶇疆 4           MWPnCFG4
0x380             鎸囦护鑾峰彇瑙傚療鐐规€讳綋鎺у埗                 FWPC
0x381             鎸囦护鑾峰彇瑙傚療鐐规€讳綋鐘舵€?                FWPS
0x390+8n (0鈮鈮?)  鎸囦护鑾峰彇瑙傚療鐐?n 閰嶇疆 1                FWPnCFG1
0x391+8n (0鈮鈮?)  鎸囦护鑾峰彇瑙傚療鐐?n 閰嶇疆 2                FWPnCFG2
0x392+8n (0鈮鈮?)  鎸囦护鑾峰彇瑙傚療鐐?n 閰嶇疆 3                FWPnCFG3
0x393+8n (0鈮鈮?)  鎸囦护鑾峰彇瑙傚療鐐?n 閰嶇疆 4                FWPnCFG4
0x500             璋冭瘯瀵勫瓨鍣?                            DBG
0x501             璋冭瘯寮傚父杩斿洖鍦板潃                       DERA
0x502             璋冭瘯寮傚父淇濆瓨鏁版嵁瀵勫瓨鍣?                DSAVE
================= ===================================== ==============

ERA銆乀LBRERA銆丮ERRERA 鍜?DERA 鏈夋椂涔熷垎鍒О涓?EPC銆乀LBREPC銆丮ERREPC 鍜?DEPC銆?
## 鍩烘湰鎸囦护闆?

### 鎸囦护鏍煎紡


LoongArch 鎸囦护涓?32 浣嶅锛屽睘浜?9 绉嶅熀鏈寚浠ゆ牸寮忥紙鍙婂叾鍙樹綋锛夛細

=========== ==========================
鏍煎紡鍚?     缁勬垚
=========== ==========================
2R          Opcode + Rj + Rd
3R          Opcode + Rk + Rj + Rd
4R          Opcode + Ra + Rk + Rj + Rd
2RI8        Opcode + I8 + Rj + Rd
2RI12       Opcode + I12 + Rj + Rd
2RI14       Opcode + I14 + Rj + Rd
2RI16       Opcode + I16 + Rj + Rd
1RI21       Opcode + I21L + Rj + I21H
I26         Opcode + I26L + I26H
=========== ==========================

Rd 鏄洰鏍囧瘎瀛樺櫒鎿嶄綔鏁帮紝鑰?Rj銆丷k 鍜?Ra锛?a" 琛ㄧず "additional"锛岄澶栫殑锛夋槸婧愬瘎瀛樺櫒鎿嶄綔鏁般€侷8/I12/I14/I16/I21/I26 鏄浉搴斿搴︾殑绔嬪嵆鏁版搷浣滄暟銆傝緝闀跨殑 I21 鍜?I26 鍦ㄦ寚浠ゅ瓧涓垎鍒瓨鍌ㄥ湪楂樹綅鍜屼綆浣嶉儴鍒嗭紝浠?"L" 鍜?"H" 鍚庣紑琛ㄧず銆?
### 鎸囦护鍒楄〃


涓虹畝娲佽捣瑙侊紝姝ゅ浠呭垪鍑烘寚浠ゅ悕绉帮紙鍔╄绗︼級锛涜鎯呰鍙傞槄鍙傝€冭祫鏂?<loongarch-references>銆?
```
    ADD.W SUB.W ADDI.W ADD.D SUB.D ADDI.D
    SLT SLTU SLTI SLTUI
    AND OR NOR XOR ANDN ORN ANDI ORI XORI
    MUL.W MULH.W MULH.WU DIV.W DIV.WU MOD.W MOD.WU
    MUL.D MULH.D MULH.DU DIV.D DIV.DU MOD.D MOD.DU
    PCADDI PCADDU12I PCADDU18I
    LU12I.W LU32I.D LU52I.D ADDU16I.D
```

```
    SLL.W SRL.W SRA.W ROTR.W SLLI.W SRLI.W SRAI.W ROTRI.W
    SLL.D SRL.D SRA.D ROTR.D SLLI.D SRLI.D SRAI.D ROTRI.D
```

```
    EXT.W.B EXT.W.H CLO.W CLO.D SLZ.W CLZ.D CTO.W CTO.D CTZ.W CTZ.D
    BYTEPICK.W BYTEPICK.D BSTRINS.W BSTRINS.D BSTRPICK.W BSTRPICK.D
    REVB.2H REVB.4H REVB.2W REVB.D REVH.2W REVH.D BITREV.4B BITREV.8B BITREV.W BITREV.D
    MASKEQZ MASKNEZ
```

```
    BEQ BNE BLT BGE BLTU BGEU BEQZ BNEZ B BL JIRL
```

```
    LD.B LD.BU LD.H LD.HU LD.W LD.WU LD.D ST.B ST.H ST.W ST.D
    LDX.B LDX.BU LDX.H LDX.HU LDX.W LDX.WU LDX.D STX.B STX.H STX.W STX.D
    LDPTR.W LDPTR.D STPTR.W STPTR.D
    PRELD PRELDX
```

```
    LL.W SC.W LL.D SC.D
    AMSWAP.W AMSWAP.D AMADD.W AMADD.D AMAND.W AMAND.D AMOR.W AMOR.D AMXOR.W AMXOR.D
    AMMAX.W AMMAX.D AMMIN.W AMMIN.D
```

```
    IBAR DBAR
```

```
    SYSCALL BREAK CPUCFG NOP IDLE ERTN(ERET) DBCL(DBGCALL) RDTIMEL.W RDTIMEH.W RDTIME.D
    ASRTLE.D ASRTGT.D
```

```
    CSRRD CSRWR CSRXCHG
    IOCSRRD.B IOCSRRD.H IOCSRRD.W IOCSRRD.D IOCSRWR.B IOCSRWR.H IOCSRWR.W IOCSRWR.D
    CACOP TLBP(TLBSRCH) TLBRD TLBWR TLBFILL TLBCLR TLBFLUSH INVTLB LDDIR LDPTE
```

## 铏氭嫙鍐呭瓨


LoongArch 鏀寔鐩存帴鏄犲皠鐨勮櫄鎷熷唴瀛樺拰椤垫槧灏勭殑铏氭嫙鍐呭瓨銆?
鐩存帴鏄犲皠鐨勮櫄鎷熷唴瀛樼敱 CSR.DMWn锛坣=0~3锛夐厤缃紝瀹冩湁涓€涓畝鍗曠殑鍏崇郴

```
 VA = PA + FixedOffset
```

椤垫槧灏勭殑铏氭嫙鍐呭瓨涓?VA 涓?PA 涔嬮棿鏄换鎰忓叧绯伙紝璁板綍鍦?TLB 鍜岄〉琛ㄤ腑銆侺oongArch 鐨?TLB 鍖呭惈涓€涓叏鐩歌仈鐨?MTLB锛堝椤靛ぇ灏?TLB锛夊拰涓€涓粍鐩歌仈鐨?STLB锛堝崟椤靛ぇ灏?TLB锛夈€?
榛樿鎯呭喌涓嬶紝LA32 鐨勬暣涓櫄鎷熷湴鍧€绌洪棿閰嶇疆濡備笅锛?
============ =========================== =============================
鍚嶇О         鍦板潃鑼冨洿                    灞炴€?============ =========================== =============================
`UVRANGE`  `0x00000000 - 0x7FFFFFFF` 椤垫槧灏勶紝缂撳瓨锛孭LV0~3
`KPRANGE0` `0x80000000 - 0x9FFFFFFF` 鐩存帴鏄犲皠锛岄潪缂撳瓨锛孭LV0
`KPRANGE1` `0xA0000000 - 0xBFFFFFFF` 鐩存帴鏄犲皠锛岀紦瀛橈紝PLV0
`KVRANGE`  `0xC0000000 - 0xFFFFFFFF` 椤垫槧灏勶紝缂撳瓨锛孭LV0
============ =========================== =============================

鐢ㄦ埛妯″紡锛圥LV3锛夊彧鑳借闂?UVRANGE銆傚浜庣洿鎺ユ槧灏勭殑 KPRANGE0 鍜?KPRANGE1锛孭A 绛変簬娓呴櫎浜?bit30~31 鐨?VA銆備緥濡傦紝0x00001000 鐨勯潪缂撳瓨鐩存帴鏄犲皠 VA 涓?0x80001000锛岃€?0x00001000 鐨勭紦瀛樼洿鎺ユ槧灏?VA 涓?0xA0001000銆?
榛樿鎯呭喌涓嬶紝LA64 鐨勬暣涓櫄鎷熷湴鍧€绌洪棿閰嶇疆濡備笅锛?
============ ====================== ======================================
鍚嶇О         鍦板潃鑼冨洿                灞炴€?============ ====================== ======================================
`XUVRANGE` ``0x0000000000000000 - 椤垫槧灏勶紝缂撳瓨锛孭LV0~3
             0x3FFFFFFFFFFFFFFF``
`XSPRANGE` ``0x4000000000000000 - 鐩存帴鏄犲皠锛岀紦瀛?闈炵紦瀛橈紝PLV0
             0x7FFFFFFFFFFFFFFF``
`XKPRANGE` ``0x8000000000000000 - 鐩存帴鏄犲皠锛岀紦瀛?闈炵紦瀛橈紝PLV0
             0xBFFFFFFFFFFFFFFF``
`XKVRANGE` ``0xC000000000000000 - 椤垫槧灏勶紝缂撳瓨锛孭LV0
             0xFFFFFFFFFFFFFFFF``
============ ====================== ======================================

鐢ㄦ埛妯″紡锛圥LV3锛夊彧鑳借闂?XUVRANGE銆傚浜庣洿鎺ユ槧灏勭殑 XSPRANGE 鍜?XKPRANGE锛孭A 绛変簬娓呴櫎浜?bit 60~63 鐨?VA锛岀紦瀛樺睘鎬х敱 VA 涓殑 bit 60~61 閰嶇疆锛? 琛ㄧず寮哄簭闈炵紦瀛橈紝1 琛ㄧず涓€鑷存€х紦瀛橈紝2 琛ㄧず寮卞簭闈炵紦瀛樸€?
鐩墠鎴戜滑浠呬娇鐢?XKPRANGE 杩涜鐩存帴鏄犲皠锛孹SPRANGE 淇濈暀銆?
涓句緥璇存槑锛?x00000000_00001000 鐨勫己搴忛潪缂撳瓨鐩存帴鏄犲皠 VA锛堜綅浜?XKPRANGE锛変负 0x80000000_00001000锛屽叾涓€鑷存€х紦瀛樼洿鎺ユ槧灏?VA锛堜綅浜?XKPRANGE锛変负 0x90000000_00001000锛屽叾寮卞簭闈炵紦瀛樼洿鎺ユ槧灏?VA锛堜綅浜?XKPRANGE锛変负 0xA0000000_00001000銆?
## Loongson 涓?LoongArch 鐨勫叧绯?

LoongArch 鏄竴绉嶄笉鍚屼簬浠讳綍鍏朵粬鐜版湁鏋舵瀯鐨?RISC ISA锛岃€?Loongson 鏄竴涓鐞嗗櫒绯诲垪銆侺oongson 鍖呭惈 3 涓郴鍒楋細Loongson-1 鏄?32 浣嶅鐞嗗櫒绯诲垪锛孡oongson-2 鏄綆绔?64 浣嶅鐞嗗櫒绯诲垪锛孡oongson-3 鏄珮绔?64 浣嶅鐞嗗櫒绯诲垪銆傛棫娆?Loongson 鍩轰簬 MIPS锛岃€屾柊娆?Loongson 鍩轰簬 LoongArch銆備互 Loongson-3 涓轰緥锛歀oongson-3A1000/3B1500/3A2000/3A3000/3A4000 鍏煎 MIPS锛岃€?Loongson-3A5000锛堝強鍚庣画淇鐗堬級鍏ㄩ儴鍩轰簬 LoongArch銆?
## 鍙傝€冭祫鏂?

榫欒姱涓瀹樻柟缃戠珯锛?
  http://www.loongson.cn/

榫欒姱涓?LoongArch 寮€鍙戣€呯綉绔欙紙杞欢涓庢枃妗ｏ級锛?
  http://www.loongnix.cn/

  https://github.com/loongson/

  https://loongson.github.io/LoongArch-Documentation/

LoongArch ISA 鏂囨。锛?
  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-Vol1-v1.10-CN.pdf 锛堜腑鏂囷級

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-Vol1-v1.10-EN.pdf 锛堣嫳鏂囷級

LoongArch ELF psABI 鏂囨。锛?
  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-ELF-ABI-v2.01-CN.pdf 锛堜腑鏂囷級

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/LoongArch-ELF-ABI-v2.01-EN.pdf 锛堣嫳鏂囷級

榫欒姱涓?LoongArch 鐨?Linux 鍐呮牳浠撳簱锛?
  https://git.kernel.org/pub/scm/linux/kernel/git/chenhuacai/linux-loongson.git
