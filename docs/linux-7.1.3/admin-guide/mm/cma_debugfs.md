## CMA 调试接口


CMA debugfs 接口对于检索基本信息非常有用
不同的 CMA 区域并测试每个区域的分配/释放。

每个CMA区域代表<debugfs>/cma/下的一个目录，表示为
其CMA名称如下：

<调试文件系统>/cma/<cma_name>

该目录下创建的文件结构如下：

 - [RO] base_pfn：CMA 区域的基本 PFN（页帧号）。
这与 range/0/base_pfn 相同。
 - [RO] count：CMA 区域中的内存量。
 - [RO] order_per_bit：一位表示的页顺序。
 - [RO] 位图：该区域中已分配页的位图。
这与 range/0/base_pfn 相同。
 - [RO]ranges/N/base_pfn：连续范围N的基本PFN
在CMA地区。
 - [RO]ranges/N/bitmap: 中分配的页的位图
CMA 区域内的 N 范围。
```

	echo 5 > <debugfs>/cma/<cma_name>/alloc

```
会尝试从“cma_name”区域分配 5 个页面。

 - [WO] free：从该CMA区域免费N个页面，与上面类似。
