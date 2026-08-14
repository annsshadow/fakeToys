---
title: feat: Kernel full-codebase branch coverage 鈮?0%
type: feat
status: active
date: 2026-07-07
origin: docs/brainstorms/kernel-branch-coverage-90-requirements.md
---

# feat: Kernel Full-Codebase Branch Coverage 鈮?0%

## Summary

寤虹珛绯荤粺鍖栫殑 Linux 鍐呮牳鍒嗘敮瑕嗙洊鐜囨祴璇曞伐绋嬶紝瑕嗙洊鍏ㄩ儴浠ｇ爜鏍戯紙`kernel/`銆乣mm/`銆乣fs/`銆乣net/`銆乣drivers/`銆乣arch/*/`銆乣lib/`銆乣include/`锛夛紝閫氳繃缁勫悎 KUnit銆乲selftest銆乻yzkaller 鍜?Fault Injection 澶氫釜娴嬭瘯寮曟搸锛屾寜瀛愮郴缁熷垎灞傛瀯寤猴紝鏈€缁堣揪鍒板璁″彲鎺ュ彈鐨勫垎鏀鐩栫巼 90%+銆?
---

## Problem Frame

鍚堣瀹¤鏂硅姹?Linux 鍐呮牳椤圭洰鎻愪緵鍒嗘敮瑕嗙洊鐜?鈮?0% 鐨勮瘉鏄庯紝瑕嗙洊鍏ㄩ儴浠ｇ爜鏍戙€傚綋鍓嶅唴鏍哥幇鏈夋祴璇曪紙KUnit + kselftest锛変粎瑕嗙洊绾?15-25% 鐨勮瑕嗙洊鐜囷紝鍒嗘敮瑕嗙洊鐜囨洿浣庛€傚璁℃柟鐞嗚В鍐呮牳瑙勬ā锛屼笉鎺ュ彈璞佸厤鎴栨浛浠ｆ寚鏍囥€傞」鐩湁棰勭畻锛岄渶瑕佸缓绔嬪彲閲嶅銆佸彲瀹¤鐨勬祴璇曞伐绋嬨€?
---

## Requirements

**Origin actors:** A1 (鍚堣瀹¤鏂?, A2 (鍐呮牳娴嬭瘯宸ョ▼甯?, A3 (鍐呮牳寮€鍙戣€?, A4 (鍩虹璁炬柦宸ョ▼甯?, A5 (椤圭洰缁忕悊)
**Origin flows:** F1 (瑕嗙洊鐜囨祴閲忔祦姘寸嚎), F2 (瀛愮郴缁熸祴璇曞紑鍙?, F3 (瀹¤妫€鏌ョ偣), F4 (鍥炲綊闃叉姢)
**Origin acceptance examples:** AE1 (Covers R1, R2), AE2 (Covers R3), AE3 (Covers R9), AE4 (Covers R14)

- R1. 寤虹珛缁熶竴鐨勮鐩栫巼閲囬泦娴佹按绾匡紝鏀寔 gcov 鍜?kcov 涓ょ宸ュ叿锛岃緭鍑烘爣鍑嗘牸寮忕殑瑕嗙洊鐜囨姤鍛婏紙lcov/html 鎴栫瓑鏁堟牸寮忥級
- R2. 瑕嗙洊鐜囨姤鍛婂繀椤诲寘鍚垎鏀骇鍒矑搴︼紙鍝簺鍒嗘敮琚鐩栥€佸摢浜涙湭瑕嗙洊锛夛紝涓嶅緱浠呮姤鍛婅瑕嗙洊鐜?- R3. 瑕嗙洊鐜囨暟鎹彲澶嶇幇锛氱浉鍚屾祴璇曡緭鍏ュ湪鐩稿悓鐜涓嬭繍琛岋紝瑕嗙洊鐜囨暟瀛楀樊寮備笉瓒呰繃 1%
- R4. 瑕嗙洊鐜囨姤鍛婂彲杩芥函锛氭瘡涓鐩栫巼鏁版嵁鐐瑰叧鑱斿埌鍏蜂綋鐨勬祴璇曠敤渚嬪拰浠ｇ爜鎻愪氦
- R5. 缁熶竴缂栨帓 KUnit銆乲selftest銆乻yzkaller銆丗ault Injection 澶氫釜娴嬭瘯寮曟搸锛屾敮鎸佷竴閿繍琛屽叏閮ㄦ祴璇曞浠?- R6. 姣忎釜娴嬭瘯寮曟搸鐙珛鍙繍琛岋紝鏀寔鍗曠嫭杩愯鐗瑰畾瀛愮郴缁熺殑娴嬭瘯
- R7. 娴嬭瘯澶辫触鏃惰嚜鍔ㄩ噸璇曟満鍒讹紝鍖哄垎鍋跺彂鎬уけ璐ュ拰纭畾鎬уけ璐?- R8. 娴嬭瘯鐜鍙揩閫熼噸寤猴紙浠庨厤缃埌杩愯 鈮?0 鍒嗛挓锛?- R9. 鎸夊瓙绯荤粺椤哄簭鎺ㄨ繘娴嬭瘯瑕嗙洊锛歚kernel/` 鈫?`mm/` 鈫?`fs/` 鈫?`net/` 鈫?`drivers/` 鈫?`arch/*/`锛屾瘡涓瓙绯荤粺杈惧埌 90% 鍒嗘敮瑕嗙洊鐜囧悗杩涘叆涓嬩竴瀛愮郴缁?- R10. 姣忎釜瀛愮郴缁熺殑娴嬭瘯浠ｇ爜閫氳繃浠ｇ爜瀹℃煡鍚庡悎骞讹紝涓嶅緱鍚堝苟鏈揪鏍囩殑娴嬭瘯
- R11. 寤虹珛瑕嗙洊鐜囧洖褰掗槻鎶わ細鏂颁唬鐮佸悎骞朵笉寰楀鑷村凡杈炬爣瀛愮郴缁熺殑瑕嗙洊鐜囦笅闄?- R12. 鐢熸垚瀛ｅ害瀹¤鎶ュ憡锛屽寘鍚暣浣撹鐩栫巼瓒嬪娍銆佸悇瀛愮郴缁熻鐩栫巼銆佹湭瑕嗙洊鍒嗘敮鍒嗘瀽
- R13. 瀹¤鎶ュ憡鍖呭惈娴嬭瘯瀹屾暣鎬ц瘉鏄庯細姣忎釜娴嬭瘯鐢ㄤ緥鐨勬墽琛岃褰曘€佽鐩栫巼璐＄尞銆佸叧鑱旂殑闇€姹?浠ｇ爜璺緞
- R14. 鏀寔瀹¤鏂圭嫭绔嬭繍琛屾祴璇曞浠堕獙璇佽鐩栫巼鏁版嵁锛堝彲閲嶅杩愯鐜锛?
---

## Scope Boundaries

- 浠呭叧娉ㄥ垎鏀鐩栫巼锛屼笉瑕佹眰鍏朵粬瑕嗙洊鐜囨寚鏍囷紙琛岃鐩栫巼銆佸嚱鏁拌鐩栫巼锛夎揪鏍?- 涓嶄慨鏀瑰唴鏍告簮鐮佷互鎻愰珮鍙祴璇曟€э紝闄ら潪娴嬭瘯蹇呴渶涓旂粡杩囪瘎瀹?- 涓嶆瀯寤烘柊鐨勬祴璇曟鏋讹紝澶嶇敤鐜版湁 KUnit銆乲selftest銆乻yzkaller銆丗ault Injection
- 涓嶅寘鎷€ц兘娴嬭瘯銆佸熀鍑嗘祴璇曘€佸畨鍏ㄥ鍚戠殑妯＄硦娴嬭瘯锛坰yzkaller 浠呯敤浜庤鐩栫巼锛?- 涓嶅寘鎷疄鏃惰鐩栫巼浠〃鐩橈紙瀹¤涓嶈姹傦級
- 娴嬭瘯浠ｇ爜鏀剧疆鍦ㄧ幇鏈?`tools/testing/` 鐩綍缁撴瀯涓?
---

## Context & Research

### Relevant Code and Patterns

- `lib/kunit/` 鈥?KUnit 妗嗘灦瀹炵幇锛坱est.c, executor.c, assert.c锛夛紝娴嬭瘯娉ㄥ唽妯″紡涓?`kunit_test_suites()`
- `tools/testing/kunit/kunit.py` 鈥?KUnit Python 杩愯鍣紝鏀寔 UML/QEMU
- `tools/testing/selftests/` 鈥?kselftest 鐩綍锛?23 涓祴璇曞瓙鐩綍锛屼娇鐢?`kselftest_harness.h`
- `kernel/gcov/` 鈥?gcov 瑕嗙洊鐜囬噰闆嗗疄鐜帮紙base.c, fs.c, gcc_4_7.c, clang.c锛?- `lib/Kconfig.debug` 鈥?KCOV 閰嶇疆锛坄CONFIG_KCOV`銆乣CONFIG_KCOV_INSTRUMENT_ALL`锛?- `lib/fault-inject.c` 鈥?Fault Injection 妗嗘灦锛坄should_fail()` / `should_fail_ex()`锛?- `drivers/gpu/drm/ci/kunit.yml` 鈥?GitLab CI KUnit 娴嬭瘯绀轰緥
- `tools/testing/selftests/kselftest/runner.sh` 鈥?kselftest 杩愯鍣?
### Institutional Learnings

鏃犵洿鎺ョ浉鍏崇殑 `docs/solutions/` 鏉＄洰銆?
### External References

- gcov 鏂囨。锛歚Documentation/dev-tools/gcov.rst`
- kcov 鏂囨。锛歚Documentation/dev-tools/kcov.rst`
- Fault Injection 鏂囨。锛歚Documentation/fault-injection/fault-injection.rst`
- KUnit 鏂囨。锛歚Documentation/dev-tools/kunit/index.rst`
- kselftest 鏂囨。锛歚Documentation/dev-tools/kselftest.rst`

---

## Key Technical Decisions

- 鍒嗘敮瑕嗙洊鐜囦綔涓哄敮涓€鎸囨爣锛氬璁℃柟涓嶆帴鍙楄瑕嗙洊鐜囨垨鍑芥暟瑕嗙洊鐜囦綔涓哄厖鍒嗚瘉鎹?(see origin: docs/brainstorms/kernel-branch-coverage-90-requirements.md)
- 鍏ㄤ唬鐮佹爲鏃犺眮鍏嶏細瀹¤鏂圭悊瑙ｅ唴鏍歌妯′絾涓嶆帴鍙楄眮鍏?(see origin)
- 鍒嗗眰鏋勫缓娉曪細鎸夊瓙绯荤粺椤哄簭鎺ㄨ繘锛屾瘡涓瓙绯荤粺杈炬爣鍚庡啀杩涘叆涓嬩竴瀛愮郴缁燂紝纭繚璐ㄩ噺鍙帶 (see origin)
- 澶氬紩鎿庣粍鍚堬細鍗曚竴宸ュ叿鏃犳硶杈惧埌 90%锛岄渶瑕?KUnit + kselftest + syzkaller + Fault Injection 缁勫悎 (see origin)
- 瑕嗙洊鐜囬噰闆嗘柟妗堬細gcov 涓轰富锛堟彁渚涜/鍒嗘敮绮掑害锛夛紝kcov 涓鸿緟锛堢敤浜?fuzzing 鍦烘櫙锛夛紝缁熶竴杈撳嚭涓?lcov 鏍煎紡
- 娴嬭瘯缂栨帓锛氭墿灞?`tools/testing/kunit/kunit.py` 浣滀负缁熶竴鍏ュ彛锛岄泦鎴?syzkaller 鍜?kselftest

---

## Open Questions

### Resolved During Planning

- 瑕嗙洊鐜囨祴閲忓伐鍏烽€夋嫨锛歡cov锛圙CC 鍐呯疆锛屾敮鎸佸垎鏀垎鏋愶級+ kcov锛圕lang sanitizer锛岀敤浜?fuzzing锛?- 娴嬭瘯缂栨帓鍏ュ彛锛氭墿灞?`tools/testing/kunit/kunit.py` 浣滀负缁熶竴杩愯鍣?
### Deferred to Implementation

- 鍚勫瓙绯荤粺鐨勫熀绾胯鐩栫巼锛氶渶瑕佸厛杩愯鐜版湁娴嬭瘯濂椾欢娴嬮噺
- 鍝簺瀛愮郴缁熸渶闅捐鐩栵紙濡?`arch/` 涓嬬殑鐗瑰畾鏋舵瀯浠ｇ爜锛夛細闇€瑕佸疄闄呮祴閲忓悗璇勪及
- 瑕嗙洊鐜囬噰闆嗗鍐呮牳鎬ц兘鐨勫奖鍝嶏細闇€瑕佸熀鍑嗘祴璇曢獙璇?- 鏉′欢缂栬瘧锛坄#ifdef`锛夊鑷寸殑浠ｇ爜璺緞宸紓锛氶渶瑕佸湪瀹炵幇鏃跺鐞?- 鐜版湁娴嬭瘯鐢ㄤ緥涓湁澶氬皯鍙互澶嶇敤锛氶渶瑕侀€愬瓙绯荤粺瀹¤

---

## Implementation Units

### U1. 寤虹珛瑕嗙洊鐜囬噰闆嗗熀纭€璁炬柦

**Goal:** 寤虹珛缁熶竴鐨勮鐩栫巼閲囬泦娴佹按绾匡紝鏀寔 gcov 鍜?kcov锛岃緭鍑哄垎鏀骇鍒鐩栫巼鎶ュ憡銆?
**Requirements:** R1, R2, R3, R4

**Dependencies:** 鏃?
**Files:**
- Create: `tools/testing/coverage/coverage_harness.py` 鈥?瑕嗙洊鐜囬噰闆嗕富鑴氭湰
- Create: `tools/testing/coverage/gcov_parser.py` 鈥?gcov 鏁版嵁瑙ｆ瀽妯″潡
- Create: `tools/testing/coverage/kcov_parser.py` 鈥?kcov 鏁版嵁瑙ｆ瀽妯″潡
- Create: `tools/testing/coverage/report_generator.py` 鈥?lcov/HTML 鎶ュ憡鐢熸垚妯″潡
- Create: `tools/testing/coverage/configs/` 鈥?瑕嗙洊鐜囬噰闆嗘墍闇€鐨勫唴鏍搁厤缃墖娈?- Modify: `tools/testing/kunit/kunit.py` 鈥?闆嗘垚瑕嗙洊鐜囬噰闆嗗弬鏁?
**Approach:**
- 鎵╁睍 `tools/testing/kunit/kunit.py`锛屾坊鍔?`--coverage` 鍙傛暟锛岃嚜鍔ㄩ厤缃唴鏍稿惎鐢?gcov/kcov
- 鏂板缓 `tools/testing/coverage/` 鐩綍锛屽寘鍚鐩栫巼鏁版嵁瑙ｆ瀽鍜屾姤鍛婄敓鎴愬伐鍏?- 瑕嗙洊鐜囬噰闆嗘祦绋嬶細閰嶇疆鍐呮牳锛堝惎鐢?`CONFIG_GCOV_KERNEL` + `CONFIG_GCOV_PROFILE_ALL`锛夆啋 鏋勫缓 鈫?杩愯娴嬭瘯 鈫?浠?debugfs 閲囬泦鏁版嵁 鈫?瑙ｆ瀽 鈫?鐢熸垚 lcov 鎶ュ憡
- 鏀寔鍒嗘敮瑕嗙洊鐜囨ā寮忥細GCC 鐨?`--coverage` 閰嶅悎 lcov 鐨?`--rc lcov_branch_coverage=1`
- 杈撳嚭鏍煎紡锛歭cov锛堢敤浜庤缁嗗垎鏋愶級+ HTML锛堢敤浜庡璁″睍绀猴級

**Patterns to follow:**
- `tools/testing/kunit/kunit.py` 鐨?argparse + 瀛愬懡浠ゆā寮?- `kernel/gcov/fs.c` 鐨?debugfs 瀵煎嚭鏈哄埗
- `tools/testing/kunit/configs/` 鐨勯厤缃墖娈电粍缁囨柟寮?
**Test scenarios:**
- Happy path: 閰嶇疆鍐呮牳鍚敤 gcov锛岃繍琛?KUnit 娴嬭瘯濂椾欢锛屾垚鍔熺敓鎴?lcov 鎶ュ憡
- Happy path: 閰嶇疆鍐呮牳鍚敤 kcov锛岃繍琛?syzkaller 浼氳瘽锛屾垚鍔熼噰闆嗚鐩栫巼鏁版嵁
- Edge case: 鍐呮牳閰嶇疆涓儴鍒嗘枃浠剁鐢?gcov锛堥€氳繃 `GCOV_PROFILE_*.o := n`锛夛紝鎶ュ憡姝ｇ‘鍙嶆槧鎺掗櫎鏂囦欢
- Error path: debugfs 鏈寕杞芥椂锛岃剼鏈嚜鍔ㄦ寕杞芥垨鎶ラ敊閫€鍑?- Error path: 瑕嗙洊鐜囨暟鎹枃浠舵崯鍧忔椂锛岃剼鏈烦杩囨崯鍧忔枃浠跺苟缁х画澶勭悊
- Integration: 瑕嗙洊鐜囬噰闆嗕笌瀹為檯娴嬭瘯杩愯娴佹按绾块泦鎴愶紝娴嬭瘯缁撴灉鍜岃鐩栫巼鏁版嵁鍏宠仈

**Verification:**
- `tools/testing/coverage/` 鐩綍涓嬬殑鑴氭湰鍙互鐙珛杩愯
- 杩愯 `python tools/testing/kunit/kunit.py run --coverage` 鍙互鐢熸垚鏈夋晥鐨?lcov 鎶ュ憡
- lcov 鎶ュ憡涓寘鍚垎鏀鐩栫巼鏁版嵁锛坄BRDA` 璁板綍锛?- 鐩稿悓娴嬭瘯杈撳叆杩愯涓ゆ锛屽垎鏀鐩栫巼鏁板瓧宸紓 鈮?%

---

### U2. 寤虹珛缁熶竴娴嬭瘯缂栨帓妗嗘灦

**Goal:** 灏?KUnit銆乲selftest銆乻yzkaller銆丗ault Injection 缁熶竴缂栨帓涓哄彲涓€閿繍琛岀殑娴嬭瘯濂椾欢銆?
**Requirements:** R5, R6, R7, R8

**Dependencies:** U1

**Files:**
- Create: `tools/testing/orchestrator/test_orchestrator.py` 鈥?缁熶竴娴嬭瘯缂栨帓鍣?- Create: `tools/testing/orchestrator/kunit_runner.py` 鈥?KUnit 杩愯閫傞厤鍣?- Create: `tools/testing/orchestrator/kselftest_runner.py` 鈥?kselftest 杩愯閫傞厤鍣?- Create: `tools/testing/orchestrator/syzkaller_runner.py` 鈥?syzkaller 杩愯閫傞厤鍣?- Create: `tools/testing/orchestrator/faultinj_runner.py` 鈥?Fault Injection 杩愯閫傞厤鍣?- Create: `tools/testing/orchestrator/configs/` 鈥?鍚勫紩鎿庣殑閰嶇疆妯℃澘

**Approach:**
- 鎵╁睍 `tools/testing/kunit/kunit.py`锛屾坊鍔?`run_all` 瀛愬懡浠わ紝鎸夊瓙绯荤粺椤哄簭杩愯鍏ㄩ儴娴嬭瘯寮曟搸
- 姣忎釜娴嬭瘯寮曟搸閫氳繃閫傞厤鍣ㄥ皝瑁咃紝鎻愪緵缁熶竴鐨勬帴鍙ｏ細`configure()` 鈫?`build()` 鈫?`run()` 鈫?`collect_coverage()`
- 鏀寔鍗曠嫭杩愯鐗瑰畾瀛愮郴缁熺殑娴嬭瘯锛歚--subsystem mm/`
- 鑷姩閲嶈瘯鏈哄埗锛氭祴璇曞け璐ユ椂閲嶈瘯鏈€澶?3 娆★紝鍖哄垎鍋跺彂鎬уけ璐ワ紙閲嶈瘯鍚庨€氳繃锛夊拰纭畾鎬уけ璐ワ紙濮嬬粓澶辫触锛?- 娴嬭瘯鐜缂撳瓨锛氫娇鐢?`make O=build` 鐨?out-of-tree 鏋勫缓锛岄厤缃拰鏋勫缓缁撴灉缂撳瓨锛屾敮鎸佸揩閫熼噸寤?
**Patterns to follow:**
- `tools/testing/kunit/kunit.py` 鐨勯厤缃?鏋勫缓/杩愯涓夐樁娈垫ā寮?- `tools/testing/selftests/kselftest/runner.sh` 鐨?TAP 杈撳嚭鍜岃秴鏃跺鐞?- `drivers/gpu/drm/ci/kunit.sh` 鐨?CI 闆嗘垚妯″紡

**Test scenarios:**
- Happy path: `python tools/testing/kunit/kunit.py run_all` 涓€閿繍琛屽叏閮ㄦ祴璇曞紩鎿?- Happy path: `python tools/testing/kunit/kunit.py run_all --subsystem mm/` 浠呰繍琛?mm/ 瀛愮郴缁熸祴璇?- Edge case: 鏌愪釜娴嬭瘯寮曟搸涓嶅彲鐢紙濡?syzkaller 鏈畨瑁咃級锛岃烦杩囪寮曟搸骞剁户缁繍琛屽叾浠栧紩鎿?- Error path: 娴嬭瘯瓒呮椂鏃讹紝璁板綍瓒呮椂淇℃伅骞剁户缁笅涓€娴嬭瘯
- Error path: 鍋跺彂鎬ф祴璇曞け璐ユ椂锛岃嚜鍔ㄩ噸璇曞苟鍦ㄦ姤鍛婁腑鏍囨敞
- Integration: 澶氫釜娴嬭瘯寮曟搸鐨勮緭鍑哄悎骞朵负缁熶竴鐨?TAP/JSON 鏍煎紡

**Verification:**
- `tools/testing/kunit/kunit.py run_all` 鍙互涓€閿繍琛屽叏閮ㄦ祴璇曞紩鎿?- 鏀寔 `--subsystem` 鍙傛暟鍗曠嫭杩愯鐗瑰畾瀛愮郴缁?- 鍋跺彂鎬уけ璐ヨ嚜鍔ㄩ噸璇曪紝纭畾鎬уけ璐ョ洿鎺ユ姤鍛?- 娴嬭瘯鐜閰嶇疆鍜屾瀯寤虹粨鏋滃彲缂撳瓨锛岄噸寤烘椂闂?鈮?0 鍒嗛挓

---

### U3. kernel/ 瀛愮郴缁熷垎鏀鐩栫巼杈惧埌 90%

**Goal:** 涓?`kernel/` 瀛愮郴缁熺紪鍐?KUnit 鍜?kselftest 娴嬭瘯锛屼娇璇ュ瓙绯荤粺鐨勫垎鏀鐩栫巼杈惧埌 90%銆?
**Requirements:** R9, R10, R11

**Dependencies:** U1, U2

**Files:**
- Create: `tools/testing/coverage/baseline/` 鈥?鍩虹嚎瑕嗙洊鐜囨暟鎹洰褰?- Create: `kernel/test/` 鈥?`kernel/` 瀛愮郴缁熺殑 KUnit 娴嬭瘯锛堟柊寤虹洰褰曪級
- Modify: 鍚勭洰鏍囨枃浠剁殑 Makefile 娣诲姞 `GCOV_PROFILE_*.o := y`

**Approach:**
- 鍏堢敤 U1 鐨勫熀纭€璁炬柦娴嬮噺 `kernel/` 鐨勫熀绾垮垎鏀鐩栫巼
- 鍒嗘瀽鏈鐩栧垎鏀紝璇嗗埆鍙€氳繃 KUnit 瑕嗙洊鐨勮矾寰勶紙璋冨害鍣ㄣ€乸rintk銆乮rq銆乼ime銆乴ocking銆丷CU銆丅PF 绛夛級
- 缂栧啓 KUnit 娴嬭瘯鐢ㄤ緥瑕嗙洊鍏抽敭璺緞锛岀壒鍒叧娉ㄩ敊璇鐞嗗垎鏀?- 瀵归渶瑕佺敤鎴风┖闂翠氦浜掔殑璺緞锛岀紪鍐?kselftest 娴嬭瘯
- 浣跨敤 Fault Injection 瑙﹀彂閿欒澶勭悊璺緞锛堝鍐呭瓨鍒嗛厤澶辫触锛?- 姣忎釜娴嬭瘯鐢ㄤ緥鍏宠仈鍒板叿浣撶殑鏈鐩栧垎鏀紝纭繚娴嬭瘯鏈夋槑纭殑瑕嗙洊鐜囪础鐚?- 杈炬爣鍚庢彁浜ゅ璁★紝瀹¤閫氳繃鍚庤繘鍏ヤ笅涓€瀛愮郴缁?
**Patterns to follow:**
- `lib/kunit/` 鐨?KUnit 娴嬭瘯娉ㄥ唽妯″紡
- `tools/testing/selftests/` 鐨?kselftest  harness 妯″紡
- `lib/fault-inject.c` 鐨?`should_fail()` 鐢ㄦ硶

**Test scenarios:**
- Happy path: `kernel/sched/` 鏍稿績璺緞鐨?KUnit 娴嬭瘯瑕嗙洊鐜囪揪鍒?90%
- Happy path: `kernel/printk/` 鐨?printk 璺緞閫氳繃 kselftest 瑕嗙洊鐜囪揪鍒?90%
- Edge case: 璋冨害鍣ㄥ湪涓嶅悓浼樺厛绾т笅鐨勫垎鏀鐩?- Error path: 閫氳繃 Fault Injection 瑙﹀彂 kmalloc 澶辫触锛岃鐩栭敊璇鐞嗗垎鏀?- Integration: KUnit + kselftest + Fault Injection 缁勫悎瑕嗙洊 `kernel/` 鐨勫畬鏁磋矾寰?
**Verification:**
- `kernel/` 瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜?鈮?0%
- 瑕嗙洊鐜囨姤鍛婁腑鏈鐩栧垎鏀?<10%
- 姣忎釜娴嬭瘯鐢ㄤ緥鏈夋槑纭殑浠ｇ爜璺緞瑕嗙洊鐩爣
- 娴嬭瘯浠ｇ爜閫氳繃浠ｇ爜瀹℃煡

---

### U4. mm/ 瀛愮郴缁熷垎鏀鐩栫巼杈惧埌 90%

**Goal:** 涓?`mm/` 瀛愮郴缁熺紪鍐?KUnit 鍜?kselftest 娴嬭瘯锛屼娇璇ュ瓙绯荤粺鐨勫垎鏀鐩栫巼杈惧埌 90%銆?
**Requirements:** R9, R10, R11

**Dependencies:** U3

**Files:**
- Create: `mm/test/` 鈥?`mm/` 瀛愮郴缁熺殑 KUnit 娴嬭瘯
- Modify: `mm/` 涓嬪悇鐩爣鏂囦欢鐨?Makefile 娣诲姞 `GCOV_PROFILE_*.o := y`

**Approach:**
- 鍒嗘瀽 `mm/` 瀛愮郴缁熺殑鍏抽敭璺緞锛歱age allocator銆乻lab銆乿malloc銆乭ugetlb銆乻wap銆乵map銆乵advise銆乵protect 绛?- 缂栧啓 KUnit 娴嬭瘯瑕嗙洊鍐呭瓨鍒嗛厤鍣ㄧ殑鍩虹璺緞锛?buddy system銆乻lab allocator锛?- 缂栧啓 kselftest 娴嬭瘯瑕嗙洊鐢ㄦ埛绌洪棿鍙鐨勫唴瀛樼鐞嗘帴鍙ｏ紙mmap銆乵advise銆乵protect銆乥rk锛?- 浣跨敤 Fault Injection 瑙﹀彂鍐呭瓨鍒嗛厤澶辫触锛岃鐩栭敊璇鐞嗚矾寰?- 閲嶇偣瑕嗙洊绔炰簤鏉′欢璺緞锛堥€氳繃 KCSAN 妫€娴嬶級
- 杈炬爣鍚庢彁浜ゅ璁★紝瀹¤閫氳繃鍚庤繘鍏ヤ笅涓€瀛愮郴缁?
**Patterns to follow:**
- U3 涓缓绔嬬殑 `kernel/` 娴嬭瘯妯″紡鍜岃鐩栫巼娴佺▼
- `mm/damon/` 鐜版湁鐨?KUnit 娴嬭瘯锛坄tools/testing/kunit/configs/damon`锛?
**Test scenarios:**
- Happy path: page allocator 鐨勫垎閰?閲婃斁璺緞瑕嗙洊鐜囪揪鍒?90%
- Happy path: mmap/munmap 鐨勭敤鎴风┖闂磋矾寰勮鐩栫巼杈惧埌 90%
- Edge case: 涓嶅悓鍐呭瓨鍖哄煙锛圖MA銆丯ormal銆丠ighMem锛夌殑鍒嗛厤璺緞
- Error path: 閫氳繃 failslab 娉ㄥ叆 kmalloc 澶辫触锛岃鐩栭敊璇鐞嗗垎鏀?- Error path: 閫氳繃 fail_page_alloc 娉ㄥ叆 page alloc 澶辫触
- Integration: 鍐呭瓨鍘嬪姏涓嬬殑鍒嗛厤璺緞锛堢粨鍚?fault injection锛?
**Verification:**
- `mm/` 瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜?鈮?0%
- 瑕嗙洊鐜囨姤鍛婁腑鏈鐩栧垎鏀?<10%
- 閿欒澶勭悊璺緞閫氳繃 fault injection 瑕嗙洊

---

### U5. fs/ + net/ 瀛愮郴缁熷垎鏀鐩栫巼杈惧埌 90%

**Goal:** 涓?`fs/` 鍜?`net/` 瀛愮郴缁熺紪鍐欐祴璇曪紝浣夸袱涓瓙绯荤粺鐨勫垎鏀鐩栫巼杈惧埌 90%銆?
**Requirements:** R9, R10, R11

**Dependencies:** U4

**Files:**
- Create: `fs/test/` 鈥?`fs/` 瀛愮郴缁熺殑 KUnit 鍜?kselftest 娴嬭瘯
- Create: `net/test/` 鈥?`net/` 瀛愮郴缁熺殑 KUnit 鍜?kselftest 娴嬭瘯
- Modify: `fs/` 鍜?`net/` 涓嬪悇鐩爣鏂囦欢鐨?Makefile 娣诲姞 `GCOV_PROFILE_*.o := y`

**Approach:**
- `fs/`锛氶噸鐐硅鐩?VFS 灞傦紙superblock/inode/dentry 鎿嶄綔锛夈€乪xt4 鏍稿績璺緞銆乸ath lookup銆乵ount 娴佺▼
- `fs/`锛氫娇鐢?kselftest 瑕嗙洊鏂囦欢绯荤粺鎿嶄綔鐨勫畬鏁磋矾寰勶紙open/read/write/close銆乮octl銆乫cntl锛?- `net/`锛氶噸鐐硅鐩?sk_buff 鐢熷懡鍛ㄦ湡銆丯API銆乶etdevice 妯″瀷銆乻ocket 灞?- `net/`锛氫娇鐢?kselftest 瑕嗙洊缃戠粶鍗忚鏍堬紙IPv4銆両Pv6銆乀CP銆乶etfilter锛?- 浣跨敤 syzkaller 瀵?VFS 鍜岀綉缁滄爤杩涜妯＄硦娴嬭瘯锛岃鐩栫綍瑙佽矾寰?- 浣跨敤 Fault Injection 瑙﹀彂鍧楄澶?IO 閿欒銆佺綉缁滃寘涓㈠け绛夊満鏅?- 杈炬爣鍚庢彁浜ゅ璁★紝瀹¤閫氳繃鍚庤繘鍏ヤ笅涓€瀛愮郴缁?
**Patterns to follow:**
- `fs/ext4/` 鐜版湁鐨?KUnit 娴嬭瘯锛坄.kunitconfig` 鍦?`fs/ext4/`锛?- `tools/testing/selftests/net/` 鐜版湁鐨勭綉缁滄祴璇?- `tools/testing/selftests/filesystems/` 鐜版湁鐨勬枃浠剁郴缁熸祴璇?
**Test scenarios:**
- Happy path: VFS 灞傜殑璺緞鏌ユ壘锛坧ath lookup锛夎鐩栫巼杈惧埌 90%
- Happy path: ext4 鏍稿績璺緞锛坕node 鎿嶄綔銆佸潡鍒嗛厤銆乯ournal锛夎鐩栫巼杈惧埌 90%
- Happy path: 缃戠粶鏍堢殑 sk_buff 鍒嗛厤/閲婃斁璺緞瑕嗙洊鐜囪揪鍒?90%
- Happy path: TCP 杩炴帴寤虹珛/鏂紑璺緞瑕嗙洊鐜囪揪鍒?90%
- Edge case: 涓嶅悓鏂囦欢绯荤粺绫诲瀷鐨?VFS 鎿嶄綔宸紓
- Error path: 閫氳繃 fail_make_request 娉ㄥ叆鍧楄澶?IO 閿欒
- Error path: 閫氳繃 fail_skb_realloc 娉ㄥ叆缃戠粶 skb 閲嶅垎閰嶅け璐?- Integration: syzkaller 妯＄硦娴嬭瘯 VFS 鍜岀綉缁滄爤鐨勮竟鐣岃矾寰?
**Verification:**
- `fs/` 瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜?鈮?0%
- `net/` 瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜?鈮?0%
- syzkaller 鍙戠幇鐨勮竟鐣岃矾寰勬湁瀵瑰簲鐨勫洖褰掓祴璇?
---

### U6. drivers/ 瀛愮郴缁熷垎鏀鐩栫巼杈惧埌 90%

**Goal:** 涓?`drivers/` 瀛愮郴缁熺紪鍐欐祴璇曪紝浣胯瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜囪揪鍒?90%銆?
**Requirements:** R9, R10, R11

**Dependencies:** U5

**Files:**
- Create: `drivers/test/` 鈥?`drivers/` 瀛愮郴缁熺殑娴嬭瘯妗嗘灦鍜屽叕鍏辨祴璇?- Create: `drivers/base/test/` 鈥?椹卞姩鏍稿績锛坉evice/driver/bus 妯″瀷锛夌殑 KUnit 娴嬭瘯
- Create: `drivers/gpu/drm/tests/` 鈥?DRM 瀛愮郴缁熺殑 KUnit 娴嬭瘯鎵╁睍
- Modify: `drivers/` 涓嬪悇鐩爣鏂囦欢鐨?Makefile 娣诲姞 `GCOV_PROFILE_*.o := y`

**Approach:**
- 鐢变簬椹卞姩浠ｇ爜闇€瑕佺‖浠舵垨瀹屾暣骞冲彴妯℃嫙锛屼紭鍏堜娇鐢?QEMU 浣滀负娴嬭瘯骞冲彴
- 閲嶇偣瑕嗙洊椹卞姩鏍稿績妗嗘灦锛坄drivers/base/`锛夛細kobject銆乨evice銆乨river銆乥us 灞傛缁撴瀯
- 鎸夐┍鍔ㄧ被鍨嬪垎缁勶紝浼樺厛瑕嗙洊閫氱敤椹卞姩锛坆lock銆乧har銆乶et銆乻ound銆乬pu锛?- 浣跨敤 QEMU 鍚姩瀹屾暣绯荤粺锛岃繍琛?kselftest 鍜?KUnit 娴嬭瘯
- 浣跨敤 syzkaller 瀵归┍鍔ㄦ帴鍙ｈ繘琛屾ā绯婃祴璇?- 瀵逛簬闇€瑕佺壒瀹氱‖浠剁殑椹卞姩锛屼娇鐢?QEMU 鐨勮澶囨ā鎷熸垨缂栧啓 mock 娴嬭瘯
- 杈炬爣鍚庢彁浜ゅ璁★紝瀹¤閫氳繃鍚庤繘鍏ヤ笅涓€瀛愮郴缁?
**Patterns to follow:**
- `drivers/gpu/drm/ci/kunit.sh` 鐨?QEMU + KUnit 妯″紡
- `drivers/gpu/drm/tests/` 鐜版湁鐨?DRM KUnit 娴嬭瘯

**Test scenarios:**
- Happy path: 椹卞姩鏍稿績妗嗘灦锛坉evice_register/driver_register/bus_register锛夎鐩栫巼杈惧埌 90%
- Happy path: DRM 鏍稿績璺緞锛坉rm_mode_create銆乬em_create銆乨ma_resv锛夎鐩栫巼杈惧埌 90%
- Edge case: 涓嶅悓鎬荤嚎绫诲瀷锛圥CI銆乁SB銆乸latform锛夌殑椹卞姩缁戝畾娴佺▼
- Error path: 閫氳繃 fault injection 瑙﹀彂椹卞姩 probe 澶辫触
- Error path: 妯℃嫙璁惧鐑彃鎷旓紙device_add/remove锛?- Integration: QEMU 瀹屾暣绯荤粺鍚姩鍚庤繍琛?KUnit 鍜?kselftest

**Verification:**
- `drivers/` 瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜?鈮?0%
- 娴嬭瘯鍙互鍦?QEMU 涓嚜鍔ㄨ繍琛?- 椹卞姩鏍稿績妗嗘灦鐨勬祴璇曡鐩栫巼 鈮?5%锛堥┍鍔ㄦ牳蹇冪浉瀵圭ǔ瀹氾紝瑕嗙洊鐜囧簲鏇撮珮锛?
---

### U7. arch/*/ 瀛愮郴缁熷垎鏀鐩栫巼杈惧埌 90%

**Goal:** 涓?`arch/*/` 瀛愮郴缁熺紪鍐欐祴璇曪紝浣胯瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜囪揪鍒?90%銆?
**Requirements:** R9, R10, R11

**Dependencies:** U6

**Files:**
- Create: `arch/test/` 鈥?鏋舵瀯鐩稿叧娴嬭瘯鐨勫叕鍏辨鏋?- Create: `arch/x86/` 涓嬬殑鏋舵瀯鐗瑰畾娴嬭瘯
- Create: `arch/arm64/` 涓嬬殑鏋舵瀯鐗瑰畾娴嬭瘯
- Modify: `arch/*/` 涓嬪悇鐩爣鏂囦欢鐨?Makefile 娣诲姞 `GCOV_PROFILE_*.o := y`

**Approach:**
- 鏋舵瀯浠ｇ爜楂樺害渚濊禆鍏蜂綋纭欢锛屼娇鐢?QEMU 妯℃嫙澶氱鏋舵瀯锛坸86_64銆乤rm64銆乺iscv锛?- 閲嶇偣瑕嗙洊鏋舵瀯鏃犲叧鐨勫叕鍏辫矾寰勶紙entry common銆乻ignal handling銆乻yscall dispatch銆乧ontext switch锛?- 鎸夋灦鏋勫垎缁勶紝浼樺厛瑕嗙洊涓绘祦鏋舵瀯锛坸86_64銆乤rm64锛?- 浣跨敤 KUnit 娴嬭瘯鏋舵瀯鐗瑰畾鐨勮緟鍔╁嚱鏁板拰鏁版嵁缁撴瀯鐨勯€昏緫
- 浣跨敤 kselftest 娴嬭瘯鏋舵瀯鐗瑰畾鐨勭郴缁熻皟鐢ㄥ拰 ABI
- 瀵逛簬寮傚父澶勭悊璺緞锛屼娇鐢?QEMU 鐨?fault injection 鏈哄埗瑙﹀彂
- 杈炬爣鍚庢彁浜ゅ璁★紝瀹¤閫氳繃鍚庤繘鍏ヤ笅涓€瀛愮郴缁?
**Patterns to follow:**
- `arch/x86/` 鐜版湁鐨勬祴璇曪紙`tools/testing/selftests/x86/`锛?- `arch/arm64/` 鐜版湁鐨勬祴璇?- U6 涓缓绔嬬殑 QEMU 娴嬭瘯妯″紡

**Test scenarios:**
- Happy path: 绯荤粺璋冪敤鍒嗗彂璺緞锛坰yscall entry/exit锛夎鐩栫巼杈惧埌 90%
- Happy path: 淇″彿澶勭悊璺緞锛坰ignal delivery/return锛夎鐩栫巼杈惧埌 90%
- Happy path: 涓婁笅鏂囧垏鎹㈣矾寰勮鐩栫巼杈惧埌 90%
- Edge case: 涓嶅悓绯荤粺璋冪敤鍙风殑 syscall 澶勭悊宸紓
- Error path: 閫氳繃 QEMU 娉ㄥ叆椤甸敊璇€佹閿欒绛夊紓甯?- Error path: 妯℃嫙绯荤粺璋冪敤鍙傛暟鏃犳晥鐨勯敊璇鐞嗚矾寰?- Integration: 瀹屾暣绯荤粺鍚姩鍚庤繍琛屾灦鏋勭壒瀹氱殑 kselftest

**Verification:**
- `arch/*/` 瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜?鈮?0%
- 涓绘祦鏋舵瀯锛坸86_64銆乤rm64锛夌殑瑕嗙洊鐜?鈮?5%
- 娴嬭瘯鍙互鍦?QEMU 涓嚜鍔ㄨ繍琛?
---

### U8. 瀹¤鎶ュ憡銆佸洖褰掗槻鎶ゅ拰 CI 闆嗘垚

**Goal:** 寤虹珛瀹屾暣鐨勫璁℃姤鍛婄郴缁熴€佽鐩栫巼鍥炲綊闃叉姢鏈哄埗鍜?CI 闆嗘垚銆?
**Requirements:** R12, R13, R14, R11

**Dependencies:** U7

**Files:**
- Create: `tools/testing/audit/report_generator.py` 鈥?瀛ｅ害瀹¤鎶ュ憡鐢熸垚鍣?- Create: `tools/testing/audit/coverage_regression.py` 鈥?瑕嗙洊鐜囧洖褰掓娴嬪伐鍏?- Create: `tools/testing/audit/ci_configs/` 鈥?CI 閰嶇疆妯℃澘
- Modify: `.gitlab-ci.yml`锛堟垨鏂板缓锛夆€?CI 娴佹按绾块厤缃?- Create: `docs/dev-tools/coverage-audit.rst` 鈥?瑕嗙洊鐜囧璁℃枃妗?
**Approach:**
- 瀹¤鎶ュ憡鐢熸垚鍣細姹囨€诲悇瀛愮郴缁熺殑瑕嗙洊鐜囨暟鎹紝鐢熸垚瓒嬪娍鍥俱€佹湭瑕嗙洊鍒嗘敮鍒嗘瀽銆佹祴璇曞畬鏁存€ц瘉鏄?- 瑕嗙洊鐜囧洖褰掓娴嬶細鍦?CI 涓泦鎴愯鐩栫巼闂ㄦ帶锛坈overage gate锛夛紝鏂颁唬鐮佸悎骞朵笉瀵艰嚧宸茶揪鏍囧瓙绯荤粺瑕嗙洊鐜囦笅闄?- CI 闆嗘垚锛氬缓绔?GitLab CI 娴佹按绾匡紝鏀寔鑷姩杩愯娴嬭瘯銆侀噰闆嗚鐩栫巼銆佺敓鎴愭姤鍛娿€佽Е鍙戣鐩栫巼闂ㄦ帶
- 瀹¤鏂囨。锛氱紪鍐欒缁嗙殑瑕嗙洊鐜囬噰闆嗗拰楠岃瘉鏂囨。锛屾敮鎸佸璁℃柟鐙珛杩愯娴嬭瘯濂椾欢
- 鏀寔瀹¤鏂瑰鍑哄師濮嬭鐩栫巼鏁版嵁锛岀嫭绔嬮獙璇佹姤鍛婄粨鏋?
**Patterns to follow:**
- `drivers/gpu/drm/ci/kunit.yml` 鐨?GitLab CI 妯″紡
- `tools/testing/selftests/kselftest/runner.sh` 鐨?TAP 杈撳嚭妯″紡

**Test scenarios:**
- Happy path: 鎻愪氦 PR 鍚庯紝CI 鑷姩杩愯鐩稿叧瀛愮郴缁熸祴璇曪紝瑕嗙洊鐜囦笉涓嬮檷
- Happy path: 鐢熸垚瀛ｅ害瀹¤鎶ュ憡锛屽寘鍚墍鏈夊繀闇€瀛楁
- Edge case: 鏂颁唬鐮佽鐩栦簡鏂扮殑鍒嗘敮锛岃鐩栫巼涓婂崌
- Error path: 鏂颁唬鐮佸紩鍏ヤ簡鏈鐩栫殑鍒嗘敮锛岃鐩栫巼涓嬮檷锛孋I 闃绘柇鍚堝苟
- Integration: 瀹¤鏂规寜鐓ф枃妗ｇ嫭绔嬭繍琛屾祴璇曞浠讹紝寰楀埌涓庨」鐩姤鍛婁竴鑷寸殑瑕嗙洊鐜囨暟鎹?
**Verification:**
- 瀛ｅ害瀹¤鎶ュ憡鍖呭惈鎵€鏈夊繀闇€瀛楁锛堣鐩栫巼瓒嬪娍銆佹湭瑕嗙洊鍒嗘敮鍒嗘瀽銆佹祴璇曞畬鏁存€ц瘉鏄庯級
- CI 娴佹按绾垮彲浠ヨ嚜鍔ㄨ繍琛屾祴璇曘€侀噰闆嗚鐩栫巼銆佹墽琛屽洖褰掓娴?- 瑕嗙洊鐜囬棬鎺ф湁鏁堬細鏂颁唬鐮佸悎骞朵笉瀵艰嚧瑕嗙洊鐜囦笅闄?- 瀹¤鏂瑰彲浠ョ嫭绔嬭繍琛屾祴璇曞浠跺苟澶嶇幇瑕嗙洊鐜囨暟鎹?
---

## System-Wide Impact

- **Interaction graph:** 娴嬭瘯缂栨帓鍣ㄤ笌鎵€鏈夋祴璇曞紩鎿庯紙KUnit銆乲selftest銆乻yzkaller銆丗ault Injection锛変氦浜掞紱瑕嗙洊鐜囬噰闆嗕笌鍐呮牳鏋勫缓绯荤粺锛圞build锛変氦浜掞紱CI 娴佹按绾夸笌 GitLab 浜や簰
- **Error propagation:** 瑕嗙洊鐜囬噰闆嗗け璐ヤ笉搴旈樆鏂祴璇曡繍琛岋紱娴嬭瘯寮曟搸鏁呴殰搴旈殧绂诲奖鍝?- **State lifecycle risks:** 瑕嗙洊鐜囨暟鎹枃浠跺彲鑳藉緢澶э紙GB 绾э級锛岄渶瑕佸畾鏈熸竻鐞嗗拰褰掓。锛涙祴璇曠幆澧冮厤缃渶瑕佺増鏈帶鍒?- **API surface parity:** 瑕嗙洊鐜囨姤鍛婃牸寮忛渶瑕佸悜鍚庡吋瀹癸紝鏀寔鍘嗗彶鏁版嵁瀵规瘮
- **Integration coverage:** 娴嬭瘯缂栨帓鍣ㄩ渶瑕佷笌鐜版湁鏋勫缓绯荤粺锛坄make kselftest`銆乣make kunit`锛夐泦鎴愶紝涓嶇牬鍧忕幇鏈夊伐浣滄祦
- **Unchanged invariants:** 鍐呮牳婧愮爜涓嶅洜娴嬭瘯宸ョ▼鑰屼慨鏀癸紙闄ゅ繀瑕佺殑 `GCOV_PROFILE_*.o` 鏍囪澶栵級锛涚幇鏈夋祴璇曞浠剁户缁甯稿伐浣?
---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| 閮ㄥ垎鏋舵瀯浠ｇ爜锛堝 arch/*/ 鐨勫紓甯稿鐞嗭級鍦ㄨ蒋浠跺眰闈㈡棤娉曡鐩?| High | High | 涓庡璁℃柟鍗忓晢锛屽鏃犳硶瑕嗙洊鐨勪唬鐮佹彁渚涙浛浠ｈ瘉鏄庯紙浠ｇ爜瀹℃煡銆侀潤鎬佸垎鏋愶級 |
| 瑕嗙洊鐜囬噰闆嗗鑷村唴鏍告€ц兘澶у箙涓嬮檷锛屽奖鍝嶆祴璇曟晥鐜?| Medium | Medium | 浣跨敤閲囨牱妯″紡鎴栧垎闃舵閲囬泦锛涘湪闈炴€ц兘鍏抽敭璺緞涓婁娇鐢?kcov |
| syzkaller 瀵规煇浜涘瓙绯荤粺鐨勮鐩栨晥鐜囦綆 | Medium | Medium | 閽堝鎬х紪鍐?seed corpus锛涚粨鍚?kselftest 琛ュ厖瑕嗙洊 |
| 娴嬭瘯鐢ㄤ緥缁存姢鎴愭湰楂橈紝闅忓唴鏍告紨鍖栭渶瑕佹寔缁洿鏂?| Medium | Medium | 寤虹珛娴嬭瘯缁存姢娴佺▼锛涘皢娴嬭瘯鐢ㄤ緥绾冲叆鍐呮牳瀹℃煡娴佺▼ |
| 瑕嗙洊鐜囨暟鎹噺澶э紙GB 绾э級锛屽瓨鍌ㄥ拰浼犺緭鎴愭湰楂?| Medium | Low | 浣跨敤 lcov 鐨勫帇缂╂牸寮忥紱浠呬繚鐣欐眹鎬绘暟鎹紝璇︾粏鏁版嵁鎸夐渶鐢熸垚 |
| 鍥㈤槦鎶€鑳界己鍙ｏ細娴嬭瘯宸ョ▼甯堥渶瑕佺啛鎮夊唴鏍稿唴閮?| Medium | Medium | 鍩硅璁″垝锛涗笌鍐呮牳寮€鍙戣€呯粨瀵圭紪绋?|

---

## Documentation / Operational Notes

- 瑕嗙洊鐜囧璁℃枃妗ｏ細`docs/dev-tools/coverage-audit.rst`
- 娴嬭瘯缂栧啓鎸囧崡锛歚docs/dev-tools/testing-coverage-guide.rst`
- CI 娴佹按绾挎枃妗ｏ細`.gitlab-ci.yml` 娉ㄩ噴
- 姣忎釜瀛愮郴缁熺殑瑕嗙洊鐜囧熀绾挎暟鎹瓨妗ｅ湪 `tools/testing/coverage/baseline/`
- 瀹¤鎶ュ憡瀛樻。鍦?`tools/testing/audit/reports/`

---

## Sources & References

- **Origin document:** [docs/brainstorms/kernel-branch-coverage-90-requirements.md](../brainstorms/kernel-branch-coverage-90-requirements.md)
- **KUnit 妗嗘灦:** [lib/kunit/](../../lib/kunit/)
- **KUnit 杩愯鍣?** [tools/testing/kunit/kunit.py](../../tools/testing/kunit/kunit.py)
- **kselftest 鐩綍:** [tools/testing/selftests/](../../tools/testing/selftests/)
- **gcov 瀹炵幇:** [kernel/gcov/](../../kernel/gcov/)
- **kcov 閰嶇疆:** [lib/Kconfig.debug](../../lib/Kconfig.debug)
- **Fault Injection:** [lib/fault-inject.c](../../lib/fault-inject.c)
- **DRM CI 绀轰緥:** [drivers/gpu/drm/ci/](../../drivers/gpu/drm/ci/)
- **gcov 鏂囨。:** [Documentation/dev-tools/gcov.rst](../../Documentation/dev-tools/gcov.rst)
- **kcov 鏂囨。:** [Documentation/dev-tools/kcov.rst](../../Documentation/dev-tools/kcov.rst)
