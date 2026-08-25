
## dm-clone


## 简

dm-clone 是一个设备映射器目标，它将一个已存在的、只读的源设备一对一地拷贝到一个可写的目地设备中：它呈现一个虚拟块设备，使所有数据立即可见，并相应地对读写进行重定向
dm-clone 的主要用例是将一个可能位于远程、高延迟、只读、归档类型的块设备，克隆到一个可写、快速、主类型的设备中，以获得快速、低延迟I/O。克隆后的设备立即可可挂载，而源设备到目地设备的拷贝在后台进行，与用I/O 并行
例如，可以将一个通过网络安全存储协议（NBD、Fibre Channel、iSCSI、AoE 等）访问的只读副本中的应用备份，恢复到本地的 SSD NVMe 设备，并立即开始使用该设备，而无需等待恢复完成
当克隆完成时，可以彻底移dm-clone 表，并替换为例如直接映射到目地设备的线性（linear）表
dm-clone 目标复用了瘦供给（thin-provisioning）目标所使用的元数据库

## 术语

   Hydration（水合）
     将目地设备的一个区域用来自源设备同一区域的数据填满的过程，即，将该区域从源设备拷贝到目地设备
一旦某个区域被水合（hydrated），我们就将该区域的所I/O 重定向到目地设备

## 设计


### 子设

该目标通过向它传入三个设备（以及稍后详述的其它参数）来构建
1. 一个源设备——被克隆的只读设备，也是水合的来源
2. 一个目地设备——水合的目的地，它将成为源设备的一个克隆
3. 一个小的元数据设备——它记录哪些区域在目地设备中已经有效，即哪些区域已经被水合，或者已经通过用户 I/O 被直接写入
目地设备的大小必须至少等于源设备的大小

### 区域（Regions

dm-clone 将源设备和目地设备划分为固定大小的区域。区域是水合的单位，即从源设备拷贝到目地设备的最小数据量
区域大小在你首次创建 dm-clone 设备时可配置。推荐的区域大小与文件系统块大小相同，通常4KB。区域大小必须在 8 个扇区（4KB）到 2097152 个扇区（1GB）之间，且为 2 的幂
对已水合区域的读写由目地设备提供服务
对尚未水合区域的读直接从源设备提供服务
对尚未水合区域的写将被延迟，直到相应的区域已被水合并立即开始水合
注意，大小等于区域大小的写请求将跳过从源设备拷贝相应区域，并直接覆盖目地设备的该区域

### 丢弃（Discards

dm-clone 将针对尚未水合范围的 discard 请求解释为跳过这些被请求覆盖区域的水合的提示，即，它跳过将该区域的数据从源设备拷贝到目地设备，而只更新其元数据
如果目地设备支持 discard，那么默认情况下 dm-clone 会将 discard 请求向下透传到它

### 后台水合


dm-clone 持续地从源设备拷贝到目地设备，直到整个设备都被拷贝完成
从源设备到目地设备拷贝数据会占用带宽。用户可以设置一个节流值，以防止在任何时刻发生超过一定数量的拷贝。此外，dm-clone 会考虑发往这些设备的用I/O 流量，并在有 I/O 在途时暂停后台水合
可以使用消息 `hydration_threshold <#regions>` 来设置被拷贝区域的最大数量，默认1 个区域
dm-clone 使用 dm-kcopyd 来将源设备的部分内容拷贝到目地设备。默认情况下，我们发出大小等于区域大小的拷贝请求。可以使用消`hydration_batch_size <#regions>` 来调整这些拷贝请求的大小。增大水合批大小会使 dm-clone 尝试将连续的多个区域合并成批，从而我们成批地（每批这么多个区域）从源设备拷贝数据到目地设备
当目地设备的水合完成时，会向用户空间发送一dm 事件

### 更新磁盘上的元数

每次写入一FLUSH FUA bio 时，磁盘上的元数据都会被提交。如果没有发出此类请求，则提交会每一秒发生一次。这意味着 dm-clone 设备的行为类似于一个带有易失性写缓存的物理磁盘。如果断电，你可能会丢失一些最近的写入。尽管如此，在任何崩溃之后元数据都应当保持一致

## 目标接口


### 构造函

```
   clone <metadata dev> <destination dev> <source dev> <region size>
         [<#feature args> [<feature arg>]* [<#core args> [<core arg>]*]]

 ================ ==============================================================
 metadata dev     Fast device holding the persistent metadata
 destination dev  The destination device, where the source will be cloned
 source dev       Read only device containing the data that gets cloned
 region size      The size of a region in sectors

 #feature args    Number of feature arguments passed
 feature args     no_hydration or no_discard_passdown

 #core args       An even number of arguments corresponding to key/value pairs
                  passed to dm-clone
 core args        Key/value pairs passed to dm-clone, e.g. `hydration_threshold
                  256`
 ================ ==============================================================

```
可选的特性参数有
 ==================== =========================================================
 no_hydration         创建一个禁用了后台水合dm-clone 实例
 no_discard_passdown  禁止discard 向下透传到目地设 ==================== =========================================================

可选的核心参数有：

 ================================ ==============================================
 hydration_threshold <#regions>   在后台水合期间，任何时刻从源设备拷贝到目                                  设备的区域的最大数量 hydration_batch_size <#regions>  在后台水合期间，尝试将连续的多个区域合并                                  批，从而我们成批地（每批这么多个区域）从源
                                  设备拷贝数据到目地设备 ================================ ==============================================

### 状

```
   <metadata block size> <#used metadata blocks>/<#total metadata blocks>
   <region size> <#hydrated regions>/<#total regions> <#hydrating regions>
   <#feature args> <feature args>* <#core args> <core args>*
   <clone metadata mode>

 ======================= =======================================================
 metadata block size     Fixed block size for each metadata block in sectors
 #used metadata blocks   Number of metadata blocks used
 #total metadata blocks  Total number of metadata blocks
 region size             Configurable region size for the device in sectors
 #hydrated regions       Number of regions that have finished hydrating
 #total regions          Total number of regions to hydrate
 #hydrating regions      Number of regions currently hydrating
 #feature args           Number of feature arguments to follow
 feature args            Feature arguments, e.g. `no_hydration`
 #core args              Even number of core arguments to follow
 core args               Key/value pairs for tuning the core, e.g.
                         `hydration_threshold 256`
 clone metadata mode     ro if read-only, rw if read-write

                         In serious cases where even a read-only mode is deemed
                         unsafe no further I/O will be permitted and the status
                         will just contain the string 'Fail'. If the metadata
                         mode changes, a dm event will be sent to user space.
 ======================= =======================================================

```
### 消息


  `disable_hydration`
      禁用目地设备的后台水合
  `enable_hydration`
      启用目地设备的后台水合
  `hydration_threshold <#regions>`
      设置后台水合阈值
  `hydration_batch_size <#regions>`
      设置后台水合批大小

## 示例


### 克隆一个包含文件系统的设备


1. 创建 dm-clone 设备
```

    dmsetup create clone --table "0 1048576000 clone $metadata_dev $dest_dev \
      $source_dev 8 1 no_hydration"

```
2. 挂载该设备并对文件系统进trim。dm-clone 会解释文件系统发出的 discard，并不会对未使用的空间进行水合
```

    mount /dev/mapper/clone /mnt/cloned-fs
    fstrim /mnt/cloned-fs

```
3. 启用目地设备的后台水合
```

    dmsetup message clone 0 enable_hydration

```
4. 当水合完成时，我们可以用一个线性表替换 dm-clone 表
```

    dmsetup suspend clone
    dmsetup load clone --table "0 1048576000 linear $dest_dev 0"
    dmsetup resume clone

   The metadata device is no longer needed and can be safely discarded or reused
   for other purposes.

```
## 已知问题


1. 我们将对尚未水合区域的读重定向到源设备。如果读取源设备的延迟很高，而用户反复读取相同的区域，这种行为会降低性能。我们应该将这些读作为提示，以便尽快水合相关区域。目前我们依赖页缓存来缓存这些区域，所以希望我们不会从源设备多次读取它们
2. 在水合完成后，释放核心内资源（即跟踪哪些区域已被水合的位图）
3. 在后台水合期间，如果我们无法读取源设备或写入目地设备，我们会打印一条错误消息，但水合过程会无限期地继续，直到成功为止。我们应该在失败若干次后停止后台水合，并发出一dm 事件以便用户空间察觉

## 为什么不用……？


在实dm-clone 之前，我们探讨了以下替代方案
1. 使用 dm-cache，其缓存大小等于源设备，并实现一种新的克隆策略：

   - 生成的缓存设备并不是源设备的一对一镜像，因此我们在克隆完成后无法移除缓存设备
   - dm-cache 会写入源设备，这违反了我们要求源设备必须被视为只读的条件
   - 缓存与克隆在语义上是不同的
2. 使用 dm-snapshot，其 COW 设备等于源设备：

   - dm-snapshot 将其元数据存储在 COW 设备中，因此生成的设备并不是源设备的一对一镜像
   - 没有后台拷贝机制
   - dm-snapshot 需要在每个待处理异常（pending exception）完成时提交其元数据，以保证快照一致性。而在克隆的情况下，我们不需要如此严格，可以依赖每次写入 FLUSH FUA bio 时提交元数据，或者周期性地提交，就dm-thin dm-cache 所做的那样。这显著提升了性能
3. 使用 dm-mirror：mirror 目标有后台拷镜像机制，但它会写入所有的镜像，从而违反了我们要求源设备必须被视为只读的条件
4. 使用 dm-thin 的外部快照功能。在所有替代方案中，这种方法最有前途，因为瘦供给卷是源设备的一对一镜像，并且以dm-clone 相同的方式处理对未供尚未克隆区域的读写
   尽管如此
   - 没有后台拷贝机制，尽管可以实现一个
   - 最重要的是，我们希望支持任意块设备作为克隆过程的目地，而不是将自己限制在瘦供给卷上。瘦供给为了维护瘦卷映射而具有固有的元数据开销，这会显著降低性能
   此外，克隆一个设备不应强制使用瘦供给。另一方面，如果我们希望使用瘦供给，我们只需使用一个瘦 LV 作为 dm-clone 的目地设备即可