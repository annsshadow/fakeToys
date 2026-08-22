## io_mapping 函数


## API


linux/io-mapping.h 中的 io_mapping 函数提供了一种抽象，用于高效地将 I/O 设备的小块区域映射到 CPU。其最初用途是支持 32 位处理器上较大的图形 aperture，因为在这些处理器上无法使用 ioremap_wc 将整aperture 静态映射到 CPU（那样会消耗过多的内核地址空间）
```

	struct io_mapping *io_mapping_create_wc(unsigned long base,
						unsigned long size)

```

'base' 是要使其可映射的区域的总线地址，'size' 表示要启用的映射区域大小。两者均以字节为单位
_wc 变体提供的映射只能与 io_mapping_map_atomic_wc()、io_mapping_map_local_wc() io_mapping_map_wc() 一起使用
借助此映射对象，可以根据需求以临时或长期方式映射单个页。当然，临时映射
```

	void *io_mapping_map_local_wc(struct io_mapping *mapping,
				      unsigned long offset)

	void *io_mapping_map_atomic_wc(struct io_mapping *mapping,
				       unsigned long offset)

```

'offset' 是所定义映射区域内的偏移量。访问创建函数中指定区域之外的地址会产生未定义的结果。使用未按页对齐的偏移量也会产生未定义的结果。返回值指CPU 地址空间中的单个页
_wc 变体会返回该页的一个写入合并（write-combining）映射，且只能用于由 io_mapping_create_wc() 创建的映射
临时映射仅在调用者的上下文中有效。该映射不保证对所CPU 全局可见
io_mapping_map_local_wc() X86 32 位上有副作用：它会禁用迁移以使映射代码正常工作。任何调用者都不得依赖这一副作用
io_mapping_map_atomic_wc() 的副作用是禁用抢占（preemption）和缺页（pagefaults）。不要在新代码中使用它，请改io_mapping_map_local_wc()
嵌套映射必须以相反顺序撤销，因为映
```

 addr1 = io_mapping_map_local_wc(map1, offset1);
 addr2 = io_mapping_map_local_wc(map2, offset2);
 ...
 io_mapping_unmap_local(addr2);
 io_mapping_unmap_local(addr1);

```

```

	void io_mapping_unmap_local(void *vaddr)
	void io_mapping_unmap_atomic(void *vaddr)

```

'vaddr' 必须是最后一io_mapping_map_local_wc() io_mapping_map_atomic_wc() 调用返回的值。这会取消映射指定的映射，并撤销映射函数的副作用
如果你在持有一个映射期间需要睡眠，可以使用常规
```

	void *io_mapping_map_wc(struct io_mapping *mapping,
				unsigned long offset)

```

其工作方式类似于 io_mapping_map_atomic/local_wc()，只是没有副作用，且指针全局可见
```

	void io_mapping_unmap(void *vaddr)

```

用于解除io_mapping_map_wc() 映射的页
```

	void io_mapping_free(struct io_mapping *mapping)

```
