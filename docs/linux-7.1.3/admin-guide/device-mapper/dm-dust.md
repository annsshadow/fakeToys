## dm-dust


璇ョ洰鏍囨ā鎷熷湪浠绘剰
浣嶇疆鍑虹幇鍧忔墖鍖虹殑琛屼负锛屼互鍙婅兘澶熷湪浠绘剰鏃跺埢
鍚敤杩欑鏁呴殰妯℃嫙鐨勮兘鍔涖€?

璇ョ洰鏍囩殑琛屼负绫讳技浜?linear 鐩爣銆傚湪缁欏畾鏃跺埢锛?
鐢ㄦ埛鍙互鍚戠洰鏍囧彂閫佹秷鎭紝浠ュ紑濮嬭瀵圭壒瀹氬潡鐨勮
璇锋眰澶辫触锛堜互妯℃嫙鍏锋湁
鍧忔墖鍖虹殑纭洏椹卞姩鍣ㄧ殑琛屼负锛夈€?
1. 灏嗚鍧椾粠 鈥渂ad block list鈥?涓Щ闄ゃ€?
褰撴晠闅滆涓鸿鍚敤鏃讹紙鍗筹細褰?
"dmsetup status" 鏄剧ず 鈥渇ail_read_on_bad_block鈥?鏃讹級锛屽
鈥滃潖鍧楀垪琛ㄢ€濅腑鍧楃殑璇诲彇灏嗕互 EIO锛堚€滆緭鍏?杈撳嚭閿欒鈥濓級澶辫触銆?

瀵光€滃潖鍧楀垪琛ㄢ€濅腑鍧楃殑鍐欏叆灏嗗鑷翠互涓嬬粨鏋滐細
鍊熷姪 dm-dust锛岀敤鎴峰彲浠ヤ娇鐢?鈥渁ddbadblock鈥?鍜?鈥渞emovebadblock鈥?娑堟伅鍦ㄦ柊鐨勪綅缃坊鍔犱换鎰忓潖鍧楋紝浠ュ強浣跨敤 鈥渆nable鈥?鍜?鈥渄isable鈥?娑堟伅鏉ヨ皟鑺傛墍閰嶇疆鐨?鈥渂ad blocks鈥?鏄瑙嗕负鍧忓潡杩樻槸琚粫杩囥€?
1. 灏嗚鍧椾粠鈥滃潖鍧楀垪琛ㄢ€濅腑绉婚櫎銆?
2. 鎴愬姛瀹屾垚鍐欏叆銆?
### 琛ㄥ弬鏁?
杩欐ā鎷熶簡鍏锋湁鍧忔墖鍖虹殑椹卞姩鍣?
鐨勨€滈噸鏄犲皠鎵囧尯鈥濊涓恒€?
<device_path> <offset> <blksz>
閫氬父锛岄亣鍒板潖鎵囧尯鐨勯┍鍔ㄥ櫒寰堝彲鑳藉湪
鏈煡鐨勬椂闂存垨浣嶇疆閬囧埌鏇村鍧忔墖鍖恒€?
鍊熷姪 dm-dust锛岀敤鎴峰彲浠ヤ娇鐢?"addbadblock" 涓?"removebadblock"
娑堟伅鍦ㄦ柊鐨勪綅缃坊鍔犱换鎰忓潖鍧楋紝骞朵娇鐢?
"enable" 涓?"disable" 娑堟伅鏉ヨ皟鑺傚凡閰嶇疆鐨勨€滃潖鍧椻€?
鏄褰撲綔鍧忓潡锛岃繕鏄缁曡繃銆?
杩欏厑璁稿湪妯℃嫙鍧忔墖鍖哄紑濮嬪嚭鐜扮殑鈥滄晠闅溾€濅簨浠朵箣鍓嶏紝
棰勫厛鍐欏叆娴嬭瘯鏁版嵁涓庡厓鏁版嵁銆?
    <blksz>:
### 琛ㄥ弬鏁?

<device_path> <offset> <blksz>

蹇呴€夊弬鏁帮細


```

        $ sudo blockdev --getsz /dev/vdb1
        33552384

```
鍒?dm-dust 璁惧锛?
锛堝浜庡潡澶у皬涓?512 瀛楄妭鐨勮澶囷級

### 浣跨敤璇存槑

        $ sudo dmsetup create dust1 --table '0 33552384 dust /dev/vdb1 0 512'

```
锛堝浜庡潡澶у皬涓?4096 瀛楄妭鐨勮澶囷級

```

鍒涘缓 dm-dust 璁惧锛?
锛堝浜庡潡澶у皬涓?512 瀛楄妭鐨勮澶囷級
```
妫€鏌ヨ琛屼负鐨勭姸鎬侊紙鈥渂ypass鈥?琛ㄧず鎵€鏈?I/O 閮藉皢鐩撮€氬埌搴曞眰璁惧锛涒€渧erbose鈥?琛ㄧず
```

        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 bypass verbose
锛堝浜庡潡澶у皬涓?4096 瀛楄妭鐨勮澶囷級
        $ sudo dd if=/dev/mapper/dust1 of=/dev/null bs=512 count=128 iflag=direct
        128+0 records in
        128+0 records out

        $ sudo dd if=/dev/zero of=/dev/mapper/dust1 bs=512 count=128 oflag=direct
        128+0 records in
妫€鏌ヨ鍙栬涓虹殑鐘舵€侊紙鈥渂ypass鈥濊〃绀烘墍鏈?I/O
閮藉皢閫忎紶鍒板簳灞傝澶囷紱鈥渧erbose鈥濊〃绀?
```
### 娣诲姞鍜岀Щ闄ゅ潖鍧?


鍦ㄤ换浣曟椂鍒伙紙鍗筹細鏃犺璁惧鏄惎鐢ㄤ簡 鈥渂ad block鈥?妯℃嫙杩樻槸绂佺敤浜嗗畠锛夛紝閮藉彲浠ヤ粠
```

        $ sudo dmsetup message dust1 0 addbadblock 60
        kernel: device-mapper: dust: badblock added at block 60

        $ sudo dmsetup message dust1 0 addbadblock 67
        kernel: device-mapper: dust: badblock added at block 67

        $ sudo dmsetup message dust1 0 addbadblock 72
### 娣诲姞涓庣Щ闄ゅ潖鍧?

```
鍦ㄤ换浣曟椂鍒伙紙鍗筹細鏃犺璁惧鏄浜庘€滃潖鍧椻€濇ā鎷?
鍚敤杩樻槸绂佺敤鐘舵€侊級锛岄兘鍙互浠?

        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 bypass

```
### 鍚敤鍧楄澶辫触


```

        $ sudo dmsetup message dust1 0 enable
        kernel: device-mapper: dust: enabling read failures on bad sectors
杩欎簺鍧忓潡灏嗗瓨鍌ㄥ湪鈥滃潖鍧楀垪琛ㄢ€濅腑銆?
        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 fail_read_on_bad_block

```
鍦ㄨ澶囧浜?鈥渇ail read on bad block锛堣鍙栧潖鍧楀け璐ワ級鈥?妯″紡涓嬫椂锛屽皾璇曡鍙?
```
### 鍚敤鍧楄鍙栧け璐?
        $ sudo dd if=/dev/mapper/dust1 of=/dev/null bs=512 count=1 skip=67 iflag=direct
        dd: error reading '/dev/mapper/dust1': Input/output error
        0+0 records in
        0+0 records out
        0 bytes copied, 0.00040651 s, 0.0 kB/s

```
...鑰屽鍧忓潡鐨勫啓鍏ヤ細灏嗗潡浠庡垪琛ㄤ腑绉婚櫎锛?
```

        $ sudo dd if=/dev/zero of=/dev/mapper/dust1 bs=512 count=128 oflag=direct
褰撹澶囧浜庘€渇ail read on bad block鈥濇ā寮忔椂锛屽皾璇曡鍙?
        128+0 records out

        kernel: device-mapper: dust: block 60 removed from badblocklist by write
        kernel: device-mapper: dust: block 67 removed from badblocklist by write
        kernel: device-mapper: dust: block 72 removed from badblocklist by write
        kernel: device-mapper: dust: block 87 removed from badblocklist by write

```
### 鍧忓潡娣诲姞/绉婚櫎閿欒澶勭悊
鈥︹€﹁€屽鍧忓潡鍐欏叆浼氬皢杩欎簺鍧椾粠鍒楄〃涓Щ闄わ紝

灏濊瘯娣诲姞涓€涓凡缁忓瓨鍦ㄤ簬鍒楄〃涓殑鍧忓潡浼?
```

        $ sudo dmsetup message dust1 0 addbadblock 88
        device-mapper: message ioctl on dust1  failed: Invalid argument
        kernel: device-mapper: dust: block 88 already in badblocklist

```
灏濊瘯绉婚櫎涓€涓湪鍒楄〃涓笉瀛樺湪鐨勫潖鍧椾細
```

### 鍧忓潡娣诲姞/绉婚櫎鐨勯敊璇鐞?
        device-mapper: message ioctl on dust1  failed: Invalid argument
        kernel: device-mapper: dust: block 87 not found in badblocklist
灏濊瘯娣诲姞鍒楄〃涓凡瀛樺湪鐨勫潖鍧楀皢
```
### 缁熻鍧忓潡鍒楄〃涓殑鍧忓潡鏁伴噺


瑕佺粺璁¤澶囦腑閰嶇疆鐨勫潖鍧楁暟閲忥紝杩愯
```

灏濊瘯绉婚櫎鍒楄〃涓笉瀛樺湪鐨勫潖鍧楀皢

```
浼氭墦鍗颁竴鏉″寘鍚綋鍓嶅潖鍧楁暟閲忕殑娑堟伅
```

        countbadblocks: 895 badblock(s) found

### 缁熻鍧忓潡鍒楄〃涓殑鍧忓潡鏁伴噺
### 鏌ヨ鐗瑰畾鍧忓潡

瑕佺粺璁¤澶囦腑閰嶇疆鐨勫潖鍧楁暟閲忥紝杩愯
瑕佹煡鏄庢煇涓壒瀹氬潡鏄惁鍦ㄥ潖鍧楀垪琛ㄤ腑锛岃繍琛?
```

        $ sudo dmsetup message dust1 0 queryblock 72

灏嗘墦鍗颁竴鏉℃秷鎭紝鍖呭惈褰撳墠

```

        dust_query_block: block 72 found in badblocklist

### 鏌ヨ鐗瑰畾鍧忓潡

```
瑕佹煡鏄庢煇涓壒瀹氬潡鏄惁鍦ㄥ潖鍧楀垪琛ㄤ腑锛岃繍琛?
        dust_query_block: block 72 not found in badblocklist

```
鈥渜ueryblock鈥?娑堟伅鍛戒护鍦?鈥渆nabled鈥?鍜?鈥渄isabled鈥?涓ょ妯″紡涓嬮兘鑳藉伐浣滐紝鍏佽鍦ㄤ笉鍚戣澶囧彂鍑?I/O 鎴栦笉蹇?鈥渆nable鈥?鍧忓潡妯℃嫙鐨勬儏鍐典笅锛岄獙璇佹煇涓潡鏄惁浼氳瑙嗕负 鈥渂ad鈥濄€?

### 娓呯┖鍧忓潡鍒楄〃


瑕佹竻绌哄潖鍧楀垪琛紙鏃犻渶涓烘瘡涓潡鍗曠嫭杩愯 鈥渞emovebadblock鈥?娑堟伅鍛戒护锛夛紝杩愯
```

        $ sudo dmsetup message dust1 0 clearbadblocks

```

"queryblock" 娑堟伅鍛戒护鍦ㄢ€渆nabled鈥?
鍜?"disabled" 妯″紡涓嬪潎鍙伐浣滐紝鍏佽鍦ㄤ笉鍚戣澶囧彂璧?I/O
鐨勬儏鍐典笅楠岃瘉鏌愪釜鍧楁槸鍚︿細琚綋浣溾€滃潖鍧椻€濓紝
涔熸棤闇€鈥滃惎鐢ㄢ€濆潖鍧楁ā鎷熴€?
```
### 娓呯┖鍧忓潡鍒楄〃
```

瑕佹竻绌哄潖鍧楀垪琛紙鏃犻渶閫愪釜杩愯
閽堝姣忎釜鍧楃殑 "removebadblock" 娑堟伅鍛戒护锛夛紝杩愯
```
### 鍒楀嚭鍧忓潡鍒楄〃


瑕佸垪鍑哄潖鍧楀垪琛ㄤ腑鐨勬墍鏈夊潖鍧楋紙浣跨敤鍧忓潡鍒楄〃涓湁鍧?1 鍜?2 鐨勭ず渚嬭澶囷級锛岃繍琛屼互涓嬫秷鎭?
```

        $ sudo dmsetup message dust1 0 listbadblocks
        1
        2
濡傛灉娌℃湁鍙竻绌虹殑鍧忓潡锛屽皢鏄剧ず浠ヤ笅娑堟伅
```
濡傛灉鍧忓潡鍒楄〃涓病鏈夊潖鍧楋紝璇ュ懡浠や細
```

        $ sudo dmsetup message dust1 0 listbadblocks
### 鍒楀嚭鍧忓潡鍒楄〃
```
### 娑堟伅鍛戒护鍒楄〃
瑕佸垪鍑哄潖鍧楀垪琛ㄤ腑鐨勬墍鏈夊潖鍧楋紙浠ヤ竴涓潖鍧楀垪琛ㄤ腑鍖呭惈鍧?1 鍜?2 鐨勭ず渚嬭澶囦负渚嬶級锛岃繍琛屼互涓嬫秷鎭?

浠ヤ笅鏄彲鍙戦€佺粰 dust 璁惧鐨勬秷鎭垪琛細

```

        addbadblock <blknum>
        queryblock <blknum>
        removebadblock <blknum>
濡傛灉鍧忓潡鍒楄〃涓病鏈夊潖鍧楋紝璇ュ懡浠ゅ皢
```
...鍏朵腑 <blknum> 鏄澶囪寖鍥村唴鐨勫潡鍙凤紙瀵瑰簲浜庤澶囩殑鍧楀ぇ灏忥級銆?

```

### 娑堟伅鍛戒护鍒楄〃
        clearbadblocks
        listbadblocks
浠ヤ笅鏄彲鍙戦€佺粰 dust 璁惧鐨勬秷鎭垪琛細
        enable
        quiet

```
### 璁惧绉婚櫎


```
鈥︹€﹀叾涓?<blknum> 鏄澶囪寖鍥村唴鐨勫潡鍙?
锛堜笌璁惧鐨勫潡澶у皬鐩稿搴旓級銆?

```
### 瀹夐潤妯″紡


鍦ㄥ叿鏈夊ぇ閲忓潖鍧楃殑娴嬭瘯杩愯涓紝鍙兘甯屾湜閬垮厤杩囧鐨勬棩蹇楋紙鏉ヨ嚜娣诲姞銆佺Щ闄ゆ垨 鈥渞emapped鈥?鐨勫潖鍧楋級銆?
```

        $ sudo dmsetup message dust1 0 quiet

```
### 璁惧绉婚櫎

```

        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 fail_read_on_bad_block quiet

```
### 闈欓粯妯″紡
```

鍦ㄥ寘鍚ぇ閲忓潖鍧楃殑娴嬭瘯杩愯涓紝鍙兘甯屾湜閬垮厤
杩囧鐨勬棩蹇楋紙鏉ヨ嚜娣诲姞銆佺Щ闄ゆ垨鈥滈噸鏄犲皠鈥濈殑鍧忓潡锛夈€?
        $ sudo dmsetup status dust1
        0 33552384 dust 252:17 fail_read_on_bad_block verbose

```
锛堚€渧erbose鈥?鐨勫瓨鍦ㄨ〃绀烘甯哥殑鏃ュ織銆傦級
杩欏皢鎶戝埗鏉ヨ嚜 add / remove / 鐢卞啓鍏ョЩ闄?
鎿嶄綔鐨勬棩蹇楁秷鎭€傛潵鑷?"countbadblocks" 鎴?"queryblock"
娑堟伅鍛戒护鐨勬棩蹇椾粛浼氬湪闈欓粯妯″紡涓嬫墦鍗般€?

scsi_debug 鏈変竴涓?鈥渕edium error鈥?妯″紡锛屽彲浠ヤ娇涓€涓寚瀹氭墖鍖猴紙鎵囧尯 0x1234锛岀‖缂栫爜鍦ㄦ簮浠ｇ爜涓級鐨勮鍙栧け璐ワ紝浣嗗畠浣跨敤 RAM 浣滀负鎸佷箙瀛樺偍锛岃繖澶уぇ鍑忓皬浜嗚澶囧彲鑳界殑灏哄銆?

dm-flakey 鍦ㄦ寚瀹氱殑鏃堕棿棰戠巼锛堣€屼笉鏄煇涓粰瀹氭椂闂寸偣锛変娇鏉ヨ嚜鎵€鏈夊潡浣嶇疆鐨勬墍鏈?I/O 澶辫触銆?

褰撶‖鐩橀┍鍔ㄥ櫒涓婂嚭鐜板潖鎵囧尯鏃讹紝瀵硅鎵囧尯鐨勮鍙栦細鐢辫澶囦娇澶辫触锛岄€氬父瀵艰嚧 EIO锛堚€淚/O error鈥濓紝I/O 閿欒锛夋垨 ENODATA锛堚€淣o data available鈥濓紝鏃犲彲鐢ㄦ暟鎹級鐨勯敊璇爜銆備絾鏄紝瀵硅鎵囧尯鐨勫啓鍏ュ彲鑳芥垚鍔燂紝骞跺湪璁惧鎺у埗鍣ㄤ笉鍐嶉亣鍒拌鍙栬鎵囧尯鐨勯敊璇紙鎴栧湪鎵囧尯琚噸鏂板垎閰嶄箣鍚庯級瀵艰嚧璇ユ墖鍖哄彉涓哄彲璇汇€傜劧鑰岋紝鏈潵鍙兘浼氬湪璁惧涓婄殑涓嶅悓銆佷笉鍙娴嬬殑浣嶇疆鍑虹幇鍧忔墖鍖恒€?

姝?target 鏃ㄥ湪鎻愪緵涓€涓澶囷紝鑳藉鍩轰簬涓€涓ぇ瀹归噺瀛樺偍璁惧锛堣嚦灏戞暟鍗?GB锛屼笉鍗犵敤绯荤粺鍐呭瓨锛夛紝鍦ㄥ凡鐭ユ墖鍖轰綅缃€佸凡鐭ユ椂闂磋〃鐜板嚭鍧忔墖鍖虹殑琛屼负銆?
