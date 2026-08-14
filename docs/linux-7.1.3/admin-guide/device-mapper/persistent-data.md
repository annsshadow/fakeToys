## 持久化数据


## 简介


更为复杂的 device-mapper 目标需要由内核管理的复杂元数据。在 2010 年末，我们注意到各种不同的目标都在各自实现自己的数据结构，例如：

- Mikulas Patocka 的 multisnap 实现
- Heinz Mauelshagen 的精简配置（thin provisioning）目标
- 另一个发布到 dm-devel 的基于 btree 的缓存目标
- 另一个基于 Daniel Phillips 设计的多重快照目标

维护这些数据结构需要大量工作，因此如果可能的话，我们希望减少其数量。

persistent-data 库旨在为希望在 device-mapper 目标中存储元数据的人提供一个可复用的框架。它目前被精简配置目标和即将推出的分层存储目标所使用。

## 概述


主要文档位于头文件中，这些都可以在 drivers/md/persistent-data 下找到。

### 块管理器


dm-block-manager.[hc]

它提供以固定块大小访问磁盘上数据的能力。其中包含读/写锁接口，以防止并发访问，并将正在使用的数据保留在缓存中。

persistent-data 的使用者不太可能直接使用它。

### 事务管理器


dm-transaction-manager.[hc]

它限制对块的访问并强制执行写时复制（copy-on-write）语义。通过事务管理器获取可写块的唯一方式是影子化（shadowing）一个已有块（即执行写时复制）或分配一个新块。在同一事务内会省略影子化操作，因此性能尚可接受。commit 方法确保在写入超级块之前刷新所有数据。发生电源故障时，你的元数据将保持在最后一次提交时的状态。

### 空间映射（Space Maps）


dm-space-map.h
dm-space-map-metadata.[hc]
dm-space-map-disk.[hc]

用于跟踪块的引用计数的磁盘数据结构。同时充当新块的分配器。目前有两种实现：一种较简单，用于管理不同设备上的块（例如精简配置的数据块）；另一种用于管理元数据空间。后者因为需要在其所管理的空间内存储自身数据而变得复杂。

### 数据结构


dm-btree.[hc]
dm-btree-remove.c
dm-btree-spine.c
dm-btree-internal.h

目前只有一种数据结构，即分层 btree。有计划添加更多。例如，带有类似数组接口的结构将会被广泛使用。

该 btree 之所以是"分层"的，是因为你可以将其定义为由嵌套的 btree 组成，并接受多个键。例如，精简配置目标使用的 btree 具有两级嵌套。第一级将设备 id 映射到一棵映射树，该映射树进而将虚拟块映射到物理块。

存储在 btree 中的值可以具有任意大小。键始终是 64 位，尽管嵌套允许使用多个键。
