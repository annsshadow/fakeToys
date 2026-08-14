## vlocks 鐢ㄤ簬瑁告満浜掓枼


鎶曠エ閿侊紙Voting Locks锛屾垨鈥渧locks鈥濓級鎻愪緵浜嗕竴绉嶇畝鍗曠殑搴曞眰浜掓枼鏈哄埗锛屽鍐呭瓨绯荤粺鐨勮姹傚悎鐞嗕笖鏈€灏忋€?

杩欎簺閿佹棬鍦ㄧ敤浜庡崗璋冮偅浜涙湰韬笉鍏蜂竴鑷存€х殑 CPU 涔嬮棿鐨勫叧閿椿鍔紝閫傜敤浜庣‖浠朵笉鎻愪緵鍏朵粬鏀寔鏈哄埗銆佷笖鏃犳硶浣跨敤鏅€氳嚜鏃嬮攣鐨勬儏鍐点€?


vlocks 鍒╃敤浜嗗唴瀛樼郴缁熷鍐欏叆鍗曚竴鍐呭瓨浣嶇疆鐨勫師瀛愭€с€備负浜嗕徊瑁侊紝姣忎釜 CPU 閫氳繃鍚戜竴涓叕鍏卞唴瀛樹綅缃瓨鍌ㄤ竴涓敮涓€缂栧彿鏉モ€滀负鑷繁鎶曠エ鈥濄€傚綋鎵€鏈夋姇绁ㄩ兘瀹屾垚涔嬪悗锛岃鍐呭瓨浣嶇疆涓墍鐪嬪埌鐨勬渶缁堝€肩‘瀹氫簡鑳滆€呫€?

涓轰簡纭繚閫変妇鑳藉湪鏈夐檺鏃堕棿鍐呬骇鐢熸槑纭殑缁撴灉锛屽彧鏈夊綋灏氭湭閫夊嚭鑳滆€呬笖閫変妇鐪嬭捣鏉ュ皻鏈紑濮嬫椂锛孋PU 鎵嶄細棣栧厛杩涘叆閫変妇銆?


### Algorithm


```


	int currently_voting[NR_CPUS] = { 0, };
	int last_vote = -1; /* no votes yet */

	bool vlock_trylock(int this_cpu)
	{
		/* signal our desire to vote */
		currently_voting[this_cpu] = 1;
		if (last_vote != -1) {
			/* someone already volunteered himself */
			currently_voting[this_cpu] = 0;
			return false; /* not ourself */
		}

		/* let's suggest ourself */
		last_vote = this_cpu;
		currently_voting[this_cpu] = 0;

		/* then wait until everyone else is done voting */
		for_each_cpu(i) {
			while (currently_voting[i] != 0)
				/* wait */;
		}

		/* result */
		if (last_vote == this_cpu)
			return true; /* we won */
		return false;
	}

	bool vlock_unlock(void)
	{
		last_vote = -1;
	}


```
currently_voting[] 鏁扮粍涓哄悇 CPU 鎻愪緵浜嗕竴绉嶅垽鏂€変妇鏄惁姝ｅ湪杩涜鐨勬柟寮忥紝鍏朵綔鐢ㄧ被浼间簬 Lamport 闈㈠寘搴楃畻娉?[^1^] 涓殑鈥渆ntering鈥濇暟缁勩€?

鐒惰€岋紝涓€鏃﹂€変妇寮€濮嬶紝搴曞眰鐨勩€佺敱鍐呭瓨绯荤粺鎻愪緵鐨勫師瀛愭€у氨琚敤鏉ユ寫閫夎儨鑰呫€傝繖閬垮厤浜嗛渶瑕佷竴涓潤鎬佷紭鍏堢骇瑙勫垯浣滀负鍐宠儨鏈哄埗锛屼篃閬垮厤浜嗕换浣曞彲鑳芥孩鍑虹殑璁℃暟鍣ㄣ€?

鍙 last_vote 鍙橀噺瀵规墍鏈?CPU 鍏ㄥ眬鍙锛屽畠灏卞彧浼氬寘鍚竴涓€硷紝骞朵笖鍦ㄦ瘡涓?CPU 閮芥竻闄や簡鍏?currently_voting 鏍囧織涔嬪墠涓嶄細鏀瑰彉銆?

```

	/* first level: local election */
	my_town = towns[(this_cpu >> 4) & 0xf];
	I_won = vlock_trylock(my_town, this_cpu & 0xf);
	if (I_won) {
		/* we won the town election, let's go for the state */
		my_state = states[(this_cpu >> 8) & 0xf];
		I_won = vlock_lock(my_state, this_cpu & 0xf));
		if (I_won) {
			/* and so on */
			I_won = vlock_lock(the_whole_country, this_cpu & 0xf];
			if (I_won) {
				/* ... */
			}
			vlock_unlock(the_whole_country);
		}
		vlock_unlock(my_state);
	}
	vlock_unlock(my_town);


```
### ARM 瀹炵幇


褰撳墠鐨?ARM 瀹炵幇 [^2^] 鍦ㄥ熀纭€绠楁硶涔嬪杩樺寘鍚竴浜涗紭鍖栵細

 - 閫氳繃灏?currently_voting 鏁扮粍鐨勬垚鍛樼揣鍑戝湴鎺掑湪涓€璧凤紝鎴戜滑鍙互鍦ㄤ竴娆′簨鍔′腑璇诲彇鏁翠釜鏁扮粍锛堝墠鎻愭槸鍙兘绔炰簤璇ラ攣鐨?CPU 鏁伴噺瓒冲灏忥級銆傝繖鍑忓皯浜嗚闂閮ㄥ唴瀛樻墍闇€鐨勫線杩旀鏁般€?

   鍦?ARM 瀹炵幇涓紝杩欐剰鍛崇潃鎴戜滑鍙互浣跨敤涓€娆″姞杞?

```

	LDR	Rt, [Rn]
	CMP	Rt, #0

   ...in place of code equivalent to::

	LDRB	Rt, [Rn]
	CMP	Rt, #0
	LDRBEQ	Rt, [Rn, #1]
	CMPEQ	Rt, #0
	LDRBEQ	Rt, [Rn, #2]
	CMPEQ	Rt, #0
	LDRBEQ	Rt, [Rn, #3]
	CMPEQ	Rt, #0

   This cuts down on the fast-path latency, as well as potentially
   reducing bus contention in contended cases.

   The optimisation relies on the fact that the ARM memory system
   guarantees coherency between overlapping memory accesses of
   different sizes, similarly to many other architectures.  Note that
   we do not care which element of currently_voting appears in which
   bits of Rt, so there is no need to worry about endianness in this
   optimisation.

   If there are too many CPUs to read the currently_voting array in
   one transaction then multiple transactions are still required.  The
   implementation uses a simple loop of word-sized loads for this
   case.  The number of transactions is still fewer than would be
   required if bytes were loaded individually.


   In principle, we could aggregate further by using LDRD or LDM, but
   to keep the code simple this was not attempted in the initial
   implementation.


 * vlocks are currently only used to coordinate between CPUs which are
   unable to enable their caches yet.  This means that the
   implementation removes many of the barriers which would be required
   when executing the algorithm in cached memory.

   packing of the currently_voting array does not work with cached
   memory unless all CPUs contending the lock are cache-coherent, due
   to cache writebacks from one CPU clobbering values written by other
   CPUs.  (Though if all the CPUs are cache-coherent, you should be
   probably be using proper spinlocks instead anyway).


 * The "no votes yet" value used for the last_vote variable is 0 (not
   -1 as in the pseudocode).  This allows statically-allocated vlocks
   to be implicitly initialised to an unlocked state simply by putting
   them in .bss.

   An offset is added to each CPU's ID for the purpose of setting this
   variable, so that no CPU uses the value 0 for its ID.


```
### Colophon


鏈€鍒濈敱 Dave Martin 涓?Linaro Limited 鍒涘缓骞惰褰曪紝鐢ㄤ簬鍩轰簬 ARM 鐨?big.LITTLE 骞冲彴锛屽苟鎰熸縺鍦版帴鍙椾簡鏉ヨ嚜 Nicolas Pitre 涓?Achin Gupta 鐨勫闃呬笌鎰忚銆傛劅璋?Nicolas 浠庣浉鍏抽偖浠惰璁轰覆涓彁鍙栦簡澶ч儴鍒嗘枃鏈苟缂栧啓浜嗕吉浠ｇ爜銆?

Copyright (C) 2012-2013  Linaro Limited
渚濇嵁 linux/COPYING 涓畾涔夌殑 GNU General Public License 绗?2 鐗堢殑鏉℃鍒嗗彂銆?


### References


[^1^] Lamport, L. "A New Solution of Dijkstra's Concurrent Programming
    Problem", Communications of the ACM 17, 8 (August 1974), 453-455.

    https://en.wikipedia.org/wiki/Lamport%27s_bakery_algorithm

[^2^] linux/arch/arm/common/vlock.S, www.kernel.org.
