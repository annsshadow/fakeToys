## SMARTPQI - Microchip Smart Storage SCSI 椹卞姩


鏈枃浠舵弿杩颁簡 Microchip (http://www.microchip.com) PQI 鎺у埗鍣ㄧ殑 smartpqi SCSI 椹卞姩銆俿martpqi 椹卞姩
鏄?Microchip Corp. 鐨勬柊涓€浠?SCSI 椹卞姩锛屼篃鏄涓疄鐜?PQI 闃熷垪妯″瀷鐨?SCSI 椹卞姩銆?
smartpqi 椹卞姩灏嗗彇浠?Adaptec Series 9 鎺у埗鍣ㄧ殑 aacraid 椹卞姩銆備娇鐢?Adaptec Series 9 鎺у埗鍣ㄤ笖杩愯杈冩棫
鍐呮牳锛?.9 涔嬪墠锛夌殑瀹㈡埛蹇呴』閰嶇疆 smartpqi 椹卞姩锛屽惁鍒欏叾鍗峰皢涓嶄細琚坊鍔犲埌鎿嶄綔绯荤粺銆?
瑕佽幏寰?Microchip smartpqi 鎺у埗鍣ㄧ殑鏀寔锛岃鍦ㄩ厤缃唴鏍告椂鍚敤 smartpqi 椹卞姩銆?
鏈夊叧 PQI 闃熷垪鎺ュ彛鐨勬洿澶氫俊鎭紝璇峰弬瑙侊細

- http://www.t10.org/drafts.htm
- http://www.t10.org/members/w_pqi2.htm

## 鏀寔鐨勮澶?
<Controller names to be added as they become publicly available.>

## /sys 涓?smartpqi 涓撶敤鐨勬潯鐩?

### smartpqi 涓绘満灞炴€?
  - /sys/class/scsi_host/host*/rescan
  - /sys/class/scsi_host/host*/driver_version

  host rescan 灞炴€ф槸涓€涓彧鍐欏睘鎬с€傚悜璇ュ睘鎬у啓鍏ュ皢瑙﹀彂椹卞姩鎵弿鏂板銆佹洿鏀规垨绉婚櫎鐨?  璁惧锛屽苟閫氱煡 SCSI 涓棿灞傛墍妫€娴嬪埌鐨勪换浣曞彉鍖栥€?
  version 灞炴€ф槸鍙鐨勶紝灏嗚繑鍥為┍鍔ㄧ増鏈笌鎺у埗鍣ㄥ浐浠剁増鏈€?```

              driver: 0.9.13-370
              firmware: 0.01-522

```
### smartpqi sas 璁惧灞炴€?
  HBA 璁惧浼氳娣诲姞鍒?SAS 浼犺緭灞傘€傝繖浜涘睘鎬х敱 SAS 浼犺緭灞傝嚜鍔ㄦ坊鍔犮€?
  /sys/class/sas_device/end_device-X:X/sas_address
  /sys/class/sas_device/end_device-X:X/enclosure_identifier
  /sys/class/sas_device/end_device-X:X/scsi_target_id

## smartpqi 涓撶敤鐨?ioctls


  涓轰簡涓庝负 cciss 鍗忚缂栧啓鐨勫簲鐢ㄧ▼搴忎繚鎸佸吋瀹广€?
  CCISS_DEREGDISK, CCISS_REGNEWDISK, CCISS_REGNEWD
	涓婅堪涓変釜 ioctl 閮芥墽琛屽畬鍏ㄧ浉鍚岀殑鎿嶄綔锛屽嵆璁╅┍鍔ㄩ噸鏂版壂鎻忔柊璁惧銆傝繖涓庡啓鍏?	smartpqi 涓撶敤鐨勪富鏈?鈥渞escan鈥?灞炴€т綔鐢ㄥ畬鍏ㄧ浉鍚屻€?
  CCISS_GETPCIINFO
	杩斿洖 PCI 鍩熴€佹€荤嚎銆佽澶囧拰鍔熻兘浠ュ強 鈥渂oard ID鈥濓紙PCI 瀛愮郴缁?ID锛夈€?
  CCISS_GETDRIVVER
```

	  (DRIVER_MAJOR << 28) | (DRIVER_MINOR << 24) | (DRIVER_RELEASE << 16) | DRIVER_REVISION;

  CCISS_PASSTHRU
	鍏佽灏?鈥淏MIC鈥?鍜?鈥淐ISS鈥?鍛戒护閫忎紶鍒?Smart Storage Array銆?	杩欎簺鍛戒护琚?SSA Array Configuration Utility銆丼NMP 瀛樺偍浠ｇ悊绛夊箍娉涗娇鐢ㄣ€?
```
