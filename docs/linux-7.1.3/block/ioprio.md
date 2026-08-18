## 鍧楄澶?IO 浼樺厛绾?

### 绠€浠?

io 浼樺厛绾х壒鎬т娇鐢ㄦ埛鑳藉瀵硅繘绋嬫垨杩涚▼缁勮繘琛?io nice 璁剧疆锛岀被浼间簬闀挎湡浠ユ潵瀵?cpu 璋冨害鎵€鍙兘鍋氬埌鐨勪簨鎯呫€?瀵?io 浼樺厛绾х殑鏀寔鍙栧喅浜?io 璋冨害鍣紝鐩墠鐢?bfq 鍜?mq-deadline 鏀寔銆?
### 璋冨害绫?

涓?io 浼樺厛绾у疄鐜颁簡涓変釜閫氱敤鐨勮皟搴︾被锛屽畠浠喅瀹氫簡涓€涓繘绋嬬殑 io 濡備綍琚湇鍔°€?
IOPRIO_CLASS_RT锛氳繖鏄疄鏃?io 绫汇€傛璋冨害绫昏璧嬩簣姣旂郴缁熶腑浠讳綍鍏朵粬绫绘洿楂樼殑浼樺厛绾э紝鏉ヨ嚜姝ょ被鐨勮繘绋嬫瘡娆￠兘
浼樺厛璁块棶纾佺洏銆傚洜姝や娇鐢ㄥ畠闇€瑕佷竴浜涜皑鎱庯紝涓€涓?io RT 杩涚▼鍙兘璁╂暣涓郴缁熼タ姝汇€傚湪 RT 绫诲唴閮紝鏈?8 涓骇鍒殑
绫绘暟鎹紝鐢ㄤ簬绮剧‘鍐冲畾璇ヨ繘绋嬫瘡娆℃湇鍔￠渶瑕佸灏戠鐩樻椂闂淬€傚皢鏉ヨ繖鍙兘浼氭敼鍙樹负鏇村彲鐩存帴鏄犲皠鍒版€ц兘锛岄€氳繃浼犲叆涓€涓?鏈熸湜鐨勬暟鎹€熺巼鏉ヤ唬鏇裤€?
IOPRIO_CLASS_BE锛氳繖鏄敖鍔涜€屼负锛坆est-effort锛夎皟搴︾被锛屾槸浠讳綍鏈缃壒瀹?io 浼樺厛绾х殑杩涚▼鐨勯粯璁ょ被銆傜被鏁版嵁
鍐冲畾璇ヨ繘绋嬪皢鑾峰緱澶氬皯 io 甯﹀锛屽畠鍙洿鎺ユ槧灏勫埌 cpu nice 绾у埆锛屽彧鏄疄鐜板緱鏇寸矖鐣ャ€? 鏄渶楂?BE 浼樺厛绾х骇鍒紝
7 鏄渶浣庣骇鍒€俢pu nice 绾у埆涓?io nice 绾у埆涔嬮棿鐨勬槧灏勭‘瀹氫负锛歩o_nice = (cpu_nice + 20) / 5銆?
IOPRIO_CLASS_IDLE锛氳繖鏄┖闂茶皟搴︾被锛岃繍琛屽湪姝ょ骇鍒殑杩涚▼浠呭綋娌℃湁鍏朵粬浠讳綍浜洪渶瑕佺鐩樻椂鎵嶄細鑾峰緱 io 鏃堕棿銆?绌洪棽绫绘病鏈夌被鏁版嵁锛屽洜涓哄湪杩欓噷瀹冪‘瀹炰笉閫傜敤銆?
### 宸ュ叿


```

	# ionice -c<class> -n<level> -p<pid>

```
濡傛灉鏈粰瀹?pid锛屽垯鍋囧畾涓哄綋鍓嶈繘绋嬨€侷O 浼樺厛绾ц缃湪 fork 鏃惰缁ф壙锛屽洜姝や綘鍙互浣跨敤 ionice 鍦ㄧ粰瀹?```

	# ionice -c2 -n0 /bin/ls

```
涓嬪惎鍔ㄨ繘绋嬶紝灏嗕互鏈€楂樹紭鍏堢骇鐨勫敖鍔涜€屼负璋冨害绫昏繍琛?ls銆?```

	# ionice -c1 -n2 -p100

```
浼氬皢 pid 100 鏇存敼涓轰互瀹炴椂璋冨害绫汇€佷紭鍏堢骇 2 杩愯銆?
```

  #include <stdio.h>
  #include <stdlib.h>
  #include <errno.h>
  #include <getopt.h>
  #include <unistd.h>
  #include <sys/ptrace.h>
  #include <asm/unistd.h>

  extern int sys_ioprio_set(int, int, int);
  extern int sys_ioprio_get(int, int);

  #if defined(__i386__)
  #define __NR_ioprio_set		289
  #define __NR_ioprio_get		290
  #elif defined(__ppc__)
  #define __NR_ioprio_set		273
  #define __NR_ioprio_get		274
  #elif defined(__x86_64__)
  #define __NR_ioprio_set		251
  #define __NR_ioprio_get		252
  #else
  #error "Unsupported arch"
  #endif

  static inline int ioprio_set(int which, int who, int ioprio)
  {
	return syscall(__NR_ioprio_set, which, who, ioprio);
  }

  static inline int ioprio_get(int which, int who)
  {
	return syscall(__NR_ioprio_get, which, who);
  }

  enum {
	IOPRIO_CLASS_NONE,
	IOPRIO_CLASS_RT,
	IOPRIO_CLASS_BE,
	IOPRIO_CLASS_IDLE,
  };

  enum {
	IOPRIO_WHO_PROCESS = 1,
	IOPRIO_WHO_PGRP,
	IOPRIO_WHO_USER,
  };

  #define IOPRIO_CLASS_SHIFT	13

  const char *to_prio[] = { "none", "realtime", "best-effort", "idle", };

  int main(int argc, char *argv[])
  {
	int ioprio = 4, set = 0, ioprio_class = IOPRIO_CLASS_BE;
	int c, pid = 0;

	while ((c = getopt(argc, argv, "+n:c:p:")) != EOF) {
		switch (c) {
		case 'n':
			ioprio = strtol(optarg, NULL, 10);
			set = 1;
			break;
		case 'c':
			ioprio_class = strtol(optarg, NULL, 10);
			set = 1;
			break;
		case 'p':
			pid = strtol(optarg, NULL, 10);
			break;
		}
	}

	switch (ioprio_class) {
		case IOPRIO_CLASS_NONE:
			ioprio_class = IOPRIO_CLASS_BE;
			break;
		case IOPRIO_CLASS_RT:
		case IOPRIO_CLASS_BE:
			break;
		case IOPRIO_CLASS_IDLE:
			ioprio = 7;
			break;
		default:
			printf("bad prio class %d\n", ioprio_class);
			return 1;
	}

	if (!set) {
		if (!pid && argv[optind])
			pid = strtol(argv[optind], NULL, 10);

		ioprio = ioprio_get(IOPRIO_WHO_PROCESS, pid);

		printf("pid=%d, %d\n", pid, ioprio);

		if (ioprio == -1)
			perror("ioprio_get");
		else {
			ioprio_class = ioprio >> IOPRIO_CLASS_SHIFT;
			ioprio = ioprio & 0xff;
			printf("%s: prio %d\n", to_prio[ioprio_class], ioprio);
		}
	} else {
		if (ioprio_set(IOPRIO_WHO_PROCESS, pid, ioprio | ioprio_class << IOPRIO_CLASS_SHIFT) == -1) {
			perror("ioprio_set");
			return 1;
		}

		if (argv[optind])
			execvp(argv[optind], &argv[optind]);
	}

	return 0;
  }


```
March 11 2005, Jens Axboe <jens.axboe@oracle.com>
