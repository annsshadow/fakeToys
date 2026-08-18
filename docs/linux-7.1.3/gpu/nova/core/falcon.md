
## Falcon (FAst Logic Controller)

浠ヤ笅鍚勮妭鎻忚堪 Falcon 鏍稿績鍙婂叾涓婅繍琛岀殑寰爜锛坲code锛夈€傝繖浜涙弿杩板熀浜?Ampere GPU 鎴栨洿鏃╃殑璁捐锛涗笉杩囧畠浠ぇ浣撲笂涔熼€傜敤浜庢湭鏉ョ殑璁捐锛屼絾涓€鍒囧潎鍙兘鍙樺姩銆傛澶勬彁渚涚殑姒傝堪涓昏鏃ㄥ湪甯姪鐞嗚В nova-core 椹卞姩涓?Falcon 鐨勪氦浜掋€?
NVIDIA GPU 鍐呭祵浜嗙О涓?Falcon 鏍稿績鐨勫皬鍨嬬被 RISC 寰帶鍒跺櫒锛岃礋璐ｅ鐞嗗畨鍏ㄥ浐浠朵换鍔°€佸垵濮嬪寲鍜岀數婧愮鐞嗐€傜幇浠?NVIDIA GPU 鍙兘鎷ユ湁澶氫釜杩欐牱鐨?Falcon 瀹炰緥锛堜緥濡?GSP锛圙PU 绯荤粺澶勭悊鍣級鍜?SEC2锛堝畨鍏ㄥ紩鎿庯級锛夛紝骞朵笖涔熷彲鑳介泦鎴愪竴涓?RISC-V 鏍稿績銆傝鏍稿績鏃㈣兘杩愯 RISC-V 浠ｇ爜锛屼篃鑳借繍琛?Falcon 浠ｇ爜銆?
杩愯鍦?Falcon 鏍稿績涓婄殑浠ｇ爜涔熺О涓?'ucode'锛堝井鐮侊級锛屽悗缁珷鑺傚皢娌跨敤姝ょО鍛笺€?
Falcon 鎷ユ湁鐙珛鐨勬寚浠や笌鏁版嵁瀛樺偍鍣紙IMEM/DMEM锛夛紝骞舵彁渚涘皬鍨?DMA 寮曟搸锛堢粡鐢?FBIF鈥斺€?甯х紦鍐叉帴鍙?锛孎rame Buffer Interface锛変粠绯荤粺鍐呭瓨鍔犺浇浠ｇ爜銆俷ova-core 椹卞姩蹇呴』澶嶄綅骞堕厤缃?Falcon锛岄€氳繃 DMA 鍔犺浇鍏跺浐浠讹紝骞跺惎鍔ㄥ叾 CPU銆?
## Falcon 瀹夊叏绾у埆

Falcon 鍙互杩愯鍦ㄩ潪瀹夊叏锛圢S锛夈€佽交瀹夊叏锛圠S锛夋垨閲嶅畨鍏紙HS锛夋ā寮忎腑銆?
### 閲嶅畨鍏紙HS锛夛紝涔熺О鐗规潈绾?3锛圥L3锛?
HS 寰爜鏄渶鍙椾俊浠荤殑浠ｇ爜锛屽嚑涔庡彲浠ヨ闂姱鐗囦笂鐨勪竴鍒囥€侶S 浜岃繘鍒舵枃浠跺唴鍖呭惈涓€涓湪鍚姩鏃堕獙璇佺殑绛惧悕銆傝绛惧悕楠岃瘉鐢辩‖浠惰嚜韬畬鎴愶紝浠庤€屽缓绔嬩俊浠绘牴锛坮oot of trust锛夈€備緥濡傦紝FWSEC-FRTS 鍛戒护锛堣 fwsec.rst锛夊湪 HS 妯″紡涓嬬殑 GSP 涓婅繍琛屻€侳RTS 娑夊強寤虹珛骞跺悜 WPR锛堝啓淇濇姢鍖哄煙锛學rite Protect Region锛夊姞杞藉唴瀹癸紝蹇呴』鐢?HS 寰爜瀹屾垚锛屼富鏈?CPU 鎴?LS 寰爜閮芥棤娉曞畬鎴愩€?
### 杞诲畨鍏紙LS 鎴?PL2锛変笌闈炲畨鍏紙NS 鎴?PL0锛?
杩欎簺妯″紡鐨勫畨鍏ㄦ€т綆浜?HS銆備笌 HS 绫讳技锛孡S 鎴?NS 寰爜浜岃繘鍒舵枃浠堕€氬父涔熷寘鍚竴涓鍚嶃€傝鍚?Falcon 鍔犺浇 LS 鎴?NS 妯″紡鐨勫浐浠讹紝闇€瑕佸彟涓€涓?Falcon 杩愯鍦?HS 妯″紡涓嬶紝杩欎篃寤虹珛浜嗕俊浠绘牴銆備緥濡傦紝鍦?Ampere GPU 涓婏紝CPU 鍦?SEC2 Falcon 涓婁互 HS 妯″紡杩愯 "Booter" 寰爜锛岄殢鍚庡畠瀵硅繍琛屾椂鐨?GSP 浜岃繘鍒舵枃浠讹紙GSP-RM锛夎繘琛岃璇侊紝骞朵互 LS 妯″紡鍦?GSP Falcon 涓婅繍琛屽畠銆傜被浼肩殑渚嬪瓙鏄細Ampere 澶嶄綅鍚庯紝FWSEC 鍦?GSP 涓婅繍琛岋紝闅忓悗灏?devinit 寮曟搸浠?LS 妯″紡鍔犺浇鍒?PMU 涓娿€?
### 淇′换鏍圭殑寤虹珛

瑕佸缓绔嬩俊浠绘牴锛岃繍琛屽湪 Falcon 涓婄殑浠ｇ爜蹇呴』鏄笉鍙彉鐨勶紝骞剁‖杩炵嚎鍒板彧璇诲瓨鍌ㄥ櫒锛圧OM锛変腑銆傝繖绗﹀悎涓氱晫鍥轰欢楠岃瘉鐨勮鑼冦€傝繖娈典唬鐮佺О涓哄紩瀵?ROM锛圔oot ROM锛孊ROM锛夈€侰PU 涓婄殑 nova-core 椹卞姩閫氳繃澶氫釜浠?"BROM" 涓哄墠缂€鐨?Falcon 瀵勫瓨鍣ㄤ笌 Falcon 鐨?Boot ROM 閫氫俊锛堣 regs.rs锛夈€?
nova-core 椹卞姩浠?VBIOS 璇诲彇蹇呰鐨勫井鐮佸悗锛屼細瀵?BROM 鍜?DMA 瀵勫瓨鍣ㄨ繘琛岀紪绋嬶紝浠ヨЕ鍙?Falcon 灏?HS 寰爜浠庣郴缁熷唴瀛樺姞杞藉埌 Falcon 鐨?IMEM/DMEM 涓€侶S 寰爜鍔犺浇瀹屾垚鍚庯紝浼氱敱 Falcon 鐨?Boot ROM 楠岃瘉銆?
涓€鏃︾粡杩囬獙璇佺殑 HS 浠ｇ爜鍦?Falcon 涓婅繍琛岋紝瀹冨氨鍙互楠岃瘉骞跺皢鍏朵粬 LS/NS 寰爜浜岃繘鍒舵枃浠跺姞杞藉埌鍏朵粬 Falcon 涓婂苟鍚姩瀹冧滑銆傜鍚嶉獙璇佺殑杩囩▼涓?HS 鐩稿悓锛涘彧鏄繖绉嶆儏鍐典笅锛岃绠楃鍚嶇殑涓嶆槸纭欢锛圔ROM锛夛紝鑰屾槸 HS 寰爜銆?
鍥犳锛屼俊浠绘牴鐨勫缓绔嬭繃绋嬪涓嬶細
     Hardware (Boot ROM running on the Falcon) -> HS ucode -> LS/NS ucode.

渚嬪锛屽湪 Ampere GPU 涓婏紝鍚姩楠岃瘉娴佺▼涓猴細
     Hardware (Boot ROM running on the SEC2) ->
          HS ucode (Booter running on the SEC2) ->
               LS ucode (GSP-RM running on the GSP)

     铏界劧 CPU 鍙互灏?HS 寰爜鍔犺浇鍒?Falcon 寰帶鍒跺櫒涓婏紝骞惰瀹冪敱纭欢楠岃瘉鍚庤繍琛岋紝浣?CPU 鏈韩閫氬父涓嶄細鍘诲姞杞?LS 鎴?NS 寰爜骞惰繍琛屽畠銆侺S 鎴?NS 寰爜鐨勫姞杞戒富瑕佺敱 HS 寰爜瀹屾垚銆備緥濡傦紝鍦?Ampere GPU 涓婏紝褰?Booter 寰爜鍦?SEC2 涓婁互 HS 妯″紡杩愯骞跺皢 GSP-RM 浜岃繘鍒舵枃浠跺姞杞藉埌 GSP 涓婁箣鍚庯紝瀹冨湪杩愯鏃惰繕闇€瑕佽繍琛?"SEC2-RTOS" 寰爜銆傝繖灏卞甫鏉ヤ簡涓€涓棶棰橈細娌℃湁浠讳綍缁勪欢鑳芥妸 SEC2-RTOS 寰爜鍔犺浇鍒?SEC2 涓娿€侰PU 鏃犳硶鍔犺浇 LS 浠ｇ爜锛岃€?GSP-RM 鍙堝繀椤诲湪 LS 妯″紡涓嬭繍琛屻€備负鍏嬫湇杩欎竴鐐癸紝GSP 琚复鏃惰缃负杩愯 HS 寰爜锛堣寰爜鏈韩鐢?CPU 缁忕敱 nova-core 椹卞姩浣跨敤涓€涓?"GSP 鎻愪緵鐨勫畾搴忓櫒" 鍔犺浇锛夛紝鐢卞畠鍐嶄互 LS 妯″紡鎶?SEC2-RTOS 寰爜鍔犺浇鍒?SEC2 涓娿€傞殢鍚?GSP 鎭㈠杩愯瀹冭嚜韬殑 GSP-RM LS 寰爜銆?
## Falcon 瀛樺偍鍣ㄥ瓙绯荤粺涓?DMA 寮曟搸

Falcon 鎷ユ湁鐙珛鐨勬寚浠や笌鏁版嵁瀛樺偍鍣紙IMEM/DMEM锛夛紝骞跺寘鍚竴涓О涓?FBDMA锛堝抚缂撳啿 DMA锛孎ramebuffer DMA锛夌殑灏忓瀷 DMA 寮曟搸锛屽畠缁忕敱 FBIF锛堝抚缂撳啿鎺ュ彛锛孎ramebuffer Interface锛夊湪 Falcon 鍐呴儴鐨?IMEM/DMEM 瀛樺偍鍣ㄤ笌澶栭儴鍐呭瓨涔嬮棿鎵ц DMA 浼犺緭銆?
DMA 浼犺緭鍙互浠?Falcon 鐨勫瓨鍌ㄥ櫒鍙戝線绯荤粺鍐呭瓨鍜屽抚缂撳啿鍐呭瓨锛圴RAM锛夈€?
瑕侀€氳繃 FBDMA 鎵ц DMA锛岄渶瑕佸 FBIF 杩涜閰嶇疆锛屼互鍐冲畾鍐呭瓨濡備綍琚闂紙涔熺О涓?aperture 绫诲瀷锛夈€傚湪 nova-core 椹卞姩涓紝杩欑敱 `FalconFbifTarget` 鏋氫妇鍐冲畾銆?
Falcon 涓殑 IO-PMP 鍧楋紙杈撳叆杈撳嚭鐗╃悊鍐呭瓨淇濇姢锛孖nput/Output Physical Memory Protection锛夊崟鍏冩帶鍒剁潃 FBDMA 瀵瑰閮ㄥ唴瀛樼殑璁块棶銆?
```

               External Memory (Framebuffer / System DRAM)
                              ^  |
                              |  |
                              |  v
     +-----------------------------------------------------+
     |                           |                         |
     |   +---------------+       |                         |
     |   |     FBIF      |-------+                         |  FALCON
     |   | (FrameBuffer  |   Memory Interface              |  PROCESSOR
     |   |  InterFace)   |                                 |
     |   |  Apertures    |                                 |
     |   |  Configures   |                                 |
     |   |  mem access   |                                 |
     |   +-------^-------+                                 |
     |           |                                         |
     |           | FBDMA uses configured FBIF apertures    |
     |           | to access External Memory
     |           |
     |   +-------v--------+      +---------------+
     |   |    FBDMA       |  cfg |     RISC      |
     |   | (FrameBuffer   |<---->|     CORE      |----->. Direct Core Access
     |   |  DMA Engine)   |      |               |      |
     |   | - Master dev.  |      | (can run both |      |
     |   +-------^--------+      | Falcon and    |      |
     |           |        cfg--->| RISC-V code)  |      |
     |           |        /      |               |      |
     |           |        |      +---------------+      |    +------------+
     |           |        |                             |    |   BROM     |
     |           |        |                             <--->| (Boot ROM) |
     |           |       /                              |    +------------+
     |           |      v                               |
     |   +---------------+                              |
     |   |    IO-PMP     | Controls access by FBDMA     |
     |   | (IO Physical  | and other IO Masters         |
     |   | Memory Protect)                              |
     |   +-------^-------+                              |
     |           |                                      |
     |           | Protected Access Path for FBDMA      |
     |           v                                      |
     |   +---------------------------------------+      |
     |   |       Memory                          |      |
     |   |   +---------------+  +------------+   |      |
     |   |   |    IMEM       |  |    DMEM    |   |<-----+
     |   |   | (Instruction  |  |   (Data    |   |
     |   |   |  Memory)      |  |   Memory)  |   |
     |   |   +---------------+  +------------+   |
     |   +---------------------------------------+
     +-----------------------------------------------------+

```
