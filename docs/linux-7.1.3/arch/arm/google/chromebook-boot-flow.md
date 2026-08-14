
## Chromebook 鍚姩娴佺▼


澶у鏁拌繎鏈熶娇鐢ㄨ澶囨爲锛坉evice tree锛夌殑 Chromebook 浣跨敤寮€婧愮殑 depthcharge_ 寮曞鍔犺浇绋嬪簭銆俤epthcharge_ 鏈熸湜鎿嶄綔绯荤粺琚墦鍖呬负涓€涓?`FIT Image`_锛屽叾涓寘鍚搷浣滅郴缁熼暅鍍忎互鍙婁竴缁勮澶囨爲銆傜敱 depthcharge_ 浠?`FIT Image`_ 涓寫閫夊嚭姝ｇ‘鐨勮澶囨爲骞舵彁渚涚粰鎿嶄綔绯荤粺銆?
depthcharge_ 鐢ㄦ潵鎸戦€夎澶囨爲鐨勬柟妗堣€冭檻浜嗕笁涓彉閲忥細

- 鏉垮悕锛圔oard name锛夛紝鍦?depthcharge_ 缂栬瘧鏃舵寚瀹氥€傚嵆涓嬮潰鐨?$(BOARD)銆?- 鏉夸慨璁㈠彿锛圔oard revision number锛夛紝鍦ㄨ繍琛屾椂纭畾锛堝彲鑳介€氳繃璇诲彇 GPIO 纭欢閰嶇疆锛坰trapping锛夛紝涔熷彲鑳介€氳繃鍏朵粬鏂规硶锛夈€傚嵆涓嬮潰鐨?$(REV)銆?- SKU 鍙凤紝鍦ㄥ惎鍔ㄦ椂浠?GPIO 纭欢閰嶇疆涓鍙栥€傚嵆涓嬮潰鐨?$(SKU)銆?
瀵逛簬杩戞湡鐨?Chromebook锛宒epthcharge_ 鍒涘缓鐨勫尮閰嶅垪琛ㄥ涓嬶細

- google,$(BOARD)-rev$(REV)-sku$(SKU)
- google,$(BOARD)-rev$(REV)
- google,$(BOARD)-sku$(SKU)
- google,$(BOARD)

娉ㄦ剰锛屼竴浜涜緝鏃х殑 Chromebook 浣跨敤鐣ユ湁涓嶅悓銆佸彲鑳戒笉鍖呭惈 SKU 鍖归厤鎴栧彲鑳戒互涓嶅悓浼樺厛绾у寰?SKU/rev 鐨勫垪琛ㄣ€?
娉ㄦ剰锛屽浜庢煇浜涙澘瀛愶紝鍙兘鏈夐澶栫殑鏉跨骇鐗瑰畾閫昏緫鍚戝垪琛ㄤ腑娉ㄥ叆棰濆鐨?compatible 瀛楃涓诧紝浣嗚繖骞朵笉甯歌銆?
depthcharge_ 浼氶亶鍘?`FIT Image`_ 涓殑鎵€鏈夎澶囨爲锛岃瘯鍥炬壘鍒板尮閰嶆渶鍏蜂綋锛坢ost specific锛塩ompatible 鐨勯偅涓€涓€傜劧鍚庡畠浼氶亶鍘?`FIT Image`_ 涓殑鎵€鏈夎澶囨爲锛岃瘯鍥炬壘鍒板尮閰?*绗簩鍏蜂綋** compatible 鐨勯偅涓€涓紝渚濇绫绘帹銆?
鍦ㄦ悳绱㈣澶囨爲鏃讹紝depthcharge_ 骞朵笉鍏冲績 compatible 瀛楃涓插湪璁惧鏍戞牴 compatible 瀛楃涓叉暟缁勪腑鐨勪綅缃€備緥濡傦紝濡傛灉鎴戜滑鍦?"lazor" 鏉裤€乺ev 4銆丼KU 0 涓婏紝骞朵笖鏈変袱妫佃澶囨爲锛?
- "google,lazor-rev5-sku0", "google,lazor-rev4-sku0", "qcom,sc7180"
- "google,lazor", "qcom,sc7180"

閭ｄ箞 depthcharge_ 浼氶€夋嫨绗竴妫佃澶囨爲锛屽嵆浣?"google,lazor-rev4-sku0" 鏄偅妫佃澶囨爲涓垪鍑虹殑绗簩涓?compatible銆傝繖鏄洜涓哄畠姣?"google,lazor" 鏇村叿浣撱€?
闇€瑕佹敞鎰忕殑鏄紝depthcharge_ 娌℃湁浠讳綍鏅鸿兘鍘诲皾璇曞尮閰?鐩歌繎"鐨勬澘瀛愭垨 SKU 淇鐗堟湰銆備篃灏辨槸璇达紝濡傛灉 depthcharge_ 鐭ラ亾鑷繁鍦ㄦ煇鍧楁澘鐨?"rev4" 涓婏紝浣嗘病鏈?"rev4" 鐨勮澶囨爲锛岄偅涔?depthcharge_ **涓嶄細**鍘诲鎵?"rev3" 鐨勮澶囨爲銆?
涓€鑸€岃█锛屽綋瀵逛竴鍧楁澘瀛愬仛鍑轰换浣曢噸澶ф敼鍔ㄦ椂锛屽嵆浣垮叾涓病鏈変换浣曟敼鍔ㄩ渶瑕佸湪璁惧鏍戜腑浣撶幇锛屾澘淇鍙蜂篃浼氬鍔犮€傚洜姝ょ湅鍒板寘鍚涓慨璁㈢増鏈殑璁惧鏍戞槸鐩稿綋甯歌鐨勩€?
搴斿綋娉ㄦ剰锛岃€冭檻鍒?depthcharge_ 涓婅堪鐨勮繖濂楁満鍒讹紝濡傛灉鏀寔鏌愬潡鏉挎渶鏂颁慨璁㈢増鏈殑璁惧鏍戠渷鐣ヤ簡 "-rev{REV}" compatible 瀛楃涓诧紝灏辫兘鑾峰緱鏈€澶х殑鐏垫椿鎬с€傝繖鏍峰仛涔嬪悗锛屽鏋滀綘鎷垮埌涓€鍧楁柊鐨勬澘淇鐗堟湰骞惰瘯鍥惧湪鍏朵笂杩愯鏃ц蒋浠讹紝閭ｄ箞鎴戜滑鑷冲皯鑳芥寫閫夊埌鎴戜滑鎵€鐭ョ殑鏈€鏂扮殑璁惧鏍戙€?