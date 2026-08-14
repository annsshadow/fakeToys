
## mseal 简介


:Author: Jeff Xu <jeffxu@chromium.org>

现代 CPU 支持内存权限，例如 RW 和 NX 位。内存权限特性改善了应对内存破坏类 bug 的安全态势，即攻击者不能仅仅写入任意内存并把代码指向它，内存必须被标记为 X 位，否则会发生异常。

内存封印（memory sealing）额外保护映射本身免遭修改。这对于缓解内存破坏问题很有用，在这种问题中，一个被篡改的指针被传递给内存管理系统。例如，这样的攻击者原语可能破坏控制流完整性保证，因为本应被信任的只读内存可能变为可写，或者 .text 页可能被重新映射。内存封印可以由运行时加载器自动应用，以封印 .text 和 .rodata 页，而应用程序还可以在运行时额外封印安全关键数据。

类似的特性在 XNU 内核中已存在，即 VM_FLAGS_PERMANENT 标志 [^1^]，在 OpenBSD 中则通过 mimmutable 系统调用 [^2^] 提供。

## 系统调用（SYSCALL）

### mseal 系统调用签名

   `int mseal(void *addr, size_t len, unsigned long flags)`

   **addr**/**len**：虚拟内存地址范围。
      **addr**/**len** 设置的地址范围必须满足：
         - 起始地址必须位于已分配的 VMA 中。
         - 起始地址必须按页对齐。
         - 结束地址（**addr** + **len**）必须位于已分配的 VMA 中。
         - 起始和结束地址之间不能有空隙（未分配内存）。

      `len` 会被内核隐式按页对齐。

   **flags**：为将来使用而保留。

   **返回值**：
      - **0**：成功。
      - **-EINVAL**：
         - 输入 `flags` 无效。
         - 起始地址（`addr`）未按页对齐。
         - 地址范围（`addr` + `len`）溢出。
      - **-ENOMEM**：
         - 起始地址（`addr`）未分配。
         - 结束地址（`addr` + `len`）未分配。
         - 起始和结束地址之间存在空隙（未分配内存）。
      - **-EPERM**：
         - 封印仅支持 64 位 CPU，不支持 32 位。

   **关于错误返回的说明**：
      - 对于上述错误情况，用户可预期给定的内存范围未被修改，即没有部分更新。
      - 可能还有其他此处未列出的内部错误/情况，例如在合并/拆分 VMA 期间出错，或者进程达到所支持的最大 VMA 数量。在这些情况下，对给定内存范围的部分更新可能发生。不过，这些情况应当很罕见。

   **架构支持**：
      mseal 仅在 64 位 CPU 上工作，不在 32 位 CPU 上工作。

   **幂等（Idempotent）**：
      用户可以多次调用 mseal。对已经封印的内存再次调用 mseal 是无操作（不会报错）。

   **没有 munseal**：
      一旦映射被封印，就无法解除封印。内核不应提供 munseal，这与其他封印特性保持一致，例如文件的 F_SEAL_SEAL。

### 被封印映射所阻止的 mm 系统调用

   有一点可能很重要：**一旦映射被封印，它将一直保留在进程的内存中，直到进程终止**。

```

         *ptr = mmap(0, 4096, PROT_READ, MAP_ANONYMOUS | MAP_PRIVATE, 0, 0);
         rc = mseal(ptr, 4096, 0);
         /* munmap 将失败 */
         rc = munmap(ptr, 4096);
         assert(rc < 0);

   Blocked mm syscall:
      - munmap
      - mmap
      - mremap
      - mprotect 和 pkey_mprotect
      - 某些破坏性的 madvise 行为：MADV_DONTNEED、MADV_FREE、
        MADV_DONTNEED_LOCKED、MADV_FREE、MADV_DONTFORK、MADV_WIPEONFORK

   第一组要阻止的系统调用是 munmap、mremap、mmap。它们要么会在地址空间中留下空位，从而允许用一组新属性的新映射替换，要么可以用另一个映射覆盖现有映射。

   mprotect 和 pkey_mprotect 被阻止，因为它们会改变映射的保护位（RWX）。

   某些破坏性的 madvise 行为，特别是 MADV_DONTNEED、MADV_FREE、MADV_DONTNEED_LOCKED 和 MADV_WIPEONFORK，在应用于匿名内存且由缺少写权限的线程执行时可能引入风险。因此，在此类条件下这些操作被禁止。上述行为有可能通过丢弃页来修改区域内容，实际上是对匿名内存执行 memset(0) 操作。

   内核会对被阻止的系统调用返回 -EPERM。

   当被阻止的系统调用因封印而返回 -EPERM 时，内存区域是否改变取决于被阻止的系统调用：

      - munmap：munmap 是原子的。如果给定范围中的某个 VMA 被封印，则没有任何 VMA 被更新。
      - mprotect、pkey_mprotect、madvise：可能发生部分更新，例如在对多个 VMA 执行 mprotect 时，mprotect 可能在到达被封印的 VMA 之前更新了开头的 VMA 并返回 -EPERM。
      - mmap 和 mremap：未定义行为。

```
## 使用场景

- glibc：
  动态链接器在加载 ELF 可执行文件时，可以对映射段应用封印。

- Chrome 浏览器：保护某些安全敏感的数据结构。

- 系统映射：
  系统映射由内核创建，包括 vdso、vvar、vvar_vclock、vectors（arm 兼容模式）、sigpage（arm 兼容模式）、uprobes。

  这些系统映射是只读或只执行的，内存封印可以保护它们永不变为可写，也不会以不同属性被 unmmap/重新映射。这可用于缓解内存破坏问题，即一个被篡改的指针传递给内存管理系统。

  如果某架构支持（CONFIG_ARCH_SUPPORTS_MSEAL_SYSTEM_MAPPINGS），CONFIG_MSEAL_SYSTEM_MAPPINGS 会封印该架构的所有系统映射。

  目前支持此特性的架构有：x86-64、arm64、loongarch 和 s390。

  警告：此特性会破坏依赖重定位或解除映射系统映射的程序。撰写本文时已知的被破坏软件包括 CHECKPOINT_RESTORE、UML、gVisor、rr。因此不能无条件启用此配置。

## 何时不应使用 mseal

应用程序可以从用户空间对任一虚拟内存区域应用封印，但在应用封印之前，**彻底分析映射的生命周期**至关重要。这是因为被封印的映射**不会被解除映射**，直到进程终止或调用 exec 系统调用。

例如：
   - aio/shm
     aio/shm 可以代表用户空间调用 mmap 和 munmap，例如 shm.c 中的 ksys_shmdt()。这些映射的生命周期与进程的生命周期并不绑定。如果这些内存从用户空间被封印，那么 munmap 将失败，导致进程生命周期内 VMA 地址空间泄漏。

   - 由 malloc 分配的 ptr（堆）
     不要对 malloc() 返回的 ptr 内存使用 mseal。
     malloc() 由分配器实现，例如 glibc。堆管理器可能从 brk 或通过 mmap 创建的映射中分配一个 ptr。
     如果应用对 malloc() 返回的 ptr 调用 mseal，这会影响堆管理器管理映射的能力；结果是不确定的。

```

        ptr = malloc(size);
        /* 不要对 malloc 返回的 ptr 调用 mseal。 */
        mseal(ptr, size);
        /* free 会成功，但分配器无法把堆收缩到低于 ptr */
        free(ptr);

```
## mseal 不阻止的情况

简而言之，mseal 阻止某些 mm 系统调用修改部分 VMA 的属性，例如保护位（RWX）。被封印的映射并不意味着内存是不可变的。

正如 Jann Horn 在 [^3^] 中指出的，仍有几种写入只读内存的方式，这在某种程度上是设计使然。而这些可以通过不同的安全措施来阻止。

这些情况包括：

   - 通过 /proc/self/mem 接口（FOLL_FORCE）写入只读内存。
   - 通过 ptrace（如 PTRACE_POKETEXT）写入只读内存。
   - userfaultfd。

启发本补丁的想法来自 Stephen Röttger 在 V8 CFI 中的工作 [^4^]。ChromeOS 中的 Chrome 浏览器将成为此 API 的第一个用户。

## 参考

- [^1^] https://github.com/apple-oss-distributions/xnu/blob/1031c584a5e37aff177559b9f69dbd3c8c3fd30a/osfmk/mach/vm_statistics.h#L274
- [^2^] https://man.openbsd.org/mimmutable.2
- [^3^] https://lore.kernel.org/lkml/CAG48ez3ShUYey+ZAFsU2i1RpQn0a5eOs2hzQ426FkcgnfUGLvA@mail.gmail.com
- [^4^] https://docs.google.com/document/d/1O2jwK4dxI3nRcOJuPYkonhTkNQfbmwdvxQMyXgeaRHo/edit#heading=h.bvaojj9fu6hc
