## 瀵勫瓨鍣ㄦ枃浠舵暟鎹噰鏍凤紙Register File Data Sampling, RFDS锛?

瀵勫瓨鍣ㄦ枃浠舵暟鎹噰鏍凤紙RFDS锛夋槸涓€绉嶅井鏋舵瀯婕忔礊锛屼粎褰卞搷 Intel Atom 閮ㄤ欢锛堜篃绉颁负 E-core锛夈€俁FDS
鍙兘鍏佽鎭舵剰琛屼负鑰呮帹鏂嚭姝ゅ墠鐢ㄤ簬娴偣瀵勫瓨鍣ㄣ€佸悜閲忓瘎瀛樺櫒鎴栨暣鏁板瘎瀛樺櫒涓殑鏁版嵁鍊笺€俁FDS 骞朵笉鑳?閫夋嫨鎺ㄦ柇鍑哄摢浜涙暟鎹€俁FDS 琚垎閰嶇殑缂栧彿鏄?CVE-2023-28746銆?
## 鍙楀奖鍝嶇殑澶勭悊鍣?

浠ヤ笅鏄彈褰卞搷鐨?Intel 澶勭悊鍣ㄥ垪琛?[#f1]_锛?
   ===================  ============
   閫氱敤鍚嶇О               Family_Model
   ===================  ============
   ATOM_GOLDMONT           06_5CH
   ATOM_GOLDMONT_D         06_5FH
   ATOM_GOLDMONT_PLUS      06_7AH
   ATOM_TREMONT_D          06_86H
   ATOM_TREMONT            06_96H
   ALDERLAKE               06_97H
   ALDERLAKE_L             06_9AH
   ATOM_TREMONT_L          06_9CH
   RAPTORLAKE              06_B7H
   RAPTORLAKE_P            06_BAH
   ATOM_GRACEMONT          06_BEH
   RAPTORLAKE_S            06_BFH
   ===================  ============

## 缂撹В鎺柦


Intel 鍙戝竷浜嗕竴涓井鐮佹洿鏂帮紝浣胯蒋浠惰兘澶熶娇鐢?VERW 鎸囦护娓呴櫎鏁忔劅淇℃伅銆備笌 MDS 绫讳技锛孯FDS 閲囩敤鐩稿悓
鐨勭紦瑙ｇ瓥鐣ワ紝寮哄埗 CPU 鍦ㄦ敾鍑昏€呮彁鍙栫瀵嗕箣鍓嶆竻闄ゅ彈褰卞搷鐨勭紦鍐插尯銆傝繖鏄€氳繃灏嗗師鏈湭浣跨敤涓斿凡搴熷純鐨?VERW 鎸囦护涓庡井鐮佹洿鏂扮浉缁撳悎鏉ュ疄鐜扮殑銆傚綋鎵ц VERW 鎸囦护鏃讹紝寰爜浼氭竻闄ゅ彈褰卞搷鐨?CPU 缂撳啿鍖恒€?
### 缂撹В鐐?

VERW 鐢卞唴鏍稿湪杩斿洖鐢ㄦ埛绌洪棿涔嬪墠銆佷互鍙婄敱 KVM 鍦ㄨ繘鍏ヨ櫄鎷熸満锛圴Mentry锛変箣鍓嶆墽琛屻€傚彈褰卞搷鐨勬牳鍧囦笉
鏀寔 SMT锛屽洜姝ゆ棤闇€鍦?C-state 杞崲鏃舵墽琛?VERW銆?
### IA32_ARCH_CAPABILITIES 涓殑鏂颁綅


杈冩柊鐨勫鐞嗗櫒锛屼互鍙婂鐜版湁鍙楀奖鍝嶅鐞嗗櫒鐨勫井鐮佹洿鏂帮紝鍚?IA32_ARCH_CAPABILITIES MSR 澧炲姞浜嗘柊鐨勪綅銆?杩欎簺浣嶅彲鐢ㄤ簬鏋氫妇婕忔礊涓庣紦瑙ｈ兘鍔涳細

- 浣?27 - RFDS_NO - 缃綅鏃讹紝琛ㄧず澶勭悊鍣ㄤ笉鍙?RFDS 褰卞搷銆?- 浣?28 - RFDS_CLEAR - 缃綅鏃讹紝琛ㄧず澶勭悊鍣ㄥ彈 RFDS 褰卞搷锛屽苟涓旀嫢鏈夊湪鎵ц VERW 鏃舵竻闄ゅ彈褰卞搷
  缂撳啿鍖虹殑寰爜銆?
### 鍐呮牳鍛戒护琛屼笂鐨勭紦瑙ｆ帶鍒?

鍐呮牳鍛戒护琛屽厑璁稿湪鍚姩鏃堕€氳繃鍙傛暟 鈥渞eg_file_data_sampling=鈥?鎺у埗 RFDS 缂撹В銆傛湁鏁堢殑鍙傛暟涓猴細

  ==========  =================================================================
  on          鑻?CPU 瀛樺湪婕忔礊锛屽垯鍚敤缂撹В锛涘湪閫€鍑哄埌鐢ㄦ埛绌洪棿浠ュ強杩涘叆 VM 涔嬪墠娓呴櫎
              CPU 缂撳啿鍖恒€?  off         绂佺敤缂撹В銆?  ==========  =================================================================

缂撹В榛樿鐢?CONFIG_MITIGATION_RFDS 閫夋嫨銆?
### 缂撹В鐘舵€佷俊鎭?

Linux 鍐呮牳鎻愪緵浜嗕竴涓?sysfs 鎺ュ彛锛岀敤浜庢灇涓剧郴缁熷綋鍓嶇殑婕忔礊鐘舵€侊細绯荤粺鏄惁鏄撳彈鏀诲嚮锛屼互鍙婂摢浜?缂撹В鎺柦澶勪簬娲诲姩鐘舵€併€傜浉鍏崇殑 sysfs 鏂囦欢涓猴細

	/sys/devices/system/cpu/vulnerabilities/reg_file_data_sampling

璇ユ枃浠朵腑鍙兘鐨勫€间负锛?
```

     * - 'Not affected'
       - 澶勭悊鍣ㄤ笉鍙楀奖鍝?     * - 'Vulnerable'
       - 澶勭悊鍣ㄦ槗鍙楁敾鍑伙紝浣嗘湭鍚敤浠讳綍缂撹В
     * - 'Vulnerable: No microcode'
       - 澶勭悊鍣ㄦ槗鍙楁敾鍑伙紝浣嗘湭鏇存柊寰爜銆?     * - 'Mitigation: Clear Register File'
       - 澶勭悊鍣ㄦ槗鍙楁敾鍑伙紝涓斿凡鍚敤 CPU 缂撳啿鍖烘竻闄ょ紦瑙ｃ€?
```

### 鍙傝€?

   https://www.intel.com/content/www/us/en/developer/topic-technology/software-security-guidance/processors-affected-consolidated-product-cpu-model.html
