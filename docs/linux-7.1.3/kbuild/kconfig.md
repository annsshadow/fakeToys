## Configuration targets and editors


鏈枃妗ｆ彁渚涗竴浜涗娇鐢?`make *config` 鐨勫府鍔┿€?

浣跨敤 `make help` 鍒楀嚭鎵€鏈夊彲鑳界殑閰嶇疆鐩爣銆?

xconfig锛?qconf'锛夈€乵enuconfig锛?mconf'锛夊拰 nconfig锛?nconf'锛夌▼搴忎篃鍐呭祵浜嗗府鍔╂枃鏈€?
璇峰姟蹇呮煡鐪嬭繖浜涘叧浜庡鑸€佹悳绱互鍙婂叾浠栭€氱敤甯姪鐨勬枃鏈唴瀹广€?

gconfig锛?gconf'锛夌▼搴忕殑甯姪鏂囨湰鏈夐檺銆?


## General


鏂扮殑鍐呮牳鐗堟湰閫氬父浼氬紩鍏ユ柊鐨勯厤缃鍙枫€傚線寰€鏇撮噸瑕佺殑鏄紝鏂扮殑鍐呮牳鐗堟湰鍙兘浼氶噸鍛藉悕閰嶇疆绗﹀彿銆?
鍙戠敓杩欑鎯呭喌鏃讹紝浣跨敤涔嬪墠鍙敤鐨?.config 鏂囦欢骞惰繍琛屸€渕ake oldconfig鈥濅笉涓€瀹氳兘涓轰綘鐢熸垚涓€涓彲鐢ㄧ殑
鏂板唴鏍革紝鍥犳浣犲彲鑳戒細鍙戠幇闇€瑕佹煡鐪嬪紩鍏ヤ簡鍝簺鏂扮殑鍐呮牳绗﹀彿銆?

```

    cp user/some/old.config .config
    make listnewconfig

```
閰嶇疆绋嬪簭浼氶€愯鍒楀嚭鎵€鏈夋柊鐨勭鍙枫€?

```

    make oldconfig
    scripts/diffconfig .config.old .config | less


```
## Environment variables


`*config` 鐨勭幆澧冨彉閲忥細

`KCONFIG_CONFIG`
    璇ョ幆澧冨彉閲忓彲鐢ㄤ簬鎸囧畾涓€涓粯璁ょ殑鍐呮牳閰嶇疆鏂囦欢鍚嶏紝浠ヨ鐩栭粯璁ょ殑鈥?config鈥濆悕绉般€?

`KCONFIG_DEFCONFIG_LIST`
    璇ョ幆澧冨彉閲忔寚瀹氫竴涓厤缃枃浠跺垪琛紝鍦?.config 灏氫笉瀛樺湪鏃跺彲鐢ㄤ綔鍩虹閰嶇疆銆傚垪琛ㄤ腑鐨勬潯鐩互绌虹櫧
    瀛楃鐩镐簰鍒嗛殧锛屼娇鐢ㄧ涓€涓瓨鍦ㄧ殑鏉＄洰銆?

`KCONFIG_OVERWRITECONFIG`
    濡傛灉浣犲湪鐜涓缃簡 KCONFIG_OVERWRITECONFIG锛屽綋 .config 鏄寚鍚戝叾浠栦綅缃殑杞摼鎺ユ椂锛孠config
    涓嶄細鏂紑璇ヨ蒋閾炬帴銆?

`KCONFIG_WARN_UNKNOWN_SYMBOLS`
    璇ョ幆澧冨彉閲忎娇 Kconfig 瀵归厤缃緭鍏ヤ腑鎵€鏈夋棤娉曡瘑鍒殑绗﹀彿鍙戝嚭璀﹀憡銆?

`KCONFIG_WERROR`
    濡傛灉璁剧疆锛孠config 灏嗚鍛婅涓洪敊璇€?

`CONFIG_`
    濡傛灉浣犲湪鐜涓缃簡 `CONFIG_`锛孠config 鍦ㄤ繚瀛橀厤缃椂灏嗕负鎵€鏈夌鍙峰姞涓婅鍊间綔涓哄墠缂€锛岃€屼笉鏄?
    浣跨敤榛樿鐨?`CONFIG_`銆?

`{allyes/allmod/allno/alldef/rand}config` 鐨勭幆澧冨彉閲忥細

`KCONFIG_ALLCONFIG`
    allyesconfig/allmodconfig/alldefconfig/allnoconfig/randconfig 鍙樹綋涔熷彲浠ヤ娇鐢ㄧ幆澧冨彉閲?
    KCONFIG_ALLCONFIG 浣滀负鏍囧織鎴栦竴涓寘鍚敤鎴疯姹傝涓虹壒瀹氬€肩殑閰嶇疆绗﹀彿鐨勬枃浠跺悕銆傚鏋?
    KCONFIG_ALLCONFIG 鍦ㄦ病鏈夋枃浠跺悕鐨勬儏鍐典笅浣跨敤锛堝嵆 KCONFIG_ALLCONFIG == "" 鎴?
    KCONFIG_ALLCONFIG == "1"锛夛紝`make *config` 浼氭煡鎵惧悕涓衡€渁ll{yes/mod/no/def/random}.config鈥?
    锛堝搴斾簬鎵€浣跨敤鐨?`*config` 鍛戒护锛夌殑鏂囦欢锛屼互鑾峰彇瑕佸己鍒惰缃殑绗﹀彿鍊笺€傚鏋滄壘涓嶅埌璇ユ枃浠讹紝鍒?
    鏌ユ壘鍚嶄负鈥渁ll.config鈥濈殑鏂囦欢浠ヨ幏鍙栬寮哄埗璁剧疆鐨勫€笺€?

    杩欎娇浣犺兘澶熷垱寤哄彧鍖呭惈浣犳劅鍏磋叮閰嶇疆绗﹀彿鐨勨€滆糠浣犫€濋厤缃紙miniconfig锛夋垨鑷畾涔夐厤缃枃浠躲€傜劧鍚庡唴鏍?
    閰嶇疆绯荤粺浼氱敓鎴愬畬鏁寸殑 .config 鏂囦欢锛屽寘鎷綘 miniconfig 鏂囦欢涓殑绗﹀彿銆?

    璇?`KCONFIG_ALLCONFIG` 鏂囦欢鏄竴涓寘鍚紙閫氬父鏄叏閮ㄧ鍙风殑锛夐璁鹃厤缃鍙风殑閰嶇疆鏂囦欢銆傝繖浜涘彉閲?
    璁剧疆浠嶉渶鎺ュ彈甯歌鐨勪緷璧栨鏌ャ€?

```

        KCONFIG_ALLCONFIG=custom-notebook.config make allnoconfig

    or::

        KCONFIG_ALLCONFIG=mini.config make allnoconfig

    or::

        make KCONFIG_ALLCONFIG=mini.config allnoconfig

    These examples will disable most options (allnoconfig) but enable or
    disable the options that are explicitly listed in the specified
    mini-config files.

```
`randconfig` 鐨勭幆澧冨彉閲忥細

`KCONFIG_SEED`
    濡傛灉浣犲嚭浜庢煇绉嶅師鍥犺璋冭瘯 kconfig 瑙ｆ瀽鍣?鍓嶇鐨勮涓猴紝鍙互灏嗘椤硅涓虹敤浜庣粰 RNG 鎾鐨勬暣鏁板€笺€?
    濡傛灉鏈缃紝灏嗕娇鐢ㄥ綋鍓嶆椂闂淬€?

`KCONFIG_PROBABILITY`
    璇ュ彉閲忓彲鐢ㄤ簬鍋忔枩姒傜巼銆傝鍙橀噺鍙互鏈缃垨涓虹┖锛屾垨璁句负涓夌涓嶅悓鐨勬牸寮忥細

    =======================     ==================  =====================
    KCONFIG_PROBABILITY         y:n 鎷嗗垎甯?         y:m:n 鎷嗗垎甯?
    =======================     ==================  =====================
    unset or empty              50  : 50            33  : 33  : 34
    N                            N  : 100-N         N/2 : N/2 : 100-N
    [^1^] N:M                     N+M : 100-(N+M)      N  :  M  : 100-(N+M)
    [^2^] N:M:L                    N  : 100-N          M  :  L  : 100-(M+L)
    =======================     ==================  =====================

鍏朵腑 N銆丮 鍜?L 鏄寖鍥?[0,100] 鍐呯殑鏁存暟锛堝崄杩涘埗锛夛紝骞朵笖婊¤冻锛?

    [^1^] N+M 鍦ㄨ寖鍥?[0,100] 鍐?

    [^2^] M+L 鍦ㄨ寖鍥?[0,100] 鍐?

```

    KCONFIG_PROBABILITY=10
        10% of booleans will be set to 'y', 90% to 'n'
        5% of tristates will be set to 'y', 5% to 'm', 90% to 'n'
    KCONFIG_PROBABILITY=15:25
        40% of booleans will be set to 'y', 60% to 'n'
        15% of tristates will be set to 'y', 25% to 'm', 60% to 'n'
    KCONFIG_PROBABILITY=10:15:15
        10% of booleans will be set to 'y', 90% to 'n'
        15% of tristates will be set to 'y', 15% to 'm', 70% to 'n'

```
`syncconfig` 鐨勭幆澧冨彉閲忥細

`KCONFIG_NOSILENTUPDATE`
    濡傛灉璇ュ彉閲忓叿鏈夐潪绌哄€硷紝瀹冨皢闃绘闈欓粯鐨勫唴鏍搁厤缃洿鏂帮紙闇€瑕佹樉寮忔洿鏂帮級銆?

`KCONFIG_AUTOCONFIG`
    璇ョ幆澧冨彉閲忓彲璁剧疆浠ユ寚瀹氣€渁uto.conf鈥濇枃浠剁殑璺緞鍜屽悕绉般€傚叾榛樿鍊间负
    鈥渋nclude/config/auto.conf鈥濄€?

`KCONFIG_AUTOHEADER`
    璇ョ幆澧冨彉閲忓彲璁剧疆浠ユ寚瀹氣€渁utoconf.h鈥濓紙澶存枃浠讹級鏂囦欢鐨勮矾寰勫拰鍚嶇О銆傚叾榛樿鍊间负
    鈥渋nclude/generated/autoconf.h鈥濄€?


## menuconfig


鍦?menuconfig 涓悳绱細

    鎼滅储鍔熻兘鎼滅储鍐呮牳閰嶇疆绗﹀彿鍚嶏紝鍥犳浣犲繀椤荤煡閬撴帴杩戜綘瑕佹煡鎵惧唴瀹圭殑鍚嶇О銆?

```

        /hotplug
        This lists all config symbols that contain "hotplug",
        e.g., HOTPLUG_CPU, MEMORY_HOTPLUG.

    For search help, enter / followed by TAB-TAB (to highlight
    <Help>) and Enter.  This will tell you that you can also use
    regular expressions (regexes) in the search string, so if you
    are not interested in MEMORY_HOTPLUG, you could try::

        /^hotplug

    When searching, symbols are sorted thus:

    - first, exact matches, sorted alphabetically (an exact match
      is when the search matches the complete symbol name);
    - then, other matches, sorted alphabetically.

    For example, ^ATH.K matches:

        ATH5K ATH9K ATH5K_AHB ATH5K_DEBUG [...] ATH6KL ATH6KL_DEBUG
        [...] ATH9K_AHB ATH9K_BTCOEX_SUPPORT ATH9K_COMMON [...]

    of which only ATH5K and ATH9K match exactly and so are sorted
    first (and in alphabetical order), then come all other symbols,
    sorted in alphabetical order.

    In this menu, pressing the key in the (#) prefix will jump
    directly to that location. You will be returned to the current
    search results after exiting this new menu.

```
'menuconfig' 鐨勭敤鎴风晫闈㈤€夐」锛?

`MENUCONFIG_COLOR`
    鍙互浣跨敤璇ュ彉閲忛€夋嫨涓嶅悓鐨勯厤鑹蹭富棰?
```

        make MENUCONFIG_COLOR=<theme> menuconfig

    Available themes are::

      - mono       => selects colors suitable for monochrome displays
      - blackbg    => selects a color scheme with black background
      - classic    => theme with blue background. The classic look
      - bluetitle  => a LCD friendly version of classic. (default)

```
`MENUCONFIG_MODE`
    璇ユā寮忓皢鎵€鏈夊瓙鑿滃崟鏄剧ず鍦ㄤ竴涓ぇ鏍戜腑銆?

```

        make MENUCONFIG_MODE=single_menu menuconfig


```
## nconfig


nconfig 鏄竴涓浛浠ｇ殑銆佸熀浜庢枃鏈殑閰嶇疆鍣ㄣ€傚畠鍦ㄧ粓绔紙绐楀彛锛夊簳閮ㄥ垪鍑烘墽琛屽懡浠ょ殑鍔熻兘閿€傞櫎闈炰綘澶勪簬
鏁版嵁杈撳叆绐楀彛涓紝鍚﹀垯涔熷彲浠ョ洿鎺ヤ娇鐢ㄧ浉搴旂殑鏁板瓧閿潵鎵ц鍛戒护銆備緥濡傦紝鍙互鐢?6 浠ｆ浛 F6 鏉ヤ繚瀛樸€?

浣跨敤 F1 鑾峰彇鍏ㄥ眬甯姪锛屾垨 F3 鑾峰彇绠€鐭府鍔╄彍鍗曘€?

鍦?nconfig 涓悳绱細

    浣犲彲浠ュ湪鑿滃崟椤光€減rompt鈥濆瓧绗︿覆涓紝鎴栧湪閰嶇疆绗﹀彿涓悳绱€?

    浣跨敤 / 寮€濮嬪湪鑿滃崟椤逛腑鎼滅储銆傝繖涓嶆敮鎸佹鍒欒〃杈惧紡銆備娇鐢?<Down> 鎴?<Up> 鍒嗗埆鐢ㄤ簬涓嬩竴涓拰涓婁竴涓?
    鍖归厤椤广€備娇鐢?<Esc> 缁堟鎼滅储妯″紡銆?

    F8锛圫ymSearch锛夊湪閰嶇疆绗﹀彿涓悳绱㈢粰瀹氬瓧绗︿覆鎴栨鍒欒〃杈惧紡锛坮egex锛夈€?

    鍦?SymSearch 涓紝鎸変笅 (#) 鍓嶇紑涓殑閿皢鐩存帴璺宠浆鍒拌浣嶇疆銆傞€€鍑鸿繖涓柊鑿滃崟鍚庯紝浣犲皢杩斿洖鍒板綋鍓?
    鐨勬悳绱㈢粨鏋溿€?

鐜鍙橀噺锛?

`NCONFIG_MODE`
    璇ユā寮忓皢鎵€鏈夊瓙鑿滃崟鏄剧ず鍦ㄤ竴涓ぇ鏍戜腑銆?

```

        make NCONFIG_MODE=single_menu nconfig


```
## xconfig


鍦?xconfig 涓悳绱細

    鎼滅储鍔熻兘鎼滅储鍐呮牳閰嶇疆绗﹀彿鍚嶏紝鍥犳浣犲繀椤荤煡閬撴帴杩戜綘瑕佹煡鎵惧唴瀹圭殑鍚嶇О銆?

```

        Ctrl-F hotplug

    or::

        Menu: File, Search, hotplug

    lists all config symbol entries that contain "hotplug" in
    the symbol name.  In this Search dialog, you may change the
    config setting for any of the entries that are not grayed out.
    You can also enter a different search string without having
    to return to the main menu.


```
## gconfig


鍦?gconfig 涓悳绱細

    gconfig 涓病鏈夋悳绱㈠懡浠ゃ€備笉杩囷紝gconfig 纭疄鏈夊嚑绉嶄笉鍚岀殑鏌ョ湅閫夐」銆佹ā寮忓拰璁剧疆銆?
