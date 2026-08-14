## CerfBoard/Cube


*** CerfBoard/Cube 鐨?StrongARM 鐗堟湰宸插仠浜?***

Intrinsyc CerfBoard 鏄竴娆惧熀浜?StrongARM 1110 鐨勬澘杞借绠楁満锛屽昂瀵哥害涓?2 鑻卞
瑙佹柟銆傚畠鍖呭惈涓€涓互澶綉鎺у埗鍣ㄣ€佷竴涓吋瀹?RS232 鐨勪覆鍙ｃ€佷竴涓?USB 鍔熻兘绔彛锛屼互鍙?鑳岄潰涓€涓?CompactFlash+ 鎻掓Ы銆傚浘鐗囧彲鍦?Intrinsyc 缃戠珯 http://www.intrinsyc.com
鎵惧埌銆?
鏈枃妗ｆ弿杩?Linux 鍐呮牳瀵?Intrinsyc CerfBoard 鐨勬敮鎸併€?
## 姝ょ増鏈敮鎸?

   - CompactFlash+ 鎻掓Ы锛堝湪 General Setup 涓€夋嫨 PCMCIA 浠ュ強浠讳綍鍙兘闇€瑕佺殑閫夐」锛?   - 鏉胯浇 Crystal CS8900 浠ュお缃戞帶鍒跺櫒锛圢etwork Devices 涓殑 Cerf CS8900A 鏀寔锛?   - 甯︿覆鍙ｆ帶鍒跺彴鐨勪覆鍙ｏ紙纭紪鐮佷负 38400 8N1锛?
涓轰簡灏嗘鍐呮牳瑁呭叆浣犵殑 Cerf锛屼綘闇€瑕佷竴鍙板悓鏃惰繍琛?BOOTP 鍜?TFTP 鐨勬湇鍔″櫒銆傚叧浜?濡備綍浣跨敤寮曞鍔犺浇绋嬪簭鐨勮缁嗚鏄庡簲闅忎綘鐨勮瘎浼板浠舵彁渚涖€傝繖涓€绯诲垪鍛戒护
```

   make ARCH=arm CROSS_COMPILE=arm-linux- cerfcube_defconfig
   make ARCH=arm CROSS_COMPILE=arm-linux- zImage
   make ARCH=arm CROSS_COMPILE=arm-linux- modules
   cp arch/arm/boot/zImage <TFTP directory>

```
support@intrinsyc.com
