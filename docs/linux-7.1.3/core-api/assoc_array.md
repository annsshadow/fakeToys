## 閫氱敤鍏宠仈鏁扮粍瀹炵幇


## 姒傝堪


璇ュ叧鑱旀暟缁勫疄鐜版槸涓€涓璞″鍣紝鍏锋湁浠ヤ笅鐗规€э細

1. 瀵硅薄鏄竴浜涗笉閫忔槑鎸囬拡銆傝瀹炵幇骞朵笉鍏冲績瀹冧滑鎸囧悜浣曞锛堝鏋滄湁鎸囧悜鐨勮瘽锛夋垨鎸囧悜浠€涔堬紙濡傛灉鏈夋寚鍚戝唴瀹圭殑璇濓級銆?
```

      Pointers to objects _must_ be zero in the least significant bit.

```
2. 瀵硅薄鏃犻渶鍖呭惈渚涙暟缁勪娇鐢ㄧ殑閾炬帴鍧椼€傝繖浣垮緱涓€涓璞″彲浠ュ悓鏃跺瓨鍦ㄤ簬澶氫釜鏁扮粍涓€傜浉鍙嶏紝璇ユ暟缁勭敱鎸囧悜瀵硅薄鐨勫厓鏁版嵁鍧楃粍鎴愩€?
3. 瀵硅薄闇€瑕佺储寮曢敭鏉ュ湪鏁扮粍鍐呭畾浣嶃€?
4. 绱㈠紩閿繀椤诲敮涓€銆傛彃鍏ヤ竴涓笌鏁扮粍涓凡鏈夊璞￠敭鐩稿悓鐨勫璞℃椂锛屽皢鏇挎崲鏃у璞°€?
5. 绱㈠紩閿彲浠ユ槸浠绘剰闀垮害锛屼篃鍙互鍚勪笉鐩稿悓銆?
6. 绱㈠紩閿簲鍦ㄥ紑澶村敖鏃╃紪鐮佸叾闀垮害锛屽湪浠讳綍鍥犻暱搴﹂€犳垚鐨勫樊寮傚嚭鐜颁箣鍓嶃€?
7. 绱㈠紩閿彲浠ュ寘鍚搱甯屽€硷紝浠ヤ究灏嗗璞″垎鏁ｅ埌鏁翠釜鏁扮粍涓€?
8. 璇ユ暟缁勫彲浠ラ亶鍘嗐€傚璞＄殑杈撳嚭椤哄簭涓嶄竴瀹氫笌閿簭涓€鑷淬€?
9. 鍙杩唬鍣ㄦ寔鏈?RCU 璇婚攣锛屽氨鍙互鍦ㄦ暟缁勮淇敼鐨勫悓鏃堕亶鍘嗗畠銆備絾璇锋敞鎰忥紝鍦ㄨ繖绉嶆儏鍐典笅锛屾煇浜涘璞″彲鑳戒細琚湅鍒板娆°€傚鏋滆繖鏄釜闂锛岃凯浠ｅ櫒搴斿綋鍔犻攣浠ラ樆姝慨鏀广€備笉杩囷紝闄ら潪瀵硅薄琚垹闄わ紝鍚﹀垯涓嶄細琚仐婕忋€?
10. 鏁扮粍涓殑瀵硅薄鍙互閫氳繃鍏剁储寮曢敭杩涜鏌ユ壘銆?
11. 鍙鎵ц鏌ユ壘鐨勭嚎绋嬫寔鏈?RCU 璇婚攣锛屽氨鍙互鍦ㄦ暟缁勮淇敼鐨勫悓鏃舵煡鎵惧璞°€?
璇ュ疄鐜板湪鍐呴儴浣跨敤涓€妫电敱 16 鎸囬拡鑺傜偣缁勬垚鐨勬爲锛屾瘡涓€灞傞兘鍍忓熀鏁版爲锛坮adix tree锛夐偅鏍风敱绱㈠紩閿腑鐨勫崐瀛楄妭锛坣ibble锛夎繘琛岀储寮曘€備负浜嗘彁楂樺唴瀛樻晥鐜囷紝鍙互鎻掑叆蹇嵎鏂瑰紡锛坰hortcut锛変互璺宠繃鍘熸湰浼氭槸涓€绯诲垪鍗曞崰鐢紙single-occupancy锛夎妭鐐圭殑閮ㄥ垎銆傛澶栵紝鑺傜偣浼氬皢鍙跺瓙瀵硅薄鎸囬拡鎵撳寘杩涜妭鐐圭殑绌洪棽绌洪棿涓紝鑰屼笉鏄珛鍗抽澶栧紑鍒嗘敮锛岀洿鍒扮‘瀹為渶瑕佸悜涓€涓凡婊¤妭鐐规坊鍔犲璞℃椂涓烘銆?

## 鍏叡 API


鍏叡 API 鍙互鍦?`<linux/assoc_array.h>` 涓壘鍒般€傝鍏宠仈

```
      struct assoc_array {
              ...
      };

```
```
      ./script/config -e ASSOCIATIVE_ARRAY


```
### 缂栬緫鑴氭湰


鎻掑叆鍜屽垹闄ゅ嚱鏁颁細浜х敓涓€涓€滅紪杈戣剼鏈€濓紙edit script锛夛紝绋嶅悗鍙搴旂敤浠ュ疄鏂藉彉鏇达紝鑰屼笉浼氭湁 `ENOMEM` 椋庨櫓銆傝繖浼氫繚鐣欏皢瀹夎鍒板唴閮ㄦ爲涓殑棰勫垎閰嶅厓鏁版嵁鍧楋紝骞惰窡韪湪搴旂敤鑴氭湰鏃跺皢浠庢爲涓Щ闄ょ殑鍏冩暟鎹潡銆?
杩欎篃鐢ㄤ簬鍦ㄨ剼鏈簲鐢ㄤ箣鍚庤窡韪け鏁堬紙dead锛夊潡鍜屽け鏁堝璞★紝浠ヤ究瀹冧滑绋嶅悗琚噴鏀俱€傞噴鏀炬槸鍦ㄧ粡杩囦竴涓?RCU 瀹介檺鏈熶箣鍚庤繘琛岀殑鈥斺€斾粠鑰屽厑璁歌闂嚱鏁板湪 RCU 璇婚攣涓嬬户缁墽琛屻€?
```

    struct assoc_array_edit;

```
鏈変袱涓嚱鏁扮敤浜庡鐞嗚鑴氭湰锛?
```

    void assoc_array_apply_edit(struct assoc_array_edit *edit);

   This will perform the edit functions, interpolating various write barriers
   to permit accesses under the RCU read lock to continue.  The edit script
   will then be passed to ``call_rcu()`` to free it and any dead stuff it
   points to.

```
```

    void assoc_array_cancel_edit(struct assoc_array_edit *edit);

   This frees the edit script and all preallocated memory immediately. If
   this was for insertion, the new object is *not* released by this function,
   but must rather be released by the caller.

```
杩欎簺鍑芥暟鏄繚璇佷笉浼氬け璐ョ殑銆?

### 鎿嶄綔琛?

```

    struct assoc_array_ops {
            ...
    };

```
杩欐寚鍚戣嫢骞叉柟娉曪紝瀹冧滑閮介渶瑕佽鎻愪緵锛?
```

    unsigned long (*get_key_chunk)(const void *index_key, int level);

   This should return a chunk of caller-supplied index key starting at the
   *bit* position given by the level argument.  The level argument will be a
   multiple of ``ASSOC_ARRAY_KEY_CHUNK_SIZE`` and the function should return
   ``ASSOC_ARRAY_KEY_CHUNK_SIZE bits``.  No error is possible.


```
```

    unsigned long (*get_object_key_chunk)(const void *object, int level);

   As the previous function, but gets its data from an object in the array
   rather than from a caller-supplied index key.


```
```

    bool (*compare_object)(const void *object, const void *index_key);

   Compare the object against an index key and return ``true`` if it matches
   and ``false`` if it doesn't.


```
```

    int (*diff_objects)(const void *object, const void *index_key);

   Return the bit position at which the index key of the specified object
   differs from the given index key or -1 if they are the same.


```
```

    void (*free_object)(void *object);

   Free the specified object.  Note that this may be called an RCU grace period
   after ``assoc_array_apply_edit()`` was called, so ``synchronize_rcu()`` may
   be necessary on module unloading.


```
### 鎿嶄綔鍑芥暟


鏈夎澶氬嚱鏁扮敤浜庢搷浣滃叧鑱旀暟缁勶細

```

    void assoc_array_init(struct assoc_array *array);

   This initialises the base structure for an associative array.  It can't fail.


```
```

    struct assoc_array_edit *
    assoc_array_insert(struct assoc_array *array,
                       const struct assoc_array_ops *ops,
                       const void *index_key,
                       void *object);

   This inserts the given object into the array.  Note that the least
   significant bit of the pointer must be zero as it's used to type-mark
   pointers internally.

   If an object already exists for that key then it will be replaced with the
   new object and the old one will be freed automatically.

   The ``index_key`` argument should hold index key information and is
   passed to the methods in the ops table when they are called.

   This function makes no alteration to the array itself, but rather returns
   an edit script that must be applied.  ``-ENOMEM`` is returned in the case of
   an out-of-memory error.

   The caller should lock exclusively against other modifiers of the array.


```
```

    struct assoc_array_edit *
    assoc_array_delete(struct assoc_array *array,
                       const struct assoc_array_ops *ops,
                       const void *index_key);

   This deletes an object that matches the specified data from the array.

   The ``index_key`` argument should hold index key information and is
   passed to the methods in the ops table when they are called.

   This function makes no alteration to the array itself, but rather returns
   an edit script that must be applied.  ``-ENOMEM`` is returned in the case of
   an out-of-memory error.  ``NULL`` will be returned if the specified object
   is not found within the array.

```
璋冪敤鑰呭簲褰撻拡瀵规暟缁勭殑鍏朵粬淇敼鑰呭姞鎺掍粬閿併€?

```

    struct assoc_array_edit *
    assoc_array_clear(struct assoc_array *array,
                      const struct assoc_array_ops *ops);

   This deletes all the objects from an associative array and leaves it
   completely empty.

   This function makes no alteration to the array itself, but rather returns
   an edit script that must be applied.  ``-ENOMEM`` is returned in the case of
   an out-of-memory error.

   The caller should lock exclusively against other modifiers of the array.


```
```

    void assoc_array_destroy(struct assoc_array *array,
                             const struct assoc_array_ops *ops);

   This destroys the contents of the associative array and leaves it
   completely empty.  It is not permitted for another thread to be traversing
   the array under the RCU read lock at the same time as this function is
   destroying it as no RCU deferral is performed on memory release -
   something that would require memory to be allocated.

   The caller should lock exclusively against other modifiers and accessors
   of the array.


```
```

    int assoc_array_gc(struct assoc_array *array,
                       const struct assoc_array_ops *ops,
                       bool (*iterator)(void *object, void *iterator_data),
                       void *iterator_data);

   This iterates over the objects in an associative array and passes each one
   to ``iterator()``.  If ``iterator()`` returns ``true``, the object is kept.
   If it returns ``false``, the object will be freed.  If the ``iterator()``
   function returns ``true``, it must perform any appropriate refcount
   incrementing on the object before returning.

   The internal tree will be packed down if possible as part of the iteration
   to reduce the number of nodes in it.

   The ``iterator_data`` is passed directly to ``iterator()`` and is otherwise
   ignored by the function.

   The function will return ``0`` if successful and ``-ENOMEM`` if there wasn't
   enough memory.

   It is possible for other threads to iterate over or search the array under
   the RCU read lock while this function is in progress.  The caller should
   lock exclusively against other modifiers of the array.


```
### 璁块棶鍑芥暟


鏈変袱涓嚱鏁扮敤浜庤闂叧鑱旀暟缁勶細

```

    int assoc_array_iterate(const struct assoc_array *array,
                            int (*iterator)(const void *object,
                                            void *iterator_data),
                            void *iterator_data);

   This passes each object in the array to the iterator callback function.
   ``iterator_data`` is private data for that function.

   This may be used on an array at the same time as the array is being
   modified, provided the RCU read lock is held.  Under such circumstances,
   it is possible for the iteration function to see some objects twice.  If
   this is a problem, then modification should be locked against.  The
   iteration algorithm should not, however, miss any objects.

   The function will return ``0`` if no objects were in the array or else it
   will return the result of the last iterator function called.  Iteration
   stops immediately if any call to the iteration function results in a
   non-zero return.


```
```

    void *assoc_array_find(const struct assoc_array *array,
                           const struct assoc_array_ops *ops,
                           const void *index_key);

   This walks through the array's internal tree directly to the object
   specified by the index key.

   This may be used on an array at the same time as the array is being
   modified, provided the RCU read lock is held.

   The function will return the object if found (and set ``*_type`` to the
   object type) or will return ``NULL`` if the object was not found.


```
### 绱㈠紩閿舰寮?

绱㈠紩閿彲浠ラ噰鐢ㄤ换浣曞舰寮忥紝浣嗙敱浜庣畻娉曞苟涓嶇煡閬撻敭鏈夊闀匡紝寮虹儓寤鸿绱㈠紩閿湪寮€澶村敖鏃╁寘鍚叾闀垮害锛屽湪浠讳綍鍥犻暱搴﹂€犳垚鐨勫樊寮備細瀵规瘮杈冧骇鐢熷奖鍝嶄箣鍓嶃€?
杩欎細浣垮叿鏈変笉鍚岄暱搴﹂敭鐨勫彾瀛愬郊姝ゅ垎鏁ｅ紑锛岃€屽叿鏈夌浉鍚岄暱搴﹂敭鐨勫彾瀛愯仛闆嗗湪涓€璧枫€?
杩樺缓璁储寮曢敭浠ュ叾浣欓儴鍒嗙殑鍝堝笇寮€澶达紝浠ユ渶澶у寲鍦ㄦ暣涓敭绌洪棿涓殑鍒嗘暎绋嬪害銆?
鍒嗘暎瓒婂ソ锛屽唴閮ㄦ爲灏辫秺瀹姐€佽秺鐭€?
鍒嗘暎涓嶄匠涔熶笉鏄お澶х殑闂锛屽洜涓哄瓨鍦ㄥ揩鎹锋柟寮忥紝涓旇妭鐐瑰彲浠ュ寘鍚彾瀛愪笌鍏冩暟鎹寚閽堢殑娣峰悎銆?
绱㈠紩閿互鏈哄櫒瀛椾负鍗曚綅璇诲彇銆傛瘡涓瓧琚粏鍒嗕负姣忓眰涓€涓崐瀛楄妭锛? 浣嶏級锛屽洜姝ゅ湪 32 浣?CPU 涓婅繖閫傜敤浜?8 灞傦紝鍦?64 浣?CPU 涓婇€傜敤浜?16 灞傘€傞櫎闈炲垎鏁ｇ‘瀹炲緢宸紝鍚﹀垯涓嶅お鍙兘闇€瑕佺敤鍒版煇涓壒瀹氱储寮曢敭鐨勫浜庝竴涓瓧銆?

## 鍐呴儴宸ヤ綔鏈哄埗


鍏宠仈鏁扮粍鏁版嵁缁撴瀯鍏锋湁涓€妫靛唴閮ㄦ爲銆傝繖妫垫爲鐢变袱绫诲厓鏁版嵁鍧楁瀯鎴愶細鑺傜偣锛坣ode锛夊拰蹇嵎鏂瑰紡锛坰hortcut锛夈€?
鑺傜偣鏄竴涓Ы浣嶏紙slot锛夋暟缁勩€傛瘡涓Ы浣嶅彲浠ュ寘鍚互涓嬪洓绫诲唴瀹逛箣涓€锛?
- 涓€涓?NULL 鎸囬拡锛岃〃绀鸿妲戒綅涓虹┖銆?- 涓€涓寚鍚戝璞★紙鍙跺瓙锛夌殑鎸囬拡銆?- 涓€涓寚鍚戜笅涓€灞傝妭鐐圭殑鎸囬拡銆?- 涓€涓寚鍚戝揩鎹锋柟寮忕殑鎸囬拡銆?

### 鍩烘湰鍐呴儴鏍戝竷灞€


鏆傛椂蹇界暐蹇嵎鏂瑰紡锛岃妭鐐规瀯鎴愪竴妫靛灞傛爲銆傜储寮曢敭绌洪棿琚爲涓殑鑺傜偣涓ユ牸缁嗗垎锛岃妭鐐瑰嚭鐜板湪

```

 Level: 0               1               2               3
        =============== =============== =============== ===============
                                                        NODE D
                        NODE B          NODE C  +------>+---+
                +------>+---+   +------>+---+   |       | 0 |
        NODE A  |       | 0 |   |       | 0 |   |       +---+
        +---+   |       +---+   |       +---+   |       :   :
        | 0 |   |       :   :   |       :   :   |       +---+
        +---+   |       +---+   |       +---+   |       | f |
        | 1 |---+       | 3 |---+       | 7 |---+       +---+
        +---+           +---+           +---+
        :   :           :   :           | 8 |---+
        +---+           +---+           +---+   |       NODE E
        | e |---+       | f |           :   :   +------>+---+
        +---+   |       +---+           +---+           | 0 |
        | f |   |                       | f |           +---+
        +---+   |                       +---+           :   :
                |       NODE F                          +---+
                +------>+---+                           | f |
                        | 0 |           NODE G          +---+
                        +---+   +------>+---+
                        :   :   |       | 0 |
                        +---+   |       +---+
                        | 6 |---+       :   :
                        +---+           +---+
                        :   :           | f |
                        +---+           +---+
                        | f |
                        +---+

```
鍦ㄤ笂杩扮ず渚嬩腑锛屾湁 7 涓妭鐐癸紙A-G锛夛紝姣忎釜鏈?16 涓Ы浣嶏紙0-f锛夈€傚亣璁炬爲涓病鏈夊叾浠栧厓鏁版嵁鑺傜偣锛岄敭绌洪棿鍒掑垎濡備笅锛?
    ===========     ====
    KEY PREFIX      NODE
    ===========     ====
    137*            D
    138*            E
    13[0-69-f]*     C
    1[0-24-f]*      B
    e6*             G
    e[0-57-f]*      F
    [02-df]*        A
    ===========     ====

鍥犳锛屼緥濡傦紝鍏锋湁浠ヤ笅绀轰緥绱㈠紩閿殑閿皢鍑虹幇鍦ㄧ浉搴旂殑鑺傜偣涓細

    =============== ======= ====
    INDEX KEY       PREFIX  NODE
    =============== ======= ====
    13694892892489  13      C
    13795289025897  137     D
    13889dde88793   138     E
    138bbb89003093  138     E
    1394879524789   12      C
    1458952489      1       B
    9431809de993ba  \-      A
    b4542910809cd   \-      A
    e5284310def98   e       F
    e68428974237    e6      G
    e7fffcbd443     e       F
    f3842239082     \-      A
    =============== ======= ====

涓轰簡鑺傜渷鍐呭瓨锛屽鏋滀竴涓妭鐐硅兘澶熷绾冲叾閿┖闂撮儴鍒嗗唴鐨勬墍鏈夊彾瀛愶紝閭ｄ箞璇ヨ妭鐐瑰皢鍖呭惈鎵€鏈夎繖浜涘彾瀛愶紝骞朵笖涓嶄細鏈変换浣曞厓鏁版嵁鎸囬拡鈥斺€斿嵆浣垮叾涓煇浜涘彾瀛愭湰搴斾綅浜庡悓涓€涓Ы浣嶄腑銆?
涓€涓妭鐐瑰彲浠ュ寘鍚彾瀛愪笌鍏冩暟鎹寚閽堢殑寮傛瀯娣峰悎銆傚厓鏁版嵁鎸囬拡蹇呴』浣嶄簬涓庡叾閿┖闂寸粏鍒嗙浉鍖归厤鐨勬Ы浣嶄腑銆傚彾瀛愬彲浠ヤ綅浜庝换浣曟湭琚厓鏁版嵁鎸囬拡鍗犵敤鐨勬Ы浣嶄腑銆備繚璇佽妭鐐逛腑娌℃湁鍙跺瓙浼氫笌鍏冩暟鎹寚閽堝崰鐢ㄧ殑妲戒綅鍖归厤銆傚鏋滃厓鏁版嵁鎸囬拡瀛樺湪锛岄偅涔堜换浣曢敭涓庡厓鏁版嵁閿墠缂€鍖归厤鐨勫璞″彾瀛愰兘蹇呴』浣嶄簬璇ュ厓鏁版嵁鎸囬拡鎵€鎸囧悜鐨勫瓙鏍戜腑銆?
鍦ㄤ笂杩扮储寮曢敭鍒楄〃绀轰緥涓紝鑺傜偣 A 灏嗗寘鍚細

    ====    =============== ==================
    SLOT    CONTENT         INDEX KEY (PREFIX)
    ====    =============== ==================
    1       PTR TO NODE B   1*
    any     LEAF            9431809de993ba
    any     LEAF            b4542910809cd
    e       PTR TO NODE F   e*
    any     LEAF            f3842239082
    ====    =============== ==================

浠ュ強鑺傜偣 B锛?
    ====    =============== ==================
    SLOT    CONTENT         INDEX KEY (PREFIX)
    ====    =============== ==================
    3       PTR TO NODE C   13*
    any     LEAF            1458952489
    ====    =============== ==================


### 蹇嵎鏂瑰紡


蹇嵎鏂瑰紡鏄烦杩囦竴娈甸敭绌洪棿鐨勫厓鏁版嵁璁板綍銆傚揩鎹锋柟寮忔槸涓€绯诲垪椤虹潃灞傜骇涓婂崌鐨勫崟鍗犵敤鑺傜偣鐨勬浛浠ｃ€傚揩鎹锋柟寮忕殑瀛樺湪鏄负浜嗚妭鐪佸唴瀛樺苟鍔犻€熼亶鍘嗐€?
鏍戠殑鏍硅妭鐐规湁鍙兘鏄竴涓揩鎹锋柟寮忊€斺€斾緥濡傦紝鍋囪鏍戜腑鍖呭惈鑷冲皯 17 涓敭鍓嶇紑鍧囦负 `1111` 鐨勮妭鐐广€傛彃鍏ョ畻娉曞皢鎻掑叆涓€涓揩鎹锋柟寮忥紝涓€娆℃€ц烦杩?`1111` 閿┖闂达紝鐩磋揪杩欎簺鑺傜偣瀹為檯浜х敓宸紓鐨勭鍥涘眰銆?

### 鎷嗗垎涓庡悎骞惰妭鐐?

姣忎釜鑺傜偣鐨勬渶澶у閲忎负 16 涓彾瀛愬拰鍏冩暟鎹寚閽堛€傚鏋滄彃鍏ョ畻娉曞彂鐜板畠姝ｈ瘯鍥惧悜涓€涓妭鐐逛腑鎻掑叆绗?17 涓璞★紝閭ｄ箞璇ヨ妭鐐瑰皢琚媶鍒嗭紝浣垮緱鑷冲皯涓や釜鍦ㄨ灞傚叿鏈夊叡鍚岄敭娈碉紙key segment锛夌殑鍙跺瓙鏈€缁堣繘鍏ヤ竴涓互璇ュ叡鍚岄敭娈垫Ы浣嶄负鏍圭殑鏂拌妭鐐广€?
濡傛灉宸叉弧鑺傜偣涓殑鍙跺瓙浠ュ強姝ｅ湪鎻掑叆鐨勫彾瀛愯冻澶熺浉浼硷紝閭ｄ箞灏嗗湪鏍戜腑鎻掑叆涓€涓揩鎹锋柟寮忋€?
褰撲互鏌愪釜鑺傜偣涓烘牴鐨勫瓙鏍戜腑瀵硅薄鏁伴噺闄嶈嚦 16 涓垨鏇村皯鏃讹紝璇ュ瓙鏍戝皢琚悎骞讹紙collapse锛変负鍗曚釜鑺傜偣鈥斺€斿苟涓斿鏋滃彲鑳斤紝杩欎細鍚戞牴鑺傜偣鏂瑰悜浼犳挱銆?

### 闈為€掑綊杩唬


姣忎釜鑺傜偣鍜屽揩鎹锋柟寮忛兘鍖呭惈涓€涓寚鍚戝叾鐖惰妭鐐圭殑鍙嶅悜鎸囬拡锛屼互鍙婂湪鐖惰妭鐐逛腑鎸囧悜瀹冪殑妲戒綅鍙枫€傞潪閫掑綊杩唬鍒╃敤杩欎簺鎸囬拡鑷簳鍚戜笂锛坮ootwards锛夊湴閬嶅巻鏍戯紝杞埌鐖惰妭鐐广€佹Ы浣?N + 1锛屼互纭繚鏃犻渶鏍堝嵆鍙彇寰楄繘灞曘€?
鐒惰€岋紝杩欎簺鍙嶅悜鎸囬拡浣垮緱鍚屾椂淇敼涓庤凯浠ｅ彉寰楁鎵嬨€?

### 鍚屾椂淇敼涓庤凯浠?

鏈夎嫢骞茬鎯呭喌闇€瑕佽€冭檻锛?
1. 绠€鍗曟彃鍏?鏇挎崲銆傝繖鍙渶鍦ㄥ睆闅滐紙barrier锛変箣鍚庯紝鐢ㄤ竴涓寚鍚戞柊鍙跺瓙鐨勬寚閽堟浛鎹?NULL 鎴栨棫鐨勫尮閰嶅彾瀛愭寚閽堛€傞櫎姝や箣澶栧厓鏁版嵁鍧椾笉浼氭敼鍙樸€傛棫鍙跺瓙鍦?RCU 瀹介檺鏈熶箣鍚庢墠浼氳閲婃斁銆?
2. 绠€鍗曞垹闄ゃ€傝繖鍙秹鍙婃竻闄や竴涓棫鐨勫尮閰嶅彾瀛愩€傞櫎姝や箣澶栧厓鏁版嵁鍧椾笉浼氭敼鍙樸€傛棫鍙跺瓙鍦?RCU 瀹介檺鏈熶箣鍚庢墠浼氳閲婃斁銆?
3. 鎻掑叆鏇挎崲浜嗘垜浠皻鏈繘鍏ョ殑瀛愭爲鐨勪竴閮ㄥ垎銆傝繖鍙兘娑夊強鏇挎崲璇ュ瓙鏍戠殑涓€閮ㄥ垎鈥斺€斾絾杩欎笉浼氬奖鍝嶈凯浠ｏ紝鍥犱负鎴戜滑灏氭湭鍒拌揪鎸囧悜瀹冪殑鎸囬拡锛屼笖绁栧厛鍧椾笉浼氳鏇挎崲锛堥偅浜涘潡鐨勫竷灞€涓嶆敼鍙橈級銆?
4. 鎻掑叆鏇挎崲鎴戜滑姝ｅ湪涓诲姩澶勭悊鐨勮妭鐐广€傝繖涓嶆槸闂锛屽洜涓烘垜浠凡缁忚秺杩囦簡閿氬畾鎸囬拡锛屽苟涓斿湪娌垮弽鍚戞寚閽堝洖婧箣鍓嶄笉浼氬垏鎹㈠埌鏂板竷灞€鈥斺€旇€屽湪閭ｄ釜鏃跺€欙紝鎴戜滑宸茬粡妫€鏌ヤ簡琚浛鎹㈣妭鐐逛腑鐨勫彾瀛愶紙鎴戜滑鍦ㄨ窡闅忎换浣曞厓鏁版嵁鎸囬拡涔嬪墠浼氬厛閬嶅巻鑺傜偣涓殑鎵€鏈夊彾瀛愶級銆?
   鐒惰€岋紝鎴戜滑鍙兘浼氬啀娆＄湅鍒颁竴浜涜鎷嗗垎鍒版柊鍒嗘敮涓殑鍙跺瓙锛岃鍒嗘敮浣嶄簬鎴戜滑褰撴椂鎵€澶勪綅缃箣鍚庣殑鏌愪釜妲戒綅涓€?
5. 鎻掑叆鏇挎崲鎴戜滑姝ｅ湪澶勭悊鍏朵緷璧栧垎鏀殑鑺傜偣銆傝繖鍦ㄦ垜浠部鍙嶅悜鎸囬拡鍥炴函涔嬪墠涓嶄細褰卞搷鎴戜滑銆備笌锛?锛夌被浼笺€?
6. 鍒犻櫎鍚堝苟鎴戜滑涓嬫柟鐨勫垎鏀€傝繖涓嶄細褰卞搷鎴戜滑锛屽洜涓哄弽鍚戞寚閽堜細浣挎垜浠湪鐪嬪埌鏂拌妭鐐逛箣鍓嶅氨鍥炲埌鏂拌妭鐐圭殑鐖惰妭鐐广€傛暣涓鍚堝苟鐨勫瓙鏍戝師鏍疯涓㈠純鈥斺€斿苟涓斾粛灏嗕互鍚屼竴涓Ы浣嶄负鏍癸紝鍥犳鎴戜滑鍦ㄥ洖鍒版Ы浣?+ 1 鏃朵笉搴斿啀娆″鐞嗗畠銆?

   鍦ㄦ煇浜涙儏鍐典笅锛屾垜浠渶瑕佸悓鏃舵敼鍙樿妭鐐圭殑鐖舵寚閽堝拰鐖舵Ы浣嶆寚閽堬紙渚嬪锛屾垜浠湪瀹冧箣鍓嶆彃鍏ヤ簡鍙︿竴涓妭鐐瑰苟灏嗗叾涓婄Щ浜嗕竴灞傦級銆備笉鍔犻攣鍦拌灏辨棤娉曞仛鍒拌繖涓€鐐光€斺€斿洜姝ゆ垜浠篃蹇呴』鏇挎崲璇ヨ妭鐐广€?
   鐒惰€岋紝褰撴垜浠皢涓€涓揩鎹锋柟寮忔敼涓鸿妭鐐规椂锛岃繖涓嶆槸闂锛屽洜涓哄揩鎹锋柟寮忓彧鏈変竴涓Ы浣嶏紝鍥犳鍦ㄦ部鍏跺弽鍚戦亶鍘嗘椂涓嶄細鐢ㄥ埌鐖舵Ы浣嶅彿銆傝繖鎰忓懗鐫€鍙互鍏堟敼鍙樻Ы浣嶅彿鈥斺€斿彧瑕佷娇鐢ㄥ悎閫傜殑灞忛殰鏉ョ‘淇濈埗妲戒綅鍙锋槸鍦ㄨ鍙栧弽鍚戞寚閽堜箣鍚庢墠琚鍙栫殑銆?
澶辨晥鐨勫潡鍜屽彾瀛愬湪缁忚繃涓€涓?RCU 瀹介檺鏈熶箣鍚庢墠浼氳閲婃斁锛屽洜姝ゅ彧瑕佷换浣曟墽琛岄亶鍘嗘垨杩唬鐨勪汉鎸佹湁 RCU 璇婚攣锛屾棫鐨勬敮鎾戠粨鏋勫氨涓嶅簲褰撲粠瀹冧滑涓嬮潰娑堝け銆?