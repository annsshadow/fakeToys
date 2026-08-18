## Speculation Control


鐩稿綋澶氱殑 CPU 鍏锋湁涓庢帹娴嬫墽琛岋紙speculation锛夌浉鍏崇殑缂洪櫡鐗规€э紝杩欎簺瀹為檯涓婃槸鍦ㄥ悇绉嶅舰寮忎笅瀵艰嚧鏁版嵁娉勬紡鐨勬紡娲烇紝鐢氳嚦浼氳法瓒婄壒鏉冨煙銆?
鍐呮牳浠ュ悇绉嶅舰寮忔彁渚涢拡瀵规绫绘紡娲炵殑缂撹В锛坢itigation锛夋帾鏂姐€傚叾涓竴浜涚紦瑙ｆ帾鏂藉湪缂栬瘧鏃跺彲閰嶇疆锛屼竴浜涘彲浠ラ€氳繃鍐呮牳鍛戒护琛屾彁渚涖€?
杩樻湁涓€绫荤紦瑙ｆ帾鏂介潪甯告槀璐碉紝浣嗗彲浠ュ皢瀹冧滑闄愬埗鍦ㄥ彈鎺х幆澧冧腑鐨勬煇缁勮繘绋嬫垨浠诲姟涓娿€傛帶鍒惰繖浜涚紦瑙ｆ帾鏂界殑鏈哄埗鏄€氳繃 `prctl(2)`銆?
鏈変袱涓笌姝ょ浉鍏崇殑 prctl 閫夐」锛?
 - PR_GET_SPECULATION_CTRL

 - PR_SET_SPECULATION_CTRL

### PR_GET_SPECULATION_CTRL


PR_GET_SPECULATION_CTRL 杩斿洖鐢?prctl(2) 鐨?arg2 閫夋嫨鐨勬帹娴嬫墽琛岀己闄风壒鎬х殑鐘舵€併€傝繑鍥炲€间娇鐢ㄤ綅 0-3锛屽惈涔夊涓嬶紙浣嗚娉ㄦ剰锛孭R_SPEC_L1D_FLUSH 鐨勮涔変笉閭ｄ箞鐩磋锛岃鍙傞槄涓嬮潰璇ョ壒瀹氭帶鍒剁殑鏂囨。锛夛細

==== ====================== ==================================================
Bit  Define                 Description
==== ====================== ==================================================
0    PR_SPEC_PRCTL          Mitigation 鍙€氳繃 PR_SET_SPECULATION_CTRL 鎸変换鍔℃帶鍒躲€?1    PR_SPEC_ENABLE         鎺ㄦ祴鐗规€у凡鍚敤锛岀紦瑙ｆ帾鏂藉凡绂佺敤銆?2    PR_SPEC_DISABLE        鎺ㄦ祴鐗规€у凡绂佺敤锛岀紦瑙ｆ帾鏂藉凡鍚敤銆?3    PR_SPEC_FORCE_DISABLE  涓?PR_SPEC_DISABLE 鐩稿悓锛屼絾涓嶅彲鎾ら攢銆傚悗缁殑
                            prctl(..., PR_SPEC_ENABLE) 灏嗕細澶辫触銆?4    PR_SPEC_DISABLE_NOEXEC 涓?PR_SPEC_DISABLE 鐩稿悓锛屼絾璇ョ姸鎬佷細鍦?`execve(2)` 鏃舵竻闄ゃ€?==== ====================== ==================================================

濡傛灉鎵€鏈変綅閮戒负 0锛屽垯璇?CPU 涓嶅彈璇ユ帹娴嬫墽琛岀己闄风壒鎬х殑褰卞搷銆?
濡傛灉璁剧疆浜?PR_SPEC_PRCTL锛屽垯鍙互浣跨敤鎸変换鍔＄殑缂撹В鎺у埗銆傚鏋滄湭璁剧疆锛屽璇ユ帹娴嬫墽琛岀己闄风壒鎬ц皟鐢?prctl(PR_SET_SPECULATION_CTRL) 灏嗕細澶辫触銆?

### PR_SET_SPECULATION_CTRL


PR_SET_SPECULATION_CTRL 鍏佽鎺у埗鐢?`prctl(2)` 鐨?arg2 鎸変换鍔￠€夋嫨鐨勬帹娴嬫墽琛岀己闄风壒鎬с€俛rg3 鐢ㄤ簬浼犲叆鎺у埗鍊硷紝鍗?PR_SPEC_ENABLE 鎴?PR_SPEC_DISABLE 鎴?PR_SPEC_FORCE_DISABLE銆?
### Common error codes

======= =================================================================
Value   Meaning
======= =================================================================
EINVAL  璇?prctl 鏈敱鏋舵瀯瀹炵幇锛屾垨鏈娇鐢ㄧ殑 prctl(2) 鍙傛暟涓嶄负 0銆?
ENODEV  arg2 閫夋嫨浜嗕竴涓笉鍙楁敮鎸佺殑鎺ㄦ祴鎵ц缂洪櫡鐗规€с€?======= =================================================================

### PR_SET_SPECULATION_CTRL error codes

======= =================================================================
Value   Meaning
======= =================================================================
0       鎴愬姛

ERANGE  arg3 涓嶆纭紝鍗冲畠鏃笉鏄?PR_SPEC_ENABLE 涔熶笉鏄?        PR_SPEC_DISABLE 涔熶笉鏄?PR_SPEC_FORCE_DISABLE銆?
ENXIO   瀵逛簬 PR_SPEC_STORE_BYPASS锛氱敱浜庣郴缁熺殑鍚姩閰嶇疆锛屾棤娉曢€氳繃 prctl
        鎺у埗鎵€閫夌殑鎺ㄦ祴鎵ц缂洪櫡鐗规€с€?
EPERM   宸茬粡浣跨敤 PR_SPEC_FORCE_DISABLE 绂佺敤浜嗘帹娴嬶紝鑰岃皟鐢ㄨ€呰瘯鍥惧啀娆?        鍚敤瀹冦€?
EPERM   瀵逛簬 PR_SPEC_L1D_FLUSH 鍜?PR_SPEC_INDIRECT_BRANCH锛氱敱浜庣郴缁熺殑鍚姩
        閰嶇疆锛屾棤娉曟帶鍒剁紦瑙ｆ帾鏂姐€?
======= =================================================================

### Speculation misfeature controls

- PR_SPEC_STORE_BYPASS: 鎺ㄦ祴鎬у瓨鍌ㄧ粫杩囷紙Speculative Store Bypass锛?
  璋冪敤鏂瑰紡锛?   - prctl(PR_GET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, 0, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_ENABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_DISABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_FORCE_DISABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_STORE_BYPASS, PR_SPEC_DISABLE_NOEXEC, 0, 0);

- PR_SPEC_INDIR_BRANCH: 鐢ㄦ埛杩涚▼涓殑闂存帴鍒嗘敮鎺ㄦ祴
                        锛堢紦瑙ｉ拡瀵圭敤鎴疯繘绋嬬殑 Spectre V2 椋庢牸鏀诲嚮锛?
  璋冪敤鏂瑰紡锛?   - prctl(PR_GET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, 0, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_ENABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_DISABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_INDIRECT_BRANCH, PR_SPEC_FORCE_DISABLE, 0, 0);

- PR_SPEC_L1D_FLUSH: 鍦ㄤ换鍔′笂涓嬫枃鍒囨崲鍑哄幓鏃跺埛鏂?L1D 缂撳瓨
                        锛堜粎鍦ㄤ换鍔¤繍琛屽湪闈?SMT 鏍稿績涓婃椂鏈夋晥锛?
瀵逛簬杩欎釜鎺у埗锛孭R_SPEC_ENABLE 琛ㄧず**缂撹В鎺柦**宸插惎鐢紙L1D 琚埛鏂帮級锛孭R_SPEC_DISABLE 琛ㄧず瀹冨凡绂佺敤銆?
  璋冪敤鏂瑰紡锛?   - prctl(PR_GET_SPECULATION_CTRL, PR_SPEC_L1D_FLUSH, 0, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_L1D_FLUSH, PR_SPEC_ENABLE, 0, 0);
   - prctl(PR_SET_SPECULATION_CTRL, PR_SPEC_L1D_FLUSH, PR_SPEC_DISABLE, 0, 0);
