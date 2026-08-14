
## TPM FIFO 鎺ュ彛椹卞姩


TCG PTP 瑙勮寖瀹氫箟浜嗕袱绉嶆帴鍙ｇ被鍨嬶細FIFO 鍜?CRB銆傚墠鑰呭熀浜庢湁搴忕殑璇诲啓鎿嶄綔锛屽悗鑰呭熀浜庡寘鍚竴涓畬鏁村懡浠ゆ垨鍝嶅簲鐨勭紦鍐插尯銆?
FIFO锛團irst-In-First-Out锛屽厛杩涘厛鍑猴級鎺ュ彛琚緷璧栦簬 tpm_tis_core 鐨勯┍鍔ㄦ墍浣跨敤銆傛渶鍒?Linux 鍙湁涓€涓悕涓?tpm_tis 鐨勯┍鍔紝瀹冭鐩栧唴瀛樻槧灏勶紙鍗?MMIO锛夋帴鍙ｏ紝浣嗗悗鏉ヨ鎵╁睍涓鸿鐩?TCG 鏍囧噯鏀寔鐨勫叾浠栫墿鐞嗘帴鍙ｃ€?
鐢变簬涓婅堪鍘嗗彶鍘熷洜锛屾渶鍒濈殑 MMIO 椹卞姩琚О涓?tpm_tis锛岃€?FIFO 椹卞姩鐨勬鏋惰鍛藉悕涓?tpm_tis_core銆倀pm_tis 涓殑鍚庣紑"tis"鏉ヨ嚜 TPM Interface Specification锛圱PM 鎺ュ彛瑙勮寖锛夛紝鍗?TPM 1.x 鑺墖鐨勭‖浠舵帴鍙ｈ鑼冦€?
閫氫俊鍩轰簬涓€鍧楃敱 TPM 鑺墖閫氳繃纭欢鎬荤嚎鎴栧唴瀛樻槧灏勫叡浜殑 20 KiB 缂撳啿鍖猴紙鍙栧喅浜庣墿鐞嗘帴绾挎柟寮忥級銆傝缂撳啿鍖鸿繘涓€姝ヨ鍒掑垎涓轰簲涓瓑澶у皬鐨?4 KiB 缂撳啿鍖猴紝瀹冧滑鎻愪緵绛変环鐨勫瘎瀛樺櫒闆嗗悎锛岀敤浜?CPU 涓?TPM 涔嬮棿鐨勯€氫俊銆傝繖浜涢€氫俊绔偣琚?TCG 鏈绉颁负 localities锛堝眬閮ㄥ煙锛夈€?
褰撳唴鏍告兂瑕佸悜 TPM 鑺墖鍙戦€佸懡浠ゆ椂锛屽畠棣栧厛閫氳繃璁剧疆 TPM_ACCESS 瀵勫瓨鍣ㄤ腑鐨?requestUse 浣嶆潵淇濈暀 locality 0銆傚綋璁块棶琚巿浜堟椂锛岃浣嶇敱鑺墖娓呴櫎銆備竴鏃﹀畬鎴愰€氫俊锛屽唴鏍稿啓鍏?TPM_ACCESS.activeLocality 浣嶃€傝繖閫氱煡鑺墖璇?locality 宸茶閲婃斁銆?
寰呭鐞嗙殑 localities 鐢辫姱鐗囨寜浼樺厛绾т粠楂樺埌浣庝緷娆″鐞嗭紝涓€娆′竴涓細

- Locality 0 浼樺厛绾ф渶浣庛€?- Locality 5 浼樺厛绾ф渶楂樸€?
鍏充簬 localities 鐨勭洰鐨勫拰鍚箟鐨勮繘涓€姝ヤ俊鎭紝鍙湪 TCG PC Client Platform TPM Profile 瑙勮寖鐨?3.2 鑺備腑鎵惧埌銆?
## 鍙傝€冭祫鏂?

TCG PC Client Platform TPM Profile (PTP) Specification
https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification/
