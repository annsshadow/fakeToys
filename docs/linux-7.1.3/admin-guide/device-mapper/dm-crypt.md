# dm-crypt


Device-Mapper 鐨?鈥渃rypt鈥?鐩爣鍒╃敤鍐呮牳 crypto API 鎻愪緵瀵瑰潡璁惧鐨勯€忔槑鍔犲瘑銆?

鏈夊叧鎵€鏀寔鍙傛暟鐨勬洿璇︾粏鎻忚堪锛岃鍙傝锛?
https://gitlab.com/cryptsetup/cryptsetup/wikis/DMCrypt

```

	      <cipher> <key> <iv_offset> <device path> \
	      <offset> [<#opt_params> <opt_params>]

```
<cipher>
    鍔犲瘑绠楁硶锛坈ipher锛夈€佸姞瀵嗘ā寮忎互鍙婂垵濮嬪悜閲忥紙IV锛夌敓鎴愬櫒銆?

```

       cipher[:keycount]-chainmode-ivmode[:ivopts]

    Examples::

       aes-cbc-essiv:sha256
       aes-xts-plain64
       serpent-xts-plain64

    Cipher format also supports direct specification with kernel crypt API
    format (selected by capi: prefix). The IV specification is the same
    as for the first format type.
    This format is mainly used for specification of authenticated modes.

    The crypto API cipher specifications format is::

        capi:cipher_api_spec-ivmode[:ivopts]

    Examples::

        capi:cbc(aes)-essiv:sha256
        capi:xts(aes)-plain64

    Examples of authenticated modes::

        capi:gcm(aes)-random
        capi:authenc(hmac(sha256),xts(aes))-random
        capi:rfc7539(chacha20,poly1305)-random

    The /proc/crypto contains a list of currently loaded crypto modes.

```
<key>
    鐢ㄤ簬鍔犲瘑鐨勫瘑閽ャ€傚畠鏃㈠彲浠ョ紪鐮佷负鍗佸叚杩涘埗鏁板瓧
    or it can be passed as <key_string> prefixed with single colon
    character (':') for keys residing in kernel keyring service.
    You can only use key sizes that are valid for the selected cipher
    in combination with the selected iv mode.
    Note that for some iv modes the key string can contain additional
    keys (for example IV seed) so the key contains more parts concatenated
    into a single string.

<key_string>
    鍐呮牳瀵嗛挜鐜紙keyring锛夊瘑閽ラ€氳繃浠ヤ笅鏍煎紡鐨勫瓧绗︿覆鏍囪瘑锛?
    <key_size>:<key_type>:<key_description>銆?

<key_size>
    鍔犲瘑瀵嗛挜鐨勫ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€傚唴鏍稿瘑閽ョ殑杞借嵎澶у皬蹇呴』涓?<key_size> 涓紶鍏ョ殑鍊肩浉鍖归厤銆?

<key_type>
    鈥榣ogon鈥欍€佲€榰ser鈥欍€佲€榚ncrypted鈥?鎴?鈥榯rusted鈥?鍐呮牳瀵嗛挜绫诲瀷涔嬩竴銆?

<key_description>
    crypt 鐩爣鍦ㄥ姞杞?<key_type> 绫诲瀷瀵嗛挜鏃跺簲鏌ユ壘鐨勫唴鏍稿瘑閽ョ幆瀵嗛挜鎻忚堪銆?

<keycount>
    Multi-key compatibility mode. You can define <keycount> keys and
    then sectors are encrypted according to their offsets (sector 0 uses key0;
    sector 1 uses key1 etc.).  <keycount> must be a power of two.

<iv_offset>
    IV 鍋忕Щ鏄竴涓墖鍖鸿鏁帮紝鍦ㄥ垱寤?IV 涔嬪墠浼氳鍔犲埌鎵囧尯鍙蜂笂銆?

<device path>
    杩欐槸灏嗙敤浣滃悗绔苟鍖呭惈鍔犲瘑鏁版嵁鐨勮澶囥€備綘鍙互灏嗗叾鎸囧畾涓虹被浼?/dev/xxx 鐨勮矾寰勶紝鎴栨寚瀹氫负璁惧鍙?<major>:<minor>銆?

<offset>
    璁惧鍐呭姞瀵嗘暟鎹紑濮嬬殑璧峰鎵囧尯銆?

<#opt_params>
    鍙€夊弬鏁扮殑鏁伴噺銆傚鏋滄病鏈夊彲閫夊弬鏁帮紝鍒欏彲浠ヨ烦杩囧彲閫夊弬鏁伴儴鍒嗭紝鎴栬€呭皢 #opt_params 璁句负闆躲€傚惁鍒?#opt_params 涓哄悗缁弬鏁扮殑鏁伴噺銆?

    鍙€夊弬鏁伴儴鍒嗙ず渚嬶細
        3 allow_discards same_cpu_crypt submit_from_crypt_cpus

allow_discards
    鍧椾涪寮冭姹傦紙鍗?TRIM锛変細琚€忎紶鍒?crypt 璁惧銆傞粯璁よ涓烘槸蹇界暐涓㈠純璇锋眰銆?

    WARNING锛氬湪鍚敤姝ら€夐」涔嬪墠锛岃浠旂粏璇勪及鐗瑰畾鐨勫畨鍏ㄩ闄┿€備緥濡傦紝鍦ㄥ姞瀵嗚澶囦笂鍏佽涓㈠純鍙兘瀵艰嚧瀵嗘枃璁惧锛堟枃浠剁郴缁熺被鍨嬨€佸凡鐢ㄧ┖闂寸瓑锛夌殑淇℃伅娉勯湶锛屽墠鎻愭槸鍚庣画鍙互鍦ㄨ澶囦笂杞绘槗瀹氫綅鍒拌涓㈠純鐨勫潡銆?

same_cpu_crypt
    浣跨敤鎻愪氦 IO 鏃舵墍鐢ㄧ殑鍚屼竴涓?CPU 鎵ц鍔犲瘑銆傞粯璁ゆ槸浣跨敤鏈粦瀹氱殑宸ヤ綔闃熷垪锛屼粠鑰岃鍔犲瘑宸ヤ綔鍦ㄥ悇鍙敤 CPU 涔嬮棿鑷姩鍧囪　銆?

high_priority
    灏?dm-crypt 宸ヤ綔闃熷垪鍜屽啓鍏ョ嚎绋嬭涓洪珮浼樺厛绾с€傝繖浼氬湪闄嶄綆绯荤粺鏁翠綋鍝嶅簲鑳藉姏鐨勫悓鏃讹紝鎻愬崌 dm-crypt 鐨勫悶鍚愰噺涓庡欢杩熴€?

submit_from_crypt_cpus
    绂佺敤鍔犲瘑鍚庡皢鍐欏叆鎿嶄綔鍗歌浇鍒板崟鐙嚎绋嬬殑鍋氭硶銆傚湪鏌愪簺鎯呭喌涓嬶紝灏嗗啓鍏?bio 浠庡姞瀵嗙嚎绋嬪嵏杞藉埌鍗曚釜绾跨▼浼氭樉钁楅檷浣庢€ц兘銆傞粯璁ゆ槸灏嗗啓鍏?bio 鍗歌浇鍒板悓涓€绾跨▼锛屽洜涓轰娇鐢ㄧ浉鍚屼笂涓嬫枃鎻愪氦鍐欏叆瀵?CFQ 鏈夌泭銆?

no_read_workqueue
    缁曡繃 dm-crypt 鍐呴儴宸ヤ綔闃熷垪锛屽苟鍚屾澶勭悊璇诲彇璇锋眰銆?

no_write_workqueue
    缁曡繃 dm-crypt 鍐呴儴宸ヤ綔闃熷垪锛屽苟鍚屾澶勭悊鍐欏叆璇锋眰銆傚浜庝富鏈虹鐞嗙殑瑙勫尯锛坺oned锛夊潡璁惧锛堜緥濡備富鏈虹鐞嗙殑 SMR 纭洏锛夛紝姝ら€夐」浼氳嚜鍔ㄥ惎鐢ㄣ€?

integrity:<bytes>:<type>
    The device requires additional <bytes> metadata per-sector stored
    in per-bio integrity structure. This metadata must by provided
    by underlying dm-integrity target.

    The <type> can be "none" if metadata is used only for persistent IV.

    For Authenticated Encryption with Additional Data (AEAD)
    the <type> is "aead". An AEAD mode additionally calculates and verifies
    integrity for the encrypted device. The additional space is then
    used for storing authentication tag (and persistent IV if needed).

integrity_key_size:<bytes>
    濡傛灉涓庢憳瑕佸ぇ灏忎笉鍚岋紝鍙€夋嫨鎬у湴璁剧疆瀹屾暣鎬у瘑閽ュぇ灏忋€傚畠鍏佽浣跨敤灏佽瀵嗛挜锛坵rapped key锛夌畻娉曪紝鍏朵腑瀵嗛挜澶у皬涓庡姞瀵嗗瘑閽ュぇ灏忔棤鍏炽€?

sector_size:<bytes>
    Use <bytes> as the encryption unit instead of 512 bytes sectors.
    This option can be in range 512 - 4096 bytes and must be power of two.
    Virtual device will announce this size as a minimal IO and logical sector.

iv_large_sectors
   IV 鐢熸垚鍣ㄥ皢浣跨敤浠?<sector_size> 涓哄崟浣嶈鏁扮殑鎵囧尯鍙凤紝鑰屼笉鏄粯璁ょ殑 512 瀛楄妭鎵囧尯銆?

   渚嬪锛屽鏋?<sector_size> 涓?4096 瀛楄妭锛屽垯绗簩涓墖鍖虹殑 plain64 IV 鍦ㄦ病鏈夎鏍囧織鏃朵负 8锛岃€屽湪瀛樺湪 iv_large_sectors 鏃朵负 1銆傚鏋滄寚瀹氫簡璇ユ爣蹇楋紝鍒?<iv_offset> 蹇呴』鏄?<sector_size> 鐨勫€嶆暟锛堜互 512 瀛楄妭涓哄崟浣嶏級銆?

integrity_key_size:<bytes>
   浣跨敤澶у皬涓?<bytes> 鐨勫畬鏁存€у瘑閽ワ紝鑰屼笉鏄娇鐢ㄦ墍鐢?HMAC 绠楁硶鐨勬憳瑕佸ぇ灏忕殑瀹屾暣鎬у瘑閽ャ€?


```
   max_read_size
      Maximum size of read requests. When a request larger than this size
      is received, dm-crypt will split the request. The splitting improves
      concurrency (the split requests could be encrypted in parallel by multiple
      cores), but it also causes overhead. The user should tune this parameters to
      fit the actual workload.

   max_write_size
      Maximum size of write requests. When a request larger than this size
      is received, dm-crypt will split the request. The splitting improves
      concurrency (the split requests could be encrypted in parallel by multiple
      cores), but it also causes overhead. The user should tune this parameters to
      fit the actual workload.


```
绀轰緥鑴氭湰

LUKS锛圠inux Unified Key Setup锛夌幇鍦ㄦ槸浣跨敤 'cryptsetup' 宸ュ叿閰嶅悎 dm-crypt 璁剧疆纾佺洏鍔犲瘑鐨勯閫夋柟寮忥紝璇峰弬瑙?
https://gitlab.com/cryptsetup/cryptsetup


```

	#!/bin/sh
	# Create a crypt device using dmsetup
	dmsetup create crypt1 --table "0 `blockdev --getsz $1` crypt aes-cbc-essiv:sha256 babebabebabebabebabebabebabebabe 0 $1 0"

```
```

	#!/bin/sh
	# Create a crypt device using dmsetup when encryption key is stored in keyring service
	dmsetup create crypt2 --table "0 `blockdev --getsize $1` crypt aes-cbc-essiv:sha256 :32:logon:my_prefix:my_key 0 $1 0"

```
```

	#!/bin/sh
	# Create a crypt device using cryptsetup and LUKS header with default cipher
	cryptsetup luksFormat $1
	cryptsetup luksOpen $1 crypt1

```