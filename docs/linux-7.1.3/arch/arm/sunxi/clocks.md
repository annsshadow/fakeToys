## 鍏充簬 sunxi 鏃堕挓绯荤粺鐨勫父瑙侀棶棰?


鏈枃妗ｅ寘鍚汉浠粡甯歌闂殑鏈夊叧 sunxi 鏃堕挓绯荤粺鐨勪竴浜涙湁鐢ㄤ俊鎭紝骞跺湪閫傚綋鏃堕檮鏈?ASCII 鍥剧ず銆?

闂細涓轰粈涔堜富 24MHz 鎸崱鍣ㄥ彲浠ラ棬鎺э紵杩欎笉浼氱牬鍧忕郴缁熷悧锛?

绛旓細24MHz 鎸崱鍣ㄥ厑璁搁棬鎺т互鑺傜渷鍔熻€椼€傜‘瀹烇紝濡傛灉涓嶅姞娉ㄦ剰鍦拌繘琛岄棬鎺э紝绯荤粺灏嗗仠姝㈣繍琛岋紝浣嗛€氳繃姝ｇ‘鐨勬楠わ紝鍙互瀵瑰叾杩涜闂ㄦ帶鍚屾椂淇濇寔绯荤粺杩愯銆傝鑰冭檻浠ヤ笅绠€鍖栫殑鎸傝捣绀轰緥锛?

```

      24MHz         32kHz
       |
      PLL1
       \
        \_ CPU Mux
             |
           [CPU]

   When you are about to suspend, you switch the CPU Mux to the 32kHz
   oscillator::

      24Mhz         32kHz
       |              |
      PLL1            |
                     /
           CPU Mux _/
             |
           [CPU]

    Finally you can gate the main oscillator::

                    32kHz
                      |
                      |
                     /
           CPU Mux _/
             |
           [CPU]

```
闂細鍦ㄥ摢閲屽彲浠ヤ簡瑙ｆ洿澶氬叧浜?sunxi 鏃堕挓鐨勪俊鎭紵

绛旓細linux-sunxi wiki 鍖呭惈涓€涓褰曟椂閽熷瘎瀛樺櫒鐨勯〉闈紝浣犲彲浠ュ湪

        http://linux-sunxi.org/A10/CCM

   鎵惧埌瀹冦€傜洰鍓嶆潈濞佺殑淇℃伅鏉ユ簮鏄?Allwinner 鍙戝竷鐨?ccmu 椹卞姩锛屼綘鍙互鍦?

        https://github.com/linux-sunxi/linux-sunxi/tree/sunxi-3.0/arch/arm/mach-sun4i/clock/ccmu

   鎵惧埌瀹冦€?
