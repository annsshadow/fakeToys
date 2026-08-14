
## AMD HSMP 鎺ュ彛


鏇存柊鐨?Fam19h锛堝瀷鍙?0x00-0x1f銆?x30-0x3f銆?x90-0x9f銆?xa0-0xaf锛夈€?Fam1Ah锛堝瀷鍙?0x00-0x1f锛堿MD EPYC 鏈嶅姟鍣ㄧ郴鍒楀鐞嗗櫒閫氳繃 HSMP锛圚ost System Management
Port锛屼富鏈虹郴缁熺鐞嗙鍙ｏ級鏀寔绯荤粺绠＄悊鍔熻兘銆?
涓绘満绯荤粺绠＄悊绔彛锛圚SMP锛夋槸涓€涓帴鍙ｏ紝鐢ㄤ簬鍚戞搷浣滅郴缁熺骇鍒殑杞欢鎻愪緵瀵逛竴缁勯偖绠卞瘎瀛樺櫒鐨?绯荤粺绠＄悊鍔熻兘鐨勮闂€?
鍏充簬璇ユ帴鍙ｇ殑鏇村缁嗚妭鍙互鍦ㄥ搴?family/model 鐨?PPR 鐨?7 Host System Management Port
(HSMP)"绔犺妭涓壘鍒帮紝渚嬪锛歨ttps://docs.amd.com/v/u/en-US/55898_B1_pub_0_50


HSMP 鎺ュ彛鍦?EPYC 绯诲垪鏈嶅姟鍣?CPU 鍜?MI300A锛圓PU锛変笂鍙楁敮鎸併€?

## HSMP 璁惧


浣嶄簬 drivers/platforms/x86/amd/hsmp/ 涓嬬殑 amd_hsmp 椹卞姩锛屼负鍩轰簬 ACPI 瀵硅薄鐨勬帰娴嬨€佸熀浜?骞冲彴璁惧鐨勬帰娴嬶紝浠ュ強杩欎袱涓┍鍔ㄧ殑鍏叡浠ｇ爜锛屽垎鍒彁渚涚嫭绔嬬殑椹卞姩鏂囦欢銆?
Kconfig 閫夐」 CONFIG_AMD_HSMP_PLAT 缂栬瘧 plat.c 骞剁敓鎴?amd_hsmp.ko銆?Kconfig 閫夐」 CONFIG_AMD_HSMP_ACPI 缂栬瘧 acpi.c 骞剁敓鎴?hsmp_acpi.ko銆?閫夋嫨杩欎袱涓厤缃腑鐨勪换鎰忎竴涓兘浼氳嚜鍔ㄩ€変腑 CONFIG_AMD_HSMP銆傝繖浼氱紪璇戝叕鍏变唬鐮?hsmp.c 骞?鐢熸垚 hsmp_common.ko 妯″潡銆?
ACPI 鍜?plat 涓や釜椹卞姩閮戒細鍒涘缓 miscdevice /dev/hsmp锛屼互渚跨敤鎴风┖闂寸▼搴忚繍琛?hsmp 閭
鍛戒护銆?
椹卞姩鏀寔鐨?ACPI 瀵硅薄鏍煎紡瀹氫箟濡備笅銆?
$ ls -al /dev/hsmp
crw-r--r-- 1 root root 10, 123 Jan 21 21:41 /dev/hsmp

璁惧鑺傜偣鐨勭壒鎬э細
 - 鍐欐ā寮忕敤浜庤繍琛?set/configure锛堣缃?閰嶇疆锛夊懡浠? - 璇绘ā寮忕敤浜庤繍琛?get/status锛堣幏鍙?鐘舵€侊級鐩戣鍛戒护

璁块棶闄愬埗锛? - 鍙湁 root 鐢ㄦ埛琚厑璁镐互鍐欐ā寮忔墦寮€璇ユ枃浠躲€? - 鎵€鏈夌敤鎴烽兘鍙互浠ヨ妯″紡鎵撳紑璇ユ枃浠躲€?
鍐呮牳鍐呴泦鎴愶細
 - 鍐呮牳涓殑鍏跺畠瀛愮郴缁熷彲浠ヤ娇鐢ㄥ鍑虹殑浼犺緭鍑芥暟 hsmp_send_message()銆? - 璺ㄨ皟鐢ㄦ柟鐨勫姞閿佺敱椹卞姩璐熻矗銆?

## HSMP sysfs 鎺ュ彛


1. 鎸囨爣琛ㄤ簩杩涘埗 sysfs

AMD MI300A MCM 鎻愪緵浜?GET_METRICS_TABLE 娑堟伅锛岀敤浜庝竴娆℃€т粠 SMU 鑾峰彇澶ч儴鍒嗙殑绯荤粺绠＄悊
淇℃伅銆?
鎸囨爣琛ㄤ綔涓哄崄鍏繘鍒?sysfs 浜岃繘鍒舵枃浠舵彁渚涳紝浣嶄簬姣忎釜 socket 鐨?sysfs 鐩綍涓嬶紝璇ョ洰褰曞垱寤轰簬
/sys/devices/platform/amd_hsmp/socket%d/metrics_bin

娉ㄦ剰锛氫笉鏀寔 lseek()锛屽洜涓烘暣涓寚鏍囪〃浼氳璇诲彇銆?
鎸囨爣琛ㄧ殑瀹氫箟灏嗕綔涓?Public PPR 鐨勪竴閮ㄥ垎杩涜鏂囨。鍖栥€傚悓鏍风殑瀹氫箟涔熷湪 amd_hsmp.h 澶存枃浠朵腑銆?
2. HSMP 閬ユ祴 sysfs 鏂囦欢

浠ヤ笅 sysfs 鏂囦欢鍦?/sys/devices/platform/AMDI0097:0X/ 涓嬪彲鐢ㄣ€?
- c0_residency_input锛氬浜?C0 鐘舵€佺殑鏍哥殑鐧惧垎姣斻€?- prochot_status锛氬鏋滃鐞嗗櫒澶勪簬鐑槇鍊煎垯杩斿洖 1锛屽惁鍒欒繑鍥?0銆?- smu_fw_version锛歋MU 鍥轰欢鐗堟湰銆?- protocol_version锛欻SMP 鎺ュ彛鐗堟湰銆?- ddr_max_bw锛氱悊璁烘渶澶?DDR 甯﹀锛屽崟浣嶄负 GB/s銆?- ddr_utilised_bw_input锛氬綋鍓嶅凡浣跨敤鐨?DDR 甯﹀锛屽崟浣嶄负 GB/s銆?- ddr_utilised_bw_perc_input(%)锛氬綋鍓嶅凡浣跨敤 DDR 甯﹀鐨勭櫨鍒嗘瘮銆?- mclk_input锛氬唴瀛樻椂閽熼鐜囷紝鍗曚綅涓?MHz銆?- fclk_input锛欶abric 鏃堕挓棰戠巼锛屽崟浣嶄负 MHz銆?- clk_fmax锛歴ocket 鐨勬渶澶ч鐜囷紝鍗曚綅涓?MHz銆?- clk_fmin锛歴ocket 鐨勬渶灏忛鐜囷紝鍗曚綅涓?MHz銆?- cclk_freq_limit_input锛氭瘡涓?socket 鐨勬牳鏃堕挓棰戠巼闄愬埗锛屽崟浣嶄负 MHz銆?- pwr_current_active_freq_limit锛歴ocket 褰撳墠鐨勬椿鍔ㄩ鐜囬檺鍒讹紝鍗曚綅涓?MHz銆?- pwr_current_active_freq_limit_source锛氬綋鍓嶆椿鍔ㄩ鐜囬檺鍒剁殑鏉ユ簮銆?
## ACPI 璁惧瀵硅薄鏍煎紡


amd_hsmp 椹卞姩鏈熸湜鐨?ACPI 瀵硅薄鏍煎紡
```

  Device(HSMP)
		{
			Name(_HID, "AMDI0097")
			Name(_UID, "ID00")
			Name(HSE0, 0x00000001)
			Name(RBF0, ResourceTemplate()
			{
				Memory32Fixed(ReadWrite, 0xxxxxxx, 0x00100000)
			})
			Method(_CRS, 0, NotSerialized)
			{
				Return(RBF0)
			}
			Method(_STA, 0, NotSerialized)
			{
				If(LEqual(HSE0, One))
				{
					Return(0x0F)
				}
				Else
				{
					Return(Zero)
				}
			}
			Name(_DSD, Package(2)
			{
				Buffer(0x10)
				{
					0x9D, 0x61, 0x4D, 0xB7, 0x07, 0x57, 0xBD, 0x48,
					0xA6, 0x9F, 0x4E, 0xA2, 0x87, 0x1F, 0xC2, 0xF6
				},
				Package(3)
				{
					Package(2) {"MsgIdOffset", 0x00010934},
					Package(2) {"MsgRspOffset", 0x00010980},
					Package(2) {"MsgArgOffset", 0x000109E0}
				}
			})
		}

```
## HSMP HWMON 鎺ュ彛


HSMP 鐢垫簮浼犳劅鍣ㄥ悜 hwmon 鎺ュ彛娉ㄥ唽銆備负姣忎釜 socket 鍒涘缓涓€涓嫭绔嬬殑 hwmon 鐩綍锛屽苟鍦ㄨ hwmon
鐩綍涓敓鎴愪互涓嬫枃浠躲€?- power1_input锛堝彧璇伙級
- power1_cap_max锛堝彧璇伙級
- power1_cap锛堣銆佸啓锛?
## 涓€涓ず渚?

浠?C 绋嬪簭璁块棶 hsmp 璁惧銆?```

  #include <linux/amd_hsmp.h>

```
鍏跺畾涔変簡鍙楁敮鎸佺殑娑堟伅/娑堟伅 ID銆?```

  int file;

  file = open("/dev/hsmp", O_RDWR);
  if (file < 0) {
    /* ERROR HANDLING; 浣犲彲浠ユ鏌?errno 鏉ヤ簡瑙ｅ嚭浜嗕粈涔堥棶棰?*/
    exit(1);
  }

```
瀹氫箟浜嗕互涓?IOCTL锛?
`ioctl(file, HSMP_IOCTL_CMD, struct hsmp_message *msg)`
```

    struct hsmp_message {
    	__u32	msg_id;				/* 娑堟伅 ID */
    	__u16	num_args;			/* 娑堟伅涓緭鍏ュ弬鏁板瓧鐨勪釜鏁?*/
    	__u16	response_sz;			/* 鏈熸湜鐨勮緭鍑?鍝嶅簲瀛楃殑涓暟 */
    	__u32	args[HSMP_MAX_MSG_LEN];		/* 鍙傛暟/鍝嶅簲缂撳啿 */
    	__u16	sock_ind;			/* socket 缂栧彿 */
    };

```
璇?ioctl 鍦ㄥけ璐ユ椂浼氳繑鍥為潪闆跺€硷紱浣犲彲浠ヨ鍙?errno 鏉ヤ簡瑙ｅ彂鐢熶簡浠€涔堛€傝浜嬪姟鍦ㄦ垚鍔熸椂杩斿洖 0銆?
鍏充簬璇ユ帴鍙ｅ拰娑堟伅瀹氫箟鐨勬洿澶氱粏鑺傚彲浠ュ湪瀵瑰簲 family/model 鐨?PPR 鐨?7 Host System
Management Port (HSMP)"绔犺妭涓壘鍒帮紝渚嬪锛歨ttps://docs.amd.com/v/u/en-US/55898_B1_pub_0_50

鐢ㄦ埛绌洪棿 C-API 鍙€氳繃閾炬帴 esmi 搴撹幏寰楋紝璇ュ簱鐢?E-SMS 椤圭洰鎻愪緵
https://www.amd.com/en/developer/e-sms.html銆傚弬瑙侊細https://github.com/amd/esmi_ib_library
