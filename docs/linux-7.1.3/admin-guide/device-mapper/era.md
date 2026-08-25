## dm-era


## 简

dm-era 是一个行为类似于 linear 目标（target）的目标。此外，它会记录在一用户定义的、称“era”（时代）的时间段内哪些块被写入。每era 目标实例
将当era 维护为一个单调递增32 位计数器
其用例包括为备份软件跟踪已更改的块，以及在回滚厂商快照后部分地使缓存内容
失效以恢复缓存一致性
## 构造（Constructor

era <metadata dev> <origin dev> <block size>

 ================ ======================================================
 metadata dev     持有持久化元数据的快速设 origin dev	 持有可能更改的数据块的设 block size       源数据设备的块大小，即目标所跟踪的粒 ================ ======================================================

## 消息（Messages

dm 消息均不接受任何参数
### checkpoint


可能进入一个新era。你不应假定 era 已经递增。发送此消息后，应通过状态行
检查当era
### take_metadata_snap


创建元数据的克隆，以允许用户态进程读取它
### drop_metadata_snap


丢弃元数据快照
## 状态（Status

<metadata block size> <#used metadata blocks>/<#total metadata blocks>
<current era> <held metadata root | '-'>

========================= ==============================================
metadata block size	  每个元数据块的固定块大小，以扇区#used metadata blocks	  已使用的元数据块数量
#total metadata blocks	  元数据块的总数current era		  当前 era
held metadata root	  为供用户态读取访问而“持有”的元数据根所在的位置
			 （以块计）-' 表示没有持有的根
========================= ==============================================

## 详细用例


在开发此目标时，回滚厂商快照时使缓存失效的场景是主要用例
### 获取厂商快照


- era 目标发checkpoint 消息
- 在其状态行中记下当era
- 获取厂商快照（era 与快照现在应永远关联
### 回滚到厂商快

- 缓存进入直通模式（参见：cache.txt dm-cache 的文档）
- 回滚厂商存储
- 获取元数据快- 通过检查每个块era 来确定自快照获取以来哪些块被写入
- 在缓存软件中使这些块失效
- 缓存返回到回直写模式

## 内存使用


该目标使用一个位集（bitset）记录当era 中的写入。它还为切换到新 era
准备了一个备用位集。除此之
```
   (4 * nr_blocks) bytes + buffers

```
## 健壮性（Resilience

在向先前未写入的块执行写入之前，元数据会在磁盘上更新。因dm-era 不应
受到诸如断电之类的硬崩溃影响
## 用户态工

用户态工具位于命名越来越不恰当的 thin-provisioning-tools 项目中：

    https://github.com/jthornber/thin-provisioning-tools
