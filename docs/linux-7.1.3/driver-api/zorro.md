## 为 Zorro 设备编写设备驱动


:Author: Written by Geert Uytterhoeven <geert@linux-m68k.org>
:Last revised: September 5, 2003


### 简介


Zorro 总线是 Amiga 系列计算机中使用的总线。得益于 AutoConfig(tm)，它是 100%
即插即用（Plug-and-Play）的。

Zorro 总线有两种类型，Zorro II 与 Zorro III：

  - Zorro II 地址空间是 24 位的，位于 Amiga 地址映射的前 16 MB 内。

  - Zorro III 是 Zorro II 的 32 位扩展，向后兼容 Zorro II。Zorro III 地址空间
    位于前 16 MB 之外。


### 探测 Zorro 设备


通过调用 `zorro_find_device()` 来发现 Zorro 设备，该函数返回指向具有指定 Zorro ID
的`下一个` Zorro 设备的指针。探测循环如下：

```

    struct zorro_dev *z = NULL;

    while ((z = zorro_find_device(ZORRO_PROD_xxx, z))) {
	if (!zorro_request_region(z->resource.start+MY_START, MY_SIZE,
				  "My explanation"))
	...
    }

```
`ZORRO_WILDCARD` 充当通配符，可以找到任意 Zorro 设备。如果你的驱动如下：

```

    struct zorro_dev *z = NULL;

    while ((z = zorro_find_device(ZORRO_WILDCARD, z))) {
	if (z->id != ZORRO_PROD_xxx1 && z->id != ZORRO_PROD_xxx2 && ...)
	    continue;
	if (!zorro_request_region(z->resource.start+MY_START, MY_SIZE,
				  "My explanation"))
	...
    }


```
### Zorro 资源


在你访问 Zorro 设备的寄存器之前，必须确保它尚未被使用。这是通过 I/O 内存空间
资源管理完成的：

```

    request_mem_region()
    release_mem_region()

```
```

    zorro_request_device
    zorro_release_device


```
### 访问 Zorro 地址空间


Zorro 设备资源中的地址区域是 Zorro 总线地址区域。由于 Zorro 总线上总线-物理地址的
恒等映射，它们同时也是 CPU 物理地址。

对这些区域的处理取决于 Zorro 空间的类型：

  - Zorro II 地址空间总是被映射的，不需要使用 z_ioremap() 显式映射。

    从总线/物理 Zorro II 地址到内核虚拟地址的转换：

```

	virt_addr = ZTWO_VADDR(bus_addr);
	bus_addr = ZTWO_PADDR(virt_addr);

  - Zorro III 地址空间必须先使用 z_ioremap() 显式映射，然后才能访问::

	virt_addr = z_ioremap(bus_addr, size);
	...
	z_iounmap(virt_addr);


```
### 参考资料


#. linux/include/linux/zorro.h
#. linux/include/uapi/linux/zorro.h
#. linux/include/uapi/linux/zorro_ids.h
#. linux/arch/m68k/include/asm/zorro.h
#. linux/drivers/zorro
#. /proc/bus/zorro
