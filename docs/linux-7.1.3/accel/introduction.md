
## 绠€浠?

Linux 璁＄畻鍔犻€熷櫒锛坈ompute accelerator锛夊瓙绯荤粺鏃ㄥ湪浠ョ粺涓€鐨勬柟寮忓悜鐢ㄦ埛绌洪棿
鏆撮湶璁＄畻鍔犻€熷櫒锛屽苟鎻愪緵涓€缁勯€氱敤鐨勫姛鑳姐€?
杩欎簺璁惧鏃㈠彲浠ユ槸鐙珛鐨?ASIC锛屼篃鍙互鏄?SoC/GPU 鍐呴儴鐨?IP 妯″潡銆傚敖绠¤繖浜涜澶?閫氬父璁捐鐢ㄤ簬鍔犻€熸満鍣ㄥ涔狅紙ML锛夊拰/鎴栨繁搴﹀涔狅紙DL锛夎绠楋紝浣?accel 灞傚苟涓嶉檺浜?澶勭悊杩欑被鍔犻€熷櫒銆?
閫氬父锛屼竴涓绠楀姞閫熷櫒浼氬睘浜庝互涓嬬被鍒箣涓€锛?
- 杈圭紭 AI锛圗dge AI锛夆€斺€斿湪杈圭紭璁惧涓婅繘琛屾帹鐞嗐€傚畠鍙互鏄竴涓祵鍏ュ紡 ASIC/FPGA锛?  鎴?SoC 鍐呴儴鐨?IP锛堜緥濡傜瑪璁版湰鐢佃剳鐨勬憚鍍忓ご锛夈€傝繖绫昏澶囬€氬父閫氳繃瀵勫瓨鍣ㄩ厤缃紝
  骞朵笖鍙互鍦ㄦ湁鎴栨病鏈?DMA 鐨勬儏鍐典笅宸ヤ綔銆?
- 鎺ㄧ悊鏁版嵁涓績锛圛nference data-center锛夆€斺€斿ぇ鍨嬫湇鍔″櫒涓崟鐢ㄦ埛/澶氱敤鎴风殑璁惧銆?  杩欑被璁惧鍙互鏄嫭绔嬬殑锛屼篃鍙互鏄?SoC 鎴?GPU 鍐呴儴鐨?IP銆傚畠甯︽湁鏉胯浇 DRAM
  锛堢敤浜庡瓨鏀?DL 鎷撴墤锛夈€丏MA 寮曟搸浠ュ強鍛戒护鎻愪氦闃熷垪锛堝唴鏍告€佹垨鐢ㄦ埛鎬侀槦鍒楋級銆?  瀹冨彲鑳借繕甯︽湁鐢ㄤ簬绠＄悊澶氱敤鎴风殑 MMU锛屽苟鍙兘鍚敤铏氭嫙鍖栵紙SR-IOV锛変互鍦ㄥ悓涓€璁惧
  涓婃敮鎸佸涓櫄鎷熸満銆傛澶栵紝杩欎簺璁惧閫氬父杩樹細甯︽湁涓€浜涘伐鍏凤紝渚嬪鎬ц兘鍒嗘瀽鍣?  锛坧rofiler锛夊拰璋冭瘯鍣ㄣ€?
- 璁粌鏁版嵁涓績锛圱raining data-center锛夆€斺€斾笌鎺ㄧ悊鏁版嵁涓績鍗＄被浼硷紝浣嗛€氬父鍏锋湁
  鏇村己鐨勭畻鍔涘拰鍐呭瓨甯﹀锛堜緥濡?HBM锛夛紝骞朵笖寰堝彲鑳藉叿澶囨墿缂╋紙scale-up/out锛夋墜娈碉紝
  鍗冲垎鍒繛鎺ュ埌鏈嶅姟鍣ㄥ唴鎴栨湇鍔″櫒闂寸殑鍏跺畠璁粌鍗°€?
鎵€鏈夎繖浜涜澶囬€氬父閮芥湁鍚勮嚜瀹氬埗鐨勮繍琛屾椂鐢ㄦ埛绌洪棿杞欢鏍堬紝涓撻棬閫傞厤鍏剁‖浠躲€傛澶栵紝
瀹冧滑寰堝彲鑳借繕鍖呭惈涓€涓紪璇戝櫒锛岀敤浜庝负鍏跺畾鍒惰绠楀紩鎿庣敓鎴愮▼搴忋€傞€氬父锛岀敤鎴风┖闂翠腑
鐨勯€氱敤灞傛槸 DL 妗嗘灦锛屼緥濡?PyTorch 鍜?TensorFlow銆?
## 涓?DRM 鍏变韩浠ｇ爜


鐢变簬杩欑被璁惧鍙互鏄?GPU 鍐呴儴鐨?IP锛屾垨鍏锋湁涓?GPU 绫讳技鐨勭壒寰侊紝accel 瀛愮郴缁熷皢
澶嶇敤 DRM 瀛愮郴缁熺殑浠ｇ爜涓庡姛鑳姐€備篃灏辨槸璇达紝accel 鏍稿績浠ｇ爜灏嗘垚涓?DRM 瀛愮郴缁?鐨勪竴閮ㄥ垎锛岃€屼竴涓?accel 璁惧灏嗘槸涓€绉嶆柊鍨嬬殑 DRM 璁惧銆?
杩欏皢浣挎垜浠兘澶熷埄鐢ㄥ簽澶х殑 DRM 浠ｇ爜搴擄紝骞朵笌鍏锋湁姝ょ被璁惧缁忛獙鐨?DRM 寮€鍙戣€呭崗浣溿€?姝ゅ锛屼负鍔犻€熷櫒椹卞姩鏂板鐨勭壒鎬т篃鍙兘瀵?GPU 椹卞姩鏈夌敤銆?
## 涓?GPU 鐨勫尯鍒?

鍥犱负鎴戜滑甯屾湜閬垮厤搴炲ぇ鐨勭敤鎴风┖闂村浘褰㈣蒋浠舵爤璇曞浘灏嗗姞閫熷櫒褰撲綔 GPU 鏉ヤ娇鐢紝璁＄畻
鍔犻€熷櫒灏嗛€氳繃鏂扮殑涓昏澶囧彿锛坢ajor number锛夊拰鏂扮殑瀛楃璁惧鏂囦欢涓?GPU 鍖哄垎寮€鏉ャ€?
姝ゅ锛岃繖浜涢┍鍔ㄥ皢浣嶄簬鍐呮牳鏍戜腑涓€涓嫭绔嬬殑浣嶇疆鈥斺€攄rivers/accel/銆?
鍔犻€熷櫒璁惧灏嗕互涓撶敤鐨?261 涓昏澶囧彿鏆撮湶缁欑敤鎴风┖闂达紝骞堕伒寰互涓嬬害瀹氾細

- 瀛楃璁惧鏂囦欢 - /dev/accel/accel\*
- sysfs             - /sys/class/accel/accel\*/
- debugfs           - /sys/kernel/debug/accel/\*/

## 鍏ラ棬


棣栧厛锛岄槄璇?Documentation/gpu/index.rst 涓殑 DRM 鏂囨。銆傚畠涓嶄粎浼氳鏄庡浣曠紪鍐?涓€涓柊鐨?DRM 椹卞姩锛岃繕浼氬寘鍚叧浜庡浣曡础鐚€佽涓哄噯鍒欙紙Code Of Conduct锛変互鍙?缂栫爜椋庢牸/鏂囨。鐨勫叏閮ㄤ俊鎭€傛墍鏈夎繖浜涘 accel 瀛愮郴缁熷悓鏍烽€傜敤銆?
鍏舵锛岀‘淇濆唴鏍搁厤缃簡 CONFIG_DRM_ACCEL銆?
瑕佸皢浣犵殑璁惧浣滀负鍔犻€熷櫒鏆撮湶锛岄渶瑕佸湪椹卞姩涓紙鐩稿浜庢爣鍑?DRM 椹卞姩锛夊仛涓ゅ淇敼锛?
- 鍦ㄤ綘鐨?drm_driver 鐨?driver_features 瀛楁涓坊鍔?DRIVER_COMPUTE_ACCEL
  鐗规€ф爣蹇椼€傞渶瑕佹敞鎰忥紝璇ラ┍鍔ㄧ壒鎬т笌 DRIVER_RENDER 鍜?DRIVER_MODESET 浜掓枼銆?  甯屾湜鍚屾椂鏆撮湶鍥惧舰鍜岃绠楃殑瀛楃璁惧鏂囦欢鐨勮澶囷紝搴旂敱閫氳繃 auxiliary bus
  妗嗘灦杩炴帴鐨勪袱涓┍鍔ㄦ潵澶勭悊銆?
- 灏嗛┍鍔?fops 缁撴瀯涓殑 open 鍥炶皟鏀逛负 accel_open()銆傛垨鑰咃紝浣犵殑椹卞姩鍙互浣跨敤
  DEFINE_DRM_ACCEL_FOPS 瀹忔潵杞绘澗璁剧疆姝ｇ‘鐨勫嚱鏁版搷浣滄寚閽堢粨鏋勩€?
## 澶栭儴鍙傝€?

### 閭欢鍒楄〃璁ㄨ


- `Initial discussion on the New subsystem for acceleration devices <https://lore.kernel.org/lkml/CAFCwf11=9qpNAepL7NL+YAV_QO=Wv6pnWPhKHKAepK3fNn+2Dg@mail.gmail.com/>`_ - Oded Gabbay (2022)
- `patch-set to add the new subsystem <https://lore.kernel.org/lkml/20221022214622.18042-1-ogabbay@kernel.org/>`_ - Oded Gabbay (2022)

### 浼氳婕旇


- `LPC 2022 Accelerators BOF outcomes summary <https://airlied.blogspot.com/2022/09/accelerators-bof-outcomes-summary.html>`_ - Dave Airlie (2022)
