
## x86 鐗瑰畾鐨?ELF 杈呭姪鍚戦噺


鏈枃妗ｆ弿杩?x86 杈呭姪鍚戦噺鐨勮涔夈€?
## 绠€浠?

ELF 杈呭姪鍚戦噺浣垮唴鏍歌兘澶熼珮鏁堝湴鍚戠敤鎴风┖闂存彁渚涚壒瀹氫簬閰嶇疆鐨勫弬鏁般€傚湪姝ょず渚嬩腑锛?涓€涓▼搴?```

   #include <sys/auxv.h>
   #include <elf.h>
   #include <signal.h>
   #include <stdlib.h>
   #include <assert.h>
   #include <err.h>

   #ifndef AT_MINSIGSTKSZ
   #define AT_MINSIGSTKSZ	51
   #endif

   ....
   stack_t ss;

   ss.ss_sp = malloc(ss.ss_size);
   assert(ss.ss_sp);

   ss.ss_size = getauxval(AT_MINSIGSTKSZ) + SIGSTKSZ;
   ss.ss_flags = 0;

   if (sigaltstack(&ss, NULL))
        err(1, "sigaltstack");


```
## 鏆撮湶鐨勮緟鍔╁悜閲?

AT_SYSINFO 鐢ㄤ簬瀹氫綅 vsyscall 鍏ュ彛鐐广€傚畠鍦?64 浣嶆ā寮忎笅涓嶅鍑恒€?
AT_SYSINFO_EHDR 鏄寘鍚?vDSO 鐨勯〉闈㈣捣濮嬪湴鍧€銆?
AT_MINSIGSTKSZ 琛ㄧず鍐呮牳鍚戠敤鎴风┖闂撮€掗€佷俊鍙锋墍闇€鐨勬渶灏忔爤澶у皬銆侫T_MINSIGSTKSZ
鍖呭惈鍐呮牳涓洪€傚簲褰撳墠纭欢閰嶇疆鐨勭敤鎴蜂笂涓嬫枃鎵€娑堣€楃殑绌洪棿銆傚畠涓嶅寘鍚殢鍚庣殑鐢ㄦ埛绌洪棿
鏍堟秷鑰楋紝杩欓儴鍒嗗繀椤荤敱鐢ㄦ埛娣诲姞銆傦紙渚嬪涓婁緥涓紝鐢ㄦ埛绌洪棿鍚?AT_MINSIGSTKSZ 鍔犱笂
SIGSTKSZ銆傦級
