## BPF Type Format锛圔TF锛孊PF 绫诲瀷鏍煎紡锛?


## 1. 绠€浠?


BTF锛圔PF Type Format锛孊PF 绫诲瀷鏍煎紡锛夋槸涓€绉嶅厓鏁版嵁鏍煎紡锛岀敤浜庣紪鐮佷笌 BPF
绋嬪簭/鏄犲皠锛坢ap锛夌浉鍏崇殑璋冭瘯淇℃伅銆侭TF 杩欎釜鍚嶅瓧鏈€鍒濈敤浜庢弿杩版暟鎹被鍨嬨€傚悗鏉?BTF
琚墿灞曚负鍚屾椂鍖呭惈宸插畾涔夊瓙渚嬬▼鐨勫嚱鏁颁俊鎭紝浠ュ強婧愭枃浠?琛屽彿淇℃伅銆?

杩欎簺璋冭瘯淇℃伅鍙敤浜庢槧灏勭殑鍙嬪ソ鎵撳嵃锛坧retty print锛夈€佸嚱鏁扮鍚嶇瓑銆傚嚱鏁扮鍚?
浣垮緱 BPF 绋嬪簭/鍑芥暟鐨勫唴鏍哥鍙锋樉绀哄緱鏇村ソ銆傝鍙蜂俊鎭湁鍔╀簬鐢熸垚甯︽簮鐮佹爣娉ㄧ殑
缈昏瘧鍚庡瓧鑺傜爜銆丣IT 鍚庝唬鐮佷互鍙婇獙璇佸櫒锛坴erifier锛夋棩蹇椼€?

BTF 瑙勮寖鍖呭惈涓や釜閮ㄥ垎锛?
  - BTF 鍐呮牳 API
  - BTF ELF 鏂囦欢鏍煎紡

鍐呮牳 API 鏄敤鎴风┖闂翠笌鍐呮牳涔嬮棿鐨勫绾︺€傚唴鏍稿湪浣跨敤 BTF 淇℃伅涔嬪墠浼氬厛瀵瑰叾
杩涜鏍￠獙銆侲LF 鏂囦欢鏍煎紡鍒欐槸 ELF 鏂囦欢涓?libbpf 鍔犺浇鍣ㄤ箣闂寸殑鐢ㄦ埛绌洪棿濂戠害銆?

绫诲瀷锛坱ype锛夊拰瀛楃涓诧紙string锛夋灞炰簬 BTF 鍐呮牳 API 鐨勪竴閮ㄥ垎锛屾弿杩颁簡琚?BPF
绋嬪簭寮曠敤鐨勮皟璇曚俊鎭紙涓昏鏄笌绫诲瀷鐩稿叧鐨勪俊鎭級銆傝繖涓ゆ鍦?BTF_Type_String
涓湁璇︾粏璁ㄨ銆?


## 2. BTF 绫诲瀷涓庡瓧绗︿覆缂栫爜


鏂囦欢 `include/uapi/linux/btf.h` 鎻愪緵浜嗙被鍨?瀛楃涓插浣曠紪鐮佺殑楂樺眰瀹氫箟銆?

```

    struct btf_header {
        __u16   magic;
        __u8    version;
        __u8    flags;
        __u32   hdr_len;

        /* 鎵€鏈夊亸绉婚噺閮戒互瀛楄妭涓哄崟浣嶏紝鐩稿浜庢湰澶撮儴鏈熬 */
        __u32   type_off;       /* 绫诲瀷娈靛亸绉?      */
        __u32   type_len;       /* 绫诲瀷娈甸暱搴?      */
        __u32   str_off;        /* 瀛楃涓叉鍋忕Щ     */
        __u32   str_len;        /* 瀛楃涓叉闀垮害     */
    };

```
magic 涓?`0xeB9F`锛屽湪澶х鍜屽皬绔郴缁熶笂缂栫爜涓嶅悓锛屽彲鐢ㄤ簬娴嬭瘯 BTF 鏄负澶х
杩樻槸灏忕鐩爣鐢熸垚鐨勩€傝璁?`btf_header` 鏃朵繚鐣欎簡鍙墿灞曟€э紝褰撶敓鎴愭暟鎹潡鏃?
`hdr_len` 绛変簬 `sizeof(struct btf_header)`銆?

### 2.1 瀛楃涓茬紪鐮?


瀛楃涓叉涓殑绗竴涓瓧绗︿覆蹇呴』鏄┖瀛楃涓层€傚瓧绗︿覆琛ㄧ殑鍏朵綑閮ㄥ垎鏄叾浠?
浠?null 缁撳熬鐨勫瓧绗︿覆鐨勬嫾鎺ャ€?

### 2.2 绫诲瀷缂栫爜


绫诲瀷 id `0` 淇濈暀缁?`void` 绫诲瀷銆傜被鍨嬫琚『搴忚В鏋愶紝绫诲瀷 id 浠?1 寮€濮?
渚濇鍒嗛厤缁欐瘡涓璇嗗埆鍑虹殑绫诲瀷銆?
```

    #define BTF_KIND_INT            1       /* Integer      */
    #define BTF_KIND_PTR            2       /* Pointer      */
    #define BTF_KIND_ARRAY          3       /* Array        */
    #define BTF_KIND_STRUCT         4       /* Struct       */
    #define BTF_KIND_UNION          5       /* Union        */
    #define BTF_KIND_ENUM           6       /* Enumeration up to 32-bit values */
    #define BTF_KIND_FWD            7       /* Forward      */
    #define BTF_KIND_TYPEDEF        8       /* Typedef      */
    #define BTF_KIND_VOLATILE       9       /* Volatile     */
    #define BTF_KIND_CONST          10      /* Const        */
    #define BTF_KIND_RESTRICT       11      /* Restrict     */
    #define BTF_KIND_FUNC           12      /* Function     */
    #define BTF_KIND_FUNC_PROTO     13      /* Function Proto       */
    #define BTF_KIND_VAR            14      /* Variable     */
    #define BTF_KIND_DATASEC        15      /* Section      */
    #define BTF_KIND_FLOAT          16      /* Floating point       */
    #define BTF_KIND_DECL_TAG       17      /* Decl Tag     */
    #define BTF_KIND_TYPE_TAG       18      /* Type Tag     */
    #define BTF_KIND_ENUM64         19      /* Enumeration up to 64-bit values */

```
娉ㄦ剰绫诲瀷娈电紪鐮佺殑鏄皟璇曚俊鎭紝鑰屼笉浠呬粎鏄函绮圭殑绫诲瀷銆俙BTF_KIND_FUNC` 涓嶆槸
涓€涓被鍨嬶紝瀹冭〃绀轰竴涓凡瀹氫箟鐨勫瓙绋嬪簭銆?

```

    struct btf_type {
        __u32 name_off;
        /* "info" 浣嶇殑鎺掑竷
         * bits  0-15: vlen锛堜緥濡?struct 鐨勬垚鍛樻暟锛?
         * bits 16-23: 鏈娇鐢?
         * bits 24-28: kind锛堜緥濡?int銆乸tr銆乤rray鈥︹€︾瓑锛?
         * bits 29-30: 鏈娇鐢?
         * bit     31: kind_flag锛岀洰鍓嶇敤浜?
         *             struct銆乽nion銆乪num銆乫wd銆乪num64銆?
         *             decl_tag 鍜?type_tag
         */
        __u32 info;
        /* "size" 鐢ㄤ簬 INT銆丒NUM銆丼TRUCT銆乁NION 鍜?ENUM64銆?
         * "size" 琛ㄧず瀹冩墍鎻忚堪绫诲瀷鐨勫ぇ灏忋€?
         *
         * "type" 鐢ㄤ簬 PTR銆乀YPEDEF銆乂OLATILE銆丆ONST銆丷ESTRICT銆?
         * FUNC銆丗UNC_PROTO銆丏ECL_TAG 鍜?TYPE_TAG銆?
         * "type" 鏄竴涓寚鍚戝彟涓€涓被鍨嬬殑 type_id銆?
         */
        union {
                __u32 size;
                __u32 type;
        };
    };

```
瀵逛簬鏌愪簺 kind锛屽叕鍏辨暟鎹箣鍚庝細璺熼殢璇?kind 鐗规湁鐨勬暟鎹€俙struct btf_type`
涓殑 `name_off` 鎸囧畾浜嗗湪瀛楃涓茶〃涓殑鍋忕Щ閲忋€備互涓嬪悇灏忚妭璇︾粏璇存槑浜嗘瘡绉?
kind 鐨勭紪鐮併€?

#### 2.2.1 BTF_KIND_INT


`struct btf_type` 缂栫爜瑕佹眰锛?
 - `name_off`锛氫换鎰忔湁鏁堝亸绉?
 - `info.kind_flag`锛?
 - `info.kind`锛欱TF_KIND_INT
 - `info.vlen`锛?
 - `size`锛歩nt 绫诲瀷鐨勫ぇ灏忥紙瀛楄妭鏁帮級

```

  #define BTF_INT_ENCODING(VAL)   (((VAL) & 0x0f000000) >> 24)
  #define BTF_INT_OFFSET(VAL)     (((VAL) & 0x00ff0000) >> 16)
  #define BTF_INT_BITS(VAL)       ((VAL)  & 0x000000ff)

```
```

  #define BTF_INT_SIGNED  (1 << 0)
  #define BTF_INT_CHAR    (1 << 1)
  #define BTF_INT_BOOL    (1 << 2)

```
`BTF_INT_ENCODING()` 鎻愪緵棰濆淇℃伅锛氭湁绗﹀彿鎬э紙signedness锛夈€乧har 鎴?
bool锛岄拡瀵?int 绫诲瀷銆俢har 鍜?bool 缂栫爜涓昏鐢ㄤ簬鍙嬪ソ鎵撳嵃銆俰nt 绫诲瀷
鏈€澶氬彧鑳芥寚瀹氫竴绉嶇紪鐮併€?

`BTF_INT_BITS()` 鎸囧畾璇?int 绫诲瀷鎵€鎸佹湁鐨勫疄闄呬綅鏁般€備緥濡傦紝涓€涓?4 浣嶇殑浣嶅煙
缂栫爜涓?`BTF_INT_BITS()` 绛変簬 4銆俙btf_type.size * 8` 蹇呴』
澶т簬鎴栫瓑浜庤绫诲瀷鐨?`BTF_INT_BITS()`銆俙BTF_INT_BITS()` 鐨勬渶澶у€间负 128銆?

`BTF_INT_OFFSET()` 鎸囧畾璁＄畻璇?int 鍊兼椂鐨勮捣濮嬩綅鍋忕Щ銆備緥濡傦紝涓€涓綅鍩?
struct 鎴愬憳鍏锋湁锛?

 - btf 鎴愬憳鐩稿缁撴瀯浣撹捣濮嬬殑浣嶅亸绉讳负 100锛?
 - btf 鎴愬憳鎸囧悜涓€涓?int 绫诲瀷锛?
 - 璇?int 绫诲瀷鐨?`BTF_INT_OFFSET() = 2` 涓?`BTF_INT_BITS() = 4`

閭ｄ箞鍦ㄧ粨鏋勪綋鍐呭瓨甯冨眬涓紝璇ユ垚鍛樺皢鍗犳嵁浠庝綅 `100 + 2 = 102` 寮€濮嬬殑 `4` 浣嶃€?

鍙︿竴绉嶆柟寮忔槸锛屼笅闈㈢殑浣嶅煙 struct 鎴愬憳鍙互璁块棶涓庝笂杩扮浉鍚岀殑浣嶏細

 - btf 鎴愬憳浣嶅亸绉讳负 102锛?
 - btf 鎴愬憳鎸囧悜涓€涓?int 绫诲瀷锛?
 - 璇?int 绫诲瀷鐨?`BTF_INT_OFFSET() = 0` 涓?`BTF_INT_BITS() = 4`

`BTF_INT_OFFSET()` 鐨勫師濮嬫剰鍥炬槸涓轰簡鎻愪緵浣嶅煙缂栫爜鐨勭伒娲绘€с€傜洰鍓嶏紝llvm 鍜?
pahole 瀵规墍鏈?int 绫诲瀷閮界敓鎴?`BTF_INT_OFFSET() = 0`銆?

#### 2.2.2 BTF_KIND_PTR


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛?
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_PTR
  - `info.vlen`锛?
  - `type`锛氳鎸囬拡鎵€鎸囧悜鐨勭被鍨?

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.3 BTF_KIND_ARRAY


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛?
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_ARRAY
  - `info.vlen`锛?
  - `size/type`锛?锛屾湭浣跨敤

```

    struct btf_array {
        __u32   type;
        __u32   index_type;
        __u32   nelems;
    };

```
`struct btf_array` 鐨勭紪鐮侊細
  - `type`锛氬厓绱犵被鍨?
  - `index_type`锛氱储寮曠被鍨?
  - `nelems`锛氳鏁扮粍鐨勫厓绱犱釜鏁帮紙`0` 涔熷厑璁革級

`index_type` 鍙互鏄换鎰忓父瑙?int 绫诲瀷锛坄u8`銆乣u16`銆乣u32`銆乣u64`銆?
`unsigned __int128`锛夈€傚寘鍚?`index_type` 鐨勫師濮嬭璁￠伒寰?DWARF锛屽洜涓?
DWARF 鐨勬暟缁勭被鍨嬩篃甯︽湁涓€涓?`index_type`銆傜洰鍓嶅湪 BTF 涓紝闄や簡绫诲瀷鏍￠獙涔嬪锛?
`index_type` 骞舵湭琚娇鐢ㄣ€?

`struct btf_array` 閫氳繃鍏冪礌绫诲瀷閾惧寲浠ユ敮鎸佸缁存暟缁勩€備緥濡傦紝瀵逛簬
`int a[^5^][^6^]`锛屼笅闈㈢被鍨嬩俊鎭睍绀轰簡閾惧寲杩囩▼锛?

  - [^1^]锛歩nt
  - [^2^]锛歛rray锛宍btf_array.type = [^1^]`锛宍btf_array.nelems = 6`
  - [^3^]锛歛rray锛宍btf_array.type = [^2^]`锛宍btf_array.nelems = 5`

鐩墠锛宲ahole 鍜?llvm 閮戒細鎶婂缁存暟缁勬姌鍙犳垚涓€缁存暟缁勶紝渚嬪瀵逛簬
`a[^5^][^6^]`锛宍btf_array.nelems` 绛変簬 `30`銆傝繖鏄洜涓烘渶鍒濈殑浣跨敤鍦烘櫙鏄?
map 鍙嬪ソ鎵撳嵃锛屽湪閭ｉ噷鏁翠釜鏁扮粍閮借 dump 鍑烘潵锛屾墍浠ヤ竴缁存暟缁勫氨瓒冲浜嗐€傞殢鐫€
鏇村 BTF 鐢ㄩ€旇鎸栨帢锛宲ahole 鍜?llvm 鍙互鏀逛负鐢熸垚閽堝澶氱淮鏁扮粍鐨勬伆褰撻摼鍖?
琛ㄧず銆?

#### 2.2.4 BTF_KIND_STRUCT

#### 2.2.5 BTF_KIND_UNION


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛? 鎴栨寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛? 鎴?1
  - `info.kind`锛欱TF_KIND_STRUCT 鎴?BTF_KIND_UNION
  - `info.vlen`锛歴truct/union 鎴愬憳鐨勪釜鏁?
  - `info.size`锛歴truct/union 鐨勫ぇ灏忥紙瀛楄妭鏁帮級

```

    struct btf_member {
        __u32   name_off;
        __u32   type;
        __u32   offset;
    };

```
`struct btf_member` 缂栫爜锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `type`锛氭垚鍛樼被鍨?
  - `offset`锛?瑙佷笅鏂?

濡傛灉绫诲瀷淇℃伅 `kind_flag` 鏈疆浣嶏紝offset 鍙惈鏈夎鎴愬憳鐨勪綅鍋忕Щ銆傛敞鎰忎綅鍩熺殑
鍩虹被鍨嬪彧鑳芥槸 int 鎴?enum 绫诲瀷銆傚鏋滀綅鍩熷ぇ灏忎负 32锛屽熀绫诲瀷鍙互鏄?int 鎴?
enum 绫诲瀷銆傚鏋滀綅鍩熷ぇ灏忎笉涓?32锛屽熀绫诲瀷蹇呴』鏄?int锛屼笖 int 绫诲瀷鐨?
`BTF_INT_BITS()` 缂栫爜浜嗕綅鍩熷ぇ灏忋€?

濡傛灉 `kind_flag` 琚疆浣嶏紝`btf_member.offset` 鍚屾椂鍖呭惈鎴愬憳鐨勪綅鍩熷ぇ灏忓拰浣嶅亸绉汇€?
浣嶅煙澶у皬鍜屼綅鍋忕Щ鎸夊涓嬫柟寮忚绠楋細
```

  #define BTF_MEMBER_BITFIELD_SIZE(val)   ((val) >> 24)
  #define BTF_MEMBER_BIT_OFFSET(val)      ((val) & 0xffffff)

```
鍦ㄨ繖绉嶆儏鍐典笅锛屽鏋滃熀绫诲瀷鏄?int 绫诲瀷锛屽畠蹇呴』鏄父瑙?int 绫诲瀷锛?

  - `BTF_INT_OFFSET()` 蹇呴』涓?0銆?
  - `BTF_INT_BITS()` 蹇呴』绛変簬 `{1,2,4,8,16} * 8`銆?

commit 9d5f9f701b18 寮曞叆浜?`kind_flag`锛屽苟瑙ｉ噴浜嗕负浣曚袱绉嶆ā寮忓苟瀛樸€?

#### 2.2.6 BTF_KIND_ENUM


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛? 鎴栨寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛氭棤绗﹀彿涓?0锛屾湁绗﹀彿涓?1
  - `info.kind`锛欱TF_KIND_ENUM
  - `info.vlen`锛歟num 鍊肩殑涓暟
  - `size`锛?/2/4/8

```

    struct btf_enum {
        __u32   name_off;
        __s32   val;
    };

```
`btf_enum` 缂栫爜锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `val`锛氫换鎰忓€?

濡傛灉鍘熷 enum 鍊兼槸鏈夌鍙风殑涓斿ぇ灏忓皬浜?4锛岃鍊间細琚鍙锋墿灞曚负 4 瀛楄妭銆傚鏋?
澶у皬涓?8锛岃鍊间細琚埅鏂负 4 瀛楄妭銆?

#### 2.2.7 BTF_KIND_FWD


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛歴truct 涓?0锛寀nion 涓?1
  - `info.kind`锛欱TF_KIND_FWD
  - `info.vlen`锛?
  - `type`锛?

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.8 BTF_KIND_TYPEDEF


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_TYPEDEF
  - `info.vlen`锛?
  - `type`锛歚name_off` 澶勫悕瀛楁墍鑳芥寚浠ｇ殑绫诲瀷

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.9 BTF_KIND_VOLATILE


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛?
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_VOLATILE
  - `info.vlen`锛?
  - `type`锛氬甫鏈?`volatile` 闄愬畾鐨勭被鍨?

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.10 BTF_KIND_CONST


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛?
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_CONST
  - `info.vlen`锛?
  - `type`锛氬甫鏈?`const` 闄愬畾鐨勭被鍨?

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.11 BTF_KIND_RESTRICT


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛?
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_RESTRICT
  - `info.vlen`锛?
  - `type`锛氬甫鏈?`restrict` 闄愬畾鐨勭被鍨?

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.12 BTF_KIND_FUNC


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_FUNC
  - `info.vlen`锛氶摼鎺ヤ俊鎭紙BTF_FUNC_STATIC銆丅TF_FUNC_GLOBAL
                   鎴?BTF_FUNC_EXTERN 鈥斺€?瑙?BTF_Function_Linkage_Constants锛?
  - `type`锛氫竴涓?BTF_KIND_FUNC_PROTO 绫诲瀷

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

BTF_KIND_FUNC 瀹氫箟鐨勪笉鏄竴涓被鍨嬶紝鑰屾槸涓€涓瓙绋嬪簭锛堝嚱鏁帮級锛屽叾绛惧悕鐢?`type`
瀹氫箟銆傚洜姝よ瀛愮▼搴忔槸閭ｄ釜绫诲瀷鐨勪竴涓疄渚嬨€侭TF_KIND_FUNC 鍙嶈繃鏉ュ張鍙兘琚?
BTF_Ext_Section锛圗LF锛変腑鐨?func_info 鎴?BPF_Prog_Load 鐨勫弬鏁帮紙ABI锛夋墍
寮曠敤銆?

鐩墠锛屽唴鏍稿彧鏀寔 BTF_FUNC_STATIC 鍜?BTF_FUNC_GLOBAL 杩欎袱绉嶉摼鎺ュ€笺€?

#### 2.2.13 BTF_KIND_FUNC_PROTO


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛?
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_FUNC_PROTO
  - `info.vlen`锛氬弬鏁颁釜鏁?
  - `type`锛氳繑鍥炵被鍨?

```

    struct btf_param {
        __u32   name_off;
        __u32   type;
    };

```
濡傛灉涓€涓?BTF_KIND_FUNC_PROTO 绫诲瀷琚煇涓?BTF_KIND_FUNC 绫诲瀷寮曠敤锛岄偅涔?
`btf_param.name_off` 蹇呴』鎸囧悜涓€涓湁鏁堢殑 C 鏍囪瘑绗︼紝鍙兘鐨勬渶鍚庝竴涓〃绀哄彲鍙?
鍙傛暟鐨勫弬鏁伴櫎澶栥€俙btf_param.type` 鎸囧悜鍙傛暟绫诲瀷銆?

濡傛灉鍑芥暟甯︽湁鍙彉鍙傛暟锛屾渶鍚庝竴涓弬鏁扮紪鐮佷负 `name_off = 0` 涓?`type = 0`銆?

#### 2.2.14 BTF_KIND_VAR


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_VAR
  - `info.vlen`锛?
  - `type`锛氬彉閲忕殑绫诲瀷

`btf_type` 涔嬪悗璺熼殢涓€涓崟鐙殑 `struct btf_variable`锛屽叾
```

    struct btf_var {
        __u32   linkage;
    };

```
`btf_var.linkage` 鍙彇浠ヤ笅鍊硷細BTF_VAR_STATIC銆丅TF_VAR_GLOBAL_ALLOCATED 鎴?
BTF_VAR_GLOBAL_EXTERN 鈥斺€?瑙?BTF_Var_Linkage_Constants銆?

鐩墠 LLVM 骞堕潪鏀寔鎵€鏈夌被鍨嬬殑鍏ㄥ眬鍙橀噺銆傚綋鍓嶅彲鐢ㄧ殑鏄細

  - 甯︽垨涓嶅甫 section 灞炴€х殑闈欐€佸彉閲?
  - 甯?section 灞炴€х殑鍏ㄥ眬鍙橀噺

鍚庤€呯敤浜庡皢鏉ヤ粠 map 瀹氫箟涓娊鍙?map 閿?鍊肩被鍨?id銆?

#### 2.2.15 BTF_KIND_DATASEC


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛氭寚鍚戜笌鏌愪釜鍙橀噺鐩稿叧鑱旂殑鏈夋晥鍚嶅瓧鐨勫亸绉伙紝鎴栦负
                  .data/.bss/.rodata 涔嬩竴
  - `info.kind_flag`锛?
  - `info.kind`锛欱TF_KIND_DATASEC
  - `info.vlen`锛氬彉閲忎釜鏁?
  - `size`锛氭鎬诲ぇ灏忥紙瀛楄妭鏁帮紝缂栬瘧鏃朵负 0锛岀敱 libbpf 绛?BPF 鍔犺浇鍣?
              琛ヤ竵涓哄疄闄呭ぇ灏忥級

```

    struct btf_var_secinfo {
        __u32   type;
        __u32   offset;
        __u32   size;
    };

```
`struct btf_var_secinfo` 缂栫爜锛?
  - `type`锛欱TF_KIND_VAR 鍙橀噺鐨勭被鍨?
  - `offset`锛氬彉閲忓湪娈靛唴鐨勫亸绉?
  - `size`锛氬彉閲忓ぇ灏忥紙瀛楄妭鏁帮級

#### 2.2.16 BTF_KIND_FLOAT


`struct btf_type` 缂栫爜瑕佹眰锛?
 - `name_off`锛氫换鎰忔湁鏁堝亸绉?
 - `info.kind_flag`锛?
 - `info.kind`锛欱TF_KIND_FLOAT
 - `info.vlen`锛?
 - `size`锛歠loat 绫诲瀷鐨勫ぇ灏忥紙瀛楄妭鏁帮級锛?銆?銆?銆?2 鎴?16銆?

`btf_type` 涔嬪悗娌℃湁棰濆鐨勭被鍨嬫暟鎹€?

#### 2.2.17 BTF_KIND_DECL_TAG


`struct btf_type` 缂栫爜瑕佹眰锛?
 - `name_off`锛氭寚鍚戜竴涓潪绌哄瓧绗︿覆鐨勫亸绉?
 - `info.kind_flag`锛? 鎴?1
 - `info.kind`锛欱TF_KIND_DECL_TAG
 - `info.vlen`锛?
 - `type`锛歚struct`銆乣union`銆乣func`銆乣var` 鎴?`typedef`

```

    struct btf_decl_tag {
        __u32   component_idx;
    };

```
`type` 搴斾负 `struct`銆乣union`銆乣func`銆乣var` 鎴?`typedef`銆傚浜?`var` 鎴?
`typedef` 绫诲瀷锛宍btf_decl_tag.component_idx` 蹇呴』涓?`-1`銆傚浜庡彟澶栦笁绉嶇被鍨嬶紝
濡傛灉 btf_decl_tag 灞炴€у簲鐢ㄤ簬 `struct`銆乣union` 鎴?`func` 鑷韩锛?
`btf_decl_tag.component_idx` 蹇呴』涓?`-1`銆傚惁鍒欙紝璇ュ睘鎬у簲鐢ㄤ簬鏌愪釜
`struct`/`union` 鎴愬憳鎴栨煇涓?`func` 鍙傛暟锛宍btf_decl_tag.component_idx` 搴斾负
涓€涓湁鏁堢储寮曪紙浠?0 寮€濮嬶級锛屾寚鍚戞煇涓垚鍛樻垨鍙傛暟銆?

濡傛灉 `info.kind_flag` 涓?0锛屽垯杩欐槸涓€涓櫘閫氱殑 decl tag锛宍name_off` 缂栫爜鐨勬槸
btf_decl_tag 灞炴€у瓧绗︿覆銆?

濡傛灉 `info.kind_flag` 涓?1锛屽垯璇?decl tag 琛ㄧず浠绘剰鐨?`__attribute__`銆傚湪杩欑
鎯呭喌涓嬶紝`name_off` 缂栫爜鐨勬槸涓€涓唬琛ㄥ睘鎬ц鏄庣锛坅ttribute specifier锛夊睘鎬у垪琛?
鐨勫瓧绗︿覆銆備緥濡傦紝瀵逛簬 `__attribute__((aligned(4)))`锛屽瓧绗︿覆鍐呭涓?`aligned(4)`銆?

#### 2.2.18 BTF_KIND_TYPE_TAG


`struct btf_type` 缂栫爜瑕佹眰锛?
 - `name_off`锛氭寚鍚戜竴涓潪绌哄瓧绗︿覆鐨勫亸绉?
 - `info.kind_flag`锛? 鎴?1
 - `info.kind`锛欱TF_KIND_TYPE_TAG
 - `info.vlen`锛?
 - `type`锛氬甫鏈?`btf_type_tag` 灞炴€х殑绫诲瀷

鐩墠锛宍BTF_KIND_TYPE_TAG` 浠呴拡瀵规寚閽堢被鍨嬬敓鎴愩€傚畠鍏锋湁濡備笅 btf 绫诲瀷閾撅細
```

  ptr -> [type_tag]*
      -> [const | volatile | restrict | typedef]*
      -> base_type

```
鍩烘湰涓婏紝涓€涓寚閽堢被鍨嬫寚鍚戦浂涓垨澶氫釜 type_tag锛岀劧鍚庢槸闆朵釜鎴栧涓?
const/volatile/restrict/typedef锛屾渶鍚庢槸鍩虹被鍨嬨€傚熀绫诲瀷鏄?int銆乸tr銆?
array銆乻truct銆乽nion銆乪num銆乫unc_proto 鍜?float 绫诲瀷涔嬩竴銆?

涓?decl tag 绫讳技锛屽鏋?`info.kind_flag` 涓?0锛屽垯杩欐槸涓€涓櫘閫氱殑 type tag锛?
`name_off` 缂栫爜鐨勬槸 btf_type_tag 灞炴€у瓧绗︿覆銆?

濡傛灉 `info.kind_flag` 涓?1锛屽垯璇?type tag 琛ㄧず浠绘剰鐨?`__attribute__`锛?
`name_off` 缂栫爜鐨勬槸涓€涓唬琛ㄥ睘鎬ц鏄庣灞炴€у垪琛ㄧ殑瀛楃涓层€?

#### 2.2.19 BTF_KIND_ENUM64


`struct btf_type` 缂栫爜瑕佹眰锛?
  - `name_off`锛? 鎴栨寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `info.kind_flag`锛氭棤绗﹀彿涓?0锛屾湁绗﹀彿涓?1
  - `info.kind`锛欱TF_KIND_ENUM64
  - `info.vlen`锛歟num 鍊肩殑涓暟
  - `size`锛?/2/4/8

```

    struct btf_enum64 {
        __u32   name_off;
        __u32   val_lo32;
        __u32   val_hi32;
    };

```
`btf_enum64` 缂栫爜锛?
  - `name_off`锛氭寚鍚戜竴涓湁鏁?C 鏍囪瘑绗︾殑鍋忕Щ
  - `val_lo32`锛?4 浣嶅€肩殑浣?32 浣?
  - `val_hi32`锛?4 浣嶅€肩殑楂?32 浣?

濡傛灉鍘熷 enum 鍊兼槸鏈夌鍙风殑涓斿ぇ灏忓皬浜?8锛岃鍊间細琚鍙锋墿灞曚负 8 瀛楄妭銆?

### 2.3 甯搁噺鍊?


#### 2.3.1 鍑芥暟閾炬帴甯搁噺鍊?


  ===================  =====  ===========
  kind                 value  description
  ===================  =====  ===========
  `BTF_FUNC_STATIC`  0x0    瀛愮▼搴忓畾涔夛紝鍦ㄦ墍灞炵紪璇戝崟鍏冧箣澶栦笉鍙
  `BTF_FUNC_GLOBAL`  0x1    瀛愮▼搴忓畾涔夛紝鍦ㄦ墍灞炵紪璇戝崟鍏冧箣澶栧彲瑙?
  `BTF_FUNC_EXTERN`  0x2    瀛愮▼搴忓０鏄庯紝鍏跺畾涔夊湪鎵€灞炵紪璇戝崟鍏冧箣澶?
  ===================  =====  ===========

#### 2.3.2 鍙橀噺閾炬帴甯搁噺鍊?


  ============================  =====  ===========
  kind                          value  description
  ============================  =====  ===========
  `BTF_VAR_STATIC`            0x0    鍏ㄥ眬鍙橀噺瀹氫箟锛屽湪鎵€灞炵紪璇戝崟鍏冧箣澶栦笉鍙
  `BTF_VAR_GLOBAL_ALLOCATED`  0x1    鍏ㄥ眬鍙橀噺瀹氫箟锛屽湪鎵€灞炵紪璇戝崟鍏冧箣澶栧彲瑙?
  `BTF_VAR_GLOBAL_EXTERN`     0x2    鍏ㄥ眬鍙橀噺澹版槑锛屽叾瀹氫箟鍦ㄦ墍灞炵紪璇戝崟鍏冧箣澶?
  ============================  =====  ===========

## 3. BTF 鍐呮牳 API


浠ヤ笅 bpf 绯荤粺璋冪敤鍛戒护娑夊強 BTF锛?
   - BPF_BTF_LOAD锛氬皢涓€鍧?BTF 鏁版嵁鍔犺浇杩涘唴鏍?
   - BPF_MAP_CREATE锛氬垱寤哄甫 btf 閿拰鍊肩被鍨嬩俊鎭殑 map
   - BPF_PROG_LOAD锛氬甫 btf 鍑芥暟鍜岃鍙蜂俊鎭姞杞界▼搴?
   - BPF_BTF_GET_FD_BY_ID锛氳幏鍙栦竴涓?btf 鏂囦欢鎻忚堪绗︼紙fd锛?
   - BPF_OBJ_GET_INFO_BY_FD锛氳繑鍥?btf銆乫unc_info銆乴ine_info
     鍙婂叾浠?btf 鐩稿叧淇℃伅

鍏稿瀷鐨勫伐浣滄祦绋嬪涓嬶細
```

  Application:
      BPF_BTF_LOAD
          |
          v
      BPF_MAP_CREATE and BPF_PROG_LOAD
          |
          V
      ......

  Introspection tool:
      ......
      BPF_{PROG,MAP}_GET_NEXT_ID (鑾峰彇 prog/map 鐨?id)
          |
          V
      BPF_{PROG,MAP}_GET_FD_BY_ID (鑾峰彇涓€涓?prog/map 鐨?fd)
          |
          V
      BPF_OBJ_GET_INFO_BY_FD (鐢?btf_id 鑾峰彇 bpf_prog_info/bpf_map_info)
          |                                     |
          V                                     |
      BPF_BTF_GET_FD_BY_ID (鑾峰彇 btf_fd)         |
          |                                     |
          V                                     |
      BPF_OBJ_GET_INFO_BY_FD (鑾峰彇 btf)          |
          |                                     |
          V                                     V
      pretty print 绫诲瀷銆乨ump 鍑芥暟绛惧悕涓庤鍙蜂俊鎭瓑

```
### 3.1 BPF_BTF_LOAD


灏嗕竴鍧?BTF 鏁版嵁鍔犺浇杩涘唴鏍搞€備竴鍧楁暟鎹紙濡?BTF_Type_String 鎵€杩帮級鍙互鐩存帴鍔犺浇
杩涘唴鏍搞€備細鍚戠敤鎴风┖闂磋繑鍥炰竴涓?`btf_fd`銆?

### 3.2 BPF_MAP_CREATE


```

    __u32   btf_fd;         /* 鎸囧悜 BTF 绫诲瀷鏁版嵁鐨?fd */
    __u32   btf_key_type_id;        /* 閿殑 BTF type_id */
    __u32   btf_value_type_id;      /* 鍊肩殑 BTF type_id */

```
鍦?libbpf 涓紝鍙互鍍忎笅闈㈣繖鏍风敤棰濆娉ㄨВ鏉ュ畾涔?map锛?
```

    struct {
        __uint(type, BPF_MAP_TYPE_ARRAY);
        __type(key, int);
        __type(value, struct ipv_counts);
        __uint(max_entries, 4);
    } btf_map SEC(".maps");

```
鍦?ELF 瑙ｆ瀽鏈熼棿锛宭ibbpf 鑳藉鎶藉嚭閿?鍊?type_id 骞惰嚜鍔ㄨ祴鍊肩粰 BPF_MAP_CREATE
鐨勫睘鎬с€?


### 3.3 BPF_PROG_LOAD


鍦?prog_load 鏈熼棿锛屽彲浠ュ皢 func_info 鍜?line_info 杩炲悓浠ヤ笅灞炴€х殑鎭板綋鍙栧€?
浼犲叆鍐呮牳锛?
```

    __u32           insn_cnt;
    __aligned_u64   insns;
    ......
    __u32           prog_btf_fd;    /* 鎸囧悜 BTF 绫诲瀷鏁版嵁鐨?fd */
    __u32           func_info_rec_size;     /* 鐢ㄦ埛绌洪棿 bpf_func_info 澶у皬 */
    __aligned_u64   func_info;      /* func 淇℃伅 */
    __u32           func_info_cnt;  /* bpf_func_info 璁板綍鏁?*/
    __u32           line_info_rec_size;     /* 鐢ㄦ埛绌洪棿 bpf_line_info 澶у皬 */
    __aligned_u64   line_info;      /* line 淇℃伅 */
    __u32           line_info_cnt;  /* bpf_line_info 璁板綍鏁?*/

```
```

    struct bpf_func_info {
        __u32   insn_off; /* [0, insn_cnt - 1] */
        __u32   type_id;  /* 鎸囧悜涓€涓?BTF_KIND_FUNC 绫诲瀷 */
    };
    struct bpf_line_info {
        __u32   insn_off; /* [0, insn_cnt - 1] */
        __u32   file_name_off; /* 鎸囧悜鏂囦欢鍚嶇殑瀛楃涓茶〃鍋忕Щ */
        __u32   line_off; /* 鎸囧悜婧愮爜琛岀殑瀛楃涓茶〃鍋忕Щ */
        __u32   line_col; /* 琛屽彿涓庡垪鍙?*/
    };

```
func_info_rec_size 鏄瘡鏉?func_info 璁板綍鐨勫ぇ灏忥紝line_info_rec_size 鏄瘡鏉?
line_info 璁板綍鐨勫ぇ灏忋€傚皢璁板綍澶у皬浼犵粰鍐呮牳锛屼娇寰楀皢鏉ユ墿灞曡褰曟湰韬垚涓哄彲鑳姐€?

浠ヤ笅鏄?func_info 鐨勮姹傦細
  - func_info[^0^].insn_off 蹇呴』涓?0銆?
  - func_info 鐨?insn_off 蹇呴』涓ユ牸閫掑锛屽苟涓斾笌 bpf 鍑芥暟杈圭晫鍖归厤銆?

浠ヤ笅鏄?line_info 鐨勮姹傦細
  - 姣忎釜鍑芥暟涓殑绗竴鏉℃寚浠ゅ繀椤绘湁涓€鏉℃寚鍚戝畠鐨?line_info 璁板綍銆?
  - line_info 鐨?insn_off 蹇呴』涓ユ牸閫掑銆?

瀵逛簬 line_info锛岃鍙峰拰鍒楀彿瀹氫箟濡備笅锛?
```

    #define BPF_LINE_INFO_LINE_NUM(line_col)        ((line_col) >> 10)
    #define BPF_LINE_INFO_LINE_COL(line_col)        ((line_col) & 0x3ff)

```
### 3.4 BPF_{PROG,MAP}_GET_NEXT_ID


鍦ㄥ唴鏍镐腑锛屾瘡涓鍔犺浇鐨勭▼搴忋€乵ap 鎴?btf 閮芥湁涓€涓敮涓€ id銆傝 id 鍦ㄧ▼搴忋€乵ap
鎴?btf 鐨勭敓鍛藉懆鏈熷唴涓嶄細鏀瑰彉銆?

bpf 绯荤粺璋冪敤鍛戒护 BPF_{PROG,MAP}_GET_NEXT_ID 浼氬垎鍒繑鍥?bpf 绋嬪簭鎴?map 鐨勬墍鏈?
id锛堟瘡涓懡浠や竴涓級锛屼氦缁欑敤鎴风┖闂达紝浠ヤ究涓€涓唴鐪佸伐鍏峰彲浠ユ鏌ユ墍鏈夌殑绋嬪簭鍜?map銆?

### 3.5 BPF_{PROG,MAP}_GET_FD_BY_ID


鍐呯渷宸ュ叿鏃犳硶鐩存帴浣跨敤 id 鏉ヨ幏鍙栫▼搴忔垨 map 鐨勮缁嗕俊鎭€傞渶瑕佸厛鑾峰彇涓€涓枃浠?
鎻忚堪绗︼紝浠ヤ究杩涜寮曠敤璁℃暟銆?

### 3.6 BPF_OBJ_GET_INFO_BY_FD


涓€鏃︽嬁鍒扮▼搴?map 鐨?fd锛屽唴鐪佸伐鍏峰氨鍙互浠庡唴鏍歌幏鍙栧叧浜庤 fd 鐨勮缁嗕俊鎭紝鍏朵腑
涓€浜涗笌 BTF 鐩稿叧銆備緥濡傦紝`bpf_map_info` 杩斿洖 `btf_id` 浠ュ強閿?鍊肩被鍨?id銆?
`bpf_prog_info` 杩斿洖 `btf_id`銆乫unc_info锛屼互鍙婄炕璇戝悗鐨?bpf 瀛楄妭鐮佺殑 line info
鍜?jited_line_info銆?

### 3.7 BPF_BTF_GET_FD_BY_ID


鍊熷姪鍦?`bpf_map_info` 鍜?`bpf_prog_info` 涓幏鍙栫殑 `btf_id`锛宐pf 绯荤粺璋冪敤鍛戒护
BPF_BTF_GET_FD_BY_ID 鍙互鍙栧嚭涓€涓?btf fd銆傜劧鍚庯紝閫氳繃鍛戒护 BPF_OBJ_GET_INFO_BY_FD锛?
鍙互鎶婃渶鍒濈敤 BPF_BTF_LOAD 鍔犺浇杩涘唴鏍哥殑 btf 鏁版嵁鍧楀彇鍥炪€?

鎷ユ湁浜?btf 鏁版嵁鍧椼€乣bpf_map_info` 鍜?`bpf_prog_info`锛屽唴鐪佸伐鍏峰氨鎺屾彙浜嗗畬鏁寸殑
btf 鐭ヨ瘑锛岃兘澶熷弸濂芥墦鍗?map 鐨勯敭/鍊笺€乨ump 鍑芥暟绛惧悕涓庤鍙蜂俊鎭紝浠ュ強瀛楄妭鐮?JIT
浠ｇ爜銆?

## 4. ELF 鏂囦欢鏍煎紡鎺ュ彛


### 4.1 .BTF 娈?


.BTF 娈靛寘鍚被鍨嬪拰瀛楃涓叉暟鎹€傝娈电殑鏍煎紡涓?BTF_Type_String 涓弿杩扮殑鐩稿悓銆?


### 4.2 .BTF.ext 娈?


.BTF.ext 娈电紪鐮?func_info銆乴ine_info 浠ュ強 CO-RE 閲嶅畾浣嶄俊鎭紝杩欎簺鍐呭鍦ㄥ姞杞借繘
鍐呮牳涔嬪墠闇€瑕佸姞杞藉櫒杩涜澶勭悊銆?

.BTF.ext 娈电殑瑙勮寖瀹氫箟浜?`tools/lib/bpf/btf.h` 鍜?`tools/lib/bpf/btf.c`銆?

```

    struct btf_ext_header {
        __u16   magic;
        __u8    version;
        __u8    flags;
        __u32   hdr_len;

        /* 鎵€鏈夊亸绉婚噺閮戒互瀛楄妭涓哄崟浣嶏紝鐩稿浜庢湰澶撮儴鏈熬 */
        __u32   func_info_off;
        __u32   func_info_len;
        __u32   line_info_off;
        __u32   line_info_len;

        /* .BTF.ext 澶撮儴鐨勫彲閫夐儴鍒?*/
        __u32   core_relo_off;
        __u32   core_relo_len;
    };

```
瀹冧笌 .BTF 娈甸潪甯哥浉浼笺€傚畠涓嶅寘鍚被鍨?瀛楃涓叉锛岃€屾槸鍖呭惈 func_info銆乴ine_info
鍜?core_relo 瀛愭銆傚叧浜?func_info 鍜?line_info 璁板綍鏍煎紡鐨勮鎯咃紝瑙?BPF_Prog_Load銆?

```

     func_info_rec_size              /* __u32 鍊?*/
     btf_ext_info_sec for section #1 /* section #1 鐨?func_info */
     btf_ext_info_sec for section #2 /* section #2 鐨?func_info */
     ...

```
`func_info_rec_size` 鎸囧畾鐢熸垚 .BTF.ext 鏃?`bpf_func_info` 缁撴瀯鐨勫ぇ灏忋€?
`btf_ext_info_sec`锛堝畾涔夊涓嬶級鏄竴涓泦鍚堬細
```

     struct btf_ext_info_sec {
        __u32   sec_name_off; /* 娈靛悕鍋忕Щ */
        __u32   num_info;
        /* 绱ц窡鐫€ num_info * record_size 涓瓧鑺?*/
        __u8    data[0];
     };

```
姝ゅ num_info 蹇呴』澶т簬 0銆?

```

     line_info_rec_size              /* __u32 鍊?*/
     btf_ext_info_sec for section #1 /* section #1 鐨?line_info */
     btf_ext_info_sec for section #2 /* section #2 鐨?line_info */
     ...

```
`line_info_rec_size` 鎸囧畾鐢熸垚 .BTF.ext 鏃?`bpf_line_info` 缁撴瀯鐨勫ぇ灏忋€?

`bpf_func_info->insn_off` 鍜?`bpf_line_info->insn_off` 鍦ㄢ€滃唴鏍?API鈥濅笌鈥淓LF API鈥?
涓殑瑙ｉ噴涓嶅悓銆傚浜庡唴鏍?API锛宍insn_off` 鏄互 ``struct bpf_insn` 涓哄崟浣嶇殑鎸囦护
鍋忕Щ銆傚浜?ELF API锛宍insn_off` 鏄粠娈靛紑澶寸畻璧风殑瀛楄妭鍋忕Щ
锛坄btf_ext_info_sec->sec_name_off`锛夈€?

```

     core_relo_rec_size              /* __u32 鍊?*/
     btf_ext_info_sec for section #1 /* section #1 鐨?core_relo */
     btf_ext_info_sec for section #2 /* section #2 鐨?core_relo */

```
`core_relo_rec_size` 鎸囧畾鐢熸垚 .BTF.ext 鏃?`bpf_core_relo` 缁撴瀯鐨勫ぇ灏忋€傚崟涓?
`btf_ext_info_sec` 鍐呯殑鎵€鏈?`bpf_core_relo` 缁撴瀯鎻忚堪搴旂敤浜庣敱
`btf_ext_info_sec->sec_name_off` 鍛藉悕鐨勬涓婄殑閲嶅畾浣嶃€?

璇﹁ Documentation/bpf/llvm_reloc.rst <btf-co-re-relocations>
浜嗚В鍏充簬 CO-RE 閲嶅畾浣嶇殑鏇村淇℃伅銆?

### 4.3 .BTF_ids 娈?


.BTF_ids 娈电紪鐮佸唴鏍镐腑浣跨敤鐨?BTF ID 鍊笺€?

璇ユ鍦ㄥ唴鏍哥紪璇戞湡闂村€熷姪 `include/linux/btf_ids.h` 澶存枃浠朵腑瀹氫箟鐨勫畯鍒涘缓銆傚唴鏍?
浠ｇ爜鍙互鐢ㄥ畠浠潵鍒涘缓 BTF ID 鍊肩殑鍒楄〃鍜岄泦鍚堬紙鏈夊簭鍒楄〃锛夈€?

`BTF_ID_LIST` 鍜?`BTF_ID` 瀹忓畾涔夋棤搴忕殑 BTF ID 鍊煎垪琛紝
```

  BTF_ID_LIST(list)
  BTF_ID(type1, name1)
  BTF_ID(type2, name2)

```
```

  __BTF_ID__type1__name1__1:
  .zero 4
  __BTF_ID__type2__name2__2:
  .zero 4

```
瀹氫箟浜?`u32 list[];` 鍙橀噺鏉ヨ闂鍒楄〃銆?

`BTF_ID_UNUSED` 瀹忓畾涔?4 涓浂瀛楄妭銆傚綋鎴戜滑闇€瑕佸崰浣嶆椂浣跨敤瀹冿紝渚嬪
```

      BTF_ID_LIST(bpf_skb_output_btf_ids)
      BTF_ID(struct, sk_buff)
      BTF_ID_UNUSED
      BTF_ID(struct, task_struct)

```
`BTF_SET_START/END` 瀹忓瀹氫箟鏈夊簭鐨?BTF ID 鍊奸泦鍚?
```

  BTF_SET_START(set)
  BTF_ID(type1, name1)
  BTF_ID(type2, name2)
  BTF_SET_END(set)

```
```

  __BTF_ID__set__set:
  .zero 4
  __BTF_ID__type1__name1__3:
  .zero 4
  __BTF_ID__type2__name2__4:
  .zero 4

```
瀹氫箟浜?`struct btf_id_set set;` 鍙橀噺鏉ヨ闂鍒楄〃銆?

```

   struct, union, typedef, func

```
骞跺湪瑙ｆ瀽 BTF ID 鍊兼椂浣滀负杩囨护鍣ㄤ娇鐢ㄣ€?

鎵€鏈夌殑 BTF ID 鍒楄〃鍜岄泦鍚堥兘琚紪璇戣繘 .BTF_ids 娈碉紝骞跺湪鍐呮牳鏋勫缓鐨勯摼鎺ラ樁娈电敱
`resolve_btfids` 宸ュ叿瑙ｆ瀽銆?

### 4.4 .BTF.base 娈?


鎷嗗垎 BTF锛圫plit BTF锛夆€斺€斿叾涓?.BTF 娈靛彧鍖呭惈涓嶅湪鍏宠仈鐨勫熀纭€ .BTF 娈典腑鐨勭被鍨嬧€斺€旀槸
缂栫爜鍐呮牳妯″潡绫诲瀷淇℃伅鐨勪竴绉嶆瀬鍏堕珮鏁堢殑鏂瑰紡锛屽洜涓哄唴鏍告ā鍧楅€氬父鐢卞皯閲忔ā鍧椾笓灞炵被鍨?
鍔犱笂澶ч噺鍏变韩鐨勫唴鏍哥被鍨嬬粍鎴愩€傚墠鑰呯紪鐮佸湪鎷嗗垎 BTF 涓紝鑰屽悗鑰呯紪鐮佸湪鍩虹 BTF 涓紝
浠庤€屽緱鍒版洿绱у噾鐨勮〃绀恒€傛媶鍒?BTF 涓寚鍚戝熀纭€ BTF 涓煇涓被鍨嬬殑绫诲瀷锛屼娇鐢ㄥ叾鍩虹 BTF
ID 鏉ュ紩鐢ㄥ畠锛岃€屾媶鍒?BTF 鐨?ID 浠?last_base_BTF_ID + 1 寮€濮嬨€?

鐒惰€岃繖绉嶅仛娉曠殑缂虹偣鏄鎷嗗垎 BTF 鏈夌偣鑴嗗急鈥斺€斿綋鍩虹 BTF 鍙戠敓鍙樺寲鏃讹紝鍩虹 BTF ID
寮曠敤灏变笉鍐嶆湁鏁堬紝鎷嗗垎 BTF 鏈韩涔熷氨姣棤鐢ㄥ浜嗐€?BTF.base 娈电殑浣滅敤灏辨槸璁╂媶鍒?BTF
鍦ㄩ潰瀵瑰熀纭€ BTF 鍙兘鍙樺寲鐨勬儏鍐典笅鏇村叿闊ф€э紝鍐呮牳妯″潡骞堕潪姣忔閮介殢鍐呮牳涓€璧锋瀯寤虹殑
鎯呭舰姝ｆ槸濡傛銆?BTF.base 鍖呭惈鏈夊悕瀛楃殑鍩虹绫诲瀷锛欼NT銆丗LOAT銆丼TRUCT銆乁NION銆?
ENUM[^64^] 鍜?FWD銆侷NT 鍜?FLOAT 鍦?.BTF.base 娈典腑琚畬鏁存弿杩帮紝鑰屽儚 struct 鍜?
union 杩欐牱鐨勫鍚堢被鍨嬪垯鏈瀹屾暣瀹氫箟鈥斺€?BTF.base 绫诲瀷浠呬綔涓烘媶鍒?BTF 鎵€鎸囩被鍨嬬殑
鎻忚堪锛屽洜姝?struct/union 鍦?.BTF.base 娈典腑鏈?0 涓垚鍛樸€侲NUM[^64^] 鍚屾牱浠?0 涓?
鎴愬憳璁板綍銆備换浣曞叾浠栫被鍨嬮兘琚姞鍏ユ媶鍒?BTF銆傝繖涓€鈥滆捀棣忊€濊繃绋嬫渶缁堝緱鍒颁竴涓甫鏈夋绫?
鏈€灏忓寲鍩虹绫诲瀷鎻忚堪鐨?.BTF.base 娈碉紝浠ュ強涓€涓紩鐢ㄩ偅浜涘熀纭€绫诲瀷鐨?.BTF 鎷嗗垎娈点€備箣鍚庯紝
鎴戜滑鍙互缁撳悎 .BTF.base 娈典腑瀛樺偍鐨勪俊鎭拰鏂扮殑 .BTF 鍩虹娈垫潵瀵规媶鍒?BTF 杩涜閲嶅畾浣嶏紱
.BTF.base 娈典腑鐨勭被鍨嬩俊鎭鎴戜滑鑳藉鏇存柊鎷嗗垎 BTF 鐨勫紩鐢紝浣垮叾鎸囧悜瀵瑰簲鐨勬柊鍩虹 BTF
ID銆?

BTF 閲嶅畾浣嶅湪鍐呮牳妯″潡鍔犺浇鏃跺彂鐢燂紙褰撳唴鏍告ā鍧楀甫鏈?.BTF.base 娈垫椂锛夛紝libbpf 涔熸彁渚?
浜?btf__relocate() API 鏉ュ畬鎴愭浜嬨€?

```

      [1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
      [2] STRUCT 'foo' size=8 vlen=2
              'f1' type_id=1 bits_offset=0
              'f2' type_id=1 bits_offset=32

```
```

      [3] PTR '(anon)' type_id=2

```
鍗虫媶鍒?BTF 鎻忚堪浜嗕竴涓寚鍚?struct foo { int f1; int f2 }; 鐨勬寚閽?

```

      [1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
      [2] STRUCT 'foo' size=8 vlen=0

```
```

      [1] INT 'long unsigned int' size=8 bits_offset=0 nr_bits=64 encoding=(none)
      [2] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED
      [3] STRUCT 'foo' size=8 vlen=2
              'f1' type_id=2 bits_offset=0
              'f2' type_id=2 bits_offset=32

```
鈥︹€︽垜浠彲浠ュ埄鐢ㄦ垜浠殑 .BTF.base 鎻忚堪鏉ョ煡閬撹鎷嗗垎 BTF 寮曠敤
```

      [4] PTR '(anon)' type_id=3

```
娉ㄦ剰鎴戜滑涓嶅緱涓嶆洿鏂版媶鍒?BTF 鐨?BTF ID 鍜岃捣濮?BTF ID銆?

鐢辨鍙 .BTF.base 濡備綍璧峰埌淇冭繘鍚庣画閲嶅畾浣嶇殑浣滅敤锛屼粠鑰屽甫鏉ユ洿鍏烽煣鎬х殑鎷嗗垎 BTF銆?

.BTF.base 娈典細鍦ㄦ爲澶栵紙out-of-tree锛夊唴鏍告ā鍧楁瀯寤烘椂鑷姩鐢熸垚鈥斺€斿嵆璁剧疆浜?KBUILD_EXTMOD
鐨勬儏褰紙灏卞儚 "make M=path/2/mod" 閭ｆ牱锛夈€?BTF.base 鐨勭敓鎴愰渶瑕?pahole 瀵?
"distilled_base" BTF 鐗规€х殑鏀寔锛涜繖鍦?pahole v1.28 鍙婃洿楂樼増鏈腑鍙敤銆?

## 5. 浣跨敤 BTF


### 5.1 bpftool map 鍙嬪ソ鎵撳嵃


鍊熷姪 BTF锛宮ap 鐨勯敭/鍊煎彲浠ュ熀浜庡瓧娈垫墦鍗帮紝鑰岄潪绠€鍗曞湴鎸夊師濮嬪瓧鑺傛墦鍗般€傝繖瀵逛簬澶у瀷
缁撴瀯浣撴垨鑰呭綋浣犵殑鏁版嵁
```

      enum A { A1, A2, A3, A4, A5 };
      typedef enum A ___A;
      struct tmp_t {
           char a1:4;
           int  a2:4;
           int  :4;
           __u32 a3:4;
           int b;
           ___A b1:4;
           enum A b2:4;
      };
      struct {
           __uint(type, BPF_MAP_TYPE_ARRAY);
           __type(key, int);
           __type(value, struct tmp_t);
           __uint(max_entries, 1);
      } tmpmap SEC(".maps");

```
bpftool 鑳藉鍍忎笅闈㈣繖鏍峰弸濂芥墦鍗帮細
```

      [{
            "key": 0,
            "value": {
                "a1": 0x2,
                "a2": 0x4,
                "a3": 0x6,
                "b": 7,
                "b1": 0x8,
                "b2": 0xa
            }
        }
      ]

```
### 5.2 bpftool prog dump


涓嬮潰鏄竴涓ず渚嬶紝灞曠ず func_info 鍜?line_info 濡備綍鍊熷姪鏇村ソ鐨勫唴鏍哥鍙峰悕銆佸嚱鏁板師鍨?
鍜岃鍙蜂俊鎭潵甯姪 prog dump
```

    $ bpftool prog dump jited pinned /sys/fs/bpf/test_btf_haskv
    [...]
    int test_long_fname_2(struct dummy_tracepoint_args * arg):
    bpf_prog_44a040bf25481309_test_long_fname_2:
    ; static int test_long_fname_2(struct dummy_tracepoint_args *arg)
       0:   push   %rbp
       1:   mov    %rsp,%rbp
       4:   sub    $0x30,%rsp
       b:   sub    $0x28,%rbp
       f:   mov    %rbx,0x0(%rbp)
      13:   mov    %r13,0x8(%rbp)
      17:   mov    %r14,0x10(%rbp)
      1b:   mov    %r15,0x18(%rbp)
      1f:   xor    %eax,%eax
      21:   mov    %rax,0x20(%rbp)
      25:   xor    %esi,%esi
    ; int key = 0;
      27:   mov    %esi,-0x4(%rbp)
    ; if (!arg->sock)
      2a:   mov    0x8(%rdi),%rdi
    ; if (!arg->sock)
      2e:   cmp    $0x0,%rdi
      32:   je     0x0000000000000070
      34:   mov    %rbp,%rsi
    ; counts = bpf_map_lookup_elem(&btf_map, &key);
    [...]

```
### 5.3 楠岃瘉鍣ㄦ棩蹇?


涓嬮潰鏄竴涓ず渚嬶紝灞曠ず line_info 濡備綍甯姪璋冭瘯楠岃瘉杩囩▼
```

       /* tools/testing/selftests/bpf/test_xdp_noinline.c 涓殑浠ｇ爜
        * 琚慨鏀瑰涓嬨€?
        */
       data = (void *)(long)xdp->data;
       data_end = (void *)(long)xdp->data_end;
       /*
       if (data + 4 > data_end)
               return XDP_DROP;
       */
       *(u32 *)data = dst->dst;

    $ bpftool prog load ./test_xdp_noinline.o /sys/fs/bpf/test_xdp_noinline type xdp
        ; data = (void *)(long)xdp->data;
        224: (79) r2 = *(u64 *)(r10 -112)
        225: (61) r2 = *(u32 *)(r2 +0)
        ; *(u32 *)data = dst->dst;
        226: (63) *(u32 *)(r2 +0) = r1
        invalid access to packet, off=0 size=4, R2(id=0,off=0,r=0)
        R2 offset is outside of the packet

```
## 6. BTF 鐢熸垚


浣犻渶瑕佹渶鏂扮増鏈殑 pahole

  https://git.kernel.org/pub/scm/devel/pahole/pahole.git/

鎴?llvm锛?.0 鎴栨洿楂樼増鏈級銆俻ahole 鍏呭綋 dwarf2btf 杞崲鍣ㄣ€傚畠涓?
```

      -bash-4.4$ cat t.c
      struct t {
        int a:2;
        int b:3;
        int c:2;
      } g;
      -bash-4.4$ gcc -c -O2 -g t.c
      -bash-4.4$ pahole -JV t.o
      File t.o:
      [1] STRUCT t kind_flag=1 size=4 vlen=3
              a type_id=2 bitfield_size=2 bits_offset=0
              b type_id=2 bitfield_size=3 bits_offset=2
              c type_id=2 bitfield_size=2 bits_offset=5
      [2] INT int size=4 bit_offset=0 nr_bits=32 encoding=SIGNED

```
llvm 鑳藉鐩存帴鐢?-g 涓?bpf 鐩爣鐢熸垚 .BTF 鍜?.BTF.ext锛堜粎闄?bpf 鐩爣锛夈€傛眹缂栦唬鐮?
锛?S锛夎兘澶熷睍绀?BTF 鍦ㄦ眹缂栦腑鐨勭紪鐮?
```

    -bash-4.4$ cat t2.c
    typedef int __int32;
    struct t2 {
      int a2;
      int (*f2)(char q1, __int32 q2, ...);
      int (*f3)();
    } g2;
    int main() { return 0; }
    int test() { return 0; }
    -bash-4.4$ clang -c -g -O2 --target=bpf t2.c
    -bash-4.4$ readelf -S t2.o
      ......
      [ 8] .BTF              PROGBITS         0000000000000000  00000247
           000000000000016e  0000000000000000           0     0     1
      [ 9] .BTF.ext          PROGBITS         0000000000000000  000003b5
           0000000000000060  0000000000000000           0     0     1
      [10] .rel.BTF.ext      REL              0000000000000000  000007e0
           0000000000000040  0000000000000010          16     9     8
      ......
    -bash-4.4$ clang -S -g -O2 --target=bpf t2.c
    -bash-4.4$ cat t2.s
      ......
            .section        .BTF,"",@progbits
            .short  60319                   # 0xeb9f
            .byte   1
            .byte   0
            .long   24
            .long   0
            .long   220
            .long   220
            .long   122
            .long   0                       # BTF_KIND_FUNC_PROTO(id = 1)
            .long   218103808               # 0xd000000
            .long   2
            .long   83                      # BTF_KIND_INT(id = 2)
            .long   16777216                # 0x1000000
            .long   4
            .long   16777248                # 0x1000020
      ......
            .byte   0                       # string offset=0
            .ascii  ".text"                 # string offset=1
            .byte   0
            .ascii  "/home/yhs/tmp-pahole/t2.c" # string offset=7
            .byte   0
            .ascii  "int main() { return 0; }" # string offset=33
            .byte   0
            .ascii  "int test() { return 0; }" # string offset=58
            .byte   0
            .ascii  "int"                   # string offset=83
      ......
            .section        .BTF.ext,"",@progbits
            .short  60319                   # 0xeb9f
            .byte   1
            .byte   0
            .long   24
            .long   0
            .long   28
            .long   28
            .long   44
            .long   8                       # FuncInfo
            .long   1                       # FuncInfo section string offset=1
            .long   2
            .long   .Lfunc_begin0
            .long   3
            .long   .Lfunc_begin1
            .long   5
            .long   16                      # LineInfo
            .long   1                       # LineInfo section string offset=1
            .long   2
            .long   .Ltmp0
            .long   7
            .long   33
            .long   7182                    # Line 7 Col 14
            .long   .Ltmp3
            .long   7
            .long   58
            .long   8206                    # Line 8 Col 14

```
## 7. 娴嬭瘯


鍐呮牳 BPF 鑷祴璇?`tools/testing/selftests/bpf/prog_tests/btf.c`_
鎻愪緵浜嗕竴濂楀箍娉涚殑 BTF 鐩稿叧娴嬭瘯銆?

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/prog_tests/btf.c
