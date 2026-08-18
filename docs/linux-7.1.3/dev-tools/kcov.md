## KCOV锛氱敤浜庢ā绯婃祴璇曠殑瑕嗙洊鐜囷紙code coverage锛?

KCOV 鏀堕泦骞朵互閫傚悎瑕嗙洊鐜囧紩瀵兼ā绯婃祴璇曠殑褰㈠紡鏆撮湶鍐呮牳浠ｇ爜瑕嗙洊鐜囦俊鎭€傝繍琛屼腑鍐呮牳鐨勮鐩栫巼鏁版嵁閫氳繃 `kcov` debugfs 鏂囦欢瀵煎嚭銆傝鐩栫巼鏀堕泦鏄寜浠诲姟鍚敤鐨勶紝鍥犳 KCOV 鍙互鎹曡幏鍗曟绯荤粺璋冪敤鐨勭簿纭鐩栫巼銆?
娉ㄦ剰锛孠COV 鐨勭洰鏍囧苟闈炴敹闆嗗敖鍙兘澶氱殑瑕嗙洊鐜囥€傚畠鐨勭洰鏍囨槸鏀堕泦澶ц嚧绋冲畾鐨勩€佷綔涓虹郴缁熻皟鐢ㄨ緭鍏ュ嚱鏁扮殑瑕嗙洊鐜囥€備负浜嗗疄鐜拌繖涓€鐩爣锛屽畠涓嶄細鍦ㄨ蒋/纭腑鏂腑鏀堕泦瑕嗙洊鐜囷紙闄ら潪鍚敤浜嗚繙绋嬭鐩栫巼鏀堕泦锛岃涓嬫枃锛夛紝涔熶笉浼氫粠鍐呮牳涓竴浜涙湰璐ㄤ笂闈炵‘瀹氭€х殑閮ㄥ垎锛堜緥濡傝皟搴﹀櫒銆侀攣锛夋敹闆嗐€?
闄や簡鏀堕泦浠ｇ爜瑕嗙洊鐜囷紝KCOV 杩樺彲浠ユ敹闆嗘瘮杈冩搷浣滄暟銆傝瑙?"Comparison operands collection" 涓€鑺傘€?
闄や簡浠庣郴缁熻皟鐢ㄥ鐞嗙▼搴忔敹闆嗚鐩栫巼鏁版嵁锛孠COV 杩樺彲浠ヤ负鍦ㄥ唴鏍稿悗鍙颁换鍔℃垨杞腑鏂腑鎵ц鐨勫唴鏍稿凡娉ㄨВ閮ㄥ垎鏀堕泦瑕嗙洊鐜囥€傝瑙?"Remote coverage collection" 涓€鑺傘€?
### 鍏堝喅鏉′欢


KCOV 渚濊禆缂栬瘧鍣ㄦ彃妗╋紝闇€瑕?GCC 6.1.0 鎴栨洿楂樼増鏈紝鎴栬€呭唴鏍告敮鎸佺殑浠绘剰 Clang 鐗堟湰銆?
鏀堕泦姣旇緝鎿嶄綔鏁板彈 GCC 8+ 鎴?Clang 鏀寔銆?
```

        CONFIG_KCOV=y

```

```

	CONFIG_KCOV_ENABLE_COMPARISONS=y

```

```

        mount -t debugfs none /sys/kernel/debug

```
### 瑕嗙洊鐜囨敹闆?

浠ヤ笅绋嬪簭婕旂ず浜嗗浣曞湪娴嬭瘯绋嬪簭涓娇鐢?KCOV 涓哄崟娆＄郴缁熻皟鐢ㄦ敹闆嗚鐩栫巼锛?
```

    #include <stdio.h>
    #include <stddef.h>
    #include <stdint.h>
    #include <stdlib.h>
    #include <sys/types.h>
    #include <sys/stat.h>
    #include <sys/ioctl.h>
    #include <sys/mman.h>
    #include <unistd.h>
    #include <fcntl.h>
    #include <linux/types.h>

    #define KCOV_INIT_TRACE			_IOR('c', 1, unsigned long)
    #define KCOV_ENABLE			_IO('c', 100)
    #define KCOV_DISABLE			_IO('c', 101)
    #define COVER_SIZE			(64<<10)

    #define KCOV_TRACE_PC  0
    #define KCOV_TRACE_CMP 1

    int main(int argc, char **argv)
    {
	int fd;
	unsigned long *cover, n, i;

	/* A single fd descriptor allows coverage collection on a single
  - thread.
	 */
	fd = open("/sys/kernel/debug/kcov", O_RDWR);
	if (fd == -1)
		perror("open"), exit(1);
	/** Setup trace mode and trace size. **/
	if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
		perror("ioctl"), exit(1);
	/** Mmap buffer shared between kernel- and user-space. **/
	cover = (unsigned long**)mmap(NULL, COVER_SIZE ** sizeof(unsigned long),
				     PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if ((void*)cover == MAP_FAILED)
		perror("mmap"), exit(1);
	/** Enable coverage collection on the current thread. **/
	if (ioctl(fd, KCOV_ENABLE, KCOV_TRACE_PC))
		perror("ioctl"), exit(1);
	/** Reset coverage from the tail of the ioctl() call. **/
	__atomic_store_n(&cover[^0^], 0, __ATOMIC_RELAXED);
	/** Call the target syscall call. **/
	read(-1, NULL, 0);
	/** Read number of PCs collected. **/
	n = __atomic_load_n(&cover[^0^], __ATOMIC_RELAXED);
	for (i = 0; i < n; i++)
		printf("0x%lx\n", cover[i + 1]);
	/* Disable coverage collection for the current thread. After this call
  - coverage can be enabled for a different thread.
	 */
	if (ioctl(fd, KCOV_DISABLE, 0))
		perror("ioctl"), exit(1);
	/** Free resources. **/
	if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
		perror("munmap"), exit(1);
	if (close(fd))
		perror("close"), exit(1);
	return 0;
    }

```

```

    SyS_read
    fs/read_write.c:562
    __fdget_pos
    fs/file.c:774
    __fget_light
    fs/file.c:746
    __fget_light
    fs/file.c:750
    __fget_light
    fs/file.c:760
    __fdget_pos
    fs/file.c:784
    SyS_read
    fs/read_write.c:562

```
濡傛灉绋嬪簭闇€瑕佷粠澶氫釜绾跨▼锛堝悇鑷嫭绔嬪湴锛夋敹闆嗚鐩栫巼锛屽垯闇€瑕佸湪姣忎釜绾跨▼涓垎鍒墦寮€ `/sys/kernel/debug/kcov`銆?
璇ユ帴鍙ｆ槸缁嗙矑搴︾殑锛屼互渚块珮鏁堝湴 fork 娴嬭瘯杩涚▼銆備篃灏辨槸璇达紝鐖惰繘绋嬫墦寮€ `/sys/kernel/debug/kcov`銆佸惎鐢?trace 妯″紡銆乵map 瑕嗙洊鐜囩紦鍐插尯锛岀劧鍚庡湪寰幆涓?fork 瀛愯繘绋嬨€傚瓙杩涚▼鍙渶瑕佸惎鐢ㄨ鐩栫巼锛堝綋绾跨▼閫€鍑烘椂瀹冧細鑷姩绂佺敤锛夈€?
### 姣旇緝鎿嶄綔鏁版敹闆?

姣旇緝鎿嶄綔鏁扮殑鏀堕泦涓庤鐩栫巼鏀堕泦绫讳技锛?
```

    /** Same includes and defines as above. **/

    /** Number of 64-bit words per record. **/
    #define KCOV_WORDS_PER_CMP 4

    /*
     - The format for the types of collected comparisons.
     *
     - Bit 0 shows whether one of the arguments is a compile-time constant.
     - Bits 1 & 2 contain log2 of the argument size, up to 8 bytes.
     */

    #define KCOV_CMP_CONST          (1 << 0)
    #define KCOV_CMP_SIZE(n)        ((n) << 1)
    #define KCOV_CMP_MASK           KCOV_CMP_SIZE(3)

    int main(int argc, char **argv)
    {
	int fd;
	uint64_t *cover, type, arg1, arg2, is_const, size;
	unsigned long n, i;

	fd = open("/sys/kernel/debug/kcov", O_RDWR);
	if (fd == -1)
		perror("open"), exit(1);
	if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
		perror("ioctl"), exit(1);
	/*
 - Note that the buffer pointer is of type uint64_t*, because all
 - the comparison operands are promoted to uint64_t.
	*/
	cover = (uint64_t **)mmap(NULL, COVER_SIZE ** sizeof(unsigned long),
				     PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if ((void*)cover == MAP_FAILED)
		perror("mmap"), exit(1);
	/** Note KCOV_TRACE_CMP instead of KCOV_TRACE_PC. **/
	if (ioctl(fd, KCOV_ENABLE, KCOV_TRACE_CMP))
		perror("ioctl"), exit(1);
	__atomic_store_n(&cover[^0^], 0, __ATOMIC_RELAXED);
	read(-1, NULL, 0);
	/** Read number of comparisons collected. **/
	n = __atomic_load_n(&cover[^0^], __ATOMIC_RELAXED);
	for (i = 0; i < n; i++) {
		uint64_t ip;

		type = cover[i * KCOV_WORDS_PER_CMP + 1];
		/** arg1 and arg2 - operands of the comparison. **/
		arg1 = cover[i * KCOV_WORDS_PER_CMP + 2];
		arg2 = cover[i * KCOV_WORDS_PER_CMP + 3];
		/** ip - caller address. **/
		ip = cover[i * KCOV_WORDS_PER_CMP + 4];
		/** size of the operands. **/
		size = 1 << ((type & KCOV_CMP_MASK) >> 1);
		/** is_const - true if either operand is a compile-time constant.**/
		is_const = type & KCOV_CMP_CONST;
		printf("ip: 0x%lx type: 0x%lx, arg1: 0x%lx, arg2: 0x%lx, "
			"size: %lu, %s\n",
			ip, type, arg1, arg2, size,
		is_const ? "const" : "non-const");
	}
	if (ioctl(fd, KCOV_DISABLE, 0))
		perror("ioctl"), exit(1);
	/** Free resources. **/
	if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
		perror("munmap"), exit(1);
	if (close(fd))
		perror("close"), exit(1);
	return 0;
    }

```
娉ㄦ剰锛孠COV 鐨勫悇妯″紡锛堟敹闆嗕唬鐮佽鐩栫巼鎴栨瘮杈冩搷浣滄暟锛夋槸浜掓枼鐨勩€?
### 杩滅▼瑕嗙洊鐜囨敹闆?

闄や簡浠庣敤鎴风┖闂磋繘绋嬪彂璧风殑绯荤粺璋冪敤澶勭悊绋嬪簭鏀堕泦瑕嗙洊鐜囨暟鎹紝KCOV 杩樺彲浠ヤ负鍦ㄥ叾浠栦笂涓嬫枃涓墽琛岀殑鍐呮牳閮ㄥ垎鏀堕泦瑕嗙洊鐜団€斺€斿嵆鎵€璋撶殑"杩滅▼"瑕嗙洊鐜囥€?
浣跨敤 KCOV 鏀堕泦杩滅▼瑕嗙洊鐜囬渶瑕侊細

1. 淇敼鍐呮牳浠ｇ爜锛岀敤 `kcov_remote_start` 鍜?`kcov_remote_stop` 娉ㄨВ搴斿綋浠庝腑鏀堕泦瑕嗙洊鐜囩殑浠ｇ爜娈点€?
2. 鍦ㄦ敹闆嗚鐩栫巼鐨勭敤鎴风┖闂磋繘绋嬩腑浣跨敤 `KCOV_REMOTE_ENABLE` 浠ｆ浛 `KCOV_ENABLE`銆?
`kcov_remote_start` 鍜?`kcov_remote_stop` 娉ㄨВ浠ュ強 `KCOV_REMOTE_ENABLE` ioctl 閮芥帴鍙楃敤浜庢爣璇嗙壒瀹氳鐩栫巼鏀堕泦娈电殑鍙ユ焺銆傚彞鏌勭殑浣跨敤鏂瑰紡鍙栧喅浜庡尮閰嶄唬鐮佹鎵ц鐨勪笂涓嬫枃銆?
KCOV 鏀寔浠庝互涓嬩笂涓嬫枃鏀堕泦杩滅▼瑕嗙洊鐜囷細

1. 鍏ㄥ眬鍐呮牳鍚庡彴浠诲姟銆傝繖浜涙槸鍦ㄥ唴鏍稿惎鍔ㄦ湡闂寸敓鎴愩€佸疄渚嬫暟閲忔湁闄愮殑浠诲姟锛堜緥濡傛瘡涓?USB HCD 鐢熸垚涓€涓?USB `hub_event` worker锛夈€?
2. 鏈湴鍐呮牳鍚庡彴浠诲姟銆傝繖浜涙槸鍦ㄧ敤鎴风┖闂磋繘绋嬩笌鏌愪簺鍐呮牳鎺ュ彛浜や簰鏃剁敓鎴愩€侀€氬父鍦ㄨ杩涚▼閫€鍑烘椂琚潃鎺夌殑浠诲姟锛堜緥濡?vhost workers锛夈€?
3. 杞腑鏂€?
瀵逛簬 #1 鍜?#3锛屽繀椤婚€夋嫨涓€涓敮涓€鐨勫叏灞€鍙ユ焺骞朵紶閫掔粰鐩稿簲鐨?`kcov_remote_start` 璋冪敤銆傜劧鍚庣敤鎴风┖闂磋繘绋嬪繀椤诲皢璇ュ彞鏌勯€氳繃 `kcov_remote_arg` 缁撴瀯浣撶殑 `handles` 鏁扮粍瀛楁浼犻€掔粰 `KCOV_REMOTE_ENABLE`銆傝繖浼氬皢鎵€浣跨敤鐨?KCOV 璁惧闄勫姞鍒拌鍙ユ焺鎵€寮曠敤鐨勪唬鐮佹銆傚彲浠ュ悓鏃朵紶閫掓爣璇嗕笉鍚屼唬鐮佹鐨勫涓叏灞€鍙ユ焺銆?
瀵逛簬 #2锛岀敤鎴风┖闂磋繘绋嬪繀椤婚€氳繃 `kcov_remote_arg` 缁撴瀯浣撶殑 `common_handle` 瀛楁浼犻€掍竴涓潪闆跺彞鏌勩€傝鍏叡鍙ユ焺浼氳淇濆瓨鍒板綋鍓?`task_struct` 鐨?`kcov_handle` 瀛楁涓紝骞朵笖闇€瑕侀€氳繃鑷畾涔夌殑鍐呮牳浠ｇ爜淇敼浼犻€掔粰鏂扮敓鎴愮殑鏈湴浠诲姟銆傝繖浜涗换鍔″弽杩囨潵搴斿綋鍦ㄥ畠浠殑 `kcov_remote_start` 鍜?`kcov_remote_stop` 娉ㄨВ涓娇鐢ㄦ墍浼犻€掔殑鍙ユ焺銆?
KCOV 瀵瑰叏灞€鍙ユ焺鍜屽叕鍏卞彞鏌勯兘閬靛惊棰勫畾涔夋牸寮忋€傛瘡涓彞鏌勬槸涓€涓?`u64` 鏁存暟銆傜洰鍓嶅彧浣跨敤浜嗘渶楂樺瓧鑺傚拰杈冧綆鐨?4 瀛楄妭銆傚瓧鑺?4-7 淇濈暀锛屽繀椤讳负闆躲€?
瀵逛簬鍏ㄥ眬鍙ユ焺锛屽彞鏌勭殑鏈€楂樺瓧鑺傝〃绀哄畠鎵€灞炲瓙绯荤粺鐨?id銆備緥濡傦紝KCOV 浣跨敤 `1` 浣滀负 USB 瀛愮郴缁?id銆傚叏灞€鍙ユ焺杈冧綆鐨?4 瀛楄妭琛ㄧず璇ョ郴缁熷唴浠诲姟瀹炰緥鐨?id銆備緥濡傦紝姣忎釜 `hub_event` worker 浣跨敤 USB 鎬荤嚎鍙蜂綔涓轰换鍔″疄渚?id銆?
瀵逛簬鍏叡鍙ユ焺锛屼繚鐣欏€?`0` 琚敤浣滃瓙绯荤粺 id锛屽洜涓烘绫诲彞鏌勪笉灞炰簬鏌愪釜鐗瑰畾瀛愮郴缁熴€傚叕鍏卞彞鏌勮緝浣庣殑 4 瀛楄妭鏍囪瘑鐢卞悜 `KCOV_REMOTE_ENABLE` 浼犻€掑叕鍏卞彞鏌勭殑鐢ㄦ埛绌洪棿杩涚▼鎵€鐢熸垚鐨勬墍鏈夋湰鍦颁换鍔＄殑闆嗗悎瀹炰緥銆?
鍦ㄥ疄璺典腑锛屽鏋滆鐩栫巼浠呬粠绯荤粺涓婂崟涓敤鎴风┖闂磋繘绋嬫敹闆嗭紝鍒欏叕鍏卞彞鏌勫疄渚?id 鍙互浣跨敤浠绘剰鍊笺€備絾鏄紝濡傛灉鍏叡鍙ユ焺琚涓繘绋嬩娇鐢紝鍒欏繀椤讳负姣忎釜杩涚▼浣跨敤鍞竴鐨勫疄渚?id銆備竴绉嶉€夋嫨鏄娇鐢ㄨ繘绋?id 浣滀负鍏叡鍙ユ焺瀹炰緥 id銆?
浠ヤ笅绋嬪簭婕旂ず浜嗕娇鐢?KCOV 浠庤繘绋嬬敓鎴愮殑鏈湴浠诲姟浠ュ強澶勭悊 USB 鎬荤嚎 #1 鐨勫叏灞€浠诲姟鏀堕泦瑕嗙洊鐜囷細

```

    /** Same includes and defines as above. **/

    struct kcov_remote_arg {
	__u32		trace_mode;
	__u32		area_size;
	__u32		num_handles;
	__aligned_u64	common_handle;
	__aligned_u64	handles[^0^];
    };

    #define KCOV_INIT_TRACE			_IOR('c', 1, unsigned long)
    #define KCOV_DISABLE			_IO('c', 101)
    #define KCOV_REMOTE_ENABLE		_IOW('c', 102, struct kcov_remote_arg)

    #define COVER_SIZE	(64 << 10)

    #define KCOV_TRACE_PC	0

    #define KCOV_SUBSYSTEM_COMMON	(0x00ull << 56)
    #define KCOV_SUBSYSTEM_USB	(0x01ull << 56)

    #define KCOV_SUBSYSTEM_MASK	(0xffull << 56)
    #define KCOV_INSTANCE_MASK	(0xffffffffull)

    static inline __u64 kcov_remote_handle(__u64 subsys, __u64 inst)
    {
	if (subsys & ~KCOV_SUBSYSTEM_MASK || inst & ~KCOV_INSTANCE_MASK)
		return 0;
	return subsys | inst;
    }

    #define KCOV_COMMON_ID	0x42
    #define KCOV_USB_BUS_NUM	1

    int main(int argc, char **argv)
    {
	int fd;
	unsigned long *cover, n, i;
	struct kcov_remote_arg *arg;

	fd = open("/sys/kernel/debug/kcov", O_RDWR);
	if (fd == -1)
		perror("open"), exit(1);
	if (ioctl(fd, KCOV_INIT_TRACE, COVER_SIZE))
		perror("ioctl"), exit(1);
	cover = (unsigned long**)mmap(NULL, COVER_SIZE ** sizeof(unsigned long),
				     PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0);
	if ((void*)cover == MAP_FAILED)
		perror("mmap"), exit(1);

	/** Enable coverage collection via common handle and from USB bus #1. **/
	arg = calloc(1, sizeof(*arg) + sizeof(uint64_t));
	if (!arg)
		perror("calloc"), exit(1);
	arg->trace_mode = KCOV_TRACE_PC;
	arg->area_size = COVER_SIZE;
	arg->num_handles = 1;
	arg->common_handle = kcov_remote_handle(KCOV_SUBSYSTEM_COMMON,
							KCOV_COMMON_ID);
	arg->handles[^0^] = kcov_remote_handle(KCOV_SUBSYSTEM_USB,
						KCOV_USB_BUS_NUM);
	if (ioctl(fd, KCOV_REMOTE_ENABLE, arg))
		perror("ioctl"), free(arg), exit(1);
	free(arg);

	/*
  - Here the user needs to trigger execution of a kernel code section
  - that is either annotated with the common handle, or to trigger some
  - activity on USB bus #1.
	 */
	sleep(2);

        /*
         - The load to the coverage count should be an acquire to pair with
         - pair with the corresponding write memory barrier (smp_wmb()) on
         - the kernel-side in kcov_move_area().
         */
	n = __atomic_load_n(&cover[^0^], __ATOMIC_ACQUIRE);
	for (i = 0; i < n; i++)
		printf("0x%lx\n", cover[i + 1]);
	if (ioctl(fd, KCOV_DISABLE, 0))
		perror("ioctl"), exit(1);
	if (munmap(cover, COVER_SIZE * sizeof(unsigned long)))
		perror("munmap"), exit(1);
	if (close(fd))
		perror("close"), exit(1);
	return 0;
    }

```
