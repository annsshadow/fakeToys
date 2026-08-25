## MD 集群


集群 MD 是一种用于集群的共享设备 RAID，它支持两个级别：raid1 raid10（支持有限）

## 1. 磁盘格式


每个集群节点使用独立的写意图位图（write-intent-bitmap）。这些位图记录了该节点上可能已经启动的所有写入，

```

  0                    4k                     8k                    12k
  -------------------------------------------------------------------
  | idle                | md super            | bm super [0] + bits |
  | bm bits[0, contd]   | bm super[1] + bits  | bm bits[1, contd]   |
  | bm super[2] + bits  | bm bits [2, contd]  | bm super[3] + bits |
  | bm bits [3, contd]  |                     |                     |

```
在“正常”运行过程中，我们假设文件系统确保任意时刻只有一个节点写入给定的块，因此一次写入请求会
 - 设置相应的位（如果尚未设置）
 - 将写入提交到所有镜 - 安排在超时后清除该位
读取按正常方式处理。由文件系统负责确保某个节点不会从另一个节点（或同一节点）正在写入的位置读取

## 2. 用于管理DLM 

有三组锁用于管理设备
### 2.1 位图锁资源（bm_lockres

 bm_lockres 保护各个节点的位图。其命名形式为：node 1 对应 bitmap000，node 2 对应 bitmap001，依此类推。当节点加入集群时，它以 PW 模式获取该锁，并在节点作为集群成员的整个生命周期内一直持有。锁资源编号基于 DLM 子系统返回的槽号。由DLM 的节点计数从 1 开始，而位图槽位从 0 开始，因此DLM 槽号减去 1 得到位图槽号
 某个节点位图锁的 LVB 记录了该节点正在重新同步的扇区范围。其他节点不得写入这些扇区。这在有新节点加入集群时使用
### 2.2 消息传递锁


 每个节点在启动或结束重新同步，以及进行元数据超级块更新时，必须与其他节点通信。该通信通过三个锁管理：“token”、“message“ack”，以及其中一“message锁的锁值块（LVB）
### 2.3 新设备管

 使用单个“no-new-dev来协调新设备的添加——这必须在整个阵列中同步。通常所有节点都对该设备持有并发读锁
## 3. 通信


 消息可以广播到所有节点，发送方在继续之前等待所有其他节点确认该消息。任意时刻只能处理一条消息
### 3.1 消息类型


 共有六种传递的消息类型
##### 3.1.1 METADATA_UPDATED


   通知其他节点元数据已更新，该节点必须重新读取 md 超级块。这是同步执行的。它主要用于发出设备故障信号
##### 3.1.2 RESYNCING

   通知其他节点重新同步已启动或结束，以便各节点可以挂起或恢复该区域。每RESYNCING 消息标识发送节点即将重新同步的设备范围。这会覆盖该节点之前的任何通知：每个节点一次只能重新同步一个范围
##### 3.1.3 NEWDISK


   通知其他节点正在向阵列添加设备。消息包含该设备的标识符。更多细节见下文
##### 3.1.4 REMOVE


   一个故障设备或备用设备正从阵列中移除。消息中包含该设备的槽号
 3.1.5 RE_ADD:

   正重新激活一个故障设备——其前提是已确认该设备恢复正常工作
 3.1.6 BITMAP_NEEDS_SYNC:

   如果一个节点在本地停止但位图不干净，则通知另一个节点接管重新同步的所有权
### 3.2 通信机制


 DLM LVB 用于在集群节点之间通信。用于此目的的有三个资源
##### 3.2.1 token

   保护整个通信系统的资源。持token 资源的节点才允许通信
##### 3.2.2 message

   携带待通信数据的锁资源
##### 3.2.3 ack


   获取该资源意味着消息已被集群中所有节点确认。该资源BAST 用于通知接收节点有节点想要通信
该算法为
```

	sender                         receiver                 receiver
	"ack":CR                       "ack":CR                 "ack":CR

 2. sender get EX on "token",
    sender get EX on "message"::

	sender                        receiver                 receiver
	"token":EX                    "ack":CR                 "ack":CR
	"message":EX
	"ack":CR

    Sender checks that it still needs to send a message. Messages
    received or other events that happened while waiting for the
    "token" may have made this message inappropriate or redundant.

 3. sender writes LVB

    sender down-convert "message" from EX to CW

    sender try to get EX of "ack"

    ::

      [ wait until all receivers have *processed* the "message" ]

                                       [ triggered by bast of "ack" ]
                                       receiver get CR on "message"
                                       receiver read LVB
                                       receiver processes the message
                                       [ wait finish ]
                                       receiver releases "ack"
                                       receiver tries to get PR on "message"

     sender                         receiver                  receiver
     "token":EX                     "message":CR              "message":CR
     "message":CW
     "ack":EX

 4. triggered by grant of EX on "ack" (indicating all receivers
    have processed message)

    sender down-converts "ack" from EX to CR

    sender releases "message"

    sender releases "token"

    ::

                                 receiver upconvert to PR on "message"
                                 receiver get CR of "ack"
                                 receiver release "message"

     sender                      receiver                   receiver
     "ack":CR                    "ack":CR                   "ack":CR


```
## 4. 故障处理


### 4.1 节点故障


 当节点发生故障时，DLM 会通过槽号通知集群。该节点启动一个集群恢复线程。集群恢复线程会
 - 获取故障节点bitmap<number>  - 打开位图
 - 读取故障节点的位 - 将已置位的位图复制到本地节点
 - 清空故障节点的位 - 释放故障节点bitmap<number>  - 在当前节点上启动位图的重新同	  recover_bitmaps 内部调用 md_check_recovery	  然后 md_check_recovery -> metadata_update_start/finish	  它会通过 lock_comm 锁定通信	  这意味着当一个节点正在重新同步时，会阻止所	  其他节点对阵列的任何位置进行写入
 重新同步过程是常规的 md 重新同步。然而，在集群环境中执行重新同步时，需要把被挂起的区域告知其他节点。在重新同步开始前，节点会发出带有需要挂起区(lo,hi) 范围RESYNCING。每个节点维护一suspend_list，其中包含当前被挂起的范围列表。收RESYNCING 后，节点将该范围加入 suspend_list。类似地，当执行重新同步的节点完成时，它会向其他节点发送带有空范围RESYNCING，其他节点则suspend_list 中移除相应条目
 辅助函数 ->area_resyncing() 可用于检查某个特定的 I/O 范围是否应被挂起
## 4.2 设备故障


 设备故障通过元数据更新例程进行处理和通报。当节点检测到设备故障时，在故障被所有其他节点确认之前，不允许对该设备进行任何进一步写入
### 5. 添加新设

 要添加新设备，必须让所有节点都能“看到”要添加的新设备。为此使用以下算法：

   1. Node 1 执行 mdadm --manage /dev/mdX --add /dev/sdYY，其发出 ioctl(ADD_NEW_DISK，其disc.state 设为 MD_DISK_CLUSTER_ADD)
   2. Node 1 发送带uuid 和槽号的 NEWDISK 消息
   3. 其他节点发出带有 uuid 和槽号的 kobject_uevent_env（步4 可能是一udev 规则   4. 在用户空间，节点搜索磁盘，可能使blkid -t SUB_UUID=""
   5. 其他节点根据是否找到磁盘，发出以下任一操作       ioctl(ADD_NEW_DISK，其disc.state 设为 MD_DISK_CANDIDATE，且
       disc.number 设为槽号)
       ioctl(CLUSTERED_DISK_NACK)
   6. 如果找到设备，其他节点释"no-new-devs" 上的锁（CR   7. Node 1 尝试获取 "no-new-dev" EX    8. 如果 Node 1 获取到锁，则取消该磁盘的 SpareLocal 标记后发METADATA_UPDATED
   9. 否则（未获取"no-new-dev" 锁），则操作失败并发METADATA_UPDATED
   10. 其他节点通过后续METADATA_UPDATED 获知磁盘是否被添加
## 6. 模块接口


 17 个回调是 md 核心可以向集群模块发起的。理解这些回调可以很好地从整体上了解整个过程
### 6.1 join(nodes) 鍜?leave()


 当以集群位图启动阵列以及停止阵列时调用它们。join() 确保集群可用并初始化各种资源。集群中只有'nodes' 个节点可以使用该阵列
### 6.2 slot_number()


 报告集群基础设施建议的槽号。范围为 0 nodes-1
### 6.3 resync_info_update()


 这更新存储在位图锁中的重新同步范围。起点随重新同步的推进而更新。终点始终为阵列的末尾。它***发RESYNCING 消息
### 6.4 resync_start()、resync_finish()


 当重新同恢复/重塑启动或停止时调用它们。它们更新位图锁中的重新同步范围，并发RESYNCING 消息。resync_start 将整个阵列报告为正在重新同步，resync_finish 则不报告任何部分
 resync_finish() 还会发BITMAP_NEEDS_SYNC 消息，使其他节点可以接管
### 6.5 metadata_update_start()、metadata_update_finish()、metadata_update_cancel()


 metadata_update_start 用于获取对元数据的独占访问。一旦获得该访问后仍有变更需要时，metadata_update_finish() 会向所有其他节点发METADATA_UPDATE 消息；否则可使用 metadata_update_cancel() 释放该锁
### 6.6 area_resyncing()


 它结合了两部分功能
 首先，它会检查是否有节点当前正在给定扇区范围内重新同步。如果发现任何重新同步，调用方将避免在该范围内写入或进行读均衡
 其次，在节点恢复期间，它会报告所有区域对 READ 请求都处于重新同步状态。这避免了集群文件系统与集群 RAID 在处理节点故障时出现竞态
### 6.7 add_new_disk_start()、add_new_disk_finish()、new_disk_ack()


 这些用于管理上述新磁盘协议。添加新设备时，在设备绑定到阵列之前调用 add_new_disk_start()，如果成功，则调add_new_disk_finish() 完成设备的完整添加
 当设备作为对先前请求的确认而被添加，或当设备被声明为“不可用”时，调new_disk_ack()
### 6.8 remove_disk()


 当备用设备或故障设备从阵列中移除时调用。它会向其他节点发送一REMOVE 消息
### 6.9 gather_bitmaps()


 这会向所有其他节点发送一RE_ADD 消息，然后从所有位图收集位图信息。该合并后的位图随后用于恢复被重新添加的设备
### 6.10 lock_all_bitmaps() 鍜?unlock_all_bitmaps()


 当把位图改为 none 时调用它们。如果某个节点计划清除集RAID 的位图，需要确保没有其他节点正在使用该 RAID，这通过锁定集群内所有位图锁来实现，这些锁也会相应地被解锁
## 7. 不支持的特

集群 MD 目前尚不支持以下功能
- 更改 array_sectors