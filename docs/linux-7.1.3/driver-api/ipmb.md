## 闈㈠悜鍗槦 MC 鐨?IPMB 椹卞姩


鏅鸿兘骞冲彴绠＄悊鎬荤嚎锛圛ntelligent Platform Management Bus锛孖PMB锛夋槸涓€鏉?I2C 鎬荤嚎锛屾彁渚涙満绠卞唴
涓嶅悓鏉垮崱涔嬮棿鐨勬爣鍑嗗寲浜掕繛銆傝繖绉嶄簰杩炰綅浜庡熀鏉跨鐞嗭紙BMC锛変笌鏈虹鐢靛瓙鍏冧欢涔嬮棿銆侷PMB 涔熶笌閫氳繃
IPMB 鎬荤嚎鐨勬秷鎭崗璁浉鍏宠仈銆?
浣跨敤 IPMB 鐨勮澶囬€氬父鏄墽琛岀鐞嗗姛鑳界殑绠＄悊鎺у埗鍣紝渚嬪缁存姢鍓嶉潰鏉挎帴鍙ｃ€佺洃鎺у熀鏉裤€?鍦ㄧ郴缁熸満绠变腑鐑彃鎷旂鐩橀┍鍔ㄥ櫒绛夈€?
褰撶郴缁熶腑瀹炵幇浜?IPMB 鏃讹紝BMC 鍏呭綋鎺у埗鍣紝涓虹郴缁熻蒋浠舵彁渚涘 IPMB 鐨勮闂€侭MC 閫氳繃 IPMB 鍚戣澶?锛堥€氬父鏄崼鏄熺鐞嗘帶鍒跺櫒锛孲atellite Management Controller 鎴?Satellite MC锛夊彂閫?IPMI 璇锋眰锛?璁惧鍒欏皢鍝嶅簲鍙戝洖缁?BMC銆?
鏈夊叧 IPMB 涓?IPMB 娑堟伅鏍煎紡鐨勬洿澶氫俊鎭紝璇峰弬鑰?IPMB 涓?IPMI 瑙勮寖銆?
### 闈㈠悜鍗槦 MC 鐨?IPMB 椹卞姩


ipmb-dev-int - 杩欐槸鍗槦 MC 涓婇渶瑕佺殑椹卞姩锛岀敤浜庝粠 BMC 鎺ユ敹 IPMB 娑堟伅骞跺彂鍥炲搷搴斻€傝椹卞姩涓?I2C 椹卞姩
浠ュ強涓€涓敤鎴风┖闂寸▼搴忥紙濡?OpenIPMI锛夐厤鍚堝伐浣滐細

1) 瀹冩槸涓€涓?I2C 浠庢満鍚庣椹卞姩銆傚洜姝わ紝瀹冨畾涔変簡涓€涓洖璋冨嚱鏁帮紝灏嗗崼鏄?MC 璁剧疆涓?I2C 浠庢満銆?   璇ュ洖璋冨嚱鏁板鐞嗘帴鏀跺埌鐨?IPMI 璇锋眰銆?
2) 瀹冨畾涔変簡璇诲啓鍑芥暟锛屼娇鐢ㄦ埛绌洪棿绋嬪簭锛堝 OpenIPMI锛夎兘澶熶笌鍐呮牳閫氫俊銆?
### 鍔犺浇 IPMB 椹卞姩


璇ラ┍鍔ㄩ渶瑕佸湪鍚姩鏃舵垨鎵嬪姩棣栧厛鍔犺浇銆傞鍏堬紝纭繚浣犵殑閰嶇疆鏂囦欢涓寘鍚互涓嬪唴瀹癸細
CONFIG_IPMB_DEVICE_INTERFACE=y

1) 濡傛灉浣犲笇鏈涢┍鍔ㄥ湪鍚姩鏃跺姞杞斤細

```

     Device (SMB0) // 绀轰緥 SMBus 涓绘満鎺у埗鍣?     {
     Name (_HID, "<Vendor-Specific HID>") // 鍘傚晢鐗瑰畾鐨?HID
     Name (_UID, 0) // 鐗瑰畾涓绘満鎺у埗鍣ㄧ殑鍞竴 ID
     :
     :
       Device (IPMB)
       {
         Name (_HID, "IPMB0001") // IPMB 璁惧鎺ュ彛
         Name (_UID, 0) // 鍞竴璁惧鏍囪瘑绗?       }
     }

```
```

     &i2c2 {
            status = "okay";

            ipmb@10 {
                    compatible = "ipmb-dev";
                    reg = <0x10>;
                    i2c-protocol;
            };
     };

```
濡傛灉瑕佷娇鐢ㄥ師濮?i2c 鍧楄€岄潪 smbus 鏉ヤ紶杈撴暟鎹紝鍒欓渶瑕佸涓婂畾涔?"i2c-protocol"銆?
```

     modprobe ipmb-dev-int


```
### 瀹炰緥鍖栬澶?

鍔犺浇椹卞姩鍚庯紝浣犲彲浠ユ寜鐓?'Documentation/i2c/instantiating-devices.rst' 涓墍杩板疄渚嬪寲璁惧銆傚鏋滀綘鏈夊涓?BMC锛屾瘡涓兘閫氳繃涓嶅悓鐨?I2C 鎬荤嚎杩炴帴鍒颁綘鐨勫崼鏄?MC锛屼綘鍙互涓烘瘡涓?BMC 瀹炰緥鍖栦竴涓澶囥€?
瀹炰緥鍖栬澶囩殑鍚嶇О鍖呭惈 I2C 鎬荤嚎缂栧彿
```

  BMC1 ------ IPMB/I2C bus 1 ---------|   /dev/ipmb-1
				Satellite MC
  BMC1 ------ IPMB/I2C bus 2 ---------|   /dev/ipmb-2

```
渚嬪锛屼綘鍙互浠庝互涓嬫柟寮忓疄渚嬪寲 ipmb-dev-int 璁惧
```

  # echo ipmb-dev 0x1010 > /sys/bus/i2c/devices/i2c-2/new_device

```
杩欏皢鍒涘缓璁惧鏂囦欢 /dev/ipmb-2锛岀敤鎴风┖闂寸▼搴忓彲浠ヨ闂畠銆傝璁惧闇€瑕佸湪杩愯鐢ㄦ埛绌洪棿绋嬪簭涔嬪墠瀹炰緥鍖栥€?