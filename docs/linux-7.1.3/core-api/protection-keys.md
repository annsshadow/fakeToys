
## 内存保护键（Memory Protection Keys


内存保护键提供了一种强制基于页的保护机制，但不需要在应用程序更改保护域时修改页表

Pkeys Userspace（PKU）是一项可以在以下平台找到的特性：
        - Intel 服务CPU，Skylake 及更
        - Intel 客户CPU，Tiger Lake（第 11 代酷睿）及更
        - 未来AMD CPU
        - 实现 Permission Overlay Extension（FEAT_S1POE）的 arm64 CPU

## x86_64


保护键的工作原理是将每个页表项中先前保留4 个比特专用于一个“保护键（protection key）”，从而得16 个可能的键

每个键的保护由每 CPU 用户可访问寄存器（PKRU）定义。每PKRU 是一32 位寄存器，为 16 个键各存储两位（访问禁止 Access Disable 和写入禁Write Disable）

作为 CPU 寄存器，PKRU 天生是线程局部的，可能使每个线程拥有与其他线程不同的保护集合

有两条指令（RDPKRU/WRPKRU）用于读写该寄存器。即PAE PTE 中理论上存在空间，该特性也仅在 64 位模式下可用。这些权限仅对数据访问强制执行，对指令读取没有影响

## arm64


保护键在每个页表项中使用 3 个比特来编码一个“保护键索引（protection key index）”，从而得8 个可能的键

每个键的保护由每 CPU 用户可写系统寄存器（POR_EL0）定义。这是一64 位寄存器，为每个保护键索引编码读、写和执行覆盖权限

作为 CPU 寄存器，POR_EL0 天生是线程局部的，可能使每个线程拥有与其他线程不同的保护集合

x86_64 不同，保护键权限也适用于指令读取

## 系统调用


```

	int pkey_alloc(unsigned long flags, unsigned long init_access_rights)
	int pkey_free(int pkey);
	int pkey_mprotect(unsigned long start, size_t len,
			  unsigned long prot, int pkey);

```
在使pkey 之前，必须先pkey_alloc() 分配它。应用程序直接写入架构相关的 CPU 寄存器，以更改由该键覆盖的内存访问权限。在本例中，这一操作被一个名pkey_set() C 函数封装
```

	int real_prot = PROT_READ|PROT_WRITE;
	pkey = pkey_alloc(0, PKEY_DISABLE_WRITE);
	ptr = mmap(NULL, PAGE_SIZE, PROT_NONE, MAP_ANONYMOUS|MAP_PRIVATE, -1, 0);
	ret = pkey_mprotect(ptr, PAGE_SIZE, real_prot, pkey);
	... 应用程序在此运行

```
现在，如果应用程序需要更'ptr' 处的数据，它可以
```

	pkey_set(pkey, 0); // 清除 PKEY_DISABLE_WRITE
	*ptr = foo; // 赋
	pkey_set(pkey, PKEY_DISABLE_WRITE); // 重新设置 PKEY_DISABLE_WRITE

```
当释放内存时，由
```

	munmap(ptr, PAGE_SIZE);
	pkey_free(pkey);

```
它也会释放该 pkey。示例实现可tools/testing/selftests/mm/pkey-{arm64,powerpc,x86}.h 中找到

## 行为


内核试图使保护键
```

	mprotect(ptr, size, PROT_NONE);
	something(ptr);

```
保持一致。无something() 是对 'ptr' 的直接访
```

	*ptr = foo;

```
还是内核代表应用程序进行访问
```

	read(fd, ptr, 1);

```
在这两种情况下内核都会发SIGSEGV，但当违反保护键si_code 会被设为 SEGV_PKERR，而当违反普mprotect() 权限时则SEGV_ACCERR

注意，来kthread（如 io_uring）的内核访问将使用保护键寄存器的默认值，因此与用户空间的寄存器值或 mprotect() 不一致
