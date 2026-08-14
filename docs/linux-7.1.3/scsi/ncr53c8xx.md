锘?
## NCR53C8XX/SYM53C8XX 椹卞姩


浣滆€咃細Gerard Roudier <groudier@free.fr>

21 Rue Carnot

95170 DEUIL LA BARRE - 娉曞浗

1999 骞?5 鏈?29 鏃?


   1. 绠€浠?
   2. 鏀寔鐨勮姱鐗囦笌 SCSI 鐗规€?
   3. 澧炲己鍨?896 椹卞姩鐨勪紭鍔?
         3.1 浼樺寲鍚庣殑 SCSI SCRIPTS
         3.2 SYM53C896 鐨勬柊鐗规€э紙64 浣?PCI 鍙岄€氶亾 LVD SCSI 鎺у埗鍣級
   4. 鍐呭瓨鏄犲皠 I/O 涓庢櫘閫?I/O
   5. 鏍囪鍛戒护闃熷垪
   6. 濂囧伓鏍￠獙
   7. 鎬ц兘鍓栨瀽淇℃伅
   8. 鎺у埗鍛戒护
         8.1  璁剧疆鏈€灏忓悓姝ュ懆鏈熷洜瀛?
         8.2  璁剧疆鎬荤嚎瀹藉害
         8.3  璁剧疆骞跺彂鏍囪鍛戒护鐨勬渶澶ф暟閲?
         8.4  璁剧疆鏍囪鍛戒护鐨勬帓搴忕被鍨?
         8.5  璁剧疆璋冭瘯妯″紡
         8.6  娓呴櫎鎬ц兘鍓栨瀽璁℃暟鍣?
         8.7  璁剧疆鏍囧織浣嶏紙no_disc锛?
         8.8  璁剧疆璇︾粏杈撳嚭绾у埆
         8.9  澶嶄綅鐩爣鐨勫叏閮ㄩ€昏緫鍗曞厓
         8.10 涓鐩爣鎵€鏈夐€昏緫鍗曞厓鐨勫叏閮ㄤ换鍔?
   9. 閰嶇疆鍙傛暟
   10. 鍚姩璁剧疆鍛戒护
         10.1 璇硶
         10.2 鍙敤鍙傛暟
                10.2.1  涓昏澶囧鍋舵牎楠?
                10.2.2  SCSI 濂囧伓鏍￠獙
                10.2.3  SCSI 鏂紑杩炴帴
                10.2.4  鐗规畩鐗规€?
                10.2.5  Ultra SCSI 鏀寔
                10.2.6  榛樿鏍囪鍛戒护鏁伴噺
                10.2.7  榛樿鍚屾鍛ㄦ湡鍥犲瓙
                10.2.8  涓庢墍鏈夎澶囧崗鍟嗗悓姝ヤ紶杈?
                10.2.9  璇︾粏杈撳嚭绾у埆
                10.2.10 璋冭瘯妯″紡
                10.2.11 绐佸彂鏈€澶ч暱搴?
                10.2.12 LED 鏀寔
                10.2.13 鏈€澶ф€荤嚎瀹藉害
                10.2.14 宸垎妯″紡
                10.2.15 涓柇璇锋眰妯″紡
                10.2.16 鍙嶅悜鎺㈡祴
                10.2.17 淇 PCI 閰嶇疆绌洪棿
                10.2.18 涓茶 NVRAM
                10.2.19 妫€鏌?SCSI 鎬荤嚎
                10.2.20 鎺掗櫎鏌愪釜涓绘満涓嶈鎸傝浇
                10.2.21 涓轰富鏈哄缓璁粯璁?SCSI ID
                10.2.22 鍚敤 IMMEDIATE ARBITRATION锛堢珛鍗充徊瑁侊級
         10.3 寤鸿鐨勫惎鍔ㄨ缃懡浠?
         10.4 PCI 閰嶇疆淇鍚姩閫夐」
         10.5 涓茶 NVRAM 鏀寔鍚姩閫夐」
         10.6 SCSI 鎬荤嚎妫€鏌ュ惎鍔ㄩ€夐」
         10.7 IMMEDIATE ARBITRATION 鍚姩閫夐」
   11. ncr53c8xx.h 澶存枃浠朵腑鐨勪竴浜涘父閲忎笌鏍囧織
   12. 瀹夎
   13. 涓庝綋绯荤粨鏋勭浉鍏崇殑鐗规€?
   14. 宸茬煡闂
         14.1 浣跨敤 Iomega Jaz 璁惧鐨勬爣璁板懡浠?
         14.2 娣诲姞鍙︿竴鎺у埗鍣ㄦ椂璁惧鍚嶅彂鐢熷彉鍖?
         14.3 鍦?WIDE SCSI 鎺у埗鍣ㄤ笂浠呬娇鐢?8 浣嶈澶?
         14.4 鍐呭瓨鍐欏苟浣挎棤鏁堟湡闂村彲鑳藉嚭鐜扮殑鏁版嵁鎹熷潖
   15. SCSI 闂鎺掓煡
         15.1 闂杩借釜
         15.2 鐞嗚В纭欢閿欒鎶ュ憡
   16. 鍚屾浼犺緭鍗忓晢琛?
         16.1 53C875 涓?53C860 Ultra-SCSI 鎺у埗鍣ㄧ殑鍚屾鏃跺簭
         16.2 蹇€?SCSI-2 53C8XX 鎺у埗鍣ㄧ殑鍚屾鏃跺簭
   17. 涓茶 NVRAM 鏀寔锛堜綔鑰?Richard Waltham锛?
         17.1 鐗规€?
         17.2 Symbios NVRAM 甯冨眬
         17.3 Tekram NVRAM 甯冨眬
   18. 澶х鏀寔
         18.1 澶х CPU
         18.2 杩愯浜庡ぇ绔ā寮忕殑 NCR 鑺墖


## 1. 绠€浠?


鏈€鍒濈殑 Linux ncr53c8xx 椹卞姩鏄?FreeBSD 涓?ncr 椹卞姩鐨勪竴涓Щ妞嶇増鏈紝鐢?
浠ヤ笅浜哄憳鍦?1995 骞?11 鏈堝畬鎴愶細

 - Gerard Roudier              <groudier@free.fr>

鏈€鍒濈殑椹卞姩鐢变互涓嬩汉鍛樹负 386bsd 鍜?FreeBSD 缂栧啓锛?

        - Wolfgang Stanglmeier        <wolf@cologne.de>
        - Stefan Esser                <se@mi.Uni-Koeln.de>

濡備粖瀹冧互涓や釜椹卞姩鎹嗙粦鐨勫舰寮忔彁渚涳細

- ncr53c8xx 閫氱敤椹卞姩锛屾敮鎸佹暣涓?SYM53C8XX 绯诲垪锛屽寘鎷渶鏃╃殑 810 rev. 1銆?
  鏈€鏂扮殑 896锛堝弻閫氶亾 LVD SCSI 鎺у埗鍣級浠ュ強鏂扮殑 895A锛堝崟閫氶亾 LVD SCSI 鎺у埗鍣級銆?
- sym53c8xx 澧炲己鍨嬮┍鍔紙鍙堢О 896 椹卞姩锛夛紝瀹冩斁寮冧簡瀵规渶鑰佽姱鐗囩殑鏀寔锛?
  浠ヨ幏寰楁柊鐗规€х殑浼樺娍锛屼緥濡傝嚜 810A 璧峰彲鐢ㄧ殑 LOAD/STORE 鎸囦护锛屼互鍙?
  896 鍜?895A 涓婂彲鐢ㄧ殑纭欢鐩镐綅澶遍厤澶勭悊銆?

鍏充簬 NCR 8xx 绯诲垪鐨勬妧鏈俊鎭紝鍙湪 Michael Will 缂栧啓鐨?PCI-HOWTO
浠ュ強 Drew Eckhardt 缂栧啓鐨?SCSI-HOWTO 涓壘鍒般€?

鍏充簬鏂拌姱鐗囩殑淇℃伅鍙湪 LSILOGIC 鐨?Web 鏈嶅姟鍣ㄤ笂鑾峰彇锛?

          - http://www.lsilogic.com/

SCSI 鏍囧噯鏂囨。鍙湪 SYMBIOS 鐨?ftp 鏈嶅姟鍣ㄤ笂鑾峰彇锛?

          - ftp://ftp.symbios.com/

Eric Youngdale 缂栧啓鐨勪竴浜涙湁鐢ㄧ殑 SCSI 宸ュ叿鍙湪 tsx-11 鑾峰彇锛?

          - ftp://tsx-11.mit.edu/pub/linux/ALPHA/scsi/scsiinfo-X.Y.tar.gz
          - ftp://tsx-11.mit.edu/pub/linux/ALPHA/scsi/scsidev-X.Y.tar.gz

杩欎簺宸ュ叿骞堕潪 ALPHA 鐗堟湰锛岃€屾槸鐩稿綋骞插噣骞朵笖宸ヤ綔鑹ソ鐨勩€傛嫢鏈?
'scsiinfo' 杞欢鍖呮槸蹇呬笉鍙皯鐨勩€?

杩欎唤绠€鐭殑鏂囨。鎻忚堪浜嗛€氱敤椹卞姩涓庡寮哄瀷椹卞姩鐨勭壒鎬с€侀厤缃弬鏁帮紝浠ュ強
閫氳繃 proc SCSI 鏂囦欢绯荤粺鐨勮/鍐欐搷浣滃彲鐢ㄧ殑鎺у埗鍛戒护銆?

璇ラ┍鍔ㄥ凡鍦?linux/i386銆丩inux/Alpha 鍜?Linux/PPC 涓婃祴璇曢€氳繃銆?

鏈€鏂扮殑椹卞姩鐗堟湰涓庤ˉ涓佸彲鍦ㄤ互涓嬩綅缃幏鍙栵細

          - ftp://ftp.tux.org/pub/people/gerard-roudier

鎴?

          - ftp://ftp.symbios.com/mirror/ftp.tux.org/pub/tux/roudier/drivers

鎴戝苟闈炶嫳璇瘝璇€咃紝杩欎唤 README 鏂囦欢涓ぇ姒傛湁涓嶅皯閿欒銆傛杩庝换浣曞府鍔┿€?


## 2. 鏀寔鐨勮姱鐗囦笌 SCSI 鐗规€?


浠ヤ笅鐗规€у鎵€鏈夎姱鐗囬兘鎻愪緵鏀寔锛?

 - 鍚屾鍗忓晢
 - 鏂紑杩炴帴
 - 鏍囪鍛戒护闃熷垪
 - SCSI 濂囧伓鏍￠獙
 - 涓昏澶囧鍋舵牎楠?

"瀹藉崗鍟? 瀵规敮鎸佸畠鐨勮姱鐗囨彁渚涖€備笅闈㈢殑琛ㄦ牸灞曠ず浜?NCR 8xx 绯诲垪閮ㄥ垎鑺墖
鐨勭壒鎬э紝浠ュ強鍝簺椹卞姩鏀寔瀹冧滑銆?

+--------+-----------+-----+-----------+------------+------------+------------+
|        |           |     |           |            |Supported by|Supported by|
|        |On board   |     |           |            |the generic |the enhanced|
|Chip    |SDMS BIOS  |Wide |SCSI std.  | Max. sync  |driver      |driver      |
+--------+-----------+-----+-----------+------------+------------+------------+
|810     |  N        | N   |  FAST10   | 10 MB/s    |    Y       |    N       |
+--------+-----------+-----+-----------+------------+------------+------------+
|810A    |  N        | N   |  FAST10   | 10 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|815     |  Y        | N   |  FAST10   | 10 MB/s    |    Y       |    N       |
+--------+-----------+-----+-----------+------------+------------+------------+
|825     |  Y        | Y   |  FAST10   | 20 MB/s    |    Y       |    N       |
+--------+-----------+-----+-----------+------------+------------+------------+
|825A    |  Y        | Y   |  FAST10   | 20 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|860     |  N        | N   |  FAST20   | 20 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|875     |  Y        | Y   |  FAST20   | 40 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|876     |  Y        | Y   |  FAST20   | 40 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|895     |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|895A    |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|896     |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|897     |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|1510D   |  Y        | Y   |  FAST40   | 80 MB/s    |    Y       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|1010    |  Y        | Y   |  FAST80   |160 MB/s    |    N       |    Y       |
+--------+-----------+-----+-----------+------------+------------+------------+
|1010_66 |  Y        | Y   |  FAST80   |160 MB/s    |    N       |    Y       |
|[^1^]_    |           |     |           |            |            |            |
+--------+-----------+-----+-----------+------------+------------+------------+



鍏朵粬鍙楁敮鎸佺壒鎬ф憳瑕侊細

:Module:                鍏佽鍔犺浇璇ラ┍鍔?
:Memory mapped I/O:     鎻愬崌鎬ц兘
:Profiling information: 鏉ヨ嚜 proc SCSI 鏂囦欢绯荤粺鐨勮鎿嶄綔
:Control commands:      瀵?proc SCSI 鏂囦欢绯荤粺鐨勫啓鎿嶄綔
:Debugging information: 鍐欏叆 syslog锛堜粎渚涗笓瀹讹級
:Serial NVRAM:          Symbios 涓?Tekram 鏍煎紡

- 鍒嗘暎 / 鑱氶泦锛圫catter / gather锛?
- 鍏变韩涓柇
- 鍚姩璁剧疆鍛戒护


## 3. 澧炲己鍨?896 椹卞姩鐨勪紭鍔?


### 3.1 浼樺寲鍚庣殑 SCSI SCRIPTS


810A銆?25A銆?75銆?95銆?96 鍜?895A 鏀寔鍚嶄负 LOAD 鍜?STORE 鐨勬柊 SCSI
SCRIPTS 鎸囦护锛屽畠浠兘澶熸瘮 53c7xx 涓?53c8xx 绯诲垪鎵€鏀寔鐨?MOVE MEMORY
鎸囦护鏇村揩鍦板湪 IO 瀵勫瓨鍣ㄤ笌鍐呭瓨涔嬮棿绉诲姩鏈€澶?1 涓?DWORD銆?
LOAD/STORE 鎸囦护鏀寔缁濆瀵诲潃涓?DSA 鐩稿瀵诲潃妯″紡銆係CSI SCRIPTS 宸插畬鍏?
浣跨敤 LOAD/STORE 閲嶅啓浜嗭紝鍙栦唬浜?MOVE MEMORY 鎸囦护銆?

### 3.2 SYM53C896 鐨勬柊鐗规€э紙64 浣?PCI 鍙岄€氶亾 LVD SCSI 鎺у埗鍣級


896 涓?895A 鍏佽浠?SCRIPTS 澶勭悊鐩镐綅澶遍厤涓婁笅鏂囷紙閬垮厤浜嗙浉浣嶅け閰嶄腑鏂紝
璇ヤ腑鏂細鏆傚仠 SCSI 澶勭悊鍣紝鐩村埌 C 浠ｇ爜淇濆瓨浜嗕紶杈撶殑涓婁笅鏂囷級銆?
濡傛灉涓嶄娇鐢?LOAD/STORE 鎸囦护鏉ュ疄鐜拌繖涓€鐐瑰皢鍗佸垎鐥涜嫤锛屾垜鐢氳嚦閮戒笉鎯冲皾璇曘€?

896 鑺墖鏀寔 64 浣?PCI 浜嬪姟涓庡鍧€锛岃€?895A 鏀寔 32 浣?PCI 浜嬪姟涓?
64 浣嶅鍧€銆傝繖浜涜姱鐗囩殑 SCRIPTS 澶勭悊鍣ㄥ苟闈炵湡姝ｇ殑 64 浣嶏紝鑰屾槸浣跨敤
娈靛瘎瀛樺櫒鏉ュ鐞嗙 32-63 浣嶃€傚彟涓€涓湁瓒ｇ殑鐗规€ф槸锛屽鍧€鐗囦笂 RAM锛?k锛夌殑
LOAD/STORE 鎸囦护淇濇寔鍦ㄨ姱鐗囧唴閮ㄣ€?

鐢变簬浣跨敤浜?LOAD/STORE SCRIPTS 鎸囦护锛岃椹卞姩涓嶅啀鏀寔浠ヤ笅鑺墖锛?

- SYM53C810 revision < 0x10 (16)
- SYM53C815 鎵€鏈夌増鏈?
- SYM53C825 revision < 0x10 (16)

## 4. 鍐呭瓨鏄犲皠 I/O 涓庢櫘閫?I/O


鍐呭瓨鏄犲皠 I/O 姣旀櫘閫?I/O 鍏锋湁鏇翠綆鐨勫欢杩熴€傝嚜 linux-1.3.x 璧凤紝浣跨敤鍐呭瓨鏄犲皠
I/O 鑰岄潪鏅€?I/O銆傚唴瀛樻槧灏?I/O 鍦ㄥぇ澶氭暟纭欢閰嶇疆涓婁技涔庡伐浣滆壇濂斤紝浣?
涓€浜涜璁′笉浣崇殑涓绘澘鍙兘浼氱牬鍧忚繖涓€鐗规€с€?

閰嶇疆閫夐」 CONFIG_SCSI_NCR53C8XX_IOMAPPED 寮哄埗椹卞姩鍦ㄦ墍鏈夋儏鍐典笅
閮戒娇鐢ㄦ櫘閫?I/O銆?


## 5. 鏍囪鍛戒护闃熷垪


鍚戜竴涓澶囦竴娆℃帓闃熷浜?1 鏉″懡浠わ紝鍙互璁╁畠鍩轰簬瀹為檯鐨勭澶翠綅缃強鍏舵満姊?
鐗规€ц繘琛屼紭鍖栥€傝繖涓€鐗规€ц繕鍙互闄嶄綆骞冲潎鍛戒护寤惰繜銆備负浜嗙湡姝ｈ幏寰楄鐗规€х殑
浼樺娍锛岃澶囧繀椤绘嫢鏈夊悎鐞嗙殑缂撳瓨澶у皬锛堝浜?128 KB 鎴栨洿灏忕殑浣庣纭洏锛?
涓嶈鎸囨湜浠€涔堝杩癸級銆?
涓€浜涘凡鐭ョ殑 SCSI 璁惧涓嶈兘姝ｇ‘鏀寔鏍囪鍛戒护闃熷垪銆傞€氬父锛屼慨澶嶆绫婚棶棰樼殑
鍥轰欢淇鐗堝彲鍦ㄧ浉搴斿巶鍟嗙殑 Web/ftp 绔欑偣鑾峰彇銆?
鎴戞墍鑳借鐨勬槸锛屾垜鏈哄櫒涓婁娇鐢ㄧ殑纭洏鍦ㄨ椹卞姩鍚敤鏍囪鍛戒护闃熷垪鏃惰〃鐜拌壇濂斤細

- IBM S12 0662
- Conner 1080S
- Quantum Atlas I
- Quantum Atlas II

濡傛灉浣犵殑鎺у埗鍣ㄥ甫鏈?NVRAM锛屼綘鍙互閫氳繃鐢ㄦ埛璁剧疆宸ュ叿鎸夌洰鏍囬厤缃繖涓€鐗规€с€?
Tekram 璁剧疆绋嬪簭鍏佽灏嗘帓闃熷懡浠ょ殑鏈€澶ф暟閲忚皟鏁村埌 32銆係ymbios 璁剧疆绋嬪簭
鍙厑璁稿惎鐢ㄦ垨绂佺敤璇ョ壒鎬с€?

鎺掗槦鍒版煇涓澶囩殑鍚屾椂鏍囪鍛戒护鐨勬渶澶ф暟閲忕洰鍓嶉粯璁よ涓?8銆傝繖涓€奸€傜敤浜?
澶у鏁?SCSI 纭洏銆傚浜庡ぇ鍨?SCSI 纭洏锛?= 2GB锛岀紦瀛?>= 512KB锛屽钩鍧?
瀵婚亾鏃堕棿 <= 10 ms锛夛紝浣跨敤鏇村ぇ鐨勫€煎彲鑳戒細鑾峰緱鏇村ソ鐨勬€ц兘銆?

sym53c8xx 椹卞姩姣忎釜璁惧鏈€澶氭敮鎸?255 鏉″懡浠わ紝閫氱敤 ncr53c8xx 椹卞姩鏈€澶氭敮鎸?
64 鏉★紝浣嗕娇鐢ㄨ秴杩?32 鏉￠€氬父骞朵笉鍒掔畻锛岄櫎闈炰綘浣跨敤鐨勬槸闈炲父澶х殑纭洏鎴?
纾佺洏闃靛垪銆傚€煎緱娉ㄦ剰鐨勬槸锛屽ぇ澶氭暟杩戞湡鐨勭‖鐩樹技涔庝笉鎺ュ彈瓒呰繃 64 鏉″苟鍙戝懡浠ゃ€?
鍥犳锛屼娇鐢ㄨ秴杩?64 鏉℃帓闃熷懡浠ゅぇ姒傚彧鏄湪娴垂璧勬簮銆?

濡傛灉浣犵殑鎺у埗鍣ㄦ病鏈?NVRAM锛屾垨鑰呭畠鐢?SDMS BIOS/SETUP 绠＄悊锛屼綘鍙互閰嶇疆
鏍囪闃熷垪鐗规€т互鍙婅澶囬槦鍒?
```
ncr53c8xx=tags:4/t2t3q15-t4q7/t1u0q32
```
浼氬皢鏍囪鍛戒护鐨勯槦鍒楁繁搴﹁缃涓嬶細

- 鎺у埗鍣?0 涓婄殑鐩爣 2  鎵€鏈夐€昏緫鍗曞厓  --> 15
- 鎺у埗鍣?0 涓婄殑鐩爣 3  鎵€鏈夐€昏緫鍗曞厓  --> 15
- 鎺у埗鍣?0 涓婄殑鐩爣 4  鎵€鏈夐€昏緫鍗曞厓  -->  7
- 鎺у埗鍣?1 涓婄殑鐩爣 1  閫昏緫鍗曞厓 0     --> 32
- 鎵€鏈夊叾浠栫洰鏍?閫昏緫鍗曞厓             -->  4

鍦ㄦ煇浜涚壒娈婃潯浠朵笅锛屾煇浜?SCSI 纭洏鍥轰欢鍙兘浼氶拡瀵逛竴鏉?SCSI 鍛戒护杩斿洖
QUEUE FULL锛堥槦鍒楀凡婊★級鐘舵€併€傞┍鍔ㄤ娇鐢ㄤ互涓嬪惎鍙戝紡鏂规硶澶勭悊杩欑琛屼负锛?

- 姣忔杩斿洖 QUEUE FULL 鐘舵€佹椂锛屾爣璁伴槦鍒楁繁搴︿細琚檷浣庡埌褰撳墠宸叉柇寮€杩炴帴
  鍛戒护鐨勫疄闄呮暟閲忋€?

- 姣忔垚鍔熷畬鎴?1000 鏉?SCSI 鍛戒护锛岃嫢褰撳墠涓婇檺鍏佽锛屽彲鎺掗槦鐨勫懡浠ゆ渶澶ф暟閲?
  浼氶€掑銆?

鐢变簬鎺ユ敹鍜屽鐞?QUEUE FULL 鐘舵€佷細娴垂璧勬簮锛岄粯璁ゆ儏鍐典笅椹卞姩浼氶€氳繃
鎸囩ず瀹為檯浣跨敤鐨勫懡浠ゆ暟閲忓強鍏剁姸鎬侊紝浠ュ強瀹冨璁惧闃熷垪娣卞害鍙樺寲鐨勫喅瀹氾紝
灏嗘闂閫氱煡鐢ㄦ埛銆?
椹卞姩澶勭悊 QUEUE FULL 鎵€浣跨敤鐨勫惎鍙戝紡鏂规硶纭繚浜嗘€ц兘鎵€鍙楀奖鍝嶄笉浼氬お绯熴€備綘
鍙互閫氳繃浠ヤ笅鏂瑰紡灏嗘秷鎭叧鎺夛細灏嗚缁嗚緭鍑虹骇鍒涓?0锛屽涓嬫墍绀猴細

绗竴绉嶆柟娉曪細
	    浣跨敤 'ncr53c8xx=verb:0' 閫夐」鍚姩浣犵殑绯荤粺銆?

绗簩绉嶆柟娉曪細
	    鍦ㄥ惎鍔ㄥ悗锛屽涓庝綘鐨勬帶鍒跺櫒瀵瑰簲鐨?proc 鏂囦欢绯荤粺鏉＄洰搴旂敤
            "setverbose 0" 鎺у埗鍛戒护銆?

## 6. 濂囧伓鏍￠獙


璇ラ┍鍔ㄦ敮鎸?SCSI 濂囧伓鏍￠獙涓?PCI 鎬荤嚎涓昏澶囧鍋舵牎楠屻€備负浜嗙‘淇濇暟鎹紶杈?
瀹夊叏锛屽繀椤诲惎鐢ㄨ繖浜涚壒鎬с€傜劧鑰岋紝鏌愪簺鏈夌己闄风殑璁惧鎴栦富鏉夸細鍦ㄥ鍋舵牎楠屼笂
閬囧埌闂銆備綘鍙互閫氳繃鍦ㄥ惎鍔ㄥ懡浠よ涓緭鍏ラ€傚綋鐨勯€夐」鏉ョ鐢?PCI 濂囧伓鏍￠獙
鎴?SCSI 濂囧伓鏍￠獙銆傦紙鍙傝 10锛氬惎鍔ㄨ缃懡浠わ級銆?

## 7. 鎬ц兘鍓栨瀽淇℃伅


鎬ц兘鍓栨瀽淇℃伅鍙€氳繃 proc SCSI 鏂囦欢绯荤粺鑾峰彇銆傜敱浜庢敹闆嗘€ц兘鍓栨瀽淇℃伅鍙兘
褰卞搷鎬ц兘锛岃鐗规€ч粯璁ゆ槸绂佺敤鐨勶紝骞朵笖闇€瑕佸皢璇ョ紪璇戦厤缃€夐」璁句负 Y銆?

```

          /proc/scsi/ncr53c8xx/N     (N=0,1,2 ....)

```
```

          /proc/scsi/ncr53c8xx/0

```
鐒惰€岋紝濡傛灉椹卞姩琚紪璇戜负妯″潡锛屼富鏈虹殑缂栧彿浼氬湪姣忔鍔犺浇椹卞姩鏃堕€掑銆?

```

         cat /proc/scsi/ncr53c8xx/0

```
```

    General information:
    Chip NCR53C810, device id 0x1, revision id 0x2
    IO port address 0x6000, IRQ number 10
    Using memory mapped IO at virtual address 0x282c000
    Synchronous transfer period 25, max commands per lun 4
    Profiling information:
    num_trans    = 18014
    num_kbytes   = 671314
    num_disc     = 25763
    num_break    = 1673
    num_int      = 1685
    num_fly      = 18038
    ms_setup     = 4940
    ms_data      = 369940
    ms_disc      = 183090
    ms_post      = 1320

```
甯歌淇℃伅寰堝ソ鐞嗚В銆傝澶?ID 涓庝慨璁?ID 瀵?SCSI 鑺墖鐨勬爣璇嗗涓嬶細

======= ============= ===========
Chip    Device id     Revision Id
======= ============= ===========
810       0x1            <  0x10
810A      0x1            >= 0x10
815       0x4
825       0x3            <  0x10
860       0x6
825A      0x3            >= 0x10
875       0xf
895       0xc
======= ============= ===========

鎬ц兘鍓栨瀽淇℃伅鍦?SCSI 鍛戒护瀹屾垚鏃舵洿鏂般€傚綋涓绘満閫傞厤鍣ㄨ鎸傝浇鏃朵細鍒嗛厤骞?
娓呴浂涓€涓暟鎹粨鏋勩€傚洜姝わ紝濡傛灉椹卞姩鏄ā鍧楋紝鍒欐瘡娆″姞杞介┍鍔ㄦ椂鎬ц兘鍓栨瀽
璁℃暟鍣ㄩ兘浼氳娓呴浂銆?clearprof" 鍛戒护鍏佽浣犻殢鏃舵竻闄よ繖浜涜鏁板櫒銆?

鍙敤鐨勮鏁板櫒濡備笅锛?

锛?num" 鍓嶇紑琛ㄧず"鏁伴噺"锛?
"ms" 琛ㄧず姣锛?

num_trans
	宸插畬鎴愮殑鍛戒护鏁伴噺
	浠ヤ笂绀轰緥锛?8014 鏉″凡瀹屾垚鐨勫懡浠?

num_kbytes
	宸蹭紶杈撶殑鍗冨瓧鑺傛暟
	浠ヤ笂绀轰緥锛氬凡浼犺緭 671 MB

num_disc
	SCSI 鏂紑杩炴帴娆℃暟
	浠ヤ笂绀轰緥锛?5763 娆?SCSI 鏂紑杩炴帴

num_break
	鑴氭湰涓柇娆℃暟锛堢浉浣嶅け閰嶏級
	浠ヤ笂绀轰緥锛?673 娆¤剼鏈腑鏂?

num_int
	闈?杩愯涓?锛坥n the fly锛夌殑涓柇娆℃暟
	浠ヤ笂绀轰緥锛?685 娆￠潪"杩愯涓?鐨勪腑鏂?

num_fly
	"杩愯涓?鐨勪腑鏂鏁?
	浠ヤ笂绀轰緥锛?8038 娆?杩愯涓?鐨勪腑鏂?

ms_setup
	SCSI 鍛戒护寤虹珛鐨勮€楁椂
	浠ヤ笂绀轰緥锛?.94 绉?

ms_data
	鏁版嵁浼犺緭鐨勮€楁椂
	浠ヤ笂绀轰緥锛氭暟鎹紶杈撹€楁椂 369.94 绉?

ms_disc
	SCSI 鏂紑杩炴帴鐨勮€楁椂
	浠ヤ笂绀轰緥锛氭柇寮€杩炴帴鑰楁椂 183.09 绉?

ms_post
	鍛戒护鍚庡鐞嗙殑鑰楁椂
	锛堜粠鑾峰彇 SCSI 鐘舵€佸埌璋冪敤鍛戒护瀹屾垚鐨勬椂闂达級
	浠ヤ笂绀轰緥锛氬悗澶勭悊鑰楁椂 1.32 绉?

鐢变簬绯荤粺鏃堕挓鐨?1/100 绉掕妭鎷嶏紝"ms_post" 鏃堕棿鍙兘鏄敊璇殑銆?

鍦ㄤ笂闈㈢殑绀轰緥涓紝鎴戜滑寰楀埌浜?18038 娆?杩愯涓?鐨勪腑鏂紝浠ュ強閫氬父鐢变簬
鍒嗘暎鍒楄〃鏌愪竴娈靛唴閮ㄧ殑鏂紑杩炴帴鎵€瀵艰嚧鐨?1673 娆¤剼鏈腑鏂€?


## 8. 鎺у埗鍛戒护


鎺у埗鍛戒护鍙互閫氳繃瀵?proc SCSI 鏂囦欢绯荤粺鎵ц鍐欐搷浣滃彂閫佺粰椹卞姩銆傞€氱敤
鍛戒护璇硶濡備笅锛?

```

      echo "<verb> <parameters>" >/proc/scsi/ncr53c8xx/0
      (assumes controller number is 0)

```
瀵逛互涓嬪懡浠や娇鐢?"all" 浣滀负 "<target>" 鍙傛暟灏嗗簲鐢ㄤ簬 SCSI 閾句笂鐨勬墍鏈?
鐩爣锛堟帶鍒跺櫒鏈韩闄ゅ锛夈€?

鍙敤鍛戒护锛?

### 8.1 璁剧疆鏈€灏忓悓姝ュ懆鏈熷洜瀛?


    setsync <target> <period factor>

    :target:   target number
    :period:   minimum synchronous period.
               Maximum speed = 1000/(4*period factor) except for special
               cases below.

    鎸囧畾鍛ㄦ湡鍊?255锛屼互寮哄埗杩涘叆寮傛浼犺緭妯″紡銆?

      - 10 琛ㄧず 25 绾崇鍚屾鍛ㄦ湡
      - 11 琛ㄧず 30 绾崇鍚屾鍛ㄦ湡
      - 12 琛ㄧず 50 绾崇鍚屾鍛ㄦ湡

### 8.2 璁剧疆鎬荤嚎瀹藉害


    setwide <target> <size>

    :target:   target number
    :size:     0=8 bits, 1=16bits

### 8.3 璁剧疆骞跺彂鏍囪鍛戒护鐨勬渶澶ф暟閲?


    settags <target> <tags>

    :target:   target number
    :tags:     number of concurrent tagged commands
               must not be greater than SCSI_NCR_MAX_TAGS (default: 8)

### 8.4 璁剧疆鏍囪鍛戒护鐨勬帓搴忕被鍨?


    setorder <order>

    :order:    3 possible values:

               simple:
			use SIMPLE TAG for all operations (read and write)

               ordered:
			use ORDERED TAG for all operations

               default:
			use default tag type,
                        SIMPLE  TAG for read  operations
                        ORDERED TAG for write operations


### 8.5 璁剧疆璋冭瘯妯″紡


    setdebug <list of debug flags>

    Available debug flags:

	======== ========================================================
        alloc    print info about memory allocations (ccb, lcb)
        queue    print info about insertions into the command start queue
        result   print sense data on CHECK CONDITION status
        scatter  print info about the scatter process
        scripts  print info about the script binding process
	tiny     print minimal debugging information
	timing   print timing information of the NCR chip
	nego     print information about SCSI negotiations
	phase    print information on script interruptions
	======== ========================================================

    浣跨敤涓嶅甫鍙傛暟鐨?"setdebug" 鏉ラ噸缃皟璇曟爣蹇椼€?


### 8.6 娓呴櫎鎬ц兘鍓栨瀽璁℃暟鍣?


    clearprof

    The profile counters are automatically cleared when the amount of
    data transferred reaches 1000 GB in order to avoid overflow.
    The "clearprof" command allows you to clear these counters at any time.


### 8.7 璁剧疆鏍囧織浣嶏紙no_disc锛?


    setflag <target> <flag>

    target:    target number

    For the moment, only one flag is available:

        no_disc:   not allow target to disconnect.

    Do not specify any flag in order to reset the flag. For example:

    setflag 4
      will reset no_disc flag for target 4, so will allow it disconnections.

    setflag all
      will allow disconnection for all devices on the SCSI bus.


### 8.8 璁剧疆璇︾粏杈撳嚭绾у埆


    setverbose #level

    The driver default verbose level is 1. This command allows to change
    th driver verbose level after boot-up.

### 8.9 澶嶄綅鐩爣鐨勫叏閮ㄩ€昏緫鍗曞厓


    resetdev <target>

    :target:   target number

    The driver will try to send a BUS DEVICE RESET message to the target.
    (Only supported by the SYM53C8XX driver and provided for test purpose)

### 8.10 涓鐩爣鎵€鏈夐€昏緫鍗曞厓鐨勫叏閮ㄤ换鍔?


    cleardev <target>

    :target:   target number

    The driver will try to send a ABORT message to all the logical units
    of the target.

    (Only supported by the SYM53C8XX driver and provided for test purpose)

## 9. 閰嶇疆鍙傛暟


濡傛灉鎵€鏈夎澶囩殑鍥轰欢閮借冻澶熷畬鍠勶紝椹卞姩鏀寔鐨勫叏閮ㄧ壒鎬ч兘鍙互鍦ㄥ惎鍔ㄦ椂鍚敤銆?
鐒惰€岋紝濡傛灉鍙湁涓€涓澶囧湪鏌愪釜 SCSI 鐗规€т笂鏈夌己闄凤紝浣犲彲浠ュ湪 Linux 鍚姩鏃?
绂佺敤椹卞姩瀵硅鐗规€х殑鏀寔锛屽苟鍦ㄥ惎鍔ㄥ悗浠呬负鑳藉畨鍏ㄦ敮鎸佽鐗规€х殑璁惧鍚敤瀹冦€?

CONFIG_SCSI_NCR53C8XX_IOMAPPED       (榛樿鍥炵瓟锛歯)
    濡傛灉浣犳€€鐤戜綘鐨勪富鏉夸笉鍏佽鍐呭瓨鏄犲皠 I/O锛屽洖绛?"y"銆?

    鍙兘浼氱◢寰檷浣庝竴鐐规€ц兘銆侺inux/PPC 闇€瑕佹閫夐」锛屾棤璁轰綘鍦ㄦ閫夋嫨浠€涔?
    閮戒細浣跨敤瀹冦€侺inux/PPC 浣跨敤姝ら€夐」涓嶄細鎹熷け鎬ц兘锛屽洜涓烘墍鏈?IO 閮芥槸
    鍐呭瓨鏄犲皠鐨勩€?

CONFIG_SCSI_NCR53C8XX_DEFAULT_TAGS    (榛樿鍥炵瓟锛?)
    榛樿鏍囪鍛戒护闃熷垪娣卞害銆?

CONFIG_SCSI_NCR53C8XX_MAX_TAGS         (榛樿鍥炵瓟锛?)
    姝ら€夐」鍏佽浣犳寚瀹氬彲鎺掗槦鍒版煇涓澶囩殑鏈€澶ф爣璁板懡浠ゆ暟閲忋€傛渶澶ф敮鎸佸€间负 32銆?

CONFIG_SCSI_NCR53C8XX_SYNC            (榛樿鍥炵瓟锛?)
    姝ら€夐」鍏佽浣犳寚瀹氶┍鍔ㄥ湪鍚姩鏃剁敤浜庡悓姝ユ暟鎹紶杈撳崗鍟嗙殑棰戠巼锛堝崟浣?MHz锛夈€?
    姝ら鐜囦箣鍚庡彲鐢?"setsync" 鎺у埗鍛戒护鏇存敼銆? 琛ㄧず"寮傛鏁版嵁浼犺緭"銆?

CONFIG_SCSI_NCR53C8XX_FORCE_SYNC_NEGO (榛樿鍥炵瓟锛歯)
    瀵规墍鏈?SCSI-2 璁惧寮哄埗杩涜鍚屾鍗忓晢銆?

    鏌愪簺 SCSI-2 璁惧涓嶅湪鏌ヨ鍝嶅簲鐨勭 7 瀛楄妭涓姤鍛婃鐗规€э紝浣嗗嵈鑳?
    姝ｇ‘鏀寔瀹冿紙渚嬪 TAMARACK 鎵弿浠級銆?

CONFIG_SCSI_NCR53C8XX_NO_DISCONNECT   (榛樿浠ュ強鍞竴鍚堢悊鐨勫洖绛旓細n)
    濡傛灉浣犳€€鐤戜綘鐨勬煇涓澶囦笉鑳芥纭敮鎸佹柇寮€杩炴帴锛屽彲浠ュ洖绛?"y"銆傝繖鏍凤紝
    鎵€鏈?SCSI 璁惧鍗充娇鍦ㄦ墽琛岄暱鏃堕棿鐨?SCSI 鎿嶄綔鏃朵篃姘歌繙涓嶄細鏂紑鎬荤嚎銆?

CONFIG_SCSI_NCR53C8XX_SYMBIOS_COMPAT
    姝ｅ搧 SYMBIOS 鏉垮崱浣跨敤 GPIO0 浣滀负杈撳嚭浠ラ┍鍔ㄦ帶鍒跺櫒 LED锛屽苟浣跨敤 GPIO3
    浣嶄綔涓哄崟绔?宸垎鎺ュ彛鐨勬爣绀烘爣蹇椼€?
    濡傛灉浣犵殑绯荤粺涓殑鎵€鏈夋澘鍗￠兘鏄鍝?SYMBIOS 鏉垮崱锛屾垨鑰呬娇鐢ㄦ潵鑷?SYMBIOS
    鐨?BIOS 鍜岄┍鍔紝浣犱細甯屾湜鍚敤姝ら€夐」銆?

    濡傛灉浣犵殑绯荤粺鑷冲皯鏈変竴涓熀浜?53C8XX 鐨?SCSI 鏉垮崱甯︽湁鍘傚晢鐗瑰畾鐨?BIOS锛?
    鍒欑粷涓嶈兘鍚敤姝ら€夐」銆備緥濡傦紝Tekram DC-390/U銆丏C-390/W 鍜?DC-390/F
    SCSI 鎺у埗鍣ㄤ娇鐢ㄥ巶鍟嗙壒瀹氱殑 BIOS锛屽苟涓斿凡鐭ヤ笉浣跨敤 SYMBIOS 鍏煎鐨?GPIO
    鎺ョ嚎銆傚洜姝わ紝濡傛灉浣犵殑绯荤粺瀹夎浜嗚繖鏍风殑鏉垮崱锛屽垯缁濅笉鑳藉惎鐢ㄦ閫夐」銆?

CONFIG_SCSI_NCR53C8XX_NVRAM_DETECT
    鍚敤瀵?Symbios 浠ュ強閮ㄥ垎 Symbios 鍏煎鍗★紙杩樻湁 Tekram DC390W/U/F 鍗★級
    涓婄殑涓茶 NVRAM 鏁版嵁鐨勮鍙栨敮鎸併€傚浜庢嫢鏈夊涓?Symbios 鍏煎鎺у埗鍣ㄣ€?
    鍏朵腑鑷冲皯涓€涓甫鏈変覆琛?NVRAM 鐨勭郴缁燂紝鎴栧悓鏃舵贩鏈?Symbios 涓?Tekram
    鍗＄殑绯荤粺寰堟湁鐢ㄣ€傚彲璁╀富鏈洪€傞厤鍣ㄧ殑鎵弿椤哄簭璁剧疆涓洪粯璁ら『搴忔垨"鍙嶅悜鎺㈡祴"
    椤哄簭涔嬪鐨勫叾浠栭『搴忋€?
    杩樿兘鍖哄垎 Symbios 鍗′笌 Tekram 鍗★紝浠庤€屽彲浠ュ湪娣锋湁 Symbios 涓?Tekram
    鍗＄殑绯荤粺涓婅缃?CONFIG_SCSI_NCR53C8XX_SYMBIOS_COMPAT锛屼娇 Symbios 鍗?
    鑳藉浣跨敤鍖呮嫭宸垎銆丩ED 寮曡剼鍦ㄥ唴鐨勫叏閮?Symbios 鐗规€э紝鑰屼笉浼氱粰 Tekram
    鍗″甫鏉ラ棶棰樸€?


## 10. 鍚姩璁剧疆鍛戒护


### 10.1 璇硶


鍚姩璁剧疆鍛戒护鏃㈠彲浠ュ湪鍚姩鏃朵紶閫掔粰椹卞姩锛屼篃鍙互浣滀负瀛楃涓插彉閲忎娇鐢?
'insmod' 浼犻€掋€?

ncr53c8xx锛坰ym53c8xx锛夐┍鍔ㄧ殑鍚姩璁剧疆鍛戒护浠ラ┍鍔ㄥ悕 "ncr53c8xx="锛坰ym53c8xx锛?
寮€澶淬€傜劧鍚庡唴鏍歌娉曡В鏋愬櫒鏈熸湜涓€涓彲閫夌殑銆佷互閫楀彿鍒嗛殧鐨勬暣鏁板垪琛紝鍚庤窡涓€涓?
鍙€夌殑銆佷互閫楀彿鍒嗛殧鐨勫瓧绗︿覆鍒楄〃銆俵ilo 涓嬪惎鍔ㄨ缃懡浠ょ殑绀轰緥锛?

```
    lilo: linux root=/dev/hda2 ncr53c8xx=tags:4,sync:10,debug:0x200

```
- 鍚敤鏍囪鍛戒护锛屾渶澶氭帓闃?4 鏉℃爣璁板懡浠ゃ€?
- 灏嗗悓姝ュ崗鍟嗛€熷害璁句负 10 鍏嗕紶杈?绉掋€?
- 璁剧疆 DEBUG_NEGO 鏍囧織銆?

鐢变簬鍦ㄤ娇鐢?'insmod' 瀹氫箟瀛楃涓插彉閲忔椂浼间箮涓嶅厑璁镐娇鐢ㄩ€楀彿锛岄┍鍔ㄤ篃鎺ュ彈
灏?<绌烘牸> 浣滀负閫夐」鍒嗛殧绗︺€備互涓嬪懡浠ゅ皢浣跨敤涓庝笂闈㈢浉鍚岀殑閫夐」瀹夎椹卞姩妯″潡锛?

```
    insmod ncr53c8xx.o ncr53c8xx="tags:4 sync:10 debug:0x200"

```
鐩墠锛屾暣鏁板弬鏁板垪琛ㄤ細琚┍鍔ㄤ涪寮冦€傚皢鏉ヤ細鐢ㄤ簬鏀寔鎸夋帶鍒跺櫒杩涜璁剧疆銆?

姣忎釜瀛楃涓插弬鏁板繀椤绘寚瀹氫负 "keyword:value"銆傚彧鍏佽灏忓啓瀛楁瘝鍜屾暟瀛椼€?

鍦ㄥ寘鍚涓?53C8xx 閫傞厤鍣ㄧ殑绯荤粺涓紝insmod 浼氬湪姣忎釜閫傞厤鍣ㄤ笂瀹夎鎸囧畾鐨?
椹卞姩銆傝鎺掗櫎鏌愪釜鑺墖锛岃浣跨敤 'excl' 鍏抽敭瀛椼€?

```
    insmod sym53c8xx sym53c8xx=excl:0x1400
    insmod ncr53c8xx

```
灏嗗湪闄?IO 绔彛鍦板潃 0x1400 澶勭殑閫傞厤鍣ㄤ箣澶栫殑鎵€鏈夐€傞厤鍣ㄤ笂瀹夎 sym53c8xx
椹卞姩锛岀劧鍚庡湪 IO 绔彛鍦板潃 0x1400 澶勭殑閫傞厤鍣ㄤ笂瀹夎 ncr53c8xx 椹卞姩銆?


### 10.2 鍙敤鍙傛暟


##### 10.2.1  涓昏澶囧鍋舵牎楠?


	======     ========
        mpar:y     enabled
        mpar:n     disabled
	======     ========

##### 10.2.2  SCSI 濂囧伓鏍￠獙


	======     ========
        spar:y     enabled
        spar:n     disabled
	======     ========

##### 10.2.3  SCSI 鏂紑杩炴帴


	======     ========
        disc:y     enabled
        disc:n     disabled
	======     ========

##### 10.2.4  鐗规畩鐗规€?


   浠呴€傜敤浜?810A銆?25A銆?60銆?75 鍜?895 鎺у埗鍣ㄣ€傚鍏朵粬鎺у埗鍣ㄦ棤鏁堛€?

	=======    =================================================
        specf:y    (or 1) enabled
        specf:n    (or 0) disabled
        specf:3           enabled except Memory Write And Invalidate
	=======    =================================================

   椹卞姩鐨勯粯璁よ缃负 'specf:3'銆傚洜姝わ紝鑻ヨ鍚敤"鍐呭瓨鍐欏苟浣挎棤鏁?
   锛圡emory Write And Invalidate锛夛紝蹇呴』鍦ㄥ惎鍔ㄨ缃懡浠や腑鎸囧畾閫夐」 'specf:y'銆?

##### 10.2.5  Ultra SCSI 鏀寔


   浠呴€傜敤浜?860銆?75銆?95銆?95a銆?96銆?010 鍜?1010_66 鎺у埗鍣ㄣ€傚鍏朵粬鎺у埗鍣ㄦ棤鏁堛€?

	=======    ========================
        ultra:n    All ultra speeds enabled
        ultra:2    Ultra2 enabled
        ultra:1    Ultra enabled
        ultra:0    Ultra speeds disabled
	=======    ========================

##### 10.2.6  榛樿鏍囪鍛戒护鏁伴噺


	======================= ===============================
        tags:0     (or tags:1 ) tagged command queuing disabled
        tags:#tags (#tags  > 1) tagged command queuing enabled
	======================= ===============================

  #tags 浼氳鎴柇涓?鏈€澶ф帓闃熷懡浠ゆ暟"閰嶇疆鍙傛暟鐨勫€笺€傛閫夐」杩樺厑璁镐负
  姣忎釜鏀寔鏍囪鍛戒护闃熷垪鐨勮澶囨寚瀹氬懡浠ら槦鍒楁繁搴︺€?

```
      ncr53c8xx=tags:10/t2t3q16-t5q24/t1u2q32

  will set devices queue depth as follow:

      - controller #0 target #2 and target #3                  -> 16 commands,
      - controller #0 target #5                                -> 24 commands,
      - controller #1 target #1 logical unit #2                -> 32 commands,
      - all other logical units (all targets, all controllers) -> 10 commands.

```
##### 10.2.7  榛樿鍚屾鍛ㄦ湡鍥犲瓙


============ ========================================================
sync:255     disabled (asynchronous transfer mode)
sync:#factor
	     ============     =======================================
	     #factor = 10     Ultra-2 SCSI 40 Mega-transfers / second
	     #factor = 11     Ultra-2 SCSI 33 Mega-transfers / second
	     #factor < 25     Ultra   SCSI 20 Mega-transfers / second
	     #factor < 50     Fast    SCSI-2
	     ============     =======================================
============ ========================================================

  鍦ㄦ墍鏈夋儏鍐典笅锛岄┍鍔ㄩ兘灏嗘牴鎹?NCR53C8XX 鑺墖绫诲瀷浣跨敤鎺у埗鍣ㄦ墍鏀寔鐨勬渶灏?
  浼犺緭鍛ㄦ湡銆?

##### 10.2.8  涓庢墍鏈夎澶囧崗鍟嗗悓姝ヤ紶杈?


        (force sync nego)

        =====      =========
        fsn:y      enabled
        fsn:n      disabled
        =====      =========

##### 10.2.9  璇︾粏杈撳嚭绾у埆


        ======     =========
        verb:0     minimal
        verb:1     normal
        verb:2     too much
        ======     =========

##### 10.2.10 璋冭瘯妯″紡


========   ==================================================================
debug:0    clear debug flags
debug:#x   set debug flags

	    #x is an integer value combining the following power-of-2 values:

	    =============  ======
	    DEBUG_ALLOC       0x1
	    DEBUG_PHASE       0x2
	    DEBUG_POLL        0x4
	    DEBUG_QUEUE       0x8
	    DEBUG_RESULT     0x10
	    DEBUG_SCATTER    0x20
	    DEBUG_SCRIPT     0x40
	    DEBUG_TINY       0x80
	    DEBUG_TIMING    0x100
	    DEBUG_NEGO      0x200
	    DEBUG_TAGS      0x400
	    DEBUG_FREEZE    0x800
	    DEBUG_RESTART  0x1000
	    =============  ======
========   ==================================================================

  浣犲彲浠ュ畨鍏ㄥ湴璇曠敤 DEBUG_NEGO銆備絾鏄紝鍏朵腑鏌愪簺鏍囧織鍙兘浼氫骇鐢熷ぇ閲?
  syslog 娑堟伅銆?

##### 10.2.11 绐佸彂鏈€澶ч暱搴?


=========  ==================================================================
burst:0    burst disabled
burst:255  get burst length from initial IO register settings.
burst:#x   burst enabled (1<<#x burst transfers max)

	   #x 鏄竴涓暣鏁板€硷紝涓虹獊鍙戜紶杈撴渶澶у€肩殑浠?2 涓哄簳鐨勫鏁般€?

	   NCR53C875 涓?NCR53C825A 鏈€澶氭敮鎸?128 娆＄獊鍙戜紶杈擄紙#x = 7锛夈€?

	   鍏朵粬鑺墖鏈€澶氬彧鏀寔 16 娆★紙#x = 4锛夈€?

	   杩欐槸涓€涓渶澶у€笺€傞┍鍔ㄦ牴鎹姱鐗囦笌淇 ID 璁剧疆绐佸彂闀垮害銆傞粯璁ゆ儏鍐典笅
	   椹卞姩浣跨敤鑺墖鎵€鏀寔鐨勬渶澶у€笺€?
=========  ==================================================================

##### 10.2.12 LED 鏀寔


        =====      ===================
        led:1      enable  LED support
        led:0      disable LED support
        =====      ===================

  濡傛灉浣犵殑 SCSI 鏉垮崱涓嶄娇鐢?SDMS BIOS锛岃鍕垮惎鐢?LED 鏀寔銆?
  锛堝弬瑙?閰嶇疆鍙傛暟"锛?

##### 10.2.13 鏈€澶ф€荤嚎瀹藉害


        ======     ===================
        wide:1      wide scsi enabled
        wide:0      wide scsi disabled
        ======     ===================

  鏌愪簺 SCSI 鏉垮崱浣跨敤 875锛坲ltra wide锛変絾鍙彁渚涚獎鍨嬭繛鎺ュ櫒銆傚鏋滀綘鐢ㄤ竴鏍?
  50 閽堣浆 68 閽堢殑鐢电紗杞帴鍣ㄨ繛鎺ヤ簡涓€涓鍨嬭澶囷紝浠讳綍琚帴鍙楃殑瀹藉崗鍟嗛兘浼?
  鐮村潖鍚庣画鐨勬暟鎹紶杈撱€傚湪杩欑鎯呭喌涓嬶紝鍦ㄥ惎鍔ㄥ懡浠や腑浣跨敤 "wide:0" 浼氬緢鏈夊府鍔┿€?

##### 10.2.14 宸垎妯″紡


	======	=================================
        diff:0	never set up diff mode
        diff:1	set up diff mode if BIOS set it
        diff:2	always set up diff mode
        diff:3	set diff mode if GPIO3 is not set
	======	=================================

##### 10.2.15 涓柇璇锋眰妯″紡


	=========  ========================================================
        irqm:0     always open drain
        irqm:1     same as initial settings (assumed BIOS settings)
        irqm:2     always totem pole
        irqm:0x10  driver will not use IRQF_SHARED flag when requesting irq
	=========  ========================================================
    锛?x10 涓?0x20 浣嶅彲涓庣‖浠朵腑鏂姹傛ā寮忛€夐」缁勫悎锛?

##### 10.2.16 鍙嶅悜鎺㈡祴


	=========   ========================================================
        revprob:n   probe chip ids from the PCI configuration in this order:
                    810, 815, 820, 860, 875, 885, 895, 896
        revprob:y   probe chip ids in the reverse order.
	=========   ========================================================

##### 10.2.17 淇 PCI 閰嶇疆绌洪棿


        pcifix:<option bits>

    Available option bits:

	===    ===============================================================
        0x0    No attempt to fix PCI configuration space registers values.
        0x1    Set PCI cache-line size register if not set.
        0x2    Set write and invalidate bit in PCI command register.
        0x4    Increase if necessary PCI latency timer according to burst max.
	===    ===============================================================

    浣跨敤 'pcifix:7' 浠ュ厑璁搁┍鍔ㄤ慨澶嶆墍鏈?PCI 鐗规€с€?

##### 10.2.18 涓茶 NVRAM


	=======     =========================================
        nvram:n     do not look for serial NVRAM
        nvram:y     test controllers for onboard serial NVRAM
	=======     =========================================

        锛堜簩杩涘埗澶囬€夊舰寮忥級
        mvram=<bits options>

        ====   =================================================================
        0x01   look for NVRAM  (equivalent to nvram=y)
        0x02   ignore NVRAM "Synchronous negotiation" parameters for all devices
        0x04   ignore NVRAM "Wide negotiation"  parameter for all devices
        0x08   ignore NVRAM "Scan at boot time" parameter for all devices
        0x80   also attach controllers set to OFF in the NVRAM (sym53c8xx only)
        ====   =================================================================

##### 10.2.19 妫€鏌?SCSI 鎬荤嚎


        buschk:<option bits>

    Available option bits:

        ====   ================================================
        0x0:   No check.
        0x1:   Check and do not attach the controller on error.
        0x2:   Check and just warn on error.
        0x4:   Disable SCSI bus integrity checking.
        ====   ================================================

##### 10.2.20 鎺掗櫎鏌愪釜涓绘満涓嶈鎸傝浇


        excl=<io_address>

    闃绘浣嶄簬缁欏畾 IO 鍦板潃鐨勪富鏈鸿鎸傝浇銆?
    渚嬪 'ncr53c8xx=excl:0xb400,excl:0xc000' 鎸囩ず ncr53c8xx 椹卞姩
    涓嶈鎸傝浇鍦板潃涓?0xb400 涓?0xc000 鐨勪富鏈恒€?

##### 10.2.21 涓轰富鏈哄缓璁粯璁?SCSI ID


	==========	==========================================
        hostid:255	no id suggested.
        hostid:#x	(0 < x < 7) x suggested for hosts SCSI id.
	==========	==========================================

    濡傛灉 NVRAM 涓彁渚涗簡涓绘満 SCSI ID锛岄┍鍔ㄥ皢蹇界暐浠讳綍浣滀负鍚姩閫夐」寤鸿鐨?
    鍊笺€傚惁鍒欙紝濡傛灉鎻愪緵浜嗕笉鍚屼簬 255 鐨勫缓璁€硷紝鍒欎細浣跨敤瀹冦€傚惁鍒欙紝瀹冧細
    灏濊瘯鎺ㄦ柇姝ゅ墠鍦ㄧ‖浠朵腑璁剧疆鐨勫€硷紝骞跺湪纭欢鍊间负闆舵椂浣跨敤鍊?7銆?

##### 10.2.22 鍚敤 IMMEDIATE ARBITRATION锛堢珛鍗充徊瑁侊級


        锛堜粎鐢?sym53c8xx 椹卞姩鏀寔銆傝瑙?10.7锛?

=======   =================================================================
iarb:0    do not use this feature.
iarb:#x   use this feature according to bit fields as follow:

	  ========= =======================================================
	  bit 0 (1) enable IARB each time the initiator has been reselected
		    when it arbitrated for the SCSI BUS.
	  (#x >> 4) maximum number of successive settings of IARB if the
		    initiator win arbitration and it has other commands
		    to send to a device.
	  ========= =======================================================
=======   =================================================================

Boot fail safe
    safe:y	load the following assumed fail safe initial setup

  ========================	======================	==========
  master parity			disabled		mpar:n
  scsi parity			enabled			spar:y
  disconnections		not allowed		disc:n
  special features		disabled		specf:n
  ultra scsi			disabled		ultra:n
  force sync negotiation	disabled		fsn:n
  reverse probe			disabled		revprob:n
  PCI fix up                    disabled                pcifix:0
  serial NVRAM                  enabled                 nvram:y
  verbosity level		2			verb:2
  tagged command queuing	disabled		tags:0
  synchronous negotiation	disabled		sync:255
  debug flags			none			debug:0
  burst length			from BIOS settings	burst:255
  LED support			disabled		led:0
  wide support			disabled		wide:0
  settle time			10 seconds		settle:10
  differential support		from BIOS settings	diff:1
  irq mode			from BIOS settings	irqm:1
  SCSI BUS check		do not attach on error	buschk:1
  immediate arbitration		disabled		iarb:0
  ========================	======================	==========

##### 10.3 寤鸿鐨勫惎鍔ㄨ缃懡浠?


濡傛灉椹卞姩浣跨敤榛樿閫夐」閰嶇疆锛岀瓑鏁堢殑

```
   ncr53c8xx=mpar:y,spar:y,disc:y,specf:3,fsn:n,ultra:2,fsn:n,revprob:n,verb:1\
             tags:0,sync:50,debug:0,burst:7,led:0,wide:1,settle:2,diff:0,irqm:0

```
瀵逛簬涓€涓畨瑁呰蒋鐩樻垨涓€涓畨鍏ㄤ絾涓嶅揩鐨勭郴缁燂紝

```
    ncr53c8xx=safe:y,mpar:y,disc:y
    ncr53c8xx=safe:y,disc:y
    ncr53c8xx=safe:y,mpar:y
    ncr53c8xx=safe:y

```
```

   ncr53c8xx=mpar:y,spar:y,disc:y,specf:1,fsn:n,ultra:2,fsn:n,revprob:n,verb:1\
             tags:32,sync:12,debug:0,burst:7,led:1,wide:1,settle:2,diff:0,irqm:0

```
褰撹缁嗚緭鍑虹骇鍒负 2 鏃讹紝椹卞姩浼氭墦鍗板叾瀹為檯璁剧疆銆備綘鍙互灏濊瘯
"ncr53c8xx=verb:2" 鏉ヨ幏鍙栭┍鍔ㄧ殑"闈欐€?璁剧疆锛屾垨鑰呭湪浣犵殑鍚姩璁剧疆鍛戒护涓?
鍔犱笂 "verb:2" 浠ユ煡鐪嬮┍鍔ㄦ鍦ㄤ娇鐢ㄧ殑瀹為檯璁剧疆銆?

### 10.4 PCI 閰嶇疆淇鍚姩閫夐」


pcifix:<option bits>

Available option bits:

    ===      =====================================================
    0x1      Set PCI cache-line size register if not set.
    0x2      Set write and invalidate bit in PCI command register.
    ===      =====================================================

浣跨敤 'pcifix:3' 浠ュ厑璁搁┍鍔ㄤ慨澶嶈繖涓ら」 PCI 鐗规€с€?

杩欎簺閫夐」浠呴€傜敤浜庢柊鐨?SYMBIOS 鑺墖 810A銆?25A銆?60銆?75 鍜?895锛屽苟涓?
浠呮敮鎸?Pentium 涓?486 绾у鐞嗗櫒銆傝繎鏈熺殑 SYMBIOS 53C8XX SCSI 澶勭悊鍣ㄨ兘澶?
浣跨敤 PCI 璇诲涓紙read multiple锛変笌 PCI 鍐欏苟浣挎棤鏁堬紙write and invalidate锛?
鍛戒护銆傝繖浜涚壒鎬ц姹傝姱鐗囩殑 PCI 閰嶇疆绌洪棿涓纭缃簡缂撳瓨琛屽ぇ灏忓瘎瀛樺櫒銆?
鍙︿竴鏂归潰锛岃姱鐗囧彧鏈夊湪 PCI 鍛戒护瀵勫瓨鍣ㄤ腑鐩稿簲浣嶈缃负 1 鏃舵墠浼氫娇鐢?PCI 鍐?
骞朵娇鏃犳晥鍛戒护銆?

骞堕潪鎵€鏈?PCI BIOS 閮戒細璁剧疆 53C8XX 鑺墖 PCI 閰嶇疆绌洪棿涓殑 PCI 缂撳瓨琛?
瀵勫瓨鍣ㄤ笌 PCI 鍐欏苟浣挎棤鏁堜綅銆備紭鍖栫殑 PCI 璁块棶鍦ㄦ煇浜?PCI/鍐呭瓨鎺у埗鍣ㄤ笂鍙兘浼?
鍑洪棶棰橈紝鎴栧湪鏌愪簺 PCI 鏉垮崱涓婁骇鐢熼棶棰樸€?

姝や慨澶嶅湪鎴戠殑鏃х郴缁熶笂杩愯瀹岀編銆?
锛堜富鏉?Triton HX / 53C875 / 53C810A锛?
鎴戜娇鐢ㄨ繖浜涢€夐」闇€鑷媴椋庨櫓锛屽鏋滀綘鍐冲畾浣跨敤瀹冧滑涔熸槸濡傛銆?

### 10.5 涓茶 NVRAM 鏀寔鍚姩閫夐」


=======     =========================================
nvram:n     do not look for serial NVRAM
nvram:y     test controllers for onboard serial NVRAM
=======     =========================================

姝ら€夐」涔熷彲浠ヤ互鍗佸叚杩涘埗鍊煎舰寮忚緭鍏ワ紝鐢ㄤ簬鎺у埗椹卞姩灏嗕粠 NVRAM 鑾峰彇鍝簺
淇℃伅銆佸拷鐣ュ摢浜涗俊鎭€?
璇︽儏鍙傝"17. 涓茶 NVRAM 鏀寔"銆?

鍚敤姝ら€夐」鏃讹紝椹卞姩浼氬皾璇曟娴嬫墍鏈変娇鐢ㄤ覆琛?NVRAM 鐨勬澘鍗°€傝瀛樺偍鍣ㄧ敤浜?
淇濆瓨鐢ㄦ埛璁剧疆鐨勫弬鏁般€?

椹卞姩鑳藉浠?NVRAM 鑾峰彇鐨勫弬鏁板彇鍐充簬鎵€浣跨敤鐨勬暟鎹牸寮忥紝濡備笅鎵€绀猴細

+-------------------------------+------------------+--------------+
|                               |Tekram format     |Symbios format|
+-------------------------------+------------------+--------------+
|General and host parameters    |                  |              |
+-------------------------------+------------------+--------------+
|  * Boot order                 |        N         |       Y      |
+-------------------------------+------------------+--------------+
|  * Host SCSI ID               |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * SCSI parity checking       |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Verbose boot messages      |        N         |       Y      |
+-------------------------------+------------------+--------------+
|SCSI devices parameters                                          |
+-------------------------------+------------------+--------------+
|  * Synchronous transfer speed |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Wide 16 / Narrow           |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Tagged Command Queuing     |        Y         |       Y      |
|    enabled                    |                  |              |
+-------------------------------+------------------+--------------+
|  * Disconnections enabled     |        Y         |       Y      |
+-------------------------------+------------------+--------------+
|  * Scan at boot time          |        N         |       Y      |
+-------------------------------+------------------+--------------+

涓轰簡鍔犲揩绯荤粺鍚姩锛屽浜庢瘡涓厤缃负涓嶅甫"鍚姩鏃舵壂鎻?锛坰can at boot time锛?
閫夐」鐨勮澶囷紝椹卞姩浼氬湪鏀跺埌鐨勮璁惧鐨勭涓€鏉?TEST UNIT READY 鍛戒护涓婂己鍒?
杩斿洖涓€涓敊璇€?

鏌愪簺 SDMS BIOS 淇鐗堜技涔庢棤娉曚笌闈炲父蹇殑纭洏涓€璧峰共鍑€鍦板惎鍔ㄣ€傚湪杩欑鎯呭喌涓嬶紝
浣犳棤娉曠敤浼樺寲鍚庣殑鍙傛暟鍊兼潵閰嶇疆 NVRAM銆?

'nvram' 鍚姩閫夐」鍙互浠ュ崄鍏繘鍒跺舰寮忚緭鍏ワ紝浠ュ拷鐣?NVRAM 涓厤缃殑鏌愪簺閫夐」锛?
濡備笅鎵€绀猴細

mvram=<bits options>

      ====   =================================================================
      0x01   look for NVRAM  (equivalent to nvram=y)
      0x02   ignore NVRAM "Synchronous negotiation" parameters for all devices
      0x04   ignore NVRAM "Wide negotiation"  parameter for all devices
      0x08   ignore NVRAM "Scan at boot time" parameter for all devices
      0x80   also attach controllers set to OFF in the NVRAM (sym53c8xx only)
      ====   =================================================================

閫夐」 0x80 浠呯敱 sym53c8xx 椹卞姩鏀寔锛岄粯璁ょ鐢ㄣ€傜粨鏋滄槸锛岄粯璁ゆ儏鍐典笅锛堟湭璁剧疆
璇ラ€夐」锛夛紝sym53c8xx 椹卞姩涓嶄細鎸傝浇鍦?NVRAM 涓涓?OFF 鐨勬帶鍒跺櫒銆?

ncr53c8xx 濮嬬粓灏濊瘯鎸傝浇鎵€鏈夋帶鍒跺櫒銆傞€夐」 0x80 娌℃湁琚姞鍏?ncr53c8xx 椹卞姩锛?
鍥犱负鎹姤鍛婂畠浼氳闀挎湡浣跨敤璇ラ┍鍔ㄧ殑鐢ㄦ埛鎰熷埌鍥版儜銆傚鏋滀綘甯屾湜鍦?Linux 鍚姩鏃?
涓嶈 ncr53c8xx 椹卞姩鎸傝浇鏌愪釜鎺у埗鍣紝蹇呴』浣跨敤 'excl' 椹卞姩鍚姩閫夐」銆?

##### 10.6 SCSI 鎬荤嚎妫€鏌ュ惎鍔ㄩ€夐」銆?


褰撴閫夐」琚涓洪潪闆跺€兼椂锛岄┍鍔ㄤ細鍦ㄦ柇瑷€ SCSI RESET 绾?100 寰鍚庢鏌?SCSI
淇″彿绾跨殑閫昏緫鐘舵€併€傞┍鍔ㄥ彧鏄鍙?SCSI 淇″彿绾匡紝骞舵鏌ラ櫎 RESET 澶栨墍鏈変俊鍙风嚎
璇诲彇缁撴灉鍧囦负 FALSE銆傜敱浜?SCSI 璁惧鍦?SCSI RESET 琚柇瑷€鍚庢渶澶?800 绾崇鍐?
浼氶噴鏀炬€荤嚎锛屼换浣曚俊鍙蜂负 TRUE 閮藉彲鑳借〃鏄庡瓨鍦?SCSI 鎬荤嚎闂銆傞仐鎲剧殑鏄紝
浠ヤ笅甯歌鐨?SCSI 鎬荤嚎闂鏃犳硶琚娴嬪埌锛?

- 鍙畨瑁呬簡涓€涓粓缁撳櫒锛坱erminator锛夈€?
- 缁堢粨鍣ㄤ綅缃敊璇€?
- 缁堢粨鍣ㄨ川閲忓樊銆?

鍙︿竴鏂归潰锛岄敊璇殑甯冪嚎銆佹崯鍧忕殑璁惧銆佷笉绗﹀悎瑙勮寖鐨勮澶団€︹€﹂兘鍙兘瀵艰嚧椹卞姩
璇诲彇鏃舵煇涓?SCSI 淇″彿閿欒銆?

##### 10.7 IMMEDIATE ARBITRATION 鍚姩閫夐」


姝ら€夐」浠呯敱 SYM53C8XX 椹卞姩鏀寔锛堜笉鐢?NCR53C8XX 鏀寔锛夈€?

SYMBIOS 53C8XX 鑺墖鑳藉鍦ㄦ娴嬪埌棰勬湡鐨勬柇寮€杩炴帴锛圔US FREE 鐩镐綅锛夊悗
绔嬪嵆瀵?SCSI 鎬荤嚎杩涜浠茶銆傝浣胯杩囩▼鍚姩锛屽綋鑺墖杩炴帴鍒?SCSI 鎬荤嚎鏃讹紝
SCNTL1 IO 瀵勫瓨鍣ㄧ殑浣?1 蹇呴』琚疆浣嶃€?

褰撲负褰撳墠杩炴帴鍚敤浜嗘鐗规€у悗锛屽鏋滃彧鏈変紭鍏堢骇杈冧綆鐨勮澶囧湪绔炰簤 SCSI 鎬荤嚎锛?
鑺墖鏈夊崄瓒崇殑鎶婃彙璧㈠緱浠茶銆傞『渚胯涓€鍙ワ紝褰撹姱鐗囦娇鐢?SCSI ID 7 鏃讹紝瀹冨繀灏?
璧㈠緱涓嬩竴娆?SCSI 鎬荤嚎浠茶銆?

鐢变簬鏃犳硶鐭ラ亾鏈夊摢浜涜澶囨璇曞浘瀵规€荤嚎杩涜浠茶锛屼娇鐢ㄦ鐗规€у彲鑳芥瀬涓嶅叕骞炽€?
鍥犳锛屼笉寤鸿浣犲惎鐢ㄥ畠锛屾垨鑰呮渶澶氫粎涓鸿姱鐗囧湪涓婁竴娆′徊瑁佷腑澶辫触鐨勬儏鍐?
锛堝惎鍔ㄩ€夐」 'iarb:1'锛夊惎鐢ㄦ鐗规€с€?

姝ょ壒鎬у叿鏈変互涓嬩紭鐐癸細

a) 鍏佽 ID 涓?7 鐨勫彂璧疯€呭湪闇€瑕佹椂璧㈠緱浠茶銆?
b) 灏嗚嚦灏?4 寰鐨勪徊瑁佹椂闂翠笌澶勭悊褰撳墠杩炴帴缁撴潫骞跺紑濮嬩笅涓€涓换鍔＄殑
   SCRIPTS 鎵ц閲嶅彔銆?

鍡€︹€︿絾鏄紙a锛夊彲鑳藉彧鏄樆姝㈠叾浠栬澶囬噸鏂伴€夋嫨鍙戣捣鑰咃紝骞跺欢杩熸暟鎹紶杈撴垨
鐘舵€?瀹屾垚锛涜€岋紙b锛夊鏋?SCRIPTS 鎵ц鎸佺画鏃堕棿瓒呰繃 4 寰锛屽彲鑳藉彧鏄?
娴垂 SCSI 鎬荤嚎甯﹀銆?

浣跨敤 IARB 闇€瑕佸湪缂栬瘧鏃跺畾涔?SCSI_NCR_IARB_SUPPORT 閫夐」锛屽苟鍦ㄥ惎鍔ㄦ椂灏?
'iarb' 鍚姩閫夐」璁句负闈為浂鍊笺€傚畠瀵瑰疄闄呭伐浣滃苟涓嶆槸閭ｄ箞鏈夌敤锛屼絾鍙敤浜?
缁?SCSI 璁惧鏂藉姞鍘嬪姏锛屾垨鐢ㄤ簬鏌愪簺鑳戒粠涓幏鐩婄殑搴旂敤銆傞『渚胯涓€鍙ワ紝濡傛灉
浣犲湪楂?IO 璐熻浇涓嬩娇鐢?IARB 鏃堕亣鍒拌濡?鎰忓鐨勬柇寮€杩炴帴"銆?閿欒鐨勯噸鏂伴€夋嫨"
绛夐棶棰橈紝浣犱笉搴旇鎰熷埌鎯婅锛屽洜涓哄悓鏃跺己琛屽杺鍏ヤ换浣曚笢瑗垮張鍫典綇瀹冪殑鍚庤矾
鏄笉鍙兘闀挎湡濂忔晥鐨勩€?:-))

## 11. ncr53c8xx.h 澶存枃浠朵腑鐨勪竴浜涘父閲忎笌鏍囧織


鍏朵腑涓€浜涚敱閰嶇疆鍙傛暟瀹氫箟銆傝鏇存敼鍏朵粬 "defines"锛屼綘蹇呴』缂栬緫澶存枃浠躲€?
鍙湁鍦ㄤ綘鐭ラ亾鑷繁鍦ㄥ仛浠€涔堟椂鎵嶈繖鏍峰仛銆?

SCSI_NCR_SETUP_SPECIAL_FEATURES	(default: defined)
	濡傛灉瀹氫箟锛岄┍鍔ㄥ皢鏍规嵁鑺墖涓庝慨璁?ID 鍚敤涓€浜涚壒娈婄壒鎬с€?

        瀵逛簬 810A銆?60銆?25A銆?75 鍜?895 杩欎簺 SCSI 鑺墖锛屾閫夐」鍚敤
	鍦?SCSI 浼犺緭澶勭悊鏈熼棿鍑忚交 PCI 鎬荤嚎涓庡唴瀛樿闂礋杞界殑鐗规€э細绐佸彂
	鍙栨寚锛坆urst op-code fetch锛夈€佽澶氫釜锛坮ead multiple锛夈€佽琛?
	锛坮ead line锛夈€侀鍙栵紙prefetch锛夈€佺紦瀛樿锛坈ache line锛夈€佸啓骞朵娇鏃犳晥
	锛坵rite and invalidate锛夈€佺獊鍙?128锛堜粎 875锛夈€佸ぇ DMA FIFO
	锛堜粎 875锛夈€佸亸绉?16锛堜粎 875锛夈€?

```
	ncr53c8xx=specf:n

```
SCSI_NCR_IOMAPPED		(default: not defined)
	濡傛灉瀹氫箟锛屽己鍒朵娇鐢ㄦ櫘閫?I/O銆?

SCSI_NCR_SHARE_IRQ		(default: defined)
	濡傛灉瀹氫箟锛岃姹傚叡浜?IRQ銆?

SCSI_NCR_MAX_TAGS		(default: 8)
	鍒版煇涓澶囩殑骞跺彂鏍囪鍛戒护鐨勬渶澶ф暟閲忋€?

	鍙€氳繃 "settags <target> <maxtags>" 鏇存敼

SCSI_NCR_SETUP_DEFAULT_SYNC     (default: 50)
	椹卞姩鍦ㄥ惎鍔ㄦ椂鐢ㄤ簬鍚屾鍗忓晢鐨勪紶杈撳懆鏈熷洜瀛愩€? 琛ㄧず寮傛銆?

	鍙€氳繃 "setsync <target> <period factor>" 鏇存敼

SCSI_NCR_SETUP_DEFAULT_TAGS     (default: 8)
	鍒版煇涓澶囩殑骞跺彂鏍囪鍛戒护鐨勯粯璁ゆ暟閲忋€?

	< 1 琛ㄧず鍚姩鏃剁鐢ㄦ爣璁板懡浠ら槦鍒椼€?

SCSI_NCR_ALWAYS_SIMPLE_TAG	(default: defined)
	瀵硅鍐欏懡浠や娇鐢?SIMPLE TAG銆?

	鍙€氳繃 "setorder <ordered|simple|default>" 鏇存敼

SCSI_NCR_SETUP_DISCONNECTION	(default: defined)
	濡傛灉瀹氫箟锛屽厑璁哥洰鏍囨柇寮€杩炴帴銆?

SCSI_NCR_SETUP_FORCE_SYNC_NEGO	(default: not defined)
	濡傛灉瀹氫箟锛屽鎵€鏈?SCSI-2 璁惧灏濊瘯鍚屾鍗忓晢銆?

	鍙€氳繃 "setsync <target> <period>" 鏇存敼

SCSI_NCR_SETUP_MASTER_PARITY	(default: defined)
	濡傛灉瀹氫箟锛屽惎鐢ㄤ富璁惧濂囧伓鏍￠獙銆?

SCSI_NCR_SETUP_SCSI_PARITY	(default: defined)
	濡傛灉瀹氫箟锛屽惎鐢?SCSI 濂囧伓鏍￠獙銆?

SCSI_NCR_PROFILE_SUPPORT	(default: not defined)
	濡傛灉瀹氫箟锛屾敹闆嗘€ц兘鍓栨瀽淇℃伅銆?

SCSI_NCR_MAX_SCATTER		(default: 128)
	椹卞姩 ccb 鐨勫垎鏁ｅ垪琛ㄥぇ灏忋€?

SCSI_NCR_MAX_TARGET		(default: 16)
	姣忎釜涓绘満鐨勬渶澶х洰鏍囨暟閲忋€?

SCSI_NCR_MAX_HOST		(default: 2)
	涓绘満鎺у埗鍣ㄧ殑鏈€澶ф暟閲忋€?

SCSI_NCR_SETTLE_TIME		(default: 2)
	椹卞姩鍦ㄥ浣嶅悗绛夊緟鐨勭鏁般€?

SCSI_NCR_TIMEOUT_ALERT		(default: 3)
	濡傛灉涓€鏉℃寕璧风殑鍛戒护灏嗗湪璇ョ鏁颁箣鍚庤秴鏃讹紝涓嬩竴鏉″懡浠ゅ皢浣跨敤
	鏈夊簭鏍囩锛坥rdered tag锛夈€?

	閬垮厤鏃犲簭鏍囪鍛戒护鐨勮秴鏃躲€?

SCSI_NCR_CAN_QUEUE		(default: 7*SCSI_NCR_MAX_TAGS)
	鍙帓闃熷埌鏌愪釜涓绘満鐨勬渶澶у懡浠ゆ暟閲忋€?

SCSI_NCR_CMD_PER_LUN		(default: SCSI_NCR_MAX_TAGS)
	鎺掗槦鍒版煇涓富鏈虹殑鏌愪釜璁惧鐨勫懡浠ゆ渶澶ф暟閲忋€?

SCSI_NCR_SG_TABLESIZE		(default: SCSI_NCR_MAX_SCATTER-1)
	Linux 鍒嗘暎/鑱氶泦鍒楄〃鐨勬渶澶уぇ灏忋€?

SCSI_NCR_MAX_LUN	(default: 8)
	姣忎釜鐩爣鐨勬渶澶?LUN 鏁伴噺銆?

## 12. 瀹夎


璇ラ┍鍔ㄦ槸 Linux 鍐呮牳鍙戣鐗堢殑涓€閮ㄥ垎銆傞┍鍔ㄦ枃浠朵綅浜庡唴鏍告簮浠ｇ爜鏍戠殑
"drivers/scsi" 瀛愮洰褰曚腑銆?

```
	README.ncr53c8xx	: this file
	ChangeLog.ncr53c8xx	: change log
	ncr53c8xx.h		: definitions
	ncr53c8xx.c		: the driver code

```
鏂扮増椹卞姩浼氬崟鐙彁渚涳紝浠ヤ究鍦ㄥ皢鍏剁撼鍏?Linux 鍐呮牳鍙戣鐗堜箣鍓嶆祴璇曞彉鏇翠笌
鏂扮壒鎬с€備互涓?URL 鎻愪緵浜嗘渶鏂板彲鐢ㄨˉ涓佺殑淇℃伅锛?

      ftp://ftp.tux.org/pub/people/gerard-roudier/README

## 13. 涓庝綋绯荤粨鏋勭浉鍏崇殑鐗规€?


<灏氭湭缂栧啓>

## 14. 宸茬煡闂


### 14.1 浣跨敤 Iomega Jaz 璁惧鐨勬爣璁板懡浠?


鎴戞病鏈夎瘯鐢ㄨ繃姝よ澶囷紝浣嗘湁浜哄悜鎴戞姤鍛婁簡浠ヤ笅鎯呭喌锛氭璁惧鍏峰鏍囪鍛戒护
闃熷垪鑳藉姏銆傜劧鑰屽湪鑷棆鍚姩锛坰pinning up锛夋湡闂达紝瀹冧細鎷掔粷鏍囪鍛戒护銆傝繖绉?
琛屼负绗﹀悎 SCSI-2 瑙勮寖 6.8.2 鑺傘€傞┍鍔ㄥ湪杩欑鎯呭喌涓嬬殑褰撳墠琛屼负骞朵笉浠や汉婊℃剰銆?
鍥犳锛屼笉瑕佷负鑳藉鑷棆闄嶉€燂紙spin down锛夌殑璁惧鍚敤鏍囪鍛戒护闃熷垪銆傚彟涓€涓?
鍙兘鍑虹幇鐨勯棶棰樻槸瓒呮椂銆傞伩鍏嶈秴鏃剁殑鍞竴鏂规硶浼间箮鏄紪杈?
linux/drivers/scsi/sd.c 骞跺澶у綋鍓嶇殑瓒呮椂鍊笺€?

### 14.2 娣诲姞鍙︿竴鎺у埗鍣ㄦ椂璁惧鍚嶅彂鐢熷彉鍖?


褰撲綘鍚戜竴涓凡缁忔嫢鏈変竴鍧楁垨澶氬潡璇ョ郴鍒楁帶鍒跺櫒鐨勭郴缁熸坊鍔犱竴鍧楁柊鐨?
NCR53C8XX 鑺墖鎺у埗鍣ㄦ椂锛岄┍鍔ㄥ悜鍐呮牳娉ㄥ唽瀹冧滑鐨勯『搴忓彲鑳戒細瀵艰嚧鍥犺澶囧悕
鍙樺寲鑰屼骇鐢熺殑闂銆傚綋鑷冲皯鏈変竴鍧楁帶鍒跺櫒浣跨敤 NvRAM 鏃讹紝SDMS BIOS 4 鐗堝厑璁?
浣犲畾涔?BIOS 鎵弿 SCSI 鏉垮崱鐨勯『搴忋€傚鏋滆缃簡 NvRAM 妫€娴嬮€夐」锛岄┍鍔ㄤ細
鏍规嵁 BIOS 淇℃伅鎸傝浇鎺у埗鍣ㄣ€?

濡傛灉浣犵殑鎺у埗鍣ㄦ病鏈?NvRAM锛屼綘鍙互锛?

- 鍦ㄥ惎鍔ㄥ懡浠よ涓姹傞┍鍔ㄤ互鐩稿弽椤哄簭鎺㈡祴鑺墖 ID锛歯cr53c8xx=revprob:y
- 瀵?fstab 鍋氶€傚綋鐨勪慨鏀广€?
- 浣跨敤 Eric Youngdale 鐨?'scsidev' 宸ュ叿銆?

### 14.3 鍦?WIDE SCSI 鎺у埗鍣ㄤ笂浠呬娇鐢?8 浣嶈澶?


褰撳彧鏈?8 浣嶇獎鍨嬶紙NARROW锛夎澶囪繛鎺ュ埌 16 浣嶅鍨嬶紙WIDE锛塖CSI 鎺у埗鍣ㄦ椂锛?
浣犲繀椤荤‘淇?SCSI 鎬荤嚎瀹藉瀷閮ㄥ垎鐨勪俊鍙风嚎琚笂鎷夈€傝繖鍙互閫氳繃鍚敤 SCSI
鎺у埗鍣ㄥ崱鐨勫鍨嬬粓缁撳櫒锛圵IDE TERMINATOR锛夐儴鍒嗘潵瀹炵幇銆?

TYAN 1365 鏂囨。 1.2 鐗堝叧浜庢绫昏缃殑鎻忚堪涓嶆纭€傦紙绗?10 椤碉紝鍥?3.3锛夈€?

### 14.4 鍐呭瓨鍐欏苟浣挎棤鏁堟湡闂村彲鑳藉嚭鐜扮殑鏁版嵁鎹熷潖


姝ら棶棰樺湪 SYMBIOS DEL 397銆侀儴浠跺彿 69-039241銆佹潯鐩?4 涓湁鎻忚堪銆?

鍦ㄦ煇浜涘鏉傛儏鍐典笅锛屼慨璁㈠彿 <= 3 鐨?53C875 鑺墖鍙兘浼氫粠涓€涓湭涓庣紦瀛樿
瀵归綈鐨?4 涓?DWORD 杈圭晫寮€濮?PCI 鍐欏苟浣挎棤鏁堝懡浠ゃ€傝繖鍙湁鍦ㄧ紦瀛樿澶у皬涓?
8 涓?DWORD 鎴栨洿澶ф椂鎵嶅彲鑳藉彂鐢熴€侾entium 绯荤粺浣跨敤 8 涓?DWORD 鐨勭紦瀛樿澶у皬锛?
鍥犳鍙楁鑺墖缂洪櫡褰卞搷锛岃€?i486 绯荤粺浣跨敤 4 涓?DWORD 鐨勭紦瀛樿澶у皬锛屼笉鍙?
褰卞搷銆?

褰撹繖绉嶆儏鍐靛彂鐢熸椂锛岃姱鐗囧彲鑳藉湪鍙～鍏呬簡浼犺緭鎵€娑夊強鐨勬渶鍚庝竴涓紦瀛樿鐨?
閮ㄥ垎鍐呭鍚庡氨瀹屾垚浜嗗啓骞朵娇鏃犳晥鍛戒护锛屼粠鑰岃璇ョ紦瀛樿鐨勫叾浣欓儴鍒嗗彂鐢熸暟鎹?
鎹熷潖銆?

涓嶄娇鐢ㄥ啓骞朵娇鏃犳晥鏄剧劧鍙互瑙勯伩姝よ姱鐗囩己闄凤紝鍥犳鐜板湪瀹冩槸椹卞姩鐨勯粯璁よ缃€?
鐒惰€岋紝瀵逛簬鍍忔垜杩欐牱鎯冲惎鐢ㄦ鐗规€х殑浜猴紝鎴戝姞鍏ヤ簡 SYMBIOS 寤鸿鐨勯儴鍒嗗簲瀵?
鏂规硶銆傝搴斿鏂规硶鍦ㄨ繘鍏?DATA IN 鐩镐綅鏃堕噸缃鍧€閫昏緫锛屼粠鑰岄槻姝㈣缂洪櫡鍦?
鐩镐綅鐨勭涓€娆?SCSI MOVE 鏃惰瑙﹀彂銆傛牴鎹互涓嬪垎鏋愶紝璇ュ簲瀵规柟娉曞簲褰撹冻澶燂細

椹卞姩鍐呴儴鍞竴澶т簬 8 涓?DWORD 涓旂敱 SCRIPTS 澶勭悊鍣ㄧЩ鍔ㄧ殑鏁版嵁缁撴瀯鏄?
鍖呭惈 SCSI 浼犺緭涓婁笅鏂囩殑"CCB 澶?锛圕CB header锛夈€傝鏁版嵁缁撴瀯鎸?8 涓?DWORD
杈圭晫锛圥entium 缂撳瓨琛屽ぇ灏忥級瀵归綈锛屽洜姝ゅ湪 Pentium 绯荤粺涓婁笉鍙楁鑺墖缂洪櫡
褰卞搷銆?

浣嗘槸锛屽綋浣跨敤鏈笌缂撳瓨琛屽榻愮殑 4 涓?DWORD 缂撳啿鍖烘墽琛?SCSI 璇诲懡浠ゆ椂锛?
鍙兘婊¤冻璇ョ己闄风殑鏉′欢銆傚湪 Linux 涓嬩娇鐢ㄥ垎鏁?鑱氶泦鍒楄〃鏃朵笉浼氬彂鐢熻繖绉嶆儏鍐碉紝
鍥犱负瀹冧滑鍙紩鐢ㄥ榻愯壇濂界殑绯荤粺缂撳啿鍖恒€傚洜姝わ紝鍦?Linux 涓嬶紝浠呭綋鏈娇鐢?
鍒嗘暎/鑱氶泦鍒楄〃锛屼笖鍦ㄧ浉浣嶅け閰嶅悗閲嶆柊杩涘叆 SCSI DATA IN 鐩镐綅鏃讹紝鎵嶅彲鑳介渶瑕?
搴斿鏂规硶銆?

## 15. SCSI 闂鎺掓煡


### 15.1 闂杩借釜


澶у鏁?SCSI 闂婧愪簬涓嶇鍚堣鑼冪殑 SCSI 鎬荤嚎鎴栨湁缂洪櫡鐨勮澶囥€傚鏋滀綘涓嶅垢
閬囧埌浜?SCSI 闂锛屽彲浠ユ鏌ヤ互涓嬩簨椤癸細

- SCSI 鎬荤嚎鐢电紗
- SCSI 閾句袱绔鐨勭粓缁撳櫒
- Linux 鐨?syslog 娑堟伅锛堝叾涓竴浜涘彲鑳戒細瀵逛綘鏈夊府鍔╋級

濡傛灉浣犳壘涓嶅埌闂鐨勬牴婧愶紝鍙互灏嗛┍鍔ㄩ厤缃负涓嶅惎鐢ㄤ换浣曠壒鎬с€?

- 浠呭紓姝ユ暟鎹紶杈?
- 绂佺敤鏍囪鍛戒护
- 涓嶅厑璁告柇寮€杩炴帴

鐜板湪锛屽鏋滀綘鐨?SCSI 鎬荤嚎姝ｅ父锛屼綘鐨勭郴缁熷緢鏈夋満浼氬湪姝ゅ畨鍏ㄩ厤缃笅宸ヤ綔锛?
浣嗘€ц兘涓嶄細鏄渶浼樼殑銆?

濡傛灉浠嶇劧澶辫触锛屽垯浣犲彲浠ュ皢浣犵殑闂鎻忚堪鍙戦€佸埌鐩稿簲鐨勯偖浠跺垪琛ㄦ垨鏂伴椈缁勩€?
缁欐垜鍙戜竴浠藉壇鏈紝浠ョ‘淇濇垜鑳芥敹鍒般€傛樉鐒讹紝椹卞姩浠ｇ爜涓彲鑳藉瓨鍦?bug銆?

     鎴戠殑鐢靛瓙閭欢鍦板潃锛欸erard Roudier <groudier@free.fr>

濡傛灉浣犲湪 SCSI 鎬荤嚎涓婁娇鐢ㄤ簡澶氫釜璁惧锛屽厑璁告柇寮€杩炴帴寰堥噸瑕侊紝浣嗗父甯镐細
瀵规湁缂洪櫡鐨勮澶囬€犳垚闂銆傚悓姝ユ暟鎹紶杈撳彲鎻愰珮鍍忕‖鐩樿繖鏍峰揩閫熻澶囩殑鍚炲悙閲忋€?
鎷ユ湁澶х紦瀛樼殑浼樿川 SCSI 纭洏鑳戒粠鏍囪鍛戒护闃熷垪涓幏鐩娿€?

灏濊瘯鐢ㄦ帶鍒跺懡浠や竴娆″惎鐢ㄤ竴涓壒鎬с€備緥濡傦細

```
    echo "setsync all 25" >/proc/scsi/ncr53c8xx/0

```
灏嗕负鎵€鏈夌洰鏍囧惎鐢ㄥ揩閫熷悓姝ユ暟鎹紶杈撳崗鍟嗐€?

```
    echo "setflag 3" >/proc/scsi/ncr53c8xx/0

```
灏嗛噸缃洰鏍?3 鐨勬爣蹇楋紙no_disc锛夛紝浠庤€屽厑璁稿畠鏂紑 SCSI 鎬荤嚎銆?

```
    echo "settags 3 8" >/proc/scsi/ncr53c8xx/0

```
濡傛灉璇ヨ澶囨敮鎸侊紝灏嗕负鐩爣 3 鍚敤鏍囪鍛戒护闃熷垪銆?

涓€鏃︿綘鎵惧埌浜嗗鑷撮棶棰樼殑璁惧涓庣壒鎬э紝鍙渶涓鸿璁惧绂佺敤璇ョ壒鎬у嵆鍙€?

### 15.2 鐞嗚В纭欢閿欒鎶ュ憡


褰撻┍鍔ㄦ娴嬪埌鎰忓鐨勯敊璇潯浠舵椂锛屽畠鍙兘浼氭樉绀?

```
    sym53c876-0:1: ERROR (0:48) (1-21-65) (f/95) @ (script 7c0:19000000).
    sym53c876-0: script cmd = 19000000
    sym53c876-0: regdump: da 10 80 95 47 0f 01 07 75 01 81 21 80 01 09 00.

```
姝ょ被娑堟伅涓殑鏌愪簺瀛楁鍙互甯姪浣犵悊瑙ｅ師鍥?

```
    sym53c876-0:1: ERROR (0:48) (1-21-65) (f/95) @ (script 7c0:19000000).
    ............A.........B.C....D.E..F....G.H.......I.....J...K.......

```
瀛楁 A锛氱洰鏍囩紪鍙枫€?
  鍙戠敓閿欒鏃舵帶鍒跺櫒姝ｅ湪涓庝箣閫氫俊鐨勮澶囩殑 SCSI ID銆?

瀛楁 B锛欴STAT IO 瀵勫瓨鍣紙DMA 鐘舵€侊級
  ========   =============================================================
  Bit 0x40   MDPE Master Data Parity Error
             鍦?PCI 鎬荤嚎涓婃娴嬪埌鐨勬暟鎹鍋堕敊璇€?
  Bit 0x20   BF   Bus Fault
             妫€娴嬪埌鐨?PCI 鎬荤嚎鏁呴殰鏉′欢銆?
  Bit 0x01   IID  Illegal Instruction Detected
             褰撹姱鐗囧湪鏌愪簺浣挎寚浠ら潪娉曠殑鏉′欢涓嬫娴嬪埌闈炴硶鎸囦护鏍煎紡鏃剁敱鑺墖缃綅銆?
  Bit 0x80   DFE Dma Fifo Empty
             绾姸鎬佷綅锛屼笉琛ㄧず閿欒銆?
  ========   =============================================================

  濡傛灉鎶ュ憡鐨?DSTAT 鍊煎寘鍚?MDPE (0x40) 涓?BF (0x20) 鐨勭粍鍚堬紝鍒欏師鍥?
  寰堝彲鑳芥槸 PCI 鎬荤嚎闂銆?

瀛楁 C锛歋IST IO 瀵勫瓨鍣紙SCSI 涓柇鐘舵€侊級
  ========   ==================================================================
  Bit 0x08   SGE  SCSI GROSS ERROR
             琛ㄧず鑺墖鍦?SCSI 鎬荤嚎涓婃娴嬪埌浜嗕弗閲嶇殑閿欒鏉′欢锛屽鑷?SCSI 鍗忚
             鏃犳硶姝ｅ父宸ヤ綔銆?
  Bit 0x04   UDC  Unexpected Disconnection
             琛ㄧず璁惧鍦ㄨ姱鐗囨湭棰勬湡鐨勬儏鍐典笅閲婃斁浜?SCSI 鎬荤嚎銆傝澶囧彲鑳藉姝?
             琛ㄧ幇锛屼互鍚?SCSI 鍙戣捣鑰呮寚绀哄彂鐢熶簡鏃犳硶鐢?SCSI 鍗忚鎶ュ憡鐨?
             閿欒鏉′欢銆?
  Bit 0x02   RST  SCSI BUS Reset
             閫氬父 SCSI 鐩爣涓嶄細澶嶄綅 SCSI 鎬荤嚎锛屽敖绠℃€荤嚎涓婄殑浠讳綍璁惧閮?
             鍙互鍦ㄤ换浣曟椂鍊欏浣嶅畠銆?
  Bit 0x01   PAR  Parity
             妫€娴嬪埌鐨?SCSI 濂囧伓閿欒銆?
  ========   ==================================================================

  鍦ㄦ湁鏁呴殰鐨?SCSI 鎬荤嚎涓婏紝鑺墖鍙兘妫€娴嬪埌 SGE (0x08)銆乁DC (0x04) 涓?
  PAR (0x01) 涓殑浠讳綍閿欒鏉′欢銆傚鏋滀綘鐨?SCSI 绯荤粺鏈夋椂閬囧埌姝ょ被閿欒鏉′欢锛?
  灏ゅ叾鏄?SCSI GROSS ERROR锛屽垯 SCSI 鎬荤嚎闂寰堝彲鑳芥槸杩欎簺閿欒鐨勬牴婧愩€?

瀵逛簬瀛楁 D銆丒銆丗銆丟 鍜?H锛屼綘鍙互鏌ョ湅 sym53c8xx_defs.h 鏂囦欢锛屽叾涓寘鍚?
瀵?IO 瀵勫瓨鍣ㄤ綅鐨勪竴浜涚畝瑕佹敞閲娿€?

瀛楁 D锛歋OCL  Scsi Output Control Latch
          璇ュ瘎瀛樺櫒鍙嶆槧鑺墖鎯宠椹卞姩鎴栦笌涔嬫瘮杈冪殑 SCSI 鎺у埗绾跨殑鐘舵€併€?

瀛楁 E锛歋BCL  Scsi Bus Control Lines
          SCSI 鎬荤嚎涓婃帶鍒剁嚎鐨勫疄闄呭€笺€?

瀛楁 F锛歋BDL  Scsi Bus Data Lines
          SCSI 鎬荤嚎涓婃暟鎹嚎鐨勫疄闄呭€笺€?

瀛楁 G锛歋XFER  SCSI Transfer
          鍖呭惈鐢ㄤ簬杈撳嚭鐨勫悓姝ュ懆鏈熻缃互鍙婂綋鍓嶅悓姝ュ亸绉伙紙鍋忕Щ 0 琛ㄧず寮傛锛夈€?

瀛楁 H锛歋CNTL3 Scsi Control Register 3
          鍖呭惈寮傛涓庡悓姝ユ暟鎹紶杈撶殑鏃跺簭璁剧疆鍊笺€?

鐞嗚В瀛楁 I銆丣銆並 涓庤浆鍌ㄩ渶瑕佸 SCSI 鏍囧噯銆佽姱鐗囨牳蹇冨姛鑳戒互鍙婇┍鍔ㄥ唴閮?
鏁版嵁缁撴瀯鏈夎壇濂界殑浜嗚В銆傞櫎闈炰綘鎯冲府蹇欑淮鎶ら┍鍔ㄤ唬鐮侊紝鍚﹀垯涓嶉渶瑕佽В鐮佸苟
鐞嗚В瀹冧滑銆?

## 16. 鍚屾浼犺緭鍗忓晢琛?


涓嬭〃鏄€氳繃璋冪敤椹卞姩鐢ㄤ簬鍚屾鍗忓晢鏃跺簭璁＄畻涓庤姱鐗囪缃殑渚嬬▼鍒涘缓鐨勩€?
绗竴寮犺〃瀵瑰簲浜庝娇鐢?80 MHz 鏃堕挓涓?5 涓椂閽熷垎棰戝櫒鐨?Ultra 鑺墖 53875
涓?53C860銆傜浜屽紶琛ㄦ槸閫氳繃灏?SCSI 鏃堕挓璁句负 40 MHz 骞朵娇鐢?4 涓椂閽熷垎棰戝櫒
璁＄畻鐨勶紝鍥犳閫傜敤浜庡揩閫?SCSI-2 妯″紡涓嬬殑鎵€鏈?NCR53C8XX 鑺墖銆?

鍛ㄦ湡浠ョ撼绉掍负鍗曚綅锛岄€熷害浠ュ厗浼犺緭/绉掍负鍗曚綅銆? 鍏嗕紶杈?绉掑湪 8 浣?SCSI 涓?
琛ㄧず 1 MB/s锛屽湪 Wide16 SCSI 涓嬭〃绀?2 MB/s銆?

16.1 53C895銆?3C875 涓?53C860 SCSI 鎺у埗鍣ㄧ殑鍚屾鏃跺簭

+-----------------------------+--------+-------+--------------+
|Negotiated                   |NCR settings    |              |
+-------+--------+------------+--------+-------+              |
|Factor |Period  |Speed       |Period  |Speed  |              |
+-------+--------+------------+--------+-------+--------------+
|10     | 25     |40.000      | 25     |40.000 | (53C895 only)|
+-------+--------+------------+--------+-------+--------------+
|11     | 30.2   |33.112      | 31.25  |32.000 | (53C895 only)|
+-------+--------+------------+--------+-------+--------------+
|12     | 50     |20.000      | 50     |20.000 |              |
+-------+--------+------------+--------+-------+--------------+
|13     | 52     |19.230      | 62     |16.000 |              |
+-------+--------+------------+--------+-------+--------------+
|14     | 56     |17.857      | 62     |16.000 |              |
+-------+--------+------------+--------+-------+--------------+
|15     | 60     |16.666      | 62     |16.000 |              |
+-------+--------+------------+--------+-------+--------------+
|16     | 64     |15.625      | 75     |13.333 |              |
+-------+--------+------------+--------+-------+--------------+
|17     | 68     |14.705      | 75     |13.333 |              |
+-------+--------+------------+--------+-------+--------------+
|18     | 72     |13.888      | 75     |13.333 |              |
+-------+--------+------------+--------+-------+--------------+
|19     | 76     |13.157      | 87     |11.428 |              |
+-------+--------+------------+--------+-------+--------------+
|20     | 80     |12.500      | 87     |11.428 |              |
+-------+--------+------------+--------+-------+--------------+
|21     | 84     |11.904      | 87     |11.428 |              |
+-------+--------+------------+--------+-------+--------------+
|22     | 88     |11.363      | 93     |10.666 |              |
+-------+--------+------------+--------+-------+--------------+
|23     | 92     |10.869      | 93     |10.666 |              |
+-------+--------+------------+--------+-------+--------------+
|24     | 96     |10.416      |100     |10.000 |              |
+-------+--------+------------+--------+-------+--------------+
|25     |100     |10.000      |100     |10.000 |              |
+-------+--------+------------+--------+-------+--------------+
|26     |104     | 9.615      |112     | 8.888 |              |
+-------+--------+------------+--------+-------+--------------+
|27     |108     | 9.259      |112     | 8.888 |              |
+-------+--------+------------+--------+-------+--------------+
|28     |112     | 8.928      |112     | 8.888 |              |
+-------+--------+------------+--------+-------+--------------+
|29     |116     | 8.620      |125     | 8.000 |              |
+-------+--------+------------+--------+-------+--------------+
|30     |120     | 8.333      |125     | 8.000 |              |
+-------+--------+------------+--------+-------+--------------+
|31     |124     | 8.064      |125     | 8.000 |              |
+-------+--------+------------+--------+-------+--------------+
|32     |128     | 7.812      |131     | 7.619 |              |
+-------+--------+------------+--------+-------+--------------+
|33     |132     | 7.575      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|34     |136     | 7.352      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|35     |140     | 7.142      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|36     |144     | 6.944      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|37     |148     | 6.756      |150     | 6.666 |              |
+-------+--------+------------+--------+-------+--------------+
|38     |152     | 6.578      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|39     |156     | 6.410      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|40     |160     | 6.250      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|41     |164     | 6.097      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|42     |168     | 5.952      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|43     |172     | 5.813      |175     | 5.714 |              |
+-------+--------+------------+--------+-------+--------------+
|44     |176     | 5.681      |187     | 5.333 |              |
+-------+--------+------------+--------+-------+--------------+
|45     |180     | 5.555      |187     | 5.333 |              |
+-------+--------+------------+--------+-------+--------------+
|46     |184     | 5.434      |187     | 5.333 |              |
+-------+--------+------------+--------+-------+--------------+
|47     |188     | 5.319      |200     | 5.000 |              |
+-------+--------+------------+--------+-------+--------------+
|48     |192     | 5.208      |200     | 5.000 |              |
+-------+--------+------------+--------+-------+--------------+
|49     |196     | 5.102      |200     | 5.000 |              |
+-------+--------+------------+--------+-------+--------------+

16.2 蹇€?SCSI-2 53C8XX 鎺у埗鍣ㄧ殑鍚屾鏃跺簭

+-----------------------------+----------------+
|Negotiated                   |NCR settings    |
+-------+--------+------------+--------+-------+
|Factor |Period  |Speed       |Period  |Speed  |
+-------+--------+------------+--------+-------+
|25     |100     |10.000      |100     |10.000 |
+-------+--------+------------+--------+-------+
|26     |104     |9.615       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|27     |108     |9.259       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|28     |112     |8.928       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|29     |116     |8.620       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|30     |120     |8.333       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|31     |124     |8.064       |125     | 8.000 |
+-------+--------+------------+--------+-------+
|32     |128     |7.812       |131     | 7.619 |
+-------+--------+------------+--------+-------+
|33     |132     |7.575       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|34     |136     |7.352       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|35     |140     |7.142       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|36     |144     |6.944       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|37     |148     |6.756       |150     | 6.666 |
+-------+--------+------------+--------+-------+
|38     |152     |6.578       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|39     |156     |6.410       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|40     |160     |6.250       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|41     |164     |6.097       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|42     |168     |5.952       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|43     |172     |5.813       |175     | 5.714 |
+-------+--------+------------+--------+-------+
|44     |176     |5.681       |187     | 5.333 |
+-------+--------+------------+--------+-------+
|45     |180     |5.555       |187     | 5.333 |
+-------+--------+------------+--------+-------+
|46     |184     |5.434       |187     | 5.333 |
+-------+--------+------------+--------+-------+
|47     |188     |5.319       |200     | 5.000 |
+-------+--------+------------+--------+-------+
|48     |192     |5.208       |200     | 5.000 |
+-------+--------+------------+--------+-------+
|49     |196     |5.102       |200     | 5.000 |
+-------+--------+------------+--------+-------+

## 17. 涓茶 NVRAM


锛堢敱 Richard Waltham 娣诲姞锛歞ormouse@farsrobt.demon.co.uk锛?

### 17.1 鐗规€?


鍚敤涓茶 NVRAM 鏀寔鍚庯紝鍙互妫€娴?Symbios 浠ュ強閮ㄥ垎 Symbios 鍏煎涓绘満
閫傞厤鍣紙杩樻湁 Tekram 鏉垮崱锛変笂鍖呭惈鐨勪覆琛?NVRAM銆備覆琛?NVRAM 琚?Symbios
涓?Tekram 鐢ㄦ潵淇濆瓨涓绘満閫傞厤鍣ㄥ強鍏舵墍杩為┍鍔ㄥ櫒鐨勮缃弬鏁般€?

Symbios NVRAM 杩樹繚瀛樹簡鎷ユ湁澶氫釜涓绘満閫傞厤鍣ㄧ殑绯荤粺涓富鏈洪€傞厤鍣ㄧ殑鍚姩
椤哄簭鏁版嵁銆傝繖鏍峰彲浠ユ敼鍙樻娴嬩富鏈洪€傞厤鍣ㄦ椂鎵弿鍚勫崱浠ュ鎵鹃┍鍔ㄥ櫒鐨勯『搴忋€?

鐩墠锛屼娇鐢?鍙嶅悜鎺㈡祴"鍙兘鍦ㄤ竴瀹氱▼搴︿笂鍋氬埌杩欎竴鐐癸紝鑰屼笖瀹冨彧浼氭敼鍙?
涓嶅悓绫诲瀷鍗＄殑妫€娴嬮『搴忋€?NVRAM 鍚姩椤哄簭"璁剧疆鏃㈣兘鍋氬埌杩欎竴鐐癸紝涔熻兘鏀瑰彉
鍚岀被鍗＄殑鎵弿椤哄簭锛岃繖鏄?鍙嶅悜鎺㈡祴"鍋氫笉鍒扮殑銆?

浣跨敤 Symbios 鑺墖鐨?Tekram 鏉垮崱锛圖C390W/F/U锛夊甫鏈?NVRAM锛屼細琚娴嬪嚭鏉ワ紝
骞剁敤浜庡尯鍒?Symbios 鍏煎涓?Tekram 涓绘満閫傞厤鍣ㄣ€傚鏋滆缃簡
CONFIG_SCSI_53C8XX_SYMBIOS_COMPAT 閰嶇疆鍙傛暟锛岃繖鐢ㄤ簬绂佺敤鍦?Tekram 鏉垮崱涓?
閿欒璁剧疆鐨?Symbios 鍏煎"宸垎"锛坉iff锛夎缃紝浠庤€岃 Symbios 鍗′笌 Tekram
鍗″彲浠ヤ竴璧蜂娇鐢紝Symbios 鍗′娇鐢ㄥ叾鍏ㄩ儴鐗规€э紝鍖呮嫭"宸垎"鏀寔銆傦紙瀵?Symbios
鍏煎鍗′娇鐢?LED 寮曡剼"锛坙ed pin锛夋敮鎸佸彲浠ヤ繚鎸佸惎鐢ㄣ€傚畠瀵?Tekram 涓绘満
閫傞厤鍣ㄦ病鏈夊疄闄呯敤澶勶紝浣嗕篃涓嶄細閫犳垚闂銆傦級

### 17.2 Symbios NVRAM 甯冨眬


```

    00 00
    64 01
    8e 0b

    00 30 00 00 00 00 07 00 00 00 00 00 00 00 07 04 10 04 00 00

    04 00 0f 00 00 10 00 50 00 00 01 00 00 62
    04 00 03 00 00 10 00 58 00 00 01 00 00 63
    04 00 01 00 00 10 00 48 00 00 01 00 00 61
    00 00 00 00 00 00 00 00 00 00 00 00 00 00

    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00

    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00

    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00

    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00

    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00
    00 00 00 00 00 00 00 00

    fe fe
    00 00
    00 00

```
NVRAM 甯冨眬璇︽儏

=============  ================
NVRAM Address
=============  ================
0x000-0x0ff    not used
0x100-0x26f    initialised data
0x270-0x7ff    not used
=============  ================

```

        header  -   6 bytes,
        data    - 356 bytes (checksum is byte sum of this data)
        trailer -   6 bytes
                  ---
        total     368 bytes

```

        controller set up  -  20 bytes
        boot configuration -  56 bytes (4x14 bytes)
        device set up      - 128 bytes (16x8 bytes)
        unused (spare锛?   - 152 bytes (19x8 bytes)
                             ---
        total                356 bytes

```

    00 00   - ?? start marker
    64 01   - byte count (lsb/msb excludes header/trailer)
    8e 0b   - checksum (lsb/msb excludes header/trailer)

```

    00 30 00 00 00 00 07 00 00 00 00 00 00 00 07 04 10 04 00 00
		    |     |           |     |
		    |     |           |      -- host ID
		    |     |           |
		    |     |            --Removable Media Support
		    |     |               0x00 = none
		    |     |               0x01 = Bootable Device
		    |     |               0x02 = All with Media
		    |     |
		    |      --flag bits 2
		    |        0x00000001= scan order hi->low
		    |            (default 0x00 - scan low->hi)
			--flag bits 1
			0x00000001 scam enable
			0x00000010 parity enable
			0x00000100 verbose boot msgs

```
鍓╀綑瀛楄妭鏈煡鈥斺€斿湪鎴戠殑褰撳墠璁剧疆涓紝瀵逛簬浠讳綍鎺у埗鍣ㄥ畠浠技涔庨兘涓嶄細鏀瑰彉銆?

53c810a 涓?53c875 NVRAM 鐨勯粯璁よ缃浉鍚?
锛堝彲绉诲姩浠嬭川鑷?Symbios BIOS 4.09 鐗堣捣娣诲姞锛?

鍚姩閰嶇疆

```

    04 00 0f 00 00 10 00 50 00 00 01 00 00 62 -- 1st controller
    04 00 03 00 00 10 00 58 00 00 01 00 00 63    2nd controller
    04 00 01 00 00 10 00 48 00 00 01 00 00 61    3rd controller
    00 00 00 00 00 00 00 00 00 00 00 00 00 00    4th controller
	|  |  |  |     |        |     |  |
	|  |  |  |     |        |      ---- PCI io port adr
	|  |  |  |     |         --0x01 init/scan at boot time
	|  |  |  |      --PCI device/function number (0xdddddfff)
	|  |   ----- ?? PCI vendor ID (lsb/msb)
	    ----PCI device ID (lsb/msb)

    ?? use of this data is a guess but seems reasonable

```
鍓╀綑瀛楄妭鏈煡鈥斺€斿湪鎴戠殑褰撳墠璁剧疆涓畠浠技涔庝笉浼氭敼鍙?

### default set up is identical for 53c810a and 53c875 NVRAM


```

    0f 00 08 08 64 00 0a 00 - id 0
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00

    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00
    0f 00 08 08 64 00 0a 00 - id 15
    |     |  |  |     |  |
    |     |  |  |      ----timeout (lsb/msb)
    |     |  |   --synch period (0x?? 40 Mtrans/sec- fast 40) (probably 0x28)
    |     |  |                  (0x30 20 Mtrans/sec- fast 20)
    |     |  |                  (0x64 10 Mtrans/sec- fast )
    |     |  |                  (0xc8  5 Mtrans/sec)
    |     |  |                  (0x00  asynchronous)
    |     |   -- ?? max sync offset (0x08 in NVRAM on 53c810a)
    |     |                         (0x10 in NVRAM on 53c875)
    |      --device bus width (0x08 narrow)
    |                         (0x10 16 bit wide)
    --flag bits
	0x00000001 - disconnect enabled
	0x00000010 - scan at boot time
	0x00000100 - scan luns
	0x00001000 - queue tags enabled

```
鍓╀綑瀛楄妭鏈煡鈥斺€斿湪鎴戠殑褰撳墠璁剧疆涓畠浠技涔庝笉浼氭敼鍙?

?? 姝ゆ暟鎹殑鐢ㄩ€旀槸鐚滄祴锛屼絾浼间箮鍚堢悊
锛堜絾瀹冨彲鑳芥槸鏈€澶ф€荤嚎瀹藉害锛?

53c810a NVRAM 鐨勯粯璁よ缃?
53c875 NVRAM 鐨勯粯璁よ缃?

```
    - bus width     - 0x10
                                - sync offset ? - 0x10
                                - sync period   - 0x30

?? spare device space (32 bit bus ?锛?

```

    00 00 00 00 00 00 00 00  (19x8bytes)
    .
    .
    00 00 00 00 00 00 00 00

```

### default set up is identical for 53c810a and 53c875 NVRAM


```

    fe fe   - ? end marker ?
    00 00
    00 00

```

### default set up is identical for 53c810a and 53c875 NVRAM


### 17.3 Tekram NVRAM 甯冨眬


nvram 64x16 (1024 bit)

```

    Drive ID 0-15 (addr 0x0yyyy0 = device setup, yyyy = ID)
		(addr 0x0yyyy1 = 0x0000)

	x x x x  x x x x  x x x x  x x x x
		| | |      | |  | | | |
		| | |      | |  | | |  ----- parity check   0 - off
		| | |      | |  | | |                       1 - on
		| | |      | |  | | |
		| | |      | |  | |  ------- sync neg       0 - off
		| | |      | |  | |                         1 - on
		| | |      | |  | |
		| | |      | |  |  --------- disconnect     0 - off
		| | |      | |  |                           1 - on
		| | |      | |  |
		| | |      | |   ----------- start cmd      0 - off
		| | |      | |                              1 - on
		| | |      | |
		| | |      |  -------------- tagged cmds    0 - off
		| | |      |                                1 - on
		| | |      |
		| | |       ---------------- wide neg       0 - off
		| | |                                       1 - on
		| | |
		    --------------------------- sync rate      0 - 10.0 Mtrans/sec
							    1 -  8.0
							    2 -  6.6
							    3 -  5.7
							    4 -  5.0
							    5 -  4.0
							    6 -  3.0
							    7 -  2.0
							    7 -  2.0
							    8 - 20.0
							    9 - 16.7
							    a - 13.9
							    b - 11.9

```
鍏ㄥ眬璁剧疆

```

    x x x x  x x x x  x x x x  x x x x
    | | | |  | | | |           | | | |
    | | | |  | | | |            ----------- host ID    0x00 - 0x0f
    | | | |  | | | |
    | | | |  | | |  ----------------------- support for    0 - off
    | | | |  | | |                          > 2 drives     1 - on
    | | | |  | | |
    | | | |  | |  ------------------------- support drives 0 - off
    | | | |  | |                            > 1Gbytes      1 - on
    | | | |  | |
    | | | |  |  --------------------------- bus reset on   0 - off
    | | | |  |                                power on     1 - on
    | | | |  |
    | | | |   ----------------------------- active neg     0 - off
    | | | |                                                1 - on
    | | | |
    | | |  -------------------------------- imm seek       0 - off
    | | |                                                  1 - on
    | | |
    | |  ---------------------------------- scan luns      0 - off
    | |                                                    1 - on
    | |
     -------------------------------------- removable      0 - disable
                                            as BIOS dev    1 - boot device
                                                           2 - all

```

```

    x x x x  x x x x  x x x x  x x x x
               | | |             | | |
               | | |              --------- boot delay     0 -   3 sec
               | | |                                       1 -   5
               | | |                                       2 -  10
               | | |                                       3 -  20
               | | |                                       4 -  30
               | | |                                       5 -  60
               | | |                                       6 - 120
               | | |
                --------------------------- max tag cmds   0 -  2
                                                           1 -  4
                                                           2 -  8
                                                           3 - 16
                                                           4 - 32

```

```

    x x x x  x x x x  x x x x  x x x x
                                     |
                                      ----- F2/F6 enable   0 - off ???
                                                           1 - on  ???

```
鏍￠獙鍜岋紙鍦板潃 0x111111锛?

鏍￠獙鍜?= 0x1234 - (鍦板潃 0-63 鐨勬眰鍜?

----------------------------------------------------------------------------

```

    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000
    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000
    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000
    0x0037 0x0000 0x0037 0x0000 0x0037 0x0000 0x0037 0x0000

    0x0f07 0x0400 0x0001 0x0000 0x0000 0x0000 0x0000 0x0000
    0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000
    0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000
    0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0x0000 0xfbbc


```
## 18. 澶х鏀寔


PCI 灞€閮ㄦ€荤嚎涓昏鏄负 x86 浣撶郴缁撴瀯璁捐銆傚洜姝わ紝PCI 璁惧閫氬父鏈熸湜浣跨敤
灏忕锛坙ittle endian锛夊瓧鑺傚簭鐨?DWORD銆?

### 18.1 澶х CPU


涓轰簡鍦ㄥぇ绔紙Big Endian锛変綋绯荤粨鏋勪笂鏀寔 NCR 鑺墖锛岄┍鍔ㄥ繀椤诲湪姣忔
闇€瑕佹椂鎵ц瀛楄妭閲嶆帓銆傛鐗规€х敱 Cort <cort@cs.nmt.edu> 娣诲姞鍒伴┍鍔ㄤ腑锛?
鍦ㄩ┍鍔ㄧ増鏈?2.5 鍙婃洿楂樼増鏈腑鍙敤銆傜洰鍓嶅ぇ绔敮鎸佷粎鍦?Linux/PPC
锛圥owerPC锛変笂娴嬭瘯杩囥€?

### 18.2 杩愯浜庡ぇ绔ā寮忕殑 NCR 鑺墖


鍦?SYMBIOS 鏂囨。涓彲浠ョ湅鍒帮紝鏌愪簺鑺墖鏀寔涓€绉嶇壒娈婄殑澶х妯″紡锛岀悊璁轰笂
鍖呮嫭锛?3C815銆?3C825A銆?3C875銆?3C875N銆?3C895銆傛宸ヤ綔妯″紡涓嶆槸鐢?
杞欢閫夋嫨鐨勶紝鑰屾槸闇€瑕佸皢鍚嶄负 BigLit 鐨勫紩鑴氫笂鎷夈€備娇鐢ㄦ妯″紡锛屽綋椹卞姩杩愯
浜庡ぇ绔?CPU 涓婃椂锛屽簲褰撳彲浠ラ伩鍏嶅ぇ閮ㄥ垎瀛楄妭閲嶆帓銆傞┍鍔ㄧ増鏈?2.5 鍦ㄧ悊璁轰笂
涔熷凡涓烘鐗规€у仛濂藉噯澶囥€?
