## I2C/SMBus Functionality


### 绠€浠?


鐢变簬骞堕潪姣忎釜 I2C 鎴?SMBus 閫傞厤鍣ㄩ兘瀹炵幇浜?I2C 瑙勮寖涓殑鍏ㄩ儴鍐呭锛屽洜姝ゅ綋涓€涓鎴风鑾峰緱
鎸傝浇鍒版煇涓€傞厤鍣ㄧ殑閫夐」鏃讹紝瀹冧笉鑳戒俊浠昏嚜韬墍闇€鐨勫叏閮ㄥ姛鑳介兘宸茶瀹炵幇锛氬鎴风闇€瑕佹煇绉嶆柟寮忔潵
妫€鏌ラ€傞厤鍣ㄦ槸鍚﹀叿澶囨墍闇€鐨勫姛鑳姐€?


### 鍔熻兘甯搁噺


鏈夊叧鏈€鏂扮増鐨勫姛鑳藉父閲忓垪琛紝璇锋煡鐪?<uapi/linux/i2c.h>锛?

  =============================== ==============================================
  I2C_FUNC_I2C                    绾?i2c 绾у懡浠わ紙绾?SMBus
                                  閫傞厤鍣ㄩ€氬父鏃犳硶鎵ц杩欎簺鍛戒护锛?
  I2C_FUNC_10BIT_ADDR             澶勭悊 10 浣嶅湴鍧€鎵╁睍
  I2C_FUNC_PROTOCOL_MANGLING      浜嗚В I2C_M_IGNORE_NAK銆?
                                  I2C_M_REV_DIR_ADDR 鍜?I2C_M_NO_RD_ACK
                                  鏍囧織锛堣繖浜涗細淇敼 I2C 鍗忚锛侊級
  I2C_FUNC_NOSTART                鍙互璺宠繃 repeated start 搴忓垪
  I2C_FUNC_SMBUS_QUICK            澶勭悊 SMBus write_quick 鍛戒护
  I2C_FUNC_SMBUS_READ_BYTE        澶勭悊 SMBus read_byte 鍛戒护
  I2C_FUNC_SMBUS_WRITE_BYTE       澶勭悊 SMBus write_byte 鍛戒护
  I2C_FUNC_SMBUS_READ_BYTE_DATA   澶勭悊 SMBus read_byte_data 鍛戒护
  I2C_FUNC_SMBUS_WRITE_BYTE_DATA  澶勭悊 SMBus write_byte_data 鍛戒护
  I2C_FUNC_SMBUS_READ_WORD_DATA   澶勭悊 SMBus read_word_data 鍛戒护
  I2C_FUNC_SMBUS_WRITE_WORD_DATA  澶勭悊 SMBus write_byte_data 鍛戒护
  I2C_FUNC_SMBUS_PROC_CALL        澶勭悊 SMBus process_call 鍛戒护
  I2C_FUNC_SMBUS_READ_BLOCK_DATA  澶勭悊 SMBus read_block_data 鍛戒护
  I2C_FUNC_SMBUS_WRITE_BLOCK_DATA 澶勭悊 SMBus write_block_data 鍛戒护
  I2C_FUNC_SMBUS_READ_I2C_BLOCK   澶勭悊 SMBus read_i2c_block_data 鍛戒护
  I2C_FUNC_SMBUS_WRITE_I2C_BLOCK  澶勭悊 SMBus write_i2c_block_data 鍛戒护
  =============================== ==============================================

涓婇潰杩欎簺鏍囧織鐨勪竴浜涚粍鍚堜篃涓轰簡浣犵殑鏂逛究鑰屽畾涔夛細

  =========================       ======================================
  I2C_FUNC_SMBUS_BYTE             澶勭悊 SMBus read_byte
                                  涓?write_byte 鍛戒护
  I2C_FUNC_SMBUS_BYTE_DATA        澶勭悊 SMBus read_byte_data
                                  涓?write_byte_data 鍛戒护
  I2C_FUNC_SMBUS_WORD_DATA        澶勭悊 SMBus read_word_data
                                  涓?write_word_data 鍛戒护
  I2C_FUNC_SMBUS_BLOCK_DATA       澶勭悊 SMBus read_block_data
                                  涓?write_block_data 鍛戒护
  I2C_FUNC_SMBUS_I2C_BLOCK        澶勭悊 SMBus read_i2c_block_data
                                  涓?write_i2c_block_data 鍛戒护
  I2C_FUNC_SMBUS_EMUL             澶勭悊鎵€鏈夊彲鐢辩湡瀹?I2C 閫傞厤鍣ㄦā鎷熺殑
                                  SMBus 鍛戒护锛堜娇鐢ㄩ€忔槑鐨?
                                  妯℃嫙灞傦級
  =========================       ======================================

鍦?3.5 涔嬪墠鐨勫収鏍哥増鏈腑锛孖2C_FUNC_NOSTART 鏄綔涓?
I2C_FUNC_PROTOCOL_MANGLING 鐨勪竴閮ㄥ垎瀹炵幇鐨勩€?


### 閫傞厤鍣ㄥ疄鐜?


褰撲綘缂栧啓涓€涓柊鐨勯€傞厤鍣ㄩ┍鍔ㄦ椂锛屼綘灏嗕笉寰椾笉瀹炵幇涓€涓悕涓?`functionality` 鐨勫嚱鏁板洖璋冦€?
鍏稿瀷鐨勫疄鐜板涓嬫墍绀恒€?

涓€涓吀鍨嬬殑浠呮敮鎸?SMBus 鐨勯€傞厤鍣ㄤ細鍒楀嚭瀹冩敮鎸佺殑鎵€鏈?SMBus 浜嬪姟
```

  static u32 piix4_func(struct i2c_adapter *adapter)
  {
	return I2C_FUNC_SMBUS_QUICK | I2C_FUNC_SMBUS_BYTE |
	       I2C_FUNC_SMBUS_BYTE_DATA | I2C_FUNC_SMBUS_WORD_DATA |
	       I2C_FUNC_SMBUS_BLOCK_DATA;
  }

```
涓€涓吀鍨嬬殑瀹屾暣 I2C 閫傞厤鍣ㄤ細浣跨敤浠ヤ笅鍐呭锛堟潵鑷?i2c-pxa
```

  static u32 i2c_pxa_functionality(struct i2c_adapter *adap)
  {
	return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
  }

```
I2C_FUNC_SMBUS_EMUL 鍖呭惈浜?i2c-core 鍙互鍦ㄦ棤闇€閫傞厤鍣ㄩ┍鍔ㄥ府鍔╃殑鎯呭喌涓嬨€佷娇鐢?
I2C_FUNC_I2C 妯℃嫙鐨勬墍鏈?SMBus 浜嬪姟锛堝鍔?I2C 鍧椾簨鍔★級銆傚叾鎬濇兂鏄瀹㈡埛绔┍鍔ㄦ鏌?
瀵?SMBus 鍔熻兘鐨勬敮鎸侊紝鑰屾棤闇€鍏冲績杩欎簺鍔熻兘鏄敱閫傞厤鍣ㄥ湪纭欢涓疄鐜帮紝杩樻槸鐢?i2c-core
鍦?I2C 閫傞厤鍣ㄤ箣涓婁互杞欢妯℃嫙銆?


### 瀹㈡埛绔鏌?


鍦ㄥ鎴风灏濊瘯鎸傝浇鍒版煇涓€傞厤鍣ㄤ箣鍓嶏紝鐢氳嚦鍦ㄦ墽琛屾祴璇曚互妫€鏌ュ畠鎵€鏀寔鐨勬煇涓澶囨槸鍚﹀嚭鐜板湪
閫傞厤鍣ㄤ笂涔嬪墠锛屽畠搴旇妫€鏌ユ墍闇€鐨勫姛鑳芥槸鍚﹀瓨鍦ㄣ€傚吀鍨嬬殑鏂瑰紡鏄?
```

  static int lm75_detect(...)
  {
	(...)
	if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA |
				     I2C_FUNC_SMBUS_WORD_DATA))
		goto exit;
	(...)
  }

```
杩欓噷锛宭m75 椹卞姩妫€鏌ラ€傞厤鍣ㄦ槸鍚﹁兘澶熷悓鏃舵墽琛?SMBus byte data 鍜?SMBus word data 浜嬪姟銆?
濡傛灉涓嶈兘锛岄偅涔堣椹卞姩灏嗘棤娉曞湪姝ら€傞厤鍣ㄤ笂宸ヤ綔锛岀户缁笅鍘讳篃娌℃湁鎰忎箟銆傚鏋滀笂杩版鏌ユ垚鍔燂紝
椹卞姩渚跨煡閬撳畠鍙互璋冪敤浠ヤ笅鍑芥暟锛歩2c_smbus_read_byte_data()銆乮2c_smbus_write_byte_data()銆?
i2c_smbus_read_word_data() 鍜?i2c_smbus_write_word_data()銆備綔涓虹粡楠屾硶鍒欙紝浣犻€氳繃
i2c_check_functionality() 娴嬭瘯鐨勫姛鑳藉父閲忥紝搴斿綋涓庝綘椹卞姩鎵€璋冪敤鐨?i2c_smbus_* 鍑芥暟
绮剧‘鍖归厤銆?

娉ㄦ剰锛屼笂杩版鏌ュ苟涓嶈兘璇存槑杩欎簺鍔熻兘鏄敱搴曞眰閫傞厤鍣ㄥ湪纭欢涓疄鐜帮紝杩樻槸鐢?i2c-core 鍦?
杞欢涓ā鎷熴€傚鎴风椹卞姩鏃犻渶鍏冲績杩欎竴鐐癸紝鍥犱负 i2c-core 浼氶€忔槑鍦板湪 I2C 閫傞厤鍣ㄤ箣涓?
瀹炵幇 SMBus 浜嬪姟銆?


### 閫氳繃 /DEV 妫€鏌?


濡傛灉浣犲皾璇曚粠鐢ㄦ埛绌洪棿绋嬪簭璁块棶鏌愪釜閫傞厤鍣紝浣犲皢涓嶅緱涓嶄娇鐢?/dev 鎺ュ彛銆傚綋鐒讹紝浣犱粛鐒堕渶瑕?
妫€鏌ユ墍闇€鐨勫姛鑳芥槸鍚﹀彈鏀寔銆傝繖閫氳繃 I2C_FUNCS ioctl 瀹屾垚銆備笅闈竴涓敼缂栬嚜 i2cdetect
绋嬪簭鐨勭ず渚嬶細
```

  int file;
  if (file = open("/dev/i2c-0", O_RDWR) < 0) {
	/* Some kind of error handling */
	exit(1);
  }
  if (ioctl(file, I2C_FUNCS, &funcs) < 0) {
	/* Some kind of error handling */
	exit(1);
  }
  if (!(funcs & I2C_FUNC_SMBUS_QUICK)) {
	/* Oops, the needed functionality (SMBus write_quick function) is
           not available! */
	exit(1);
  }
  /* Now it is safe to use the SMBus write_quick command */

```