## 鍐呮牳椹卞姩 i2c-sis96x


鍙栦唬 2.4.x 鐨?i2c-sis645

鏀寔鐨勯€傞厤鍣細

  - Silicon Integrated Systems Corp (SiS)

    杩欎簺涓绘ˉ鐨勪换鎰忕粍鍚堬細
	645, 645DX (aka 646), 648, 650, 651, 655, 735, 745, 746

    浠ュ強杩欎簺鍗楁ˉ锛?	961, 962, 963(L)

Author: Mark M. Hoffman <mhoffman@lightlink.com>

### 鎻忚堪


杩欎釜浠?SMBus 鐨勯┍鍔ㄥ凡鐭ュ彲鍦ㄥ甫鏈変笂杩拌姱鐗囩粍缁勫悎鐨勭殑涓绘澘涓婂伐浣溿€傝椹卞姩鏄湪娌℃湁 SiS
姝ｈ鏁版嵁鎵嬪唽鐨勬儏鍐典笅寮€鍙戠殑銆係MBus 瀵勫瓨鍣ㄨ鍋囧畾涓?SiS630 鐨勫吋瀹癸紝灏界瀹冧滑浣嶄簬瀹屽叏
涓嶅悓鐨勪綅缃€傛劅璋?Alexander Malysh <amalysh@web.de> 鎻愪緵浜?SiS630 鏁版嵁鎵嬪唽锛堝強椹卞姩锛夈€?
```

  00:00.0 Host bridge: Silicon Integrated Systems [SiS]: Unknown device 0645
  00:02.0 ISA bridge: Silicon Integrated Systems [SiS] 85C503/5513
  00:02.1 SMBus: Silicon Integrated Systems [SiS]: Unknown device 0016

```
```

  00:00.0 Host bridge: Silicon Integrated Systems [SiS]: Unknown device 0645
  00:02.0 ISA bridge: Silicon Integrated Systems [SiS]: Unknown device 0961
  00:02.1 SMBus: Silicon Integrated Systems [SiS]: Unknown device 0016

```
锛?.4.18 涔嬪悗鐗堟湰鐨勫唴鏍稿彲鑳戒細濉笂閭ｄ簺鈥淯nknown鈥濓級

濡傛灉浣犵湅涓嶅埌瀹冿紝璇锋煡鐪?quirk_sis_96x_smbus锛坉rivers/pci/quirks.c锛夛紙鍗楁ˉ妫€娴嬪け璐?鏃朵篃閫傜敤锛?
鎴戞€€鐤戣繖涓┍鍔ㄤ篃鍙互琚敼閫犱负鏀寔浠ヤ笅 SiS 鑺墖缁勶細635 涓?635T銆傚鏋滄湁浜烘嫢鏈夊甫杩欎簺
鑺墖鐨勪富鏉匡紝骞朵笖鎰挎剰涓轰簡杩涙鑰屽啋闄╄涓€涓師鏈涓鸿壇濂界殑鍐呮牳宕╂簝鈥︹€﹁閫氳繃
<mhoffman@lightlink.com> 鎴栭€氳繃 linux-i2c 閭欢鍒楄〃锛?linux-i2c@vger.kernel.org>
鑱旂郴鎴戙€備篃璇峰彂閫?bug 鎶ュ憡涓?鎴栨垚鍔熸渚嬨€?

### 寰呭姙锛圱O DOs锛?

- 璇ラ┍鍔ㄤ笉鏀寔 SMBus 鍧楄/鍐欙紱濡傛灉鍙戠幇闇€瑕佸畠浠殑鍦烘櫙锛屾垜鍙兘浼氭坊鍔犮€?

### 鑷磋阿锛圱hank You锛?

Mark D. Studebaker <mdsxyz123@yahoo.com>
 - 璁捐鎻愮ず涓?bug 淇

Alexander Maylsh <amalysh@web.de>
 - 鍚屼笂锛屽鍔犱竴浠介噸瑕佺殑鏁版嵁鎵嬪唽鈥︹€﹀嚑涔庡氨鏄垜鐪熸鎯宠鐨勯偅浠?
Hans-G眉nter L眉tke Uphues <hg_lu@t-online.de>
 - SiS735 鐨勮ˉ涓?
Robert Zwerus <arzie@dds.nl>
 - SiS645DX 鐨勬祴璇?
Kianusch Sayah Karadji <kianusch@sk-tech.net>
 - SiS645DX/962 鐨勮ˉ涓?
Ken Healy
 - SiS655 鐨勮ˉ涓?
涔熸劅璋㈠叾瀹冧换浣曟彁渚涘弽棣堟潵鍑界殑浜猴紒
