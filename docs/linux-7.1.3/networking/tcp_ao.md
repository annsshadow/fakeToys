
## TCP 璁よ瘉閫夐」 Linux 瀹炵幇锛圧FC5925锛?

TCP 璁よ瘉閫夐」锛圱CP-AO锛夋彁渚涗簡涓€涓棬鍦ㄩ獙璇佸彲淇″绛変綋涔嬮棿鎶ユ枃娈电殑 TCP 鎵╁睍銆傚畠娣诲姞
浜嗕竴涓柊鐨?TCP 澶撮儴閫夐」锛屽叾涓寘鍚竴涓秷鎭璇佺爜锛圡AC锛夈€侻AC 鏄娇鐢ㄤ竴涓弻鏂归兘鐭ラ亾
鍙ｄ护鐨勫搱甯屽嚱鏁帮紝浠?TCP 鎶ユ枃娈电殑鍐呭璁＄畻寰楀嚭鐨勩€俆CP-AO 鐨勬剰鍥炬槸搴熷純 TCP-MD5锛屾彁渚?鏇村ソ鐨勫畨鍏ㄦ€с€佸瘑閽ヨ疆鎹互鍙婂澶氱鍝堝笇绠楁硶鐨勬敮鎸併€?
## 1. 绠€浠?

| | TCP-MD5 | TCP-AO |
|---|---|---|
| 鏀寔鐨勫搱甯岀畻娉?| MD5锛堝瘑鐮佸涓婅緝寮憋級 | 蹇呴』鏀寔 HMAC-SHA1锛堥€夋嫨鍓嶇紑鏀诲嚮锛夊拰 CMAC-AES-128锛堜粎鏃佽矾鏀诲嚮锛夈€傚彲鑳芥敮鎸佷换鎰忓搱甯岀畻娉曘€?|
| MAC 闀垮害锛堝瓧鑺傦級 | 16 | 閫氬父 12-16銆傚厑璁稿叾瀹冭兘鏀惧叆 TCP 澶撮儴鐨勫彉浣撱€?|
| 姣忎釜 TCP 杩炴帴鐨勫瘑閽ユ暟 | 1 | 澶氫釜 |
| 鏇存敼娲诲姩瀵嗛挜鐨勫彲鑳芥€?| 涓嶅疄鐢紙鍙屾柟閮藉繀椤诲湪 MSL 鏈熼棿鏇存敼锛?| 鍗忚鏀寔 |
| 閽堝 ICMP 鈥滅‖閿欒鈥?鐨勪繚鎶?| 鍚?| 鏄細榛樿鍦ㄥ凡寤虹珛杩炴帴涓婂拷鐣ュ畠浠?|
| 閽堝娴侀噺浜ゅ弶鏀诲嚮鐨勪繚鎶?| 鍚?| 鏄細浼ご閮ㄥ寘鍚?TCP 绔彛 |
| 閽堝閲嶆斁 TCP 娈电殑淇濇姢 | 鍚?| 搴忓垪鍙锋墿灞曪紙SNE锛夊拰鍒濆搴忓垪鍙凤紙ISN锛?|
| 鏀寔鏃犺繛鎺ラ噸缃?| 鏄?| 鍚︺€傞渶瑕?ISN+SNE 鎵嶈兘姝ｇ‘绛剧讲 RST |
| 鏍囧噯 | RFC 2385 | RFC 5925銆丷FC 5926 |


### 1.1 缁忓父闂埌鐨勯棶棰橈紙FAQ锛夛紝骞跺紩鐢?RFC 5925


Q锛氬浜庣浉鍚岀殑 4 鍏冪粍锛坰rcaddr銆乻rcport銆乨staddr銆乨stport锛夛紝SendID 鎴?RecvID 鏄惁
鍙兘涓嶅敮涓€锛?
```

   >> The IDs of MKTs MUST NOT overlap where their TCP connection
   identifiers overlap.

```
Q锛氳兘鍚︾Щ闄ゆ椿鍔ㄨ繛鎺ョ殑涓诲瘑閽ュ厓缁勶紙MKT锛夛紵

```

   It is presumed that an MKT affecting a particular connection cannot
   be destroyed during an active connection -- or, equivalently, that
   its parameters are copied to an area local to the connection (i.e.,
   instantiated) and so changes would affect only new connections.

```
Q锛氬鏋滈渶瑕佸垹闄や竴涓棫鐨?MKT锛屽簲璇ュ浣曟搷浣滄墠鑳戒笉鎶婂畠浠庢椿鍔ㄨ繛鎺ヤ笂绉婚櫎锛燂紙鍥犱负瀹冨湪
涔嬪悗浠讳綍鏃跺埢浠嶅彲鑳藉湪浣跨敤锛?
A锛歊FC 5925 鏈寚瀹氾紝杩欎技涔庢槸瀵嗛挜绠＄悊鐨勯棶棰橈紝浠ョ‘淇濆湪灏濊瘯绉婚櫎涔嬪墠娌℃湁浜轰娇鐢ㄨ MKT銆?
Q锛氫竴涓棫鐨?MKT 鑳藉惁姘歌繙瀛樺湪骞惰鍙︿竴涓绛変綋浣跨敤锛?
```

   Deciding when to start using a key is a performance issue. Deciding
   when to remove an MKT is a security issue. Invalid MKTs are expected
   to be removed. TCP-AO provides no mechanism to coordinate their removal,
   as we consider this a key management operation.

```
```

   The only way to avoid reuse of previously used MKTs is to remove the MKT
   when it is no longer considered permitted.

```
Linux TCP-AO 浼氬敖鍔涢樆姝綘绉婚櫎姝ｅ湪浣跨敤鐨勫瘑閽ワ紝灏嗗叾瑙嗕负瀵嗛挜绠＄悊澶辫触銆備絾鐢变簬淇濈暀杩囨椂
鐨勫瘑閽ュ彲鑳戒細鎴愪负瀹夊叏闂锛屽苟涓斿绛変綋鍙兘閫氳繃濮嬬粓灏嗗叾璁剧疆涓?RNextKeyID 鑰屾棤鎰忎腑闃绘
绉婚櫎鏃у瘑閽モ€斺€斿洜姝ゆ彁渚涗簡涓€绉嶅己鍒跺瘑閽ョЩ闄ゆ満鍒讹紝鐢ㄦ埛绌洪棿蹇呴』鎻愪緵瑕佷娇鐢ㄧ殑 KeyID 鏉ユ浛浠?姝ｅ湪琚Щ闄ょ殑閭ｄ釜锛屽唴鏍镐細鍘熷瓙鍦板垹闄ゆ棫瀵嗛挜锛屽嵆浣垮绛変綋浠嶅湪璇锋眰瀹冦€傚己鍒跺垹闄ゆ病鏈変换浣?淇濊瘉锛屽洜涓哄绛変綋鍙兘杩樻病鏈夋柊瀵嗛挜鈥斺€擳CP 杩炴帴鍙兘灏辨涓柇銆傛垨鑰咃紝鍙互閫夋嫨鍏抽棴濂楁帴瀛椼€?
Q锛氬綋鍦ㄤ竴涓柊杩炴帴涓婃帴鏀跺埌涓€涓病鏈夊凡鐭?MKT 鐨?RecvID 鐨勬姤鏂囨鏃讹紝浼氬彂鐢熶粈涔堬紵

A锛歊FC 5925 瑙勫畾榛樿鎯呭喌涓嬫槸鎺ュ彈瀹冨苟璁板綍璀﹀憡锛屼絾
```

   If the segment is a SYN, then this is the first segment of a new
   connection. Find the matching MKT for this segment, using the segment's
   socket pair and its TCP-AO KeyID, matched against the MKT's TCP connection
   identifier and the MKT's RecvID.

      i. If there is no matching MKT, remove TCP-AO from the segment.
         Proceed with further TCP handling of the segment.
         NOTE: this presumes that connections that do not match any MKT
         should be silently accepted, as noted in Section 7.3.

```
```

   >> A TCP-AO implementation MUST allow for configuration of the behavior
   of segments with TCP-AO but that do not match an MKT. The initial default
   of this configuration SHOULD be to silently accept such connections.
   If this is not the desired case, an MKT can be included to match such
   connections, or the connection can indicate that TCP-AO is required.
   Alternately, the configuration can be changed to discard segments with
   the AO option not matching an MKT.

```
```

   Connections not matching any MKT do not require TCP-AO. Further, incoming
   segments with TCP-AO are not discarded solely because they include
   the option, provided they do not match any MKT.

```
璇锋敞鎰忥紝Linux TCP-AO 瀹炵幇鍦ㄨ繖鏂归潰鏈夋墍涓嶅悓銆傜洰鍓嶏紝绛惧悕鏈煡鐨?TCP-AO 鎶ユ枃娈典細琚涪寮?骞惰褰曡鍛娿€?
Q锛歊FC 鏄惁浠ヤ换浣曟柟寮忔殫绀洪泦涓殑鍐呮牳瀵嗛挜绠＄悊锛燂紙鍗虫槸鍚﹁姹傛墍鏈夎繛鎺ヤ笂鐨勫瘑閽ュ繀椤诲悓鏃?杞崲锛燂級

A锛氭湭鎸囧畾銆侻KT 鍙互鍦ㄧ敤鎴风┖闂寸鐞嗭紝鍞竴鐩稿叧鐨勯儴鍒嗘槸
```

   >> All TCP segments MUST be checked against the set of MKTs for matching
   TCP connection identifiers.

```
Q锛氬綋瀵圭瓑浣撹姹傜殑 RNextKeyID 鏈煡鏃朵細鍙戠敓浠€涔堬紵鏄惁搴旇閲嶇疆杩炴帴锛?
```

   ii. If they differ, determine whether the RNextKeyID MKT is ready.

       1. If the MKT corresponding to the segment鈥檚 socket pair and RNextKeyID
       is not available, no action is required (RNextKeyID of a received
       segment needs to match the MKT鈥檚 SendID).

```
Q锛歝urrent_key 鏄浣曡缃殑锛屼綍鏃朵細鍙樺寲锛熷畠鏄敤鎴疯Е鍙戠殑鏇存敼锛岃繕鏄敱杩滅瀵圭瓑浣撶殑
璇锋眰瑙﹀彂锛熸槸鐢辩敤鎴锋樉寮忚缃紝杩樻槸鐢卞尮閰嶈鍒欒缃紵

```

   Rnext_key is changed only by manual user intervention or MKT management
   protocol operation. It is not manipulated by TCP-AO. Current_key is updated
   by TCP-AO when processing received TCP segments as discussed in the segment
   processing description in Section 7.5. Note that the algorithm allows
   the current_key to change to a new MKT, then change back to a previously
   used MKT (known as "backing up"). This can occur during an MKT change when
   segments are received out of order, and is considered a feature of TCP-AO,
   because reordering does not result in drops.

```
```

   2. If the matching MKT corresponding to the segment鈥檚 socket pair and
   RNextKeyID is available:

      a. Set current_key to the RNextKeyID MKT.

```
Q锛氬鏋滀袱涓绛変綋閮芥湁澶氫釜鍖归厤璇ヨ繛鎺ュ鎺ュ瓧瀵圭殑 MKT锛堝叿鏈変笉鍚岀殑 KeyID锛夛紝鍙戦€佹柟/
鎺ユ敹鏂瑰簲濡備綍閫夋嫨瑕佷娇鐢ㄧ殑 KeyID锛?
```

   Multiple MKTs may match a single outgoing segment, e.g., when MKTs
   are being changed. Those MKTs cannot have conflicting IDs (as noted
   elsewhere), and some mechanism must determine which MKT to use for each
   given outgoing segment.

   >> An outgoing TCP segment MUST match at most one desired MKT, indicated
   by the segment鈥檚 socket pair. The segment MAY match multiple MKTs, provided
   that exactly one MKT is indicated as desired. Other information in
   the segment MAY be used to determine the desired MKT when multiple MKTs
   match; such information MUST NOT include values in any TCP option fields.

```
Q锛歍CP-MD5 杩炴帴鑳藉惁杩佺Щ鍒?TCP-AO锛堝弽涔嬩害鐒讹級锛?
```

   TCP MD5-protected connections cannot be migrated to TCP-AO because TCP MD5
   does not support any changes to a connection鈥檚 security algorithm
   once established.

```
Q锛氬鏋滀粠杩炴帴涓婄Щ闄や簡鎵€鏈?MKT锛屽畠鑳藉惁鍙樻垚闈?TCP-AO 绛惧悕鐨勮繛鎺ワ紵

A锛歔7.5.2] 娌℃湁鍍?[7.5.1.i] 涓?SYN 鍖呭鐞嗛偅鏍风殑閫夐」锛堣閫夐」浼氬厑璁告帴鍙楁棤绛惧悕鐨?鎶ユ枃娈碉紝閭ｅ皢鏄笉瀹夊叏鐨勶級銆傝櫧鐒跺垏鎹㈠埌闈?TCP-AO 杩炴帴骞舵湭琚洿鎺ョ姝紝浣嗚繖浼间箮鏄?RFC
鐨勬剰鍥俱€傛澶栵紝TCP-AO 杩炴帴瑕佹眰
```

   TCP-AO requires that every protected TCP segment match exactly one MKT.

```
```

   >> An incoming TCP segment including TCP-AO MUST match exactly one MKT,
   indicated solely by the segment鈥檚 socket pair and its TCP-AO KeyID.

```
```

   One or more MKTs. These are the MKTs that match this connection鈥檚
   socket pair.

```
Q锛氶潪 TCP-AO 杩炴帴鑳藉惁鍙樻垚鍚敤 TCP-AO 鐨勮繛鎺ワ紵

A锛氫笉鑳斤細瀵逛簬涓€涓凡缁忓缓绔嬬殑銆侀潪 TCP-AO 杩炴帴锛屼笉鍙兘鍒囨崲鍒颁娇鐢?TCP-AO锛屽洜涓烘祦閲忓瘑閽?鐨勭敓鎴愰渶瑕佸垵濮嬪簭鍒楀彿銆傛崲鍙ヨ瘽璇达紝寮€濮嬩娇鐢?TCP-AO 闇€瑕侀噸鏂板缓绔?TCP 杩炴帴銆?
## 2. 鍐呮牳鍐?MKT 鏁版嵁搴撲笌鐢ㄦ埛绌洪棿鏁版嵁搴撳姣?

Linux TCP-AO 鏀寔浣跨敤 `setsockopt()` 瀹炵幇锛屾柟寮忎笌 TCP-MD5 绫讳技銆傝繖鎰忓懗鐫€鎯宠浣跨敤
TCP-AO 鐨勭敤鎴风┖闂村簲鐢ㄧ▼搴忓簲璇ュ湪 TCP 濂楁帴瀛椾笂鎵ц `setsockopt()` 鏉ユ坊鍔犮€佺Щ闄ゆ垨杞崲
MKT銆傝繖绉嶆柟娉曞皢瀵嗛挜绠＄悊鐨勮矗浠讳互鍙婂杈圭晫鎯呭喌锛堜緥濡傦紝濡傛灉瀵圭瓑浣撲笉閬靛畧 RNextKeyID 璇?鎬庝箞鍔烇級鐨勫喅绛栫Щ鍒颁簡鐢ㄦ埛绌洪棿锛屽嵆灏嗘洿澶氫唬鐮佺Щ鍒颁簡鐢ㄦ埛绌洪棿锛岀壒鍒槸璐熻矗绛栫暐鍐崇瓥鐨勪唬鐮併€?姝ゅ锛屽畠鐏垫椿涓斿彲鑹ソ鎵╁睍锛堜笌鍐呮牳鍐呮暟鎹簱鐩告瘮闇€瑕佹洿灏戠殑閿侊級銆傝繕搴旇浣忥紝涓昏鐨勭洰鏍囩敤鎴?鏄?BGP 杩涚▼锛岃€屼笉鏄换浣曢殢鏈哄簲鐢ㄧ▼搴忥紝杩欐剰鍛崇潃涓?IPsec 闅ч亾鐩告瘮锛屽疄闄呬笂涓嶉渶瑕侀€忔槑鎬э紝
鑰岀幇浠?BGP 瀹堟姢杩涚▼宸茬粡鏈変簡鐢ㄤ簬 TCP-MD5 鏀寔鐨?`setsockopt()`銆?
| | `setsockopt()` | 鍐呮牳鍐呮暟鎹簱 |
|---|---|---|
| 鍙墿灞曟€?| `setsockopt()` 鍛戒护搴旀槸鍙墿灞曠殑绯荤粺璋冪敤 | Netlink 娑堟伅绠€鍗曚笖鍙墿灞?|
| 鎵€闇€鐨勭敤鎴风┖闂存敼鍔?| 鎯宠 TCP-AO 鐨?BGP 鎴栦换浣曞簲鐢ㄧ▼搴忛渶瑕佹墽琛?`setsockopt()` 骞惰繘琛屽瘑閽ョ鐞?| 鍙互鍍忛毀閬撲竴鏍烽€忔槑锛屾彁渚涚被浼?`ip tcpao add key`锛堝垹闄?鏄剧ず/杞崲锛夌殑鍔熻兘 |
| MKT 鐨勭Щ闄ゆ垨娣诲姞 | 瀵圭敤鎴风┖闂存洿闅?| 瀵瑰唴鏍告洿闅?|
| 鍙浆鍌ㄦ€?| `getsockopt()` | Netlink .dump() 鍥炶皟 |
| 鍐呮牳璧勬簮/鍐呭瓨闄愬埗 | 鐩哥瓑 | 鐩哥瓑 |
| 鍙墿灞曟€?| `TCP_LISTEN` 濂楁帴瀛椾笂鐨勪簤鐢?| 鏁翠釜鏁版嵁搴撲笂鐨勪簤鐢?|
| 鐩戞帶涓庤鍛?| `TCP_DIAG` | 鐩稿悓鐨?Netlink 濂楁帴瀛?|
| MKT 鍖归厤 | 鍗婁釜闂锛氫粎鐩戝惉濂楁帴瀛?| 鍥伴毦 |


## 3. uAPI


Linux 鎻愪緵浜嗕竴缁?`setsockopt()` 鍜?`getsockopt()`锛岃鐢ㄦ埛绌洪棿鑳藉鍦ㄦ瘡涓鎺ュ瓧鐨勫熀纭€涓?绠＄悊 TCP-AO銆備负浜嗘坊鍔?鍒犻櫎 MKT锛屽繀椤讳娇鐢?`TCP_AO_ADD_KEY` 鍜?`TCP_AO_DEL_KEY` TCP
濂楁帴瀛楅€夐」銆備笉鍏佽鍦ㄥ凡寤虹珛鐨勯潪 TCP-AO 杩炴帴涓婃坊鍔犲瘑閽ワ紝涔熶笉鍏佽浠?TCP-AO 杩炴帴涓婄Щ闄?鏈€鍚庝竴涓瘑閽ャ€?
**`setsockopt(TCP_AO_DEL_KEY)` 鍛戒护鍙互鎸囧畾 ``tcp_ao_del**
锛歝urrent_key``
- `tcp_ao_del::set_current` 鍜?鎴?`tcp_ao_del::rnext`
- `tcp_ao_del::set_rnext`锛屽畠浣挎绫诲垹闄ゆ垚涓?鈥滃己鍒垛€?鐨勶細瀹冧负鐢ㄦ埛绌洪棿鎻愪緵浜嗕竴绉嶆柟寮忔潵
鍒犻櫎姝ｅ湪浣跨敤鐨勫瘑閽ワ紝骞跺師瀛愬湴璁剧疆涓€涓浛浠ｅ瘑閽ャ€傝繖骞堕潪鐢ㄤ簬姝ｅ父浣跨敤锛屽彧搴斿湪瀵圭瓑浣撳拷鐣?RNextKeyID 骞舵寔缁姹?浣跨敤鏃у瘑閽ユ椂浣跨敤銆傚畠鎻愪緵浜嗕竴绉嶅己鍒跺垹闄や笉鍙椾俊浠诲瘑閽ョ殑鏂规硶锛屼絾
鍙兘浼氫腑鏂?TCP-AO 杩炴帴銆?
閫氬父/姝ｅ父鐨勫瘑閽ヨ疆鎹㈠彲浠ヤ娇鐢?`setsockopt(TCP_AO_INFO)` 鎵ц銆傚畠杩樻彁渚涗簡涓€涓?uAPI 鏉?鏇存敼姣忓鎺ュ瓧鐨?TCP-AO 璁剧疆锛屼緥濡傚拷鐣?ICMP锛屼互鍙婃竻闄ゆ瘡濂楁帴瀛楃殑 TCP-AO 鎶ユ枃璁℃暟鍣ㄣ€?鐩稿簲鐨?`getsockopt(TCP_AO_INFO)` 鍙敤浜庤幏鍙栬繖浜涙瘡濂楁帴瀛楃殑 TCP-AO 璁剧疆銆?
鍙︿竴涓湁鐢ㄧ殑鍛戒护鏄?`getsockopt(TCP_AO_GET_KEYS)`銆傚彲浠ヤ娇鐢ㄥ畠鍒楀嚭 TCP 濂楁帴瀛椾笂鐨勬墍鏈?MKT锛屾垨鑰呬娇鐢ㄨ繃婊ゅ櫒鏉ヨ幏鍙栫壒瀹氬绛変綋浠ュ強/鎴?sndid/rcvid銆乂RF L3 鎺ュ彛鎴?current_key/
rnext_key 鐨勫瘑閽ャ€?
涓轰簡淇 TCP-AO 杩炴帴锛宍setsockopt(TCP_AO_REPAIR)` 鍙敤锛屽墠鎻愭槸鐢ㄦ埛涔嬪墠宸茬粡浣跨敤
`getsockopt(TCP_AO_REPAIR)` 瀵瑰鎺ュ瓧杩涜浜嗘鏌ョ偣/杞偍銆?
瀵逛簬鍏锋湁鏁板崈涓?TCP-AO 瀵嗛挜鐨勩€佽妯″寲鐨?TCP_LISTEN 濂楁帴瀛楋紝涓€涓缓璁槸锛氬湪
`getsockopt(TCP_AO_GET_KEYS)` 涓娇鐢ㄨ繃婊ゅ櫒锛屽苟浣跨敤 `setsockopt(TCP_AO_DEL_KEY)` 杩涜
寮傛鍒犻櫎銆?
Linux TCP-AO 杩樻彁渚涗簡涓€缁勬姤鏂囨璁℃暟鍣紝鏈夊姪浜庢帓鏌?璋冭瘯闂銆傛瘡涓?MKT 閮芥湁 good/bad
璁℃暟鍣紝鍙嶆槧鏈夊灏戞姤鏂囬€氳繃浜?鏈€氳繃楠岃瘉銆傛瘡涓?TCP-AO 濂楁帴瀛楀叿鏈変互涓嬭鏁板櫒锛?- 閽堝姝ｅ父鎶ユ枃娈碉紙姝ｇ‘绛惧悕鐨勶級
- 閽堝閿欒鎶ユ枃娈碉紙TCP-AO 楠岃瘉澶辫触鐨勶級
- 閽堝浣跨敤鏈煡瀵嗛挜鐨勬姤鏂囨
- 閽堝鏈熸湜鏈?AO 绛惧悕浣嗘湭鎵惧埌鐨勬姤鏂囨
- 閽堝琚拷鐣ョ殑 ICMP 鏁伴噺

TCP-AO 姣忓鎺ュ瓧璁℃暟鍣ㄤ篃涓庢瘡缃戠粶鍛藉悕绌洪棿锛坣etns锛夎鏁板櫒涓€璧烽噸澶嶏紝閫氳繃 SNMP 鏆撮湶銆傚畠浠?鏄?`TCPAOGood`銆乣TCPAOBad`銆乣TCPAOKeyNotFound`銆乣TCPAORequired` 鍜?`TCPAODroppedIcmps`銆?
鍑轰簬鐩戞帶鐩殑锛屾湁浠ヤ笅 TCP-AO 璺熻釜浜嬩欢锛歚tcp_hash_bad_header`銆乣tcp_hash_ao_required`銆?`tcp_ao_handshake_failure`銆乣tcp_ao_wrong_maclen`銆乣tcp_ao_wrong_maclen`銆?`tcp_ao_key_not_found`銆乣tcp_ao_rnext_request`銆乣tcp_ao_synack_no_key`銆?`tcp_ao_snd_sne_update`銆乣tcp_ao_rcv_sne_update`銆傚彲浠ュ崟鐙惎鐢ㄥ畠浠腑鐨勪换鎰忎竴涓紝骞?鍙互鎸夌綉缁滃懡鍚嶇┖闂淬€? 鍏冪粍銆佹棌銆丩3 绱㈠紩鍜?TCP 澶撮儴鏍囧織杩涜杩囨护銆傚鏋滄姤鏂囨甯︽湁
TCP-AO 澶撮儴锛岃繃婊ゅ櫒杩樺彲浠ュ寘鍚?keyid銆乺next 鍜?maclen銆係NE 鏇存柊鍖呭惈缈昏浆鐨勬暟瀛椼€?
RFC 5925 闈炲父瀹芥澗鍦拌瀹氫簡濡備綍瀵?TCP 绔彛杩涜鍖归厤
```

   TCP connection identifier. A TCP socket pair, i.e., a local IP
   address, a remote IP address, a TCP local port, and a TCP remote port.
   Values can be partially specified using ranges (e.g., 2-30), masks
   (e.g., 0xF0), wildcards (e.g., "*"), or any other suitable indication.

```
鐩墠 Linux TCP-AO 瀹炵幇涓嶆彁渚涗换浣?TCP 绔彛鍖归厤銆備篃璁哥鍙ｈ寖鍥村浜?uAPI 鏉ヨ鏈€鐏垫椿锛屼絾
鍒扮洰鍓嶄负姝㈠皻鏈疄鐜般€?
## 4. ``setsockopt()`` 涓?``accept()`` 鐨勭珵浜?

涓庡彧鏈変竴涓瘑閽ョ殑宸插缓绔?TCP-MD5 杩炴帴涓嶅悓锛孴CP-AO 杩炴帴鍙兘鏈夊緢澶氬瘑閽ワ紝杩欐剰鍛崇潃鐩戝惉
濂楁帴瀛椾笂琚帴鍙楃殑杩炴帴涔熷彲鑳芥湁浠绘剰鏁伴噺鐨勫瘑閽ャ€傜敱浜庡湪涓€涓涓纭鍚嶇殑 SYN 涓婂鍒舵墍鏈?杩欎簺瀵嗛挜浼氫娇璇锋眰濂楁帴瀛楀彉澶э紝杩欐槸涓嶆湡鏈涚殑銆傜洰鍓嶏紝瀹炵幇涓嶄細灏嗗瘑閽ュ鍒跺埌璇锋眰濂楁帴瀛楋紝鑰屾槸
鍦?鈥滅埗鈥?鐩戝惉濂楁帴瀛椾笂鏌ユ壘瀹冧滑銆?
鍏剁粨鏋滄槸锛屽綋鐢ㄦ埛绌洪棿绉婚櫎 TCP-AO 瀵嗛挜鏃讹紝鍙兘浼氱牬鍧忚姹傚鎺ュ瓧涓婂皻鏈缓绔嬬殑杩炴帴锛屼互鍙?涓嶄細浠庡凡缁忓缓绔嬩絾灏氭湭琚?`accept()` 鐨勮繛鎺ワ紙鎮寕鍦?accept 闃熷垪涓級涓婄Щ闄ゅ瘑閽ャ€?
鍙嶄箣浜︾劧锛氬鏋滅敤鎴风┖闂村湪鐩戝惉濂楁帴瀛椾笂涓烘煇涓绛変綋娣诲姞浜嗕竴涓柊瀵嗛挜锛岄偅涔?accept 闃熷垪涓?宸插缓绔嬬殑濂楁帴瀛楀皢涓嶄細鏈夎繖浜涙柊瀵嗛挜銆?
鐩墠锛岃繖涓ょ绔炰簤鐨勮В鍐虫柟妗堬細
`setsockopt(TCP_AO_ADD_KEY)` 涓?`accept()` 涔嬮棿鐨勭珵浜夛紝
浠ュ強 `setsockopt(TCP_AO_DEL_KEY)` 涓?`accept()` 涔嬮棿鐨勭珵浜夛紝琚鎵樼粰鐢ㄦ埛绌洪棿銆傝繖鎰忓懗鐫€
鏈熸湜鐢ㄦ埛绌洪棿妫€鏌ョ敱 `accept()` 杩斿洖鐨勫鎺ュ瓧涓婄殑 MKT锛屼互楠岃瘉鐩戝惉濂楁帴瀛椾笂鍙戠敓鐨勪换浣曞瘑閽?杞崲鏄惁鍙嶆槧鍦ㄦ柊寤虹珛鐨勮繛鎺ヤ笂銆?
杩欎笌鍐呮牳渚у TCP-MD5 鐨?鈥渄o-nothing鈥濓紙涓嶅仛浠讳綍浜嬶級鏂规硶绫讳技锛屼互鍚庡彲鑳戒細閫氳繃涓?`tcp_ao_add` 鍜?`tcp_ao_del` 寮曞叆鏂版爣蹇楁潵鏀瑰彉銆?
璇锋敞鎰忥紝杩欑绔炰簤寰堝皯瑙侊紝鍥犱负瀹冮渶瑕佹柊鐨?TCP 杩炴帴鐨?3 娆℃彙鎵嬫湡闂村彂鐢?TCP-AO 瀵嗛挜杞崲銆?
## 5. 涓?TCP-MD5 鐨勪氦浜?

TCP 杩炴帴涓嶈兘鍦?TCP-AO 鍜?TCP-MD5 閫夐」涔嬮棿杩佺Щ銆傚凡缁忓缓绔嬩簡甯︽湁 AO 鎴?MD5 瀵嗛挜鐨勫鎺ュ瓧
琚檺鍒朵负涓嶈兘娣诲姞鍙︿竴绉嶉€夐」鐨勫瘑閽ャ€?
瀵逛簬鐩戝惉濂楁帴瀛楋紝鎯呭喌鍒欎笉鍚岋細BGP 鏈嶅姟鍣ㄥ彲鑳藉笇鏈涘悓鏃舵帴鏀?TCP-AO 鍜岋紙宸插純鐢ㄧ殑锛塗CP-MD5
瀹㈡埛绔€傚洜姝わ紝涓ょ绫诲瀷鐨勫瘑閽ラ兘鍙互娣诲姞鍒?TCP_CLOSED 鎴?TCP_LISTEN 濂楁帴瀛椾笂銆備笉鍏佽
涓哄悓涓€涓绛変綋娣诲姞涓嶅悓绫诲瀷鐨勫瘑閽ャ€?
## 6. SNE 鐨?Linux 瀹炵幇


RFC 5925 [6.2] 鎻忚堪浜嗗浣曠敤 SNE 鎵╁睍 TCP 搴忓垪鍙风殑绠楁硶銆傜畝鑰岃█涔嬶細TCP 蹇呴』璺熻釜鍏堝墠鐨?搴忓垪鍙凤紝骞跺湪褰撳墠 SEQ 鍙风炕杞椂璁剧疆 sne_flag銆傚綋褰撳墠鍜屽厛鍓嶇殑 SEQ 鍙烽兘瓒婅繃 0x7fff锛堝嵆
32Kb锛夋椂锛岃鏍囧織琚竻闄ゃ€?
鍦?sne_flag 琚疆浣嶇殑鏈熼棿锛岀畻娉曞皢姣忎釜鎶ユ枃鐨?SEQ 涓?0x7fff 姣旇緝锛屽鏋滃畠楂樹簬 32Kb锛屽垯
鍋囧畾璇ユ姤鏂囧簲璇ョ敤閫掑涔嬪墠鐨?SNE 鏉ラ獙璇併€傜粨鏋滐紝瀛樺湪杩欎釜 [0; 32Kb] 鐨勭獥鍙ｏ紝鍦ㄦ鏈熼棿鍙互
鎺ュ彈甯︽湁锛圫NE - 1锛夌殑鎶ユ枃銆?
Linux 瀹炵幇瀵规鍋氫簡涓€浜涚畝鍖栵細鐢变簬缃戠粶鏍堝凡缁忚窡韪簡鏈熸湜 ACK 鐨勭涓€涓?SEQ 瀛楄妭锛坰nd_una锛?鍜屾湡鏈涚殑涓嬩竴涓?SEQ 瀛楄妭锛坮cv_nxt锛夆€斺€旇繖瓒充互绮楃暐浼拌鍙戦€佹柟鍜屾帴鏀舵柟鍦?4GB SEQ 鍙风┖闂翠腑
鐨勪綅缃€傚綋瀹冧滑缈昏浆鍒伴浂鏃讹紝鐩稿簲鐨?SNE 浼氶€掑銆?
tcp_ao_compute_sne() 瀵规瘡涓?TCP-AO 鎶ユ枃娈佃皟鐢ㄣ€傚畠灏嗘姤鏂囦腑鐨?SEQ 鍙蜂笌 snd_una 鎴?rcv_nxt 姣旇緝锛屽苟灏嗙粨鏋滈€傞厤鍒板畠浠懆鍥?2GB 鐨勭獥鍙ｄ腑锛屼粠鑰屾娴?SEQ 鍙风殑缈昏浆銆傝繖澶уぇ绠€鍖?浜嗕唬鐮侊紝骞朵笖鍙渶瑕佸湪姣忎釜 TCP-AO 濂楁帴瀛椾笂瀛樺偍 SNE 鍙枫€?
2GB 绐楀彛涔嶇湅涔嬩笅浼间箮姣?RFC 5926 瀹芥澗寰楀銆備絾瀹冨彧鐢ㄤ簬鍦ㄧ炕杞箣鍓?涔嬪悗閫夋嫨姝ｇ‘鐨?SNE銆傚畠
鍏佽鏇村鐨?TCP 鎶ユ枃娈甸噸鏀撅紝浣嗗湪宸查獙璇佺殑鎶ユ枃娈典笂浠嶇劧浼氬簲鐢?tcp_sequence() 涓殑鎵€鏈夊父瑙?TCP 妫€鏌ャ€傚洜姝わ紝瀹冪敤瀵归噸鏀?閲嶄紶鎶ユ枃娈电殑绋嶅鏉炬帴鍙楋紝鎹㈠彇浜嗙畻娉曠殑绠€鍗曟€т互鍙婂澶?TCP
绐楀彛浼间箮鏇村ソ鐨勮涓恒€?
## 7. 閾炬帴


RFC 5925 The TCP Authentication Option
   https://www.rfc-editor.org/rfc/pdfrfc/rfc5925.txt.pdf

RFC 5926 Cryptographic Algorithms for the TCP Authentication Option (TCP-AO)
   https://www.rfc-editor.org/rfc/pdfrfc/rfc5926.txt.pdf

鑽夋 鈥淪HA-2 Algorithm for the TCP Authentication Option (TCP-AO)鈥?   https://datatracker.ietf.org/doc/html/draft-nayak-tcp-sha2-03

RFC 2385 Protection of BGP Sessions via the TCP MD5 Signature Option
   https://www.rfc-editor.org/rfc/pdfrfc/rfc2385.txt.pdf

:Author: Dmitry Safonov <dima@arista.com>
