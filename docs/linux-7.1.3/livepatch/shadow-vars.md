## 影子变量（Shadow Variables

影子变量（Shadow Variables）是一种让 livepatch 模块将额外的“影子”数据与已有数据结构关联起来的简单方式。影子数据独立于父数据结构分配，父数据结构保持不变。本文档描述的影子变API 用于将影子变量分添加以及移除/释放到其父对象
该实现引入了一个全局的内核态哈希表，将指向父对象的指针与影子数据的数字标识符关联起来。该数字标识符是一个简单的枚举，可用于描述影子变量的版本、类别或类型等。更具体地说，父指针作为哈希表的键，而数id 随后用于过滤哈希表查询。多个影子变量可以附加到同一个父对象，但它们的数字标识符将它们区分开来

## 1. 简API 概要


（完整的 API 使用 docbook 说明livepatch/shadow.c。）

一个哈希表引用了所有影子变量。这些引用通过 <obj, id> 对来存储和检索
- `klp_shadow` 变量数据结构同时封装了跟踪元数据和影子数据：

  - 元数
    - obj - 指向父对象的指针
    - id - 鏁版嵁鏍囪瘑绗。
  - data[] - 影子数据的存储空
需要注意的是，`klp_shadow_alloc()` `klp_shadow_get_or_alloc()` 默认会将变量清零。当需要一个非零值时，它们也允许调用一个自定义的构造函数。调用者应当提供所需的任何互斥保护
注意，构造函数在 klp_shadow_lock 自旋锁下调用。它允许执行那些在分配新变量时只能做一次的操作
- klp_shadow_get() - 检索一个影子变量数据指  - 在哈希表中搜<obj, id> 
- klp_shadow_alloc() - 分配并添加一个新的影子变  - 在哈希表中搜<obj, id> 
  - 如果存在

    - 璀﹀憡骞惰繑鍥?NULL

  - 如果 <obj, id> 尚不存在

    - 分配一个新的影子变    - 如果提供了自定义构造函数和数据，则使用它们初始化该变量
    - <obj, id> 添加到全局哈希
- klp_shadow_get_or_alloc() - 获取已有的或分配一个新的影子变  - 在哈希表中搜<obj, id> 
  - 如果存在

    - 返回已有的影子变
  - 如果 <obj, id> 尚不存在

    - 分配一个新的影子变    - 如果提供了自定义构造函数和数据，则使用它们初始化该变量
    - <obj, id> 对添加到全局哈希
- klp_shadow_free() - 分离并释放一<obj, id> 影子变量
  - 从全局哈希表中查找并移除一<obj, id> 引用

    - 如果找到

      - 濡傛灉瀹氫箟浜嗘瀽鏋勫嚱鏁板垯璋冪敤瀹?      - 閲婃斁褰卞瓙鍙橀噺

- klp_shadow_free_all() - 分离并释放所<_, id> 影子变量
  - 从全局哈希表中查找并移除任<_, id> 引用

    - 如果找到

      - 濡傛灉瀹氫箟浜嗘瀽鏋勫嚱鏁板垯璋冪敤瀹?      - 閲婃斁褰卞瓙鍙橀噺


## 2. 使用场景


（完整的可运行演示请参见 samples/livepatch/ 中的影子变量 livepatch 模块示例。）

对于以下使用场景示例，请考虑提交 1d147bfa6429（“mac80211: fix AP powersave TX vs. wakeup race”），它net/mac80211/sta_info.h 添加了一**spinlock（自旋锁*：struct sta_info。每个使用场景示例都可以视为该修复的一个独livepatch 实现

### 匹配父对象的生命周期


如果父数据结构频繁地被创建和销毁，最简单的方法可能是将它们的影子变量生命周期对齐到相同的分配和释放函数。在这种情况下，父数据结构通常会被分配、初始化，然后以某种方式注册。影子变量的分配和设置可视为父对象初始化的一部分，并且应在父对象“上线”（即对<obj, id> 对发出任何影子变get-API 请求）之前完成
对于提交 1d147bfa6429，当分配一个父 sta_info 结构时，
```

  #define PS_LOCK 1
  struct sta_info *sta_info_alloc(struct ieee80211_sub_if_data *sdata,
				  const u8 *addr, gfp_t gfp)
  {
	struct sta_info *sta;
	spinlock_t *ps_lock;

	/* Parent structure is created */
	sta = kzalloc(sizeof(*sta) + hw->sta_data_size, gfp);

	/* Attach a corresponding shadow variable, then initialize it */
	ps_lock = klp_shadow_alloc(sta, PS_LOCK, sizeof(*ps_lock), gfp,
				   NULL, NULL);
	if (!ps_lock)
		goto shadow_fail;
	spin_lock_init(ps_lock);
	...

```
当需要一ps_lock 时，查询影子变量 API 来检索一```

  void ieee80211_sta_ps_deliver_wakeup(struct sta_info *sta)
  {
	spinlock_t *ps_lock;

	/* sync with ieee80211_tx_h_unicast_ps_buf */
	ps_lock = klp_shadow_get(sta, PS_LOCK);
	if (ps_lock)
		spin_lock(ps_lock);
	...

```
当父 sta_info 结构被释放时，先释放影子变量
```

  void sta_info_free(struct ieee80211_local *local, struct sta_info *sta)
  {
	klp_shadow_free(sta, PS_LOCK, NULL);
	kfree(sta);
	...


```

### 在途父对象


有时，与其父对象一起分配影子变量可能不方便或不可行。或者，一livepatch 修复可能只需要为父对象实例的一个子集设置影子变量。在这些情况下，可以使用 klp_shadow_get_or_alloc() 调用来将影子变量附加到已经在途的父对象上
对于提交 1d147bfa6429，分配影子自旋锁的一个合适位置是
```

  int ps_lock_shadow_ctor(void *obj, void *shadow_data, void *ctor_data)
  {
	spinlock_t *lock = shadow_data;

	spin_lock_init(lock);
	return 0;
  }

  #define PS_LOCK 1
  void ieee80211_sta_ps_deliver_wakeup(struct sta_info *sta)
  {
	spinlock_t *ps_lock;

	/* sync with ieee80211_tx_h_unicast_ps_buf */
	ps_lock = klp_shadow_get_or_alloc(sta, PS_LOCK,
			sizeof(*ps_lock), GFP_ATOMIC,
			ps_lock_shadow_ctor, NULL);

	if (ps_lock)
		spin_lock(ps_lock);
	...

```
这种用法会在需要时创建一个影子变量，否则会使用已经为<obj, id> 对创建的那个
与前面的使用场景类似，影子自旋锁需要被清理。影子变量可以在其父对象被释放之前释放，甚至可以在影子变量本身不再需要时释放

### 其他使用场景


影子变量也可以用作一个标志，表明某个数据结构是由新的、经livepatch 的代码分配的。在这种情况下，影子变量持有何种数据值并不重要，它的存在本身就暗示了如何处理父对象

## 3. 参考资

- https://github.com/dynup/kpatch

  livepatch 实现基于 kpatch 版本的影子变量
- http://files.mkgnu.net/files/dynamos/doc/papers/dynamos_eurosys_07.pdf

  《Dynamic and Adaptive Updates of Non-Quiescent Subsystems in Commodity Operating System Kernels》（Kritis Makris、Kyung Dong Ryu007）提出了一种称为“影子数据结构（shadow data structures）”的数据类型更新技术