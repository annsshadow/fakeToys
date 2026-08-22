
## 内存热插

## 内存热插拔事件通知

热插拔事件会被发送到一个通知队列
### 内存通知

`include/linux/memory.h` 中定义了六种类型的通知
MEM_GOING_ONLINE
  在新内存变得可用之前产生，以便能够让各子系统做好准备来处理内存。此时页分配器仍无法从新内存中分配
MEM_CANCEL_ONLINE
  MEM_GOING_ONLINE 失败时产生
MEM_ONLINE
  当内存成功上线时产生。回调可以从新内存中分配页
MEM_GOING_OFFLINE
  在开始内存下线过程时产生。此时已无法再从该内存中分配，但部分将被下线的内存仍在使用中。该回调可用于从指定的内存块中释放某个子系统已知的内存
MEM_CANCEL_OFFLINE
  MEM_GOING_OFFLINE 失败时产生。我们尝试下线的内存块重新可用
MEM_OFFLINE
  在下线内存完成后产生
```

  hotplug_memory_notifier(callback_func, priority)

```
priority 值较大的回调函数会在 priority 值较小的回调函数之前被调用
```

  int callback_func(
    struct notifier_block *self, unsigned long action, void *arg);

```
回调函数的第一个参数（self）是指向通知链中某个块的指针，该块指向回调函
数自身。第二个参数（action）是上述事件类型之一
```
	struct memory_notify {
		unsigned long start_pfn;
		unsigned long nr_pages;
	}

```
- start_pfn 为上下线内存的起pfn- nr_pages 为上下线内存的页数
有可能在未收MEM_GOING_ONLINE 通知的情况下就收MEM_CANCEL_ONLINE 通知MEM_CANCEL_OFFLINE MEM_GOING_OFFLINE 也同样如此。这会在某个消费者失时发生，意味着我们中断了调用链并停止调用通知器的其余消费者。因此，
memory_notify 的使用者不应做任何假设，并应准备好处理此类情况
回调例程应返`include/linux/notifier.h` 中定义的以下值之一NOTIFY_DONE、NOTIFY_OK、NOTIFY_BAD、NOTIFY_STOP

NOTIFY_DONE NOTIFY_OK 对后续处理没有影响
NOTIFY_BAD 用作MEM_GOING_ONLINE、MEM_GOING_OFFLINE、MEM_ONLINE MEM_OFFLINE 动作的响应，用于取消热插拔。它会停止通知队列的后续处理
NOTIFY_STOP 停止通知队列的后续处理
### NUMA 鑺傜偣閫氱煡鍣。

`include/linux/node.h` 中定义了六种类型的通知
NODE_ADDING_FIRST_MEMORY
 在该节点首次有内存可用之前产生
NODE_CANCEL_ADDING_FIRST_MEMORY
 NODE_ADDING_FIRST_MEMORY 失败时产生
NODE_ADDED_FIRST_MEMORY
 当该节点首次有内存可用时产生
NODE_REMOVING_LAST_MEMORY
 当该节点最后可用的内存即将被下线时产生
NODE_CANCEL_REMOVING_LAST_MEMORY
 NODE_CANCEL_REMOVING_LAST_MEMORY 失败时产生
NODE_REMOVED_LAST_MEMORY
 当该节点最后可用的内存已被下线时产生
```

  hotplug_node_notifier(callback_func, priority)

```
priority 值较大的回调函数会在 priority 值较小的回调函数之前被调用
```

  int callback_func(

    struct notifier_block *self, unsigned long action, void *arg);

```
回调函数的第一个参数（self）是指向通知链中某个块的指针，该块指向回调函
数自身。第二个参数（action）是上述事件类型之一
```
        struct node_notify {
                int nid;
        }

```
- nid 为我们要添加或移除内存的节点
有可能在未收NODE_ADDING_FIRST_MEMORY 通知的情况下就收NODE_CANCEL_ADDING_FIRST_MEMORY 通知，NODE_CANCEL_REMOVING_LAST_MEMORY NODE_REMOVING_LAST_MEMORY 也同样如此。这会在某个消费者失败时发送，意味着我们
中断了调用链并停止调用通知器的其余消费者。因此，node_notify 的使用者不做任何假设，并应准备好处理此类情况
回调例程应返`include/linux/notifier.h` 中定义的以下值之一NOTIFY_DONE、NOTIFY_OK、NOTIFY_BAD、NOTIFY_STOP

NOTIFY_DONE NOTIFY_OK 对后续处理没有影响
NOTIFY_BAD 用作NODE_ADDING_FIRST_MEMORY、NODE_REMOVING_LAST_MEMORYNODE_ADDED_FIRST_MEMORY NODE_REMOVED_LAST_MEMORY 动作的响应，用于取消热插拔它会停止通知队列的后续处理
NOTIFY_STOP 停止通知队列的后续处理
请注意，对于 NODE_ADDED_FIRST_MEMORY / NODE_REMOVED_FIRST_MEMORY 我们不应失败因为此时 memory_hotplug 代码已无法回滚
## 锁的内部机制


当添移除使用内存块设备（即普RAM）的内存时，应持device_hotplug_lock，以
- 与上下线请求（例如通过 sysfs）保持同步。这样，内存块设备只有在内存  完全添加后，才能被用户空间访问（.online/.state 属性）。而在移除内存时，
  我们知道没有人在关键区段中- CPU 热插拔及类似操作保持同步（例如与 ACPI PPC 相关）
特别地，在添加内存而用户空间试图比预期更快地将该内存上线时，存在一种可能的
锁反转，使用 device_hotplug_lock 可避免该问题
- device_online() 会先获取 device_lock()，随后获mem_hotplug_lock
- add_memory_resource() 会先获取 mem_hotplug_lock，随后获device_lock()
  （在创建设备期间，于 bus_add_device() 中）
由于该设备在对用户空间可见之后才会获device_lock()，因此可能导致锁反转
内存的上下线应通过 device_online()/device_offline() 完成 —以确保其sysfs 发起的操作正确同步。建议持device_hotplug_lock（例如以保护
online_type）
当添移除/上线/下线内存，或添加/移除异构/设备内存时，我们应始终以写模持有 mem_hotplug_lock，以串行化内存热插拔（例如对全局/zone 变量的访问）
此外，mem_hotplug_lock（与 device_hotplug_lock 不同）在读模式下允许一个相高效get_online_mems/put_online_mems 实现，因此访问内存的代码可借此防止
该内存消失