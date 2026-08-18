## 缂撳啿鍖猴紙Buffers锛?

- struct iio_buffer 鈥?閫氱敤缂撳啿鍖虹粨鏋?- `iio_validate_scan_mask_onehot` 鈥?鏍￠獙鏄惁鎭板ソ閫変腑浜嗕竴涓€氶亾
- `iio_buffer_get` 鈥?鑾峰彇瀵圭紦鍐插尯鐨勫紩鐢?- `iio_buffer_put` 鈥?閲婃斁瀵圭紦鍐插尯鐨勫紩鐢?
Industrial I/O 鏍稿績鎻愪緵浜嗕竴绉嶅熀浜庤Е鍙戞簮锛坱rigger source锛夎繘琛岃繛缁暟鎹噰闆嗙殑鏂瑰紡銆傚彲浠ヤ粠 `/dev/iio:device{X}` 瀛楃璁惧鑺傜偣涓€娆℃€ц鍙栧涓暟鎹€氶亾锛屼粠鑰岄檷浣?CPU 璐熻浇銆?
## IIO 缂撳啿鍖?sysfs 鎺ュ彛

涓€涓?IIO 缂撳啿鍖哄湪 `/sys/bus/iio/devices/iio:device{X}/buffer/*` 涓嬫湁涓€涓叧鑱旂殑 attributes 鐩綍銆備互涓嬫槸涓€浜涘凡鏈夊睘鎬э細

- `length`锛岀紦鍐插尯鍙瓨鍌ㄧ殑鏁版嵁鏍锋湰鎬绘暟锛堝閲忥級銆?- `enable`锛屾縺娲荤紦鍐插尯閲囬泦銆?
## IIO 缂撳啿鍖鸿缃?

涓庢斁鍏ョ紦鍐插尯涓殑鏌愭閫氶亾璇诲彇鐩稿叧鐨勫厓淇℃伅绉颁负鎵弿鍏冪礌锛坰can element锛夈€傞厤缃壂鎻忓厓绱犵殑閲嶈浣嶉€氳繃 `/sys/bus/iio/devices/iio:device{X}/scan_elements/` 鐩綍鏆撮湶缁欑敤鎴风┖闂村簲鐢ㄧ▼搴忋€傝鐩綍鍖呭惈浠ヤ笅褰㈠紡鐨勫睘鎬э細

- `enable`锛岀敤浜庡惎鐢ㄦ煇涓€氶亾銆傚綋涓斾粎褰撳叾灞炴€ч潪 **闆?* 鏃讹紝瑙﹀彂寮忛噰闆嗘墠浼氬寘鍚閫氶亾鐨勬暟鎹牱鏈€?- `index`锛岃閫氶亾鐨?scan_index銆?- `type`锛屾弿杩版壂鎻忓厓绱犳暟鎹湪缂撳啿鍖轰腑鐨勫瓨鍌ㄦ柟寮忥紝浠ュ強鍥犳浠庣敤鎴风┖闂磋鍙栧畠鐨勫舰寮忋€?  鏍煎紡涓?[be|le]:[s|u]bits/storagebits[Xrepeat][>>shift] 銆?
  - **be** 鎴?**le**锛屾寚瀹氬ぇ绔垨灏忕銆?  - **s** 鎴?**u**锛屾寚瀹氭湁绗﹀彿锛堣ˉ鐮侊級鎴栨棤绗﹀彿銆?  - **bits**锛屾槸鏈夋晥鏁版嵁浣嶆暟銆?  - **storagebits**锛屾槸鏁版嵁鍦ㄧ紦鍐插尯涓崰鎹殑浣嶆暟锛堝惈濉厖锛夈€?  - **repeat**锛屾寚瀹?bits/storagebits 鐨勯噸澶嶆鏁般€傚綋 repeat 鍏冪礌涓?0 鎴?1 鏃讹紝鐪佺暐 repeat 鍊笺€?  - **shift**锛岃嫢鎸囧畾锛屽垯鏄湪灞忚斀鎺夋湭浣跨敤浣嶄箣鍓嶉渶瑕佸簲鐢ㄧ殑绉讳綅銆?
渚嬪锛屼竴涓?12 浣嶅垎杈ㄧ巼鐨?3 杞村姞閫熷害璁￠┍鍔紝鍏朵腑
```

        7   6   5   4   3   2   1   0
      +---+---+---+---+---+---+---+---+
      |D3 |D2 |D1 |D0 | X | X | X | X | (LOW byte, address 0x06)
      +---+---+---+---+---+---+---+---+

        7   6   5   4   3   2   1   0
      +---+---+---+---+---+---+---+---+
      |D11|D10|D9 |D8 |D7 |D6 |D5 |D4 | (HIGH byte, address 0x07)
      +---+---+---+---+---+---+---+---+

```
```

      $ cat /sys/bus/iio/devices/iio:device0/scan_elements/in_accel_y_type
      le:s12/16>>4

```
鐢ㄦ埛绌洪棿搴旂敤绋嬪簭浼氭妸浠庣紦鍐插尯璇诲彇鐨勬暟鎹牱鏈В閲婁负涓ゅ瓧鑺傚皬绔湁绗﹀彿鏁版嵁锛岄渶瑕佸湪灞忚斀鍑?12 浣嶆湁鏁堟暟鎹箣鍓嶅厛鍙崇Щ 4 浣嶃€?
涓哄疄鐜扮紦鍐插尯鏀寔锛岄┍鍔ㄥ簲鍒濆鍖栦互涓嬪唴瀹?```

   struct iio_chan_spec {
   /* other members */
           int scan_index
           struct {
                   char sign;
                   u8 realbits;
                   u8 storagebits;
                   u8 shift;
                   u8 repeat;
                   enum iio_endian endianness;
                  } scan_type;
          };

```
涓婅堪鍔犻€熷害璁＄殑椹卞姩灏嗗叿鏈?```

   struct iio_chan_spec accel_channels[] = {
           {
                   .type = IIO_ACCEL,
		   .modified = 1,
		   .channel2 = IIO_MOD_X,
		   /* other stuff here */
		   .scan_index = 0,
		   .scan_type = {
		           .sign = 's',
			   .realbits = 12,
			   .storagebits = 16,
			   .shift = 4,
			   .endianness = IIO_LE,
		   },
           }
           /* similar for Y (with channel2 = IIO_MOD_Y, scan_index = 1)
            * and Z (with channel2 = IIO_MOD_Z, scan_index = 2) axis
            */
    }

```
姝ゅ **scan_index** 瀹氫箟浜嗗凡鍚敤閫氶亾鍦ㄧ紦鍐插尯鍐呮斁缃殑椤哄簭銆傝緝浣庣殑 **scan_index** 鐨勯€氶亾浼氳鏀惧湪杈冮珮绱㈠紩鐨勯€氶亾涔嬪墠銆傛瘡涓€氶亾閮介渶瑕佹湁鍞竴鐨?**scan_index**銆?
灏?**scan_index** 璁句负 -1 鍙敤浜庤〃绀鸿鐗瑰畾閫氶亾涓嶆敮鎸佺紦鍐查噰闆嗐€傝繖绉嶆儏鍐典笅锛宻can_elements 鐩綍涓笉浼氫负璇ラ€氶亾鍒涘缓浠讳綍鏉＄洰銆?
## 鏇村缁嗚妭锛圡ore details锛?
   :export:
