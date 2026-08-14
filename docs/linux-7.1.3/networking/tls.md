
## 鍐呮牳 TLS

## 姒傝堪

浼犺緭灞傚畨鍏紙Transport Layer Security锛孴LS锛夋槸涓€绉嶈繍琛屼簬 TCP 涔嬩笂鐨勪笂灞傚崗璁?
锛圲pper Layer Protocol锛孶LP锛夈€俆LS 鎻愪緵绔埌绔殑鏁版嵁瀹屾暣鎬т笌鏈哄瘑鎬с€?

## 鐢ㄦ埛鎺ュ彛

### 鍒涘缓 TLS 杩炴帴

棣栧厛鍒涘缓涓€涓柊鐨?TCP 濂楁帴瀛楋紝鍦ㄨ繛鎺ュ缓绔嬪悗璁剧疆 TLS ULP銆?

  sock = socket(AF_INET, SOCK_STREAM, 0);
  connect(sock, addr, addrlen);
  setsockopt(sock, SOL_TCP, TCP_ULP, "tls", sizeof("tls"));

璁剧疆 TLS ULP 鍚庯紝鎴戜滑灏卞彲浠ュ TLS 濂楁帴瀛楅€夐」杩涜璁剧疆/鑾峰彇銆傚綋鍓嶅彧鏈夊绉板姞瀵?
鐢卞唴鏍稿鐞嗐€傚湪 TLS 鎻℃墜瀹屾垚鍚庯紝鎴戜滑灏辨嫢鏈変簡灏嗘暟鎹矾寰勮縼绉诲埌鍐呮牳鎵€闇€鐨勫叏閮ㄥ弬鏁般€?
鍙戦€佸拰鎺ユ敹鍒嗗埆鏈夌嫭绔嬬殑濂楁帴瀛楅€夐」鐢ㄤ簬灏嗗叾杩佺Щ鍒板唴鏍搞€?

  /** From linux/tls.h **/
  struct tls_crypto_info {
          unsigned short version;
          unsigned short cipher_type;
  };

  struct tls12_crypto_info_aes_gcm_128 {
          struct tls_crypto_info info;
          unsigned char iv[TLS_CIPHER_AES_GCM_128_IV_SIZE];
          unsigned char key[TLS_CIPHER_AES_GCM_128_KEY_SIZE];
          unsigned char salt[TLS_CIPHER_AES_GCM_128_SALT_SIZE];
          unsigned char rec_seq[TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE];
  };


  struct tls12_crypto_info_aes_gcm_128 crypto_info;

  crypto_info.info.version = TLS_1_2_VERSION;
  crypto_info.info.cipher_type = TLS_CIPHER_AES_GCM_128;
  memcpy(crypto_info.iv, iv_write, TLS_CIPHER_AES_GCM_128_IV_SIZE);
  memcpy(crypto_info.rec_seq, seq_number_write,
					TLS_CIPHER_AES_GCM_128_REC_SEQ_SIZE);
  memcpy(crypto_info.key, cipher_key_write, TLS_CIPHER_AES_GCM_128_KEY_SIZE);
  memcpy(crypto_info.salt, implicit_iv_write, TLS_CIPHER_AES_GCM_128_SALT_SIZE);

  setsockopt(sock, SOL_TLS, TLS_TX, &crypto_info, sizeof(crypto_info));

鍙戦€佷笌鎺ユ敹鏄垎鍒缃殑锛屼絾璁剧疆鏂瑰紡鐩稿悓锛屽彧闇€浣跨敤 TLS_TX 鎴?TLS_RX 鍏朵腑涔嬩竴銆?

### 鍙戦€?TLS 搴旂敤鏁版嵁

鍦ㄨ缃?TLS_TX 濂楁帴瀛楅€夐」鍚庯紝閫氳繃璇ュ鎺ュ瓧鍙戦€佺殑鎵€鏈夊簲鐢ㄦ暟鎹兘浼氫娇鐢?TLS 浠ュ強
璇ュ鎺ュ瓧閫夐」涓彁渚涚殑鍙傛暟杩涜鍔犲瘑銆備緥濡傦紝鎴戜滑鍙互濡備笅鍙戦€佷竴鏉″姞瀵嗙殑 hello world
璁板綍锛?

  const char *msg = "hello world\n";
  send(sock, msg, strlen(msg));

濡傛灉鍙兘锛宻end() 鐨勬暟鎹細鐩存帴浠庣敤鎴风┖闂存彁渚涚殑缂撳啿鍖哄姞瀵嗗埌鍐呮牳鐨勫姞瀵嗗彂閫佺紦鍐插尯涓€?

sendfile 绯荤粺璋冪敤浼氫互鏈€澶ч暱搴︼紙2^14锛夌殑 TLS 璁板綍鍙戦€佹枃浠舵暟鎹€?

  file = open(filename, O_RDONLY);
  fstat(file, &stat);
  sendfile(sock, file, &offset, stat.st_size);

闄ら潪浼犲叆 MSG_MORE锛屽惁鍒欐瘡娆?send() 璋冪敤鍚庨兘浼氬垱寤哄苟鍙戦€?TLS 璁板綍銆侻SG_MORE 浼?
鎺ㄨ繜璁板綍鐨勫垱寤猴紝鐩村埌涓嶅啀浼犲叆 MSG_MORE 鎴栬揪鍒版渶澶ц褰曞ぇ灏忎负姝€?

鍐呮牳闇€瑕佷负鍔犲瘑鏁版嵁鍒嗛厤缂撳啿鍖恒€傝缂撳啿鍖哄湪璋冪敤 send() 鏃跺垎閰嶏紝鍥犳瑕佷箞鏁翠釜 send()
璋冪敤杩斿洖 -ENOMEM锛堟垨闃诲绛夊緟鍐呭瓨锛夛紝瑕佷箞鍔犲瘑涓€瀹氫細鎴愬姛銆傚鏋?send() 杩斿洖 -ENOMEM锛?
涓斾笂涓€娆′娇鐢?MSG_MORE 鐨勮皟鐢ㄥ湪濂楁帴瀛楃紦鍐插尯涓粛娈嬬暀鏈夋暟鎹紝鍒?MSG_MORE 鐨勬暟鎹細
淇濈暀鍦ㄥ鎺ュ瓧缂撳啿鍖轰腑銆?

### 鎺ユ敹 TLS 搴旂敤鏁版嵁

鍦ㄨ缃?TLS_RX 濂楁帴瀛楅€夐」鍚庯紝鎵€鏈?recv 绯诲垪鐨勫鎺ュ瓧璋冪敤閮戒細浣跨敤鎻愪緵鐨?TLS 鍙傛暟
杩涜瑙ｅ瘑銆傚繀椤绘帴鏀跺埌涓€涓畬鏁寸殑 TLS 璁板綍鍚庢墠鑳借繘琛岃В瀵嗐€?

  char buffer[^16384^];
  recv(sock, buffer, 16384);

濡傛灉鐢ㄦ埛鐨勭紦鍐插尯瓒冲澶э紝鎺ユ敹鍒扮殑鏁版嵁浼氱洿鎺ヨВ瀵嗗埌鐢ㄦ埛缂撳啿鍖轰腑锛屼笉浼氬彂鐢熼澶栫殑
鍒嗛厤銆傚鏋滅敤鎴风┖闂寸紦鍐插尯澶皬锛屾暟鎹細鍏堝湪鍐呮牳涓В瀵嗗啀鎷疯礉鍒扮敤鎴风┖闂淬€?

濡傛灉鎺ユ敹鍒扮殑娑堟伅涓殑 TLS 鐗堟湰涓?setsockopt 浼犲叆鐨勭増鏈笉涓€鑷达紝杩斿洖 `EINVAL`銆?

濡傛灉鎺ユ敹鍒扮殑娑堟伅杩囧ぇ锛岃繑鍥?`EMSGSIZE`銆?

濡傛灉鍥犱换浣曞叾浠栧師鍥犲鑷磋В瀵嗗け璐ワ紝杩斿洖 `EBADMSG`銆?

### 鍙戦€?TLS 鎺у埗娑堟伅

闄ゅ簲鐢ㄦ暟鎹锛孴LS 杩樻湁鎺у埗娑堟伅锛屼緥濡傚憡璀︽秷鎭紙璁板綍绫诲瀷 21锛夊拰鎻℃墜娑堟伅锛堣褰曠被鍨?
22锛夌瓑銆傝繖浜涙秷鎭彲浠ラ€氳繃 CMSG 鎻愪緵 TLS 璁板綍绫诲瀷鏉ョ粡鐢卞鎺ュ瓧鍙戦€併€備緥濡傦紝涓嬮潰鐨?
鍑芥暟浣跨敤绫诲瀷涓?@record_type 鐨勮褰曞彂閫?@length 瀛楄妭鐨?@data銆?

  /** send TLS control message using record_type **/
  static int klts_send_ctrl_message(int sock, unsigned char record_type,
                                    void *data, size_t length)
  {
        struct msghdr msg = {0};
        int cmsg_len = sizeof(record_type);
        struct cmsghdr *cmsg;
        char buf[CMSG_SPACE(cmsg_len)];
        struct iovec msg_iov;   /** Vector of data to send/receive into.  **/

        msg.msg_control = buf;
        msg.msg_controllen = sizeof(buf);
        cmsg = CMSG_FIRSTHDR(&msg);
        cmsg->cmsg_level = SOL_TLS;
        cmsg->cmsg_type = TLS_SET_RECORD_TYPE;
        cmsg->cmsg_len = CMSG_LEN(cmsg_len);
        *CMSG_DATA(cmsg) = record_type;
        msg.msg_controllen = cmsg->cmsg_len;

        msg_iov.iov_base = data;
        msg_iov.iov_len = length;
        msg.msg_iov = &msg_iov;
        msg.msg_iovlen = 1;

        return sendmsg(sock, &msg, 0);
  }

鎺у埗娑堟伅鏁版嵁搴斾互鏈姞瀵嗗舰寮忔彁渚涳紝骞剁敱鍐呮牳鍔犲瘑銆?

### 鎺ユ敹 TLS 鎺у埗娑堟伅

TLS 鎺у埗娑堟伅浼氫紶鍏ョ敤鎴风┖闂寸紦鍐插尯锛屾秷鎭被鍨嬮€氳繃 cmsg 浼犻€掋€傚鏋滄湭鎻愪緵 cmsg 缂撳啿鍖猴紝
鍒欐帴鏀跺埌鎺у埗娑堟伅鏃朵細杩斿洖閿欒銆傛暟鎹秷鎭彲浠ュ湪鏈缃?cmsg 缂撳啿鍖虹殑鎯呭喌涓嬫帴鏀躲€?

  char buffer[^16384^];
  char cmsg[CMSG_SPACE(sizeof(unsigned char))];
  struct msghdr msg = {0};
  msg.msg_control = cmsg;
  msg.msg_controllen = sizeof(cmsg);

  struct iovec msg_iov;
  msg_iov.iov_base = buffer;
  msg_iov.iov_len = 16384;

  msg.msg_iov = &msg_iov;
  msg.msg_iovlen = 1;

  int ret = recvmsg(sock, &msg, 0 /** flags **/);

  struct cmsghdr *cmsg = CMSG_FIRSTHDR(&msg);
  if (cmsg->cmsg_level == SOL_TLS &&
      cmsg->cmsg_type == TLS_GET_RECORD_TYPE) {
      int record_type = **((unsigned char **)CMSG_DATA(cmsg));
      // Do something with record_type, and control message data in
      // buffer.
      //
      // Note that record_type may be == to application data (23).
  } else {
      // Buffer contains application data.
  }

recv 姘歌繙涓嶄細杩斿洖鏉ヨ嚜涓嶅悓绫诲瀷 TLS 璁板綍娣峰悎鐨勬暟鎹€?

### TLS 1.3 瀵嗛挜鏇存柊

鍦?TLS 1.3 涓紝KeyUpdate 鎻℃墜娑堟伅琛ㄧず鍙戦€佹柟姝ｅ湪鏇存柊鍏?TX 瀵嗛挜銆侹eyUpdate 涔嬪悗鍙戦€佺殑
浠讳綍娑堟伅閮戒細浣跨敤鏂板瘑閽ュ姞瀵嗐€傜敤鎴风┖闂村簱鍙互鍍忔彁渚涘垵濮嬪瘑閽ヤ竴鏍凤紝閫氳繃 TLS_TX 鍜?TLS_RX
濂楁帴瀛楅€夐」灏嗘柊瀵嗛挜浼犻€掔粰鍐呮牳銆俆LS 鐗堟湰鍜屽姞瀵嗗浠朵笉鑳芥洿鏀广€?

涓洪槻姝娇鐢ㄩ敊璇瘑閽ュ皾璇曡В瀵嗕紶鍏ヨ褰曪紝褰撳唴鏍告帴鏀跺埌 KeyUpdate 娑堟伅鏃朵細鏆傚仠瑙ｅ瘑锛岀洿鍒?
閫氳繃 TLS_RX 濂楁帴瀛楅€夐」鎻愪緵鏂板瘑閽ヤ负姝€傚湪璇诲彇鍒?KeyUpdate 涔嬪悗銆佹彁渚涙柊瀵嗛挜涔嬪墠鍙戠敓鐨?
浠讳綍璇诲彇閮戒細浠?EKEYEXPIRED 澶辫触銆傚湪鎻愪緵鏂板瘑閽ヤ箣鍓嶏紝poll() 涓嶄細鎶ュ憡鏉ヨ嚜璇ュ鎺ュ瓧鐨勪换浣?
璇诲彇浜嬩欢銆傚彂閫佷晶娌℃湁鏆傚仠鏈哄埗銆?

鐢ㄦ埛绌洪棿搴旂‘淇濇墍鎻愪緵鐨?crypto_info 宸茶姝ｇ‘璁剧疆銆傜壒鍒槸锛屽唴鏍镐笉浼氭鏌ュ瘑閽?nonce 鐨?
閲嶇敤銆?

鎴愬姛鍜屽け璐ョ殑瀵嗛挜鏇存柊娆℃暟鍒嗗埆鍦?`TlsTxRekeyOk`銆乣TlsRxRekeyOk`銆乣TlsTxRekeyError`銆?
`TlsRxRekeyError` 缁熻椤逛腑璺熻釜銆俙TlsRxRekeyReceived` 缁熻椤硅褰曞凡鎺ユ敹鍒扮殑 KeyUpdate
鎻℃墜娑堟伅鏁伴噺銆?

### 闆嗘垚鍒扮敤鎴风┖闂?TLS 搴?

浠庨珮灞傛潵鐪嬶紝鍐呮牳 TLS ULP 鏄竴涓敤鎴风┖闂?TLS 搴撹褰曞眰锛坮ecord layer锛夌殑鏇夸唬鍝併€?

灏?OpenSSL 鎵撹ˉ涓佷互浣跨敤 ktls 浣滀负璁板綍灞傜殑琛ヤ竵闆嗗湪
`姝ゅ <https://github.com/Mellanox/openssl/commits/tls_rx2>`_銆?

`涓€涓ず渚?<https://github.com/ktls/af_ktls-tool/commits/RX>`_锛?
鍦ㄦ彙鎵嬩箣鍚庣洿鎺ヤ娇鐢?gnutls 璋冪敤 send銆傜敱浜庡畠娌℃湁瀹炵幇瀹屾暣鐨勮褰曞眰锛屽洜姝や笉鏀寔鎺у埗
娑堟伅銆?

### 鍙€変紭鍖?

濡傛灉鏄惧紡璇锋眰锛孴LS ULP 鍙互鍋氭煇浜涢拡瀵圭壒瀹氭潯浠剁殑浼樺寲銆傝繖浜涗紭鍖栬涔堝苟闈炴櫘閬嶆湁鐩婏紝
瑕佷箞鍙兘褰卞搷姝ｇ‘鎬э紝鍥犳闇€瑕佹樉寮忓紑鍚紙opt-in锛夈€傛墍鏈夐€夐」閮介€氳繃 setsockopt() 鎸?
濂楁帴瀛楄缃紝鍏剁姸鎬佸彲閫氳繃 getsockopt() 浠ュ強濂楁帴瀛楄瘖鏂紙`ss`锛夋煡鐪嬨€?

#### TLS_TX_ZEROCOPY_RO

浠呯敤浜庤澶囧嵏杞姐€傚厑璁?sendfile() 鐨勬暟鎹洿鎺ヤ紶杈撳埌 NIC锛岃€屾棤闇€鍦ㄥ唴鏍镐腑鎷疯礉銆傝繖鏍峰湪
鍚敤璁惧鍗歌浇鏃跺彲浠ュ疄鐜扮湡姝ｇ殑闆舵嫹璐濊涓恒€?

搴旂敤绋嬪簭蹇呴』纭繚鏁版嵁鍦ㄦ彁浜や笌浼犺緭瀹屾垚涔嬮棿涓嶈淇敼銆傛崲鍙ヨ瘽璇达紝杩欎富瑕侀€傜敤浜庨€氳繃
sendfile() 鍦ㄥ鎺ュ瓧涓婂彂閫佺殑鏁版嵁鏄彧璇荤殑鎯呭喌銆?

淇敼鏁版嵁鍙兘瀵艰嚧鍘熷 TCP 浼犺緭鍜?TCP 閲嶄紶浣跨敤涓嶅悓鐗堟湰鐨勬暟鎹€傚鎺ユ敹鏂硅€岃█锛岃繖鐪嬭捣鏉?
灏卞儚鏄?TLS 璁板綍琚鏀癸紝骞朵細瀵艰嚧璁板綍璁よ瘉澶辫触銆?

#### TLS_RX_EXPECT_NO_PAD

浠呯敤浜?TLS 1.3銆傛湡鏈涘彂閫佹柟涓嶅璁板綍杩涜濉厖銆傝繖鏍峰彲浠ュ湪 TLS 1.3 涓嬪皢鏁版嵁鐩存帴瑙ｅ瘑鍒?
鐢ㄦ埛绌洪棿缂撳啿鍖恒€?

鍙湁鍦ㄨ繙绔彲淇＄殑鎯呭喌涓嬫墠閫傚悎寮€鍚浼樺寲锛屽惁鍒欏畠浼氭垚涓轰竴涓皢 TLS 澶勭悊鎴愭湰缈诲€嶇殑
鏀诲嚮鍚戦噺銆?

濡傛灉瑙ｅ瘑鍚庣殑璁板綍鍙戠幇鏇捐濉厖銆佹垨涓嶆槸鏁版嵁璁板綍锛屽垯浼氬啀娆¤В瀵嗗埌涓€涓唴鏍哥紦鍐插尯涓紝
鑰屼笉浣跨敤闆舵嫹璐濄€傛绫讳簨浠惰鍏?`TlsDecryptRetry` 缁熻椤广€?

#### TLS_TX_MAX_PAYLOAD_LEN

鎸囧畾鎵€鍙戦€?TLS 璁板綍鏄庢枃璐熻浇鐨勬渶澶уぇ灏忋€?

璁剧疆姝ら€夐」鍚庯紝鍐呮牳浼氬鎵€鏈夊嚭绔?TLS 璁板綍寮哄埗鎵ц璇ラ檺鍒躲€傛病鏈変换浣曟槑鏂囧垎鐗囦細瓒呰繃璇ュぇ灏忋€?
璇ラ€夐」鍙敤浜庡疄鐜?TLS Record Size Limit 鎵╁睍 [^1^]銆?

- 瀵逛簬 TLS 1.2锛岃鍊肩洿鎺ュ搴旇褰曞ぇ灏忛檺鍒躲€?
- 瀵逛簬 TLS 1.3锛岃鍊煎簲璁句负 record_size_limit - 1锛屽洜涓鸿褰曞ぇ灏忛檺鍒朵负 ContentType
  瀛楁棰濆鍖呭惈浜嗕竴涓瓧鑺傘€?

璇ラ€夐」鐨勬湁鏁堣寖鍥存槸锛歍LS 1.2 涓?64 鍒?16384 瀛楄妭锛孴LS 1.3 涓?63 鍒?16384 瀛楄妭銆?
TLS 1.3 鐨勬渶灏忎笅闄愭洿浣庯紝鏄洜涓?ContentType 瀛楁棰濆鍗犵敤浜嗕竴涓瓧鑺傘€?

[^1^] https://datatracker.ietf.org/doc/html/rfc8449

## 缁熻淇℃伅

TLS 瀹炵幇鏆撮湶浜嗕互涓嬫瘡涓懡鍚嶇┖闂寸殑缁熻淇℃伅锛坄/proc/net/tls_stat`锛夛細

- `TlsCurrTxSw`, `TlsCurrRxSw` -
  褰撳墠宸插畨瑁呫€佺敱涓绘満澶勭悊鍔犲瘑鐨?TX 涓?RX 浼氳瘽鏁伴噺

- `TlsCurrTxDevice`, `TlsCurrRxDevice` -
  褰撳墠宸插畨瑁呫€佺敱 NIC 澶勭悊鍔犲瘑鐨?TX 涓?RX 浼氳瘽鏁伴噺

- `TlsTxSw`, `TlsRxSw` -
  浠ヤ富鏈哄姞瀵嗘柟寮忔墦寮€鐨?TX 涓?RX 浼氳瘽鏁伴噺

- `TlsTxDevice`, `TlsRxDevice` -
  浠?NIC 鍔犲瘑鏂瑰紡鎵撳紑鐨?TX 涓?RX 浼氳瘽鏁伴噺

- `TlsDecryptError` -
  璁板綍瑙ｅ瘑澶辫触锛堜緥濡傜敱浜庤璇佹爣绛句笉姝ｇ‘锛?

- `TlsDeviceRxResync` -
  鍙戦€佺粰澶勭悊鍔犲瘑鐨?NIC 鐨?RX 閲嶆柊鍚屾娆℃暟

- `TlsDecryptRetry` -
  鐢变簬 `TLS_RX_EXPECT_NO_PAD` 棰勬祴閿欒鑰屼笉寰椾笉閲嶆柊瑙ｅ瘑鐨?RX 璁板綍鏁伴噺銆?
  娉ㄦ剰璇ヨ鏁板櫒涔熶細鍥犻潪鏁版嵁璁板綍鑰岄€掑銆?

- `TlsRxNoPadViolation` -
  鐢变簬 `TLS_RX_EXPECT_NO_PAD` 棰勬祴閿欒鑰屼笉寰椾笉閲嶆柊瑙ｅ瘑鐨勬暟鎹?RX 璁板綍鏁伴噺銆?

- `TlsTxRekeyOk`, `TlsRxRekeyOk` -
  鐜版湁浼氳瘽涓?TX 涓?RX 鎴愬姛閲嶆柊瀵嗛挜锛坮ekey锛夌殑娆℃暟

- `TlsTxRekeyError`, `TlsRxRekeyError` -
  鐜版湁浼氳瘽涓?TX 涓?RX 閲嶆柊瀵嗛挜澶辫触鐨勬鏁?

- `TlsRxRekeyReceived` -
  鎺ユ敹鍒扮殑 KeyUpdate 鎻℃墜娑堟伅鏁伴噺锛岃姹傜敤鎴风┖闂存彁渚涙柊鐨?RX 瀵嗛挜
