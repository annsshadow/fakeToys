
## 鏈€灏忓爢 API锛圡in Heap API锛?


:Author: Kuan-Wei Chiu <visitorckw@gmail.com>

## 绠€浠?


鏈€灏忓爢锛圡in Heap锛堿PI 鎻愪緵浜嗕竴缁勫嚱鏁板拰瀹忥紝鐢ㄤ簬鍦?Linux 鍐呮牳涓鐞嗘渶灏忓爢銆傛渶灏忓爢鏄竴绉?
浜屽弶鏍戠粨鏋勶紝鍏朵腑姣忎釜鑺傜偣鐨勫€奸兘灏忎簬鎴栫瓑浜庡叾瀛愯妭鐐圭殑鍊硷紝浠庤€屼繚璇佹渶灏忕殑鍏冪礌濮嬬粓浣嶄簬鏍硅妭鐐广€?

鏈枃妗ｆ彁渚涗簡鏈€灏忓爢 API 鐨勪娇鐢ㄦ寚鍗楋紝璇﹁堪濡備綍瀹氫箟鍜屼娇鐢ㄦ渶灏忓爢銆傜敤鎴蜂笉搴旂洿鎺ヨ皟鐢ㄥ甫鏈?
**__min_heap_*()** 鍓嶇紑鐨勫嚱鏁帮紝鑰屽簲浣跨敤鎵€鎻愪緵鐨勫畯灏佽锛坢acro wrappers锛夈€?

闄や簡杩欎簺鍑芥暟鐨勬爣鍑嗙増鏈锛岃 API 杩樺寘鍚竴缁?inline 鐗堟湰锛岀敤浜庢€ц兘鏁忔劅鐨勫満鍚堛€傝繖浜?inline
鍑芥暟鐨勫悕绉颁笌鍏堕潪 inline 瀵瑰簲鐗堟湰鐩稿悓锛屼絾甯︽湁 **_inline** 鍚庣紑銆備緥濡?
**__min_heap_init_inline** 鍙婂叾瀵瑰簲鐨勫畯灏佽 **min_heap_init_inline**銆俰nline 鐗堟湰鍏佽
鐩存帴璋冪敤鑷畾涔夌殑姣旇緝鍜屼氦鎹㈠嚱鏁帮紝鑰屼笉缁忚繃闂存帴鍑芥暟璋冪敤銆傝繖鍙互鏄捐憲鍑忓皯寮€閿€锛屽挨鍏舵槸鍦ㄥ惎鐢?
CONFIG_MITIGATION_RETPOLINE 鏃讹紝鍥犱负闂存帴鍑芥暟璋冪敤浼氬彉寰楁洿鍔犳槀璐点€備笌闈?inline 鐗堟湰涓€鏍凤紝
閲嶈鐨勬槸瀵?inline 鍑芥暟涔熻浣跨敤瀹忓皝瑁咃紝鑰屼笉鏄洿鎺ヨ皟鐢ㄥ嚱鏁版湰韬€?

## 鏁版嵁缁撴瀯


### 鏈€灏忓爢鐨勫畾涔?


琛ㄧず鏈€灏忓爢鐨勬牳蹇冩暟鎹粨鏋勪娇鐢?**MIN_HEAP_PREALLOCATED** 鍜?**DEFINE_MIN_HEAP** 瀹忔潵瀹氫箟銆?
杩欎簺瀹忓厑璁镐綘瀹氫箟涓€涓甫鏈夐鍒嗛厤缂撳啿鍖烘垨鍔ㄦ€佸垎閰嶅唴瀛樼殑鏈€灏忓爢銆?

绀轰緥锛?


    #define MIN_HEAP_PREALLOCATED(_type, _name, _nr)
    struct _name {
        size_t nr;         /** 鍫嗕腑鍏冪礌鐨勬暟閲?**/
        size_t size;       /** 鍙绾崇殑鏈€澶у厓绱犳暟閲?**/
        _type **data;    /** 鎸囧悜鍫嗘暟鎹殑鎸囬拡 */
        _type preallocated[_nr];  /** 闈欐€侀鍒嗛厤鏁扮粍 **/
    }

    #define DEFINE_MIN_HEAP(_type, _name) MIN_HEAP_PREALLOCATED(_type, _name, 0)

涓€涓吀鍨嬬殑鍫嗙粨鏋勪細鍖呭惈涓€涓厓绱犺鏁帮紙`nr`锛夈€佸爢鐨勬渶澶у閲忥紙`size`锛夛紝浠ュ強涓€涓寚鍚戝厓绱犳暟缁?
鐨勬寚閽堬紙`data`锛夈€傚彲閫夊湴锛屼綘鍙互浣跨敤 **MIN_HEAP_PREALLOCATED** 鎸囧畾涓€涓潤鎬佹暟缁勭敤浜庡爢鐨?
棰勫垎閰嶅瓨鍌ㄣ€?

### 鏈€灏忓爢鍥炶皟


**struct min_heap_callbacks** 鎻愪緵浜嗙敤浜庡爢涓厓绱犳帓搴忎笌浜ゆ崲鐨勮嚜瀹氫箟閫夐」銆傚畠鍖呭惈涓や釜鍑芥暟
鎸囬拡锛?


    struct min_heap_callbacks {
        bool (**less)(const void **lhs, const void **rhs, void **args);
        void (**swp)(void **lhs, void **rhs, void **args);
    };

- **less** 鏄敤浜庣‘瀹氬厓绱犻『搴忕殑姣旇緝鍑芥暟銆?
- **swp** 鏄敤浜庝氦鎹㈠爢涓厓绱犵殑鍑芥暟銆傚鏋?swp 璁句负 NULL锛屽垯灏嗕娇鐢ㄩ粯璁ょ殑浜ゆ崲鍑芥暟锛岃鍑芥暟
  鏍规嵁鍏冪礌澶у皬杩涜浜ゆ崲銆?

## 瀹忓皝瑁?


涓轰簡浠ョ敤鎴峰弸濂界殑鏂瑰紡涓庡爢浜や簰锛屾彁渚涗簡浠ヤ笅瀹忓皝瑁呫€傛瘡涓畯瀵瑰簲涓€涓搷浣滃爢鐨勫嚱鏁帮紝瀹冧滑灞忚斀浜?
瀵瑰唴閮ㄥ嚱鏁扮殑鐩存帴璋冪敤銆?

姣忎釜瀹忔帴鍙楄嫢骞插弬鏁帮紝璇︽儏濡備笅銆?

### 鍫嗗垵濮嬪寲



    min_heap_init(heap, data, size);

- **heap**锛氭寚鍚戝緟鍒濆鍖栫殑鍫嗙粨鏋勭殑鎸囬拡銆?
- **data**锛氭寚鍚戠敤浜庡瓨鍌ㄥ爢鍏冪礌鐨勭紦鍐插尯鐨勬寚閽堛€傚鏋滀负 `NULL`锛屽垯浣跨敤鍫嗙粨鏋勫唴鐨勯鍒嗛厤
  缂撳啿鍖恒€?
- **size**锛氬爢鍙绾崇殑鏈€澶у厓绱犳暟閲忋€?

璇ュ畯鍒濆鍖栧爢锛岃缃叾鍒濆鐘舵€併€傚鏋?`data` 涓?`NULL`锛屽垯浣跨敤鍫嗙粨鏋勫唴鐨勯鍒嗛厤鍐呭瓨杩涜
瀛樺偍锛涘惁鍒欎娇鐢ㄧ敤鎴锋彁渚涚殑缂撳啿鍖恒€傝鎿嶄綔澶嶆潅搴︿负 **O(1)**銆?

**Inline 鐗堟湰锛?* min_heap_init_inline(heap, data, size)

### 璁块棶鍫嗛《鍏冪礌



    element = min_heap_peek(heap);

- **heap**锛氭寚鍚戜粠涓幏鍙栨渶灏忓厓绱犵殑鍫嗙殑鎸囬拡銆?

璇ュ畯杩斿洖鎸囧悜鍫嗕腑鏈€灏忓厓绱狅紙鏍硅妭鐐癸級鐨勬寚閽堬紝濡傛灉鍫嗕负绌哄垯杩斿洖 `NULL`銆傝鎿嶄綔澶嶆潅搴︿负 **O(1)**銆?

**Inline 鐗堟湰锛?* min_heap_peek_inline(heap)

### 鍫嗘彃鍏?



    success = min_heap_push(heap, element, callbacks, args);

- **heap**锛氭寚鍚戣鎻掑叆鍏冪礌鐨勫爢鐨勬寚閽堛€?
- **element**锛氭寚鍚戣鎻掑叆鍫嗕腑鐨勫厓绱犵殑鎸囬拡銆?
- **callbacks**锛氭寚鍚?`struct min_heap_callbacks` 鐨勬寚閽堬紝鎻愪緵 `less` 鍜?`swp` 鍑芥暟銆?
- **args**锛氫紶閫掔粰 `less` 鍜?`swp` 鍑芥暟鐨勫彲閫夊弬鏁般€?

璇ュ畯灏嗕竴涓厓绱犳彃鍏ュ爢涓€傚鏋滄彃鍏ユ垚鍔熻繑鍥?`true`锛屽鏋滃爢宸叉弧鍒欒繑鍥?`false`銆傝鎿嶄綔澶嶆潅搴︿负
**O(log n)**銆?

**Inline 鐗堟湰锛?* min_heap_push_inline(heap, element, callbacks, args)

### 鍫嗗垹闄?



    success = min_heap_pop(heap, callbacks, args);

- **heap**锛氭寚鍚戣浠庝腑鍒犻櫎鏈€灏忓厓绱犵殑鍫嗙殑鎸囬拡銆?
- **callbacks**锛氭寚鍚?`struct min_heap_callbacks` 鐨勬寚閽堬紝鎻愪緵 `less` 鍜?`swp` 鍑芥暟銆?
- **args**锛氫紶閫掔粰 `less` 鍜?`swp` 鍑芥暟鐨勫彲閫夊弬鏁般€?

璇ュ畯浠庡爢涓垹闄ゆ渶灏忓厓绱狅紙鏍硅妭鐐癸級銆傚鏋滃厓绱犺鎴愬姛鍒犻櫎杩斿洖 `true`锛屽鏋滃爢涓虹┖鍒欒繑鍥?`false`銆?
璇ユ搷浣滃鏉傚害涓?**O(log n)**銆?

**Inline 鐗堟湰锛?* min_heap_pop_inline(heap, callbacks, args)

### 鍫嗙淮鎶?


浣犲彲浠ヤ娇鐢ㄤ互涓嬪畯鏉ョ淮鎶ゅ爢鐨勭粨鏋勶細


    min_heap_sift_down(heap, pos, callbacks, args);

- **heap**锛氭寚鍚戝爢鐨勬寚閽堛€?
- **pos**锛氬紑濮嬪悜涓嬬瓫閫夛紙sift down锛夌殑绱㈠紩銆?
- **callbacks**锛氭寚鍚?`struct min_heap_callbacks` 鐨勬寚閽堬紝鎻愪緵 `less` 鍜?`swp` 鍑芥暟銆?
- **args**锛氫紶閫掔粰 `less` 鍜?`swp` 鍑芥暟鐨勫彲閫夊弬鏁般€?

璇ュ畯閫氳繃灏嗘寚瀹氱储寮曪紙`pos`锛夊鐨勫厓绱犳部鍫嗗悜涓嬬Щ鍔紝鐩村埌瀹冨浜庢纭綅缃紝浠庤€屾仮澶嶅爢鎬ц川銆?
璇ユ搷浣滃鏉傚害涓?**O(log n)**銆?

**Inline 鐗堟湰锛?* min_heap_sift_down_inline(heap, pos, callbacks, args)


    min_heap_sift_up(heap, idx, callbacks, args);

- **heap**锛氭寚鍚戝爢鐨勬寚閽堛€?
- **idx**锛氳鍚戜笂绛涢€夌殑鍏冪礌鐨勭储寮曘€?
- **callbacks**锛氭寚鍚?`struct min_heap_callbacks` 鐨勬寚閽堬紝鎻愪緵 `less` 鍜?`swp` 鍑芥暟銆?
- **args**锛氫紶閫掔粰 `less` 鍜?`swp` 鍑芥暟鐨勫彲閫夊弬鏁般€?

璇ュ畯閫氳繃灏嗘寚瀹氱储寮曪紙`idx`锛夊鐨勫厓绱犳部鍫嗗悜涓婄Щ鍔紝浠庤€屾仮澶嶅爢鎬ц川銆傝鎿嶄綔澶嶆潅搴︿负 **O(log n)**銆?

**Inline 鐗堟湰锛?* min_heap_sift_up_inline(heap, idx, callbacks, args)


    min_heapify_all(heap, callbacks, args);

- **heap**锛氭寚鍚戝爢鐨勬寚閽堛€?
- **callbacks**锛氭寚鍚?`struct min_heap_callbacks` 鐨勬寚閽堬紝鎻愪緵 `less` 鍜?`swp` 鍑芥暟銆?
- **args**锛氫紶閫掔粰 `less` 鍜?`swp` 鍑芥暟鐨勫彲閫夊弬鏁般€?

璇ュ畯纭繚鏁翠釜鍫嗘弧瓒冲爢鎬ц川銆傚畠鍦ㄥ爢浠庡ご鏋勫缓鎴栫粡杩囧娆′慨鏀瑰悗琚皟鐢ㄣ€傝鎿嶄綔澶嶆潅搴︿负 **O(n)**銆?

**Inline 鐗堟湰锛?* min_heapify_all_inline(heap, callbacks, args)

### 鍒犻櫎鐗瑰畾鍏冪礌



    success = min_heap_del(heap, idx, callbacks, args);

- **heap**锛氭寚鍚戝爢鐨勬寚閽堛€?
- **idx**锛氳鍒犻櫎鐨勫厓绱犵殑绱㈠紩銆?
- **callbacks**锛氭寚鍚?`struct min_heap_callbacks` 鐨勬寚閽堬紝鎻愪緵 `less` 鍜?`swp` 鍑芥暟銆?
- **args**锛氫紶閫掔粰 `less` 鍜?`swp` 鍑芥暟鐨勫彲閫夊弬鏁般€?

璇ュ畯浠庡爢涓垹闄ゆ寚瀹氱储寮曪紙`idx`锛夊鐨勫厓绱犲苟鎭㈠鍫嗘€ц川銆傝鎿嶄綔澶嶆潅搴︿负 **O(log n)**銆?

**Inline 鐗堟湰锛?* min_heap_del_inline(heap, idx, callbacks, args)

## 鍏朵粬宸ュ叿


- **min_heap_full(heap)**锛氭鏌ュ爢鏄惁宸叉弧銆傚鏉傚害锛?*O(1)**銆?


    bool full = min_heap_full(heap);

- `heap`锛氭寚鍚戣妫€鏌ョ殑鍫嗙殑鎸囬拡銆?

璇ュ畯鍦ㄥ爢宸叉弧鏃惰繑鍥?`true`锛屽惁鍒欒繑鍥?`false`銆?

**Inline 鐗堟湰锛?* min_heap_full_inline(heap)

- **min_heap_empty(heap)**锛氭鏌ュ爢鏄惁涓虹┖銆傚鏉傚害锛?*O(1)**銆?


    bool empty = min_heap_empty(heap);

- `heap`锛氭寚鍚戣妫€鏌ョ殑鍫嗙殑鎸囬拡銆?

璇ュ畯鍦ㄥ爢涓虹┖鏃惰繑鍥?`true`锛屽惁鍒欒繑鍥?`false`銆?

**Inline 鐗堟湰锛?* min_heap_empty_inline(heap)

## 绀轰緥鐢ㄦ硶


鏈€灏忓爢 API 鐨勫吀鍨嬬敤娉曞寘鎷畾涔夊爢缁撴瀯銆佸垵濮嬪寲瀹冿紝浠ュ強鎸夐渶鎻掑叆鍜屽垹闄ゅ厓绱犮€?


    #include <linux/min_heap.h>

    int my_less_function(const void **lhs, const void **rhs, void *args) {
        return (**(int **)lhs < **(int **)rhs);
    }

    struct min_heap_callbacks heap_cb = {
        .less = my_less_function,    /** 鐢ㄤ簬鍫嗛『搴忕殑姣旇緝鍑芥暟 **/
        .swp  = NULL,                /** 浣跨敤榛樿浜ゆ崲鍑芥暟 **/
    };

    void example_usage(void) {
        /** 鐢ㄥ厓绱犻濉厖缂撳啿鍖?**/
        int buffer[^5^] = {5, 2, 8, 1, 3};
        /** 澹版槑涓€涓渶灏忓爢 **/
        DEFINE_MIN_HEAP(int, my_heap);

        /** 鐢ㄩ鍒嗛厤缂撳啿鍖哄拰澶у皬鍒濆鍖栧爢 **/
        min_heap_init(&my_heap, buffer, 5);

        /** 浣跨敤 min_heapify_all 鏋勫缓鍫?**/
        my_heap.nr = 5;  /** 璁剧疆鍫嗕腑鍏冪礌鐨勬暟閲?**/
        min_heapify_all(&my_heap, &heap_cb, NULL);

        /** 鏌ョ湅鍫嗛《鍏冪礌锛堟湰渚嬩腑搴斾负 1锛?**/
        int *top = min_heap_peek(&my_heap);
        pr_info("Top element: %d\n", *top);

        /** 寮瑰嚭鍫嗛《鍏冪礌锛?锛夊苟鑾峰彇鏂扮殑鍫嗛《锛?锛?**/
        min_heap_pop(&my_heap, &heap_cb, NULL);
        top = min_heap_peek(&my_heap);
        pr_info("New top element: %d\n", *top);

        /** 鎻掑叆涓€涓柊鍏冪礌锛?锛夊苟閲嶆柊妫€鏌ュ爢椤?**/
        int new_element = 0;
        min_heap_push(&my_heap, &new_element, &heap_cb, NULL);
        top = min_heap_peek(&my_heap);
        pr_info("Top element after insertion: %d\n", *top);
    }
