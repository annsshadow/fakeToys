## 姹囩紪鍣ㄦ敞瑙?

Copyright (c) 2017-2019 Jiri Slaby

鏈枃妗ｆ弿杩颁簡鐢ㄤ簬鍦ㄦ眹缂栦腑鏍囨敞鏁版嵁鍜屼唬鐮佺殑鏂板畯銆傜壒鍒槸锛屽畠鍖呭惈鏈夊叧 `SYM_FUNC_START`銆乣SYM_FUNC_END`銆乣SYM_CODE_START` 绛夊畯鐨勪俊鎭€?
### 鍔ㄦ満

鏌愪簺浠ｇ爜锛堝鍏ュ彛銆佽烦鏉挎垨鍚姩浠ｇ爜锛夐渶瑕佺敤姹囩紪缂栧啓銆備笌 C 璇█涓€鏍凤紝杩欑被浠ｇ爜琚垎缁勪负鍑芥暟骞堕檮甯︽暟鎹€傛爣鍑嗘眹缂栧櫒骞朵笉寮哄埗鐢ㄦ埛绮剧‘鍦板皢閭ｄ簺鐗囨鏍囪涓轰唬鐮併€佹暟鎹紝鐢氳嚦涓嶈姹傛寚瀹氬叾闀垮害銆傜劧鑰岋紝姹囩紪鍣ㄤ細鍚戝紑鍙戣€呮彁渚涙绫绘敞瑙ｏ紝浠ヨ緟鍔╂暣涓眹缂栬繃绋嬩腑鐨勮皟璇曞櫒銆傛澶栵紝寮€鍙戣€呰繕甯屾湜灏嗘煇浜涘嚱鏁版爣璁颁负 **global锛堝叏灞€锛?*锛屼互渚垮湪缈昏瘧鍗曞厓涔嬪鍙銆?
闅忕潃鏃堕棿鐨勬帹绉伙紝Linux 鍐呮牳閲囩撼浜嗘潵鑷涓」鐩紙濡?`binutils`锛夌殑瀹忔潵绠€鍖栨绫绘敞瑙ｃ€傚洜姝ゅ嚭浜庡巻鍙插師鍥狅紝寮€鍙戣€呬竴鐩村湪姹囩紪涓娇鐢?`ENTRY`銆乣END`銆乣ENDPROC` 绛夋敞瑙ｃ€傜敱浜庣己涔忕浉鍏虫枃妗ｏ紝杩欎簺瀹忓湪鏌愪簺鍦版柟琚敤鍦ㄤ簡鐩稿綋閿欒鐨勪笂涓嬫枃涓€傛樉鐒讹紝`ENTRY` 鏃ㄥ湪琛ㄧず鍏ㄥ眬绗﹀彿锛堟棤璁烘槸鏁版嵁杩樻槸浠ｇ爜锛夌殑寮€澶淬€俙END` 鐢ㄤ簬鏍囪鏁版嵁鐨勭粨鏉熸垨鍏锋湁 **non-standard锛堥潪鏍囧噯锛?* 璋冪敤绾﹀畾鐨勭壒娈婂嚱鏁扮殑缁撴潫銆傜浉姣斾箣涓嬶紝`ENDPROC` 鍙簲娉ㄨВ **standard锛堟爣鍑嗭級** 鍑芥暟鐨勭粨灏俱€?
褰撹繖浜涘畯琚纭娇鐢ㄦ椂锛屽畠浠兘甯姪姹囩紪鍣ㄧ敓鎴愬ぇ灏忓拰绫诲瀷閮借姝ｇ‘璁剧疆鐨勭悊鎯崇洰鏍囨枃浠躲€備緥濡傦紝浠ヤ笅鍛戒护鐨勮緭鍑虹粨鏋滐細

```
   Num:    Value          Size Type    Bind   Vis      Ndx Name
    25: 0000000000000000    33 FUNC    GLOBAL DEFAULT    1 __put_user_1
    29: 0000000000000030    37 FUNC    GLOBAL DEFAULT    1 __put_user_2
    32: 0000000000000060    36 FUNC    GLOBAL DEFAULT    1 __put_user_4
    35: 0000000000000090    37 FUNC    GLOBAL DEFAULT    1 __put_user_8

```

杩欎笉浠呭璋冭瘯寰堥噸瑕併€傚綋瀛樺湪鍍忚繖鏍疯姝ｇ‘娉ㄨВ鐨勭洰鏍囨枃浠舵椂锛屽彲浠ュ湪鍏朵笂杩愯宸ュ叿鏉ョ敓鎴愭洿鏈夌敤鐨勪俊鎭€傜壒鍒槸锛屽湪姝ｇ‘娉ㄨВ鐨勭洰鏍囨枃浠朵笂锛屽彲浠ヨ繍琛?`objtool` 鏉ユ鏌ュ苟鍦ㄩ渶瑕佹椂淇鐩爣鏂囦欢銆傜洰鍓嶏紝`objtool` 鍙互鎶ュ憡鍑芥暟涓己澶辩殑甯ф寚閽堝缓绔?閿€姣併€傚畠杩樺彲浠ヤ负澶у鏁颁唬鐮佽嚜鍔ㄧ敓鎴?ORC unwinder锛圖ocumentation/arch/x86/orc-unwinder.rst锛夋墍闇€鐨勬敞瑙ｃ€傝繖涓よ€呭浜庢敮鎸佸彲闈犵殑鏍堝洖婧挨鍏堕噸瑕侊紝鑰屽彲闈犵殑鏍堝洖婧張鏄唴鏍稿疄鏃惰ˉ涓侊紙Documentation/livepatch/livepatch.rst锛夋墍蹇呴渶鐨勩€?
### 娉ㄦ剰浜嬮」涓庤璁?
姝ｅ鏈変汉鍙兘宸茬粡鎰忚瘑鍒扮殑锛屼互鍓嶅彧鏈変笁涓畯銆傝繖纭疄涓嶈冻浠ヨ鐩栨墍鏈夋儏鍐电粍鍚堬細

- 鏍囧噯/闈炴爣鍑嗗嚱鏁?- 浠ｇ爜/鏁版嵁
- 鍏ㄥ眬/灞€閮ㄧ鍙?
鏇剧粡鏈夎繃涓€娆?discussion_锛屽苟涓旀病鏈夋墿灞曞綋鍓嶇殑 `ENTRY/END*`

```
    So how about using macro names that actually show the purpose, instead
    of importing all the crappy, historic, essentially randomly chosen
    debug symbol macro names from the binutils and older kernels?

```

### 瀹忚鏄?

鏂板畯浠?`SYM_` 鍓嶇紑寮€澶达紝鍙垎涓轰笁涓富瑕佺被鍒細

1. `SYM_FUNC_*` 鈥斺€?鐢ㄤ簬鏍囨敞绫?C 鍑芥暟銆傚嵆閲囩敤鏍囧噯 C 璋冪敤绾﹀畾鐨勫嚱鏁般€備緥濡傦紝鍦?x86 涓婏紝杩欐剰鍛崇潃鏍堝湪棰勫畾浣嶇疆鍖呭惈涓€涓繑鍥炲湴鍧€锛屽苟涓斿嚱鏁扮殑杩斿洖鍙互浠ユ爣鍑嗘柟寮忚繘琛屻€傚綋鍚敤甯ф寚閽堟椂锛屽抚鎸囬拡鐨勪繚瀛?鎭㈠涔熷簲褰撳垎鍒湪鍑芥暟鐨勫紑澶?缁撳熬杩涜銆?
   璇稿 `objtool` 涔嬬被鐨勬鏌ュ伐鍏峰簲纭繚杩欎簺琚爣娉ㄧ殑鍑芥暟绗﹀悎杩欎簺瑙勫垯銆傝繖浜涘伐鍏疯繕鍙互杞绘槗鍦扮敤璋冭瘯淇℃伅锛堝 **ORC data**锛夎嚜鍔ㄦ敞瑙ｈ繖浜涘嚱鏁般€?
2. `SYM_CODE_*` 鈥斺€?浣跨敤鐗规畩鏍堣皟鐢ㄧ殑鐗规畩鍑芥暟銆傚彲浠ユ槸甯︽湁鐗规畩鏍堝唴瀹圭殑涓柇澶勭悊绋嬪簭銆佽烦鏉挎垨鍚姩鍑芥暟銆?
   妫€鏌ュ伐鍏峰ぇ澶氬拷鐣ュ杩欎簺鍑芥暟鐨勬鏌ャ€備絾浠嶅彲鑷姩鐢熸垚閮ㄥ垎璋冭瘯淇℃伅銆備负浜嗚幏寰楁纭殑璋冭瘯鏁版嵁锛岃繖娈典唬鐮侀渶瑕佸紑鍙戣€呮彁渚涜濡?`UNWIND_HINT_REGS` 涔嬬被鐨勬彁绀恒€?
3. `SYM_DATA*` 鈥斺€?鏄剧劧鏄睘浜?`.data` 娈佃€岄潪 `.text` 娈电殑鏁版嵁銆傛暟鎹笉鍖呭惈鎸囦护锛屽洜姝ゅ伐鍏峰繀椤诲鍏惰繘琛岀壒娈婂鐞嗭細鏃笉鑳藉皢杩欎簺瀛楄妭褰撲綔鎸囦护锛屼篃涓嶈兘涓哄叾鍒嗛厤浠讳綍璋冭瘯淇℃伅銆?
#### 鎸囦护瀹?
鏈妭娑电洊涓婃枃鍒椾妇鐨?`SYM_FUNC_**` 鍜?`SYM_CODE_**`銆?
`objtool` 瑕佹眰鎵€鏈変唬鐮侀兘蹇呴』鍖呭惈鍦ㄤ竴涓?ELF 绗﹀彿涓€傚甫鏈?`.L` 鍓嶇紑鐨勭鍙峰悕涓嶄細鐢熸垚绗﹀彿琛ㄩ」銆傚甫鏈?`.L` 鍓嶇紑鐨勭鍙峰彲浠ュ湪浠ｇ爜鍖哄煙鍐呬娇鐢紝浣嗗簲閬垮厤鐢ㄤ簬閫氳繃 `SYM_*_START/END` 娉ㄨВ鏉ヨ〃绀轰竴娈典唬鐮佽寖鍥淬€?
- `SYM_FUNC_START` 涓?`SYM_FUNC_START_LOCAL` 搴斿綋鏄?**鏈€甯哥敤** 鐨勬爣璁般€傚畠浠敤浜庡叿鏈夋爣鍑嗚皟鐢ㄧ害瀹氱殑鍑芥暟鈥斺€斿叏灞€鍜屽眬閮ㄥ嚱鏁般€備笌 C 璇█绫讳技锛屼簩鑰呴兘浼氬皢鍑芥暟鎸夋灦鏋勭壒瀹氱殑 `__ALIGN` 瀛楄妭瀵归綈銆備篃瀛樺湪 `_NOALIGN` 鍙樹綋锛岀敤浜庡紑鍙戣€呬笉甯屾湜杩涜杩欑闅愬紡瀵归綈鐨勭壒娈婃儏鍐点€?
  `SYM_FUNC_START_WEAK` 涓?`SYM_FUNC_START_WEAK_NOALIGN` 鏍囪涔熶綔涓?C 璇█涓凡鐭ョ殑 **weak** 灞炴€х殑姹囩紪瀵瑰簲鐗╂彁渚涖€?
  鎵€鏈夎繖浜涙爣璁?**閮?* 搴斾笌 `SYM_FUNC_END` 閰嶅浣跨敤銆傞鍏堬紝瀹冨皢鎸囦护搴忓垪鏍囪涓轰竴涓嚱鏁板苟璁＄畻鍏跺ぇ灏忓啓鍏ョ敓鎴愮殑鐩爣鏂囦欢銆傚叾娆★紝杩欎篃绠€鍖栦簡姝ょ被鐩爣鏂囦欢鐨勬鏌ヤ笌澶勭悊锛屽洜涓哄伐鍏峰彲浠ヨ交鏉炬壘鍒扮‘鍒囩殑鍑芥暟杈圭晫銆?
  鍥犳锛屽湪澶у鏁版儏鍐典笅锛屽紑鍙戣€呭簲璇ョ紪鍐欑被浼间互涓嬬殑鍐呭锛?
```
    SYM_FUNC_START(memset)
        ... asm insns ...
    SYM_FUNC_END(memset)

  In fact, this kind of annotation corresponds to the now deprecated ``ENTRY``
  and ``ENDPROC`` macros.

```
- `SYM_FUNC_ALIAS`銆乣SYM_FUNC_ALIAS_LOCAL` 浠ュ強 `SYM_FUNC_ALIAS_WEAK` 鍙互

```
    SYM_FUNC_START(__memset)
        ... asm insns ...
    SYN_FUNC_END(__memset)
    SYM_FUNC_ALIAS(memset, __memset)

  In this example, one can call ``__memset`` or ``memset`` with the same
  result, except the debug information for the instructions is generated to
  the object file only once -- for the non-``ALIAS`` case.

```
- `SYM_CODE_START` 涓?`SYM_CODE_START_LOCAL` 搴斾粎鐢ㄤ簬鐗规畩鎯呭喌鈥斺€斿嵆浣犳竻妤氳嚜宸卞湪鍋氫粈涔堛€傚畠涓撶敤浜庝腑鏂鐞嗙▼搴忓強璋冪敤绾﹀畾闈?C 绾﹀畾鐨勭被浼煎満鏅€備篃瀛樺湪 `_NOALIGN` 鍙樹綋銆傚叾鐢ㄦ硶涓?`FUNC`

```
    SYM_CODE_START_LOCAL(bad_put_user)
        ... asm insns ...
    SYM_CODE_END(bad_put_user)

  Again, every ``SYM_CODE_START*`` **shall** be coupled by ``SYM_CODE_END``.

  To some extent, this category corresponds to deprecated ``ENTRY`` and
  ``END``. Except ``END`` had several other meanings too.

```
- `SYM_INNER_LABEL*` 鐢ㄤ簬琛ㄧず鏌愪簺 `SYM_{CODE,FUNC}_START` 涓?`SYM_{CODE,FUNC}_END` 鍐呴儴鐨勬爣绛俱€傚畠浠潪甯哥被浼间簬

```
    SYM_CODE_START(ftrace_caller)
        /* save_mcount_regs fills in first two parameters */
        ...

    SYM_INNER_LABEL(ftrace_caller_op_ptr, SYM_L_GLOBAL)
        /* Load the ftrace_ops into the 3rd parameter */
        ...

    SYM_INNER_LABEL(ftrace_call, SYM_L_GLOBAL)
        call ftrace_stub
        ...
        retq
    SYM_CODE_END(ftrace_caller)

```
#### 鏁版嵁瀹?
涓庢寚浠ょ被浼硷紝涔熸湁鍑犱釜瀹忕敤浜庢弿杩版眹缂栦腑鐨勬暟鎹€?
- `SYM_DATA_START` 涓?`SYM_DATA_START_LOCAL` 鏍囪鏌愪簺鏁版嵁鐨勮捣濮嬶紝骞跺簲涓?`SYM_DATA_END` 鎴?`SYM_DATA_END_LABEL` 閰嶅悎浣跨敤銆傚悗鑰呰繕浼氬湪缁撳熬娣诲姞涓€涓爣绛撅紝浠ヤ究浜轰滑鍙互浣跨敤 `lstack` 浠ュ強锛堝眬閮ㄧ殑锛塦lstack_end`锛屽涓嬫墍绀猴細

```
    SYM_DATA_START_LOCAL(lstack)
        .skip 4096
    SYM_DATA_END_LABEL(lstack, SYM_L_LOCAL, lstack_end)

```
- `SYM_DATA` 涓?`SYM_DATA_LOCAL` 鏄敤浜庣畝鍗曘€佸ぇ澶氫负涓€琛岀殑

```
    SYM_DATA(HEAP,     .long rm_heap)
    SYM_DATA(heap_end, .long rm_stack)

  In the end, they expand to ``SYM_DATA_START`` with ``SYM_DATA_END``
  internally.

```
#### 杈呭姪瀹?
涓婅堪鎵€鏈夊畯鏈€缁堥兘浼氬綊缁撲负瀵?`SYM_START`銆乣SYM_END` 鎴?`SYM_ENTRY` 鐨勬煇绉嶈皟鐢ㄣ€傞€氬父锛屽紑鍙戣€呭簲閬垮厤浣跨敤杩欎簺銆傛澶栵紝鍦ㄤ笂杩扮ず渚嬩腑锛屽彲浠ョ湅鍒?`SYM_L_LOCAL`銆傝繕鏈?`SYM_L_GLOBAL` 涓?`SYM_L_WEAK`銆傚畠浠兘鐢ㄤ簬琛ㄧず琚叾鏍囪鐨勭鍙风殑閾炬帴灞炴€с€傚畠浠棦鐢ㄤ簬鍓嶈堪瀹忕殑 `_LABEL` 鍙樹綋涓紝涔熺敤浜?`SYM_START`銆?

#### 瑕嗙洊瀹?
鏋舵瀯涔熷彲浠ュ湪鑷繁鐨?`asm/linkage.h` 涓鐩栦换鎰忓畯锛屽寘鎷寚瀹氱鍙风被鍨嬬殑瀹忥紙`SYM_T_FUNC`銆乣SYM_T_OBJECT` 浠ュ強 `SYM_T_NONE`锛夈€傜敱浜庢湰鏂囨。涓弿杩扮殑姣忎釜瀹忛兘琚?`#ifdef` + `#endif` 鍖呭洿锛屽彧闇€鍦ㄤ笂杩版灦鏋勭浉鍏崇殑澶存枃浠朵腑浠ヤ笉鍚屾柟寮忓畾涔夎繖浜涘畯鍗冲彲銆?