
## AMD 杈瑰甫锛圫IDE BAND锛夋帴鍙?

閮ㄥ垎鍩轰簬 AMD Zen 鐨勫鐞嗗櫒閫氳繃绉颁负楂樼骇骞冲彴绠＄悊閾捐矾锛圓PML, Advanced Platform
Management Link锛夌殑杈瑰甫鎺ュ彛锛圫BI锛夋敮鎸佺郴缁熺鐞嗗姛鑳姐€侫PML 鏄竴涓熀浜?I2C/I3C 鐨?涓ょ嚎澶勭悊鍣ㄧ洰鏍囨帴鍙ｃ€侫PML 鐢ㄤ簬涓庤繙绋嬬鐞嗘帴鍙ｏ紙SB 杩滅▼绠＄悊鎺ュ彛锛圫B-RMI锛変笌 SB 娓╁害
浼犳劅鍣ㄦ帴鍙ｏ紙SB-TSI锛夛級閫氫俊銆?
鍏充簬璇ユ帴鍙ｇ殑鏇村缁嗚妭鍙互鍦ㄥ鏃?鍨嬪彿 PPR [^1^]_ 鐨勨€? Advanced Platform Management
Link (APML)鈥濈珷鑺備腑鎵惧埌銆?

## SBRMI 璁惧


drivers/misc/amd-sbi 涓嬬殑 apml_sbrmi 椹卞姩鍒涘缓 miscdevice /dev/sbrmi-*锛屼互璁╃敤鎴?绌洪棿绋嬪簭杩愯 APML mailbox銆丆PUID銆丮CAMSR 涓?register xfer 鍛戒护銆?
瀵勫瓨鍣ㄩ泦鍦?APML 鍗忚涔嬮棿鏄€氱敤鐨勩€侷OCTL 鍦ㄥ崗璁箣闂存彁渚涘悓姝ワ紝鍥犱负浜嬪姟鍙兘浜х敓
绔炰簤鏉′欢銆?

   $ ls -al /dev/sbrmi-3c
   crw-------    1 root     root       10,  53 Jul 10 11:13 /dev/sbrmi-3c

apml_sbrmi 椹卞姩娉ㄥ唽 hwmon 浼犳劅鍣紝鐢ㄤ簬鐩戞帶 power_cap_max銆佸綋鍓嶅姛鑰椾互鍙婄鐞?power_cap銆?
dev 鑺傜偣鐨勭壒鎬э細
 - 瀹氫箟浜嗕笉鍚岀殑 xfer 鍗忚锛? - Mailbox
 - CPUID
 - MCA_MSR
 - Register xfer

璁块棶闄愬埗锛? - 鍙湁 root 鐢ㄦ埛鎵嶅厑璁告墦寮€璇ユ枃浠躲€? - APML Mailbox 娑堟伅涓?Register xfer 璁块棶鏄彲璇诲啓鐨勶紝
 - CPUID 涓?MCA_MSR 璁块棶鏄彧璇荤殑銆?
## 椹卞姩 IOCTL


   :doc: SBRMI_IOCTL_MBOX_CMD
   :doc: SBRMI_IOCTL_CPUID_CMD
   :doc: SBRMI_IOCTL_MCAMSR_CMD
   :doc: SBRMI_IOCTL_REG_XFER_CMD

## 鐢ㄦ埛绌洪棿鐢ㄦ硶


浠?C 绋嬪簭璁块棶杈瑰甫鎺ュ彛銆?```

  #include <uapi/misc/amd-apml.h>

```
鍏朵腑瀹氫箟浜嗗彈鏀寔鐨?IOCTL 浠ュ強瑕佷粠鐢ㄦ埛绌洪棿浼犲叆鐨勬暟鎹粨鏋勩€?```

  int file;

  file = open("/dev/sbrmi-*", O_RDWR);
  if (file < 0) {
    /* 閿欒澶勭悊 */
    exit(1);
  }

```
瀹氫箟浜嗕互涓?IOCTL锛?
`#define SB_BASE_IOCTL_NR      	0xF9`
`#define SBRMI_IOCTL_MBOX_CMD		_IOWR(SB_BASE_IOCTL_NR, 0, struct apml_mbox_msg)`
`#define SBRMI_IOCTL_CPUID_CMD		_IOWR(SB_BASE_IOCTL_NR, 1, struct apml_cpuid_msg)`
`#define SBRMI_IOCTL_MCAMSR_CMD	_IOWR(SB_BASE_IOCTL_NR, 2, struct apml_mcamsr_msg)`
`#define SBRMI_IOCTL_REG_XFER_CMD	_IOWR(SB_BASE_IOCTL_NR, 3, struct apml_reg_xfer_msg)`


鐢ㄦ埛绌洪棿 C-API 鐢?esmi_oob_library 鎻愪緵锛屾墭绠′簬 [^2^]_锛岀敱 E-SMS 椤圭洰 [^3^]_ 鎻愪緵銆?