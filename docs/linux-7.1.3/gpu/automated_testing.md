
## DRM 瀛愮郴缁熺殑鑷姩鍖栨祴璇?


## 绠€浠?


褰撻渶瑕佹祴璇曞ぇ閲忎笉鍚岀殑纭欢閰嶇疆鏃讹紝纭繚瀵规牳蹇冩垨椹卞姩鐨勪慨鏀逛笉浼氬紩鍏ュ洖褰掑彲鑳戒細闈炲父鑰楁椂銆傛澶栵紝瀵逛簬姣忎釜鏈夋剰杩涜姝ょ被娴嬭瘯鐨勪汉鏉ヨ锛屽幓鑾峰彇骞剁淮鎶ゅ彲鑳界浉褰撳彲瑙傜殑纭欢璁惧鏄笉鍒囧疄闄呯殑銆?

鍚屾椂锛屽紑鍙戣€呮渶濂借兘澶熻嚜琛屾鏌ュ叾浠ｇ爜涓殑鍥炲綊锛岃€屼笉鏄緷璧栫淮鎶よ€呭幓鍙戠幇瀹冧滑鍐嶅洖棣堟姤鍛娿€?

gitlab.freedesktop.org 涓婃彁渚涗簡鐢ㄤ簬鑷姩娴嬭瘯 Mesa 鐨勮鏂斤紝鍚屾牱鍙敤浜庢祴璇?DRM 瀛愮郴缁熴€傛湰鏂囨。璇存槑浜嗘湁鎰忚繘琛屾祴璇曠殑浜哄浣曞埄鐢ㄨ繖濂楀叡浜殑鍩虹璁炬柦锛屼粠鑰岃妭鐪佺浉褰撳鐨勬椂闂村拰绮惧姏銆?


## 鐩稿叧鏂囦欢


### drivers/gpu/drm/ci/gitlab-ci.yml


杩欐槸 GitLab CI 鐨勬牴閰嶇疆鏂囦欢銆傞櫎鍏朵粬涓嶅お閲嶈鐨勫唴瀹瑰锛屽畠杩樻寚瀹氫簡瑕佷娇鐢ㄧ殑鑴氭湰鐨勫叿浣撶増鏈€傛湁涓€浜涘彉閲忓彲浠ヤ慨鏀逛互鏀瑰彉娴佹按绾跨殑琛屼负锛?

DRM_CI_PROJECT_PATH
    鍖呭惈 CI 鎵€鐢?Mesa 杞欢鍩虹璁炬柦鐨勪唬鐮佷粨搴?

DRM_CI_COMMIT_SHA
    瑕佷粠璇ヤ唬鐮佷粨搴撲娇鐢ㄧ殑鐗瑰畾淇鐗堟湰

UPSTREAM_REPO
    鍖呭惈鐩爣鍒嗘敮鐨?git 浠撳簱鐨?URL

TARGET_BRANCH
    鏈垎鏀皢瑕佸悎骞跺埌鐨勭洰鏍囧垎鏀?

IGT_VERSION
    鎵€浣跨敤鐨?igt-gpu-tools 鐨勪慨璁㈢増鏈紝鏉ヨ嚜
    https://gitlab.freedesktop.org/drm/igt-gpu-tools

### drivers/gpu/drm/ci/testlist.txt


瑕佸湪鎵€鏈夐┍鍔ㄤ笂杩愯鐨?IGT 娴嬭瘯锛堥櫎闈炲湪鏌愪釜椹卞姩鐨?\*-skips.txt 鏂囦欢涓湁鎵€鎻愬強锛岃涓嬫枃锛夈€?

### drivers/gpu/drm/ci/${DRIVER_NAME}-${HW_REVISION}-fails.txt


鍒楀嚭鏌愪釜椹卞姩鍦ㄦ煇涓€鐗瑰畾纭欢淇鐗堟湰涓婄殑宸茬煡澶辫触椤广€?

### drivers/gpu/drm/ci/${DRIVER_NAME}-${HW_REVISION}-flakes.txt


鍒楀嚭鏌愪釜椹卞姩鍦ㄦ煇涓€鐗瑰畾纭欢淇鐗堟湰涓婂凡鐭ヨ涓轰笉鍙潬鐨勬祴璇曘€傛棤璁虹粨鏋滃浣曪紝杩欎簺娴嬭瘯閮戒笉浼氬鑷翠綔涓氬け璐ャ€傚畠浠粛浼氳杩愯銆?

姣忎釜鏂扮殑 flake 鏉＄洰閮藉繀椤诲叧鑱斾竴涓寚鍚戦偖浠剁殑閾炬帴锛岃閭欢鍚戝彈褰卞搷椹卞姩鐨勪綔鑰呮垨鐩稿叧 GitLab issue 鎶ュ憡浜嗚缂洪櫡銆傝鏉＄洰杩樺繀椤诲寘鍚澘鍗″悕绉版垨璁惧鏍戝悕绉般€侀涓彈褰卞搷鐨勫唴鏍哥増鏈€佺敤浜庢祴璇曠殑 IGT 鐗堟湰锛屼互鍙婂け璐ョ巼鐨勮繎浼煎€笺€?

```

  # Bug Report: $LORE_URL_OR_GITLAB_ISSUE
  # Board Name: broken-board.dtb
  # Linux Version: 6.6-rc1
  # IGT Version: 1.28-gd2af13d9f
  # Failure Rate: 100
  flaky-test

```
浣跨敤涓嬫柟鐩稿簲鐨勯摼鎺ユ潵鍒涘缓涓€涓?GitLab issue锛?
amdgpu driver: https://gitlab.freedesktop.org/drm/amd/-/issues
i915 driver: https://gitlab.freedesktop.org/drm/i915/kernel/-/issues
msm driver: https://gitlab.freedesktop.org/drm/msm/-/issues
xe driver: https://gitlab.freedesktop.org/drm/xe/kernel/-/issues

### drivers/gpu/drm/ci/${DRIVER_NAME}-${HW_REVISION}-skips.txt


鍒楀嚭鏌愪釜椹卞姩鍦ㄦ煇涓€鐗瑰畾纭欢淇鐗堟湰涓婁笉浼氳杩愯鐨勬祴璇曘€傝繖浜涢€氬父鏄細鍥犳寕璧锋満鍣ㄣ€佸鑷?OOM銆佽€楁椂杩囬暱绛夊師鍥犺€屽共鎵版祴璇曞垪琛ㄨ繍琛岀殑娴嬭瘯銆?


## 濡備綍鍦ㄤ綘鑷繁鐨勪唬鐮佹爲涓婂惎鐢ㄨ嚜鍔ㄥ寲娴嬭瘯


1. 濡傛灉浣犺繕娌℃湁鐨勮瘽锛屽湪 https://gitlab.freedesktop.org/ 涓婂垱寤轰竴涓?Linux 浠ｇ爜鏍?

2. 鍦ㄤ綘鐨勫唴鏍镐粨搴撶殑閰嶇疆涓紙渚嬪
   https://gitlab.freedesktop.org/janedoe/linux/-/settings/ci_cd锛夛紝灏?
   CI/CD 閰嶇疆鏂囦欢浠?.gitlab-ci.yml 鏀逛负
   drivers/gpu/drm/ci/gitlab-ci.yml銆?

3. 璇锋眰琚坊鍔犲埌 drm/ci-ok 缁勶紝浣夸綘鐨勭敤鎴锋嫢鏈夊湪
   https://gitlab.freedesktop.org/drm/ci-ok 涓婅繍琛?CI 鎵€闇€鐨勬潈闄?

4. 涓嬫浣犳帹閫佸埌璇ヤ唬鐮佷粨搴撴椂锛屼綘灏嗙湅鍒颁竴鏉?CI 娴佹按绾胯鍒涘缓锛堜緥濡?
   https://gitlab.freedesktop.org/janedoe/linux/-/pipelines锛?

5. 鍚勯」浣滀笟灏嗕細杩愯锛屽綋娴佹按绾跨粨鏉熸椂锛岄櫎闈炲彂鐜颁簡鍥炲綊锛屽惁鍒欐墍鏈変綔涓氶兘搴斿綋鏄豢鑹茬殑銆?

6. 娴佹按绾夸腑鐨勮鍛婅〃鏄庯紝鍦ㄦ祴璇曟湡闂存娴嬪埌浜?lockdep
   锛堝弬瑙?Documentation/locking/lockdep-design.rst锛夐棶棰樸€?


## 濡備綍鏇存柊娴嬭瘯鏈熸湜


濡傛灉浣犲浠ｇ爜鐨勪慨鏀逛慨澶嶄簡鏌愪簺娴嬭瘯锛屼綘灏嗛渶瑕佷粠
drivers/gpu/drm/ci/${DRIVER_NAME}_*_fails.txt 涓彈璇ヤ慨鏀瑰奖鍝嶇殑姣忎釜娴嬭瘯骞冲彴瀵瑰簲鐨勬枃浠朵腑锛?
鍒犻櫎涓€琛屾垨澶氳銆?


## 濡備綍鎵╁睍娴嬭瘯瑕嗙洊


濡傛灉浣犵殑浠ｇ爜淇敼浣垮緱鍙互杩愯鏇村娴嬭瘯锛堜緥濡傞€氳繃瑙ｅ喅鍙潬鎬ч棶棰橈級锛屼綘鍙互浠?flakes 鍜?鎴?skips 鍒楄〃涓Щ闄ゆ祴璇曪紝浠ュ強锛堝鏋滃瓨鍦ㄥ凡鐭ュけ璐ワ級鐩稿簲鐨勯鏈熺粨鏋溿€?

濡傛灉闇€瑕佹洿鏂版墍浣跨敤鐨?IGT 鐗堟湰锛堜篃璁镐綘鍚戝叾涓坊鍔犱簡鏇村娴嬭瘯锛夛紝璇锋洿鏂?gitlab-ci.yml 鏂囦欢椤堕儴鐨?IGT_VERSION 鍙橀噺銆?


## 濡備綍娴嬭瘯浣犲鑴氭湰鐨勪慨鏀?


涓轰簡娴嬭瘯瀵?drm-ci 浠撳簱涓剼鏈殑淇敼锛岃灏?
drivers/gpu/drm/ci/gitlab-ci.yml 涓殑 DRM_CI_PROJECT_PATH 鍜?DRM_CI_COMMIT_SHA 鍙橀噺鏀逛负涓庝綘鐨勯」鐩垎鏀紙渚嬪 janedoe/drm-ci锛夌浉鍖归厤銆傝鍒嗘敮闇€瑕佷綅浜?https://gitlab.freedesktop.org/銆?


## 濡備綍鍦ㄦ祴璇曚腑寮曞叆澶栭儴淇


閫氬父锛屽叾浠栦唬鐮佹爲涓殑鍥炲綊浼氶樆姝㈠褰撳墠琚祴浠ｇ爜鏍戜腑鏈湴淇敼鐨勬祴璇曘€傝繖浜涗慨澶嶄細鍦ㄦ瀯寤轰綔涓氭湡闂翠粠鐩爣浠ｇ爜鏍戜腑涓€涓悕涓?
${TARGET_BRANCH}-external-fixes 鐨勫垎鏀嚜鍔ㄥ悎骞惰繘鏉ャ€?

濡傛灉娴佹按绾夸笉鍦ㄥ悎骞惰姹備腑锛屽苟涓旀湰鍦颁唬鐮佹爲涓瓨鍦ㄥ悓鍚嶇殑鍒嗘敮锛岄偅涔堣鍒嗘敮涓殑鎻愪氦涔熶細琚悎骞惰繘鏉ャ€?


## 濡備綍澶勭悊鍙兘瀹曟満鐨勮嚜鍔ㄥ寲娴嬭瘯瀹為獙瀹?


濡傛灉鏌愪釜纭欢鍐滃満瀹曟満锛屼粠鑰屽鑷存湰搴旈€氳繃鍗翠娇娴佹按绾垮け璐ワ紝鍙互閫氳繃缂栬緫
https://gitlab.freedesktop.org/gfx-ci/lab-status/-/blob/main/lab-status.yml 澶勭殑鏂囦欢锛?
鏉ョ鐢ㄦ墍鏈夊皢琚彁浜ゅ埌璇ョ‖浠跺啘鍦虹殑浣滀笟銆?
