## The SMBus Protocol锛圫MBus 鍗忚锛?

浠ヤ笅鏄 SMBus 鍗忚鐨勬瑕佽鏄庯紝閫傜敤浜庤鍗忚鐨勬墍鏈変慨璁㈢増鏈紙1.0銆?.1 涓?2.0锛夈€傛煇浜涗笉琚湰杞欢鍖呮敮鎸佺殑鍗忚鐗规€э紝灏嗗湪鏈枃妗ｆ湯灏剧畝瑕佽鏄庛€?

閮ㄥ垎閫傞厤鍣ㄥ彧鑳界悊瑙?SMBus锛圫ystem Management Bus锛岀郴缁熺鐞嗘€荤嚎锛夊崗璁紝瀹冩槸 I2C 鍗忚鐨勪竴涓瓙闆嗐€傚垢杩愮殑鏄紝璁稿璁惧鍙娇鐢ㄤ簡鐩稿悓鐨勮繖涓瓙闆嗭紝鍥犺€屽彲浠ユ妸瀹冧滑鎸傚湪 SMBus 涓娿€?

濡傛灉浣犱负鏌愪釜 I2C 璁惧缂栧啓椹卞姩锛岃灏藉彲鑳戒娇鐢?SMBus 鍛戒护锛堝墠鎻愭槸璁惧鍙娇鐢ㄤ簡 I2C 鍗忚鐨勮瀛愰泦锛夈€傝繖鏍峰氨鑳借鍚屼竴椹卞姩鏃㈠彲鐢ㄤ簬 SMBus 閫傞厤鍣紝涔熷彲鐢ㄤ簬 I2C 閫傞厤鍣紙鍦?I2C 閫傞厤鍣ㄤ笂锛孲MBus 鍛戒护闆嗕細鑷姩杞崲涓?I2C锛涗絾绾?I2C 鍛戒护鍦ㄥぇ澶氭暟绾?SMBus 閫傞厤鍣ㄤ笂瀹屽叏鏃犳硶澶勭悊锛夈€?

涓嬮潰鍒楀嚭 SMBus 鍗忚鎿嶄綔鍙婂叾瀵瑰簲鐨勬墽琛屽嚱鏁般€傝娉ㄦ剰锛孲MBus 鍗忚瑙勮寖涓娇鐢ㄧ殑鍚嶇О閫氬父涓庤繖浜涘嚱鏁板悕骞朵笉涓€鑷达紱瀵逛簬鏌愪簺鍙紶閫掑崟涓暟鎹瓧鑺傜殑鎿嶄綔锛屼娇鐢?SMBus 鍗忚鎿嶄綔鍚嶇殑鍑芥暟瀹為檯涓婃墽琛岀殑鏄畬鍏ㄤ笉鍚岀殑鍗忚鎿嶄綔銆?

姣忕浜嬪姟绫诲瀷閮藉搴斾竴涓姛鑳芥爣蹇楋紙functionality flag锛夈€傚湪璋冪敤鏌愪釜浜嬪姟鍑芥暟涔嬪墠锛岃澶囬┍鍔ㄥ簲褰擄紙鍙渶涓€娆★級鍏堟鏌ョ浉搴旂殑鍔熻兘鏍囧織锛屼互纭搴曞眰 I2C 閫傞厤鍣ㄦ敮鎸佽浜嬪姟銆傝瑙?Documentation/i2c/functionality.rst銆?

## Key to symbols锛堢鍙疯鏄庯級

=============== =============================================================
S               Start 鏉′欢锛堣捣濮嬫潯浠讹級
Sr              Repeated start 鏉′欢锛堥噸澶嶈捣濮嬫潯浠讹級锛岀敤浜庡湪鍐欎笌璇讳箣闂村垏鎹?
P               Stop 鏉′欢锛堝仠姝㈡潯浠讹級
Rd/Wr (1 bit)   Read/Write 浣嶃€俁d 绛変簬 1锛學r 绛変簬 0銆?
A, NA (1 bit)   搴旂瓟锛圓CK锛変笌闈炲簲绛旓紙NACK锛変綅
Addr  (7 bits)  I2C 7 浣嶅湴鍧€銆傛敞鎰忚鍦板潃鍙墿灞曚负 10 浣嶃€?
Comm  (8 bits)  鍛戒护瀛楄妭锛屼竴涓暟鎹瓧鑺傦紝閫氬父鐢ㄦ潵閫夋嫨璁惧涓婄殑鏌愪釜瀵勫瓨鍣ㄣ€?
Data  (8 bits)  涓€涓櫘閫氱殑鏁版嵁瀛楄妭銆侱ataLow 涓?DataHigh 琛ㄧず 16 浣嶅瓧涓殑浣庡瓧鑺備笌楂樺瓧鑺傘€?
Count (8 bits)  涓€涓寘鍚潡鎿嶄綔闀垮害鐨勬暟鎹瓧鑺傘€?
[..]            鐢?I2C 璁惧鍙戦€佺殑鏁版嵁锛屼笌涓绘満閫傞厤鍣ㄥ彂閫佺殑鏁版嵁鐩稿銆?
=============== =============================================================

## SMBus Quick Command

  S Addr Rd/Wr [A] P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_QUICK

璇ュ懡浠ゅ悜璁惧鍐欏叆涓€涓瘮鐗癸紙浣嶄簬 Rd/Wr 浣嶄腑锛夈€傞儴鍒嗚澶囦細鍊熸瑙﹀彂鏌愪釜鍔ㄤ綔銆?

## SMBus Receive Byte

  S Addr Rd [A] [Data] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_READ_BYTE

鐢?i2c_smbus_read_byte() 瀹炵幇銆?

姝ゆ搷浣滀粠璁惧璇诲彇涓€涓瓧鑺傦紝涓斾笉鎸囧畾璁惧瀵勫瓨鍣ㄣ€傛湁浜涜澶囬潪甯哥畝鍗曪紝杩欎釜鎺ュ彛灏辫冻澶熶簡锛涘浜庡叾瀹冭澶囷紝濡傛灉浣犲笇鏈涜鍙栦笌涓嬫枃鐩稿悓鐨勫瘎瀛樺櫒锛屽畠鍙槸涓€绉嶇畝鍐欏舰寮忋€?

## SMBus Send Byte

  S Addr Wr [A] [Data] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_WRITE_BYTE

鐢?i2c_smbus_write_byte() 瀹炵幇銆?

杩欐槸 Receive Byte 鐨勯€嗘搷浣滐細瀹冨悜璁惧鍙戦€佷竴涓瓧鑺傘€傛洿澶氫俊鎭鍙傞槄鈥淩eceive Byte鈥濄€?

## SMBus Read Byte

  S Addr Wr [A] Comm [A] Sr Addr Rd [A] [Data] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_READ_BYTE_DATA

鐢?i2c_smbus_read_byte_data() 瀹炵幇銆?

姝ゆ搷浣滀粠涓€涓寚瀹氱殑璁惧瀵勫瓨鍣紙閫氳繃 Comm 鎸囧畾锛夎鍙栦竴涓瓧鑺傘€?

## SMBus Read Word

  S Addr Wr [A] Comm [A] Sr Addr Rd [A] [DataLow] A [DataHigh] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_READ_WORD_DATA

鐢?i2c_smbus_read_word_data() 瀹炵幇銆?

璇ユ搷浣滀笌 Read Byte 闈炲父鐩镐技锛涘悓鏍锋槸浠庤澶囥€佷粠涓€涓€氳繃 Comm 鎸囧畾鐨勫瘎瀛樺櫒璇诲彇鏁版嵁銆傛敞鎰忥紝瀵逛簬涓や釜鏁版嵁瀛楄妭椤哄簭鐩稿弽锛堜笉绗﹀悎 SMBus锛屼絾闈炲父娴佽锛夌殑璇诲彇锛屽彲浠ヤ娇鐢ㄤ究鎹峰嚱鏁?i2c_smbus_read_word_swapped()銆?

## SMBus Write Byte

  S Addr Wr [A] Comm [A] [Data] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_WRITE_BYTE_DATA

鐢?i2c_smbus_write_byte_data() 瀹炵幇銆?

姝ゆ搷浣滃悜璁惧鐨勪竴涓寚瀹氬瘎瀛樺櫒鍐欏叆涓€涓瓧鑺傘€傚瘎瀛樺櫒閫氳繃 Comm 瀛楄妭鎸囧畾銆傝繖鏄?Read Byte 鎿嶄綔鐨勯€嗘搷浣溿€?

## SMBus Write Word

  S Addr Wr [A] Comm [A] [DataLow] A [DataHigh] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_WRITE_WORD_DATA

鐢?i2c_smbus_write_word_data() 瀹炵幇銆?

杩欐槸 Read Word 鎿嶄綔鐨勯€嗘搷浣滐紝鍚戣澶囥€佸悜鎸囧畾鐨勫瘎瀛樺櫒鍐欏叆 16 浣嶆暟鎹€傛敞鎰忥紝瀵逛簬涓や釜鏁版嵁瀛楄妭椤哄簭鐩稿弽锛堜笉绗﹀悎 SMBus锛屼絾闈炲父娴佽锛夌殑鍐欏叆锛屽彲浠ヤ娇鐢ㄤ究鎹峰嚱鏁?i2c_smbus_write_word_swapped()銆?

## SMBus Process Call

  S Addr Wr [A] Comm [A] [DataLow] A [DataHigh] NA Sr Addr Rd [A] [DataLow] A [DataHigh] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_PROC_CALL

鐢?i2c_smbus_proc_call() 瀹炵幇銆?

璇ュ懡浠ら€夋嫨涓€涓澶囧瘎瀛樺櫒锛堥€氳繃 Comm 瀛楄妭锛夛紝鍙戦€?16 浣嶆暟鎹紝鍐嶈鍥?16 浣嶆暟鎹€?

## SMBus Block Read

  S Addr Wr [A] Comm [A] Sr Addr Rd [A] [Count] A [Data] ... A P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_READ_BLOCK_DATA

鐢?i2c_smbus_read_block_data() 瀹炵幇銆?

姝ゅ懡浠や粠涓€涓寚瀹氱殑璁惧瀵勫瓨鍣紙閫氳繃 Comm 瀛楄妭鎸囧畾锛夎鍙栨渶澶?32 瀛楄妭鐨勫潡銆傛暟鎹噺鐢辫澶囬€氳繃 Count 瀛楄妭鎸囧畾銆?

## SMBus Block Write

  S Addr Wr [A] Comm [A] [Count] A [Data] ... A P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_WRITE_BLOCK_DATA

鐢?i2c_smbus_write_block_data() 瀹炵幇銆?

杩欐槸 Block Read 鍛戒护鐨勯€嗘搷浣滐紝鍚戣澶囥€佸悜閫氳繃 Comm 瀛楄妭鎸囧畾鐨勫瘎瀛樺櫒鍐欏叆鏈€澶?32 瀛楄妭銆傛暟鎹噺鍦?Count 瀛楄妭涓寚瀹氥€?

## SMBus Block Write - Block Read Process Call

  S Addr Wr [A] Comm [A] [Count] A [Data] ... A Sr Addr Rd [A] [Count] A [Data] ... A P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_BLOCK_PROC_CALL

鐢?i2c_smbus_block_proc_call() 瀹炵幇銆?

SMBus Block Write - Block Read Process Call 鍦ㄨ鑼冪殑 2.0 淇鐗堜腑寮曞叆銆傚畠鍏堝啓鍏ヤ竴涓暟鎹潡锛屽啀璇诲洖涓€涓暟鎹潡銆?

## SMBus Host Notify

  [S] [HostAddr] [Wr] A [DevAddr] A [DataLow] A [DataHigh] A [P]

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_HOST_NOTIFY

璇ュ懡浠ょ敱鍏呭綋涓昏澶囩殑 SMBus 璁惧鍙戦€佺粰鍏呭綋浠庤澶囩殑 SMBus 涓绘満銆傚畠鐨勫舰寮忎笌 Write Word 鐩稿悓锛屽彧鏄懡浠ょ爜琚浛鎹负鎶ヨ璁惧鐨勫湴鍧€銆?

鍦?Linux 鍐呮牳涓紝瀹冪殑瀹炵幇鏂瑰紡濡備笅锛?

- 鏀寔 SMBus Host Notify 鐨?I2C 鎬荤嚎椹卞姩搴旀姤鍛?I2C_FUNC_SMBUS_HOST_NOTIFY銆?
- 瀵逛簬鑳藉瑙﹀彂 SMBus Host Notify 鐨勮澶囷紝鍏?I2C 椹卞姩濡傛灉娌℃湁琚叾浠栦汉鎸囧畾鍏跺畠涓柇锛屽垯 client->irq 浼氳鍒嗛厤涓轰竴涓?Host Notify IRQ銆?

## Packet Error Checking (PEC)

Packet Error Checking 鍦ㄨ鑼冪殑 1.1 淇鐗堜腑寮曞叆銆侾EC 鍦ㄤ娇鐢ㄥ畠鐨勪紶杈撲腑銆佺揣鎺ュ湪缁堟鐨?STOP 涔嬪墠锛屾坊鍔犱竴涓?CRC-8 閿欒妫€鏌ュ瓧鑺傘€?

## Address Resolution Protocol (ARP)

鍦板潃瑙ｆ瀽鍗忚锛圓ddress Resolution Protocol锛夋槸鍦ㄨ鑼冪殑 2.0 淇鐗堜腑寮曞叆鐨勩€傚畠鏄竴涓娇鐢ㄤ笂杩版秷鎭殑鏇撮珮灞傚崗璁€侫RP 涓哄崗璁鍔犱簡璁惧鏋氫妇涓庡姩鎬佸湴鍧€鍒嗛厤鍔熻兘銆傛墍鏈?ARP 閫氫俊閮戒娇鐢ㄤ粠鏈哄湴鍧€ 0x61锛屽苟涓旈渶瑕?PEC 鏍￠獙鍜屻€?

## SMBus Alert

SMBus 鎶ヨ鍗忚鍦ㄨ鑼冪殑 1.0 淇鐗堜腑寮曞叆銆係MBus 鎶ヨ鍗忚鍏佽澶氫釜 SMBus 浠庤澶囧叡浜?SMBus 涓昏澶囦笂鐨勪竴涓腑鏂紩鑴氾紝鍚屾椂浠嶅厑璁镐富璁惧鐭ラ亾鏄摢涓粠璁惧瑙﹀彂浜嗕腑鏂€?

杩欏湪 Linux 鍐呮牳涓寜浠ヤ笅鏂瑰紡瀹炵幇锛?

- 鏀寔 SMBus Alert 鐨?I2C 鎬荤嚎椹卞姩搴旇皟鐢?i2c_new_smbus_alert_device() 鏉ュ畨瑁?SMBus Alert 鏀寔銆?
- I2C 鎬荤嚎椹卞姩閫氳繃璋冪敤鐩稿簲鎺ュ彛鏉ヨЕ鍙?SMBus Host Notify銆?

## I2C 鍧椾簨鍔?

I2C 鍧椾簨鍔′笉闄愬埗浼犺緭鐨勫瓧鑺傛暟锛屼絾 SMBus 灞傛柦鍔犱簡 32 瀛楄妭鐨勯檺鍒躲€?

  S Addr Wr [A] Comm [A]
            Sr Addr Rd [A] [Data] A [Data] A ... A [Data] NA P

鍔熻兘鏍囧織锛欼2C_FUNC_SMBUS_READ_I2C_BLOCK

鐢?i2c_smbus_read_i2c_block_data() 瀹炵幇銆?

姝ゅ懡浠や粠涓€涓寚瀹氬瘎瀛樺櫒璇诲彇瀛楄妭銆傛敞鎰忥紝闀垮害涓?0銆? 鎴栨洿澶氬瓧鑺傜殑鍛戒护鏄彈鏀寔鐨勶紝鍥犱负瀹冧滑涓庢暟鎹棤娉曞尯鍒嗐€?

  S Addr Wr [A] Comm [A] Data [A] Data [A] ... [A] Data [A] P

鐢?i2c_smbus_write_i2c_block_data() 瀹炵幇銆?

杩欐槸鍧楄鍙栧懡浠ょ殑閫嗘搷浣滐紝鍚戣澶囥€佸悜閫氳繃 Comm 瀛楄妭鎸囧畾鐨勫瘎瀛樺櫒鍐欏叆瀛楄妭銆?
