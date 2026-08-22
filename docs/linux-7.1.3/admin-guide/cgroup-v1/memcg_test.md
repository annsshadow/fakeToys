## 内存资源控制器（Memcg）实现备忘录


最后更新：2010/2

基准内核版本：基2.6.33-rc7-mm4 的候选版本）

由于 VM 正变得复杂（原因之一便是 memcg……），memcg 的行
也十分复杂。本文档描述 memcg 的内部行为
请注意，实现细节可能会发生变化

）有API 的主题请参见 `Documentation/admin-guide/cgroup-v1/memory.rst`

## 0. 如何记录用量


   使用2 个对象

   `page_cgroup`……每个页对应一个对象

	在启动或内存热插拔时分配，在内存热移除时释放

   `swap_cgroup`……每`swp_entry` 对应一项

	swapon() 时分配，swapoff() 时释放

   `page_cgroup` 带有 USED 位，且永远不会对同一 `page_cgroup` 重复计数
   `swap_cgroup` 仅在被计费的页换出（swapped-out）时使用

## 1. 计费（Charge


   一个页 / `swp_entry` 可能在以下位置被计费（usage += PAGE_SIZE）：

	mem_cgroup_try_charge()

## 2. 注销计费（Uncharge


   一个页 / `swp_entry` 可通过以下函数被注销计费（usage -= PAGE_SIZE）：

	mem_cgroup_uncharge()
	  在页的引用计数降0 时调用

	mem_cgroup_uncharge_swap()
	  `swp_entry` 的引用计数降0 时调用。针对交换区的计费随之消失

## 3. 计费-提交（charge-commit


	Memcg 页的计费分两步进行：

  - `mem_cgroup_try_charge()`
  - `commit_charge()`

	`try_charge()` 时，尚不存在表示“本页已被计费”的标志
	此时 usage += PAGE_SIZE

	`commit()` 时，页与 memcg 建立关联

在下面的说明中，我们假设 `CONFIG_SWAP=y`

## 4. 匿名页（Anonymous


	匿名页在以下情形新分配：

    - `MAP_ANONYMOUS` 映射发生缺页（page fault）
    - 写时复制（Copy-On-Write）

	4.1 换入（Swap-in）
	换入时，页取swap-cache。存在两种情况

	(a) `SwapCache` 是新分配并被读取的，则它未被计费
	(b) `SwapCache` 已被进程映射，则它已经被计费

	4.2 换出（Swap-out）
	换出时，典型的状态转换如下

	(a) 加入交换缓存（标记为 `SwapCache`）
	    `swp_entry` 的引用计+= 1
	(b) 完全解除映射
	    `swp_entry` 的引用计+= PTE 的数量
	(c) 写回交换区
	(d) 从交换缓存删除（移出 `SwapCache`）
	    `swp_entry` 的引用计-= 1


	最后，在任务退出时
	(e) 调用 zap_pte()，`swp_entry` 的引用计-= 1 0

## 5. 页缓存（Page Cache


	页缓存（Page Cache）在以下位置被计费：

 - `filemap_add_folio()`銆。

	逻辑非常清晰。（关于迁移，见下文

	注意
	  `__filemap_remove_folio()` 鐢?`filemap_remove_folio()`
	  `__remove_mapping()` 调用

## 6. Shmem（tmpfs）页缓存


	理解 shmem 页状态转换的最佳方式是阅读
	`mm/shmem.c`銆。

	但对 memcg 围绕 shmem 的行为做简要说明，有助于理解其逻辑

	Shmem 的页（仅叶子页，不含直接/间接块）可以位于

  - shmem inode radix-tree（基数树）
  - `SwapCache`銆。
  - 同时位于 radix-tree `SwapCache` 中。这发生在换入（swap-in）时
		以及换出（swap-out）时

	它在以下情形被计费：

 - 一个新页被添加shmem radix-tree 中
 - 读取一swp 页。（将计费从 `swap_cgroup` 转移`page_cgroup`

## 7. 页迁移（Page Migration


	mem_cgroup_migrate()

## 8. LRU


	每个 memcg 都拥有自己的一LRU 向量（非活跃匿名、活跃匿名
	非活跃文件、活跃文件、不可回收），其页来自各个节点；
	每个 LRU 在该 memcg 与节点对应的单一 `lru_lock` 下处理

## 9. 典型测试


   针对竞态（racy）情况的测试

### 9.1 memcg 设置较小限制


	进行竞态测试时，将 memcg 的限制设得很小（而非 GB 级）是个不错的测试
	xKB xxMB 级别的限制下能发现大量竞态

	（内存在 GB 级与 MB 级下的行为表现差异很大。）

### 9.2 Shmem


	历史上，memcg shmem 的处理较差，我们也在此遇到过一些问题
	这是因为 shmem 既是页缓存，又可能是 `SwapCache`。使shmem/tmpfs
	进行测试始终是个好选择

### 9.3 迁移（Migration


	对于 NUMA，迁移是另一个特例。为便于测试，可使用 cpuset
```

		mount -t cgroup -o cpuset none /opt/cpuset

		mkdir /opt/cpuset/01
		echo 1 > /opt/cpuset/01/cpuset.cpus
		echo 0 > /opt/cpuset/01/cpuset.mems
		echo 1 > /opt/cpuset/01/cpuset.memory_migrate
		mkdir /opt/cpuset/02
		echo 1 > /opt/cpuset/02/cpuset.cpus
		echo 1 > /opt/cpuset/02/cpuset.mems
		echo 1 > /opt/cpuset/02/cpuset.memory_migrate

	In above set, when you moves a task from 01 to 02, page migration to
	node 0 to node 1 will occur. Following is a script to migrate all
	under cpuset.::

		--
		move_task()
		{
		for pid in $1
		do
			/bin/echo $pid >$2/tasks 2>/dev/null
			echo -n $pid
			echo -n " "
		done
		echo END
		}

		G1_TASK=`cat ${G1}/tasks`
		G2_TASK=`cat ${G2}/tasks`
		move_task "${G1_TASK}" ${G2} &
		--

```

### 9.4 内存热插拔（Memory hotplug


	memory hotplug 测试是一种不错的测试
```

		# echo offline > /sys/devices/system/memory/memoryXXX/state

	(XXX is the place of memory)

	This is an easy way to test page migration, too.

```

### 9.5 嵌套 cgroup（nested cgroups


```

		mkdir /opt/cgroup/01/child_a
		mkdir /opt/cgroup/01/child_b

		set limit to 01.
		add limit to 01/child_b
		run jobs under child_a and child_b

	create/delete following groups at random while jobs are running::

		/opt/cgroup/01/child_a/child_aa
		/opt/cgroup/01/child_b/child_bb
		/opt/cgroup/01/child_c

	running new jobs in new group is also good.

```

### 9.6 与其他子系统一起挂


	与其他子系统一起挂载是一个不错的测试，因为与其他 cgroup 子系
	之间存在竞态与锁依赖
```

		# mount -t cgroup none /cgroup -o cpuset,memory,cpu,devices

	and do task move, mkdir, rmdir etc...under this.

```

### 9.7 swapoff


	除交换区管理本身memcg 中较复杂的部分外，swapoff 时的换入调用路径
	也与通常的换入路径不同，值得专门测试

	例如，下面这样的测试是不错的
```

		# mount -t cgroup none /cgroup -o memory
		# mkdir /cgroup/test
		# echo 40M > /cgroup/test/memory.limit_in_bytes
		# echo 0 > /cgroup/test/tasks

	Run malloc(100M) program under this. You'll see 60M of swaps.

	(Shell-B)::

		# move all tasks in /cgroup/test to /cgroup
		# /sbin/swapoff -a
		# rmdir /cgroup/test
		# kill malloc task.

	Of course, tmpfs v.s. swapoff test should be tested, too.

```

### 9.8 OOM-Killer（内存耗尽杀手）


	memcg 限制引发Out-of-memory 会终止该 memcg 下的任务
	使用层级（hierarchy）时，层级下的任务会被内核终止

	在这种情况下，不应触panic_on_oom，也不应终止其他组的任务

	memcg 下引OOM 并不困难，如下所示
```

		#swapoff -a
		#echo 50M > /memory.limit_in_bytes

	run 51M of malloc

	Case B) when you use mem+swap limitation::

		#echo 50M > memory.limit_in_bytes
		#echo 50M > memory.memsw.limit_in_bytes

	run 51M of malloc

```

### 9.9 任务迁移时移动计费（Move charges


	与任务关联的计费可随任务迁移一起移动
```

		#mkdir /cgroup/A
		#echo $$ >/cgroup/A/tasks

	run some programs which uses some amount of memory in /cgroup/A.

	(Shell-B)::

		#mkdir /cgroup/B
		#echo 1 >/cgroup/B/memory.move_charge_at_immigrate
		#echo "pid of the program running in group A" >/cgroup/B/tasks

	You can see charges have been moved by reading ``*.usage_in_bytes`` or
	memory.stat of both A and B.

	See 8.2 of Documentation/admin-guide/cgroup-v1/memory.rst to see what value should
	be written to move_charge_at_immigrate.

```

### 9.10 内存阈值（Memory thresholds


	内存控制器使cgroups 的通知 API 实现内存阈值
	你可以使tools/cgroup/cgroup_event_listener.c 来测试
```

		# mkdir /cgroup/A
		# ./cgroup_event_listener /cgroup/A/memory.usage_in_bytes 5M

	(Shell-B) Add task to cgroup and try to allocate and free memory::

		# echo $$ >/cgroup/A/tasks
		# a="$(dd if=/dev/zero bs=1M count=10)"
		# a=

	You will see message from cgroup_event_listener every time you cross
	the thresholds.

	Use /cgroup/A/memory.memsw.usage_in_bytes to test memsw thresholds.

	It's good idea to test root cgroup as well.

```
