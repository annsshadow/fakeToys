
## 鍐呮牳椹卞姩 i2c-sis630


鏀寔閫傞厤鍣細
  - Silicon Integrated Systems Corp (SiS)
	630 鑺墖缁勶紙Datasheet: available at http://www.sfr-fresh.com/linux锛?	730 鑺墖缁?	964 鑺墖缁?  - 鍙兘杩樻湁鍏朵粬 SiS 鑺墖缁勶紵

Author:
        - Alexander Malysh <amalysh@web.de>
 - Amaury Decr锚me <amaury.decreme@gmail.com> - SiS964 鏀寔

### 妯″潡鍙傛暟


==================      =====================================================
force = [1|0]           寮哄埗鍚敤 SIS630銆傚嵄闄╋紒
                        杩欏浜庝笂杩版湭鍒楀嚭鐨勮姱鐗囩粍鍙兘鏈夌敤锛屼互妫€鏌ュ畠鏄惁閫傜敤浜庝綘鐨?                        鑺墖缁勶紝浣嗗緢鍗遍櫓锛?
high_clock = [1|0]      寮哄埗灏嗕富鏈轰富鏃堕挓璁句负 56KHz锛堥粯璁ゅ嵆浣犵殑 BIOS 鎵€鐢ㄥ€硷級銆傚嵄闄╋紒
			杩欏簲璇ヤ細绋嶅揩涓€浜涳紝浣嗕細浣挎煇浜涚郴缁燂紙濡傛垜鐨勭瑪璁版湰锛夋鏈恒€?			SIS630/730 鑺墖涓撶敤銆?==================      =====================================================


### 鎻忚堪


宸茬煡璇ヤ粎鏀寔 SMBus 鐨勯┍鍔ㄥ彲鍦ㄤ娇鐢ㄤ笂杩拌姱鐗囩粍鐨勪富鏉夸笂宸ヤ綔銆?
```

  00:00.0 Host bridge: Silicon Integrated Systems [SiS] 630 Host (rev 31)
  00:01.0 ISA bridge: Silicon Integrated Systems [SiS] 85C503/5513

```
```
  00:00.0 Host bridge: Silicon Integrated Systems [SiS] 730 Host (rev 02)
  00:01.0 ISA bridge: Silicon Integrated Systems [SiS] 85C503/5513

```
```
  00:00.0 Host bridge: Silicon Integrated Systems [SiS] 760/M760 Host (rev 02)
  00:02.0 ISA bridge: Silicon Integrated Systems [SiS] SiS964 [MuTIOL Media IO]
							LPC Controller (rev 36)

```
鑻ヤ笂杩拌緭鍑哄嚭鐜板湪浣犵殑 `lspci` 杈撳嚭涓紝鍒欐湰椹卞姩閫傜敤浜庝綘鐨勮姱鐗囩粍銆?
### 鑷磋阿


Philip Edelbrock <phil@netroedge.com>
- 娴嬭瘯 SiS730 鏀寔
Mark M. Hoffman <mhoffman@lightlink.com>
- bug 淇

涔熸劅璋㈣繖閲岃鎴戦仐婕忕殑浠讳綍浜?;)
