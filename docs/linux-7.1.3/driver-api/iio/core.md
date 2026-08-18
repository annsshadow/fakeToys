## 鏍稿績瑕佺礌


宸ヤ笟 I/O锛圛ndustrial I/O锛孖IO锛夋牳蹇冩棦涓虹紪鍐欏绉嶄笉鍚岀被鍨嬪祵鍏ュ紡浼犳劅鍣ㄧ殑椹卞姩鎻愪緵浜嗕竴涓?
缁熶竴鐨勬鏋讹紝涔熶负鎿嶄綔鐢ㄦ埛绌洪棿浼犳劅鍣ㄥ簲鐢ㄧ▼搴忔彁渚涗簡鏍囧噯鎺ュ彛銆傚叾瀹炵幇鍙湪
`drivers/iio/industrialio-*` 涓嬫壘鍒般€?

### 宸ヤ笟 I/O 璁惧


- struct iio_dev - 宸ヤ笟 I/O 璁惧
- iio_device_alloc() - 浠庨┍鍔ㄥ垎閰嶄竴涓?`iio_dev`
- iio_device_free() - 浠庨┍鍔ㄩ噴鏀句竴涓?`iio_dev`
- iio_device_register() - 鍚?IIO 瀛愮郴缁熸敞鍐屼竴涓澶?
- iio_device_unregister() - 浠?IIO 瀛愮郴缁熸敞閿€涓€涓澶?

涓€涓?IIO 璁惧閫氬父瀵瑰簲浜庡崟涓‖浠朵紶鎰熷櫒锛屽苟鎻愪緵澶勭悊璇ヨ澶囩殑椹卞姩鎵€闇€鐨勫叏閮ㄤ俊鎭€?
璁╂垜浠厛浜嗚В涓€涓嬪祵鍏ュ湪 IIO 璁惧涓殑鍔熻兘锛岀劧鍚庡啀灞曠ず璁惧椹卞姩濡備綍浣跨敤涓€涓?IIO 璁惧銆?

鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍙互閫氳繃涓ょ鏂瑰紡涓?IIO 椹卞姩浜や簰銆?

1. `/sys/bus/iio/devices/iio:device{X}/`锛岃繖浠ｈ〃涓€涓‖浠朵紶鎰熷櫒锛屽苟灏嗗悓涓€鑺墖鐨勬暟鎹€氶亾鍒嗙粍鍦ㄤ竴璧枫€?
2. `/dev/iio:device{X}`锛岀敤浜庣紦鍐叉暟鎹紶杈撳拰浜嬩欢淇℃伅鑾峰彇鐨勫瓧绗﹁澶囪妭鐐规帴鍙ｃ€?

涓€涓吀鍨嬬殑 IIO 椹卞姩浼氬皢鑷繁娉ㄥ唽涓?[I2C <../i2c>](I2C <../i2c>) 鎴?
[SPI <../spi>](SPI <../spi>) 椹卞姩锛屽苟鍒涘缓 probe 涓?remove 涓や釜渚嬬▼銆?

鍦?probe 鏃讹細

1. 璋冪敤 iio_device_alloc()锛屼负 IIO 璁惧鍒嗛厤鍐呭瓨銆?
2. 鐢ㄩ┍鍔ㄧ壒瀹氱殑淇℃伅锛堜緥濡傝澶囧悕銆佽澶囬€氶亾锛夊垵濮嬪寲 IIO 璁惧瀛楁銆?
3. 璋冪敤 iio_device_register()锛屽皢璁惧娉ㄥ唽鍒?IIO 鏍稿績銆傚湪姝よ皟鐢ㄤ箣鍚庯紝璁惧鍗冲彲鎺ュ彈鏉ヨ嚜鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鐨勮姹傘€?

鍦?remove 鏃讹紝鎴戜滑浠ョ浉鍙嶇殑椤哄簭閲婃斁 probe 涓垎閰嶇殑璧勬簮锛?

1. iio_device_unregister()锛屼粠 IIO 鏍稿績娉ㄩ攢璁惧銆?
2. iio_device_free()锛岄噴鏀句负 IIO 璁惧鍒嗛厤鐨勫唴瀛樸€?

## IIO 璁惧 sysfs 鎺ュ彛


灞炴€ф槸鐢ㄤ簬鏆撮湶鑺墖淇℃伅骞跺厑璁稿簲鐢ㄧ▼搴忚缃悇绉嶉厤缃弬鏁扮殑 sysfs 鏂囦欢銆傚浜庣储寮曚负 X 鐨?
璁惧锛屽睘鎬у彲鍦?/sys/bus/iio/devices/iio:deviceX/ 鐩綍涓嬫壘鍒般€傚父瑙佸睘鎬у寘鎷細

- `name`锛屽鐗╃悊鑺墖鐨勬弿杩般€?
- `dev`锛屾樉绀轰笌 `/dev/iio:deviceX` 鑺傜偣鍏宠仈鐨?major:minor 瀵广€?
- `sampling_frequency_available`锛岃澶囧彲鐢ㄧ殑绂绘暎閲囨牱棰戠巼鍊奸泦鍚堛€?
- IIO 璁惧鐨勫彲鐢ㄦ爣鍑嗗睘鎬у湪 Linux 鍐呮牳婧愮爜鐨?
  :file:Documentation/ABI/testing/sysfs-bus-iio 鏂囦欢涓湁鎻忚堪銆?

## IIO 璁惧閫氶亾


struct iio_chan_spec - 鍗曚釜閫氶亾鐨勮鏍艰鏄?

涓€涓?IIO 璁惧閫氶亾鏄涓€涓暟鎹€氶亾鐨勮〃绀恒€備竴涓?IIO 璁惧鍙互鏈変竴涓垨澶氫釜閫氶亾銆備緥濡傦細

- 娓╁害璁′紶鎰熷櫒鏈変竴涓〃绀烘俯搴︽祴閲忕殑閫氶亾銆?
- 涓€涓厜浼犳劅鍣ㄦ湁涓や釜閫氶亾锛屽垎鍒〃绀哄彲瑙佸厜涓庣孩澶栧厜璋辩殑娴嬮噺鍊笺€?
- 鍔犻€熷害璁℃渶澶氬彲鏈?3 涓€氶亾锛屽垎鍒〃绀?X銆乊 涓?Z 杞翠笂鐨勫姞閫熷害銆?

涓€涓?IIO 閫氶亾鐢?struct iio_chan_spec 鎻忚堪銆?
涓婇潰绀轰緥涓俯搴︿紶鎰熷櫒鐨勬俯搴﹁椹卞姩灏?

```

   static const struct iio_chan_spec temp_channel[] = {
        {
            .type = IIO_TEMP,
            .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
        },
   };

```
鍚戠敤鎴风┖闂存毚闇茬殑閫氶亾 sysfs 灞炴€т互浣嶆帺鐮佺殑褰㈠紡鎸囧畾銆傛牴鎹叾鍏变韩淇℃伅鐨勪笉鍚岋紝灞炴€у彲浠?
璁剧疆鍦ㄤ互涓嬫帺鐮佷箣涓€涓細

- **info_mask_separate**锛屽睘鎬у皢鐗瑰畾浜庤閫氶亾
- **info_mask_shared_by_type**锛屽睘鎬х敱鍚屼竴绫诲瀷鐨勬墍鏈夐€氶亾鍏变韩
- **info_mask_shared_by_dir**锛屽睘鎬х敱鍚屼竴鏂瑰悜鐨勬墍鏈夐€氶亾鍏变韩
- **info_mask_shared_by_all**锛屽睘鎬х敱鎵€鏈夐€氶亾鍏变韩

褰撴瘡涓€氶亾绫诲瀷鏈夊涓暟鎹€氶亾鏃讹紝鎴戜滑鏈変袱绉嶆柟寮忓尯鍒嗗畠浠細

- 灏?`iio_chan_spec` 鐨?**.modified** 瀛楁璁句负 1銆備慨楗扮閫氳繃鍚屼竴 `iio_chan_spec`
  缁撴瀯鐨?**.channel2** 瀛楁鎸囧畾锛岀敤浜庤〃绀洪€氶亾鐨勬煇涓墿鐞嗕笂鍞竴鐨勭壒寰侊紝渚嬪鍏舵柟鍚戞垨
  鍏夎氨鍝嶅簲銆備緥濡傦紝涓€涓厜浼犳劅鍣ㄥ彲浠ユ湁涓や釜閫氶亾锛屼竴涓敤浜庣孩澶栧厜锛屼竴涓敤浜庣孩澶栦笌鍙鍏夈€?
- 灏?`iio_chan_spec` 鐨?**.indexed** 瀛楁璁句负 1銆傚湪杩欑鎯呭喌涓嬶紝璇ラ€氶亾鍙槸鍙︿竴涓?
  甯︽湁鐢?**.channel** 瀛楁鎸囧畾鐨勭储寮曠殑瀹炰緥銆?

```

   static const struct iio_chan_spec light_channels[] = {
           {
                   .type = IIO_INTENSITY,
                   .modified = 1,
                   .channel2 = IIO_MOD_LIGHT_IR,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
                   .info_mask_shared = BIT(IIO_CHAN_INFO_SAMP_FREQ),
           },
           {
                   .type = IIO_INTENSITY,
                   .modified = 1,
                   .channel2 = IIO_MOD_LIGHT_BOTH,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
                   .info_mask_shared = BIT(IIO_CHAN_INFO_SAMP_FREQ),
           },
           {
                   .type = IIO_LIGHT,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
                   .info_mask_shared = BIT(IIO_CHAN_INFO_SAMP_FREQ),
           },
      }

```
璇ラ€氶亾鐨勫畾涔夊皢涓哄師濮嬫暟鎹幏鍙栫敓鎴愪袱涓嫭绔嬬殑 sysfs 鏂囦欢锛?

- `/sys/bus/iio/devices/iio:device{X}/in_intensity_ir_raw`
- `/sys/bus/iio/devices/iio:device{X}/in_intensity_both_raw`

涓€涓敤浜庡鐞嗗悗鏁版嵁鐨勬枃浠讹細

- `/sys/bus/iio/devices/iio:device{X}/in_illuminance_input`

浠ュ強涓€涓敤浜庨噰鏍烽鐜囩殑鍏变韩 sysfs 鏂囦欢锛?

- `/sys/bus/iio/devices/iio:device{X}/sampling_frequency`銆?

```

   static const struct iio_chan_spec light_channels[] = {
           {
                   .type = IIO_VOLTAGE,
		   .indexed = 1,
		   .channel = 0,
		   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
	   },
           {
	           .type = IIO_VOLTAGE,
                   .indexed = 1,
                   .channel = 1,
                   .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
           },
   }

```
杩欏皢涓哄師濮嬫暟鎹幏鍙栫敓鎴愪袱涓嫭绔嬬殑灞炴€ф枃浠讹細

- `/sys/bus/iio/devices/iio:device{X}/in_voltage0_raw`锛岃〃绀洪€氶亾 0 鐨勭數鍘嬫祴閲忓€笺€?
- `/sys/bus/iio/devices/iio:device{X}/in_voltage1_raw`锛岃〃绀洪€氶亾 1 鐨勭數鍘嬫祴閲忓€笺€?

## 鏇村缁嗚妭

   :export:
