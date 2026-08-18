
## lmedm04 缃戝崱鐨勫浐浠舵枃浠?

瑕佷负 DM04/QQBOX 鎻愬彇鍥轰欢锛岄渶灏嗕笅鍒楁枃浠跺鍒跺埌鏈洰褰曘€?
### 閫傜敤浜?DM04+/QQBOX LME2510C锛圫harp 7395 璋冭皭鍣級


Sharp 7395 椹卞姩鍙湪 windows/system32/drivers 涓壘鍒?
US2A0D.sys锛堟棩鏈?2009 骞?3 鏈?17 鏃ワ級


鐒跺悗杩愯锛?

	scripts/get_dvb_firmware lme2510c_s7395

浼氱敓鎴?dvb-usb-lme2510c-s7395.fw

鍙︿竴浠借緝鏃х殑鍥轰欢鍙湪椹卞姩鍏夌洏 DVB-S_EN_3.5A 鐨?BDADriver/driver 涓壘鍒?
LMEBDA_DVBS7395C.sys锛堟棩鏈?2008 骞?1 鏈?18 鏃ワ級


鐒跺悗杩愯锛?

	./get_dvb_firmware lme2510c_s7395_old

浼氱敓鎴?dvb-usb-lme2510c-s7395.fw

LG 鍥轰欢鍙湪椹卞姩鍏夌洏 DM04+_5.1A[LG] 鐨?BDADriver/driver 涓壘鍒?
### 閫傜敤浜?DM04 LME2510锛圠G 璋冭皭鍣級


LMEBDA_DVBS.sys锛堟棩鏈?2007 骞?11 鏈?13 鏃ワ級


鐒跺悗杩愯锛?

	./get_dvb_firmware lme2510_lg

浼氱敓鎴?dvb-usb-lme2510-lg.fw


鍏朵粬 LG 鍥轰欢鍙兘浠?windows/system32/drivers 涓殑 US280D.sys 鎵嬪姩鎻愬彇

dd if=US280D.sys ibs=1 skip=42360 count=3924 of=dvb-usb-lme2510-lg.fw

### 閫傜敤浜?DM04 LME2510C锛圠G 璋冭皭鍣級


	dd if=US280D.sys ibs=1 skip=35200 count=3850 of=dvb-usb-lme2510c-lg.fw


Sharp 0194 璋冭皭鍣ㄩ┍鍔ㄥ彲鍦?windows/system32/drivers 涓壘鍒?
US290D.sys锛堟棩鏈?2009 骞?4 鏈?9 鏃ワ級

### 閫傜敤浜?LME2510


	dd if=US290D.sys ibs=1 skip=36856 count=3976 of=dvb-usb-lme2510-s0194.fw


### 閫傜敤浜?LME2510C


	dd if=US290D.sys ibs=1 skip=33152 count=3697 of=dvb-usb-lme2510c-s0194.fw


m88rs2000 璋冭皭鍣ㄩ┍鍔ㄥ彲鍦?windows/system32/drivers 涓壘鍒?
US2B0D.sys锛堟棩鏈?2010 骞?6 鏈?29 鏃ワ級


	dd if=US2B0D.sys ibs=1 skip=34432 count=3871 of=dvb-usb-lme2510c-rs2000.fw

鎴戜滑闇€瑕佷慨鏀?rs2000 鍥轰欢鐨?id锛屽惁鍒欏畠灏嗕互鐑惎鍔?id 3344:1120 鍚姩銆?

	echo -ne \\xF0\\x22 | dd conv=notrunc bs=1 count=2 seek=266 of=dvb-usb-lme2510c-rs2000.fw

灏嗗浐浠舵枃浠跺鍒跺埌 /lib/firmware
