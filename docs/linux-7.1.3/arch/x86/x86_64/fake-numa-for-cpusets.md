
## 用于 CPUSet 的伪 NUMA


:Author: David Rientjes <rientjes@cs.washington.edu>

使用 numa=fake CPUSet 进行资源管理

本文档描述了如何结合 cpusets 使用 numa=fake x86_64 命令行选项来进行粗粒度的内存管理。使用该特性，你可以创建代表连续内存块的伪 NUMA 节点，并将它们分配给 cpusets 及其附加的任务。这是一种限制某类任务可用系统内存总量的方法
关于 cpusets 特性的更多信息，请参见 Documentation/admin-guide/cgroup-v1/cpusets.rst你可以根据你的需求使用多种不同的配置。关numa=fake 命令行选项及其配置伪节点的各种方式，请参见 Documentation/admin-guide/kernel-parameters.txt

就本简介而言，我们假设一个非常原始的 NUMA 仿真设置 "numa=fake=4*512,"。这将把系统内存拆分为四个各 512M 的相等块，现在我们可以将它们分配cpusets。随着你更熟悉使用这一组合进行资源控制，你会确定一个更好的设置，以尽量减少需要处理的节点数量
```

	Faking node 0 at 0000000000000000-0000000020000000 (512MB)
	Faking node 1 at 0000000020000000-0000000040000000 (512MB)
	Faking node 2 at 0000000040000000-0000000060000000 (512MB)
	Faking node 3 at 0000000060000000-0000000080000000 (512MB)
	...
	On node 0 totalpages: 130975
	On node 1 totalpages: 131072
	On node 2 totalpages: 131072
	On node 3 totalpages: 131072

```
现在按照 Documentation/admin-guide/cgroup-v1/cpusets.rst 中挂cpuset 文件系统的说明，你可以分配伪节点（即连续内存
```

	[root@xroads /]# mkdir exampleset
	[root@xroads /]# mount -t cpuset none exampleset
	[root@xroads /]# mkdir exampleset/ddset
	[root@xroads /]# cd exampleset/ddset
	[root@xroads /exampleset/ddset]# echo 0-1 > cpus
	[root@xroads /exampleset/ddset]# echo 0-1 > mems

```
现在这个名为 'ddset' cpuset 将只允许访问伪节0 1 进行内存分配G）
你现在可以将任务分配给这cpuset，以限制内存资源
```

	[root@xroads /exampleset/ddset]# echo $$ > tasks
	[root@xroads /exampleset/ddset]# dd if=/dev/zero of=tmp bs=1024 count=1G
	[1] 13425

```
注意上面受限 cpuset 情况与不受限情况（即在未分配给伪 NUMA cpuset 的情况下运行相同 'dd' 命令）之间，/proc/meminfo 所报告的系统内存使用量差异
	========	============	==========
	Name		Unrestricted	Restricted
	========	============	==========
	MemTotal	3091900 kB	3091900 kB
	MemFree		42113 kB	1513236 kB
	========	============	==========

这实现了对你分配给特cpuset 的任务进行粗粒度内存管理。由cpuset 可以形成层级结构，你可以为各类任务的内存管理需求创建一些相当有趣的组合用例