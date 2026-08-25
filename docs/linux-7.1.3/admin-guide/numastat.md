## NUMA 策略命中/未命中统

/sys/devices/system/node/node*/numastat

所有单位均为页。大页（Hugepages）有独立的计数器
numa_hit、numa_miss numa_foreign 计数器反映了进程能够从它偏好的节点上分配内存的良好程度。如果成功，则在偏好节点上递增
numa_hit；否则在偏好节点上递增 numa_foreign，并在实际分配成功的
节点上递增 numa_miss
通常偏好节点是进程所运行CPU 所在的本地节点，但诸如内存策略
（mempolicies）之类的限制会改变这一点，因此还有两个基于 CPU 本地
节点的计数器。local_node 类似numa_hit，在由同一节点上的 CPU
从该节点分配时递增。other_node 类似numa_miss，在由不同节点的
CPU 从该节点分配成功时递增。注意没有与 numa_foreign 对应的计数器
更详细地
=============== ============================================================
numa_hit	进程希望从该节点分配内存，并且成功了
numa_miss	进程希望从另一个节点分配内存，但最终从该节点获得了内存
numa_foreign	进程希望在该节点上分配，但最终从另一个节点获得了内存
local_node	进程运行在该节点CPU 上，并从该节点获得了内存
other_node	进程运行在另一个节点的 CPU 上，并从该节点获得了内存
interleave_hit 	交错分配希望从该节点分配，并且成功了=============== ============================================================

为便于阅读，你可以使numactl 软件包中numastat 工具
（http://oss.sgi.com/projects/libnuma/）。注意该工具目前仅在
CPU 数量较少的机器上工作良好
注意，在带有无内存节点（节点拥有 CPU 但没有内存）的系统上numa_hit、numa_miss numa_foreign 统计可能会被严重扭曲。在当前内核实现中，如果进程偏好一个无内存节点（即因为它运行在其某个本CPU 上），实现实际上会将最近的、带有内存的节点之一视为偏好节点因此，这样的分配不会增加无内存节点上numa_foreign 计数器，并会
扭曲最近节点的 numa_hit、numa_miss numa_foreign 统计