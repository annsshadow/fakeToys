## 应用数据完整性（ADI

SPARC M7 处理器新增了应用数据完整性（ADI）特性ADI 允许任务在其地址空间的任意子集上设置版本标签。一旦启用了 ADI 并为任务地址空间范围设置了版本标签，处理器就会将指向这些范围内内存的指针中的标签
与应用程序先前设置的版本进行比较。只有当给定指针中的标签与应用程序设置的
标签相匹配时，才允许访问内存。如果不匹配，处理器会引发异常
为了完全启用 ADI，任务必须采取以下步骤：

1. 设置用户模式PSTATE.mcde 位。它作为任务整个地址空间的主开关，用于启用/
   禁用该任务的 ADI
2. 在与启用 ADI 的地址范围对应的任TLB 表项上设TTE.mcd 位。MMU 只会   设置TTE.mcd 位的页面检查版本标签
3. 使用 stxa 指令以及某个 MCD 专用ASI 来设置虚拟地址的版本标签。每stxa
   指令为一ADI 块大小数量的字节设置给定的标签。必须对整页重复此步骤，才能
   为整页设置标签
平台上的 ADI 块大小由 hypervisor（虚拟机监控器）在机器描述表中提供给内核Hypervisor 还提供虚拟地址中用于指定版本标签的高位位数。一旦为某个内存位置
设置了版本标签，该标签就存储在物理内存中，并且在呈现MMU 的虚拟地址ADI
版本标签位中必须存在相同的标签。例如，SPARC M7 处理器上，MMU 使用63-60
作为版本标签，ADI 块大小与缓存行大小相同，64 字节。一个在一段内存上ADI
版本设置为（例如0 的任务，必须使用63-60 位中包含 0xa 的虚拟地址来访该内存
ADI 通过带有 PROT_ADI 标志mprotect() 在一组页面上启用。当任务首次在一组页上启ADI 时，内核为该任务设置 PSTATE.mcde 位。内存地址的版本标签通过 stxa
指令、使用地址上的 ASI_MCD_PRIMARY ASI_MCD_ST_BLKINIT_PRIMARY 来设置。ADI 大小hypervisor 提供给内核。内核通过辅助向量（auxiliary vector）将 ADI 块大的值与其他 ADI 信息一起返回给用户空间。内核提供以下辅助向量：

	============	===========================================
	AT_ADI_BLKSZ	ADI 块大小。这ADI 版本控制的粒度和
			对齐方式，以字节为单位	AT_ADI_NBITS	VA ADI 版本位的位数
	============	===========================================


## 重要说明


- 0x0 0xf 的版本标签值被保留。这些值匹配虚拟地址中的任何标签，永远不  产生不匹配异常
- 版本标签是在用户空间对虚拟地址设置的，尽管标签存储在物理内存中。标签是  物理页面分配给任务并为其创建pte 之后，在该物理页面上设置的
- 当任务释放它曾设置过版本标签的内存页时，该页会回到空闲页池。当此页被重  分配给某个任务时，内核使用块初始ASI 清除该页，同时也清除了该页的版本
  标签。如果一个分配给任务的页被释放后又分配回同一个任务，该任务之前在该页  设置的旧版本标签将不再存在
- 对于非故障加载（non-faulting loads），不会检测到 ADI 标签不匹配
- 内核不会为用户页设置任何标签，设置任何版本标签完全是任务自己的责任。内  确实会确保：如果一个页被换出到磁盘再换入，版本标签会被保留；如果页被迁移，
  版本标签也会被保留
- ADI 适用于任意大小的页面。用户空间任务在使用 ADI 时不需要知道页面大小。它
  只需选择一个虚拟地址范围，使mprotect() 在该范围上启ADI，并为整个范  设置版本标签。mprotect() 确保范围按页面大小对齐且是页面大小的整数倍
- ADI 标签只能设置在可写内存上。例如，ADI 标签不能设置在只读映射上

## ADI 相关的陷

启用 ADI 后，可能会发生以下新的陷阱：

### Disrupting memory corruption（破坏性内存损坏）


	当一次存储访问一个具TTE.mcd=1 的内存位置、任务正在以 ADI 启用状	运行（PSTATE.mcde=1）、且所用地址中的 ADI 标签（位 63:60）与相应缓存	上设置的标签不匹配时，就会发生内存损坏陷阱。默认情况下，它是一个破坏	陷阱，首先被发送给 hypervisor。Hypervisor 创建一sun4v 错误报告，并	内核发送一个可恢复错误（TT=0x7e）陷阱。内核向导致此陷阱的任务发送一	SIGSEGV，其内容为以```

		siginfo.si_signo = SIGSEGV;
		siginfo.errno = 0;
		siginfo.si_code = SEGV_ADIDERR;
		siginfo.si_addr = addr; /* 首次发生不匹配的 PC */
		siginfo.si_trapno = 0;


```
### Precise memory corruption（精确内存损坏）


	当一次存储访问一个具TTE.mcd=1 的内存位置、任务正在以 ADI 启用状	运行（PSTATE.mcde=1）、且所用地址中的 ADI 标签（位 63:60）与相应缓存	上设置的标签不匹配时，就会发生内存损坏陷阱。如果启用了 MCD 精确异常
	（MCDPERR=1），则会向内核发送一个精确异常，TT=0x1a。内核向导致此陷阱的
	任务发送一SIGSEGV，其内容为以```

		siginfo.si_signo = SIGSEGV;
		siginfo.errno = 0;
		siginfo.si_code = SEGV_ADIPERR;
		siginfo.si_addr = addr;	/* 引发陷阱的地址 */
		siginfo.si_trapno = 0;

	注意		对加载的 ADI 标签不匹配总是导致精确陷阱

```
### MCD disabled（MCD 已禁用）


	当任务尚未启ADI 却尝试在内存地址上设ADI 版本时，处理器会发送一	MCD 已禁用陷阱。此陷阱首先hypervisor 处理，hypervisor 通过将该陷阱
	向量化到内核，作为故障类型设置为 0xa（无ASI）的数据访问异常陷阱。当
	发生这种情况时，内核
```

		siginfo.si_signo = SIGSEGV;
		siginfo.errno = 0;
		siginfo.si_code = SEGV_ACCADI;
		siginfo.si_addr = addr;	/* 引发陷阱的地址 */
		siginfo.si_trapno = 0;


```
### Sample program to use ADI（使ADI 的示例程序）


以下示例程序旨在说明如何使用 ADI
```

  #include <unistd.h>
  #include <stdio.h>
  #include <stdlib.h>
  #include <elf.h>
  #include <sys/ipc.h>
  #include <sys/shm.h>
  #include <sys/mman.h>
  #include <asm/asi.h>

  #ifndef AT_ADI_BLKSZ
  #define AT_ADI_BLKSZ	48
  #endif
  #ifndef AT_ADI_NBITS
  #define AT_ADI_NBITS	49
  #endif

  #ifndef PROT_ADI
  #define PROT_ADI	0x10
  #endif

  #define BUFFER_SIZE     32*1024*1024UL

  main(int argc, char* argv[], char* envp[])
  {
          unsigned long i, mcde, adi_blksz, adi_nbits;
          char *shmaddr, *tmp_addr, *end, *veraddr, *clraddr;
          int shmid, version;
	Elf64_auxv_t *auxv;

	adi_blksz = 0;

	while(*envp++ != NULL);
	for (auxv = (Elf64_auxv_t *)envp; auxv->a_type != AT_NULL; auxv++) {
		switch (auxv->a_type) {
		case AT_ADI_BLKSZ:
			adi_blksz = auxv->a_un.a_val;
			break;
		case AT_ADI_NBITS:
			adi_nbits = auxv->a_un.a_val;
			break;
		}
	}
	if (adi_blksz == 0) {
		fprintf(stderr, "Oops! ADI is not supported\n");
		exit(1);
	}

	printf("ADI capabilities:\n");
	printf("\tBlock size = %ld\n", adi_blksz);
	printf("\tNumber of bits = %ld\n", adi_nbits);

          if ((shmid = shmget(2, BUFFER_SIZE,
                                  IPC_CREAT | SHM_R | SHM_W)) < 0) {
                  perror("shmget failed");
                  exit(1);
          }

          shmaddr = shmat(shmid, NULL, 0);
          if (shmaddr == (char *)-1) {
                  perror("shm attach failed");
                  shmctl(shmid, IPC_RMID, NULL);
                  exit(1);
          }

	if (mprotect(shmaddr, BUFFER_SIZE, PROT_READ|PROT_WRITE|PROT_ADI)) {
		perror("mprotect failed");
		goto err_out;
	}

          /* shm 段上设置 ADI 版本标签
           */
          version = 10;
          tmp_addr = shmaddr;
          end = shmaddr + BUFFER_SIZE;
          while (tmp_addr < end) {
                  asm volatile(
                          "stxa %1, [%0]0x90\n\t"
                          :
                          : "r" (tmp_addr), "r" (version));
                  tmp_addr += adi_blksz;
          }
	asm volatile("membar #Sync\n\t");

          /* 通过将版本标签放在高adi_nbits 位中	 * 由普通地址创建一个带版本的地址
           */
          tmp_addr = (void *) ((unsigned long)shmaddr << adi_nbits);
          tmp_addr = (void *) ((unsigned long)tmp_addr >> adi_nbits);
          veraddr = (void *) (((unsigned long)version << (64-adi_nbits))
                          | (unsigned long)tmp_addr);

          printf("Starting the writes:\n");
          for (i = 0; i < BUFFER_SIZE; i++) {
                  veraddr[i] = (char)(i);
                  if (!(i % (1024 * 1024)))
                          printf(".");
          }
          printf("\n");

          printf("Verifying data...");
	fflush(stdout);
          for (i = 0; i < BUFFER_SIZE; i++)
                  if (veraddr[i] != (char)i)
                          printf("\nIndex %lu mismatched\n", i);
          printf("Done.\n");

          /* 禁用 ADI 并清           */
	if (mprotect(shmaddr, BUFFER_SIZE, PROT_READ|PROT_WRITE)) {
		perror("mprotect failed");
		goto err_out;
	}

          if (shmdt((const void *)shmaddr) != 0)
                  perror("Detach failure");
          shmctl(shmid, IPC_RMID, NULL);

          exit(0);

  err_out:
          if (shmdt((const void *)shmaddr) != 0)
                  perror("Detach failure");
          shmctl(shmid, IPC_RMID, NULL);
          exit(1);
  }

```
