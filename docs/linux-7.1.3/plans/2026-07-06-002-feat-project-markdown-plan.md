---
title: feat: Generate PROJECT.md from kernel source
type: feat
status: active
date: 2026-07-06
origin: docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md
deepened: 2026-07-06
---

# feat: Generate PROJECT.md from kernel source

## Summary

缂栧啓涓€涓?Python 鑴氭湰 `tools/docs/gen-project-md.py`锛屼粠 Linux 鍐呮牳婧愮爜鐩存帴鎻愬彇淇℃伅鐢熸垚 `docs/PROJECT.md`锛屽唴瀹瑰寘鎷洰褰曟爲锛堝甫鐢ㄩ€旇鏄庯級銆並config 閰嶇疆椤规憳瑕併€丮akefile 鐩爣鍒楄〃銆佸悇瀛愮郴缁?README/澶存枃浠舵敞閲婃憳瑕併€備俊鎭潵婧愪负婧愮爜鏂囦欢鏈韩锛屼笉渚濊禆 `Documentation/` 涓嬪凡鏈夌殑 `.rst` 鏂囦欢銆?
---

## Problem Frame

Linux 鍐呮牳椤圭洰瑙勬ā搴炲ぇ锛垀36,681 涓?C 鏂囦欢銆亊26,666 涓ご鏂囦欢锛夛紝 newcomers 鎴栦笅娓告秷璐硅€呴毦浠ュ揩閫熷缓绔嬪椤圭洰缁撴瀯鐨勬暣浣撹鐭ャ€俙Documentation/` 涓嬬殑鏂囨。璇﹀敖浣嗗垎鏁ｏ紝涓斾互 reStructuredText 涓轰富銆備竴浠戒粠婧愮爜鐩存帴鐢熸垚鐨?Markdown 鎬昏鏂囨。鑳介檷浣庡叆闂ㄩ棬妲涳紝鏀寔 LLM ingestion銆侀潤鎬佺珯鐐圭敓鎴愬拰璺ㄩ」鐩暅鍍忋€?see origin: docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md)

---

## Requirements

- R1. 蹇呴』浠庢簮鐮佺洿鎺ユ彁鍙栦俊鎭紝涓嶄緷璧?`Documentation/` 涓嬪凡鏈夌殑 `.rst` 鏂囦欢浣滀负淇℃伅鏉ユ簮銆?- R2. 杈撳嚭鏂囦欢涓?`docs/PROJECT.md`锛圡arkdown 鏍煎紡锛夈€?- R3. 鏂囨。蹇呴』鍖呭惈鐩綍鏍戯細椤圭洰椤跺眰鍜屽叧閿瓙鐩綍鐨勯€掑綊缁撴瀯锛屾瘡涓洰褰曢檮甯︾敤閫旇鏄庯紙浠?README銆丮akefile 娉ㄩ噴銆佹垨鐩綍鍚嶆帹鏂級銆?- R4. 鏂囨。蹇呴』鍖呭惈 Kconfig 閰嶇疆椤规憳瑕侊細浠庢牴 `Kconfig` 鍜屽叧閿瓙绯荤粺 Kconfig 涓彁鍙栦富瑕侀厤缃€夐」锛屼互 Markdown 琛ㄦ牸鎴栧垪琛ㄥ舰寮忓憟鐜般€?- R5. 鏂囨。蹇呴』鍖呭惈 Makefile 鐩爣鍒楄〃锛氫粠椤跺眰 `Makefile` 鍜?`Documentation/Makefile` 涓彁鍙栦富瑕佹瀯寤虹洰鏍囷紝闄勫甫绠€鐭鏄庛€?- R6. 鏂囨。蹇呴』鍖呭惈鍚勫瓙绯荤粺鐨?README/澶存枃浠舵敞閲婃憳瑕侊細鎵弿姣忎釜鍏抽敭瀛愮洰褰曚笅鐨?`README*` 鏂囦欢鍜屾牳蹇冨ご鏂囦欢寮€澶寸殑娉ㄩ噴鍧楋紝鎻愬彇瀛愮郴缁熸弿杩般€?- R7. 鐢熸垚杩囩▼蹇呴』鏄彲閲嶅鐨勶細閲嶆柊杩愯鐢熸垚鑴氭湰搴斾骇鐢熶竴鑷寸殑杈撳嚭銆?- R8. 鐢熸垚鑴氭湰蹇呴』涓嶄慨鏀逛换浣曟簮鐮佹枃浠讹紝浠呰鍙栥€?
**Origin actors:** 鍐呮牳 newcomers銆佷笅娓告秷璐硅€呫€丩LM ingestion 绠￠亾
**Origin flows:** F1 鏈湴椤圭洰鎬昏 / F2 CI 鐢熸垚 / F3 璺ㄩ」鐩暅鍍?
---

## Scope Boundaries

- 涓嶆彁鍙?kernel-doc 娉ㄩ噴锛坄/** ... */`锛夊埌 PROJECT.md锛沰ernel-doc 鐨勮緭鍑虹暀缁欑幇鏈夊伐鍏烽摼澶勭悊銆?- 涓嶇敓鎴?API 绾у埆鐨勮缁嗗嚱鏁?缁撴瀯浣撴枃妗ｏ紱PROJECT.md 鏄」鐩€昏锛屼笉鏄?API 鍙傝€冦€?- 涓嶄慨鏀逛换浣曟簮鐮佹垨 `Documentation/` 涓嬬殑鏂囦欢銆?- 涓嶆浛浠?`Documentation/` 鐨勭幇鏈夋枃妗ｄ綋绯伙紱PROJECT.md 鏄叆鍙ｇ骇鎬昏锛屼笉鏄畬鏁存枃妗ｃ€?- 涓嶉€掑綊鎵弿鎵€鏈?~6,143 涓洰褰曪紱鑱氱劍椤跺眰鍜屽叧閿瓙鐩綍锛堢害 15-20 涓級銆?
---

## Context & Research

### Relevant Code and Patterns

- `tools/docs/kernel-doc` 鈥?鐜版湁 Python 鑴氭湰锛屼粠 C/H 鏂囦欢鎻愬彇 kernel-doc 娉ㄩ噴锛岃緭鍑?ReST 鎴?man page銆傚叾鏋舵瀯锛堣В鏋愬櫒 + 杈撳嚭鏍煎紡绫伙級鍙綔涓?gen-project-md.py 鐨勫弬鑰冩ā寮忋€?- `tools/docs/md-convert.py` 鈥?鏂板缓鐨?Markdown 鐢熸垚鑴氭湰锛堟湰椤圭洰鍚屾湡瀹炴柦锛夛紝浣跨敤鏍囧噯搴?+ subprocess锛屾棤绗笁鏂逛緷璧栵紝椋庢牸鍙€熼壌銆?- `tools/docs/sphinx-pre-install` 鈥?渚濊禆妫€鏌ラ鏍煎弬鑰冦€?- `tools/lib/python/kdoc/kdoc_output.py` 鈥?kernel-doc 鐨勮緭鍑烘牸寮忓熀绫?`OutputFormat` 鍜?`RestFormat`/`ManFormat` 瀹炵幇銆俙MarkdownFormat` 鍙互浣滀负 future work 鍔犲叆姝や綋绯汇€?- `Kconfig` 鈥?鏍归厤缃枃浠讹紝34 琛岋紝閫氳繃 `source` 鎸囦护寮曠敤瀛愮洰褰?Kconfig锛岃娉曚负 `config`/`menuconfig`/`choice`/`endchoice`/`endif`銆?- `Makefile` 鈥?2307 琛岋紝鐩爣瀹氫箟妯″紡涓?`<target>:` 鎴?`PHONY += <target>`锛岀洰鏍囧垎鏁ｅ湪澶氬銆?- `README` 鈥?鏍圭洰褰?README 鍜屽涓瓙鐩綍 README 鏂囦欢瀛樺湪锛屼絾鍏抽敭瀛愮郴缁燂紙`arch/`銆乣drivers/`銆乣fs/` 绛夛級娌℃湁 README銆?
### Institutional Learnings

鏃犵洿鎺ョ浉鍏崇殑 `docs/solutions/` 鏉＄洰銆?
### External References

- Kconfig 璇硶鏂囨。锛歚Documentation/kbuild/kconfig-language.rst`锛堟湰鑴氭湰涓嶈鍙栨鏂囦欢锛屼粎浣滀负澶栭儴鍙傝€冪悊瑙ｈ娉曪級

---

## Key Technical Decisions

- **鍗曡剼鏈疄鐜般€?* 鎵€鏈夊洓涓緭鍑虹粍浠讹紙鐩綍鏍戙€並config銆丮akefile銆丷EADME锛夌敱涓€涓?Python 鑴氭湰 `tools/docs/gen-project-md.py` 鐢熸垚锛岄伩鍏嶅鑴氭湰鍗忚皟鎴愭湰銆?- **Kconfig 瑙ｆ瀽閲囩敤姝ｅ垯琛ㄨ揪寮忚€岄潪瀹屾暣璇嶆硶鍒嗘瀽鍣ㄣ€?* Kconfig 璇硶绠€鍗曪紙`config`/`menuconfig`/`source`/`comment`/`endmenu`/`endif`锛夛紝姝ｅ垯琛ㄨ揪寮忚冻浠ユ彁鍙栭厤缃」鍚嶇О鍜屾弿杩帮紝涓斾繚鎸佽剼鏈交閲忋€?- **鐩綍鏍戞繁搴﹂檺鍒朵负 2 灞傘€?* 閫掑綊鎵弿鍏ㄩ儴 ~6,143 涓洰褰曚細浜х敓 MB 绾ц緭鍑猴紝杩濊儗"KB 绾?鎴愬姛鏍囧噯銆倂1 鎵弿椤跺眰 + 鍏抽敭瀛愮洰褰曪紙`arch/`銆乣drivers/`銆乣fs/`銆乣kernel/`銆乣mm/`銆乣net/`銆乣include/`銆乣lib/`銆乣scripts/`銆乣tools/`銆乣security/`銆乣crypto/`銆乣sound/`銆乣virt/`銆乣io_uring/`銆乣ipc/`銆乣samples/`銆乣rust/`锛夛紝姣忎釜瀛愮洰褰曚笅浠呭垪鍑虹涓€绾у瓙鐩綍銆?- **Makefile 鐩爣鎻愬彇鍏奸【椤跺眰 Makefile 鍜?Documentation/Makefile銆?* 椤跺眰 Makefile 鏈夋瀯寤虹洰鏍囷紝Documentation/Makefile 鏈夋枃妗ｇ洰鏍囷紝涓よ€呬簰琛ャ€?- **README 鎻愬彇鑱氱劍鍏抽敭瀛愮洰褰曘€?* 浠呭湪 `arch/<arch>/`銆乣drivers/<driver>/` 绛夊叧閿瓙鐩綍涓嬫悳绱?`README*` 鏂囦欢锛屼笉鍏ㄦ爲鎵弿銆?- **杈撳嚭纭畾鎬с€?* 鎵€鏈夌洰褰曢亶鍘嗕娇鐢?`sorted()` 鎺掑簭锛岀‘淇濆娆¤繍琛岃緭鍑轰竴鑷达紙婊¤冻 R7锛夈€?
---

## Open Questions

### Resolved During Planning

- **杈撳嚭鏍煎紡锛?* 閫夊畾鍗曟枃浠?`docs/PROJECT.md`锛岃€岄潪澶氭枃浠躲€?- **鐩綍鏍戞繁搴︼細** 闄愬埗涓?2 灞傦紙椤跺眰 + 鍏抽敭瀛愮洰褰曠殑绗竴绾э級锛屼繚鎸佽緭鍑?KB 绾с€?- **Kconfig 瑙ｆ瀽绛栫暐锛?* 姝ｅ垯琛ㄨ揪寮忔彁鍙?`config`/`menuconfig` 鏉＄洰鍚嶇О鍜屽府鍔╂枃鏈紝涓嶈拷姹傚畬鏁磋瘝娉曞垎鏋愩€?
### Deferred to Implementation

- 閮ㄥ垎瀛愮洰褰曪紙濡?`drivers/` 涓嬫湁鏁扮櫨涓┍鍔ㄥ瓙鐩綍锛夌殑鐢ㄩ€旇鏄庡彲鑳介渶瑕佷汉宸ユ爣娉ㄦ垨浠?`Kconfig` 涓帹鏂紝瀹炵幇鏃堕渶纭鎺ㄦ柇鍑嗙‘搴︺€?- Makefile 鐩爣璇存槑鏂囨湰鐨勬潵婧愨€斺€旀湁浜涚洰鏍囨湁娉ㄩ噴锛屾湁浜涙病鏈夛紝闇€瑕佸疄鐜版椂鍐冲畾鏄惁浠庣洰鏍囧悕鎺ㄦ柇鎴栫暀绌恒€?
---

## Implementation Units

### U1. 鍒涘缓椤圭洰缁撴瀯鎵弿鍣?
**Goal:** 瀹炵幇鐩綍鏍戞壂鎻忛€昏緫锛岄€掑綊閬嶅巻椤圭洰椤跺眰鍜屽叧閿瓙鐩綍锛屾彁鍙栨瘡涓洰褰曠殑鐢ㄩ€旇鏄庛€?
**Requirements:** R1, R3, R7, R8

**Dependencies:** 鏃?
**Files:**
- Create: `tools/docs/gen-project-md.py`

**Approach:**
- 瀹氫箟 KEY_DIRS 鍒楄〃锛屽寘鍚渶瑕佹壂鎻忕殑鍏抽敭瀛愮洰褰曪紙`arch/`銆乣drivers/`銆乣fs/`銆乣kernel/`銆乣mm/`銆乣net/`銆乣include/`銆乣lib/`銆乣scripts/`銆乣tools/` 绛夛級銆?- 瀵规瘡涓叧閿瓙鐩綍锛岄€掑綊鎵弿鍏剁涓€绾у瓙鐩綍锛堟繁搴﹂檺鍒朵负 2锛夈€?- 鐢ㄩ€旇鏄庢彁鍙栦紭鍏堢骇锛歊EADME 鏂囦欢绗竴琛?> Makefile 椤堕儴鐨勬敞閲婂潡 > 鐩綍鍚嶆帹鏂€?- 杈撳嚭涓?Python 鏁版嵁缁撴瀯锛堝祵濂楀瓧鍏革級锛屼緵鍚庣画鍗曞厓缁勫悎涓?Markdown銆?- 鎵€鏈夌洰褰曢亶鍘嗕娇鐢?`sorted()` 纭繚纭畾鎬ц緭鍑恒€?
**Patterns to follow:**
- `tools/docs/kernel-doc` 鐨勬ā鍧楀寲璁捐锛堟壂鎻忓櫒 + 杈撳嚭鍣ㄥ垎绂伙級銆?- `tools/docs/md-convert.py` 鐨勬爣鍑嗗簱浼樺厛銆佹棤绗笁鏂逛緷璧栭鏍笺€?
**Test scenarios:**
- Happy path: 鎵弿鍏抽敭瀛愮洰褰曪紝姝ｇ‘鎻愬彇鐩綍鍚嶅拰鐢ㄩ€旇鏄庛€?- 鏃?README: 鐩綍涓嬫棤 README 鏃讹紝鍥為€€鍒?Makefile 娉ㄩ噴鎴栫洰褰曞悕銆?- 娣卞害闄愬埗: `drivers/` 涓嬩粎鍒楀嚭绗竴绾у瓙鐩綍锛堝 `drivers/net/`銆乣drivers/block/`锛夛紝涓嶉€掑綊鍒?`drivers/net/wireless/`銆?- 纭畾鎬? 杩炵画杩愯涓ゆ锛岀洰褰曟爲椤哄簭瀹屽叏涓€鑷淬€?- 闈炴簮鐮佷慨鏀? 鑴氭湰杩愯鏈熼棿涓嶄慨鏀逛换浣曟枃浠躲€?
**Verification:**
- 鐢熸垚鐨勭洰褰曟爲鏁版嵁缁撴瀯鍖呭惈鎵€鏈?KEY_DIRS锛屾瘡涓洰褰曟湁鍚嶇О鍜岀敤閫旇鏄庡瓧娈点€?- 杈撳嚭椤哄簭纭畾鎬ч獙璇侀€氳繃銆?
---

### U2. 瀹炵幇 Kconfig 閰嶇疆椤规憳瑕佹彁鍙栧櫒

**Goal:** 瑙ｆ瀽鏍?`Kconfig` 鍜屽叧閿瓙绯荤粺 Kconfig 鏂囦欢锛屾彁鍙栦富瑕侀厤缃」锛堝悕绉般€佺被鍨嬨€佸府鍔╂枃鏈級锛岀敓鎴?Markdown 琛ㄦ牸銆?
**Requirements:** R1, R4, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/docs/gen-project-md.py`锛堝悓涓€鑴氭湰鐨勬柊妯″潡锛?
**Approach:**
- 姝ｅ垯琛ㄨ揪寮忓尮閰?Kconfig 璇硶锛?  - `config <NAME>` / `menuconfig <NAME>` 鈥?閰嶇疆椤瑰叆鍙?  - `bool/string/int/hex` 鈥?绫诲瀷
  - `help` / `---help---` 鈥?甯姪鏂囨湰寮€濮嬫爣璁?  - `source "<path>"` 鈥?閫掑綊瑙ｆ瀽寮曠敤鐨?Kconfig 鏂囦欢
  - `comment` / `menu` / `endmenu` / `endif` 鈥?缁撴瀯鏍囪锛岃烦杩?- 鎻愬彇閰嶇疆椤瑰悕绉般€佺被鍨嬨€佸府鍔╂枃鏈紙鎴柇鑷冲悎鐞嗛暱搴︼級銆?- **鎸夊瓙绯荤粺鍒嗙粍锛?* 鏍规嵁 `source "<path>"` 涓殑璺緞鎺ㄦ柇瀛愮郴缁熷綊灞炪€傛槧灏勮鍒欎负锛歚arch/` 鈫?鏋舵瀯銆乣drivers/` 鈫?椹卞姩銆乣fs/` 鈫?鏂囦欢绯荤粺銆乣net/` 鈫?缃戠粶銆乣security/` 鈫?瀹夊叏銆乣crypto/` 鈫?鍔犲瘑銆乣sound/` 鈫?澹伴煶銆乣lib/` 鈫?搴撱€乣kernel/` 鈫?鍐呮牳鏍稿績銆乣mm/` 鈫?鍐呭瓨绠＄悊銆乣fs/` 鈫?鏂囦欢绯荤粺銆乣Documentation/` 鈫?鏂囨。銆傝矾寰勫墠缂€涓嶅尮閰嶆椂褰掑叆 "Other"銆?- 鐢熸垚 Markdown 琛ㄦ牸锛歚| 閰嶇疆椤?| 绫诲瀷 | 璇存槑 |`銆?
**Patterns to follow:**
- `tools/lib/python/kdoc/kdoc_re.py` 鐨?`KernRe` 姝ｅ垯灏佽妯″紡銆?
**Test scenarios:**
- Happy path: 瑙ｆ瀽鏍?Kconfig 鐨?source 寮曠敤锛屾纭彁鍙?10+ 涓厤缃」銆?- help 鏂囨湰鎻愬彇: 澶氳 help 鏂囨湰琚纭嫾鎺ヤ负鍗曡鎽樿銆?- 宓屽缁撴瀯: `menu`/`endmenu` 鍐呯殑閰嶇疆椤硅姝ｇ‘鎻愬彇锛屼笉琚烦杩囥€?- source 閫掑綊: 瑙ｆ瀽 `fs/Kconfig` 寮曠敤鐨勬枃浠剁郴缁熼厤缃」銆?- 纭畾鎬? 鐩稿悓杈撳叆浜х敓鐩稿悓杈撳嚭椤哄簭銆?
**Verification:**
- Kconfig 鎻愬彇缁撴灉鍖呭惈鑷冲皯 5 涓富瑕佺被鍒殑閰嶇疆椤癸紙鏋舵瀯銆侀┍鍔ㄣ€佹枃浠剁郴缁熴€佺綉缁溿€佸畨鍏級銆?- Markdown 琛ㄦ牸鏍煎紡姝ｇ‘锛屾棤璇硶閿欒銆?
---

### U3. 瀹炵幇 Makefile 鐩爣鎻愬彇鍣?
**Goal:** 瑙ｆ瀽椤跺眰 `Makefile` 鍜?`Documentation/Makefile`锛屾彁鍙栦富瑕佹瀯寤虹洰鏍囧強鍏惰鏄庛€?
**Requirements:** R1, R5, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/docs/gen-project-md.py`锛堝悓涓€鑴氭湰鐨勬柊妯″潡锛?
**Approach:**
- 姝ｅ垯琛ㄨ揪寮忓尮閰?Makefile 鐩爣瀹氫箟锛?  - `^<target>:` 鈥?鏍囧噯鐩爣锛堟帓闄や互 `#` 寮€澶寸殑娉ㄩ噴琛岋級
  - `^PHONY += <target>` 鈥?PHONY 鐩爣
  - `^# <comment>` 鈥?绱ч偦鐩爣涓婃柟鐨勬敞閲婁綔涓鸿鏄?  - Pattern rules锛堝 `%.o: %.c`锛塿1 璺宠繃锛屼笉绾冲叆 PROJECT.md
- 浠庨《灞?Makefile 鎻愬彇甯歌鐩爣锛歚all`銆乣modules`銆乣clean`銆乣mrproper`銆乣distclean`銆乣help`銆乣defconfig`銆乣menuconfig`銆乣O=` 鐩稿叧璇存槑绛夈€?- 浠?`Documentation/Makefile` 鎻愬彇鏂囨。鐩爣锛歚htmldocs`銆乣pdfdocs`銆乣epubdocs`銆乣markdowndocs` 绛夈€?- 鐩爣璇存槑鎻愬彇浼樺厛绾э細鐩爣涓婃柟娉ㄩ噴 > 浠庣洰鏍囧悕鎺ㄦ柇 > 绌恒€?- 鎸夌洰鏍囩被鍒垎缁勶紙鏋勫缓鐩爣銆侀厤缃洰鏍囥€佹竻鐞嗙洰鏍囥€佹枃妗ｇ洰鏍囷級銆?
**Patterns to follow:**
- `tools/docs/kernel-doc` 鐨勮繃婊ゆā寮忥紙浠呮彁鍙栭渶瑕佺殑绗﹀彿锛夈€?
**Test scenarios:**
- Happy path: 鎻愬彇椤跺眰 Makefile 鐨?10+ 涓富瑕佺洰鏍囥€?- 娉ㄩ噴鎻愬彇: 鐩爣涓婃柟 3 琛屽唴鐨勬敞閲婅姝ｇ‘鎹曡幏浣滀负璇存槑銆?- PHONY 鐩爣: `PHONY +=` 澹版槑鐨勭洰鏍囪姝ｇ‘璇嗗埆銆?- 鏂囨。 Makefile: `Documentation/Makefile` 涓殑 `htmldocs`銆乣pdfdocs` 绛夌洰鏍囪鎻愬彇銆?- 纭畾鎬? 鐩稿悓杈撳叆浜х敓鐩稿悓杈撳嚭椤哄簭銆?
**Verification:**
- 鎻愬彇鐨勭洰鏍囧垪琛ㄥ寘鍚嚦灏?10 涓父鐢ㄧ洰鏍囧強鍏惰鏄庛€?- Markdown 鍒楄〃鏍煎紡姝ｇ‘銆?
---

### U4. 瀹炵幇 README/澶存枃浠舵敞閲婃彁鍙栧櫒

**Goal:** 鎵弿鍏抽敭瀛愮洰褰曚笅鐨?README 鏂囦欢鍜屾牳蹇冨ご鏂囦欢寮€澶寸殑娉ㄩ噴鍧楋紝鎻愬彇瀛愮郴缁熸弿杩般€?
**Requirements:** R1, R6, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/docs/gen-project-md.py`锛堝悓涓€鑴氭湰鐨勬柊妯″潡锛?
**Approach:**
- 鍦?KEY_DIRS 涓殑姣忎釜鍏抽敭瀛愮洰褰曚笅鎼滅储 `README*` 鏂囦欢锛坄README`銆乣README.md`銆乣README.rst` 绛夛級銆?- 瀵规壘鍒扮殑 README 鏂囦欢锛屾彁鍙栧墠 10 琛屾垨绗竴涓┖琛屽墠鐨勬枃鏈綔涓烘憳瑕併€?- 瀵规病鏈?README 鐨勫叧閿瓙鐩綍锛屾壂鎻忓叾鏍稿績澶存枃浠讹紝鎻愬彇鏂囦欢寮€澶存敞閲婂潡锛坄/*` 鍒?`*/`锛変綔涓哄瓙绯荤粺鎻忚堪銆傚悇瀛愮郴缁熷搴旂殑鏍稿績澶存枃浠舵槧灏勫涓嬶細
  - `kernel/` 鈫?`include/linux/sched.h`銆乣kernel/sched/sched.h`
  - `mm/` 鈫?`include/linux/mm.h`銆乣mm/mmap.c`锛堟枃浠跺ご娉ㄩ噴锛?  - `fs/` 鈫?`include/linux/fs.h`
  - `net/` 鈫?`include/linux/net.h`銆乣net/core/skbuff.h`
  - `arch/` 鈫?鍚勬灦鏋勭殑 `include/asm/entry-common.h` 鎴?`arch/<arch>/kernel/` 涓嬬殑鏍稿績鏂囦欢澶存敞閲?  - `drivers/` 鈫?鏃犵粺涓€澶存枃浠讹紝鍥為€€鍒扮洰褰曞悕 + `drivers/base/` 涓嬬殑 `base.h`
  - `lib/` 鈫?`include/linux/bitops.h`銆乣lib/radix-tree.c`锛堟枃浠跺ご娉ㄩ噴锛?  - `include/` 鈫?`include/linux/printk.h`锛堟枃浠跺ご娉ㄩ噴锛?- 濡傛灉涓よ€呴兘涓嶅瓨鍦紝浣跨敤鐩綍鍚?+ 宸茬煡椤圭洰鐭ヨ瘑鐢熸垚绠€瑕佽鏄庛€?- 鎸夊瓙绯荤粺鍒嗙粍杈撳嚭涓?Markdown 鍒楄〃鎴栫畝鐭钀姐€?
**Patterns to follow:**
- `tools/docs/kernel-doc` 鐨勬敞閲婃彁鍙栨ā寮忥紙璇嗗埆 `/**` 鍧楋級銆?
**Test scenarios:**
- Happy path: `arch/x86/` 涓嬬殑 README 琚纭彁鍙栧拰鎽樿銆?- 鏃?README: `kernel/` 涓嬫棤 README 鏃讹紝浠庢牳蹇冨ご鏂囦欢鎻愬彇娉ㄩ噴銆?- 澶存枃浠舵敞閲? `include/linux/sched.h` 寮€澶寸殑娉ㄩ噴鍧楄姝ｇ‘鎻愬彇銆?- 鍥為€€: 鏃㈡棤 README 涔熸棤澶存枃浠舵敞閲婃椂锛岀敓鎴愬熀浜庣洰褰曞悕鐨勭畝瑕佽鏄庛€?- 鏁伴噺: 鑷冲皯鎻愬彇 5 涓瓙绯荤粺鐨勬弿杩般€?
**Verification:**
- 鑷冲皯 5 涓瓙绯荤粺鐨勬弿杩拌鎻愬彇骞跺寘鍚湪杈撳嚭涓€?- 鎻愬彇鐨勬枃鏈暱搴﹀悎鐞嗭紙涓嶈秴杩?500 瀛楃/瀛愮郴缁燂級銆?
---

### U5. 缁勮 Markdown 杈撳嚭骞跺垱寤虹敓鎴愬叆鍙?
**Goal:** 灏嗗洓涓彁鍙栧櫒鐨勮緭鍑虹粍瑁呬负瀹屾暣鐨?`docs/PROJECT.md`锛屽苟鎻愪緵鍛戒护琛屽叆鍙ｃ€?
**Requirements:** R2, R7, R8

**Dependencies:** U1, U2, U3, U4

**Files:**
- Create: `tools/docs/gen-project-md.py`锛堢粍瑁呭拰 CLI 鍏ュ彛锛?- Create: `docs/PROJECT.md`锛堢敓鎴愮殑鐩爣鏂囦欢锛?
**Approach:**
- 瀹氫箟 Markdown 妯℃澘缁撴瀯锛?  ```
  # Linux Kernel Project Overview
  
  ## Directory Structure
  
  ## Kconfig Summary
  
  ## Makefile Targets
  
  ## Subsystem Descriptions
  ```
- 缁勮鍥涗釜鎻愬彇鍣ㄧ殑杈撳嚭锛屾寜妯℃澘缁撴瀯鎺掑垪銆?- 娣诲姞 `if __name__ == "__main__"` 鍏ュ彛锛屾敮鎸佸懡浠よ鍙傛暟锛?  - `--output` 鎸囧畾杈撳嚭鏂囦欢璺緞锛堥粯璁?`docs/PROJECT.md`锛?  - `--srcdir` 鎸囧畾婧愮爜鏍圭洰褰曪紙榛樿褰撳墠鐩綍锛?- 鑴氭湰鍚姩鏃堕獙璇佹簮鐮佹牴鐩綍瀛樺湪锛岃嫢涓嶅瓨鍦ㄥ垯鎶ラ敊閫€鍑恒€?- 鑴氭湰杩愯鍚庢墦鍗扮敓鎴愭憳瑕侊紙鍚勬澘鍧楁潯鐩暟锛夈€?
**Patterns to follow:**
- `tools/docs/kernel-doc` 鐨?CLI 鍏ュ彛妯″紡锛坅rgparse + main锛夈€?- `tools/docs/md-convert.py` 鐨?`if __name__ == "__main__"` 妯″紡銆?
**Test scenarios:**
- Happy path: 鑴氭湰鍦ㄦ簮鐮佹牴鐩綍杩愯锛屾垚鍔熺敓鎴?`docs/PROJECT.md`銆?- 鑷畾涔夎緭鍑? `--output /tmp/test.md` 灏嗚緭鍑哄啓鍏ユ寚瀹氳矾寰勩€?- 鑷畾涔夋簮鐮佺洰褰? `--srcdree /path/to/linux` 浠庢寚瀹氱洰褰曡鍙栨簮鐮併€?- 鍙噸澶嶆€? 杩炵画杩愯涓ゆ锛岃緭鍑烘枃浠跺瓧鑺傜骇涓€鑷淬€?- 闈炴簮鐮佷慨鏀? 鑴氭湰杩愯鏈熼棿涓嶄慨鏀逛换浣曟枃浠躲€?- 鏂囦欢澶у皬: 杈撳嚭鏂囦欢澶у皬鍦ㄥ悎鐞嗚寖鍥村唴锛圞B 绾э紝闈?MB 绾э級銆?
**Verification:**
- `docs/PROJECT.md` 鎴愬姛鐢熸垚锛屽寘鍚洓涓富瑕佹澘鍧椼€?- 鏂囦欢澶у皬鍦?10KB - 500KB 涔嬮棿銆?- 杩炵画杩愯涓ゆ锛岃緭鍑哄畬鍏ㄤ竴鑷达紙`diff` 鏃犲樊寮傦級銆?
---

## System-Wide Impact

- **璋冪敤閾撅細** `python3 tools/docs/gen-project-md.py` 鈫?鐙珛鑴氭湰锛屼笉璋冪敤 Sphinx銆佷笉璋冪敤 kernel-doc銆佷笉淇敼浠讳綍鏋勫缓娴佺▼銆?- **閿欒浼犳挱锛?* 鑴氭湰閬囧埌鏃犳硶瑙ｆ瀽鐨勬枃浠舵椂鎵撳嵃璀﹀憡骞惰烦杩囷紝涓嶇粓姝紙`--strict` 妯″紡鍙€夛紝v1 榛樿瀹芥澗锛夈€?- **鐘舵€佺敓鍛藉懆鏈燂細** `docs/PROJECT.md` 鏄潤鎬佷骇鐗╋紝涓嶅弬涓庡唴鏍告瀯寤烘祦绋嬨€?- **API 琛ㄩ潰锛?* 鏂板涓€涓嫭绔嬭剼鏈拰涓€涓緭鍑烘枃浠讹紝涓嶄慨鏀逛换浣曠幇鏈?API銆佹帴鍙ｆ垨琛屼负銆?- **涓嶅彉绾﹀畾锛?* `Documentation/sphinx/min_requirements.txt`銆乣Documentation/conf.py`銆乣tools/docs/sphinx-build-wrapper`銆乣tools/docs/kernel-doc` 鍧囦笉鍙樸€?
---

## Risks & Dependencies

| Risk | 鍙兘鎬?| 褰卞搷 | 缂撹В鎺柦 |
|---|---|---|---|
| 鍏抽敭瀛愮洰褰曟棤 README/娉ㄩ噴锛岀敤閫旇鏄庢帹鏂笉鍑?| 楂?| 浣?| 浣跨敤鐩綍鍚嶅洖閫€锛涗汉宸ュ鏍稿悗鍙井璋冭剼鏈殑鐩綍鍚嶆槧灏勮〃 |
| Kconfig 璇硶澶嶆潅锛屾鍒欒В鏋愭紡鎻愬彇 | 涓?| 浣?| v1 鍙彁鍙?`config`/`menuconfig` 鏉＄洰锛岃烦杩?`choice`/`comment` 绛夊鏉傜粨鏋勶紱瑕嗙洊 80% 甯哥敤閰嶇疆椤?|
| Makefile 鐩爣鍒嗘暎锛屾彁鍙栦笉鍏?| 涓?| 浣?| v1 鑱氱劍椤跺眰 Makefile 鍜?Documentation/Makefile锛涘瓙鐩綍 Makefile 鐩爣 deferred |
| 杈撳嚭鏂囦欢杩囧ぇ锛圡B 绾э級 | 浣?| 涓?| 鐩綍鏍戞繁搴﹂檺鍒朵负 2 灞傦紱README 鎽樿鎴柇鑷?500 瀛楃/瀛愮郴缁?|
| 杩愯鎬ц兘锛堝叏搴撴壂鎻忔參锛?| 浣?| 浣?| 浠呮壂鎻忓叧閿瓙鐩綍 + 绗竴绾у瓙鐩綍锛屼笉閫掑綊鍏ㄥ簱 |

---

## Documentation / Operational Notes

- 杩愯鏂瑰紡锛歚python3 tools/docs/gen-project-md.py`锛岃緭鍑哄埌 `docs/PROJECT.md`銆?- 鑴氭湰鏃犵涓夋柟渚濊禆锛屼粎浣跨敤 Python 鏍囧噯搴撱€?- `docs/PROJECT.md` 搴斿姞鍏?`.gitignore` 鍚楋紵涓嶁€斺€斿畠鏄粠婧愮爜鐢熸垚鐨勪骇鐗╋紝浣嗕唬琛ㄩ」鐩姸鎬佸揩鐓э紝閫傚悎绾冲叆鐗堟湰鎺у埗锛堢被浼?`Documentation/output/` 鐨勫摬瀛︿絾浣滀负鏂囨。鑰岄潪鏋勫缓浜х墿锛夈€傚缓璁撼鍏ョ増鏈帶鍒讹紝姣忔鑴氭湰鏇存柊鏃堕噸鏂扮敓鎴愩€?
---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md](docs/brainstorms/2026-07-06-project-overview-markdown-requirements.md)
- **鐩稿叧浠ｇ爜锛?* `tools/docs/kernel-doc`銆乣tools/docs/md-convert.py`銆乣tools/lib/python/kdoc/kdoc_output.py`銆乣Kconfig`銆乣Makefile`
