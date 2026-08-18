## 鍦ㄧ敤鎴风┖闂村疄鐜?I2C 璁惧椹卞姩


閫氬父锛孖2C 璁惧鐢卞唴鏍搁┍鍔ㄦ帶鍒躲€備絾涔熷彲浠ラ€氳繃 /dev 鎺ュ彛锛屼粠鐢ㄦ埛绌洪棿璁块棶閫傞厤鍣ㄤ笂鐨勬墍鏈夎澶囥€備负姝や綘闇€瑕佸姞杞?i2c-dev 妯″潡銆?
姣忎釜娉ㄥ唽鐨?I2C 閫傞厤鍣ㄩ兘浼氳幏寰椾竴涓粠 0 寮€濮嬭鏁扮殑缂栧彿銆備綘鍙互鏌ョ湅 /sys/class/i2c-dev/ 鏉ヤ簡瑙ｅ摢涓紪鍙峰搴斿摢涓€傞厤鍣ㄣ€傛垨鑰咃紝浣犲彲浠ヨ繍琛?鈥渋2cdetect -l鈥?鏉ヨ幏鍙栫郴缁熷湪鏌愪竴鏃跺埢瀛樺湪鐨勬墍鏈?I2C 閫傞厤鍣ㄧ殑鏍煎紡鍖栧垪琛ㄣ€俰2cdetect 鏄?i2c-tools 杞欢鍖呯殑涓€閮ㄥ垎銆?
I2C 璁惧鏂囦欢鏄富璁惧鍙蜂负 89銆佹璁惧鍙峰搴斾簬涓婅堪鍒嗛厤缂栧彿鐨勫瓧绗﹁澶囨枃浠躲€傚畠浠簲琚О涓?鈥渋2c-%d鈥濓紙i2c-0銆乮2c-1銆佲€︹€︺€乮2c-10銆佲€︹€︼級銆傛墍鏈?256 涓璁惧鍙烽兘淇濈暀缁?I2C 浣跨敤銆?
## C 绀轰緥


閭ｄ箞鍋囪浣犲笇鏈涗粠涓€涓?C 绋嬪簭璁块棶鏌愪釜 I2C 閫傞厤鍣ㄣ€?```

  #include <linux/i2c-dev.h>
  #include <i2c/smbus.h>

```
鐜板湪锛屼綘蹇呴』鍐冲畾瑕佽闂摢涓€傞厤鍣ㄣ€備綘搴旀鏌?/sys/class/i2c-dev/ 鎴栬繍琛?鈥渋2cdetect -l鈥?鏉ュ喅瀹氥€傞€傞厤鍣ㄧ紪鍙风殑鍒嗛厤鏈変簺鍔ㄦ€侊紝鍥犳浣犱笉鑳藉鍏跺仛澶鍋囪銆傚畠浠敋鑷冲彲鑳藉湪涓ゆ鍚姩涔嬮棿鍙戠敓鍙樺寲銆?
```

  int file;
  int adapter_nr = 2; /* 鍙兘鍔ㄦ€佺‘瀹?*/
  char filename[20];

  snprintf(filename, 19, "/dev/i2c-%d", adapter_nr);
  file = open(filename, O_RDWR);
  if (file < 0) {
    /* 閿欒澶勭悊锛涗綘鍙互妫€鏌?errno 浜嗚В鍑洪敊鍘熷洜 */
    exit(1);
  }

```
褰撲綘鎵撳紑璁惧鍚庯紝蹇呴』鎸囧畾瑕佷笌涔嬮€氫俊鐨勮澶?```

  int addr = 0x40; /* I2C 鍦板潃 */

  if (ioctl(file, I2C_SLAVE, addr) < 0) {
    /* 閿欒澶勭悊锛涗綘鍙互妫€鏌?errno 浜嗚В鍑洪敊鍘熷洜 */
    exit(1);
  }

```
濂戒簡锛岀幇鍦ㄤ竴鍒囬兘鍑嗗灏辩华銆備綘鍙互浣跨敤 SMBus 鍛戒护鎴栫函 I2C 涓庝綘鐨勮澶囬€氫俊銆傚鏋滃彲鑳斤紝浼樺厛浣跨敤 SMBus 鍛戒护
```

  __u8 reg = 0x10; /* 瑕佽闂殑璁惧瀵勫瓨鍣?*/
  __s32 res;
  char buf[10];

  /* 浣跨敤 SMBus 鍛戒护 */
  res = i2c_smbus_read_word_data(file, reg);
  if (res < 0) {
    /* 閿欒澶勭悊锛欼2C 浜嬪姟澶辫触 */
  } else {
    /* res 鍖呭惈璇诲彇鍒扮殑瀛?*/
  }

  /*
   * 浣跨敤 I2C 鍐欙紝绛変环浜?   * i2c_smbus_write_word_data(file, reg, 0x6543)
   */
  buf[0] = reg;
  buf[1] = 0x43;
  buf[2] = 0x65;
  if (write(file, buf, 3) != 3) {
    /* 閿欒澶勭悊锛欼2C 浜嬪姟澶辫触 */
  }

  /* 浣跨敤 I2C 璇伙紝绛変环浜?i2c_smbus_read_byte(file) */
  if (read(file, buf, 1) != 1) {
    /* 閿欒澶勭悊锛欼2C 浜嬪姟澶辫触 */
  } else {
    /* buf[0] 鍖呭惈璇诲彇鍒扮殑瀛楄妭 */
  }

```
娉ㄦ剰锛屽彧鏈夐€氳繃 read() 鍜?write() 璋冪敤鎵嶈兘瀹炵幇 I2C 鍜?SMBus 鍗忚鐨勪竴涓瓙闆嗐€傜壒鍒槸锛屾墍璋撶殑缁勫悎浜嬪姟锛堝湪鍚屼竴浜嬪姟涓贩鍚堣鍐欐秷鎭級涓嶈鏀寔銆傚洜姝わ紝杩欎釜鎺ュ彛鍑犱箮浠庝笉琚敤鎴风┖闂寸▼搴忎娇鐢ㄣ€?
閲嶈锛氱敱浜庝娇鐢ㄤ簡鍐呰仈鍑芥暟锛岀紪璇戜綘鐨勭▼搴忔椂**蹇呴』**浣跨敤 鈥?O鈥?鎴栧叾鏌愮鍙樹綋锛?
## 瀹屾暣鎺ュ彛鎻忚堪


瀹氫箟浜嗕互涓?IOCTL锛?
`ioctl(file, I2C_SLAVE, long addr)`
  鏇存敼浠庤澶囧湴鍧€銆傚湴鍧€閫氳繃鍙傛暟鐨勪綆 7 浣嶄紶鍏ワ紙10 浣嶅湴鍧€闄ゅ锛屾鏃堕€氳繃浣?10 浣嶄紶鍏ワ級銆?
`ioctl(file, I2C_TENBIT, long select)`
  濡傛灉 select 涓嶇瓑浜?0锛屽垯閫夋嫨 10 浣嶅湴鍧€锛涘鏋?select 绛変簬 0锛屽垯閫夋嫨鏅€?7 浣嶅湴鍧€銆傞粯璁?0銆傛璇锋眰浠呭湪閫傞厤鍣ㄥ叿鏈?I2C_FUNC_10BIT_ADDR 鏃舵墠鏈夋晥銆?
`ioctl(file, I2C_PEC, long select)`
  濡傛灉 select 涓嶇瓑浜?0锛屽垯閫夋嫨鐢熸垚骞舵牎楠?SMBus PEC锛堝寘閿欒妫€鏌ワ級锛涘鏋?select 绛変簬 0锛屽垯绂佺敤銆傞粯璁?0銆備粎鐢ㄤ簬 SMBus 浜嬪姟銆傛璇锋眰浠呭湪閫傞厤鍣ㄥ叿鏈?I2C_FUNC_SMBUS_PEC 鏃舵墠璧蜂綔鐢紱鍗充究娌℃湁涔熶粛鐒跺畨鍏紝鍙槸娌℃湁浠讳綍鏁堟灉銆?
`ioctl(file, I2C_FUNCS, unsigned long *funcs)`
  鑾峰彇閫傞厤鍣ㄥ姛鑳藉苟鏀惧叆 `*funcs`銆?
`ioctl(file, I2C_RDWR, struct i2c_rdwr_ioctl_data *msgset)`
  鎵ц缁勫悎璇?鍐欎簨鍔★紝涓棿涓嶅彂閫佸仠姝紙stop锛夈€備粎褰撻€傞厤鍣ㄥ叿鏈?I2C_FUNC_I2C 鏃舵墠鏈夋晥銆傚弬鏁版槸
```

    struct i2c_rdwr_ioctl_data {
      struct i2c_msg *msgs;  /* 鎸囧悜绠€鍗曟秷鎭暟缁勭殑鎸囬拡 */
      int nmsgs;             /* 瑕佷氦鎹㈢殑娑堟伅鏁伴噺 */
    }

  杩欎簺 msgs[] 鑷韩鍚湁鎸囧悜鏁版嵁缂撳啿鍖虹殑杩涗竴姝ユ寚閽堛€傚嚱鏁颁細鏍规嵁鐗瑰畾娑堟伅涓槸鍚﹁缃簡 I2C_M_RD 鏍囧織锛屽悜杩欎簺缂撳啿鍖哄啓鍏ユ垨浠庡叾涓鍙栨暟鎹€備粠璁惧鍦板潃浠ュ強鏄惁浣跨敤 10 浣嶅湴鍧€妯″紡蹇呴』鍦ㄦ瘡鏉℃秷鎭腑璁剧疆锛岃鐩栦笂杩?ioctl 璁剧疆鐨勫€笺€?
```
`ioctl(file, I2C_SMBUS, struct i2c_smbus_ioctl_data *args)`
  濡傛灉鍙兘锛岃浣跨敤涓嬮潰鎻忚堪鐨?`i2c_smbus_*` 鏂规硶锛岃€屼笉鏄洿鎺ュ彂鍑?ioctl銆?
浣犲彲浠ヤ娇鐢?read(2) 鍜?write(2) 璋冪敤鎵ц绾?I2C 浜嬪姟銆備綘鏃犻渶浼犻€掑湴鍧€瀛楄妭锛涚浉鍙嶏紝鍦ㄥ皾璇曡闂澶囦箣鍓嶉€氳繃 ioctl I2C_SLAVE 璁剧疆瀹冦€?
浣犲彲浠ユ墽琛?SMBus 绾т簨鍔★紙鍙傝鏂囨。鏂囦欢 smbus-protocol.rst
```

  __s32 i2c_smbus_write_quick(int file, __u8 value);
  __s32 i2c_smbus_read_byte(int file);
  __s32 i2c_smbus_write_byte(int file, __u8 value);
  __s32 i2c_smbus_read_byte_data(int file, __u8 command);
  __s32 i2c_smbus_write_byte_data(int file, __u8 command, __u8 value);
  __s32 i2c_smbus_read_word_data(int file, __u8 command);
  __s32 i2c_smbus_write_word_data(int file, __u8 command, __u16 value);
  __s32 i2c_smbus_process_call(int file, __u8 command, __u16 value);
  __s32 i2c_smbus_block_process_call(int file, __u8 command, __u8 length,
                                     __u8 *values);
  __s32 i2c_smbus_read_block_data(int file, __u8 command, __u8 *values);
  __s32 i2c_smbus_write_block_data(int file, __u8 command, __u8 length,
                                   __u8 *values);

```
鎵€鏈夎繖浜涗簨鍔″湪澶辫触鏃惰繑鍥?-1锛涗綘鍙互璇诲彇 errno 浜嗚В鍙戠敓浜嗕粈涔堛€傗€樺啓鈥欎簨鍔″湪鎴愬姛鏃惰繑鍥?0锛涒€樿鈥欎簨鍔¤繑鍥炶鍙栧埌鐨勫€硷紝浣?read_block 渚嬪锛屽畠杩斿洖璇诲彇鍒扮殑鍊肩殑鏁伴噺銆傚潡缂撳啿鍖轰笉蹇呴暱浜?32 瀛楄妭銆?
涓婅堪鍑芥暟閫氳繃閾炬帴 libi2c 搴撴彁渚涳紝璇ュ簱鐢?i2c-tools 椤圭洰鎻愪緵銆傚弬瑙侊細
https://git.kernel.org/pub/scm/utils/i2c-tools/i2c-tools.git/銆?
## 瀹炵幇缁嗚妭


瀵逛簬鎰熷叴瓒ｇ殑浜猴紝浠ヤ笅鏄綋浣犱娇鐢?/dev 鎺ュ彛璁块棶 I2C 鏃讹紝鍐呮牳鍐呴儴鍙戠敓鐨勪唬鐮佹祦绋嬶細

1) 浣犵殑绋嬪簭鎵撳紑 /dev/i2c-N 骞跺鍏惰皟鐢?ioctl()锛屽涓婇潰鈥淐 绀轰緥鈥濅竴鑺傛墍杩般€?
2) 杩欎簺 open() 鍜?ioctl() 璋冪敤鐢?i2c-dev 鍐呮牳椹卞姩澶勭悊锛氬垎鍒弬瑙?i2c-dev.c:i2cdev_open() 鍜?i2c-dev.c:i2cdev_ioctl()銆備綘鍙互鎶?i2c-dev 鐪嬩綔涓€涓彲浠庣敤鎴风┖闂寸紪绋嬬殑閫氱敤 I2C 鑺墖椹卞姩銆?
3) 鏌愪簺 ioctl() 璋冪敤鐢ㄤ簬绠＄悊浠诲姟锛岀敱 i2c-dev 鐩存帴澶勭悊銆備緥瀛愬寘鎷?I2C_SLAVE锛堣缃綘瑕佽闂殑璁惧鐨勫湴鍧€锛夊拰 I2C_PEC锛堝湪鏈潵浜嬪姟涓婂惎鐢ㄦ垨绂佺敤 SMBus 閿欒妫€鏌ワ級銆?
4) 鍏朵粬 ioctl() 璋冪敤鐢?i2c-dev 杞崲涓哄唴鏍稿唴鍑芥暟璋冪敤銆備緥瀛愬寘鎷?I2C_FUNCS锛屽畠浣跨敤 i2c.h:i2c_get_functionality() 鏌ヨ I2C 閫傞厤鍣ㄥ姛鑳斤紱浠ュ強 I2C_SMBUS锛屽畠浣跨敤 i2c-core-smbus.c:i2c_smbus_xfer() 鎵ц SMBus 浜嬪姟銆?
   i2c-dev 椹卞姩璐熻矗妫€鏌ユ潵鑷敤鎴风┖闂寸殑鎵€鏈夊弬鏁版槸鍚︽湁鏁堛€傚湪姝や箣鍚庯紝杩欎簺閫氳繃 i2c-dev 鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勮皟鐢紝涓庣敱鍐呮牳 I2C 鑺墖椹卞姩鐩存帴鎵ц鐨勮皟鐢ㄤ箣闂村氨娌℃湁鍖哄埆浜嗐€傝繖鎰忓懗鐫€ I2C 鎬荤嚎椹卞姩鏃犻渶瀹炵幇浠讳綍鐗规畩鐨勪笢瑗挎潵鏀寔鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勮闂€?
5) 杩欎簺 i2c.h 鍑芥暟鏄綘鐨?I2C 鎬荤嚎椹卞姩瀹為檯瀹炵幇鐨勫皝瑁呫€傛瘡涓€傞厤鍣ㄩ兘蹇呴』澹版槑瀹炵幇杩欎簺鏍囧噯璋冪敤鐨勫洖璋冨嚱鏁般€俰2c.h:i2c_get_functionality() 璋冪敤 i2c_adapter.algo->functionality()锛岃€?i2c-core-smbus.c:i2c_smbus_xfer() 瑕佷箞璋冪敤 adapter.algo->smbus_xfer()锛堝鏋滃凡瀹炵幇锛夛紝瑕佷箞璋冪敤 i2c-core-smbus.c:i2c_smbus_xfer_emulated()锛屽悗鑰呰繘鑰岃皟鐢?i2c_adapter.algo->master_xfer()銆?
鍦ㄤ綘鐨?I2C 鎬荤嚎椹卞姩澶勭悊瀹岃繖浜涜姹傚悗锛屾墽琛屾部璋冪敤閾惧悜涓婅繑鍥烇紝鍑犱箮涓嶅仛浠讳綍澶勭悊锛岄櫎浜?i2c-dev 鍦ㄩ渶瑕佹椂鎶婅繑鍥炵殑鏁版嵁鎵撳寘鎴愰€傚悎 ioctl 鐨勬牸寮忋€?