
## x86 特定ELF 辅助向量


本文档描x86 辅助向量的语义
## 简

ELF 辅助向量使内核能够高效地向用户空间提供特定于配置的参数。在此示例中一个程```

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
## 暴露的辅助向

AT_SYSINFO 用于定位 vsyscall 入口点。它64 位模式下不导出
AT_SYSINFO_EHDR 是包vDSO 的页面起始地址
AT_MINSIGSTKSZ 表示内核向用户空间递送信号所需的最小栈大小。AT_MINSIGSTKSZ
包含内核为适应当前硬件配置的用户上下文所消耗的空间。它不包含随后的用户空间
栈消耗，这部分必须由用户添加。（例如上例中，用户空间AT_MINSIGSTKSZ 加上
SIGSTKSZ。）
