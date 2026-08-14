## 闈欐€侀敭



   宸插純鐢ㄧ殑 API锛?

   鐩存帴浣跨敤 'struct static_key' 鐜板凡寮冪敤銆傛澶?
```

	struct static_key false = STATIC_KEY_INIT_FALSE;
	struct static_key true = STATIC_KEY_INIT_TRUE;
	static_key_true()
	static_key_false()

   The updated API replacements are::

	DEFINE_STATIC_KEY_TRUE(key);
	DEFINE_STATIC_KEY_FALSE(key);
	DEFINE_STATIC_KEY_ARRAY_TRUE(keys, count);
	DEFINE_STATIC_KEY_ARRAY_FALSE(keys, count);
	static_branch_likely()
	static_branch_unlikely()

```
## 鎽樿


闈欐€侀敭鍏佽閫氳繃 GCC 鐗规€у拰涓€娈典唬鐮侊紝灏嗗緢灏戜娇鐢ㄧ殑鐗规€у寘鍚湪鎬ц兘鏁忔劅鐨勫揩閫熻矾寰勫唴鏍镐唬鐮佷腑銆?
```

	DEFINE_STATIC_KEY_FALSE(key);

	...

        if (static_branch_unlikely(&key))
                do unlikely code
        else
                do likely code

	...
	static_branch_enable(&key);
	...
	static_branch_disable(&key);
	...

```
static_branch_unlikely() 鍒嗘敮浼氳鐢熸垚涓轰唬鐮侊紝瀵瑰彲鑳芥墽琛岀殑浠ｇ爜璺緞褰卞搷灏藉彲鑳藉皬銆?


## 鍔ㄦ満



鐩墠锛岃窡韪偣鏄€氳繃鏉′欢鍒嗘敮瀹炵幇鐨勩€傝鏉′欢妫€鏌ラ渶瑕佸姣忎釜璺熻釜鐐规鏌ヤ竴涓叏灞€鍙橀噺銆傚敖绠℃妫€鏌ョ殑寮€閿€寰堝皬锛屼絾褰撳唴瀛樼紦瀛樻壙鍙楀帇鍔涙椂浼氬澶э紙杩欎簺鍏ㄥ眬鍙橀噺鐨勫唴瀛樼紦瀛樿鍙兘涓庡叾浠栧唴瀛樿闂叡浜級銆傞殢鐫€鍐呮牳涓窡韪偣鏁伴噺鐨勫鍔狅紝杩欎竴寮€閿€鍙兘浼氭垚涓烘洿澶х殑闂銆傛澶栵紝璺熻釜鐐归€氬父鏄紤鐪犵殑锛堣绂佺敤锛変笖涓嶆彁渚涚洿鎺ョ殑鍐呮牳鍔熻兘銆傚洜姝わ紝灏藉彲鑳介檷浣庡畠浠殑褰卞搷鏄潪甯稿彲鍙栫殑銆傚敖绠¤窡韪偣鏄椤瑰伐浣滄渶鍒濈殑鍔ㄦ満锛屽叾浠栧唴鏍镐唬鐮佽矾寰勪篃搴斿綋鑳藉鍒╃敤闈欐€侀敭鏈哄埗銆?


## 瑙ｅ喅鏂规



gcc锛坴4.5锛夋柊澧炰簡涓€鏉?'asm goto' 璇彞锛屽厑璁歌烦杞埌鏍囩锛?

https://gcc.gnu.org/ml/gcc-patches/2009-07/msg01556.html

鍊熷姪 'asm goto'锛屾垜浠彲浠ュ垱寤洪粯璁よ涔堟墽琛屻€佽涔堜笉鎵ц鐨勮烦杞垎鏀紝鑰屾棤闇€妫€鏌ュ唴瀛樸€傞殢鍚庯紝鍦ㄨ繍琛屾椂鎴戜滑鍙互淇ˉ鍒嗘敮鐐规潵鏀瑰彉鍒嗘敮鏂瑰悜銆?

```

	if (static_branch_unlikely(&key))
		printk("I am the true branch\n");

```
鍥犳锛岄粯璁ゆ儏鍐典笅涓嶄細鍙戝嚭 'printk'銆傜敓鎴愮殑浠ｇ爜灏嗙敱鐩寸嚎浠ｇ爜璺緞涓殑鍗曟潯鍘熷瓙 'no-op' 鎸囦护锛坸86 涓婁负 5 瀛楄妭锛夌粍鎴愩€傚綋鍒嗘敮琚€滅炕杞€濇椂锛屾垜浠細鐢ㄨ烦杞埌绂荤嚎鏉′欢鐪熷垎鏀殑 'jump' 鎸囦护鏉ヤ慨琛ョ洿绾夸唬鐮佽矾寰勪腑鐨?'no-op'銆傚洜姝わ紝鏀瑰彉鍒嗘敮鏂瑰悜浠ｄ环楂樻槀锛屼絾鍒嗘敮閫夋嫨鍩烘湰涓婃槸鈥滃厤璐光€濈殑銆傝繖灏辨槸璇ヤ紭鍖栫殑鍩烘湰鏉冭　銆?

杩欎竴搴曞眰淇ˉ鏈哄埗琚О涓?'jump label patching'锛堣烦杞爣绛句慨琛ワ級锛屽畠鏋勬垚浜嗛潤鎬侀敭鏈哄埗鐨勫熀纭€銆?

## 闈欐€侀敭鏍囩 API銆佺敤娉曚笌绀轰緥



```

	DEFINE_STATIC_KEY_TRUE(key);

```
```

	DEFINE_STATIC_KEY_FALSE(key);


```
璇ラ敭蹇呴』鏄叏灞€鐨勶紝涔熷氨鏄锛屽畠涓嶈兘鍦ㄦ爤涓婂垎閰嶏紝涔熶笉鑳藉湪杩愯鏃跺姩鎬佸垎閰嶃€?

```

        if (static_branch_unlikely(&key))
                do unlikely code
        else
                do likely code

```
```

        if (static_branch_likely(&key))
                do likely code
        else
                do unlikely code

```
閫氳繃 DEFINE_STATIC_KEY_TRUE() 鎴?DEFINE_STATIC_KEY_FALSE 瀹氫箟鐨勯敭锛屽彲鐢ㄤ簬 static_branch_likely() 鎴?static_branch_unlikely() 璇彞涓€?

```

	static_branch_enable(&key);

```
```

	static_branch_disable(&key);

```
```

	static_branch_inc(&key);
	...
	static_branch_dec(&key);

```
'static_branch_inc()' 琛ㄧず鈥滀娇鍒嗘敮涓虹湡鈥濓紝'static_branch_dec()' 琛ㄧず鈥滀娇鍒嗘敮涓哄亣鈥濓紝骞跺甫鏈夌浉搴旂殑寮曠敤璁℃暟銆備緥濡傦紝濡傛灉閿垵濮嬪寲涓虹湡锛屽垯 static_branch_dec() 浼氬皢鍒嗘敮鍒囨崲涓哄亣锛涢殢鍚庣殑 static_branch_inc() 浼氬皢鍒嗘敮閲嶆柊鏀逛负鐪熴€傜被浼煎湴锛屽鏋滈敭鍒濆鍖栦负鍋囷紝鍒?'static_branch_inc()' 浼氬皢鍒嗘敮鏀逛负鐪燂紱鐒跺悗 'static_branch_dec()' 浼氬啀娆′娇鍒嗘敮涓哄亣銆?

鍙互浣跨敤 'static_key_enabled()' 鍜?'static_key_count()' 鑾峰彇鐘舵€佸拰寮曠敤璁℃暟銆備竴鑸潵璇达紝濡傛灉浣跨敤杩欎簺鍑芥暟锛屽簲褰撶敤涓?enable/disable 鎴?increment/decrement 鍑芥暟鍛ㄥ洿鐩稿悓鐨勪簰鏂ラ攣鍔犱互淇濇姢銆?

娉ㄦ剰锛屽垏鎹㈠垎鏀細瀵艰嚧鑾峰彇涓€浜涢攣锛岀壒鍒槸 CPU 鐑彃鎷旈攣锛堜互閬垮厤鍦ㄤ慨琛ュ唴鏍告椂 CPU 琚帴鍏ュ唴鏍歌€屼骇鐢熺珵浜夛級銆傚洜姝わ紝鍦ㄧ儹鎻掓嫈閫氱煡鍣ㄤ腑璋冪敤闈欐€侀敭 API 娉ㄥ畾浼氬鑷存閿併€備负浜嗕粛鐒跺厑璁镐娇鐢ㄨ鍔熻兘锛屾彁渚涗簡浠ヤ笅鍑芥暟锛?

	static_key_enable_cpuslocked()
	static_key_disable_cpuslocked()
	static_branch_enable_cpuslocked()
	static_branch_disable_cpuslocked()

杩欎簺鍑芥暟**骞堕潪**閫氱敤鐩殑锛屽繀椤讳笖浠呭綋鍦ㄧ‘瀹炲浜庝笂杩颁笂涓嬫枃銆佷笖娌℃湁鍏跺畠涓婁笅鏂囨椂浣跨敤銆?

```

	DEFINE_STATIC_KEY_ARRAY_TRUE(keys, count);

```
```

	DEFINE_STATIC_KEY_ARRAY_FALSE(keys, count);

```
4) 鏋舵瀯绾т唬鐮佷慨琛ユ帴鍙ｏ紝'jump labels'锛堣烦杞爣绛撅級


涓轰簡鍒╃敤杩欎竴浼樺寲锛屾灦鏋勫繀椤诲疄鐜拌嫢骞插嚱鏁板拰瀹忋€傚鏋滄病鏈夋灦鏋勬敮鎸侊紝鎴戜滑浼氱畝鍗曞湴鍥為€€鍒颁紶缁熺殑鈥滃姞杞姐€佹祴璇曘€佽烦杞€濆簭鍒椼€傛澶栵紝struct jump_entry 琛ㄥ繀椤昏嚦灏?4 瀛楄妭瀵归綈锛屽洜涓?static_key->entry 瀛楁浣跨敤浜嗘渶浣庝袱浣嶃€?

- `select HAVE_ARCH_JUMP_LABEL`锛屽弬瑙侊細arch/x86/Kconfig

- `#define JUMP_LABEL_NOP_SIZE`锛屽弬瑙侊細arch/x86/include/asm/jump_label.h

- `__always_inline bool arch_static_branch(struct static_key *key, bool branch)`锛屽弬瑙侊細arch/x86/include/asm/jump_label.h

- `__always_inline bool arch_static_branch_jump(struct static_key *key, bool branch)`锛屽弬瑙侊細arch/x86/include/asm/jump_label.h

- `void arch_jump_label_transform(struct jump_entry *entry, enum jump_label_type type)`锛屽弬瑙侊細arch/x86/kernel/jump_label.c

- `struct jump_entry`锛屽弬瑙侊細arch/x86/include/asm/jump_label.h


5) 闈欐€侀敭 / 璺宠浆鏍囩鍒嗘瀽锛岀粨鏋滐紙x86_64锛夛細


浣滀负绀轰緥锛屾垜浠湪 'getppid()' 涓坊鍔犲涓嬪垎鏀紝浣垮緱
```

  SYSCALL_DEFINE0(getppid)
  {
        int pid;

  +     if (static_branch_unlikely(&key))
  +             printk("I am the true branch\n");

        rcu_read_lock();
        pid = task_tgid_vnr(rcu_dereference(current->real_parent));
        rcu_read_unlock();

        return pid;
  }

```
```

  ffffffff81044290 <sys_getppid>:
  ffffffff81044290:       55                      push   %rbp
  ffffffff81044291:       48 89 e5                mov    %rsp,%rbp
  ffffffff81044294:       e9 00 00 00 00          jmpq   ffffffff81044299 <sys_getppid+0x9>
  ffffffff81044299:       65 48 8b 04 25 c0 b6    mov    %gs:0xb6c0,%rax
  ffffffff810442a0:       00 00
  ffffffff810442a2:       48 8b 80 80 02 00 00    mov    0x280(%rax),%rax
  ffffffff810442a9:       48 8b 80 b0 02 00 00    mov    0x2b0(%rax),%rax
  ffffffff810442b0:       48 8b b8 e8 02 00 00    mov    0x2e8(%rax),%rdi
  ffffffff810442b7:       e8 f4 d9 00 00          callq  ffffffff81051cb0 <pid_vnr>
  ffffffff810442bc:       5d                      pop    %rbp
  ffffffff810442bd:       48 98                   cltq
  ffffffff810442bf:       c3                      retq
  ffffffff810442c0:       48 c7 c7 e3 54 98 81    mov    $0xffffffff819854e3,%rdi
  ffffffff810442c7:       31 c0                   xor    %eax,%eax
  ffffffff810442c9:       e8 71 13 6d 00          callq  ffffffff8171563f <printk>
  ffffffff810442ce:       eb c9                   jmp    ffffffff81044299 <sys_getppid+0x9>

```
```

  ffffffff810441f0 <sys_getppid>:
  ffffffff810441f0:       8b 05 8a 52 d8 00       mov    0xd8528a(%rip),%eax        # ffffffff81dc9480 <key>
  ffffffff810441f6:       55                      push   %rbp
  ffffffff810441f7:       48 89 e5                mov    %rsp,%rbp
  ffffffff810441fa:       85 c0                   test   %eax,%eax
  ffffffff810441fc:       75 27                   jne    ffffffff81044225 <sys_getppid+0x35>
  ffffffff810441fe:       65 48 8b 04 25 c0 b6    mov    %gs:0xb6c0,%rax
  ffffffff81044205:       00 00
  ffffffff81044207:       48 8b 80 80 02 00 00    mov    0x280(%rax),%rax
  ffffffff8104420e:       48 8b 80 b0 02 00 00    mov    0x2b0(%rax),%rax
  ffffffff81044215:       48 8b b8 e8 02 00 00    mov    0x2e8(%rax),%rdi
  ffffffff8104421c:       e8 2f da 00 00          callq  ffffffff81051c50 <pid_vnr>
  ffffffff81044221:       5d                      pop    %rbp
  ffffffff81044222:       48 98                   cltq
  ffffffff81044224:       c3                      retq
  ffffffff81044225:       48 c7 c7 13 53 98 81    mov    $0xffffffff81985313,%rdi
  ffffffff8104422c:       31 c0                   xor    %eax,%eax
  ffffffff8104422e:       e8 60 0f 6d 00          callq  ffffffff81715193 <printk>
  ffffffff81044233:       eb c9                   jmp    ffffffff810441fe <sys_getppid+0xe>
  ffffffff81044235:       66 66 2e 0f 1f 84 00    data32 nopw %cs:0x0(%rax,%rax,1)
  ffffffff8104423c:       00 00 00 00

```
鍥犳锛岀鐢ㄨ烦杞爣绛剧殑鎯呭喌浼氬鍔犱竴鏉?'mov'銆?test' 鍜?'jne' 鎸囦护锛岃€岃烦杞爣绛炬儏鍐靛彧鏈変竴鏉?'no-op' 鎴?'jmp 0'銆傦紙jmp 0 鍦ㄥ惎鍔ㄦ椂琚慨琛ヤ负 5 瀛楄妭鐨勫師瀛?no-op 鎸囦护銆傦級鍥犳锛岃绂佺敤鐨勮烦杞?
```

  6 (mov) + 2 (test) + 2 (jne) = 10 - 5 (5 byte jump 0) = 5 addition bytes.

```
濡傛灉鎴戜滑鍐嶈鍏ュ～鍏呭瓧鑺傦紝璺宠浆鏍囩浠ｇ爜涓鸿繖涓皬鍑芥暟鑺傜渷浜嗘€昏 16 瀛楄妭鐨勬寚浠ゅ唴瀛樸€傚湪鏈緥涓紝闈炶烦杞爣绛惧嚱鏁伴暱 80 瀛楄妭銆傚洜姝わ紝鎴戜滑鑺傜渷浜?20% 鐨勬寚浠ゅ崰鐢ㄣ€備簨瀹炰笂鎴戜滑杩樿兘杩涗竴姝ユ敼杩涳紝鍥犱负 5 瀛楄妭 no-op 瀹為檯涓婂彲浠ユ槸 2 瀛楄妭 no-op锛屽洜涓烘垜浠彲浠ョ敤 2 瀛楄妭 jmp 鍒拌揪鍒嗘敮銆備笉杩囷紝鎴戜滑灏氭湭瀹炵幇鏈€浼樼殑 no-op 澶у皬锛堢洰鍓嶆槸纭紪鐮佺殑锛夈€?

鐢变簬璋冨害鍣ㄨ矾寰勪腑鏈夊澶勪娇鐢ㄩ潤鎬侀敭 API锛屽彲浠ヤ娇鐢?'pipe-test'锛堜篃绉颁负 'perf bench sched pipe'锛夋潵灞曠ず鎬ц兘鎻愬崌銆傚湪 3.3.0-rc2 涓婂畬鎴愮殑娴嬭瘯锛?

```

 Performance counter stats for 'bash -c /tmp/pipe-test' (50 runs):

        855.700314 task-clock                #    0.534 CPUs utilized            ( +-  0.11% )
           200,003 context-switches          #    0.234 M/sec                    ( +-  0.00% )
                 0 CPU-migrations            #    0.000 M/sec                    ( +- 39.58% )
               487 page-faults               #    0.001 M/sec                    ( +-  0.02% )
     1,474,374,262 cycles                    #    1.723 GHz                      ( +-  0.17% )
   <not supported> stalled-cycles-frontend
   <not supported> stalled-cycles-backend
     1,178,049,567 instructions              #    0.80  insns per cycle          ( +-  0.06% )
       208,368,926 branches                  #  243.507 M/sec                    ( +-  0.06% )
         5,569,188 branch-misses             #    2.67% of all branches          ( +-  0.54% )

       1.601607384 seconds time elapsed                                          ( +-  0.07% )

```
```

 Performance counter stats for 'bash -c /tmp/pipe-test' (50 runs):

        841.043185 task-clock                #    0.533 CPUs utilized            ( +-  0.12% )
           200,004 context-switches          #    0.238 M/sec                    ( +-  0.00% )
                 0 CPU-migrations            #    0.000 M/sec                    ( +- 40.87% )
               487 page-faults               #    0.001 M/sec                    ( +-  0.05% )
     1,432,559,428 cycles                    #    1.703 GHz                      ( +-  0.18% )
   <not supported> stalled-cycles-frontend
   <not supported> stalled-cycles-backend
     1,175,363,994 instructions              #    0.82  insns per cycle          ( +-  0.04% )
       206,859,359 branches                  #  245.956 M/sec                    ( +-  0.04% )
         4,884,119 branch-misses             #    2.36% of all branches          ( +-  0.85% )

       1.579384366 seconds time elapsed

```
鑺傜渷鐨勫垎鏀櫨鍒嗘瘮涓?0.7%锛屽苟涓斿湪 'branch-misses'锛堝垎鏀娴嬪け璐ワ級涓婅妭鐪佷簡 12%銆傝繖姝ｆ槸鎴戜滑鏈熸湜鑾峰緱鏈€澶氳妭鐪佺殑鍦版柟锛屽洜涓鸿浼樺寲鏃ㄥ湪鍑忓皯鍒嗘敮鏁伴噺銆傛澶栵紝鎴戜滑鍦ㄦ寚浠や笂鑺傜渷浜?0.2%锛屽湪鍛ㄦ湡涓婅妭鐪佷簡 2.8%锛屽湪鑰楁椂涓婅妭鐪佷簡 1.4%銆?
