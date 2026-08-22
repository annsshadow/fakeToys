
## 通用基数页表


	:doc: Generic Radix Page Table

	:doc: Generic Page Table Language

## 用法


通用 PT 被构建为一个多编译系统。由于每种格式都使用一组通用名称来提API，因此在
一个编译单元内只能有一种格式处于活动状态。这种设计避免了围绕底层 API 的函数指针
相反，函数指针可以落在更高层级的 API（即 map/unmap 等）上，而每种格式的代码可以
直接内联到该格式的编译单元中。对于类IOMMU 的情况，每种格式都会被编译进一按格式划分的 IOMMU 操作内核模块
为此，每个编译单元的 .c 文件将同时包含格式头文件和用于实现的通用代码。例如，一个实现编译单元中，头文件通常会按如下方式包含
```

	#include <linux/generic_pt/common.h>
	#include "defs_amdv1.h"
	#include "../pt_defs.h"
	#include "amdv1.h"
	#include "../pt_common.h"
	#include "../pt_iter.h"
	#include "../iommu_pt.h"  /* The IOMMU implementation */

```
iommu_pt.h 包含了将根据 AMDv1 提供的宏定义来生map/unmap 等操作的宏定义。生成的
模块将具有诸pt_iommu_amdv1_init() 这样的导出符号
有关 IOMMU 实现如何使用多编译来生成按格式划分的 ops 结构体指针，请参drivers/iommu/generic_pt/fmt/iommu_template.h 中的示例
格式代码的编写方式是，通用名称#define 映射到各格式特定的唯一名称。这旨在通过
避免所有不同格式之间的符号冲突来辅助调试
导出的符号和其他全局名称通过 NS() 辅助宏使用按格式划分的字符串进行修饰（mangle）
该格式使struct pt_common 作为表的顶层结构体，每种格式都会有自己的 struct pt_xxx
来内嵌它，以存储格式特定的信息
该实现会进一步将 struct pt_common 包装在它自己的顶层结构体中，例如
struct pt_iommu_amdv1銆。
### 位于 struct pt_common 级别的格式函

	:identifiers:

### 迭代辅助函数



### 编写一种格

最好从与目标相似的简单格式开始。x86_64 通常是简单情形的良好参考，AMDv1 则相完整
所需inline 函数需要在格式头文件中实现```

 static inline pt_oaddr_t amdv1pt_entry_oa(const struct pt_state *pts)
 {
	[..]
 }
 #define pt_entry_oa amdv1pt_entry_oa

```
其中，一个唯一命名的按格式 inline 函数提供实现，而一define 将其映射到通用名称这旨在使调试符号工作得更好。应始终使用 inline 函数，因pt_common.h 中的原型会让
编译器验证函数签名以防止错误
查看 pt_fmt_defaults.h 以了解一些可选的 inline 函数
一旦该格式编译通过，就应当让它通过通用页表
```

   $ tools/testing/kunit/kunit.py run --build_dir build_kunit_x86_64 --arch x86_64 --kunitconfig ./drivers/iommu/generic_pt/.kunitconfig amdv1_fmt_test.*
   [...]
   [11:15:08] Testing complete. Ran 9 tests: passed: 9
   [11:15:09] Elapsed time: 3.137s total, 0.001s configuring, 2.368s building, 0.311s running

```
通用测试旨在验证格式函数，并提供更清晰的失败信息以加快问题定位。一旦这些通过，就
应当运行整个 kunit 测试套件
### IOMMU 失效特

失效是页表算法如何与页表内存的硬件缓存（通常称为 TLB（对IOMMU 情形则为
IOTLB））保持同步的方式
根据设计，TLB 可以存储存在（present）的 PTE、不存在（non-present）的 PTE 以及指针。每个硬件都有自己描述哪些内容已变更、从而将已变更项TLB 中移除的方法
#### PT_FEAT_FLUSH_RANGE


PT_FEAT_FLUSH_RANGE 是最容易理解的方案。它试图为每个操作生成单个范围失效，如果
存在不需要失效的 VA 间隙，则会过度失效。它在受影响VA 范围与失效操作数量之间做
权衡。它不跟踪正在失效的内容；但是，如果必须释放页，则必须从 walk 缓存中清理页指针。该范围可以在任意页边界开结束
#### PT_FEAT_FLUSH_RANGE_NO_GAPS


PT_FEAT_FLUSH_RANGE_NO_GAPS PT_FEAT_FLUSH_RANGE 类似；但是，它通过发出额外刷新操作来最小化受影响的 VA 数量。如果处VA 的代价非常高，例如因为虚拟机监控正在使用影子算法处理页表，那么这就很有用