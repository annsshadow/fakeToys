
## lmedm04 网卡的固件文件


要为 DM04/QQBOX 提取固件，需将下列文件复制到本目录。

### 适用于 DM04+/QQBOX LME2510C（Sharp 7395 调谐器）


Sharp 7395 驱动可在 windows/system32/drivers 中找到

US2A0D.sys（日期 2009 年 3 月 17 日）


然后运行：


	scripts/get_dvb_firmware lme2510c_s7395

会生成 dvb-usb-lme2510c-s7395.fw

另一份较旧的固件可在驱动光盘 DVB-S_EN_3.5A 的 BDADriver/driver 中找到

LMEBDA_DVBS7395C.sys（日期 2008 年 1 月 18 日）


然后运行：


	./get_dvb_firmware lme2510c_s7395_old

会生成 dvb-usb-lme2510c-s7395.fw

LG 固件可在驱动光盘 DM04+_5.1A[LG] 的 BDADriver/driver 中找到

### 适用于 DM04 LME2510（LG 调谐器）


LMEBDA_DVBS.sys（日期 2007 年 11 月 13 日）


然后运行：


	./get_dvb_firmware lme2510_lg

会生成 dvb-usb-lme2510-lg.fw


其他 LG 固件只能从 windows/system32/drivers 中的 US280D.sys 手动提取

dd if=US280D.sys ibs=1 skip=42360 count=3924 of=dvb-usb-lme2510-lg.fw

### 适用于 DM04 LME2510C（LG 调谐器）


	dd if=US280D.sys ibs=1 skip=35200 count=3850 of=dvb-usb-lme2510c-lg.fw


Sharp 0194 调谐器驱动可在 windows/system32/drivers 中找到

US290D.sys（日期 2009 年 4 月 9 日）

### 适用于 LME2510


	dd if=US290D.sys ibs=1 skip=36856 count=3976 of=dvb-usb-lme2510-s0194.fw


### 适用于 LME2510C


	dd if=US290D.sys ibs=1 skip=33152 count=3697 of=dvb-usb-lme2510c-s0194.fw


m88rs2000 调谐器驱动可在 windows/system32/drivers 中找到

US2B0D.sys（日期 2010 年 6 月 29 日）


	dd if=US2B0D.sys ibs=1 skip=34432 count=3871 of=dvb-usb-lme2510c-rs2000.fw

我们需要修改 rs2000 固件的 id，否则它将以热启动 id 3344:1120 启动。


	echo -ne \\xF0\\x22 | dd conv=notrunc bs=1 count=2 seek=266 of=dvb-usb-lme2510c-rs2000.fw

将固件文件复制到 /lib/firmware
