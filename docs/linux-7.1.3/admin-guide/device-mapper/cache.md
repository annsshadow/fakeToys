## 缓存（Cache
## 简
dm-cache 是一个由 Joe Thornber、Heinz Mauelshagen Mike Snitzer 编写device mapper（设备映射器）目标
它的目标是通过将块设备（例如机械硬盘）的部分数据动态地迁移到更快、更小的设备（例SSD）上，来提升该块设备的性能
这个设备映射器方案允许我们在 dm 栈的不同层级插入这一缓存，例如位于瘦供给（thin-provisioning）池的数据设备之上。与虚拟内存系统集成得更紧密的缓存方案应该能提供更好的性能
该目标复用了瘦供给库中所使用的元数据库
关于迁移哪些数据、何时迁移的决策，留给了可插拔的策略模块。在我们进行实验的过程中已经编写了若干这样的模块，并且我们希望其他人也能为特定的 I/O 场景（例如虚拟机镜像服务器）贡献其他的模块
## 术语
  Migration（迁移）
	       将逻辑块的主副本从一个设备移动到另一个设备  Promotion（提升）
	       从慢速设备迁移到快速设备  Demotion（降级）
	       从快速设备迁移到慢速设备
origin 设备始终包含逻辑块的一个副本，它可能已过期，也可能与缓存设备上的副本保持同步（取决于策略）
## 设计

### 子设
该目标通过向它传入三个设备（以及稍后详述的其他参数）来构建
1. 一origin 设备——大而慢的那个
2. 一cache 设备——小而快的那个
3. 一个小的元数据设备——记录哪些块在缓存中、哪些是脏的，以及供策略对象使用的额外提示。这些信息本可以放在缓存设备上，但将其分开可以让卷管理器以不同的方式配置它，例如配置为一个镜像以获得额外的健壮性。这个元数据设备可能仅能被单个缓存设备使用
### 固定块大
origin 被划分为固定大小的块。这个块大小在你首次创建缓存时是可配置的。通常我们使用 256KB - 1024KB 的块大小。块大小必须64 个扇区（32KB）到 2097152 个扇区（1GB）之间，并且64 个扇区（32KB）的倍数
拥有固定的块大小大大简化了该目标。但它也算是一种折中。例如，一个块的一小部分可能经常被访问，但整个块都会被提升到缓存中。因此大的块大小不好，因为它们浪费缓存空间；而小的块大小也不好，因为它们会增加元数据的数量（无论是在内存中还是磁盘上）
### 缓存运行模式

缓存有三种运行模式：writeback（回写）、writethrough（直写）passthrough（直通）
如果选择默认writeback，那么对某个已被缓存的块的写入将只写入缓存，并且该块会在元数据中被标记为脏
如果选择 writethrough，那么对某个已缓存块的写入要等到它同时命origin 设备cache 设备后才会完成。干净块应当保持干净
如果选择 passthrough（在缓存内容未知是否origin 设备一致时很有用），那么所有读请求都从 origin 设备提供（所有读都未命中缓存），并且所有写请求都被转发origin 设备；此外，写命中会导致缓存块失效。要启用 passthrough 模式，缓存必须是干净的。Passthrough 模式允许在不担心一致性的情况下激活一个缓存设备。已存在的一致性会被保持，不过随着写入的发生，缓存会逐渐变冷。如果之后能够验证缓存的一致性，或者通过使用 "invalidate_cblocks" 消息来建立一致性，那么缓存设备就可以在仍处于热状态时切换writethrough writeback 模式。否则，在切换到期望的运行模式之前，可以丢弃缓存内容
提供了一个简单的 cleaner 策略，它会清理（回写）缓存中的所有脏块。这在停用缓存或收缩缓存时很有用。收缩缓存的快速设备要求被移除区域中的所有缓存块都是干净的。如果从缓存中移除的区域仍然包含脏块，则调整大小会失败。必须注意，在缓存干净之前，绝不能缩减用于缓存快速设备的卷。如果使用了 writeback 模式，这一点尤为重要。Writethrough passthrough 模式已经保持了一个干净的缓存。未来将支持在指定阈值之上部分清理缓存，从而在调整大小期间保持缓存的热状态并维持 writeback 模式
### 迁移节流

origin cache 设备之间迁移数据会占用带宽。用户可以设置一个节流值，以防止在任一时刻发生超过一定数量的迁移。目前我们完全没有考虑发往设备的正I/O 流量。这里还需要做更多工作，以避免在那I/O 峰值时刻进行迁移
目前，可以使用消"migration_threshold <#sectors>" 来设置被迁移扇区的最大数量，默认2048 个扇区（1MB）
### 更新磁盘上的元数
每次写入一FLUSH FUA bio 时，磁盘上的元数据都会被提交。如果没有发出此类请求，则提交会每一秒发生一次。这意味着缓存的行为类似于一个带有易失性写缓存的物理磁盘。如果断电，你可能会丢失一些最近的写入。尽管如此，在任何崩溃之后元数据都应当保持一致
缓存块的 "dirty" 状态变化过于频繁，我们无法实时地持续更新它。因此我们将其视为一个提示。在正常操作中，它会dm 设备被挂起时写入。如果系统崩溃，重启时所有缓存块都将被视为脏的
### 每块的策略提
策略插件可以为每个缓存块存储一小块数据。这块数据有多大由策略决定，但应当保持较小。与脏标志一样，如果崩溃，这些数据会丢失，因此应当始终有一个安全的回退值
策略提示影响的是性能，而非正确性
### 策略消息

策略会有各自不同的、特定于该策略的可调参数，因此我们需要一种通用的获取和设置它们的方式。这里使device-mapper 消息。请参阅 cache-policies.txt
### 丢弃位图（Discard bitset）分辨率

如果我们知道某个块已被丢弃（discard），就可以在迁移期间避免复制数据。一个主要的例子mkfs 丢弃整个块设备时。我们存储一个位图来跟踪块的丢弃状态。不过，我们允许这个位图拥有与缓存块不同的块大小。这是因为我们需要为整个 origin 设备跟踪丢弃状态（与之相比，脏位图只针对较小的缓存设备）
## 目标接口

### 构造函
```
   cache <metadata dev> <cache dev> <origin dev> <block size>
         <#feature args> [<feature arg>]*
         <policy> <#policy args> [policy args]*

 ================ =======================================================
 metadata dev     fast device holding the persistent metadata
 cache dev	  fast device holding cached data blocks
 origin dev	  slow device holding original data blocks
 block size       cache unit size in sectors

 #feature args    number of feature arguments passed
 feature args     writethrough or passthrough (The default is writeback.)

 policy           the replacement policy to use
 #policy args     an even number of arguments corresponding to
                  key/value pairs passed to the policy
 policy args      key/value pairs passed to the policy
		  E.g. 'sequential_threshold 1024'
		  See cache-policies.txt for details.
 ================ =======================================================
```

可选的特性参数有
   ==================== ========================================================
   writethrough		直写式缓存，禁止缓存块内容与 origin 块内容不同			若不加此参数，默认行为是为了性能而稍后将缓存			内容回写，因此它们可能与对应origin 块不同
   passthrough		一种降级模式，用于各种缓存一致性场景（例如回滚
			底层存储的快照）。读和写总是发往 origin。若一			写命中了已被缓存origin 块，则该缓存块被失效			要启passthrough 模式，缓存必须处于干净状态
   metadata2		使用元数据的2 版。它将脏位存放在一个独立的
			btree 中，从而提升缓存关闭的速度
   no_discard_passdown	禁止discard 从缓存向下透传origin 的数据设备   ==================== ========================================================

一个名'default' 的策略总是被注册。它是我们当前认为能带来最佳综合性能的策略的别名
由于默认策略可能在不同内核之间有所变化，如果你依赖于某个特定策略的特性，请始终按名称请求它
### 状
```
  <metadata block size> <#used metadata blocks>/<#total metadata blocks>
  <cache block size> <#used cache blocks>/<#total cache blocks>
  <#read hits> <#read misses> <#write hits> <#write misses>
  <#demotions> <#promotions> <#dirty> <#features> <features>*
  <#core args> <core args>* <policy name> <#policy args> <policy args>*
  <cache metadata mode>


```
========================= =====================================================
metadata block size	  每个元数据块的固定块大小（以扇区计）
#used metadata blocks	  已使用的元数据块数量
#total metadata blocks	  元数据块的总数cache block size	  缓存设备可配置的块大小（以扇区计#used cache blocks	  驻留在缓存中的块数量
#total cache blocks	  缓存块的总数#read hits		  READ bio 被映射到缓存的次#read misses		  READ bio 被映射到 origin 的次#write hits		  WRITE bio 被映射到缓存的次#write misses		  WRITE bio 被映射到 origin 的次#demotions		  从缓存中移除块的次数
#promotions		  被移动到缓存中的块的次数
#dirty			  缓存中与 origin 不同的块数量
#feature args		  后续 feature 参数的数feature args		  'writethrough'（可选）
#core args		  核心参数的数量（必须为偶数）
core args		  用于调优核心的键/值对，例migration_threshold
policy name		  策略的名#policy args		  后续策略参数的数量（必须为偶数）
policy args		  值对，例sequential_threshold
cache metadata mode       ro 表示只读，rw 表示读写

			  在严重情况下，即使只读模式也被视为不安全时，
			  将不再允许任I/O，状态中将只包含字符'Fail'			  此时应使用用户空间的恢复工具needs_check		  'needs_check' 表示已设置，'-' 表示未设			  一次元数据操作失败，导致元数据的超级块中设置了
			  needs_check 标志。在缓存能够完全恢复运行之前			  必须停用该元数据设备并进行检修复			  '-' 表示未设needs_check========================= =====================================================

### 消息

策略会有各自不同的、特定于该策略的可调参数，因此我们需要一种通用的获取和设置它们的方式。这里使device-mapper 消息。（sysfs 接口也是可行的。）

```
   <key> <value>

```
```
   dmsetup message my_cache 0 sequential_threshold 1024


```
失效（Invalidation）是指从缓存中移除一个条目而不将其写回。缓存块可以通过 invalidate_cblocks 消息而失效，该消息接受任意数量的 cblock 范围。每cblock 范围的结束值是"末尾之后的一个，即 5-10 表示一个从 5 9 的值范围。每cblock 必须表示为十进制值，未来可能需要一个变体消息，接受以十六进制表示的 cblock 范围，以更好地支持对更大缓存的高效失效。缓存必须处passthrough 模式
```
   invalidate_cblocks [<cblock>|<cblock begin>-<cblock end>]*

```
```
   dmsetup message my_cache 0 invalidate_cblocks 2345 3456-4567 5678-6789

```
## 示例

测试套件可以在这里找到：

https://github.com/jthornber/device-mapper-test-suite

```
  dmsetup create my_cache --table '0 41943040 cache /dev/mapper/metadata \
	  /dev/mapper/ssd /dev/mapper/origin 512 1 writeback default 0'
  dmsetup create my_cache --table '0 41943040 cache /dev/mapper/metadata \
	  /dev/mapper/ssd /dev/mapper/origin 1024 1 writeback \
	  mq 4 sequential_threshold 1024 random_threshold 8'

```
