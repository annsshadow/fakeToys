
## DEXCR锛圖ynamic Execution Control Register锛屽姩鎬佹墽琛屾帶鍒跺瘎瀛樺櫒锛?

## 姒傝堪


DEXCR 鏄?PowerPC ISA 3.1B锛圥ower10锛夊紩鍏ョ殑涓€涓壒鏉冪壒娈婄敤閫斿瘎瀛樺櫒锛圫PR锛夛紝鍏佽
瀵规瘡涓?CPU 鐨勮嫢骞插姩鎬佹墽琛岃涓鸿繘琛屾帶鍒躲€傝繖浜涜涓哄寘鎷帹娴嬫墽琛岋紙渚嬪闂存帴鍒嗘敮鐩爣
棰勬祴锛変互鍙婂惎鐢ㄩ潰鍚戣繑鍥炵紪绋嬶紙ROP锛夌殑淇濇姢鎸囦护銆?
鎵ц鎺у埗鍦ㄧ‖浠朵腑琛ㄧ幇涓?DEXCR 涓渶澶?32 浣嶏紙鈥渁spects鈥濓紝鏂归潰锛夈€傛瘡涓?aspect 鎺у埗
鏌愮琛屼负锛屽彲浠ョ疆浣嶆垨娓呴櫎浠ュ惎鐢?绂佺敤璇?aspect銆侱EXCR 鏈夊嚑涓敤浜庝笉鍚岀洰鐨勭殑鍙樹綋锛?
DEXCR
    涓€涓壒鏉?SPR锛屽彲鎺у埗鐢ㄦ埛绌洪棿鍜屽唴鏍哥┖闂寸殑 aspects
HDEXCR
    涓€涓秴绠＄壒鏉冿紙hypervisor-privileged锛塖PR锛屽彲鎺у埗瓒呯鐨?aspects锛屽苟瀵瑰唴鏍稿拰
    鐢ㄦ埛绌洪棿寮哄埗鏌愪簺 aspects銆?UDEXCR
    涓€涓彲閫夌殑 ultravisor 鐗规潈 SPR锛屽彲鎺у埗 ultravisor 鐨?aspects銆?
鐢ㄦ埛绌洪棿鍙互浣跨敤涓€涓笓鐢?SPR 鏉ユ鏌ュ綋鍓?DEXCR 鐘舵€侊紝璇?SPR 鎻愪緵鐢ㄦ埛绌洪棿 DEXCR
aspects 鐨勯潪鐗规潈鍙瑙嗗浘銆傝繕鏈変竴涓?SPR 鎻愪緵瓒呯寮哄埗 aspects 鐨勫彧璇昏鍥撅紝瀹冧笌
鐢ㄦ埛绌洪棿 DEXCR 瑙嗗浘鐩糕€滄垨鈥濓紝鍗冲緱鍒拌繘绋嬬殑鏈夋晥 DEXCR 鐘舵€併€?

## 閰嶇疆


### prctl


涓€涓繘绋嬪彲浠ヤ娇鐢?`PR_PPC_GET_DEXCR` 涓?`PR_PPC_SET_DEXCR` 杩欏
```

    prctl(PR_PPC_GET_DEXCR, unsigned long which, 0, 0, 0);
    prctl(PR_PPC_SET_DEXCR, unsigned long which, unsigned long ctrl, 0, 0);

```
鍙兘鐨勨€渨hich鈥濅笌鈥渃trl鈥濆€煎涓嬨€傛敞鎰忊€渨hich鈥濆€间笌 DEXCR aspect 鐨勭储寮曚箣闂存病鏈?鍏崇郴銆?
   :header-rows: 1
   :widths: 2 7 1

   - - `prctl()` which
     - Aspect name
     - Aspect index

   - - `PR_PPC_DEXCR_SBHE`
     - Speculative Branch Hint Enable (SBHE)
     - 0

   - - `PR_PPC_DEXCR_IBRTPD`
     - Indirect Branch Recurrent Target Prediction Disable (IBRTPD)
     - 3

   - - `PR_PPC_DEXCR_SRAPD`
     - Subroutine Return Address Prediction Disable (SRAPD)
     - 4

   - - `PR_PPC_DEXCR_NPHIE`
     - Non-Privileged Hash Instruction Enable (NPHIE)
     - 5

   :header-rows: 1
   :widths: 2 8

   - - `prctl()` ctrl
     - Meaning

   - - `PR_PPC_DEXCR_CTRL_EDITABLE`
     - 璇?aspect 鍙€氳繃 PR_PPC_SET_DEXCR 閰嶇疆锛堜粎鐢ㄤ簬鑾峰彇锛?
   - - `PR_PPC_DEXCR_CTRL_SET`
     - 璇?aspect 宸茬疆浣?/ 缃綅璇?aspect

   - - `PR_PPC_DEXCR_CTRL_CLEAR`
     - 璇?aspect 宸叉竻闄?/ 娓呴櫎璇?aspect

   - - `PR_PPC_DEXCR_CTRL_SET_ONEXEC`
     - 璇?aspect 灏嗗湪 exec 涔嬪悗缃綅 / exec 涔嬪悗缃綅璇?aspect

   - - `PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC`
     - 璇?aspect 灏嗗湪 exec 涔嬪悗娓呴櫎 / exec 涔嬪悗娓呴櫎璇?aspect

娉ㄦ剰

- which 鏄竴涓櫘閫氬€硷紝鑰岄潪浣嶆帺鐮併€俛spects 蹇呴』閫愪釜澶勭悊銆?
- ctrl 鏄竴涓綅鎺╃爜銆俙PR_PPC_GET_DEXCR` 杩斿洖褰撳墠閰嶇疆鍜?onexec 閰嶇疆銆備緥濡傦紝
  `PR_PPC_GET_DEXCR` 鍙兘杩斿洖
  ``PR_PPC_DEXCR_CTRL_EDITABLE | PR_PPC_DEXCR_CTRL_SET |
  PR_PPC_DEXCR_CTRL_CLEAR_ONEXEC``銆傝繖琛ㄧず璇?aspect 褰撳墠宸茬疆浣嶏紝鍦ㄨ繍琛?exec 鏃跺皢
  琚竻闄わ紝骞朵笖浣犲彲浠ヤ娇鐢?`PR_PPC_SET_DEXCR` prctl 鏇存敼瀹冦€?
- set/clear 鏈鎸囩殑鏄湪 DEXCR 涓疆浣?娓呴櫎璇ヤ綅銆?```

      prctl(PR_PPC_SET_DEXCR, PR_PPC_DEXCR_IBRTPD, PR_PPC_DEXCR_CTRL_SET, 0, 0);

  灏嗙疆浣?DEXCR 涓殑 IBRTPD aspect 浣嶏紝浠庤€屽鑷撮棿鎺ュ垎鏀娴嬭绂佺敤銆?
```
- `PR_PPC_GET_DEXCR` 杩斿洖鐨勭姸鎬佽〃绀鸿繘绋嬪笇鏈涘簲鐢ㄧ殑鍊笺€傚畠涓嶅寘鍚换浣曟浛浠ｈ鐩栵紝渚嬪
  瓒呯姝ｅ己鍒惰 aspect 缃綅銆傝鏌ョ湅鐪熷疄鐨?DEXCR 鐘舵€侊紝杞欢搴旂洿鎺ヨ鍙栫浉搴旂殑 SPR銆?
- 杩涚▼鍚姩鏃剁殑 aspect 鐘舵€佸湪 `fork(2)` 鏃朵粠鐖惰繘绋嬬姸鎬佸鍒躲€傝鐘舵€佸湪 `execve(2)`
  鏃堕噸缃负涓€涓浐瀹氬€笺€俙PR_PPC_SET_DEXCR` prctl() 鍙互鎺у埗杩欎袱涓€笺€?
- `*_ONEXEC` 鎺у埗椤逛笉浼氭敼鍙樺綋鍓嶈繘绋嬬殑 DEXCR銆?
浣跨敤 `PR_PPC_SET_DEXCR` 骞堕厤鍚?`PR_PPC_DEXCR_CTRL_SET` 鎴?`PR_PPC_DEXCR_CTRL_CLEAR` 涔嬩竴鏉ョ紪杈戞煇涓?aspect銆?
鑾峰彇鍜岃缃?DEXCR 鐨勫父瑙侀敊璇爜濡備笅锛?
   :header-rows: 1
   :widths: 2 8

   - - Error
     - Meaning

   - - `EINVAL`
     - 鍐呮牳涓嶆敮鎸?DEXCR銆?
   - - `ENODEV`
     - 璇?aspect 鍐呮牳鏃犳硶璇嗗埆锛屾垨纭欢涓嶆敮鎸併€?
`PR_PPC_SET_DEXCR` 杩樺彲鑳芥姤鍛婁互涓嬮敊璇爜锛?
   :header-rows: 1
   :widths: 2 8

   - - Error
     - Meaning

   - - `EINVAL`
     - ctrl 鍊煎寘鍚棤娉曡瘑鍒殑鏍囧織銆?
   - - `EINVAL`
     - ctrl 鍊煎寘鍚浉浜掑啿绐佺殑鏍囧織锛堜緥濡?`PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_CLEAR`锛?
   - - `EPERM`
     - 璇?aspect 鏃犳硶閫氳繃 prctl() 淇敼锛堢敤 PR_PPC_GET_DEXCR 妫€鏌?       PR_PPC_DEXCR_CTRL_EDITABLE 鏍囧織锛夈€?
   - - `EPERM`
     - 杩涚▼娌℃湁瓒冲鐨勬潈闄愭墽琛岃鎿嶄綔銆備緥濡傦紝鍦?exec 鏃舵竻闄?NPHIE 鏄壒鏉冩搷浣滐紙杩涚▼
       浠嶅彲鍦ㄦ棤鐗规潈鐨勬儏鍐典笅娓呴櫎鑷韩鐨?NPHIE aspect锛夈€?
璇ユ帴鍙ｅ厑璁镐竴涓繘绋嬫帶鍒跺叾鑷韩鐨?DEXCR aspects锛屽苟璁剧疆鍏惰繘绋嬫爲涓换浣曞瓙杩涚▼鐨勫垵濮?DEXCR 鍊硷紙鐩村埌涓嬩竴涓娇鐢?`*_ONEXEC` 鎺у埗鐨勫瓙杩涚▼锛夈€傝繖鍏佽瀵?DEXCR 鐨勯粯璁ゅ€艰繘琛?缁嗙矑搴︽帶鍒讹紝渚嬪鍏佽瀹瑰櫒浠ヤ笉鍚岀殑榛樿鍊艰繍琛屻€?

## coredump 涓?ptrace


DEXCR 涓?HDEXCR 鐨勭敤鎴风┖闂村€硷紙鎸夋椤哄簭锛夐€氳繃 `NT_PPC_DEXCR` 鏆撮湶銆傚畠浠悇鑷负 64
浣嶄笖鍙锛岀敤浜庤緟鍔╂牳蹇冭浆鍌紙core dump锛夈€侱EXCR 鏈潵鍙兘鍙樹负鍙啓銆備袱涓瘎瀛樺櫒鐨?楂?32 浣嶏紙瀵瑰簲浜庨潪鐢ㄦ埛绌洪棿浣嶏級琚睆钄芥帀銆?
濡傛灉鍐呮牳閰嶇疆 `CONFIG_CHECKPOINT_RESTORE` 琚惎鐢紝閭ｄ箞 `NT_PPC_HASHKEYR` 鍙敤锛?骞舵毚闇茶繘绋嬬殑 HASHKEYR 鍊间緵璇诲啓銆傝繖鏄湪澧炲己瀹夊叏鎬т笌妫€鏌ョ偣/鎭㈠鏀寔涔嬮棿鐨勬潈琛★細杩涚▼
閫氬父鏃犻渶鐭ラ亾鍏跺瘑閽ワ紝浣嗘仮澶嶄竴涓繘绋嬮渶瑕佽缃叾鍘熷瀵嗛挜銆傚洜姝よ瀵嗛挜浼氬嚭鐜板湪鏍稿績杞偍
涓紝鏀诲嚮鑰呭彲鑳戒粠鏍稿績杞偍涓绱㈠埌瀹冿紝骞舵湁鏁堢粫杩囦换浣曞叡浜瀵嗛挜鐨勭嚎绋嬩笂鐨?ROP 淇濇姢
锛堟綔鍦ㄥ湴锛屾墍鏈夋潵鑷悓涓€鐖惰繘绋嬨€佷笖灏氭湭杩愯 `exec()` 鐨勭嚎绋嬶級銆?