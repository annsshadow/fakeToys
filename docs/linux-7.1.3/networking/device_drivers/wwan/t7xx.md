


## 鍩轰簬 MTK PCIe 鐨?T700 5G 璋冨埗瑙ｈ皟鍣ㄧ殑 t7xx 椹卞姩


t7xx 椹卞姩鏄竴涓负 linux 鎴?Chrome OS 骞冲彴寮€鍙戠殑 WWAN PCIe 涓绘満椹卞姩锛岀敤浜庡湪涓绘骞冲彴
涓?MediaTek 鐨?T700 5G 璋冨埗瑙ｈ皟鍣ㄤ箣闂撮€氳繃 PCIe 鎺ュ彛杩涜鏁版嵁浜ゆ崲銆?璇ラ┍鍔ㄦ毚闇蹭簡涓€涓鍚?MBIM 鍗忚 [^1^] 鐨勬帴鍙ｃ€備换浣曞墠绔簲鐢ㄧ▼搴忥紙渚嬪 Modem Manager锛?閮藉彲浠ヨ交鏉剧鐞?MBIM 鎺ュ彛浠ュ惎鐢ㄩ€氬線 WWAN 鐨勬暟鎹€氫俊銆傝椹卞姩杩樻彁渚涗簡涓€涓€氳繃 AT 鍛戒护
涓?MediaTek 璋冨埗瑙ｈ皟鍣ㄤ氦浜掔殑鎺ュ彛銆?
## 鍩烘湰鐢ㄦ硶


褰撲笉鍙楃鐞嗘椂锛孧BIM 鍜?AT 鍔熻兘澶勪簬闈炴椿鍔ㄧ姸鎬併€倀7xx 椹卞姩鎻愪緵浠ｈ〃 MBIM 鍜?AT 鎺у埗閫氶亾
鐨?WWAN 绔彛鐢ㄦ埛绌洪棿鎺ュ彛锛屼絾鍦ㄧ鐞嗗叾鍔熻兘鏂归潰涓嶈捣浠讳綍浣滅敤銆傛娴嬬鍙ｆ灇涓惧苟鍚敤 MBIM
鍜?AT 鍔熻兘鏄敤鎴风┖闂村簲鐢ㄧ▼搴忕殑宸ヤ綔銆?
鍑犱釜杩欐牱鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忕ず渚嬶細

- mbimcli锛堝寘鍚湪 libmbim [^2^] 搴撲腑锛夛紝浠ュ強
- Modem Manager [^3^]

绠＄悊搴旂敤绋嬪簭鎵ц浠ヤ笅寤虹珛 MBIM IP 浼氳瘽鎵€闇€鐨勬搷浣滐細

- 鎵撳紑 MBIM 鎺у埗閫氶亾
- 閰嶇疆缃戠粶杩炴帴璁剧疆
- 杩炴帴鍒扮綉缁?- 閰嶇疆 IP 缃戠粶鎺ュ彛

绠＄悊搴旂敤绋嬪簭鎵ц浠ヤ笅鍙戦€?AT 鍛戒护骞舵帴鏀跺搷搴旀墍闇€鐨勬搷浣滐細

- 浣跨敤 UART 宸ュ叿鎴栦笓鐢ㄧ敤鎴峰伐鍏锋墦寮€ AT 鎺у埗閫氶亾

## Sysfs


璇ラ┍鍔ㄥ悜鐢ㄦ埛绌洪棿鎻愪緵 sysfs 鎺ュ彛銆?
### t7xx_mode


璇?sysfs 鎺ュ彛鍚戠敤鎴风┖闂存彁渚涘璁惧妯″紡鐨勮闂紝姝ゆ帴鍙ｆ敮鎸佽鍜屽啓鎿嶄綔銆?
璁惧妯″紡锛?
- `unknown` 琛ㄧず璁惧澶勪簬鏈煡鐘舵€?- `ready` 琛ㄧず璁惧澶勪簬灏辩华鐘舵€?- `reset` 琛ㄧず璁惧澶勪簬澶嶄綅鐘舵€?- `fastboot_switching` 琛ㄧず璁惧澶勪簬 fastboot 鍒囨崲鐘舵€?- `fastboot_download` 琛ㄧず璁惧澶勪簬 fastboot 涓嬭浇鐘舵€?- `fastboot_dump` 琛ㄧず璁惧澶勪簬 fastboot 杞偍鐘舵€?
浠庣敤鎴风┖闂磋鍙栦互鑾峰彇褰撳墠璁惧妯″紡銆?
```
  $ cat /sys/bus/pci/devices/${bdf}/t7xx_mode

```
浠庣敤鎴风┖闂村啓鍏ヤ互璁剧疆璁惧妯″紡銆?
```
  $ echo fastboot_switching > /sys/bus/pci/devices/${bdf}/t7xx_mode

```
### t7xx_debug_ports


璇?sysfs 鎺ュ彛鍚戠敤鎴风┖闂存彁渚涘惎鐢?绂佺敤璋冭瘯绔彛鐨勮闂紝姝ゆ帴鍙ｆ敮鎸佽鍜屽啓鎿嶄綔銆?
璋冭瘯绔彛鐘舵€侊細

- `1` 琛ㄧず鍚敤璋冭瘯绔彛
- `0` 琛ㄧず绂佺敤璋冭瘯绔彛

褰撳墠鏀寔鐨勮皟璇曠鍙ｏ紙ADB/MIPC锛夈€?
浠庣敤鎴风┖闂磋鍙栦互鑾峰彇褰撳墠璋冭瘯绔彛鐘舵€併€?
```
  $ cat /sys/bus/pci/devices/${bdf}/t7xx_debug_ports

```
浠庣敤鎴风┖闂村啓鍏ヤ互璁剧疆璋冭瘯绔彛鐘舵€併€?
```
  $ echo 1 > /sys/bus/pci/devices/${bdf}/t7xx_debug_ports

```
## 绠＄悊搴旂敤绋嬪簭寮€鍙?

椹卞姩鍜岀敤鎴风┖闂存帴鍙ｆ弿杩板涓嬨€侻BIM 鍗忚鍦?[^1^] Mobile Broadband Interface Model
v1.0 Errata-1 涓弿杩般€?
### MBIM 鎺у埗閫氶亾鐢ㄦ埛绌洪棿 ABI


#### /dev/wwan0mbim0 瀛楃璁惧


璇ラ┍鍔ㄩ€氳繃瀹炵幇 MBIM WWAN 绔彛鍚?MBIM 鍔熻兘鏆撮湶涓€涓?MBIM 鎺ュ彛銆傛帶鍒堕€氶亾绠￠亾鐨勭敤鎴风┖闂?涓€绔槸涓€涓?/dev/wwan0mbim0 瀛楃璁惧銆傚簲鐢ㄧ▼搴忓簲浣跨敤姝ゆ帴鍙ｈ繘琛?MBIM 鍗忚閫氫俊銆?
#### 鍒嗙墖


鐢ㄦ埛绌洪棿搴旂敤绋嬪簭璐熻矗鎸夌収 MBIM 瑙勮寖杩涜鎵€鏈夋帶鍒舵秷鎭殑鍒嗙墖鍜岄噸缁勩€?
#### /dev/wwan0mbim0 write()


鏉ヨ嚜绠＄悊搴旂敤绋嬪簭鐨?MBIM 鎺у埗娑堟伅涓嶅緱瓒呰繃鍗忓晢鐨勬帶鍒舵秷鎭ぇ灏忋€?
#### /dev/wwan0mbim0 read()


绠＄悊搴旂敤绋嬪簭蹇呴』鎺ュ彈鍗忓晢鎺у埗娑堟伅澶у皬鐨勬帶鍒舵秷鎭€?
### MBIM 鏁版嵁閫氶亾鐢ㄦ埛绌洪棿 ABI


#### wwan0-X 缃戠粶璁惧


t7xx 椹卞姩鏆撮湶绫诲瀷涓?"wwan" 鐨?IP 閾捐矾鎺ュ彛 "wwan0-X"锛岀敤浜?IP 娴侀噺銆侷proute 缃戠粶
瀹炵敤绋嬪簭鐢ㄤ簬鍒涘缓 "wwan0-X" 缃戠粶鎺ュ彛锛屽苟灏嗗叾涓?MBIM IP 浼氳瘽鍏宠仈銆?
鐢ㄦ埛绌洪棿绠＄悊搴旂敤绋嬪簭璐熻矗鍦ㄥ缓绔?SessionId 澶т簬 0 鐨?MBIM IP 浼氳瘽涔嬪墠鍒涘缓鏂扮殑 IP 閾捐矾銆?
渚嬪锛屼负 SessionId 涓?1 鐨?MBIM IP 浼氳瘽鍒涘缓鏂扮殑 IP 閾捐矾锛?
  ip link add dev wwan0-1 parentdev wwan0 type wwan linkid 1

璇ラ┍鍔ㄥ皢鑷姩鎶?"wwan0-1" 缃戠粶璁惧鏄犲皠鍒?MBIM IP 浼氳瘽 1銆?
### AT 绔彛鐢ㄦ埛绌洪棿 ABI


#### /dev/wwan0at0 瀛楃璁惧


璇ラ┍鍔ㄩ€氳繃瀹炵幇 AT WWAN 绔彛鏆撮湶涓€涓?AT 绔彛銆傛帶鍒剁鍙ｇ殑鐢ㄦ埛绌洪棿涓€绔槸涓€涓?/dev/wwan0at0 瀛楃璁惧銆傚簲鐢ㄧ▼搴忓簲浣跨敤姝ゆ帴鍙ｅ彂鍑?AT 鍛戒护銆?
### fastboot 绔彛鐢ㄦ埛绌洪棿 ABI


#### /dev/wwan0fastboot0 瀛楃璁惧


璇ラ┍鍔ㄩ€氳繃瀹炵幇 fastboot WWAN 绔彛鏆撮湶涓€涓?fastboot 鍗忚鎺ュ彛銆俧astboot 閫氶亾绠￠亾鐨?鐢ㄦ埛绌洪棿涓€绔槸涓€涓?/dev/wwan0fastboot0 瀛楃璁惧銆傚簲鐢ㄧ▼搴忓簲浣跨敤姝ゆ帴鍙ｈ繘琛?fastboot
鍗忚閫氫俊銆?
璇锋敞鎰忥紝椹卞姩闇€瑕侀噸鏂板姞杞戒互瀵煎嚭 /dev/wwan0fastboot0 绔彛锛屽洜涓鸿澶囧湪杩涘叆 `fastboot_switching`
妯″紡鍚庨渶瑕佸喎澶嶄綅銆?
### ADB 绔彛鐢ㄦ埛绌洪棿 ABI


#### /dev/wwan0adb0 瀛楃璁惧


璇ラ┍鍔ㄩ€氳繃瀹炵幇 ADB WWAN 绔彛鏆撮湶涓€涓?ADB 鍗忚鎺ュ彛銆侫DB 閫氶亾绠￠亾鐨勭敤鎴风┖闂翠竴绔槸涓€涓?/dev/wwan0adb0 瀛楃璁惧銆傚簲鐢ㄧ▼搴忓簲浣跨敤姝ゆ帴鍙ｈ繘琛?ADB 鍗忚閫氫俊銆?
### MIPC 绔彛鐢ㄦ埛绌洪棿 ABI


#### /dev/wwan0mipc0 瀛楃璁惧


璇ラ┍鍔ㄩ€氳繃瀹炵幇 MIPC锛圡odem Information Process Center锛塛WAN 绔彛鏆撮湶涓€涓瘖鏂帴鍙ｃ€?MIPC 閫氶亾绠￠亾鐨勭敤鎴风┖闂翠竴绔槸涓€涓?/dev/wwan0mipc0 瀛楃璁惧銆?搴旂敤绋嬪簭搴斾娇鐢ㄦ鎺ュ彛杩涜 MTK 璋冨埗瑙ｈ皟鍣ㄨ瘖鏂€氫俊銆?
MediaTek 鐨?T700 璋冨埗瑙ｈ皟鍣ㄦ敮鎸?3GPP TS 27.007 [^4^] 瑙勮寖銆?
## 鍙傝€?

[^1^] **MBIM (Mobile Broadband Interface Model) Errata-1**

- https://www.usb.org/document-library/

[^2^] *libmbim "a glib-based library for talking to WWAN modems and devices which
speak the Mobile Interface Broadband Model (MBIM) protocol"*

- http://www.freedesktop.org/wiki/Software/libmbim/

[^3^] *Modem Manager "a DBus-activated daemon which controls mobile broadband
(2G/3G/4G/5G) devices and connections"*

- http://www.freedesktop.org/wiki/Software/ModemManager/

[^4^] **Specification # 27.007 - 3GPP**

- https://www.3gpp.org/DynaReport/27007.htm

[^5^] **fastboot "a mechanism for communicating with bootloaders"**

- https://android.googlesource.com/platform/system/core/+/refs/heads/main/fastboot/README.md

[^6^] *ADB (Android Debug Bridge) "a mechanism to keep track of Android devices
and emulators instances connected to or running on a given host developer
machine with ADB protocol"*

- https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/main/README.md
