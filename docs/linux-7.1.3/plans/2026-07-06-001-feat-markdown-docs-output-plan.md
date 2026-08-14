---
title: feat: Add Markdown docs output via Pandoc pipeline
type: feat
status: active
date: 2026-07-06
origin: docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md
deepened: 2026-07-06
---

# feat: Add Markdown docs output via Pandoc pipeline

## Summary

鍦ㄧ幇鏈?Sphinx 鏋勫缓娴佹按绾夸箣鍚庝覆鑱?Pandoc 鍚庡鐞嗭紝鏂板 `make markdowndocs` 鐩爣锛屽皢 `Documentation/` 鐨?HTML 杈撳嚭杞崲涓?Markdown锛岃緭鍑哄埌 `Documentation/output/markdown/`銆傝鏂规涓嶅崌绾?Sphinx銆佷笉寮曞叆鏂扮殑 Sphinx 鎵╁睍锛屼繚鎸佸鍐呮牳鑷畾涔?directive 鐨勬渶澶у吋瀹规€с€?
---

## Problem Frame

鍐呮牳鏂囨。浣跨敤 Sphinx 3.4.3 鏋勫缓涓?HTML/PDF/EPUB 绛夋牸寮忥紝浣嗙己涔?Markdown 杈撳嚭璺緞銆備笅娓告秷璐硅€咃紙闈欐€佺珯鐐圭敓鎴愬櫒銆丩LM  ingestion pipeline銆佽法椤圭洰闀滃儚锛夊彧鑳戒緷璧栨槗纰庣殑澶栭儴杞崲鑴氭湰銆傜幇鎴?Sphinx Markdown builder 鍧囦笉鍏煎 Sphinx 3.4.3锛屽洜姝ら噰鐢?Pandoc 鍚庡鐞嗕綔涓?v1 鏂规銆?see origin: docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md)

---

## Requirements

- R1. A new `markdowndocs` target must be added to `Documentation/Makefile` that generates Markdown output for the entire `Documentation/` tree.
- R2. Markdown generation must use a Sphinx Markdown builder extension registered in `Documentation/conf.py`.
  - **Overridden by this plan.** No Sphinx Markdown builder is compatible with Sphinx 3.4.3 + kernel custom extensions. R2 is satisfied by Pandoc post-processing (`tools/docs/md-convert.py`) invoked from `make markdowndocs`, which produces Markdown output through the documentation pipeline without modifying `conf.py`.
- R3. Generated `.md` files must be written to `Documentation/output/` alongside existing builder outputs.
  - **Clarified:** Output is written to `Documentation/output/markdown/`, nested under `BUILDDIR` following the `BUILDDIR/<format>/` convention established by existing builders (`html/`, `pdf/`, `epub/`). This keeps Markdown output isolated from other artifacts and automatically covered by `cleandocs`.
- R4. The new target must support the same `SPHINXDIRS` filtering mechanism as existing targets (e.g., `make SPHINXDIRS=process markdowndocs`).
- R5. The Markdown builder extension and its dependencies must be declared in a Sphinx requirements file under `Documentation/sphinx/`.
  - **Overridden by this plan.** Pandoc is a system-level binary dependency, not a Python/Sphinx package. It cannot be declared in `Documentation/sphinx/min_requirements.txt` or `requirements.txt`. The dependency is declared in this plan's Documentation / Operational Notes and surfaced to users via the conversion script's startup check.
- R6. The new target must integrate with the existing `sphinx-build-wrapper` and version-check flow used by other doc targets.
  - **Partially satisfied.** `markdowndocs` does not call `sphinx-build-wrapper` and does not participate in its `TARGETS` dictionary (Markdown generation is not a `sphinx-build` builder). Integration is limited to: (a) `make markdowndocs` depends on `htmldocs`, which goes through the full wrapper pipeline including `sphinx-pre-install --version-check`; (b) `SPHINXDIRS` and `BUILDDIR` Make variables are passed through to the conversion script.
- R7. Kernel custom Sphinx extensions must be evaluated for Markdown builder compatibility; incompatible extensions must be handled gracefully rather than crashing the build.
- R8. Markdown output must preserve document structure: headings, code blocks, lists, tables, and cross-references where the builder supports them.
- R9. The solution must not modify or remove any existing builder targets or output formats.

**Origin actors:** 鍐呮牳鏂囨。缁存姢鑰呫€佷笅娓?Markdown 娑堣垂鑰?**Origin flows:** F1 鏈湴棰勮 / F2 CI 鏍￠獙 / F3 瀹屾暣鏋勫缓
**Origin acceptance examples:** AE1 (SPHINXDIRS 杩囨护), AE2 (澶?subtree 杈撳嚭), AE3 (CI 楠岃瘉), AE4 (鐜版湁鐩爣闆跺奖鍝?

---

## Scope Boundaries

- 涓嶈縼绉?`.rst` 婧愭枃浠讹紱婧愭枃浠朵繚鎸佷笉鍙樸€?- 涓嶄慨鏀?`Documentation/` 涔嬪鐨勪换浣曞唴鏍告簮鐮併€?- 涓嶆浛鎹㈡垨搴熷純浠讳綍鐜版湁 builder 鐩爣銆?- 涓嶆柊澧為《灞?Makefile 鐩爣锛涘叆鍙ｄ负 `Documentation/Makefile`銆?- 涓嶅崌绾?Sphinx锛涗繚鎸?`Documentation/sphinx/min_requirements.txt` 涓?Sphinx==3.4.3 鐨勯攣瀹氥€?- Pandoc 鏄郴缁熺骇澶栭儴渚濊禆锛屼笉绾冲叆 Python 渚濊禆绠＄悊锛涙渶浣庣増鏈害鏉熷湪 Documentation / Operational Notes 涓０鏄庛€?- `make markdowndocs` 鍦?v1 涓畾浣嶄负寮€鍙戣€呭伐鍏凤紝涓嶅己鍒剁撼鍏ヤ笂娓稿唴鏍?CI锛圕I 闆嗘垚涓?deferred锛夈€?
### Deferred to Follow-Up Work

- `flat-table` 鐨?`colspan`/`rowspan` 鍦?v1 涓繀椤诲鐞嗭紙妫€娴嬪苟璀﹀憡鎴栦繚鐣欏師濮?HTML锛夛紝浣嗘洿澶嶆潅鐨勮〃鏍艰涔夎繕鍘燂紙濡傚皢鍚堝苟鍗曞厓鏍艰浆涓?Markdown 鍒楄〃锛?deferred銆?- 闄?`flat-table` 澶栫殑鍐呮牳鑷畾涔?directive锛坄kernel-doc`銆乣kernel-figure`銆乣kernel-render`銆乣automarkup`銆乣kernel_abi` 绛夛級鐨勬繁搴﹀悗澶勭悊娓呮礂閫昏緫锛岃 v1 杈撳嚭璐ㄩ噺鍐嶈凯浠ｃ€?- 灏?Markdown 杈撳嚭闆嗘垚杩涗笂娓稿唴鏍?CI 娴佹按绾匡紙褰撳墠瀹氫綅涓哄紑鍙戣€呭伐鍏凤級銆?- 鍦?`sphinx-pre-install` 涓负 pandoc 澧炲姞绯荤粺绾т緷璧栨鏌ワ紙v1 鐢辫浆鎹㈣剼鏈洿鎺ユ娴嬪苟鎶ラ敊锛夈€?- 浜ゅ弶寮曠敤閾炬帴鏍煎紡鏍囧噯鍖栵紙site-root-relative vs. relative path 绛栫暐锛夆€斺€斿彇鍐充簬 Markdown 杈撳嚭鏄綔涓虹嫭绔嬬珯鐐硅繕鏄墖娈垫嫾鎺ヤ娇鐢ㄣ€?
---

## Context & Research

### Relevant Code and Patterns

- `Documentation/Makefile` 鈥?鎵€鏈?doc builder 鐩爣鍏变韩涓€涓ā寮忚鍒欙紝閫氳繃 `sphinx-build-wrapper` 璋冪敤 `sphinx-build`銆傛柊澧?`markdowndocs` 闇€瑕佷綔涓虹嫭绔嬬洰鏍囷紝鍏堜緷璧?`htmldocs` 鍐嶄覆鑱?Pandoc銆?- `tools/docs/sphinx-build-wrapper` 鈥?`TARGETS` 瀛楀吀娉ㄥ唽 builder 鍚嶇О涓庤緭鍑哄瓙鐩綍銆俙markdowndocs` 涓嶄娇鐢ㄦ wrapper 鐨?Sphinx 璋冪敤锛屼絾澶嶇敤鍏?`SPHINXDIRS` 杩囨护涓?`BUILDDIR` 绾﹀畾銆?- `tools/docs/sphinx-pre-install` 鈥?渚濊禆妫€鏌ュ叆鍙ｏ紱v1 涓嶅湪鍏朵腑娉ㄥ唽 pandoc锛岀敱杞崲鑴氭湰鑷妫€娴嬨€?- `Documentation/sphinx/min_requirements.txt` 鈥?閿佸畾 Sphinx==3.4.3锛泇1 涓嶅姩銆?
### Institutional Learnings

鏃犵洿鎺ョ浉鍏崇殑 `docs/solutions/` 鏉＄洰銆傚閮ㄨ皟鐮旂‘璁わ細鎴嚦 2026 骞达紝Sphinx 鐢熸€佷腑鍞竴娲昏穬鐨?Markdown 杈撳嚭 builder锛坄sphinx-markdown-builder` 0.6.x锛夎姹?Sphinx 鈮?7.3锛屼笌鍐呮牳褰撳墠 3.4.3 涓嶅吋瀹癸紱鍏朵綑娴佽鎵╁睍锛坄myst-parser`銆乣m2r2`锛夊潎涓?Markdown **杈撳叆** 瑙ｆ瀽鍣紝涓嶉€傜敤銆?
### External References

- `sphinx-markdown-builder` GitHub issue #48锛氭渶浣?Sphinx 鐗堟湰瑕佹眰 7.3銆?- `sphinx-markdown-builder` GitHub issue #32锛欳 domain 涓嶅吋瀹癸紙瀵瑰唴鏍告枃妗ｈ嚧鍛斤級銆?- Pandoc 瀹樻柟鏂囨。锛歚pandoc -f html -t markdown` 杞崲绠＄嚎銆?
---

## Key Technical Decisions

- **Pandoc 鍚庡鐞嗙绾胯€岄潪 Sphinx 鎵╁睍銆?* 璋冪爺纭鏃犵幇鎴?Sphinx Markdown builder 鍏煎 Sphinx 3.4.3 + 鍐呮牳鑷畾涔夋墿灞曘€侾andoc 鏄敮涓€鑳藉湪涓嶅崌绾?Sphinx 鐨勫墠鎻愪笅浜у嚭鍙敤 Markdown 鐨勬柟妗堛€?- **鑷畾涔?docutils writer 浠嶈鎷掔粷銆?* 闇€姹傛枃妗ｇ殑 Key Decisions 涓褰曚簡涓夌鏂规锛圥andoc銆丼phinx 鎵╁睍銆佽嚜瀹氫箟 docutils writer锛夈€傝嚜瀹氫箟 writer 鐨勫疄鐜板拰缁存姢鎴愭湰锛?00鈥?00 琛岄渶闅?Sphinx/docutils API 婕旇繘鑰岀淮鎶わ級 disproportionate 浜庡叾鏀剁泭锛孭andoc 鍚庡鐞嗗湪鎴愭湰鍜屽吋瀹规€т箣闂存洿骞宠　銆?- **`markdowndocs` 浣滀负鐙珛 Makefile 鐩爣锛屼緷璧?`htmldocs`銆?* 澶嶇敤鐜版湁 HTML 鏋勫缓鍏ㄦ祦绋嬶紙鍖呮嫭 `sphinx-pre-install` 鐗堟湰妫€鏌ャ€乣SPHINXDIRS` 杩囨护銆丷ust doc 闆嗘垚锛夛紝Pandoc 浠呭湪 HTML 灏辩华鍚庝綔涓哄悗澶勭悊姝ラ杩愯銆?- **Markdown 杈撳嚭鐩綍涓?`Documentation/output/markdown/`銆?* 閲囩敤 `BUILDDIR/<format>/` 宓屽绾﹀畾锛堜笌 `html/`銆乣pdf/`銆乣epub/` 涓€鑷达級锛屼繚鎸佽緭鍑洪殧绂伙紝鑷姩琚?`cleandocs` 瑕嗙洊銆傛牴绾у埆 `Documentation/output/` 琚帓闄わ紝浠ラ伩鍏嶄笌 Sphinx 鍐呴儴浜х墿锛坄.doctrees/`銆乣<sphinxdir>/` 瀛愮洰褰曪級纰版挒銆?- **R2/R5/R6 琚柟妗堣鐩栥€?* 闇€姹傛枃妗ｄ腑 R2锛圫phinx 鎵╁睍娉ㄥ唽锛夈€丷5锛圫phinx 渚濊禆鏂囦欢澹版槑锛夊拰 R6锛坰phinx-build-wrapper 娣卞害闆嗘垚锛夊湪 Pandoc 鏋舵瀯涓嬫棤娉曟寜瀛楅潰婊¤冻銆傛湰璁″垝鏄惧紡瑕嗙洊杩欎笁鏉￠渶姹傦細鍔熻兘鎰忓浘鐢?Pandoc 鍚庡鐞嗘浛浠ｏ紝渚濊禆澹版槑鏀逛负鑴氭湰鍚姩妫€鏌?+ 鏈枃妗ｇ殑 Documentation / Operational Notes銆?- **Sphinx 鍗囩骇杩佺Щ瑙﹀彂鍣ㄣ€?* 褰撳唴鏍告枃妗ｆ瀯寤哄皢 Sphinx 鍗囩骇鍒?鈮?7.3 鏃讹紝搴旈噸鏂拌瘎浼版浛鎹负鍘熺敓 `sphinx-markdown-builder`銆傝縼绉绘椂锛?a) `md-convert.py` 涓?v1 绱Н鐨勫悗澶勭悊瑙勫垯闇€璇勪及鏄惁鍙Щ妞嶅埌 builder 鐨勬墿灞曢挬瀛愶紱(b) `markdowndocs` Makefile 鐩爣缁撴瀯鍙兘绠€鍖栥€?- **v1 鍐呮牳 directive 鍚庡鐞嗚寖鍥淬€?* `flat-table` 鐨?`colspan`/`rowspan` 鏄?v1 姝ｇ‘鎬ц姹傦紙Pandoc 鐨?Markdown 琛ㄦ牸璇硶涓嶆敮鎸佽繖浜涘睘鎬э級锛屽繀椤诲湪杞崲鑴氭湰涓鐞嗭紙妫€娴嬪苟璀﹀憡鎴栦繚鐣欎负鍘熷 HTML锛夈€傚叾浣?directive锛坄kernel-doc`銆乣kernel-figure`銆乣kernel-render`锛夌殑鍚庡鐞嗘竻娲楀湪 v1 涓渶灏忓寲瀹炵幇锛岃杈撳嚭璐ㄩ噺鍐嶈凯浠ｃ€?
---

## Open Questions

### Resolved During Planning

- **Markdown builder 閫夊瀷锛?* 璋冪爺纭鏃犵幇鎴愭墿灞曞吋瀹?Sphinx 3.4.3锛岄€夊畾 Pandoc 鍚庡鐞嗐€?- **杈撳嚭鐩綍浣嶇疆锛?* 閫夊畾 `BUILDDIR/markdown/`锛岄伒寰?`BUILDDIR/<format>/` 宓屽绾﹀畾銆?- **flat-table 澶勭悊绛栫暐锛?* v1 妫€娴?`colspan`/`rowspan` 骞朵繚鐣欏師濮?HTML锛圥andoc 鐨?Markdown 琛ㄦ牸璇硶涓嶆敮鎸佸悎骞跺崟鍏冩牸锛夛紝閬垮厤闈欓粯鎹熷潖銆?- **Pandoc 鏈€浣庣増鏈細** 閿佸畾 2.17+锛岀敱 `md-convert.py` 鍚姩鏃舵鏌ャ€?- **R2/R5/R6 瑕嗙洊锛?* 闇€姹傛枃妗ｄ腑 R2锛圫phinx 鎵╁睍娉ㄥ唽锛夈€丷5锛圫phinx 渚濊禆鏂囦欢澹版槑锛夈€丷6锛坰phinx-build-wrapper 娣卞害闆嗘垚锛夊湪 Pandoc 鏋舵瀯涓嬫棤娉曟寜瀛楅潰婊¤冻锛屽凡鍦?Requirements 鍜?Key Technical Decisions 涓樉寮忚鐩栥€?
### Deferred to Implementation

- 浜ゅ弶寮曠敤閾炬帴鏍煎紡鏍囧噯鍖栵紙site-root-relative vs. relative path 绛栫暐锛夆€斺€斿彇鍐充簬 Markdown 杈撳嚭鏄綔涓虹嫭绔嬬珯鐐硅繕鏄墖娈垫嫾鎺ヤ娇鐢ㄣ€?- 闄?`flat-table` 澶栫殑鍐呮牳鑷畾涔?directive锛坄kernel-doc`銆乣kernel-figure`銆乣kernel-render`銆乣automarkup`銆乣kernel_abi` 绛夛級鐨勬繁搴﹀悗澶勭悊娓呮礂閫昏緫锛岃 v1 杈撳嚭璐ㄩ噺鍐嶈凯浠ｃ€?- `conf.py` 涓叏閮?13 涓墿灞曠殑 HTML 杈撳嚭妯″紡瀹¤鈥斺€斿疄鐜版椂闇€閫愭墿灞曟鏌ユ槸鍚︽湁鏈鏈熺殑 HTML 缁撴瀯闇€瑕佸悗澶勭悊銆?
---

## Implementation Units

### U1. 娣诲姞 `markdowndocs` Makefile 鐩爣

**Goal:** 鍦?`Documentation/Makefile` 涓柊澧?`markdowndocs` 鐩爣锛屼綔涓?`htmldocs` 鐨勪緷璧栦覆鑱?Pandoc 杞崲姝ラ銆?
**Requirements:** R1, R3, R4, R6, R9

**Dependencies:** 鏃?
**Files:**
- Modify: `Documentation/Makefile`

**Approach:**
- 鍦ㄧ幇鏈夊叡浜ā寮忚鍒欎箣澶栵紝鏂板 `markdowndocs` 鐩爣锛屼互 `htmldocs` 涓哄厛鍐虫潯浠躲€?- `htmldocs` 瀹屾垚鍚庯紝璋冪敤 `tools/docs/md-convert.py` 灏?HTML 杈撳嚭杞崲涓?Markdown銆?- 鐩爣鏀寔 `SPHINXDIRS` 鍜?`BUILDDIR` 鍙橀噺閫忎紶锛岃涓轰笌鐜版湁鐩爣涓€鑷淬€?- 鐩爣涓嶅弬涓?`sphinx-build-wrapper` 鐨?`TARGETS` 瀛楀吀锛屽洜涓?Markdown 鐢熸垚涓嶇粡杩?`sphinx-build`銆?
**Patterns to follow:**
- 鐜版湁 `htmldocs-redirects` 鍜?`refcheckdocs` 鐩爣浣滀负鐙珛 Makefile 瑙勫垯鐨勫弬鑰冿紙瀹冧滑涔熶笉璧?`sphinx-build-wrapper`锛夈€?- `cleandocs` 鑷姩瑕嗙洊 `BUILDDIR/markdown/`锛屾棤闇€棰濆娓呯悊瑙勫垯銆?
**Test scenarios:**
- Happy path: `make markdowndocs` 鍦?`Documentation/output/markdown/` 涓嬬敓鎴?`.md` 鏂囦欢锛屽唴瀹逛负鏈夋晥 Markdown銆?- SPHINXDIRS 杩囨护: `make SPHINXDIRS=process markdowndocs` 浠呰浆鎹?`process/` 瀛愭爲銆?- 鏃犲壇浣滅敤: `make htmldocs` 鍜?`make pdfdocs` 鐨勮緭鍑轰笌鏈坊鍔?`markdowndocs` 鍓嶅畬鍏ㄤ竴鑷淬€?
**Verification:**
- `make markdowndocs` 閫€鍑虹爜涓?0锛宍Documentation/output/markdown/` 涓嬪瓨鍦?`.md` 鏂囦欢銆?- 瀵?`process/`銆乣admin-guide/`銆乣core-api/`銆乣driver-api/` 鐨勪唬琛ㄦ€ф枃浠舵娊鏍凤紝纭鏍囬銆佷唬鐮佸潡銆佸垪琛ㄣ€佽〃鏍煎湪 Markdown 涓彲璇嗗埆銆?
---

### U2. 鍒涘缓 Pandoc 杞崲鑴氭湰

**Goal:** 缂栧啓 `tools/docs/md-convert.py`锛岄亶鍘?HTML 杈撳嚭鐩綍锛岃皟鐢?Pandoc 灏嗘瘡涓?`.html` 鏂囦欢杞崲涓?`.md`锛屼繚鎸佺洰褰曠粨鏋勶紝骞跺仛鍐呮牳鏂囨。鐗规湁鐨勫悗澶勭悊銆?
**Requirements:** R1, R3, R4, R7, R8, R9

**Dependencies:** U1

**Files:**
- Create: `tools/docs/md-convert.py`

**Approach:**
- 鑴氭湰鎺ユ敹 `--htmldir`锛圚TML 杈撳嚭鏍圭洰褰曪級鍜?`--outdir`锛圡arkdown 杈撳嚭鏍圭洰褰曪級鍙傛暟銆?- **BUILDDIR 宓屽缁撴瀯锛?* Sphinx HTML 杈撳嚭浣嶄簬 `BUILDDIR/<sphinxdir>/html/`锛堢敱 `sphinx-build-wrapper` 绗?737 琛岃瀹氾級锛岃€岄潪鎵佸钩鐨?`BUILDDIR/html/`銆傝剼鏈繀椤婚€掑綊閬嶅巻姝ゅ祵濂楃粨鏋勶紝淇濇寔鐩稿璺緞鏄犲皠鍒?Markdown 杈撳嚭鐩綍銆?- `BUILDDIR` 鍙兘涓虹粷瀵硅矾寰勶紙褰?`make O=build` 鏃?`$(obj)` 瑙ｆ瀽涓虹粷瀵硅矾寰勶級锛岃剼鏈殑 `--htmldir` 蹇呴』鎺ュ彈缁濆璺緞銆?- 瀵规瘡涓?`.html` 鏂囦欢锛?  1. 璁＄畻瀵瑰簲鐨?`.md` 杈撳嚭璺緞锛屼繚鎸佺浉瀵圭洰褰曠粨鏋勩€?  2. 璋冪敤 `pandoc -f html -t markdown --wrap=none` 杩涜杞崲銆?  3. 瀵硅緭鍑哄仛浠ヤ笅鍏蜂綋鍚庡鐞嗭紙Python 瀛楃涓叉搷浣滐級锛?     - **Sphinx shell  stripping锛?* 绉婚櫎 `<div class="related" role="navigation">`銆乣<div class="header">`銆乣<div class="footer">`銆乣<div class="document">` 澶栧眰鍖呰锛屼繚鐣?`<div class="body">` 鍐呯殑姝ｆ枃鍐呭銆?     - **headerlink 娓呯悊锛?* 绉婚櫎 `<a class="headerlink" href="#anchor">露</a>` Permalink 閾炬帴銆?     - **浜ゅ弶寮曠敤閲嶅啓锛?* 灏?`<a class="reference internal" href="../subdir/page.html#anchor">` 涓殑 `.html` 鍚庣紑鏇挎崲涓?`.md`锛岀浉瀵硅矾寰勬牴鎹緭鍑虹洰褰曞眰绾ч噸绠楋紙璺?subtree 寮曠敤濡?`../core-api/...` 闇€璋冩暣涓?`../../core-api/...` 鎴栫粺涓€涓?site-root-relative锛夈€?     - **flat-table 鍚堝苟鍗曞厓鏍煎鐞嗭細** 妫€娴?`<table>` 涓殑 `colspan` 鎴?`rowspan` 灞炴€с€侾andoc 鐨?Markdown 琛ㄦ牸璇硶涓嶆敮鎸佽繖浜涘睘鎬э紝杞崲浼氫骇鐢熸崯鍧忚緭鍑恒€倂1 澶勭悊绛栫暐锛氭娴嬪埌 `colspan`/`rowspan` 鏃讹紝鎵撳嵃璀﹀憡骞朵繚鐣欒琛ㄦ牸涓哄師濮?HTML锛堜笉杞崲锛夛紝閬垮厤闈欓粯鎹熷潖銆?     - **kernel-doc 浠ｇ爜鍧楅獙璇侊細** 纭 `kerneldoc.py` 娓叉煋鍚庣殑 `<pre>` 鍧楀拰 `<dl>/<dt>/<dd>` 鍙傛暟鍒楄〃琚?Pandoc 姝ｇ‘杞负 fenced code block 鍜屽畾涔夊垪琛紱`.. LINENO` 娉ㄩ噴鍦?`parse_msg()` 闃舵宸茶鍓ョ锛屼笉浼氬嚭鐜板湪 HTML 涓紝鏃犻渶棰濆澶勭悊銆?     - **kfigure 鍥惧儚璺緞閲嶅啓锛?* `kfigure.py` 娓叉煋鐨?DOT/SVG 鍥惧儚浣嶄簬 `_static/` 鎴?`_images/` 瀛愮洰褰曘€侻arkdown 杈撳嚭涓殑 `<img src="...">` 璺緞闇€閲嶅啓涓虹浉瀵逛簬 Markdown 鏂囦欢鐨勪綅缃紝鎴栧鍒跺浘鍍忓埌 Markdown 杈撳嚭鐩綍鐨勫搴斾綅缃€?     - **Rust doc 鎺掗櫎锛?* 褰?`CONFIG_RUST=y` 鏃讹紝`sphinx-build-wrapper` 浼氬湪 `BUILDDIR` 涓敓鎴?Rust 鏂囨。 HTML銆傝浆鎹㈣剼鏈繀椤昏烦杩?`rust/` 鎴?`rustdoc/` 瀛愮洰褰曪紙鍏蜂綋鐩綍鍚嶉渶鍦ㄥ疄鐜版椂纭锛夛紝涓嶅皾璇曡浆鎹?Rust 鐢熸垚鐨勯〉闈€?- 鑴氭湰鍚姩鏃舵鏌?`pandoc` 鏄惁鍦?PATH 涓紝鑻ョ己澶卞垯鎵撳嵃娓呮櫚鐨勫畨瑁呮彁绀哄苟閫€鍑?1銆?- 鑴氭湰鍚屾椂妫€鏌?`pandoc --version` 杈撳嚭锛岄獙璇佺増鏈笉浣庝簬 2.17锛堣鐗堟湰绋冲畾鏀寔 `--wrap=none` 鍜?HTML 杈撳叆澶勭悊锛夈€?- 鑴氭湰涓嶄緷璧栦换浣曠涓夋柟 Python 搴擄紙浠呮爣鍑嗗簱 + subprocess锛夛紝閬垮厤鏂板 Python 渚濊禆銆?
**Patterns to follow:**
- `tools/docs/sphinx-build-wrapper` 鐨勭洰褰曢亶鍘嗗拰瀛愯繘绋嬭皟鐢ㄩ鏍笺€?- `tools/docs/sphinx-pre-install` 鐨勪緷璧栨鏌ヤ笌閿欒鎻愮ず椋庢牸銆?
**Execution note:** 鍏堢敤 `process/subprocess.rst` 鍜?`admin-guide/kernel-parameters.rst` 绛変唬琛ㄦ枃浠舵墜鍔ㄩ獙璇?Pandoc 杈撳嚭璐ㄩ噺锛岀‘璁ゆ爣棰樺眰绾с€佷唬鐮佸潡銆佽〃鏍笺€佷氦鍙夊紩鐢ㄥ湪杞崲鍚庡彲鎺ュ彈锛屽啀鍥哄畾鍚庡鐞嗚鍒欍€?
**Test scenarios:**
- Happy path: HTML 鏂囦欢鎴愬姛杞崲涓虹粨鏋勫搴旂殑 Markdown 鏂囦欢銆?- 缂哄け pandoc: 鑴氭湰妫€娴嬪埌 pandoc 涓嶅湪 PATH 鏃舵墦鍗板畨瑁呮彁绀哄苟閫€鍑?1銆?- 宓屽鐩綍: `BUILDDIR/process/subdir/page.html`锛堜綅浜?`BUILDDIR/<sphinxdir>/html/` 宓屽缁撴瀯涓級姝ｇ‘杞崲涓?`BUILDDIR/markdown/process/subdir/page.md`銆?- 浜ゅ弶寮曠敤: Sphinx 鐢熸垚鐨勫唴閮ㄩ摼鎺ワ紙`<a class="reference internal" href="../subdir/page.html#anchor">`锛夊湪 Markdown 涓彉涓?`[text](subdir/page.md#anchor)`锛岀浉瀵硅矾寰勬纭€?- headerlink 娓呯悊: 椤甸潰涓殑 `露` Permalink 閾炬帴鍦?Markdown 杈撳嚭涓笉瀛樺湪銆?- flat-table 鍚堝苟鍗曞厓鏍? 鍖呭惈 `colspan` 鎴?`rowspan` 鐨?`<table>` 瑙﹀彂鑴氭湰璀﹀憡锛岃琛ㄦ牸淇濈暀涓哄師濮?HTML 鍧楄€岄潪鎹熷潖鐨?Markdown 琛ㄦ牸銆?- Rust doc 鎺掗櫎: 褰?`BUILDDIR` 涓寘鍚?Rust 鐢熸垚鐨?HTML 瀛愮洰褰曟椂锛岃剼鏈烦杩囪鐩綍锛屼笉浜х敓瀵瑰簲鐨?Markdown 鏂囦欢銆?- 缁濆璺緞: `--htmldir` 鎺ユ敹缁濆璺緞锛堝 `D:/WORKSPACE/linux-7.1.3/Documentation/output/html`锛夋椂姝ｅ父杞崲銆?- Pandoc 鐗堟湰妫€鏌? 鑴氭湰妫€娴嬪埌 pandoc 鐗堟湰浣庝簬 2.17 鏃舵墦鍗拌鍛婂苟閫€鍑?1銆?
**Verification:**
- 瀵逛唬琛ㄦ€ф枃妗ｆ娊鏍锋鏌ヨ浆鎹㈠悗鐨?Markdown锛氭爣棰樺眰绾ф纭€佷唬鐮佸潡浣跨敤 fenced 璇硶銆佽〃鏍煎彲璇嗗埆銆佷氦鍙夊紩鐢ㄩ摼鎺ュ彲鐐瑰嚮銆?- `make markdowndocs` 鏁翠綋娴佺▼閫€鍑虹爜涓?0銆?
---

### U3. 纭繚鐜版湁鐩爣闆跺奖鍝嶅苟琛ュ厖浣跨敤鏂囨。

**Goal:** 楠岃瘉 `make htmldocs`銆乣make pdfdocs`銆乣make cleandocs` 绛夌幇鏈夌洰鏍囩殑琛屼负瀹屽叏涓嶅彉锛涘湪 `AGENTS.md` 涓褰?Markdown 鏋勫缓鍛戒护銆?
**Requirements:** R9

**Dependencies:** U1, U2

**Files:**
- Modify: `AGENTS.md`锛堟柊澧?Markdown 鏋勫缓鍛戒护鏉＄洰锛?- 鏃犳柊鏂囦欢鍒涘缓

**Approach:**
- 杩愯 `make htmldocs`銆乣make pdfdocs`銆乣make cleandocs`锛岀‘璁よ緭鍑轰笌 baseline 涓€鑷淬€?- 鍦?`AGENTS.md` 鐨?Build 鎴?Lint 娈佃惤涓坊鍔?`make markdowndocs` 鍛戒护璇存槑銆?
**Test scenarios:**
- `make htmldocs` 杈撳嚭涓?v1 鍙樻洿鍓嶅畬鍏ㄤ竴鑷淬€?- `make cleandocs` 鍒犻櫎 `Documentation/output/` 鍏ㄩ儴鍐呭锛堝惈鏂板鐨?`markdown/` 瀛愮洰褰曪級銆?- `make markdowndocs` 鍙湪骞插噣婧愮爜鏍戜笂鐙珛杩愯锛堝厛鑷姩瑙﹀彂 `htmldocs` 鏋勫缓锛夈€?- SPHINXDIRS 闅旂: `make SPHINXDIRS=process markdowndocs` 浠呭湪 `Documentation/output/markdown/process/` 涓嬬敓鎴?`.md` 鏂囦欢锛屼笉浜х敓鍏朵粬 subtree 鐨?Markdown 杈撳嚭銆?- HTML 杈撳嚭涓嶅彉鎬? `make markdowndocs` 瀹屾垚鍚庯紝`BUILDDIR/<sphinxdir>/html/` 涓嬬殑 `.html` 鏂囦欢鍐呭涓庤繍琛屽墠瀹屽叏涓€鑷达紙閫氳繃 checksum 鎴?mtime 楠岃瘉锛夈€?- 骞傜瓑鎬? 杩炵画杩愯 `make markdowndocs` 涓ゆ锛岀浜屾鐢熸垚鐨?`.md` 鏂囦欢涓庣涓€娆″瓧鑺傜骇涓€鑷淬€?- BUILDDIR 瑕嗙洊: `make BUILDDIR=/tmp/test-build markdowndocs` 姝ｇ‘灏?HTML 杈撳嚭璇昏嚜 `/tmp/test-build/<sphinxdir>/html/`锛孧arkdown 杈撳嚭鍐欒嚦 `/tmp/test-build/markdown/`銆?- 閮ㄥ垎澶辫触娓呯悊: 鑻?Pandoc 瀵规煇涓枃浠惰繑鍥為潪闆堕€€鍑虹爜锛岃剼鏈墦鍗板け璐ユ枃浠惰矾寰勫悗閫€鍑?1锛屽凡鐢熸垚鐨?`.md` 鏂囦欢淇濈暀锛堜笉娓呯悊锛夛紝渚夸簬璋冭瘯銆?- Rust doc 鍏卞瓨: 褰?`CONFIG_RUST=y` 涓?`BUILDDIR` 涓寘鍚?Rust 鐢熸垚鐨?HTML 鏃讹紝`make markdowndocs` 姝ｅ父瀹屾垚锛屼笉灏濊瘯杞崲 Rust 椤甸潰銆?
**Verification:**
- 鐜版湁鐩爣鐨?Makefile 瑙勫垯鏈淇敼锛涗粎鏂板鐙珛鐩爣锛岄浂渚靛叆銆?- `AGENTS.md` 鍖呭惈 `make markdowndocs` 鍛戒护銆?
---

## System-Wide Impact

- **璋冪敤閾撅細** `make markdowndocs` 鈫?`make htmldocs`锛圫phinx HTML 鏋勫缓锛夆啋 `tools/docs/md-convert.py`锛圥andoc 杞崲锛夈€侶TML 鏋勫缓璺緞涓庣幇鏈夌洰鏍囧畬鍏ㄥ叡浜紝鏃犲垎鏀€?- **閿欒浼犳挱锛?* Pandoc 杞崲澶辫触锛堥潪闆堕€€鍑虹爜锛夋椂锛宍md-convert.py` 鎹曡幏寮傚父骞舵墦鍗版枃浠惰矾寰勫悗閫€鍑?1锛孧ake 缁堟銆係phinx 鏋勫缓闃舵鐨勯敊璇敱鐜版湁鏈哄埗澶勭悊锛屼笉鍙樸€?- **鐘舵€佺敓鍛藉懆鏈燂細** Markdown 杈撳嚭浣嶄簬 `BUILDDIR/markdown/`锛岀敱 `cleandocs` 缁熶竴娓呯悊锛屾棤娈嬬暀銆?- **API 琛ㄩ潰锛?* 浠呮柊澧炰竴涓?Make 鐩爣鍜屼竴涓?Python 鑴氭湰锛屼笉淇敼浠讳綍鐜版湁 API銆佹帴鍙ｆ垨琛屼负銆?- **涓嶅彉绾﹀畾锛?* `Documentation/sphinx/min_requirements.txt` 涓?Sphinx==3.4.3 閿佸畾涓嶅彉锛沗Documentation/conf.py` 涓?extensions 鍒楄〃涓嶅彉锛沗tools/docs/sphinx-build-wrapper` 鐨?`TARGETS` 瀛楀吀涓嶅彉銆?
---

## Risks & Dependencies

| Risk | 鍙兘鎬?| 褰卞搷 | 缂撹В鎺柦 |
|---|---|---|---|
| Pandoc 瀵瑰唴鏍歌嚜瀹氫箟 directive 鐨?HTML 杈撳嚭杞崲璐ㄩ噺涓嶄匠 | 涓?| 涓?| v1 鐢ㄤ唬琛ㄦ枃浠舵墜鍔ㄩ獙璇侊紱璐ㄩ噺涓嶈冻鏃堕€氳繃鍚庡鐞嗚剼鏈竻娲楋紝涓嶉樆濉炲彂甯?|
| 閮ㄥ垎绯荤粺鐜鏃?Pandoc | 涓?| 浣?| 杞崲鑴氭湰妫€娴嬬己澶卞苟鎵撳嵃瀹夎鎻愮ず锛涗笉褰卞搷 htmldocs/pdfdocs |
| 浜ゅ弶寮曠敤杞崲鍚庨摼鎺ユ牸寮忎笉缁熶竴 | 涓?| 浣?| v1 鍏堣緭鍑哄師濮?Pandoc 缁撴灉锛屾寜瀹為檯鏍煎紡鍐冲畾鏄惁闇€鏍囧噯鍖?|
| Pandoc 鐗堟湰宸紓瀵艰嚧杈撳嚭涓嶄竴鑷?| 浣?| 浣?| 鑴氭湰鍚姩鏃舵鏌?Pandoc >= 2.17锛屼綆浜庢鐗堟湰鐩存帴鎶ラ敊閫€鍑?|
| 鍐呮牳 CI 鐜鏈瑁?Pandoc | 涓?| 浣?| v1 瀹氫綅涓哄紑鍙戣€呭伐鍏凤紝涓嶅己鍒剁撼鍏ヤ笂娓?CI锛涘悗缁闇€ CI 闆嗘垚鍐嶈瘎浼?|
| `colspan`/`rowspan` 琛ㄦ牸闈欓粯鎹熷潖 | 涓?| 涓?| v1 妫€娴嬪埌鍚堝苟鍗曞厓鏍兼椂璀﹀憡骞朵繚鐣欏師濮?HTML锛岄伩鍏?Markdown 琛ㄦ牸鎹熷潖 |

---

## Documentation / Operational Notes

- 鐢ㄦ埛鍦ㄨ繍琛?`make markdowndocs` 鍓嶉渶纭繚绯荤粺宸插畨瑁?Pandoc >= 2.17锛坄apt install pandoc` / `dnf install pandoc` / `brew install pandoc`锛夈€傝浆鎹㈣剼鏈惎鍔ㄦ椂鑷姩妫€鏌ョ増鏈紝浣庝簬 2.17 鏃舵墦鍗板畨瑁呮彁绀哄苟閫€鍑?1銆?- Markdown 杈撳嚭涓?HTML 鐨勫悗澶勭悊浜х墿锛屼笉浠ｈ〃婧愭枃浠剁殑鏉冨▉鐗堟湰锛涙墍鏈夋枃妗ｅ彉鏇翠粛闇€鍦?`.rst` 婧愭枃浠朵腑杩涜銆?- v1 瀹氫綅涓哄紑鍙戣€呭伐鍏凤紝涓嶅己鍒剁撼鍏ヤ笂娓稿唴鏍?CI銆傝嫢鍚庣画闇€瑕?CI 闆嗘垚锛孭andoc 闇€娣诲姞鍒?CI 闀滃儚鐨?provisioning 姝ラ銆?- **杩佺Щ瑙﹀彂鍣細** 褰撳唴鏍告枃妗ｆ瀯寤哄皢 Sphinx 鍗囩骇鍒?鈮?7.3 鏃讹紝搴旈噸鏂拌瘎浼版浛鎹负鍘熺敓 `sphinx-markdown-builder`銆傝縼绉绘椂锛?a) `md-convert.py` 涓?v1 绱Н鐨勫悗澶勭悊瑙勫垯闇€璇勪及鏄惁鍙Щ妞嶅埌 builder 鐨勬墿灞曢挬瀛愶紱(b) `markdowndocs` Makefile 鐩爣缁撴瀯鍙兘绠€鍖栥€侾andoc 璺緞鏄椂闂撮檺瀹氱殑妗ユ帴鏂规锛屼笉搴旀案涔呭浐鍖栥€?
---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md](docs/brainstorms/2026-07-06-markdown-docs-output-requirements.md)
- **Sphinx Markdown builder 鍏煎鎬ц皟鐮旓細** `sphinx-markdown-builder` issue #48锛圫phinx 鈮?7.3 瑕佹眰锛夈€乮ssue #32锛圕 domain 涓嶅吋瀹癸級
- **鐩稿叧浠ｇ爜锛?* `Documentation/Makefile`銆乣tools/docs/sphinx-build-wrapper`銆乣tools/docs/sphinx-pre-install`銆乣Documentation/sphinx/min_requirements.txt`銆乣Documentation/conf.py`
