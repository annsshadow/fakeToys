
## 缂栧啓 kernel-doc 娉ㄩ噴


Linux 鍐呮牳婧愭枃浠朵腑鍙兘鍖呭惈閲囩敤 kernel-doc 鏍煎紡鐨勭粨鏋勫寲鏂囨。娉ㄩ噴锛岀敤浜庢弿杩颁唬鐮佺殑鍑芥暟銆佺被鍨嬪拰璁捐銆傚綋鏂囨。宓屽叆鍦ㄦ簮鏂囦欢涓椂锛屾洿瀹规槗淇濇寔鏂囨。涓庝唬鐮佸悓姝ユ洿鏂般€?

   gtk-doc 鎴?Doxygen 绫讳技锛屼絾鐢变簬鍘嗗彶鍘熷洜鍙堟槑鏄句笉鍚屻€傚唴鏍告簮鐮佷腑鍖呭惈鏁颁互涓囪鐨?kernel-doc 娉ㄩ噴銆傝閬靛惊姝ゅ鎻忚堪鐨勯鏍笺€?

   璇峰弬闃?Documentation/rust/general-information.rst銆?

kernel-doc 缁撴瀯浼氫粠娉ㄩ噴涓彁鍙栧嚭鏉ワ紝骞舵嵁姝ょ敓鎴愬甫閿氱偣鐨勩€佹牸寮忔纭殑 `Sphinx C Domain`_ 鍑芥暟涓庣被鍨嬫弿杩般€傝繖浜涙弿杩颁細缁忚繃鐗规畩 kernel-doc 楂樹寒涓庝氦鍙夊紩鐢ㄧ殑杩囨护澶勭悊銆傝瑙佷笅鏂囥€?


姣忎釜閫氳繃 `EXPORT_SYMBOL` 鎴?`EXPORT_SYMBOL_GPL` 瀵煎嚭缁欏彲鍔犺浇妯″潡鐨勫嚱鏁伴兘搴旀嫢鏈?kernel-doc 娉ㄩ噴銆傚ご鏂囦欢涓緵妯″潡浣跨敤鐨勫嚱鏁颁笌鏁版嵁缁撴瀯鍚屾牱搴斿綋鎷ユ湁 kernel-doc 娉ㄩ噴銆?

涓哄叾浠栧唴鏍告枃浠跺彲瑙侊紙鏈爣璁颁负 `static`锛夌殑鍑芥暟鎻愪緵 kernel-doc 鏍煎紡鐨勬枃妗ｄ篃鏄竴绉嶈壇濂界殑瀹炶返銆傛垜浠繕寤鸿涓虹鏈夛紙鏂囦欢绾?`static`锛夊嚱鏁颁篃鎻愪緵 kernel-doc 鏍煎紡鏂囨。锛屼互淇濇寔鍐呮牳婧愮爜甯冨眬鐨勪竴鑷存€с€傝繖灞炰簬杈冧綆浼樺厛绾э紝鐢辫鍐呮牳婧愭枃浠剁淮鎶よ€呰嚜琛屽喅瀹氥€?

### 濡備綍鏍煎紡鍖?kernel-doc 娉ㄩ噴


kernel-doc 娉ㄩ噴浣跨敤寮€澶寸殑娉ㄩ噴鏍囪 `/**`銆俙kernel-doc` 宸ュ叿浼氭彁鍙栦互姝ゆ柟寮忔爣璁扮殑娉ㄩ噴銆傛敞閲婄殑鍏朵綑閮ㄥ垎鏍煎紡濡傚悓鏅€氱殑澶氳娉ㄩ噴锛屽乏渚ф湁涓€鍒楁槦鍙凤紝骞朵互鐙崰涓€琛岀殑 `*/` 缁撴潫銆?

鍑芥暟涓庣被鍨嬬殑 kernel-doc 娉ㄩ噴搴旂揣閭绘墍鎻忚堪鐨勫嚱鏁版垨绫诲瀷涔嬪墠鏀剧疆锛屼互鏈€澶х▼搴︽彁楂樹唬鐮佹敼鍔ㄨ€呭悓鏃舵洿鏂版枃妗ｇ殑鍙兘鎬с€傛杩扮被锛坥verview锛夌殑 kernel-doc 娉ㄩ噴鍙互鏀惧湪椤跺眰缂╄繘绾у埆鐨勪换鎰忎綅缃€?

浠ユ洿楂樼殑璇︾粏绋嬪害杩愯 `kernel-doc` 宸ュ叿涓斾笉瀹為檯鐢熸垚杈撳嚭锛屽彲鐢ㄤ簬楠岃瘉 kernel-doc 娉ㄩ噴鏍煎紡鏄惁姝ｇ‘锛?

```
	tools/docs/kernel-doc -v -none drivers/foo/bar.c
```

`.c` 鏂囦欢鐨勬枃妗ｆ牸寮忎篃浼氱敱鍐呮牳鏋勫缓杩囩▼楠岃瘉锛?

```
	make W=n
```

涓嶈繃锛屼笂杩板懡浠や笉浼氶獙璇佸ご鏂囦欢銆傚ご鏂囦欢搴斾娇鐢?`kernel-doc` 鍗曠嫭妫€鏌ャ€?

### 鍑芥暟鏂囨。


```
  /**
   * function_name() - Brief description of function.
   * @arg1: Describe the first argument.
   * @arg2: Describe the second argument.
   *        One can provide multiple line descriptions
   *        for arguments.
   *
   * A longer description, with more discussion of the function function_name()
   * that might be useful to those using or modifying it. Begins with an
   * empty comment line, and may include additional embedded empty
   * comment lines.
   *
   * The longer description may have multiple paragraphs.
   *
   * Context: Describes whether the function can sleep, what locks it takes,
   *          releases, or expects to be held. It can extend over multiple
   *          lines.
   * Return: Describe the return value of function_name.
   *
   * The return value description can also have multiple paragraphs, and should
   * be placed at the end of the comment block.
   */
```

鍑芥暟鍚嶄箣鍚庣殑绠€瑕佹弿杩板彲浠ヨ法瓒婂琛岋紝骞跺湪鍙傛暟鎻忚堪銆佺┖娉ㄩ噴琛屾垨娉ㄩ噴鍧楃粨鏉熸椂缁堟銆?

#### 鍑芥暟鍙傛暟


姣忎釜鍑芥暟鍙傛暟搴旀寜椤哄簭绱ч殢绠€鐭殑鍑芥暟鎻忚堪涔嬪悗杩涜鎻忚堪銆傚嚱鏁版弿杩颁笌鍙傛暟涔嬮棿銆佷互鍙婂悇鍙傛暟涔嬮棿涓嶈鐣欑┖琛屻€?

姣忎釜 `@argument:` 鎻忚堪鍙互璺ㄨ秺澶氳銆?


   If the `@argument` description has multiple lines, the continuation
```
      * @argument: some long description
      *            that continues on next lines

   or::

      * @argument:
      *		some long description
      *		that continues on next lines
```

濡傛灉鍑芥暟鎷ユ湁鏁伴噺鍙彉鐨勫弬鏁帮紝鍏舵弿杩板簲涓猴細

```
      * @...: description
```

#### 鍑芥暟涓婁笅鏂?


鍑芥暟鍙璋冪敤鐨勪笂涓嬫枃搴斿湪涓€涓悕涓?`Context` 鐨勫皬鑺備腑鎻忚堪銆傚叾涓簲鍖呮嫭璇ュ嚱鏁版槸鍚︿細浼戠湢銆佽兘鍚﹀湪涓柇涓婁笅鏂囦腑璋冪敤锛屼互鍙婂畠鑾峰彇銆侀噴鏀炬垨鏈熸湜鍏惰皟鐢ㄨ€呮寔鏈夊摢浜涢攣銆?

```
  * Context: Any context.
  * Context: Any context. Takes and releases the RCU lock.
  * Context: Any context. Expects <lock> to be held by caller.
  * Context: Process context. May sleep if @gfp flags permit.
  * Context: Process context. Takes and releases <mutex>.
  * Context: Softirq or process context. Takes and releases <lock>, BH-safe.
  * Context: Interrupt context.
```

#### 杩斿洖鍊?


杩斿洖鍊硷紙鑻ユ湁锛夊簲鍦ㄤ竴涓悕涓?`Return`锛堟垨 `Returns`锛夌殑涓撶敤灏忚妭涓弿杩般€?


  #) 浣犳彁渚涚殑澶氳鎻忚堪鎬ф枃鏈?*涓嶄細**琚瘑鍒负锛?

```
	* Return:
	* %0 - OK
	* %-EINVAL - invalid argument
	* %-ENOMEM - out of memory
```

     涓婅堪鍐欐硶浼氳鍏ㄩ儴鎷兼帴鍦ㄤ竴璧凤紝浜х敓锛?

```
	Return: 0 - OK -EINVAL - invalid argument -ENOMEM - out of memory
```

     鍥犳锛屼负浜嗕骇鐢熸湡鏈涚殑鎹㈣锛岄渶瑕佷娇鐢?ReST 鍒楄〃锛屼緥濡傦細

```
      * Return:
      * * %0		- OK to runtime suspend the device
      * * %-EBUSY	- Device should not be runtime suspended
```

  #) 濡傛灉浣犳彁渚涚殑鎻忚堪鎬ф枃鏈腑鏈変互鈥滅煭璇姞鍐掑彿鈥濆紑澶寸殑琛岋紝姣忎釜杩欐牱鐨勭煭璇兘浼氳褰撲綔鏂扮殑灏忚妭鏍囬锛岃繖寰堝彲鑳芥棤娉曚骇鐢熸湡鏈涚殑鏁堟灉銆?

### 缁撴瀯浣撱€佽仈鍚堜綋涓庢灇涓炬枃妗?


`struct`銆乣union` 涓?`enum` 鐨?kernel-doc 閫氱敤鏍煎紡涓猴細

```
  /**
   * struct struct_name - Brief description.
   * @member1: Description of member1.
   * @member2: Description of member2.
   *           One can provide multiple line descriptions
   *           for members.
   *
   * Description of the structure.
   */
```

浣犲彲浠ュ皢涓婅堪绀轰緥涓殑 `struct` 鏇挎崲涓?`union` 鎴?`enum` 鏉ユ弿杩拌仈鍚堜綋鎴栨灇涓俱€俙member` 涓€璇嶆棦鐢ㄤ簬鎸囦唬 `struct` 鍜?`union` 鐨勬垚鍛樺悕锛屼篃鐢ㄤ簬鎸囦唬 `enum` 涓殑鏋氫妇椤广€?

缁撴瀯浣撳悕涔嬪悗鐨勭畝瑕佹弿杩板彲浠ヨ法瓒婂琛岋紝骞跺湪鎴愬憳鎻忚堪銆佺┖娉ㄩ噴琛屾垨娉ㄩ噴鍧楃粨鏉熸椂缁堟銆?

#### 鎴愬憳


缁撴瀯浣撱€佽仈鍚堜綋涓庢灇涓剧殑鎴愬憳搴斿鍚屽嚱鏁板弬鏁颁竴鏍疯繘琛屾枃妗ｅ寲锛涘畠浠揣璺熺畝鐭弿杩颁箣鍚庯紝骞朵笖鍙互璺ㄥ琛屻€?

鍦?`struct` 鎴?`union` 鎻忚堪鍐呴儴锛屼綘鍙互浣跨敤 `private:` 涓?`public:` 娉ㄩ噴鏍囩銆備綅浜?`private:` 鍖哄煙鍐呴儴鐨勭粨鏋勪綋瀛楁涓嶄細鍦ㄧ敓鎴愮殑杈撳嚭鏂囨。涓垪鍑恒€?

`private:` 涓?`public:` 鏍囩蹇呴』绱ц窡鍦?`/*` 娉ㄩ噴鏍囪涔嬪悗寮€濮嬨€傚畠浠彲浠ュ彲閫夊湴鍖呭惈浣嶄簬 `:` 涓庣粨鏉熺殑 `*/` 鏍囪涔嬮棿鐨勬敞閲娿€?

褰?`private:` 鐢ㄤ簬宓屽缁撴瀯浣撴椂锛屽畠鍙細浼犳挱鍒板唴灞傜粨鏋勪綋/鑱斿悎浣撱€?


```
  /**
   * struct my_struct - short description
   * @a: first member
   * @b: second member
   * @d: fourth member
   *
   * Longer description
   */
  struct my_struct {
      int a;
      int b;
  /* private: internal use only */
      int c;
  /* public: the next one is public */
      int d;
  };
```

#### 宓屽缁撴瀯浣?鑱斿悎浣?


```
      /**
       * struct nested_foobar - a struct with nested unions and structs
       * @memb1: first member of anonymous union/anonymous struct
       * @memb2: second member of anonymous union/anonymous struct
       * @memb3: third member of anonymous union/anonymous struct
       * @memb4: fourth member of anonymous union/anonymous struct
       * @bar: non-anonymous union
       * @bar.st1: struct st1 inside @bar
       * @bar.st2: struct st2 inside @bar
       * @bar.st1.memb1: first member of struct st1 on union bar
       * @bar.st1.memb2: second member of struct st1 on union bar
       * @bar.st2.memb1: first member of struct st2 on union bar
       * @bar.st2.memb2: second member of struct st2 on union bar
       */
      struct nested_foobar {
        /* Anonymous union/struct*/
        union {
          struct {
            int memb1;
            /* private: hides memb2 from documentation */
            int memb2;
          };
          /* Everything here is public again, as private scope finished */
          struct {
            void *memb3;
            int memb4;
          };
        };
        union {
          struct {
            int memb1;
            int memb2;
          } st1;
          struct {
            void *memb1;
            int memb2;
          } st2;
        } bar;
      };
```

   #) 鍦ㄤ负宓屽缁撴瀯浣撴垨鑱斿悎浣撶紪鍐欐枃妗ｆ椂锛屽鏋?`struct`/`union` `foo` 鍏峰悕锛屽垯鍏跺唴閮ㄧ殑鎴愬憳 `bar` 搴旇涓?`@foo.bar:`銆?
   #) 褰撳祵濂楃殑 `struct`/`union` 涓哄尶鍚嶆椂锛屽叾涓殑鎴愬憳 `bar` 搴旇涓?`@bar:`銆?

#### 琛屽唴鎴愬憳鏂囨。娉ㄩ噴


缁撴瀯浣撴垚鍛樹篃鍙互鍦ㄥ叾瀹氫箟鍐呴儴浠ヨ鍐呮柟寮忕紪鍐欐枃妗ｃ€傛湁涓ょ椋庢牸锛氬崟琛屾敞閲婏紙寮€澶?`/**` 涓庣粨灏?`*/` 浣嶄簬鍚屼竴琛岋級锛屼互鍙婂琛屾敞閲婏紙浜岃€呭悇鍗犱竴琛岋級锛?

```
  /**
   * struct foo - Brief description.
   * @foo: The Foo member.
   */
  struct foo {
        int foo;
        /**
         * @bar: The Bar member.
         */
        int bar;
        /**
         * @baz: The Baz member.
         *
         * Here, the member description may contain several paragraphs.
         */
        int baz;
        union {
                /** @foobar: Single line description. */
                int foobar;
        };
        /** @bar2: Description for struct @bar2 inside @foo */
        struct {
                /**
                 * @bar2.barbar: Description for @barbar inside @foo.bar2
                 */
                int barbar;
        } bar2;
  };
```

### Typedef 鏂囨。


```
  /**
   * typedef type_name - Brief description.
   *
   * Description of the type.
   */
```

```
  /**
   * typedef type_name - Brief description.
   * @arg1: description of arg1
   * @arg2: description of arg2
   *
   * Description of the type.
   *
   * Context: Locking context.
   * Returns: Meaning of the return value.
   */
   typedef void (*type_name)(struct v4l2_ctrl *arg1, void *arg2);
```

### 鍙橀噺鏂囨。


```
  /**
   * var var_name - Brief description.
   *
   * Description of the var_name variable.
   */
   extern int var_name;
```

### 绫诲璞″畯鏂囨。


绫诲璞″畯锛坥bject-like macro锛変笌绫诲嚱鏁板畯锛坒unction-like macro锛変笉鍚屻€備簩鑰呯殑鍖哄垎鍦ㄤ簬锛氱被鍑芥暟瀹忕殑瀹忓悕鏄惁绱ф帴宸﹀渾鎷彿 `'('`锛岀被瀵硅薄瀹忕殑瀹忓悕鍒欎笉绱ч殢宸﹀渾鎷彿銆?

绫诲嚱鏁板畯鐢?`tools/docs/kernel-doc` 鍍忓嚱鏁颁竴鏍峰鐞嗐€傚畠浠彲鑳藉甫鏈夊弬鏁板垪琛ㄣ€傜被瀵硅薄瀹忔病鏈夊弬鏁板垪琛ㄣ€?

```
  /**
   * define object_name - Brief description.
   *
   * Description of the object.
   */
```

```
  /**
   * define MAX_ERRNO - maximum errno value that is supported
   *
   * Kernel pointers have redundant information, so we can use a
   * scheme where we can return either an error code or a normal
   * pointer with the same return value.
   */
  #define MAX_ERRNO	4095
```

```
  /**
   * define DRM_GEM_VRAM_PLANE_HELPER_FUNCS - \
   *	Initializes struct drm_plane_helper_funcs for VRAM handling
   *
   * This macro initializes struct drm_plane_helper_funcs to use the
   * respective helper functions.
   */
  #define DRM_GEM_VRAM_PLANE_HELPER_FUNCS \
	.prepare_fb = drm_gem_vram_plane_helper_prepare_fb, \
	.cleanup_fb = drm_gem_vram_plane_helper_cleanup_fb
```

### 楂樹寒涓庝氦鍙夊紩鐢?


浠ヤ笅鐗规畩妯″紡浼氬湪 kernel-doc 娉ㄩ噴鐨勬弿杩版€ф枃鏈腑琚瘑鍒紝骞惰杞崲涓烘纭殑 reStructuredText 鏍囪涓?`Sphinx C Domain`_ 寮曠敤銆?

	       娉ㄦ剰锛?*涓嶈兘**鍦ㄦ櫘閫氱殑 reStructuredText 鏂囨。涓娇鐢ㄣ€?

`funcname()`
  鍑芥暟寮曠敤銆?

`@parameter`
  鍑芥暟鍙傛暟鐨勫悕绉般€傦紙浠呬綔鏍煎紡鍖栵紝涓嶈繘琛屼氦鍙夊紩鐢ㄣ€傦級

`%CONST`
  甯搁噺鐨勫悕绉般€傦紙浠呬綔鏍煎紡鍖栵紝涓嶈繘琛屼氦鍙夊紩鐢ㄣ€傦級

```
    %0    %NULL    %-1    %-EFAULT    %-EINVAL    %-ENOMEM
```

```literal```
  涓€涓簲鍘熸牱澶勭悊鐨勫瓧闈㈠潡銆傝緭鍑哄皢浣跨敤 `绛夊瀛椾綋`銆?

  濡傛灉浣犻渶瑕佷娇鐢ㄤ竴浜涚壒娈婂瓧绗︼紙鍚﹀垯杩欎簺瀛楃浼氳 kernel-doc 鑴氭湰鎴?reStructuredText 璧嬩簣鐗瑰畾鍚箟锛夛紝璇ヨ娉曠壒鍒湁鐢ㄣ€?

  褰撲綘闇€瑕佸湪鍑芥暟鎻忚堪涓娇鐢ㄧ被浼?`%ph` 杩欐牱鐨勪笢瑗挎椂锛岃繖灏ゅ叾鏈夌敤銆?

`$ENVVAR`
  鐜鍙橀噺鐨勫悕绉般€傦紙浠呬綔鏍煎紡鍖栵紝涓嶈繘琛屼氦鍙夊紩鐢ㄣ€傦級

`&struct name`
  缁撴瀯浣撳紩鐢ㄣ€?

`&enum name`
  鏋氫妇寮曠敤銆?

`&typedef name`
  Typedef 寮曠敤銆?

`&struct_name->member` 鎴?`&struct_name.member`
  `struct` 鎴?`union` 鎴愬憳寮曠敤銆備氦鍙夊紩鐢ㄦ寚鍚?`struct` 鎴?`union` 鐨勫畾涔夛紝鑰岄潪鐩存帴鎸囧悜鎴愬憳銆?

`&name`
  閫氱敤绫诲瀷寮曠敤銆傚缓璁紭鍏堜娇鐢ㄤ笂杩板畬鏁村紩鐢ㄥ舰寮忋€傝繖涓昏鐢ㄤ簬閬楃暀娉ㄩ噴銆?

#### 浠?reStructuredText 杩涜浜ゅ弶寮曠敤


浠?reStructuredText 鏂囨。涓氦鍙夊紩鐢?kernel-doc 娉ㄩ噴閲屽畾涔夌殑鍑芥暟涓庣被鍨嬫棤闇€棰濆璇硶銆傚彧闇€鍦ㄥ嚱鏁板悕鍚庡姞涓?`()`锛屽苟鍦ㄧ被鍨嬪墠鍐欎笂 `struct`銆乣union`銆乣enum` 鎴?`typedef` 鍗冲彲銆?

```
  See foo().
  See struct foo.
  See union bar.
  See enum baz.
  See typedef meh.
```

涓嶈繃锛屽鏋滀綘甯屾湜浜ゅ弶寮曠敤閾炬帴浣跨敤鑷畾涔夋枃瀛楋紝鍙互杩欐牱鍐欙細

```
  See :c:func:`my custom link text for function foo <foo>`.
  See :c:type:`my custom link text for struct bar <bar>`.
```

鏇村缁嗚妭璇峰弬鑰?`Sphinx C Domain`_ 鏂囨。銆?

   鍙橀噺涓嶄細琚嚜鍔ㄨ繘琛屼氦鍙夊紩鐢ㄣ€傚浜庤繖浜涘彉閲忥紝浣犻渶瑕佹樉寮忔坊鍔?C 鍩熶氦鍙夊紩鐢ㄣ€?

### 姒傝堪鏂囨。娉ㄩ噴


涓轰簡渚夸簬璁╂簮浠ｇ爜涓庢敞閲婂郊姝ら潬杩戯紝浣犲彲浠ュ寘鍚?kernel-doc 鏂囨。鍧楋紝瀹冧滑鏄嚜鐢辨牸寮忔敞閲婏紝鑰屼笉鏄拡瀵瑰嚱鏁般€佺粨鏋勪綋銆佽仈鍚堜綋銆佹灇涓俱€乼ypedef 鎴栧彉閲忕殑 kernel-doc銆備緥濡傦紝杩欏彲鐢ㄤ簬鎻忚堪鏌愪釜椹卞姩鎴栧簱浠ｇ爜鐨勮繍琛屽師鐞嗐€?

杩欓€氳繃浣跨敤甯︽湁灏忚妭鏍囬鐨?`DOC:` 娈靛叧閿瓧鏉ュ疄鐜般€?

```
  /**
   * DOC: Theory of Operation
   *
   * The whizbang foobar is a dilly of a gizmo. It can do whatever you
   * want it to do, at any time. It reads your mind. Here's how it works.
   *
   * foo bar splat
   *
   * The only drawback to this gizmo is that is can sometimes damage
   * hardware, software, or its subject(s).
   */
```

`DOC:` 涔嬪悗鐨勬爣棰樻棦浣滀负婧愭枃浠朵腑鐨勬爣棰橈紝涔熶綔涓烘彁鍙栬鏂囨。娉ㄩ噴鐨勬爣璇嗙銆傚洜姝わ紝鏍囬鍦ㄦ枃浠跺唴蹇呴』鍞竴銆?

## 鍖呭惈 kernel-doc 娉ㄩ噴


鏂囨。娉ㄩ噴鍙互浣跨敤涓撶敤鐨?kernel-doc Sphinx 鎸囦护鎵╁睍锛屽寘鍚繘浠绘剰 reStructuredText 鏂囨。涓€?

```
  .. kernel-doc:: source
     :option:
```

**source** 鏄浉瀵逛簬鍐呮牳婧愮爜鏍戠殑婧愭枃浠惰矾寰勩€傛敮鎸佷互涓嬫寚浠ら€夐」锛?

export: **[source-pattern ...]**
  鍖呭惈 **source** 涓墍鏈夊凡閫氳繃 `EXPORT_SYMBOL` 鎴?`EXPORT_SYMBOL_GPL` 瀵煎嚭鐨勫嚱鏁扮殑鏂囨。锛屽鍑轰綅缃彲浠ユ槸 **source** 鏈韩锛屼篃鍙互鏄?**source-pattern** 鎸囧畾鐨勪换鎰忔枃浠躲€?

  **source-pattern** 鍦?kernel-doc 娉ㄩ噴琚斁鍦ㄥご鏂囦欢涓€佽€?`EXPORT_SYMBOL` 涓?`EXPORT_SYMBOL_GPL` 绱ч偦鍑芥暟瀹氫箟鏃堕潪甯告湁鐢ㄣ€?

```
    .. kernel-doc:: lib/bitmap.c
       :export:

    .. kernel-doc:: include/net/mac80211.h
       :export: net/mac80211/*.c
```

internal: **[source-pattern ...]**
  鍖呭惈 **source** 涓墍鏈?*鏈?*閫氳繃 `EXPORT_SYMBOL` 鎴?`EXPORT_SYMBOL_GPL` 瀵煎嚭鐨勫嚱鏁颁笌绫诲瀷鐨勬枃妗ｏ紝瀵煎嚭浣嶇疆鍙互鏄?**source** 鏈韩锛屼篃鍙互鏄?**source-pattern** 鎸囧畾鐨勪换鎰忔枃浠躲€?

```
    .. kernel-doc:: drivers/gpu/drm/i915/intel_audio.c
       :internal:
```

identifiers: **[ function/type ...]**
  鍖呭惈 **source** 涓瘡涓?**function** 涓?**type** 鐨勬枃妗ｃ€傚鏋滄湭鎸囧畾 **function**锛屽垯浼氬寘鍚?**source** 涓墍鏈夊嚱鏁颁笌绫诲瀷鐨勬枃妗ｃ€?*type** 鍙互鏄?`struct`銆乣union`銆乣enum`銆乣typedef` 鎴?`var` 鏍囪瘑绗︺€?

```
    .. kernel-doc:: lib/bitmap.c
       :identifiers: bitmap_parselist bitmap_parselist_user

    .. kernel-doc:: lib/idr.c
       :identifiers:
```

no-identifiers: **[ function/type ...]**
  鎺掗櫎 **source** 涓瘡涓?**function** 涓?**type** 鐨勬枃妗ｃ€?

```
    .. kernel-doc:: lib/bitmap.c
       :no-identifiers: bitmap_parselist
```

functions: **[ function/type ...]**
  杩欐槸 `identifiers` 鎸囦护鐨勫埆鍚嶏紝宸插簾寮冦€?

doc: **title**
  鍖呭惈 **source** 涓敱 **title** 鏍囪瘑鐨?`DOC:` 娈佃惤鐨勬枃妗ｃ€?*title** 涓厑璁稿寘鍚┖鏍硷紱涓嶈涓?**title** 鍔犲紩鍙枫€?*title** 浠呬綔涓鸿娈佃惤鐨勬爣璇嗙锛屼笉浼氬寘鍚湪杈撳嚭涓€傝纭繚鍦ㄥ鍥寸殑 reStructuredText 鏂囨。涓湁鍚堥€傜殑鏍囬銆?

```
    .. kernel-doc:: drivers/gpu/drm/i915/intel_audio.c
       :doc: High Definition Audio over HDMI and Display Port
```

涓嶅甫閫夐」鏃讹紝kernel-doc 鎸囦护浼氬寘鍚簮鏂囦欢涓墍鏈夌殑鏂囨。娉ㄩ噴銆?

kernel-doc 鎵╁睍浣嶄簬鍐呮牳婧愮爜鏍戜腑锛岃矾寰勪负 `Documentation/sphinx/kerneldoc.py`銆傚畠鍦ㄥ唴閮ㄤ娇鐢?`tools/docs/kernel-doc` 鑴氭湰鏉ヤ粠婧愮爜涓彁鍙栨枃妗ｆ敞閲娿€?

### 濡備綍浣跨敤 kernel-doc 鐢熸垚 man 鎵嬪唽椤?


```
  $ make mandocs
```

```
  $ ./tools/docs/sphinx-build-wrapper mandocs

杈撳嚭浼氫綅浜庤緭鍑虹洰褰曚笅鐨?`/man` 鐩綍涓紙榛樿锛歚Documentation/output`锛夈€?

鍙€夊湴锛屼篃鍙互閫氳繃浣跨敤 SPHINXDIRS 鏉ョ敓鎴愰儴鍒?man 鎵嬪唽椤甸泦鍚堬細

  $ make SPHINXDIRS=driver-api/media mandocs


   褰撲娇鐢?SPHINXDIRS={subdir} 鏃讹紝瀹冨彧浼氫负鏄惧紡浣嶄簬 `Documentation/{subdir}/.../*.rst` 鏂囦欢涓殑鍐呭鐢熸垚 man 鎵嬪唽椤点€?
