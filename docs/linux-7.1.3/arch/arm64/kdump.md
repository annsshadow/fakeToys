## arm64 上的 crashkernel 内存预留


Author: Baoquan He <bhe@redhat.com>

Kdump 机制用于捕获发生损坏的内核 vmcore（内存转储），以便后续进行分析。
为此，需要预先预留一段内存，用于在发生损坏时预加载 kdump 内核并启动
该内核。

为 kdump 预留的内存被调整为能够最小程度地容纳 kdump 内核以及用于
vmcore 收集所需的用户空间程序。

## 内核参数


通过以下内核参数，可以在第一内核启动的早期阶段相应地预留内存，
从而找到一段连续的大块内存。如果 crashkernel 是从高端内存区域预留的，
则还需要考虑低端内存的预留。

- crashkernel=size@offset
- crashkernel=size
- crashkernel=size,high crashkernel=size,low

## 低端内存与高端内存


对于 kdump 预留而言，低端内存是指特定限制以下的memory区域，该限制
通常由 kdump 内核运行所需、具备 DMA 能力的设备所能访问的地址位数决定。
那些与 vmcore 转储无关的设备可以忽略。在 arm64 上，低端内存的上限并不
固定：在 RPi4 平台上是 1G，而在大多数其他系统上是 4G。在禁用了
CONFIG_ZONE_(DMA|DMA32) 的特殊内核构建中，整个系统 RAM 都属于低端内存。
除上文描述的低端内存之外，系统 RAM 的其余部分被视为高端内存。

## 实现


### 1) crashkernel=size@offset


crashkernel 内存必须预留到用户指定的区域，如果已被占用则失败。


### 2) crashkernel=size


crashkernel 内存区域将按照以下搜索顺序预留到任意可用位置：

首先，内核在低内存区域搜索一块指定大小的可用区域。

如果在低内存中搜索失败，内核会回退到在高内存区域搜索一块指定大小的
可用区域。如果高内存预留成功，则会在低内存中执行一次默认大小的预留。
目前默认大小为 128M，足以满足 kdump 内核的低内存需求。

注意：crashkernel=size 是 crashkernel 内存预留的推荐选项。用户无需
了解特定平台的系统内存布局。

### 3) crashkernel=size,high crashkernel=size,low


crashkernel=size,(high|low) 是对 crashkernel=size 的重要补充。它们允许
用户分别指定需要从高端内存和低端内存分配多少内存。在许多系统上，
低端内存十分宝贵，应当尽量从该区域进行最少的 crashkernel 预留。

为 crashkernel=size,high 预留内存时，首先尝试从高内存区域搜索。如果
预留成功，则随后会进行低内存的预留。

如果从高内存预留失败，内核会回退到使用 crashkernel=,high 中指定的大小
在低内存中搜索。如果成功，则无需再为低内存进行预留。

注意：

- 如果未指定 crashkernel=,low，则会自动执行默认的低内存预留。

- 如果指定了 crashkernel=0,low，则表示有意省略低内存预留。
