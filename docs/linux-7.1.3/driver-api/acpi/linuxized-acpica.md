
## Linuxized ACPICA 鈥斺€?ACPICA 鍙戝竷鑷姩鍖栫畝浠?
:Copyright: |copy| 2013-2016, Intel Corporation

:Author: Lv Zheng <lv.zheng@intel.com>


## Abstract锛堟憳瑕侊級

鏈枃妗ｆ弿杩颁簡 ACPICA 椤圭洰浠ュ強 ACPICA 涓?Linux 涔嬮棿鐨勫叧绯汇€傚畠涔熸弿杩颁簡 drivers/acpi/acpica銆乮nclude/acpi 鍜?tools/power/acpi 涓殑 ACPICA 浠ｇ爜鏄浣曡鑷姩鏇存柊浠ヨ窡闅忎笂娓哥殑銆?
## ACPICA Project锛圓CPICA 椤圭洰锛?
ACPI 缁勪欢鏋舵瀯锛圓dvanced Configuration and Power Interface Specification锛孉CPICA锛夐」鐩彁渚涗簡涓€涓搷浣滅郴缁燂紙OS锛夋棤鍏崇殑銆佸叧浜庨珮绾ч厤缃笌鐢垫簮鎺ュ彛瑙勮寖锛圓CPI锛夌殑鍙傝€冨疄鐜般€傚畠宸茶鍚勭瀹夸富鎿嶄綔绯荤粺鎵€閲囩撼銆傞€氳繃鐩存帴闆嗘垚 ACPICA锛孡inux 涔熻兘浠?ACPICA 鍦ㄥ叾浠栧涓绘搷浣滅郴缁熶笂鐨勫簲鐢ㄧ粡楠屼腑鍙楃泭銆?
ACPICA 椤圭洰鐨勪富椤垫槸锛歸ww.acpica.org锛屽畠鐢?Intel Corporation 缁存姢鍜屾敮鎸併€?
涓嬪浘鎻忕粯浜?Linux ACPI 瀛愮郴缁燂紝鍏朵腑 ACPICA
```

      +---------------------------------------------------------+
      |                                                         |
      |   +---------------------------------------------------+ |
      |   | +------------------+                              | |
      |   | | Table Management |                              | |
      |   | +------------------+                              | |
      |   | +----------------------+                          | |
      |   | | Namespace Management |                          | |
      |   | +----------------------+                          | |
      |   | +------------------+       ACPICA Components      | |
      |   | | Event Management |                              | |
      |   | +------------------+                              | |
      |   | +---------------------+                           | |
      |   | | Resource Management |                           | |
      |   | +---------------------+                           | |
      |   | +---------------------+                           | |
      |   | | Hardware Management |                           | |
      |   | +---------------------+                           | |
      |   +---------------------------------------------------+ | |
      | | |                            +------------------+ | | |
      | | |                            | OS Service Layer | | | |
      | | |                            +------------------+ | | |
      | | +-------------------------------------------------|-+ |
      | |   +--------------------+                          |   |
      | |   | Device Enumeration |                          |   |
      | |   +--------------------+                          |   |
      | |   +------------------+                            |   |
      | |   | Power Management |                            |   |
      | |   +------------------+     Linux/ACPI Components  |   |
      | |   +--------------------+                          |   |
      | |   | Thermal Management |                          |   |
      | |   +--------------------+                          |   |
      | |   +--------------------------+                    |   |
      | |   | Drivers for ACPI Devices |                    |   |
      | |   +--------------------------+                    |   |
      | |   +--------+                                      |   |
      | |   | ...... |                                      |   |
      | |   +--------+                                      |   |
      | +---------------------------------------------------+   |
      |                                                         |
      +---------------------------------------------------------+

                 Figure 1. Linux ACPI Software Components

```
    A. OS Service Layer 鈥斺€?鐢?Linux 鎻愪緵锛岀敤浜庣粰鍑洪瀹氫箟 ACPICA 鎺ュ彛锛坅cpi_os_*锛夌殑 OS 鐩稿叧瀹炵幇銆?```
         include/acpi/acpiosxf.h
         drivers/acpi/osl.c
         include/acpi/platform
         include/asm/acenv.h
    B. ACPICA Functionality 鈥斺€?浠?ACPICA 浠ｇ爜搴撳彂甯冿紝鐢ㄤ簬缁欏嚭 ACPICA 鎺ュ彛锛坅cpi_*锛夌殑 OS 鏃犲叧瀹炵幇銆?       ::

         drivers/acpi/acpica
         include/acpi/ac*.h
         tools/power/acpi
    C. Linux/ACPI Functionality 鈥斺€?鍚戝叾浠?Linux 鍐呮牳瀛愮郴缁熶互鍙婄敤鎴风┖闂寸▼搴忔彁渚?Linux 鐗瑰畾鐨?ACPI 鍔熻兘銆?       ::

         drivers/acpi
         include/linux/acpi.h
         include/linux/acpi*.h
         include/acpi
         tools/power/acpi
    D. Architecture Specific ACPICA/ACPI Functionalities 鈥斺€?鐢?ACPI 瀛愮郴缁熸彁渚涳紝鐢ㄤ簬缁欏嚭 ACPI 鎺ュ彛鐨勬灦鏋勭浉鍏冲疄鐜般€傚畠浠槸 Linux 鐗瑰畾鐨勭粍浠讹紝涓嶅湪鏈枃妗ｈ寖鍥村唴銆?       ::

         include/asm/acpi.h
         include/asm/acpi*.h
         arch/*/acpi

```
## ACPICA Release锛圓CPICA 鍙戝竷锛?
ACPICA 椤圭洰鍦ㄥ叾浠ヤ笅浠撳簱 URL 缁存姢浠ｇ爜搴擄細https://github.com/acpica/acpica.git銆傛寜鎯緥锛屾瘡鏈堝彂甯冧竴娆°€?
鐢变簬 ACPICA 椤圭洰鎵€閲囩敤鐨勭紪鐮侀鏍间笉琚?Linux 鎺ュ彈锛屽洜姝ゅ瓨鍦ㄤ竴濂楀彂甯冩祦绋嬶紝灏?ACPICA 鐨?git 鎻愪氦杞崲涓?Linux 琛ヤ竵銆傝娴佺▼鐢熸垚鐨勮ˉ涓佽绉颁负 鈥渓inuxized ACPICA patches鈥濓紙Linux 鍖栫殑 ACPICA 琛ヤ竵锛夈€傝鍙戝竷娴佺▼鍦?ACPICA git 浠撳簱鐨勪竴浠芥湰鍦板壇鏈笂杩涜銆傛瘡鏈堝彂甯冧腑鐨勬瘡涓彁浜ら兘琚浆鎹负涓€涓?linuxized ACPICA 琛ヤ竵銆傚畠浠叡鍚屾瀯鎴愪簡闈㈠悜 Linux ACPI 绀惧尯鐨勬瘡鏈?ACPICA 鍙戝竷琛ヤ竵闆嗐€傛娴佺▼
```

    +-----------------------------+
    | acpica / master (-) commits |
    +-----------------------------+
       /|\         |
        |         \|/
        |  /---------------------\    +----------------------+
        | < Linuxize repo Utility >-->| old linuxized acpica |--+
        |  \---------------------/    +----------------------+  |
        |                                                       |
     /---------\                                                |
    < git reset >                                                \
     \---------/                                                  \
       /|\                                                        /+-+
        |                                                        /   |
    +-----------------------------+                             |    |
    | acpica / master (+) commits |                             |    |
    +-----------------------------+                             |    |
                   |                                            |    |
                  \|/                                           |    |
         /-----------------------\    +----------------------+  |    |
        < Linuxize repo Utilities >-->| new linuxized acpica |--+    |
         \-----------------------/    +----------------------+       |
                                                                    \|/
    +--------------------------+                  /----------------------\
    | Linuxized ACPICA Patches |<----------------< Linuxize patch Utility >
    +--------------------------+                  \----------------------/
                   |
                  \|/
     /---------------------------\
    < Linux ACPI Community Review >
     \---------------------------/
                   |
                  \|/
    +-----------------------+    /------------------\    +----------------+
    | linux-pm / linux-next |-->< Linux Merge Window >-->| linux / master |
    +-----------------------+    \------------------/    +----------------+

                Figure 2. ACPICA -> Linux Upstream Process

```
    A. Linuxize Utilities 鈥斺€?鐢?ACPICA 浠撳簱鎻愪緵锛屽寘鎷綅浜?source/tools/acpisrc 鏂囦欢澶逛腑鐨勪竴涓疄鐢ㄧ▼搴忥紝浠ュ強浣嶄簬 generate/linux 鏂囦欢澶逛腑鐨勮嫢骞茶剼鏈€?    B. acpica / master 鈥斺€?浣嶄簬 <https://github.com/acpica/acpica.git> 鐨?git 浠撳簱鐨?鈥渕aster鈥?鍒嗘敮銆?    C. linux-pm / linux-next 鈥斺€?浣嶄簬 <https://git.kernel.org/pub/scm/linux/kernel/git/rafael/linux-pm.git> 鐨?git 浠撳簱鐨?鈥渓inux-next鈥?鍒嗘敮銆?    D. linux / master 鈥斺€?浣嶄簬 <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git> 鐨?git 浠撳簱鐨?鈥渕aster鈥?鍒嗘敮銆?
   鍦?linuxized ACPICA 琛ヤ竵琚彂閫佺粰 Linux ACPI 绀惧尯瀹℃煡涔嬪墠锛屾湁涓€涓川閲忎繚璇佺殑鏋勫缓娴嬭瘯娴佺▼锛岀敤浠ュ噺灏戠Щ妞嶉棶棰樸€傜洰鍓嶆鏋勫缓娴佺▼鍙収椤句互涓嬪唴鏍搁厤缃€夐」锛?   CONFIG_ACPI/CONFIG_ACPI_DEBUG/CONFIG_ACPI_DEBUGGER

## ACPICA Divergences锛圓CPICA 鍒嗘锛?
鐞嗘兂鎯呭喌涓嬶紝鎵€鏈?ACPICA 鎻愪氦閮藉簲褰撹鑷姩杞崲涓?Linux 琛ヤ竵鑰屾棤闇€鎵嬪姩淇敼锛屸€渓inux / master鈥?鏍戝簲褰撳寘鍚笌 鈥渘ew linuxized acpica鈥?鏍戜腑鎵€鍚?ACPICA 浠ｇ爜绮剧‘瀵瑰簲鐨?ACPICA 浠ｇ爜锛屽苟涓斿簲褰撳彲浠ュ畬鍏ㄨ嚜鍔ㄥ湴杩愯鍙戝竷娴佺▼銆?
鐒惰€岋紝浜嬪疄涓婏紝Linux 涓殑 ACPICA 浠ｇ爜涓庝笂娓?ACPICA 浠ｇ爜涔嬮棿瀛樺湪婧愪唬鐮佸樊寮傦紝杩欒绉颁负 鈥淎CPICA Divergences鈥濓紙ACPICA 鍒嗘锛夈€?
ACPICA 鍒嗘鐨勫悇绉嶆潵婧愬寘鎷細
   1. 閬楃暀鍒嗘锛圠egacy divergences锛夆€斺€?鍦ㄥ綋鍓嶇殑 ACPICA 鍙戝竷娴佺▼寤虹珛涔嬪墠锛孡inux 涓?ACPICA 涔嬮棿灏卞凡缁忓瓨鍦ㄥ垎姝с€傝繃鍘诲嚑骞翠腑杩欎簺鍒嗘宸茶澶у箙鍑忓皯锛屼絾浠嶆湁鑻ュ共瀛樺湪锛屽苟涓旈渶瑕佹椂闂存潵鎵惧嚭瀹冧滑瀛樺湪鑳屽悗鐨勬牴鏈師鍥犮€?   2. 鎵嬪姩淇敼锛圡anual modifications锛夆€斺€?浠讳綍鐩存帴鍦?Linux 婧愮爜涓仛鐨勬墜鍔ㄤ慨鏀癸紙渚嬪缂栫爜椋庢牸淇锛夋樉鐒朵細鎹熷 ACPICA 鍙戝竷鑷姩鍖栥€傚洜姝ゅ缓璁湪涓婃父 ACPICA 婧愮爜涓慨澶嶆绫婚棶棰橈紝骞朵娇鐢?ACPICA 鍙戝竷瀹炵敤绋嬪簭鐢熸垚 linuxized 淇锛堣瑙佷笅鏂囩 4 鑺傦級銆?   3. Linux 鐗瑰畾鍔熻兘锛圠inux specific features锛夆€斺€?鏈夋椂鏃犳硶浣跨敤褰撳墠鐨?ACPICA API 鏉ュ疄鐜?Linux 鍐呮牳鎵€闇€鐨勫姛鑳斤紝鍥犳 Linux 寮€鍙戣€呭伓灏斾笉寰椾笉鐩存帴淇敼 ACPICA 浠ｇ爜銆傝繖浜涗慨鏀瑰彲鑳戒笉琚笂娓?ACPICA 鎺ュ彈锛屽湪杩欑鎯呭喌涓嬶紝闄ら潪 ACPICA 涓€鏂硅兘澶熷疄鐜版柊鐨勬満鍒舵潵鏇夸唬瀹冧滑锛屽惁鍒欏畠浠細浣滀负宸叉彁浜ょ殑 ACPICA 鍒嗘淇濈暀涓嬫潵銆?   4. ACPICA 鍙戝竷淇锛圓CPICA release fixups锛夆€斺€?ACPICA 鍙娇鐢ㄤ竴缁勭敤鎴风┖闂存ā鎷熷疄鐢ㄧ▼搴忔潵娴嬭瘯鎻愪氦锛屽洜姝?linuxized ACPICA 琛ヤ竵鍙兘浼氱牬鍧?Linux 鍐呮牳锛岀粰鎴戜滑鐣欎笅鏋勫缓/鍚姩澶辫触銆備负浜嗛伩鍏嶇牬鍧?Linux 鐨勪簩鍒嗭紙bisection锛夛紝鍦ㄥ彂甯冩祦绋嬩腑浼氬皢淇鐩存帴搴旂敤鍒?linuxized ACPICA 琛ヤ竵涓娿€傚綋杩欎簺鍙戝竷淇琚弽鍚戠Щ妞嶅埌涓婃父 ACPICA 婧愮爜鏃讹紝瀹冧滑蹇呴』閬靛惊涓婃父 ACPICA 鐨勮鍒欙紝鍥犳鍙兘浼氬嚭鐜拌繘涓€姝ョ殑淇敼銆傝繖鍙兘瀵艰嚧鏂板垎姝х殑鍑虹幇銆?   5. ACPICA 鎻愪氦蹇€熻窡韪紙Fast tracking of ACPICA commits锛夆€斺€?鏌愪簺 ACPICA 鎻愪氦鏄洖褰掍慨澶嶆垨绋冲畾鍊欓€夋潗鏂欙紝鍥犳浼氱浉瀵逛簬 ACPICA 鍙戝竷娴佺▼鑰屾彁鍓嶅簲鐢ㄣ€傚鏋滄绫绘彁浜ゅ湪 ACPICA 涓€鏂硅鍥為€€鎴栧彉鍩猴紝浠ユ彁渚涙洿浼樼殑瑙ｅ喅鏂规锛屽氨浼氱敓鎴愭柊鐨?ACPICA 鍒嗘銆?
## ACPICA Development锛圓CPICA 寮€鍙戯級

鏈寮曞 Linux 寮€鍙戣€呬娇鐢?ACPICA 涓婃父鍙戝竷瀹炵敤绋嬪簭锛屽湪瀹冧滑浠?ACPICA 鍙戝竷娴佺▼鍙敤涔嬪墠锛岃幏鍙栧搴斾簬涓婃父 ACPICA 鎻愪氦鐨?Linux 琛ヤ竵銆?
   1. Cherry-pick 涓€涓?ACPICA 鎻愪氦

   棣栧厛鎮ㄩ渶瑕?git clone 璇?ACPICA 浠撳簱锛屽苟涓旀偍鎯宠 cherry-pick 鐨?ACPICA 淇敼蹇呴』宸叉彁浜ゅ埌鏈湴浠撳簱銆?
   鐒跺悗 gen-patch.sh 鍛戒护鍙互甯姪 cherry-pick 涓€涓?ACPICA 鎻愪氦
```

   $ git clone https://github.com/acpica/acpica
   $ cd acpica
   $ generate/linux/gen-patch.sh -u [commit ID]

   杩欓噷鐨?commit ID 鏄偍鎯宠 cherry-pick 鐨?ACPICA 鏈湴浠撳簱鎻愪氦 ID銆傚鏋滄彁浜ゆ槸 鈥淗EAD鈥濓紝鍒欏彲浠ョ渷鐣ャ€?
   2. Cherry-pick 鏈€杩戠殑 ACPICA 鎻愪氦

   鏈夋椂鎮ㄩ渶瑕佸皢浠ｇ爜鍙樺熀鍒板皻鏈簲鐢ㄥ埌 Linux 鐨勬渶鏂?ACPICA 淇敼涔嬩笂銆?
   鎮ㄥ彲浠ヨ嚜宸辩敓鎴?ACPICA 鍙戝竷绯诲垪锛屽苟灏嗕唬鐮佸彉鍩哄埌鐢熸垚鐨?ACPICA 鍙戝竷琛ヤ竵涔嬩笂锛氾細

   $ git clone https://github.com/acpica/acpica
   $ cd acpica
   $ generate/linux/make-patches.sh -u [commit ID]

   璇?commit ID 搴斿綋鏄?Linux 鎺ュ彈鐨勬渶鍚庝竴涓?ACPICA 鎻愪氦銆傞€氬父锛屽畠鏄慨鏀?ACPI_CA_VERSION 鐨勬彁浜ゃ€傚畠鍙互閫氳繃鎵ц "git blame source/include/acpixf.h" 骞跺弬鑰冨寘鍚?"ACPI_CA_VERSION" 鐨勯偅涓€琛屾潵鎵惧埌銆?
   3. 妫€鏌ュ綋鍓嶇殑鍒嗘

   濡傛灉鎮ㄥ悓鏃舵嫢鏈?Linux 鍜屼笂娓?ACPICA 鐨勬湰鍦板壇鏈紝鎮ㄥ彲浠ョ敓鎴愪竴涓?diff 鏂囦欢锛屾寚绀哄綋鍓嶅垎姝х殑鐘舵€侊細锛?
   # git clone https://github.com/acpica/acpica
   # git clone https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git
   # cd acpica
   # generate/linux/divergence.sh -s ../linux

```
