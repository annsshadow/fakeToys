## NAND 绾犻敊鐮侊紙Error-correction Code锛?
## 绠€浠?
鍦ㄧ爺绌惰繃 Linux 鐨?mtd/nand Hamming 杞欢 ECC 寮曟搸椹卞姩涔嬪悗锛屾垜瑙夊緱杩樻湁浼樺寲鐨勭┖闂淬€傛垜瀵逛唬鐮佹姌鑵句簡鍑犱釜灏忔椂锛屽仛浜嗚濡傛煡琛ㄣ€佺Щ闄ゅ浣欎唬鐮佷箣绫荤殑鎶€宸с€備箣鍚庨€熷害鎻愬崌浜?35%鈥?0%銆傚敖绠″姝ゆ垜浠嶄笉澶弧鎰忥紝鍥犱负鎴戞劅瑙夎繕鏈夎繘涓€姝ョ殑鏀硅繘浣欏湴銆?
绯熺硶锛佹垜涓婄樉浜嗐€傛垜鍐冲畾鍦ㄨ繖涓枃浠堕噷璁板綍涓嬫垜鐨勬瘡涓€姝ャ€傛垨璁稿畠瀵规煇涓汉鏈夌敤锛屾垨鑰呮湁浜鸿兘浠庝腑鏈夋墍鏀惰幏銆?

## 闂

NAND 闂瓨锛堣嚦灏戞槸 SLC 閭ｇ锛夐€氬父鍏锋湁 256 瀛楄妭澶у皬鐨勬墖鍖恒€傜劧鑰?NAND 闂瓨骞朵笉鏄瀬鍏跺彲闈狅紝鍥犳闇€瑕佷竴浜涢敊璇娴嬶紙鏈夋椂杩橀渶瑕佺籂閿欙級銆?
杩欓€氳繃 Hamming 鐮佹潵瀹屾垚銆傛垜浼氬皾璇曠敤澶栬鐨勮瘽鏉ヨВ閲婂畠锛堝鏋滄垜娌℃湁浣跨敤姝ｇ‘鐨勬湳璇紝璇峰悜璇ラ鍩熸墍鏈夌殑涓撲笟浜哄＋鑷存瓑锛屾垜鐨勭紪鐮佺悊璁鸿鍑犱箮鏄湪 30 骞村墠涓婄殑锛岃€屼笖鎴戝繀椤绘壙璁ら偅骞朵笉鏄垜鏈€鍠滄鐨勮锛夈€?
姝ｅ鎴戜箣鍓嶆墍璇达紝ecc 璁＄畻鏄湪 256 瀛楄妭鐨勬墖鍖轰笂杩涜鐨勩€傝繖鏄€氳繃璁＄畻琛屽拰鍒椾笂鐨勮嫢骞插鍋舵牎楠屼綅鏉ュ疄鐜扮殑銆傛墍鐢ㄧ殑鏄伓鏍￠獙锛坋ven parity锛夛紝鍗筹細濡傛灉琚绠楀鍋舵牎楠岀殑鏁版嵁涓?1锛屽垯濂囧伓鏍￠獙浣?= 1锛涘鏋滆璁＄畻濂囧伓鏍￠獙鐨勬暟鎹负 0锛屽垯濂囧伓鏍￠獙浣?= 0銆傚洜姝わ紝琚绠楀鍋舵牎楠岀殑鏁版嵁鐨勪綅鎬绘暟鍔犱笂濂囧伓鏍￠獙浣嶄负鍋舵暟銆傦紙濡傛灉璺熶笉涓婏紝璇风湅 wikipedia銆傦級濂囧伓鏍￠獙閫氬父閫氳繃寮傛垨锛坋xclusive or锛夎繍绠楁潵璁＄畻锛屾湁鏃朵篃绉颁负 xor銆傚湪 C 璇█涓?xor 鐨勮繍绠楃鏄?^

鍥炲埌 ecc銆傝鎴戜滑缁欏嚭涓€寮犲皬鍥撅細

=========  ==== ==== ==== ==== ==== ==== ==== ====   === === === === ====
byte   0:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp2 rp4 ... rp14
byte   1:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp1 rp2 rp4 ... rp14
byte   2:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp3 rp4 ... rp14
byte   3:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp1 rp3 rp4 ... rp14
byte   4:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp2 rp5 ... rp14
...
byte 254:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp0 rp3 rp5 ... rp15
byte 255:  bit7 bit6 bit5 bit4 bit3 bit2 bit1 bit0   rp1 rp3 rp5 ... rp15
           cp1  cp0  cp1  cp0  cp1  cp0  cp1  cp0
           cp3  cp3  cp2  cp2  cp3  cp3  cp2  cp2
           cp5  cp5  cp5  cp5  cp4  cp4  cp4  cp4
=========  ==== ==== ==== ==== ==== ==== ==== ====   === === === === ====

杩欏紶鍥捐〃绀轰竴涓?256 瀛楄妭鐨勬墖鍖恒€俢p 鏄?column parity锛堝垪濂囧伓鏍￠獙锛夌殑缂╁啓锛宺p 鏄?row parity锛堣濂囧伓鏍￠獙锛夌殑缂╁啓銆?
璁╂垜浠紑濮嬭В閲婂垪濂囧伓鏍￠獙銆?
- cp0 鏄睘浜庢墍鏈?bit0銆乥it2銆乥it4銆乥it6 鐨勫鍋舵牎楠屻€?
  鍥犳鎵€鏈?bit0銆乥it2銆乥it4 鍜?bit6 鐨勫€间箣鍜屽姞涓?cp0 鏈韩涓哄伓鏁般€?
绫讳技鍦?cp1 鏄墍鏈?bit1銆乥it3銆乥it5 鍜?bit7 涔嬪拰銆?
- cp2 鏄?bit0銆乥it1銆乥it4 鍜?bit5 涓婄殑濂囧伓鏍￠獙
- cp3 鏄?bit2銆乥it3銆乥it6 鍜?bit7 涓婄殑濂囧伓鏍￠獙銆?- cp4 鏄?bit0銆乥it1銆乥it2 鍜?bit3 涓婄殑濂囧伓鏍￠獙銆?- cp5 鏄?bit4銆乥it5銆乥it6 鍜?bit7 涓婄殑濂囧伓鏍￠獙銆?
娉ㄦ剰 cp0 .. cp5 姣忎竴涓兘鎭板ソ鏄竴浣嶃€?
琛屽鍋舵牎楠岀殑宸ヤ綔鏂瑰紡鍑犱箮鐩稿悓銆?
- rp0 鏄墍鏈夊伓鏁板瓧鑺傦紙0, 2, 4, 6, ... 252, 254锛夌殑濂囧伓鏍￠獙
- rp1 鏄墍鏈夊鏁板瓧鑺傦紙1, 3, 5, 7, ..., 253, 255锛夌殑濂囧伓鏍￠獙
- rp2 鏄瓧鑺?0, 1, 4, 5, 8, 9, ... 鐨勫鍋舵牎楠?  锛堝嵆澶勭悊涓や釜瀛楄妭锛岀劧鍚庤烦杩?2 涓瓧鑺傦級銆?- rp3 瑕嗙洊 rp2 鏈鐩栫殑閭ｄ竴鍗婏紙瀛楄妭 2, 3, 6, 7, 10, 11, ...锛?- 瀵逛簬 rp4锛岃鍒欐槸瑕嗙洊 4 涓瓧鑺傦紝璺宠繃 4 涓瓧鑺傦紝瑕嗙洊 4 涓瓧鑺傦紝璺宠繃 4 涓紝渚濇绫绘帹銆?
  鎵€浠?rp4 璁＄畻瀛楄妭 0, 1, 2, 3, 8, 9, 10, 11, 16, ... 涓婄殑濂囧伓鏍￠獙
- 鑰?rp5 瑕嗙洊鍙︿竴鍗婏紝鍗冲瓧鑺?4, 5, 6, 7, 12, 13, 14, 15, 20, ..

鎺ヤ笅鏉ョ殑鍙欒堪灏卞彉寰楃浉褰撲箯鍛充簡銆傛垜鎯充綘宸茬粡鏄庣櫧鎰忔€濅簡銆?
- rp6 瑕嗙洊 8 涓瓧鑺傜劧鍚庤烦杩?8 涓紝渚濇绫绘帹
- rp7 璺宠繃 8 涓瓧鑺傜劧鍚庤鐩?8 涓紝渚濇绫绘帹
- rp8 瑕嗙洊 16 涓瓧鑺傜劧鍚庤烦杩?16 涓紝渚濇绫绘帹
- rp9 璺宠繃 16 涓瓧鑺傜劧鍚庤鐩?16 涓紝渚濇绫绘帹
- rp10 瑕嗙洊 32 涓瓧鑺傜劧鍚庤烦杩?32 涓紝渚濇绫绘帹
- rp11 璺宠繃 32 涓瓧鑺傜劧鍚庤鐩?32 涓紝渚濇绫绘帹
- rp12 瑕嗙洊 64 涓瓧鑺傜劧鍚庤烦杩?64 涓紝渚濇绫绘帹
- rp13 璺宠繃 64 涓瓧鑺傜劧鍚庤鐩?64 涓紝渚濇绫绘帹
- rp14 瑕嗙洊 128 涓瓧鑺傜劧鍚庤烦杩?128
- rp15 璺宠繃 128 涓瓧鑺傜劧鍚庤鐩?128

鏈€鍚庯紝濂囧伓鏍￠獙浣嶈鍒嗙粍鍒颁笁涓瓧鑺備腑锛屽涓嬫墍绀猴細

=====  ===== ===== ===== ===== ===== ===== ===== =====
ECC    Bit 7 Bit 6 Bit 5 Bit 4 Bit 3 Bit 2 Bit 1 Bit 0
=====  ===== ===== ===== ===== ===== ===== ===== =====
ECC 0   rp07  rp06  rp05  rp04  rp03  rp02  rp01  rp00
ECC 1   rp15  rp14  rp13  rp12  rp11  rp10  rp09  rp08
ECC 2   cp5   cp4   cp3   cp2   cp1   cp0      1     1
=====  ===== ===== ===== ===== ===== ===== ===== =====

鎴戝湪鍐欏畬杩欎簺涔嬪悗鍙戠幇锛孲T 搴旂敤绗旇 AN1823锛坔ttp://www.st.com/stonline/锛夌粰鍑轰簡涓€骞呮洿婕備寒鐨勫浘銆傦紙涓嶈繃浠栦滑鐢?line parity 杩欎釜鏈锛岃€屾垜鐢ㄧ殑鏄?row parity銆傦級鍝︾畻浜嗭紝鎴戠敾鍥句笉琛岋紝鎵€浠ヨ闄垜蹇嶅彈涓€涓?:-)

鑰屼笖鐢变簬鐗堟潈鍘熷洜锛屾垜涔熸棤娉曞鐢?ST 鐨勫浘銆?

## 灏濊瘯 0

瀹炵幇濂囧伓鏍￠獙鐨勮绠楃浉褰撶畝鍗曘€?```

  for (i = 0; i < 256; i++)
  {
    if (i & 0x01)
       rp1 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp1;
    else
       rp0 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp0;
    if (i & 0x02)
       rp3 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp3;
    else
       rp2 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp2;
    if (i & 0x04)
      rp5 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp5;
    else
      rp4 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp4;
    if (i & 0x08)
      rp7 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp7;
    else
      rp6 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp6;
    if (i & 0x10)
      rp9 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp9;
    else
      rp8 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp8;
    if (i & 0x20)
      rp11 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp11;
    else
      rp10 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp10;
    if (i & 0x40)
      rp13 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp13;
    else
      rp12 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp12;
    if (i & 0x80)
      rp15 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp15;
    else
      rp14 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ bit3 ^ bit2 ^ bit1 ^ bit0 ^ rp14;
    cp0 = bit6 ^ bit4 ^ bit2 ^ bit0 ^ cp0;
    cp1 = bit7 ^ bit5 ^ bit3 ^ bit1 ^ cp1;
    cp2 = bit5 ^ bit4 ^ bit1 ^ bit0 ^ cp2;
    cp3 = bit7 ^ bit6 ^ bit3 ^ bit2 ^ cp3
    cp4 = bit3 ^ bit2 ^ bit1 ^ bit0 ^ cp4
    cp5 = bit7 ^ bit6 ^ bit5 ^ bit4 ^ cp5
  }


```
## 鍒嗘瀽 0

C 璇█纭疄鏈変綅杩愮畻绗︼紝浣嗗苟娌℃湁鑳介珮鏁堝畬鎴愪笂杩拌繍绠楃殑杩愮畻绗︼紙鑰屼笖澶у鏁扮‖浠朵篃娌℃湁杩欑被鎸囦护锛夈€傚洜姝ゆ棤闇€瀹炵幇灏辫兘娓呮锛屼笂闈㈢殑浠ｇ爜涓嶄細缁欐垜甯︽潵璇鸿礉灏斿 :-)

骞歌繍鐨勬槸锛屽紓鎴栬繍绠楁槸鍙氦鎹㈢殑锛屾墍浠ユ垜浠彲浠ヤ互浠绘剰椤哄簭缁勫悎杩欎簺鍊笺€傚洜姝わ紝涓庡叾閫愪釜璁＄畻鎵€鏈変綅锛屼笉濡傚皾璇曢噸鏂版帓鍒椾竴涓嬨€傚浜庡垪濂囧伓鏍￠獙杩欏緢瀹规槗銆傛垜浠彲浠ョ畝鍗曞湴瀵瑰瓧鑺傚仛 xor锛屾渶鍚庡啀杩囨护鍑虹浉鍏崇殑浣嶃€傝繖闈炲父濂斤紝鍥犱负瀹冧細鎶婃墍鏈?cp 璁＄畻绉诲嚭 for 寰幆銆?
绫讳技鍦帮紝鎴戜滑鍙互鍏堜负鍚勪釜琛?xor 瀛楄妭銆傝繖鍙互寮曞嚭锛?

## 灏濊瘯 1

```
  const char parity[256] = {
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1,
      0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0
  };

  void ecc1(const unsigned char *buf, unsigned char *code)
  {
      int i;
      const unsigned char *bp = buf;
      unsigned char cur;
      unsigned char rp0, rp1, rp2, rp3, rp4, rp5, rp6, rp7;
      unsigned char rp8, rp9, rp10, rp11, rp12, rp13, rp14, rp15;
      unsigned char par;

      par = 0;
      rp0 = 0; rp1 = 0; rp2 = 0; rp3 = 0;
      rp4 = 0; rp5 = 0; rp6 = 0; rp7 = 0;
      rp8 = 0; rp9 = 0; rp10 = 0; rp11 = 0;
      rp12 = 0; rp13 = 0; rp14 = 0; rp15 = 0;

      for (i = 0; i < 256; i++)
      {
          cur = *bp++;
          par ^= cur;
          if (i & 0x01) rp1 ^= cur; else rp0 ^= cur;
          if (i & 0x02) rp3 ^= cur; else rp2 ^= cur;
          if (i & 0x04) rp5 ^= cur; else rp4 ^= cur;
          if (i & 0x08) rp7 ^= cur; else rp6 ^= cur;
          if (i & 0x10) rp9 ^= cur; else rp8 ^= cur;
          if (i & 0x20) rp11 ^= cur; else rp10 ^= cur;
          if (i & 0x40) rp13 ^= cur; else rp12 ^= cur;
          if (i & 0x80) rp15 ^= cur; else rp14 ^= cur;
      }
      code[0] =
          (parity[rp7] << 7) |
          (parity[rp6] << 6) |
          (parity[rp5] << 5) |
          (parity[rp4] << 4) |
          (parity[rp3] << 3) |
          (parity[rp2] << 2) |
          (parity[rp1] << 1) |
          (parity[rp0]);
      code[1] =
          (parity[rp15] << 7) |
          (parity[rp14] << 6) |
          (parity[rp13] << 5) |
          (parity[rp12] << 4) |
          (parity[rp11] << 3) |
          (parity[rp10] << 2) |
          (parity[rp9]  << 1) |
          (parity[rp8]);
      code[2] =
          (parity[par & 0xf0] << 7) |
          (parity[par & 0x0f] << 6) |
          (parity[par & 0xcc] << 5) |
          (parity[par & 0x33] << 4) |
          (parity[par & 0xaa] << 3) |
          (parity[par & 0x55] << 2);
      code[0] = ~code[0];
      code[1] = ~code[1];
      code[2] = ~code[2];
  }

```
浠嶇劧鐩稿綋鐩磋銆傛渶鍚庝笁鏉?invert锛堝彇鍙嶏級璇彞鏄负浜嗚绌洪棯瀛樺緱鍒?0xff 0xff 0xff 鐨勬牎楠屽拰銆傚湪绌洪棯瀛樹腑鎵€鏈夋暟鎹兘鏄?0xff锛屽洜姝ゆ牎楠屽拰闅忎箣鍖归厤銆?
鎴戣繕寮曞叆浜?parity 鏌ヨ〃銆傛垜鍘熸湰鏈熸湜杩欐槸璁＄畻濂囧伓鏍￠獙鏈€蹇殑鏂瑰紡锛屼絾鎴戠◢鍚庝細鐮旂┒鏇夸唬鏂规銆?

## 鍒嗘瀽 1

浠ｇ爜鑳界敤锛屼絾鏁堢巼骞朵笉楂樺緱鎯婁汉銆傚湪鎴戠殑绯荤粺涓婂畠鑰楄垂鐨勬椂闂村嚑涔庢槸 Linux 椹卞姩浠ｇ爜鐨?4 鍊嶃€備笉杩囷紝濡傛灉鐪熼偅涔堝鏄擄紝杩欐棭灏辫浜哄仛杩囦簡銆?娌℃湁浠樺嚭锛屽氨娌℃湁鏀惰幏銆?
骞歌繍鐨勬槸浠嶆湁澶ч噺鏀硅繘绌洪棿銆?
鍦ㄧ 1 姝ヤ腑鎴戜滑浠庨€愪綅璁＄畻杞Щ鍒颁簡閫愬瓧鑺傝绠椼€傜劧鑰屽湪 C 涓垜浠篃鍙互浣跨敤 unsigned long 鏁版嵁绫诲瀷锛岃€屼笖鍑犱箮姣忎釜鐜颁唬寰鐞嗗櫒閮芥敮鎸?32 浣嶆搷浣滐紝閭ｄ箞涓轰綍涓嶅皾璇曟妸浠ｇ爜鍐欐垚浠?32 浣嶅潡鏉ュ鐞嗘暟鎹殑鏂瑰紡鍛€?
褰撶劧锛岃繖鎰忓懗鐫€涓€浜涗慨鏀癸紝鍥犱负琛屽鍋舵牎楠屾槸閫愬瓧鑺傜殑銆備竴涓揩閫熷垎鏋愶細
瀵逛簬鍒楀鍋舵牎楠屾垜浠娇鐢?par 鍙橀噺銆傚綋鎵╁睍鍒?32 浣嶆椂锛屾垜浠渶缁堝彲浠ヨ交鏉惧湴浠庡畠璁＄畻鍑?rp0 鍜?rp1銆?锛堝洜涓?par 鐜板湪鐢?4 涓瓧鑺傜粍鎴愶紝鍒嗗埆浠?MSB 鍒?LSB 璐＄尞缁?rp1銆乺p0銆乺p1銆乺p0锛?鍚屾牱 rp2 鍜?rp3 涔熷彲浠ヨ交鏉惧湴浠?par 鍙栧緱锛屽洜涓?rp3 瑕嗙洊鍓嶄袱涓?MSB锛岃€?rp2 瑕嗙洊鍚庝袱涓?LSB銆?
娉ㄦ剰鐜板湪寰幆鍙墽琛?64 娆★紙256/4锛夈€傚苟涓旀敞鎰忓繀椤诲皬蹇冨瓧鑺傚簭锛坆yte ordering锛夈€傚瓧鑺傚湪 long 涓殑鎺掑垪椤哄簭鏄満鍣ㄧ浉鍏崇殑锛屽彲鑳戒細褰卞搷鍒版垜浠€傛棤璁哄浣曪紝濡傛灉鏈夐棶棰橈細杩欐浠ｇ爜鏄湪 x86 涓婂紑鍙戠殑锛堢‘鍒囧湴璇达細鏄竴鍙板甫鏈?D920 Intel CPU 鐨?DELL PC锛?
褰撶劧锛屾€ц兘鍙兘涔熷彇鍐充簬瀵归綈锛屼絾鎴戦鏈?nand 椹卞姩涓殑 I/O 缂撳啿鍖烘槸瀵归綈鑹ソ鐨勶紙鍚﹀垯搴斿綋淇瀹冧互鑾峰緱鏈€澶ф€ц兘锛夈€?
璁╂垜浠瘯涓€璇曗€︹€?

## 灏濊瘯 2

```
  extern const char parity[256];

  void ecc2(const unsigned char *buf, unsigned char *code)
  {
      int i;
      const unsigned long *bp = (unsigned long *)buf;
      unsigned long cur;
      unsigned long rp0, rp1, rp2, rp3, rp4, rp5, rp6, rp7;
      unsigned long rp8, rp9, rp10, rp11, rp12, rp13, rp14, rp15;
      unsigned long par;

      par = 0;
      rp0 = 0; rp1 = 0; rp2 = 0; rp3 = 0;
      rp4 = 0; rp5 = 0; rp6 = 0; rp7 = 0;
      rp8 = 0; rp9 = 0; rp10 = 0; rp11 = 0;
      rp12 = 0; rp13 = 0; rp14 = 0; rp15 = 0;

      for (i = 0; i < 64; i++)
      {
          cur = *bp++;
          par ^= cur;
          if (i & 0x01) rp5 ^= cur; else rp4 ^= cur;
          if (i & 0x02) rp7 ^= cur; else rp6 ^= cur;
          if (i & 0x04) rp9 ^= cur; else rp8 ^= cur;
          if (i & 0x08) rp11 ^= cur; else rp10 ^= cur;
          if (i & 0x10) rp13 ^= cur; else rp12 ^= cur;
          if (i & 0x20) rp15 ^= cur; else rp14 ^= cur;
      }
      /*
         we need to adapt the code generation for the fact that rp vars are now
         long; also the column parity calculation needs to be changed.
         we'll bring rp4 to 15 back to single byte entities by shifting and
         xoring
      */
      rp4 ^= (rp4 >> 16); rp4 ^= (rp4 >> 8); rp4 &= 0xff;
      rp5 ^= (rp5 >> 16); rp5 ^= (rp5 >> 8); rp5 &= 0xff;
      rp6 ^= (rp6 >> 16); rp6 ^= (rp6 >> 8); rp6 &= 0xff;
      rp7 ^= (rp7 >> 16); rp7 ^= (rp7 >> 8); rp7 &= 0xff;
      rp8 ^= (rp8 >> 16); rp8 ^= (rp8 >> 8); rp8 &= 0xff;
      rp9 ^= (rp9 >> 16); rp9 ^= (rp9 >> 8); rp9 &= 0xff;
      rp10 ^= (rp10 >> 16); rp10 ^= (rp10 >> 8); rp10 &= 0xff;
      rp11 ^= (rp11 >> 16); rp11 ^= (rp11 >> 8); rp11 &= 0xff;
      rp12 ^= (rp12 >> 16); rp12 ^= (rp12 >> 8); rp12 &= 0xff;
      rp13 ^= (rp13 >> 16); rp13 ^= (rp13 >> 8); rp13 &= 0xff;
      rp14 ^= (rp14 >> 16); rp14 ^= (rp14 >> 8); rp14 &= 0xff;
      rp15 ^= (rp15 >> 16); rp15 ^= (rp15 >> 8); rp15 &= 0xff;
      rp3 = (par >> 16); rp3 ^= (rp3 >> 8); rp3 &= 0xff;
      rp2 = par & 0xffff; rp2 ^= (rp2 >> 8); rp2 &= 0xff;
      par ^= (par >> 16);
      rp1 = (par >> 8); rp1 &= 0xff;
      rp0 = (par & 0xff);
      par ^= (par >> 8); par &= 0xff;

      code[0] =
          (parity[rp7] << 7) |
          (parity[rp6] << 6) |
          (parity[rp5] << 5) |
          (parity[rp4] << 4) |
          (parity[rp3] << 3) |
          (parity[rp2] << 2) |
          (parity[rp1] << 1) |
          (parity[rp0]);
      code[1] =
          (parity[rp15] << 7) |
          (parity[rp14] << 6) |
          (parity[rp13] << 5) |
          (parity[rp12] << 4) |
          (parity[rp11] << 3) |
          (parity[rp10] << 2) |
          (parity[rp9]  << 1) |
          (parity[rp8]);
      code[2] =
          (parity[par & 0xf0] << 7) |
          (parity[par & 0x0f] << 6) |
          (parity[par & 0xcc] << 5) |
          (parity[par & 0x33] << 4) |
          (parity[par & 0xaa] << 3) |
          (parity[par & 0x55] << 2);
      code[0] = ~code[0];
      code[1] = ~code[1];
      code[2] = ~code[2];
  }

```
parity 鏁扮粍涓嶅啀灞曠ず浜嗐€傝繕瑕佹敞鎰忥紝瀵逛簬杩欎簺绀轰緥锛屾垜鏈夋剰鍋忕浜嗘垜骞冲父鐨勭紪绋嬮鏍硷紝鍏佽涓€琛屽鏉¤鍙ャ€佸湪鍙湁鍗曟潯璇彞鐨?then 鍜?else 鍧椾腑涓嶄娇鐢?{ }锛屽苟浣跨敤浜?^= 杩欑被杩愮畻绗︺€?

## 鍒嗘瀽 2

浠ｇ爜锛堝綋鐒讹級鑳界敤锛屽苟涓斿ソ鑰讹細鎴戜滑姣?Linux 椹卞姩浠ｇ爜蹇簡涓€鐐圭偣锛堢害 15%锛夈€備笉杩囩瓑绛夛紝鍒珮鍏村緱澶棭銆傝繕鏈夋洿澶氬彲鎻愬崌鐨勭┖闂淬€?濡傛灉鎴戜滑鐪嬩緥濡?rp14 鍜?rp15锛屼細鍙戠幇鎴戜滑瑕佷箞鐢?rp14 寮傛垨鏁版嵁锛岃涔堢敤 rp15 寮傛垨鏁版嵁銆傜劧鑰屾垜浠繕鏈夐亶鍘嗘墍鏈夋暟鎹殑 par銆傝繖鎰忓懗鐫€鏃犻渶璁＄畻 rp14锛屽洜涓哄畠鍙互閫氳繃 rp14 = par ^ rp15 浠?rp15 绠楀嚭鏉ワ紝鍥犱负 par = rp14 ^ rp15锛?锛堟垨鑰呭鏋滄効鎰忥紝鎴戜滑鍙互閬垮厤璁＄畻 rp15锛岃€屼粠 rp14 绠楀嚭鏉ワ級銆傝繖灏辨槸涓轰粈涔堟湁浜涘湴鏂规彁鍒颁簡 inverse parity锛堥€嗗鍋舵牎楠岋級銆?褰撶劧锛屽悓鏍风殑鎯呭喌閫傜敤浜?rp4/5銆乺p6/7銆乺p8/9銆乺p10/11 鍜?rp12/13銆?瀹為檯涓婅繖鎰忓懗鐫€鎴戜滑鍙互浠?if 璇彞涓幓鎺?else 瀛愬彞銆傝€屼笖鎴戜滑杩樺彲浠ラ€氳繃鍏堜粠 long 鍥炲埌 byte 鏉ュ湪鏈€鍚庣◢寰紭鍖栦竴涓嬭绠椼€備簨瀹炰笂鎴戜滑鐢氳嚦鍙互涓嶇敤鏌ヨ〃銆?

## 灏濊瘯 3

```
          if (i & 0x01) rp5 ^= cur; else rp4 ^= cur;
          if (i & 0x02) rp7 ^= cur; else rp6 ^= cur;
          if (i & 0x04) rp9 ^= cur; else rp8 ^= cur;
          if (i & 0x08) rp11 ^= cur; else rp10 ^= cur;
          if (i & 0x10) rp13 ^= cur; else rp12 ^= cur;
          if (i & 0x20) rp15 ^= cur; else rp14 ^= cur;

```
```
          if (i & 0x01) rp5 ^= cur;
          if (i & 0x02) rp7 ^= cur;
          if (i & 0x04) rp9 ^= cur;
          if (i & 0x08) rp11 ^= cur;
          if (i & 0x10) rp13 ^= cur;
          if (i & 0x20) rp15 ^= cur;

```
```
          rp4  = par ^ rp5;
          rp6  = par ^ rp7;
          rp8  = par ^ rp9;
          rp10  = par ^ rp11;
          rp12  = par ^ rp13;
          rp14  = par ^ rp15;

```
姝ゅ悗浠ｇ爜鑰楁椂澧炲姞浜嗙害 30%锛屽敖绠¤鍙ユ暟閲忓噺灏戜簡銆傛眹缂栦唬鐮佷篃鍙嶆槧浜嗚繖涓€鐐广€?

## 鍒嗘瀽 3

寰堝鎬€傛垜鐚滆繖涓庣紦瀛樻垨鎸囦护骞惰涔嬬被鏈夊叧銆傛垜涔熷湪 eeePC锛圕eleron锛屼富棰?900 Mhz锛変笂璇曡繃銆備竴涓湁瓒ｇ殑瑙傚療鏄紝鍦ㄦ墽琛岃繖娈典唬鐮佹椂锛屽畠鍙瘮鎴?3Ghz 鐨?D920 澶勭悊鍣ㄦ參 30%锛堟牴鎹?time 娴嬮噺锛夈€?
鍡紝鏈潵灏辩煡閬撲笉浼氬鏄擄紝鎵€浠ヤ篃璁歌鎹㈡潯璺細璁╂垜浠洖鍒板皾璇?2 鐨勪唬鐮佸苟鍋氫竴鐐瑰惊鐜睍寮€锛坙oop unrolling锛夈€傝繖浼氭秷闄ゅ嚑涓?if 璇彞銆傛垜浼氬皾璇曚笉鍚屽睍寮€閲忥紝鐪嬬湅鍝釜鏁堟灉鏈€濂姐€?

## 灏濊瘯 4

灏嗗惊鐜睍寮€ 1銆?銆? 鍜?4 娆°€?```

    for (i = 0; i < 4; i++)
    {
        cur = *bp++;
        par ^= cur;
        rp4 ^= cur;
        rp6 ^= cur;
        rp8 ^= cur;
        rp10 ^= cur;
        if (i & 0x1) rp13 ^= cur; else rp12 ^= cur;
        if (i & 0x2) rp15 ^= cur; else rp14 ^= cur;
        cur = *bp++;
        par ^= cur;
        rp5 ^= cur;
        rp6 ^= cur;
        ...


```
## 鍒嗘瀽 4

灞曞紑涓€娆¤幏寰楃害 15% 鐨勬彁鍗?
灞曞紑涓ゆ灏嗘彁鍗囦繚鎸佸湪绾?15%

灞曞紑涓夋鐩告瘮灏濊瘯 2 鑾峰緱 30% 鐨勬彁鍗囥€?
灞曞紑鍥涙鐩告瘮灞曞紑涓夋鍙湁杈归檯鏀硅繘銆?
鎴戝喅瀹氭棤璁哄浣曡繕鏄户缁噰鐢ㄥ睍寮€鍥涙鐨勫惊鐜€傛垜鐨勭洿瑙夋槸锛屽湪鎺ヤ笅鏉ョ殑姝ラ涓垜浼氫粠涓幏寰楅澶栫殑鏀剁泭銆?
涓嬩竴姝ョ殑瑙﹀彂鐐规槸锛歱ar 鍖呭惈浜嗘墍鏈夊瓧鑺傜殑 xor锛岃€?rp4 鍜?rp5 鍚勮嚜鍖呭惈浜嗕竴鍗婂瓧鑺傜殑 xor銆?鎵€浠ュ疄闄呬笂 par = rp4 ^ rp5銆備絾鐢变簬 xor 鏄彲浜ゆ崲鐨勶紝鎴戜滑涔熷彲浠ヨ rp5 = par ^ rp4銆傚洜姝ゆ棤闇€鍚屾椂淇濈暀 rp4 鍜?rp5銆傛垜浠彲浠ュ幓鎺?rp5锛堟垨 rp4锛屼絾鎴戝凡缁忛瑙佸埌浜嗗彟涓€涓紭鍖栵級銆?鍚屾牱鐨勬儏鍐甸€傜敤浜?rp6/7銆乺p8/9銆乺p10/11銆乺p12/13 鍜?rp14/15銆?

## 灏濊瘯 5

瀹為檯涓婏紝寰幆涓墍鏈夌殑濂囨暟 rp 璧嬪€奸兘琚Щ闄や簡銆?杩欏寘鎷?if 璇彞鐨?else 瀛愬彞銆?```

    rp5 = par ^ rp4;

```
鍚屾椂锛屽垵濮嬭祴鍊硷紙rp5 = 0; 绛夛級涔熷彲浠ョЩ闄ゃ€?娌挎鎬濊矾锛屾垜涔熺Щ闄や簡 rp0/1/2/3 鐨勫垵濮嬪寲銆?

## 鍒嗘瀽 5

娴嬮噺琛ㄦ槑杩欐槸涓€涓ソ涓炬帾銆傜浉姣斿睍寮€鍥涙鐨勫皾璇?4锛岃繍琛屾椂闂村ぇ绾﹀噺鍗婏紝鑰屼笖鐩告瘮 Linux 鍐呮牳涓綋鍓嶇殑浠ｇ爜锛屾垜浠彧闇€瑕?1/3 鐨勫鐞嗗櫒鏃堕棿銆?
鐒惰€岋紝鎴戜粛瑙夊緱杩樻湁绌洪棿銆傛垜涓嶅枩娆㈤偅浜?if 璇彞銆備负浠€涔堜笉淇濇寔涓€涓繍琛屼腑鐨勫鍋舵牎楠岋紝鍙繚鐣欐渶鍚庝竴涓?if 璇彞銆傛槸鏃跺€欏啀鏉ヤ竴涓増鏈簡锛?

## 灏濊瘯 6

```

    for (i = 0; i < 4; i++)
    {
        cur = *bp++; tmppar  = cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= tmppar;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp8 ^= tmppar;

        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp10 ^= tmppar;

        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp6 ^= cur; rp8 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur; rp8 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp8 ^= cur;
        cur = *bp++; tmppar ^= cur; rp8 ^= cur;

        cur = *bp++; tmppar ^= cur; rp4 ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur;

        par ^= tmppar;
        if ((i & 0x1) == 0) rp12 ^= tmppar;
        if ((i & 0x2) == 0) rp14 ^= tmppar;
    }

```
濡備綘鎵€瑙侊紝tmppar 鐢ㄤ簬鍦?for 寰幆鐨勪竴娆¤凯浠ｅ唴绱Н濂囧伓鏍￠獙銆傚湪鏈€鍚庣殑 3 鏉¤鍙ヤ腑锛屽畠琚姞鍒?par 涓婏紝骞跺湪闇€瑕佹椂鍔犲埌 rp12 鍜?rp14 涓娿€?
鍦ㄥ仛杩欎簺鏀瑰姩鐨勫悓鏃讹紝鎴戣繕鍙戠幇鍙互鍒╃敤 tmppar 鍖呭惈鏈杩唬杩愯涓鍋舵牎楠岃繖涓€鐐广€傛墍浠ヤ笌鍏跺啓锛?rp4 ^= cur; rp6 ^= cur;
鎴戝幓鎺変簡 rp6 ^= cur; 璇彞锛屽苟鍦ㄤ笅涓€鏉¤鍙ヤ腑鍋?rp6 ^= tmppar;銆傚 rp8 鍜?rp10 涔熷仛浜嗙被浼肩殑鏀瑰姩銆?

## 鍒嗘瀽 6

鍐嶆娴嬮噺杩欐浠ｇ爜鏄剧ず浜嗗法澶х殑鏀剁泭銆傚綋鎵ц鍘熷鐨?Linux 浠ｇ爜 100 涓囨鏃讹紝鍦ㄦ垜鐨勭郴缁熶笂澶х害闇€瑕?1 绉掋€?锛堜娇鐢?time 鏉ユ祴閲忔€ц兘锛夈€傜粡杩囪繖娆¤凯浠ｅ悗鎴戝洖鍒颁簡 0.075 绉掋€傚疄闄呬笂鎴戜笉寰椾笉鍐冲畾鍦ㄨ秴杩?1000 涓囨杩唬涓婃祴閲忥紝浠ュ厤鎹熷け杩囧绮惧害銆傝繖缁濆鐪嬩笂鍘绘槸涓ぇ濂栦簡锛?
涓嶈繃杩樻湁涓€鐐规敼杩涚┖闂淬€傚湪寰幆涓湁涓夊
```

	rp4 ^= cur; rp6 ^= cur;

```
缁存姢涓€涓彉閲?rp4_6 浼间箮鏇撮珮鏁堬紱杩欐瘡娆″惊鐜秷闄?3 鏉¤鍙ャ€傚綋鐒跺湪寰幆涔嬪悗鎴戜滑
```

	rp4 ^= rp4_6;
	rp6 ^= rp4_6

```
姝ゅ鏈?4 鏉￠『搴忕殑璧嬪€肩粰 rp8銆傝繖鍙互鐢ㄧ暐寰洿楂樻晥鐨勬柟寮忕紪鐮侊細鍦ㄩ偅 4 琛屼箣鍓嶄繚瀛?tmppar锛屼箣鍚庡啀鍋?rp8 = rp8 ^ tmppar ^ notrp8;
锛堝叾涓?notrp8 鏄偅 4 琛屼箣鍓?rp8 鐨勫€硷級銆?杩欏啀娆″埄鐢ㄤ簡 xor 鐨勫彲浜ゆ崲鎬ц川銆傛槸鏃跺€欏仛鏂版祴璇曚簡锛?

## 灏濊瘯 7

```

    for (i = 0; i < 4; i++)
    {
        cur = *bp++; tmppar  = cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= tmppar;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp8 ^= tmppar;

        cur = *bp++; tmppar ^= cur; rp4_6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur; rp10 ^= tmppar;

        notrp8 = tmppar;
        cur = *bp++; tmppar ^= cur; rp4_6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur;
        rp8 = rp8 ^ tmppar ^ notrp8;

        cur = *bp++; tmppar ^= cur; rp4_6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp6 ^= cur;
        cur = *bp++; tmppar ^= cur; rp4 ^= cur;
        cur = *bp++; tmppar ^= cur;

        par ^= tmppar;
        if ((i & 0x1) == 0) rp12 ^= tmppar;
        if ((i & 0x2) == 0) rp14 ^= tmppar;
    }
    rp4 ^= rp4_6;
    rp6 ^= rp4_6;


```
鏀瑰姩涓嶅ぇ锛屼絾绉皯鎴愬 :-)


## 鍒嗘瀽 7

瀹為檯涓婅繖璁╀簨鎯呭彉绯熶簡銆備笉澶锛屼絾鎴戜笉鎯冲線閿欒鐨勬柟鍚戣蛋銆備篃璁镐互鍚庡彲浠ョ爺绌朵竴涓嬨€傚彲鑳藉張鍜岀紦瀛樻湁鍏炽€?
鎴戞兂寰幆鍐呰兘璧㈢殑涔熷氨杩欎簺浜嗐€傚啀澶氬睍寮€涓€娆′篃璁告湁甯姪銆傛垜鏆傛椂淇濈暀鏉ヨ嚜灏濊瘯 7 鐨勪紭鍖栥€?

## 灏濊瘯 8

灏嗗惊鐜啀灞曞紑涓€娆°€?

## 鍒嗘瀽 8

杩欒浜嬫儏鍙樼碂浜嗐€傝鎴戜滑鍧氭寔灏濊瘯 6锛屽苟浠庨偅閲岀户缁€傝櫧鐒跺惊鐜唴鐨勪唬鐮佷技涔庢棤娉曡繘涓€姝ヤ紭鍖栵紝浣嗙敓鎴?ecc 鐮佷粛鏈変紭鍖栫┖闂淬€?鎴戜滑鍙互绠€鍗曞湴璁＄畻鎬诲鍋舵牎楠屻€傚鏋滃畠鏄?0锛岄偅涔?rp4 = rp5 绛夌瓑銆傚鏋滃鍋舵牎楠屾槸 1锛岄偅涔?rp4 = !rp5锛?
浣嗗鏋?rp4 = rp5 鎴戜滑灏变笉闇€瑕?rp5 绛夌瓑銆傛垜浠彲浠ョ洿鎺ュ啓鍏ュ伓鏁颁綅
```

    code[0] |= (code[0] << 1);

```
璁╂垜浠祴璇曚竴涓嬨€?

## 灏濊瘯 9

鏀逛簡浠ｇ爜锛屼絾鍚屾牱杩欑暐寰檷浣庝簡鎬ц兘銆傝瘯杩囧悇绉嶅叾浠栧姙娉曪紝姣斿浣跨敤涓撶敤鐨勫鍋舵牎楠屾暟缁勪互閬垮厤 parity[rp7] << 7 涔嬪悗鐨勭Щ浣嶃€傛病鏈夋敹鐩娿€?浣跨敤绉讳綅杩愮畻绗︼紙渚嬪锛夋潵鏇挎崲 parity 鏁扮粍鐨勬煡鎵撅細
```

	rp7 ^= (rp7 << 4);
	rp7 ^= (rp7 << 2);
	rp7 ^= (rp7 << 1);
	rp7 &= 0x80;

```
娌℃湁鏀剁泭銆?
鍞竴鐨勮竟闄呮敼鍔ㄦ槸鍙嶈浆濂囧伓鏍￠獙浣嶏紝杩欐牱鎴戜滑鍙互鍘绘帀鏈€鍚庝笁鏉?invert 璇彞銆?
鍟婏紝鐪熷彲鎯滆繖娌℃湁甯︽潵鏇村鏀剁泭銆傝瘽璇村洖鏉ワ紝浣跨敤 Linux 椹卞姩浠ｇ爜鎵ц 1000 涓囨杩唬闇€瑕?13 鍒?13.5 绉掞紝鑰屾垜鐨勪唬鐮佺幇鍦ㄥ杩欎簺 1000 涓囨杩唬澶х害鍙渶 0.73 绉掋€傛墍浠ュ熀鏈笂鎴戝湪鎴戠殑绯荤粺涓婂皢鎬ц兘鎻愬崌浜?18 鍊嶃€傝繕涓嶈禆銆傚綋鐒跺湪涓嶅悓鐨勭‖浠朵笂浣犱細寰楀埌涓嶅悓鐨勭粨鏋溿€備笉鎻愪緵浠讳綍淇濊瘉锛?
浣嗗綋鐒跺ぉ涓嬫病鏈夊厤璐圭殑鍗堥銆備唬鐮佸ぇ灏忓嚑涔庣炕浜嗕笁鍊嶏紙浠?562 瀛楄妭鍒?1434 瀛楄妭锛夈€傝瘽鍙堣鍥炴潵锛屼篃娌￠偅涔堝ぇ銆?

## 绾犳閿欒

瀵逛簬绾犳閿欒锛屾垜鍐嶆浠?ST 搴旂敤绗旇涓鸿捣鐐癸紝浣嗘垜涔熺瀯浜嗕竴鐪肩幇鏈変唬鐮併€?
绠楁硶鏈韩鐩稿綋鐩存帴銆傚彧闇€ xor 缁欏畾鐨?ecc 涓庤绠楀嚭鐨?ecc銆傚鏋滄墍鏈夊瓧鑺傞兘鏄?0 灏辨病鏈夐棶棰樸€傚鏋滄湁 11 浣嶆槸 1锛屾垜浠氨鏈変竴涓彲绾犳鐨勪綅閿欒銆傚鏋滃彧鏈?1 浣嶆槸 1锛屽垯缁欏畾鐨?ecc 鐮佷腑鏈夐敊璇€?
璇佹槑鏈€蹇殑鏂规硶鏄仛鍑犳鏌ヨ〃銆傚綋闇€瑕佸仛淇鏃讹紝鐢辨寮曞叆鐨勬€ц兘鎻愬崌鍦ㄦ垜鐨勭郴缁熶笂绾︿负 2 鍊嶏紱鑰屽綋鏃犻渶淇鏃跺垯鏄?1% 宸﹀彸銆?
璇ュ嚱鏁扮殑浠ｇ爜澶у皬浠?330 瀛楄妭澧炲姞鍒?686 瀛楄妭銆?锛坓cc 4.2, -O3锛?

## 缁撹

璁＄畻 ecc 鏃剁殑鏀剁泭鏄法澶х殑銆傚湪鎴戠殑寮€鍙戠‖浠朵笂鑾峰緱浜?ecc 璁＄畻 18 鍊嶇殑鍔犻€熴€傚湪涓€涓甫鏈?MIPS 鏍稿績鐨勫祵鍏ュ紡绯荤粺娴嬭瘯涓幏寰椾簡 7 鍊嶃€?
鍦?Linksys NSLU2锛圓RMv5TE 澶勭悊鍣級鐨勬祴璇曚腑鍔犻€熶负 5 鍊嶏紙澶х妯″紡锛実cc 4.1.2, -O3锛?
瀵逛簬绾犳鍒欏緱涓嶅埌澶鏀剁泭锛堝洜涓轰綅缈昏浆寰堢綍瑙侊級銆傝瘽璇村洖鏉ワ紝閭ｉ噷鑺辫垂鐨勫懆鏈熶篃瑕佸皯寰楀銆?
浼间箮鍦ㄨ繖鏂归潰娌℃湁澶鍙耽鐨勭┖闂翠簡锛岃嚦灏戝湪鐢?C 缂栫▼鐨勬儏鍐典笅銆傚綋鐒剁敤姹囩紪绋嬪簭涔熻鑳戒粠瀹冮噷闈㈠啀姒ㄥ嚭涓€鐐癸紝浣嗙敱浜庢祦姘寸嚎琛屼负绛夊師鍥狅紝杩欓潪甯告鎵嬶紙鑷冲皯瀵?intel 纭欢鑰岃█锛夈€?
Author: Frans Meulenbroeks

Copyright (C) 2008 Koninklijke Philips Electronics NV.
