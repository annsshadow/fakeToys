---
title: feat: Multi-engine full-codebase branch coverage 鈮?0%
type: feat
status: active
date: 2026-07-08
origin: docs/brainstorms/2026-07-08-multi-engine-full-coverage-requirements.md
---

# Linux Kernel Multi-Engine Full-Codebase Branch Coverage 鈮?0%

## Summary

鍦ㄧ幇鏈?KUnit銆乲selftest銆佽鐩栫巼宸ュ叿閾惧拰 GitLab CI 娴佹按绾垮熀纭€涓婏紝鎵╁睍绾冲叆 syzkaller 鍜?Fault Injection 浣滀负琛ュ厖娴嬭瘯寮曟搸锛屽缓绔嬪叏浠ｇ爜鏍戝悓姝ユ帹杩涚殑鍒嗘敮瑕嗙洊鐜囨祴璇曞伐绋嬨€傞€氳繃"鍩虹嚎娴嬮噺鍏堣銆佸洓寮曟搸缁勫悎瑕嗙洊銆丆I 鐭╅樀鎵╁睍銆佸璁″氨缁姤鍛?鍥涙锛岃揪鍒板璁℃柟鍙帴鍙楃殑鍒嗘敮瑕嗙洊鐜?鈮?0%銆?
---

## Problem Frame

鍚堣瀹¤鏂硅姹?Linux 鍐呮牳椤圭洰鎻愪緵鍏ㄤ唬鐮佹爲鍒嗘敮瑕嗙洊鐜?鈮?0% 鐨勮瘉鏄庯紝涓嶆帴鍙楄眮鍏嶆垨鏇夸唬鎸囨爣銆傚綋鍓嶅唴鏍稿凡鏈?KUnit銆乲selftest 涓ゅ娴嬭瘯妗嗘灦锛屼互鍙?gcov/kcov 瑕嗙洊鐜囧伐鍏峰拰 GitLab CI 娴佹按绾匡紝浣嗚鐩栧垎甯冩瀬涓嶅潎琛★紝涓旂己灏?syzkaller 鍜?Fault Injection 鐨勭郴缁熷寲杩愮敤銆傞」鐩病鏈夌幇鎴愮殑鍏ㄤ唬鐮佹爲鍒嗘敮瑕嗙洊鐜囧熀绾挎暟鎹紝鏃犳硶璇勪及褰撳墠缂哄彛鍜屽埗瀹氱簿鍑嗘帹杩涜鍒掋€傛敼鍠勬祴璇曡鐩栭渶瑕佸厛寤虹珛鍙鐜扮殑鍩虹嚎娴嬮噺锛屽啀鎸変紭鍏堢骇缁勭粐鍥涘紩鎿庣粍鍚堟帹杩涳紝骞堕厤鍚堝璁″氨缁殑鎶ュ憡鍜屽洖褰掗槻鎶ょ‘淇濇柊澧炴祴璇曟寔缁湁鏁堛€?
---

## Requirements

**Origin actors:** A1锛堝悎瑙勫璁℃柟锛? A2锛堝唴鏍告祴璇曞伐绋嬪笀锛? A3锛堝唴鏍稿紑鍙戣€咃級, A4锛堝熀纭€璁炬柦宸ョ▼甯堬級, A5锛堥」鐩粡鐞嗭級

**Origin flows:** F1锛堝熀绾挎祴閲忎笌缂哄彛鍒嗘瀽锛? F2锛堝洓寮曟搸缁勫悎娴嬭瘯寮€鍙戯級, F3锛堝璁℃鏌ョ偣锛? F4锛堝洖褰掗槻鎶わ級

**Origin acceptance examples:** AE1锛圧1, R2锛? AE2锛圧3锛? AE3锛圧5锛? AE4锛圧9锛? AE5锛圧14锛?
- R1. 寤虹珛缁熶竴鐨勮鐩栫巼閲囬泦娴佹按绾匡紝鏀寔 gcov 鍜?kcov 涓ょ宸ュ叿锛岃緭鍑烘爣鍑嗘牸寮忕殑瑕嗙洊鐜囨姤鍛婏紙lcov/html 鎴栫瓑鏁堟牸寮忥級
- R2. 瑕嗙洊鐜囨姤鍛婂繀椤诲寘鍚垎鏀骇鍒矑搴︼紙鍝簺鍒嗘敮琚鐩栥€佸摢浜涙湭瑕嗙洊锛夛紝涓嶅緱浠呮姤鍛婅瑕嗙洊鐜?- R3. 瑕嗙洊鐜囨暟鎹彲澶嶇幇锛氱浉鍚屾祴璇曡緭鍏ュ湪鐩稿悓鐜涓嬭繍琛岋紝瑕嗙洊鐜囨暟瀛楀樊寮備笉瓒呰繃 1%
- R4. 瑕嗙洊鐜囨姤鍛婂彲杩芥函锛氭瘡涓鐩栫巼鏁版嵁鐐瑰叧鑱斿埌鍏蜂綋鐨勬祴璇曠敤渚嬪拰浠ｇ爜鎻愪氦
- R5. 缁熶竴缂栨帓 KUnit銆乲selftest銆乻yzkaller銆丗ault Injection 鍥涗釜娴嬭瘯寮曟搸锛屾敮鎸佷竴閿繍琛屽叏閮ㄦ祴璇曞浠?- R6. 姣忎釜娴嬭瘯寮曟搸鐙珛鍙繍琛岋紝鏀寔鍗曠嫭杩愯鐗瑰畾瀛愮郴缁熺殑娴嬭瘯
- R7. 娴嬭瘯澶辫触鏃惰嚜鍔ㄩ噸璇曟満鍒讹紝鍖哄垎鍋跺彂鎬уけ璐ュ拰纭畾鎬уけ璐?- R8. 娴嬭瘯鐜鍙揩閫熼噸寤猴紙浠庨厤缃埌杩愯 鈮?0 鍒嗛挓锛?- R9. 鍏ㄤ唬鐮佹爲鍚屾鎺ㄨ繘娴嬭瘯瑕嗙洊锛歚kernel/`銆乣mm/`銆乣fs/`銆乣net/`銆乣drivers/`銆乣arch/*/`銆乣lib/`銆乣include/` 鍚屾椂寤鸿锛屼笉鎺ュ彈鍒嗛樁娈佃眮鍏?- R10. 姣忎釜瀛愮郴缁熺殑娴嬭瘯浠ｇ爜閫氳繃浠ｇ爜瀹℃煡鍚庡悎骞讹紝纭繚鏂板娴嬭瘯纭疄鎻愬崌鐩爣瑕嗙洊鐜?- R11. 寤虹珛瑕嗙洊鐜囧洖褰掗槻鎶わ細鏂颁唬鐮佸悎骞朵笉寰楀鑷存暣浣撹鐩栫巼涓嬮檷
- R12. 鐢熸垚瀹¤灏辩华鎶ュ憡锛屽寘鍚暣浣撹鐩栫巼瓒嬪娍銆佸悇瀛愮郴缁熻鐩栫巼銆佹湭瑕嗙洊鍒嗘敮鍒嗘瀽
- R13. 瀹¤鎶ュ憡鍖呭惈娴嬭瘯瀹屾暣鎬ц瘉鏄庯細姣忎釜娴嬭瘯鐢ㄤ緥鐨勬墽琛岃褰曘€佽鐩栫巼璐＄尞銆佸叧鑱旂殑浠ｇ爜璺緞
- R14. 鏀寔瀹¤鏂圭嫭绔嬭繍琛屾祴璇曞浠堕獙璇佽鐩栫巼鏁版嵁锛堝彲閲嶅杩愯鐜锛?
---

## Scope Boundaries

- 浠呭叧娉ㄥ垎鏀鐩栫巼锛屼笉瑕佹眰鍏朵粬瑕嗙洊鐜囨寚鏍囷紙琛岃鐩栫巼銆佸嚱鏁拌鐩栫巼锛夎揪鏍?- 涓嶄慨鏀瑰唴鏍告簮鐮佷互鎻愰珮鍙祴璇曟€э紝闄ら潪娴嬭瘯蹇呴渶涓旂粡杩囪瘎瀹?- 涓嶆瀯寤烘柊鐨勬祴璇曟鏋讹紝澶嶇敤鐜版湁 KUnit銆乲selftest銆乻yzkaller銆丗ault Injection
- 涓嶅寘鎷€ц兘娴嬭瘯銆佸熀鍑嗘祴璇曘€佸畨鍏ㄥ鍚戠殑妯＄硦娴嬭瘯锛坰yzkaller 浠呯敤浜庤鐩栫巼锛?- 涓嶅寘鎷疄鏃惰鐩栫巼浠〃鐩橈紙瀹¤涓嶈姹傦級
- 娴嬭瘯浠ｇ爜鏀剧疆鍦ㄧ幇鏈?`tools/testing/` 鐩綍缁撴瀯涓?
---

## Context & Research

### Relevant Code and Patterns

**鍥涘紩鎿庣紪鎺掑櫒宸插瓨鍦?*锛坄tools/testing/orchestrator/`锛夛細
- `test_orchestrator.py` 鈥?缁熶竴鍏ュ彛锛屾敮鎸?`--engines`銆乣--subsystem`銆乣--retry`銆乣--coverage` 鍙傛暟
- `base_runner.py` 鈥?鎶借薄鍩虹被锛屾彁渚?`run_with_retry` 鍜?`SuiteResult` 鏁版嵁缁撴瀯
- `kunit_runner.py` 鈥?鍖呰 `tools/testing/kunit/kunit.py`锛岃В鏋?TAP 杈撳嚭
- `kselftest_runner.py` 鈥?鍖呰 `tools/testing/selftests/`锛岄€氳繃 `make TARGETS=<subsystem>` 杩愯
- `syzkaller_runner.py` 鈥?**stub**锛氱洰褰曚笉瀛樺湪鏃惰繑鍥?SKIP锛宍collect_coverage` 杩斿洖绌?dict
- `faultinj_runner.py` 鈥?閫氳繃 debugfs 鍚敤/绂佺敤鏁呴殰娉ㄥ叆锛屼絾**涓嶅疄闄呰繍琛屽瓙绯荤粺娴嬭瘯**
- `uml_runner.py` / `qemu_runner.py` 鈥?鐜 runner

**瑕嗙洊鐜囧伐鍏烽摼宸插瓨鍦?*锛坄tools/testing/coverage/`锛夛細
- `coverage_harness.py` 鈥?涓诲叆鍙ｏ紝鏀寔 gcov/kcov 閰嶇疆銆佹瀯寤恒€佹祴璇曟墽琛屻€佹暟鎹敹闆嗐€佹姤鍛婄敓鎴?- `gcov_parser.py` / `kcov_parser.py` 鈥?瑙ｆ瀽鍣?- `report_generator.py` 鈥?lcov/html 鎶ュ憡鐢熸垚

**瀹¤宸ュ叿閾惧凡瀛樺湪**锛坄tools/testing/audit/`锛夛細
- `coverage_regression.py` 鈥?鍥炲綊妫€娴?- `report_generator.py` 鈥?瀹¤鎶ュ憡鐢熸垚

**CI 娴佹按绾垮凡瀛樺湪**锛坄.gitlab-ci-coverage.yml`锛夛細
- 4 闃舵锛歜uild 鈫?test 鈫?coverage 鈫?audit
- 褰撳墠 test 闃舵浠呰繍琛岋細kunit銆乶et_core_kunit銆乫s_super_kunit銆乲selftest
- coverage 闃舵璋冪敤 `coverage_harness.py`
- audit 闃舵璋冪敤 `report_generator.py` 鍜?`coverage_regression.py`

**鐜版湁 KUnit 娴嬭瘯鍒嗗竷**锛?- `kernel/`锛? 涓祴璇曟枃浠讹紙sysctl-test.c銆乲allsyms_selftest.c銆乧rash_core_test.c銆乥acktracetest.c锛?- `mm/`锛?0+ 涓祴璇曟枃浠讹紙page_alloc銆乿ma銆乿malloc銆乻wap銆乻hmem 绛夛級
- `fs/`锛? 涓祴璇曟枃浠讹紙super銆乮node銆乨cache銆乶amei銆乺eaddir銆乻tatfs銆乴ibfs銆乪xt4锛?- `net/`锛? 涓祴璇曟枃浠讹紙core_kunit_test.c銆乻ocket_kunit_test.c锛?- `drivers/`锛氬涓祴璇曟枃浠讹紙i2c銆乬pio銆乧lk銆乼ty銆乻pi 绛夛級

**鐜版湁 kselftest 鍒嗗竷**锛?- `tools/testing/selftests/` 涓?90+ 涓瓙绯荤粺鐩綍

**Fault Injection 妗嗘灦**锛?- 鍐呮牳鍐呯疆锛歚CONFIG_FAULT_INJECTION`銆乣CONFIG_FAULT_INJECTION_DEBUG_FS`
- debugfs 鎺ュ彛锛歚/sys/kernel/debug/failslab`銆乣/sys/kernel/debug/fail_page_alloc`
- 宸ュ叿锛歚tools/testing/fault-injection/failcmd.sh`

**syzkaller 鐘舵€?*锛?- `tools/testing/syzkaller/` **涓嶅瓨鍦?*
- `syzkaller_runner.py` 涓?stub锛屽缁堣繑鍥?SKIP

### Institutional Learnings

`docs/solutions/` 鐩綍涓嶅瓨鍦紝`STRATEGY.md` 涓嶅瓨鍦ㄣ€傛湰娆″伐浣滃睘浜庤浠撳簱棣栨绯荤粺鍖栧洓寮曟搸娴嬭瘯瑕嗙洊宸ョ▼銆?
### External References

- `Documentation/dev-tools/testing-strategy.rst` 鈥?娴嬭瘯绛栫暐鏂囨。锛圞Unit/kselftest 閫夊瀷銆佷紭鍏堢骇銆乧hecklist锛?- `Documentation/dev-tools/testing-overview.rst` 鈥?妗嗘灦姒傝堪
- `Documentation/fault-injection/fault-injection.rst` 鈥?鍐呮牳鏁呴殰娉ㄥ叆妗嗘灦鏂囨。
- `tools/testing/selftests/kselftest_harness.h` 鈥?kselftest  harness

---

## Key Technical Decisions

- **浠ョ幇鏈?orchestrator 涓哄熀纭€鎵╁睍**锛歚tools/testing/orchestrator/` 宸叉湁鍥涘紩鎿庨€傞厤鍣ㄩ鏋讹紝琛ュ叏 syzkaller 鍜?Fault Injection 鐨?stub 瀹炵幇锛岃€岄潪閲嶅缓
- **syzkaller 閲囩敤澶栭儴閮ㄧ讲妯″紡**锛歚tools/testing/syzkaller/` 鐩綍涓嶅瓨鍦紝閫氳繃澶栭儴 syzkaller 浠撳簱閮ㄧ讲锛宱rchestrator 閫氳繃閰嶇疆鏂囦欢璺緞鍏宠仈
- **Fault Injection 浣滀负娴嬭瘯淇グ鍣?*锛氫笉鍗曠嫭杩愯锛岃€屾槸鍦?KUnit/kselftest 鎵ц鏈熼棿鍚敤鏁呴殰娉ㄥ叆锛屽鐢ㄧ幇鏈夋祴璇曞浠?- **瑕嗙洊鐜囧悎骞剁瓥鐣?*锛歡cov 鐢ㄤ簬 KUnit 鍜?kselftest 鐨勫叏灞€瑕嗙洊鐜囷紝kcov 鐢ㄤ簬 syzkaller 鐨?per-task 瑕嗙洊鐜囷紝鍚堝苟鏃朵互 gcov 涓轰富
- **鍩虹嚎娴嬮噺鍏堣**锛氬湪鍐欎换浣曟柊娴嬭瘯鍓嶏紝鍏堢敤鐜版湁 gcov + 鐜版湁娴嬭瘯濂椾欢璺戝嚭鍏ㄤ唬鐮佹爲鍒嗘敮瑕嗙洊鐜囧熀绾?- **CI 鎵╁睍鑰岄潪閲嶅缓**锛歚.gitlab-ci-coverage.yml` 宸叉湁 4 闃舵缁撴瀯锛屽湪鍏朵笂鎵╁睍 job 鐭╅樀

---

## Open Questions

### Resolved During Planning

- syzkaller 闆嗘垚鏂瑰紡锛歚tools/testing/syzkaller/` 涓嶅瓨鍦紝閲囩敤澶栭儴閮ㄧ讲 + 閰嶇疆璺緞鍏宠仈
- Fault Injection 杩愯妯″紡锛氫綔涓?KUnit/kselftest 鐨勪慨楗板櫒锛屽湪娴嬭瘯鎵ц鏈熼棿鍚敤 debugfs 鏁呴殰娉ㄥ叆
- 瑕嗙洊鐜囧悎骞剁瓥鐣ワ細gcov 涓轰富锛圞Unit/kselftest锛夛紝kcov 涓鸿緟锛坰yzkaller per-task锛?
### Deferred to Implementation

- 鍩虹嚎瑕嗙洊鐜囧叿浣撴暟鍊硷細闇€鍦?U1 杩愯娴嬮噺鍚庤幏寰?- 鍚勫瓙绯荤粺缂哄彛浼樺厛绾ф帓搴忥細闇€鍦?U1 鍩虹嚎鎶ュ憡鍚庢牴鎹疄闄呮暟鎹‘瀹?- syzkaller 鍏蜂綋閰嶇疆锛坈orpus 鐩綍銆乸oc 鏁伴噺銆佽秴鏃讹級锛氶渶鍦?U2 瀹炵幇鏃舵牴鎹幆澧冭皟鏁?- 鏉′欢缂栬瘧锛坄#ifdef`锛夊鑷寸殑瑕嗙洊鐜囧樊寮傚鐞嗙瓥鐣ワ細闇€鍦?U1 鏁版嵁鍒嗘瀽鍚庣‘瀹?
---

## Implementation Units

### U1. 鍩虹嚎娴嬮噺涓庣己鍙ｅ垎鏋?
**Goal:** 寤虹珛鍙鐜扮殑鍏ㄤ唬鐮佹爲鍒嗘敮瑕嗙洊鐜囧熀绾匡紝璇嗗埆鍚勫瓙绯荤粺缂哄彛鍒嗗竷

**Requirements:** R1, R2, R3, R4, R9

**Dependencies:** None

**Files:**
- Create: `tools/testing/coverage/baseline_report.json`
- Create: `tools/testing/coverage/baseline/`锛堝瓨鍌ㄥ熀绾挎暟鎹級
- Modify: `tools/testing/coverage/coverage_harness.py`锛堝闇€瑕侊級

**Approach:**
- 閰嶇疆鍐呮牳鍚敤 gcov 瑕嗙洊鐜囬噰闆嗭紙`CONFIG_GCOV_KERNEL=y`銆乣CONFIG_GCOV_PROFILE_ALL=y`锛?- 鏋勫缓娴嬭瘯鐜锛圲ML 鎴?QEMU锛?- 杩愯鐜版湁 KUnit + kselftest 濂椾欢
- 閲囬泦鍏ㄤ唬鐮佹爲鍒嗘敮瑕嗙洊鐜囨暟鎹?- 鐢熸垚鍩虹嚎鎶ュ憡锛屾寜瀛愮郴缁燂紙`kernel/`銆乣mm/`銆乣fs/`銆乣net/`銆乣drivers/`銆乣arch/*/`銆乣lib/`銆乣include/`锛夋爣娉ㄧ己鍙ｅ垎甯?- 楠岃瘉瑕嗙洊鐜囨暟鎹彲澶嶇幇鎬э紙杩炵画杩愯 3 娆★紝宸紓 鈮?%锛?
**Patterns to follow:**
- `tools/testing/coverage/coverage_harness.py` 鈥?瑕嗙洊鐜囨敹闆嗗叆鍙?- `tools/testing/coverage/gcov_parser.py` 鈥?gcov 鏁版嵁瑙ｆ瀽
- `tools/testing/audit/report_generator.py` 鈥?鎶ュ憡鐢熸垚

**Test scenarios:**
- Happy path: gcov 閰嶇疆鍚庢瀯寤烘垚鍔燂紝瑕嗙洊鐜囨暟鎹噰闆嗗畬鎴愶紝鎶ュ憡鐢熸垚
- Edge case: 鏌愪簺瀛愮郴缁熸棤浠讳綍娴嬭瘯鏃讹紝鍩虹嚎鎶ュ憡姝ｇ‘鏍囨敞涓?0%
- Error path: gcov 閰嶇疆澶辫触鏃讹紝harness 杩斿洖鏄庣‘閿欒鑰岄潪宕╂簝
- Integration: 鍩虹嚎鏁版嵁鍙 `coverage_regression.py` 娑堣垂

**Verification:**
- `tools/testing/coverage/coverage_harness.py` 鎴愬姛鐢熸垚鍏ㄤ唬鐮佹爲鍩虹嚎鎶ュ憡
- 鎶ュ憡鍖呭惈姣忎釜瀛愮郴缁熺殑鍒嗘敮瑕嗙洊鐜囨暟瀛楀拰鏈鐩栧垎鏀垪琛?- 杩炵画 3 娆¤繍琛岀粨鏋滃樊寮?鈮?%

---

### U2. syzkaller 闆嗘垚閮ㄧ讲

**Goal:** 瀹屾垚 syzkaller_runner.py 鐨?stub 瀹炵幇锛岄泦鎴?syzkaller 鍒?orchestrator

**Requirements:** R5, R6

**Dependencies:** U1

**Files:**
- Create: `tools/testing/syzkaller/`锛堥儴缃?syzkaller 浜岃繘鍒跺拰閰嶇疆妯℃澘锛?- Modify: `tools/testing/orchestrator/syzkaller_runner.py`锛堝畬鍠勫疄鐜帮級
- Modify: `tools/testing/orchestrator/test_orchestrator.py`锛堝鏋滈渶瑕侊級

**Approach:**
- 鍦?`tools/testing/syzkaller/` 涓嬮儴缃?syzkaller 浜岃繘鍒讹紙鎴栨彁渚涗笅杞借剼鏈級
- 鎻愪緵榛樿閰嶇疆鏂囦欢妯℃澘锛坄tools/testing/syzkaller/cfg/`锛?- 瀹屽杽 `syzkaller_runner.py`锛?  - `configure()`锛氫负鎸囧畾瀛愮郴缁熼€夋嫨鎴栫敓鎴?syzkaller 閰嶇疆
  - `build()`锛氭瀯寤?syzkaller 鐩爣锛堝鏋滄簮鐮佸瓨鍦級
  - `run()`锛氬惎鍔?syzkaller fuzzing 浼氳瘽锛屾敮鎸佽秴鏃舵帶鍒?  - `collect_coverage()`锛氭敹闆?kcov 瑕嗙洊鐜囨暟鎹?- 鍦?`test_orchestrator.py` 涓‘淇?syzkaller 寮曟搸鍙 `--engines` 鍙傛暟閫変腑

**Patterns to follow:**
- `tools/testing/orchestrator/syzkaller_runner.py` 鈥?鐜版湁 stub 缁撴瀯
- `tools/testing/orchestrator/base_runner.py` 鈥?鍩虹被鎺ュ彛
- `tools/testing/orchestrator/kunit_runner.py` 鈥?鎴愮啛 runner 鍙傝€?
**Test scenarios:**
- Happy path: syzkaller 鐩綍瀛樺湪鏃讹紝runner 鎴愬姛鍚姩 fuzzing 浼氳瘽
- Edge case: syzkaller 鐩綍涓嶅瓨鍦ㄦ椂锛屼紭闆呴檷绾т负 SKIP
- Error path: syzkaller 閰嶇疆鏃犳晥鏃讹紝杩斿洖 ERROR 鑰岄潪宕╂簝
- Integration: test_orchestrator.py 鐨?`--engines syzkaller` 鍙傛暟姝ｅ父宸ヤ綔

**Verification:**
- `python tools/testing/orchestrator/test_orchestrator.py --engines syzkaller --subsystem net` 姝ｅ父杩愯
- syzkaller 閰嶇疆妯℃澘鍙 fuzzing 浼氳瘽浣跨敤

---

### U3. Fault Injection 娴嬭瘯鎵╁睍

**Goal:** 瀹屽杽 faultinj_runner.py锛屼娇鍏惰兘瀹為檯杩愯瀛愮郴缁熸祴璇曞苟娉ㄥ叆鏁呴殰

**Requirements:** R5, R6

**Dependencies:** U1

**Files:**
- Modify: `tools/testing/orchestrator/faultinj_runner.py`
- Create: `tools/testing/fault-injection/subsystem_profiles/`锛堝悇瀛愮郴缁熸晠闅滄敞鍏ラ厤缃級

**Approach:**
- 瀹屽杽 `faultinj_runner.py`锛?  - `run()`锛氫笉鍐嶄粎杩斿洖 PASS锛岃€屾槸瀹為檯杩愯鐩爣瀛愮郴缁熺殑 KUnit/kselftest 濂椾欢锛屽悓鏃跺惎鐢ㄦ晠闅滄敞鍏?  - 鏀寔閰嶇疆鏁呴殰娉ㄥ叆姒傜巼鍜岀被鍨嬶紙slab銆乸age_alloc 绛夛級
  - 鏀堕泦鏁呴殰娉ㄥ叆瑙﹀彂鐨勬祴璇曠粨鏋?- 鍒涘缓瀛愮郴缁熼厤缃枃浠讹細
  - `tools/testing/fault-injection/subsystem_profiles/net.yaml`
  - `tools/testing/fault-injection/subsystem_profiles/fs.yaml`
  - `tools/testing/fault-injection/subsystem_profiles/mm.yaml`
- 姣忎釜閰嶇疆鏂囦欢瀹氫箟锛氱洰鏍囧唴鏍告ā鍧椼€佹晠闅滅被鍨嬨€佹敞鍏ユ鐜囥€侀鏈熸祴璇?
**Patterns to follow:**
- `tools/testing/orchestrator/faultinj_runner.py` 鈥?鐜版湁缁撴瀯
- `Documentation/fault-injection/fault-injection.rst` 鈥?鏁呴殰娉ㄥ叆妗嗘灦鏂囨。
- `tools/testing/fault-injection/failcmd.sh` 鈥?鍛戒护琛屽伐鍏峰弬鑰?
**Test scenarios:**
- Happy path: 杩愯 net/ 瀛愮郴缁熺殑 KUnit 娴嬭瘯鏃讹紝slab 鏁呴殰娉ㄥ叆鎸夐厤缃鐜囪Е鍙?- Edge case: 鏁呴殰娉ㄥ叆姒傜巼涓?0 鏃讹紝娴嬭瘯琛屼负涓庢棤鏁呴殰娉ㄥ叆涓€鑷?- Error path: debugfs 涓嶅彲鐢ㄦ椂锛宺unner 杩斿洖 SKIP 鑰岄潪宕╂簝
- Integration: fault injection 缁撴灉涓?KUnit/kselftest 缁撴灉鍚堝苟鍒板悓涓€ SuiteResult

**Verification:**
- `python tools/testing/orchestrator/test_orchestrator.py --engines fault_injection --subsystem net` 姝ｅ父杩愯
- 鏁呴殰娉ㄥ叆鏈熼棿鑷冲皯鏈変竴涓祴璇曞洜娉ㄥ叆鐨勬晠闅滆€屽け璐ワ紙璇佹槑娉ㄥ叆鐢熸晥锛?
---

### U4. 鍥涘紩鎿庤鐩栫巼鏁版嵁鍚堝苟

**Goal:** 瀹屽杽 test_orchestrator.py 鐨?`--coverage` 瀹炵幇锛屾敮鎸佸洓寮曟搸瑕嗙洊鐜囨暟鎹悎骞惰緭鍑?
**Requirements:** R1, R2, R5

**Dependencies:** U2, U3

**Files:**
- Modify: `tools/testing/orchestrator/test_orchestrator.py`
- Modify: `tools/testing/orchestrator/base_runner.py`锛堝闇€瑕侊級
- Modify: `tools/testing/coverage/coverage_harness.py`锛堝闇€瑕侊級

**Approach:**
- 瀹炵幇 `test_orchestrator.py` 鐨?`--coverage` 鍔熻兘锛?  - 姣忎釜 engine runner 杩愯鍚庤皟鐢?`collect_coverage()`
  - 鍚堝苟鍥涘紩鎿庣殑瑕嗙洊鐜囨暟鎹紙gcov 涓轰富锛宬cov 涓鸿緟锛?  - 鍘婚噸锛氬悓涓€浠ｇ爜璺緞琚涓紩鎿庤鐩栨椂锛屽彧璁′竴娆?- 鎵╁睍 `base_runner.py` 鐨?`SuiteResult`锛?  - 娣诲姞 `coverage_summary` 瀛楁
  - 鎻愪緵瑕嗙洊鐜囧悎骞舵柟娉?- 杈撳嚭鏍囧噯鏍煎紡鎶ュ憡锛坙cov info 鏂囦欢 + HTML 鎽樿锛?
**Patterns to follow:**
- `tools/testing/coverage/coverage_harness.py` 鈥?瑕嗙洊鐜囨敹闆嗗拰鎶ュ憡鐢熸垚
- `tools/testing/coverage/gcov_parser.py` 鈥?gcov 鏁版嵁瑙ｆ瀽
- `tools/testing/orchestrator/base_runner.py` 鈥?SuiteResult 鏁版嵁缁撴瀯

**Test scenarios:**
- Happy path: `--coverage` 杩愯鏃讹紝鍥涘紩鎿庢暟鎹悎骞惰緭鍑轰负鍗曚釜 lcov 鏂囦欢
- Edge case: 鏌愬紩鎿庤繑鍥炵┖瑕嗙洊鐜囨椂锛屽悎骞堕€昏緫姝ｇ‘澶勭悊
- Error path: 瑕嗙洊鐜囨暟鎹牸寮忓紓甯告椂锛宧arness 杩斿洖鏄庣‘閿欒
- Integration: 鍚堝苟鍚庣殑瑕嗙洊鐜囨暟鎹彲琚?`report_generator.py` 鐢熸垚 HTML 鎶ュ憡

**Verification:**
- `python tools/testing/orchestrator/test_orchestrator.py --engines kunit kselftest --coverage` 鎴愬姛鐢熸垚鍚堝苟瑕嗙洊鐜囨姤鍛?- 鎶ュ憡鍖呭惈鍒嗘敮绾у埆绮掑害鐨勮鐩栫姸鎬?
---

### U5. CI 鍏ㄤ唬鐮佹爲娴嬭瘯鐭╅樀鎵╁睍

**Goal:** 鎵╁睍 `.gitlab-ci-coverage.yml`锛屾敮鎸佸叏浠ｇ爜鏍?KUnit + kselftest + syzkaller + Fault Injection 鐭╅樀

**Requirements:** R8, R9, R12

**Dependencies:** U1, U2, U3, U4

**Files:**
- Modify: `.gitlab-ci-coverage.yml`
- Create: `tools/testing/ci/full_matrix.yml`锛堝闇€瑕侊級

**Approach:**
- 鎵╁睍 test 闃舵锛?  - 娣诲姞鎵€鏈夌幇鏈?KUnit 瀛愮郴缁熺殑涓撶敤 job锛坘ernel/銆乵m/銆乫s/銆乶et/銆乨rivers/锛?  - 娣诲姞 kselftest 鍏ㄩ噺杩愯 job
  - 娣诲姞 syzkaller job锛堟爣璁颁负 `allow_failure: true`锛屽洜涓?syzkaller 闇€瑕?VM锛?  - 娣诲姞 fault injection job
- 鎵╁睍 coverage 闃舵锛?  - 闆嗘垚鍥涘紩鎿庤鐩栫巼鍚堝苟
  - 鐢熸垚鎸夊瓙绯荤粺鐨勮鐩栫巼鎶ュ憡
- 淇濇寔 audit 闃舵涓嶅彉锛堝鐢ㄧ幇鏈?`coverage_regression.py`锛?
**Patterns to follow:**
- `.gitlab-ci-coverage.yml` 鈥?鐜版湁 stages 鍜?extends 妯℃澘
- `tools/testing/orchestrator/test_orchestrator.py` 鈥?CLI 鎺ュ彛

**Test scenarios:**
- Happy path: CI pipeline 瀹屾暣鎵ц build 鈫?test 鈫?coverage 鈫?audit 鍥涢樁娈?- Edge case: syzkaller job 澶辫触鏃朵笉褰卞搷鏁翠綋 pipeline锛坄allow_failure: true`锛?- Error path: 鏌愬瓙绯荤粺 KUnit 娴嬭瘯澶辫触鏃讹紝coverage 闃舵浠嶈繍琛屼絾鏍囪 FAIL
- Integration: 瑕嗙洊鐜囨姤鍛婃垚鍔熺敓鎴愬苟浣滀负 artifact 淇濆瓨

**Verification:**
- `.gitlab-ci-coverage.yml` 閫氳繃 GitLab CI lint 妫€鏌?- 鏂板鐨?job 鍦?GitLab CI 涓纭Е鍙?
---

### U6. 瀛愮郴缁?KUnit 娴嬭瘯鎵╁睍

**Goal:** 鎸変紭鍏堢骇琛ュ厖鍚勫瓙绯荤粺鐨?KUnit 娴嬭瘯锛屾彁鍗囧垎鏀鐩栫巼

**Requirements:** R9, R10

**Dependencies:** U1锛堝熀绾挎祴閲忓畬鎴愬悗锛屾牴鎹己鍙ｇ‘瀹氫紭鍏堢骇锛?
**Files:**
- Modify: `net/core_kunit_test.c`锛堟墿灞?net_device 娴嬭瘯锛?- Modify: `net/socket_kunit_test.c`锛堟墿灞?socket 娴嬭瘯锛?- Modify: `fs/super_kunit_test.c`锛堟墿灞?super_block 娴嬭瘯锛?- Modify: `fs/inode_kunit_test.c`锛堟墿灞?inode 娴嬭瘯锛?- Modify: `kernel/sysctl-test.c`锛堟墿灞?sysctl 娴嬭瘯锛?- Modify: `mm/page_alloc_kunit_test.c`锛堟墿灞?page alloc 娴嬭瘯锛?- 鍙兘鏂板锛歚net/neighbour_kunit_test.c`銆乣fs/dentry_kunit_test.c` 绛?
**Approach:**
- 鍩轰簬 U1 鐨勫熀绾挎姤鍛婏紝纭畾鍚勫瓙绯荤粺鐨勭己鍙ｄ紭鍏堢骇
- 浼樺厛瑕嗙洊 P1 鏍稿績鏁版嵁缁撴瀯锛坣et_device銆乻uper_block銆乮node銆乵m_struct銆乼ask_struct锛?- 姣忎釜鏂版祴璇曢伒寰幇鏈?`-test.c` 鍛藉悕绾﹀畾鍜?`kunit_test_suite()` 娉ㄥ唽妯″紡
- 鎵€鏈夋祴璇曢€氳繃 `scripts/checkpatch.pl` 鍜?`scripts/spdxcheck.py` 妫€鏌?
**Patterns to follow:**
- `net/core_kunit_test.c` 鈥?鐜版湁 net/ KUnit 娴嬭瘯妯″紡
- `fs/inode_kunit_test.c` 鈥?inode 鐢熷懡鍛ㄦ湡娴嬭瘯鍙傝€?- `mm/page_alloc_kunit_test.c` 鈥?澶嶆潅缁撴瀯鍒濆鍖栨祴璇曞弬鑰?
**Test scenarios:**
- Happy path: 鏂板娴嬭瘯鍦?`make kunit` 涓?PASS
- Edge case: 杈圭晫鏉′欢锛堢┖鎸囬拡銆侀浂澶у皬銆佹渶澶у€硷級姝ｇ‘瑙﹀彂 EXPECT/ASSERT
- Error path: 閿欒杈撳叆瑙﹀彂 EXPECT_FAIL 鑰岄潪宕╂簝
- Integration: 鏂版祴璇曞湪 CI 涓嚜鍔ㄨ繍琛屽苟閫氳繃

**Verification:**
- `make O=build kunit` 杩愯骞堕€氳繃鎵€鏈夋柊澧炴祴璇?- 鏂版祴璇曞湪 TAP 杈撳嚭涓樉绀?PASS
- 鍒嗘敮瑕嗙洊鐜囧熀绾挎姤鍛婃樉绀哄搴斿瓙绯荤粺瑕嗙洊鐜囨彁鍗?
---

### U7. 瀹¤灏辩华鎶ュ憡涓庡洖褰掗槻鎶?
**Goal:** 寤虹珛瀹¤灏辩华鎶ュ憡鏍煎紡鍜岃鐩栫巼鍥炲綊闃叉姢鏈哄埗

**Requirements:** R11, R12, R13, R14

**Dependencies:** U4, U5

**Files:**
- Modify: `tools/testing/audit/report_generator.py`
- Modify: `tools/testing/audit/coverage_regression.py`
- Create: `tools/testing/audit/audit_readme.md`锛堝璁℃柟杩愯鎸囧崡锛?
**Approach:**
- 鎵╁睍 `report_generator.py`锛?  - 鐢熸垚瀹¤灏辩华鎶ュ憡锛圡arkdown + HTML锛夛紝鍖呭惈鏁翠綋瑕嗙洊鐜囪秼鍔裤€佸悇瀛愮郴缁熻鐩栫巼銆佹湭瑕嗙洊鍒嗘敮鍒嗘瀽
  - 姣忎釜娴嬭瘯鐢ㄤ緥鍏宠仈瑕嗙洊鐜囪础鐚?- 鎵╁睍 `coverage_regression.py`锛?  - 鏀寔鍏ㄤ唬鐮佹爲鍥炲綊妫€娴?  - 鏂颁唬鐮佸悎骞舵椂鑷姩妫€鏌ユ槸鍚﹀鑷磋鐩栫巼涓嬮檷
- 鍒涘缓瀹¤鏂硅繍琛屾寚鍗楋細
  - 鐜鍑嗗姝ラ
  - 鐙珛杩愯娴嬭瘯濂椾欢鐨勬柟娉?  - 楠岃瘉瑕嗙洊鐜囨暟鎹殑姝ラ

**Patterns to follow:**
- `tools/testing/audit/report_generator.py` 鈥?鐜版湁鎶ュ憡鐢熸垚
- `tools/testing/audit/coverage_regression.py` 鈥?鐜版湁鍥炲綊妫€娴?
**Test scenarios:**
- Happy path: 瀹¤鏂规寜鐓ф寚鍗楃嫭绔嬭繍琛屾祴璇曪紝寰楀埌鐨勮鐩栫巼鏁版嵁涓庨」鐩姤鍛婁竴鑷达紙宸紓 鈮?%锛?- Edge case: 鍩虹嚎鏁版嵁缂哄け鏃讹紝鎶ュ憡鎻愮ず闇€瑕佸厛杩愯鍩虹嚎娴嬮噺
- Error path: 瑕嗙洊鐜囦笅闄嶆椂锛宺egression 妫€娴嬫纭爣璁板苟闃绘鍚堝苟
- Integration: 鎶ュ憡鏁版嵁鍙瀹¤鏂硅В鏋愬拰楠岃瘉

**Verification:**
- `tools/testing/audit/report_generator.py` 鎴愬姛鐢熸垚瀹¤灏辩华鎶ュ憡
- `tools/testing/audit/coverage_regression.py` 姝ｇ‘妫€娴嬭鐩栫巼鍥炲綊
- 瀹¤鏂瑰彲鍦ㄧ┖鐧界幆澧冧腑澶嶇幇娴嬭瘯缁撴灉

---

## System-Wide Impact

- **Interaction graph:** U2 鍜?U3 鏂板鐨?engine runner 琚?`test_orchestrator.py` 璋冪敤锛沀4 鐨勮鐩栫巼鍚堝苟閫昏緫褰卞搷鎵€鏈?engine runner 鐨勮緭鍑烘牸寮忥紱U5 鐨?CI 鐭╅樀鍦?merge request 鏃惰嚜鍔ㄨЕ鍙?- **Error propagation:** CI 娴佹按绾跨殑 test 闃舵澶辫触闃绘 coverage 鍜?audit 闃舵鎵ц锛堢幇鏈?stages 渚濊禆鍏崇郴锛夛紱syzkaller job 鏍囪涓?`allow_failure`锛屼笉闃绘柇 pipeline
- **State lifecycle risks:** syzkaller 闇€瑕佸閮?VM/QEMU 鐜锛孋I 涓彲鑳戒笉鍙敤锛汧ault Injection 闇€瑕?debugfs 鎸傝浇锛孶ML 鐜鍙兘鏈夐檺鍒?- **API surface parity:** 鏂板娴嬭瘯涓嶄慨鏀逛换浣曞唴鏍?API锛屼粎澧炲姞娴嬭瘯瑕嗙洊
- **Integration coverage:** U4 鐨勮鐩栫巼鍚堝苟闇€楠岃瘉鍦?gcov + kcov 娣峰悎鍦烘櫙涓嬬殑鍑嗙‘鎬?- **Unchanged invariants:** 鐜版湁 KUnit/kselftest 鐨勬祴璇曟帴鍙ｅ拰 CI 闃舵缁撴瀯淇濇寔涓嶅彉

---

## Risks & Dependencies

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| syzkaller 鐩綍涓嶅瓨鍦紝闇€澶栭儴閮ㄧ讲 | High | Medium | U2 鎻愪緵涓嬭浇鑴氭湰鍜岄厤缃ā鏉匡紝CI 涓爣璁颁负 allow_failure |
| Fault Injection 鍦?UML 鐜涓?debugfs 涓嶅彲鐢?| Medium | Medium | U3 妫€娴?debugfs 鍙敤鎬э紝涓嶅彲鐢ㄦ椂浼橀泤闄嶇骇涓?SKIP |
| 鍩虹嚎娴嬮噺鏃堕棿杩囬暱锛堝叏浠ｇ爜鏍戞瀯寤?+ 娴嬭瘯锛?| High | Low | 浣跨敤 `make O=build` 鏍戝鏋勫缓锛屽埄鐢?ccache 鍔犻€熼噸澶嶆瀯寤?|
| 瑕嗙洊鐜囨暟鎹悎骞朵笉涓€鑷达紙gcov vs kcov锛?| Medium | High | U4 浠?gcov 涓轰富锛宬cov 涓鸿緟锛屽悎骞舵椂鍘婚噸 |
| 鍏ㄤ唬鐮佹爲鍚屾鎺ㄨ繘璧勬簮娑堣€楀ぇ | High | High | U1 鍩虹嚎娴嬮噺鍚庯紝鏍规嵁瀹為檯缂哄彛璋冩暣璧勬簮鍒嗛厤浼樺厛绾?|
| 瀹¤鏂瑰鎶ュ憡鏍煎紡鏈夐澶栬姹?| Medium | High | U7 鎻愪緵澶氱鏍煎紡杈撳嚭锛圡arkdown銆丠TML銆丣SON锛夛紝棰勭暀鑷畾涔夋帴鍙?|

---

## Documentation / Operational Notes

- U1 浜у嚭鐗╀负鍩虹嚎瑕嗙洊鐜囨姤鍛婏紝鎸夊瓙绯荤粺鍒嗙被
- U2 浜у嚭鐗╀负 syzkaller 閮ㄧ讲鎸囧崡鍜岄厤缃ā鏉?- U3 浜у嚭鐗╀负 fault injection  subsystem profiles
- U7 浜у嚭鐗╀负瀹¤鏂硅繍琛屾寚鍗?- 鎵€鏈夋祴璇曚唬鐮侀渶閫氳繃 `scripts/checkpatch.pl` 鍜?`scripts/spdxcheck.py` 妫€鏌?- CI 淇敼闇€鍦?GitLab CI 涓獙璇?pipeline 閫氳繃

---

## Sources & References

- **Origin document:** [docs/brainstorms/2026-07-08-multi-engine-full-coverage-requirements.md](docs/brainstorms/2026-07-08-multi-engine-full-coverage-requirements.md)
- Related code: [tools/testing/orchestrator/test_orchestrator.py](tools/testing/orchestrator/test_orchestrator.py), [tools/testing/orchestrator/syzkaller_runner.py](tools/testing/orchestrator/syzkaller_runner.py), [tools/testing/orchestrator/faultinj_runner.py](tools/testing/orchestrator/faultinj_runner.py), [tools/testing/coverage/coverage_harness.py](tools/testing/coverage/coverage_harness.py), [.gitlab-ci-coverage.yml](.gitlab-ci-coverage.yml)
- Related docs: [Documentation/dev-tools/testing-strategy.rst](Documentation/dev-tools/testing-strategy.rst), [Documentation/fault-injection/fault-injection.rst](Documentation/fault-injection/fault-injection.rst)
- Existing tests: [net/core_kunit_test.c](net/core_kunit_test.c), [fs/super_kunit_test.c](fs/super_kunit_test.c), [kernel/sysctl-test.c](kernel/sysctl-test.c), [mm/page_alloc_kunit_test.c](mm/page_alloc_kunit_test.c)
