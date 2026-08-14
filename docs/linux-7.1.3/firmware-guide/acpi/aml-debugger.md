## AML 璋冭瘯鍣?

:Copyright: |copy| 2016, Intel Corporation
:Author: Lv Zheng <lv.zheng@intel.com>


鏈枃妗ｆ弿杩?Linux 鍐呮牳涓唴宓岀殑 AML 璋冭瘯鍣ㄧ殑鐢ㄦ硶銆?
## 1. 鏋勫缓璋冭瘯鍣?

鍚敤 AML 璋冭瘯鍣ㄩ渶瑕佷互涓嬪唴鏍搁厤缃」锛?
```
   CONFIG_ACPI_DEBUGGER=y
   CONFIG_ACPI_DEBUGGER_USER=m

```

鐢ㄦ埛绌洪棿宸ュ叿鍙互浣跨敤浠ヤ笅鍛戒护浠庡唴鏍告簮鐮佹爲鏋勫缓锛?
```
   $ cd tools
   $ make acpi

```

```
   tools/power/acpi/acpidbg

```

瀹冨彲浠ラ€氳繃杩愯 "make install"锛堜互鍏锋湁瓒冲鏉冮檺鐨勭敤鎴凤級瀹夎鍒扮郴缁熺洰褰曘€?
## 2. 鍚姩鐢ㄦ埛绌洪棿璋冭瘯鍣ㄦ帴鍙?

鍦ㄥ唴鏍镐互鍐呭缓璋冭瘯鍣ㄥ惎鍔ㄥ悗锛屽彲浠ラ€氳繃浠ヤ笅鏂瑰紡鍚姩璋冭瘯鍣細

```
   # mount -t debugfs none /sys/kernel/debug
   # modprobe acpi_dbg
   # tools/power/acpi/acpidbg

```

杩欏皢杩涘叆浜や簰寮?AML 璋冭瘯鍣ㄧ幆澧冿紝鍦ㄥ叾涓彲浠ユ墽琛岃皟璇曞櫒鍛戒护銆?
杩欎簺鍛戒护璁板綍鍦ㄢ€淎CPICA Overview and Programmer Reference鈥濅腑锛屽彲浠?
https://acpica.org/documentation

涓嬭浇銆傝缁嗙殑璋冭瘯鍣ㄥ懡浠ゅ弬鑰冧綅浜庣 12 绔犫€淎CPICA Debugger Reference鈥濄€?鍙互浣跨敤 "help" 鍛戒护杩涜蹇€熷弬鑰冦€?
## 3. 鍋滄鐢ㄦ埛绌洪棿璋冭瘯鍣ㄦ帴鍙?

浜や簰寮忚皟璇曞櫒鎺ュ彛鍙互閫氳繃鎸?Ctrl+C 鎴栦娇鐢ㄤ互涓嬫柟寮忓叧闂細

```
   # rmmod acpi_dbg

```

濡傛灉鏈変竴涓?acpidbg 瀹炰緥姝ｅ湪杩愯锛屾ā鍧楃殑鍗歌浇鍙兘浼氬け璐ャ€?
## 4. 鍦ㄨ剼鏈腑杩愯璋冭瘯鍣?

鍦ㄦ祴璇曡剼鏈腑杩愯 AML 璋冭瘯鍣ㄥ彲鑳藉緢鏈夌敤銆?acpidbg" 浠ョ壒娈婄殑鈥滄壒澶勭悊鈥濇ā寮?鏀寔杩欎竴鐐广€備緥濡傦紝浠ヤ笅鍛戒护杈撳嚭锛?
```
   # acpidbg -b "namespace"

```
