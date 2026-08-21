
## 待办事项列表


本节包含内核 DRM 图形子系统中一系列较小的“日常清理”任务，适合作为新手项目，或在没有紧急工作的闲暇日子来做
### 难度


为了方便挑选任务，将任务划分为不同的难度级别：

Starter（入门）：适合开始接DRM 子系统的好任务
Intermediate（中级）：需要一些在 DRM 子系统中工作的经验，或某些特定的 GPU/显示图形知识。调试问题时，最好手头有相关硬件（或已配置好的虚拟驱动）可供测试
Advanced（高级）：棘手的任务，需要对 DRM 子系统和图形主题有较好的理解。通常需要有相关硬件用于开发与测试
Expert（专家）：只有当你已经成功完成过一些棘手的重构工作、并且是该特定领域的专家时，才尝试这些任务
## 子系统级重构


### 内联展开 drm_simple_encoder_init()

辅助函数 `drm_simple_encoder_init()` 原本是为了简encoder 的初始化。但它实际上只是atomic modesetting DRM 驱动之间增加了一层中间层
这里的任务就是移`drm_simple_encoder_init()`。找到一个调`drm_simple_encoder_init()` 的驱动，将该辅助函数内联展开。该驱动还需要自己的 `drm_encoder_funcs` 实例
联系人：Thomas Zimmermann，相关驱动维护
难度：Easy

### 用常atomic helper 替换 struct drm_simple_display_pipe


数据类型 `struct drm_simple_display_pipe` 及其辅助函数原本是为了简化驱动开发。但它们实际上只是在 atomic modesetting DRM 驱动之间增加了一层中间层
仍有一些驱动在使用 `drm_simple_display_pipe`。这里的任务是将它们转换为使用常规的 atomic helper。找到一个调`drm_simple_display_pipe_init()` 的驱动，`drm_simple_kms_helper.c` 中的所有辅助函数内联到该驱动中，从而不再需simple-KMS 接口。同时请按照驱动约定重命名所有内联的函数
联系人：Thomas Zimmermann，相关驱动维护
难度：Easy

### 移除自定义的 dumb_map_offset 实现


所有基GEM 的驱动都应改为使`drm_gem_create_mmap_offset()`。逐个审查各个驱动，确保它们与通用实现兼容（各种实现中遗留了大量过时的加锁代码），然后移除自定义实现
联系人：Simona Vetter，相关驱动维护
难度：Intermediate

### 将现KMS 驱动转换atomic modesetting


3.19 已经提供atomic modeset 接口与辅助函数，因此现在可以转换驱动了。像 Wayland Android 上的 Surfaceflinger 这样的现代合成器非常需atomic modeset 接口，所以这一切都关乎美好的未来
关于 atomic 转换，有一份转换指[^1^]_，你所需要的只是一个尚未转换的驱动对应GPU。LWN.net 上的“Atomic mode setting design overview”系列文[^2^]_ [^3^]_ 也很有帮助
作为转换的一部分，驱动还需要转换为 universal plane（即primary cursor 作为正规plane 对象暴露出来）。不过这通过直接使用新的 atomic helper 驱动的回调来做要容易得多
  .. [^1^] https://blog.ffwll.ch/2014/11/atomic-modeset-support-for-kms-drivers.html
  .. [^2^] https://lwn.net/Articles/653071/
  .. [^3^] https://lwn.net/Articles/653466/

联系人：Simona Vetter，相关驱动维护
难度：Advanced

### 清理围绕 plane 的裁剪坐标混

我们有一个辅助函`drm_plane_helper_check_update()` 可以正确处理这个问题，但它没有被一致地使用。这个问题应当被修复，最好在 atomic helper 中修复（然后驱动再切换到裁剪后的坐标）。可能还应该把这个辅助函数从 `drm_plane_helper.c` 移到 atomic helper 中，以避免混淆——那个文件中的其他辅助函数都是已过时的遗helper
联系人：Ville Syrjälä，Simona Vetter，驱动维护
难度：Advanced

### 改进 plane atomic_check helper


除了上面提到的裁剪坐标外，当前的 helper 还有一些不够理想的地方
- `drm_plane_helper_funcs->atomic_check` 会对启用或禁用的 plane 都被调用。往好了说这会让驱动感到困惑，往坏了说这意味着plane 在没CRTC 的情况下被禁用时，驱动会崩溃。唯一的特殊处理是plane state 结构体中重置数值，而这些重置应当移`drm_plane_funcs->atomic_duplicate_state` 函数中
- 一旦完成上述工作，helper 就可以停止对禁用plane 调用 `->atomic_check`
- 然后我们可以遍历所有驱动，移除那些多少有些令人困惑`plane_state->fb` `plane_state->crtc` 检查
联系人：Simona Vetter

难度：Advanced

### 将早期的 atomic 驱动转换async commit helper


在头一年，atomic modeset helper 不支持异非阻塞提交（nonblocking commit），每个驱动都不得不自己手写。现在这个问题已经修复，但仍有大量现有驱动可以轻松转换到新的基础设施上
这些 helper 的一个问题是，它们要求驱动正确处atomic commit 的完成事件。但修复这些 bug 无论如何都是好事
与此多少相关的是 `legacy_cursor_update` 这个 hack，在使用该标志的驱动中，应该helper 中新`atomic_async_check`/`commit` 功能来替换它
联系人：Simona Vetter，相关驱动维护
难度：Advanced

### 重命drm_atomic_state


KMS 框架`state` 这个概念使用了两种略有不同的定义。对于某个给定对象（plane、CRTC、encoder 等，`drm_$OBJECT_state`），state 是该对象的完整状态。然而，在设备级别，`drm_atomic_state` 指的是对有限数量对象的一次状态更新
这个 state 并不是整个设备的状态，而只是该设备中某些对象的完整状态。这会让 newcomers 感到困惑，因`drm_atomic_state` 应当被重命名为更清晰的名字，例如 `drm_atomic_commit`
除了重命名结构体本身之外，这也意味着要重命名一些相关函数（`drm_atomic_state_alloc`、`drm_atomic_state_get`、`drm_atomic_state_put`、`drm_atomic_state_init`、`__drm_atomic_state_free` 等）
联系人：Maxime Ripard <mripard@kernel.org>

难度：Advanced

### atomic KMS 的后续工

`drm_atomic_helper.c` 提供了一批在atomic 驱动接口之上实现遗留 IOCTL 的函数。这对于驱动的逐步转换非常有用，但遗憾的是二者的语义不匹配过于严重。因此需要一些后续工作来调整函数接口以修复这些问题：

- atomic 需要锁获取上下文。目前这是用一些糟糕的 hack 隐式传递的，并且在幕后还用 `GFP_NOFAIL` 分配。所有遗留路径都需要显式地在栈上分acquire context，然后显式地将其传入驱动，以便基atomic 的遗留函数可以使用它们
  除了一些驱动代码外，这项工作已经完成。应该通过`drm_modeset_lock_all()` 中加`WARN_ON(!drm_drv_uses_atomic_modeset)` 来完成这个任务
- 一大批 vtable hook 现在放在了错误的位置：DRM 在核vfunc 表（命名`drm_foo_funcs`，用于实现用户空ABI）与 helper 库的可hook（命名为 `drm_foo_helper_funcs`，仅供内部使用）之间做了划分。其中一hook 应该`_funcs` 移到 `_helper_funcs`，因为它们不属于核心 ABI。对于每种这样的情况，`drm_crtc.h` kerneldoc 中都有一`FIXME` 注释
联系人：Simona Vetter

难度：Intermediate

### Buffer Object 加锁迁移dma_resv_lock()


许多驱动有自己的按对象加锁方案，通常使用 `mutex_lock()`。这buffer 共享带来了各种各样的麻烦，因为根据驱动是 exporter 还是 importer，加锁层级会颠倒过来
为了解决这个问题，我们需要一个标准的按对象加锁机制，`dma_resv_lock()`。这个锁需要作为最外层锁来调用，同时移除所有其他驱动特定的按对象锁。问题在于，由于 struct `dma_buf` buffer 共享，实际推行加锁约定的变更会是一个“flag day”（需要一次性切换）
难度：Expert

### 将日志输出转换为drm_device 参数drm_* 函数


对于可能存在多个实例的驱动，需要在日志中区分哪个是哪个。由`DRM_INFO`/`WARN`/`ERROR` 做不到这一点，驱动使用 `dev_info`/`warn`/`err` 来做这种区分。现在我们有drm 打印函数`drm_*` 变体，因此可以让这些驱动重新改回使用 drm 格式的特定日志消息
在开始这种转换之前，请联系相关维护者，以确保你的工作会被合入——并不是所有人都认DRM dmesg 宏更好
联系人：Sean Paul，你计划转换的驱动的维护
难度：Starter

### 将驱动转换为使用简单的 modeset suspend/resume


大多数驱动（i915 nouveau 除外）如果使`drm_atomic_helper_suspend`/`resume()`，可能可以转换为使用 `drm_mode_config_helper_suspend`/`resume()`。此外，早期atomic modeset 驱动中仍然存在手工编写的 atomic suspend/resume 代码
联系人：你计划转换的驱动的维护
难度：Intermediate

### 在不依赖 fbdev 的情况下重新实现 drm_fbdev_fb_ops 中的函数


`drm_fbdev_fb_ops` 中的许多回调函数可以从不依赖 fbdev 模块的角度重写中受益。其中一helper 还可以通过使用 `struct iosys_map` 而非裸指针来进一步受益
联系人：Thomas Zimmermann <tzimmermann@suse.de>，Simona Vetter

难度：Advanced

### blitting 与格式转换函数进行基准测试与优化


快速绘制到显示内存对于许多应用程序的性能至关重要
至少x86-64 上，`sys_imageblit()` 明显`cfb_imageblit()` 慢，尽管二者使用相同的 blitting 算法，而且后者是I/O 内存编写的。结果发`cfb_imageblit()` 使用`movl` 指令，`sys_imageblit` 显然没有。这似乎gcc 优化器的一个问题。DRM 的格式转helper 也可能存在类似问题
fbdev `sys_()` helper DRM 的格式转helper 进行基准测试并优化。在可以进一步优化的地方，也许可以实现一种不同的算法。对于微优化，显式使`movl`/`movq` 指令。这可能需要架构特定的 helper（例`storel()`、`storeq()`）
联系人：Thomas Zimmermann <tzimmermann@suse.de>

难度：Intermediate

### drm_framebuffer_funcs drm_mode_config_funcs.fb_create 清理


还有更多驱动可以切换`drm_gem_framebuffer` helper。存在各种阻碍因素：

- 需要先切换到使用通用脏跟踪代码，`drm_atomic_helper_dirtyfb`（例qxl）
- 需要切换到 `drm_fbdev_generic_setup()`，否则大量自定义fb 设置代码无法删除
- 需要切换到 `drm_gem_fb_create()`，因为现`drm_gem_fb_create()` 会为 atomic 驱动检查有效的格式
- 许多驱动`drm_framebuffer` 做了子类化，我们需要一个与之兼容的嵌入（embedding）版本的各类 `drm_gem_fb_create` 函数。也许根据需要命名为 `drm_gem_fb_create`/`_with_dirty`/`_with_funcs`
联系人：Simona Vetter

难度：Intermediate

### 通用fbdev defio 支持


fbdev 核心中的 defio 支持代码有一些非常具体的要求，这意味着驱动需要一个特殊的 framebuffer 用于 fbdev。主要问题在于它使用`struct page` 自身中的一些字段，这会破坏 shmem gem 对象（以及其他东西）。为了支defio，受影响的驱动需要使用一shadow buffer，这可能会增CPU 与内存开销
可能的解决方案是DRM fbdev 模拟中编写我们自己的 defio mmap 代码。它需要完全包裹现有的 mmap 操作，在完成了写保护/mkwrite 的技巧之后再转发一切：

- `drm_fbdev_fb_mmap` helper 中，如果我们需defio，则修改

```
      vma->vm_page_prot = pgprot_wrprotect(vma->vm_page_prot);

```

- 用与核心 fbdev defio 代码类似的实现来设置 mkwrite fsync 回调。这些都应当工作在普通的 pte 上，它们实际上并不需`struct page`。uff. 这些都应当工作在普通的 pte 上，它们实际上并不需`struct page`
- 在一个独立的结构体（每个页一bit 的位域应该可行）中跟踪脏页，以避免破`struct page`
最好也为这个准备一igt 测试用例
联系人：Simona Vetter，Noralf Tronnes

难度：Advanced

### connector 注册/注销修复


- 对于大多connector，直接从驱动代码中调`drm_connector_register`/`unregister` 是空操作，因`drm_dev_register`/`unregister` 已经处理了这件事。我们可以移除所有这些调用
- 对于 DP 驱动，情况要混乱一些，因为在调`drm_dp_aux_register` 时我们需connector 已经注册。可以通过改为调用 `drm_dp_aux_init`，并将实际的注册动作移入 `late_register` 回调（如 kerneldoc 中所建议）来修复
难度：Intermediate

### 移除 load/unload 回调


`struct &drm_driver` 中的 load/unload 回调很大程度上是中间层（midlayer），而且由于历史原因，它们在设置 `&drm_driver` 结构体与调用 `drm_dev_register()` 之间的顺序是错误的（而且我们无法修复这一点）
- 重新改造驱动，使其不再使用 load/unload 回调，而是load/unload 流程直接编码到驱动的 probe 函数中
- 一旦所有驱动都转换完成，移load/unload 回调
联系人：Simona Vetter

难度：Intermediate

### drm_display_info.is_hdmi 替换 drm_detect_hdmi_monitor()


一EDID 被解析，显示器的 HDMI 支持信息就可以通过 `drm_display_info.is_hdmi` 获取。许多驱动仍然调`drm_detect_hdmi_monitor()` 来获取相同的信息，效率较低
逐个审查调用 `drm_detect_hdmi_monitor()` 的各个驱动，如果适用则切换到 `drm_display_info.is_hdmi`
联系人：Laurent Pinchart，相关驱动维护
难度：Intermediate

### 整合各驱动自定义modeset 属

atomic modeset 出现之前，许多驱动都创建了自己的属性。除此之外，atomic 还带来了一个要求：不应使用自定义、驱动特定的属性
对于这个任务，我们的目标是引入核helper，或者在可用时复用已有的 helper
一份快速、未经确认的例子列表
引入核心 helper- audio（amdgpu、intel、gma500、radeon- brightness、contrast 等（armada、nouveau）——仅 overlay（？- broadcast rgb（gma500、intel- colorkey（armada、nouveau、rcar）——仅 overlay（？- dither（amdgpu、nouveau、radeon）——各驱动之间不同
- underscan 系列（amdgpu、radeon、nouveau
已在核心中：
- colorspace（sti- tv format 名称、增强（gma500、intel- tv overscan、margins 等（gma500、intel- zorder（omapdrm）——与 zpos 相同（？
联系人：Emil Velikov，相关驱动维护
难度：Intermediate

### 在整个代码库中使struct iosys_map


指向共享设备内存的指针存储在 `struct iosys_map` 中。每个实例都知道它指向的是系统内存还I/O 内存。大多数 DRM 范围内的接口已经转换为使`struct iosys_map`，但实现通常仍使用裸指针
任务是，在有意义的地方使`struct iosys_map`
- 内存管理器应dma-buf 导入的缓冲区使用 `struct iosys_map`- TTM 可能在内部使`struct iosys_map` 会受益- Framebuffer 复制blitting helper 应基`struct iosys_map` 操作
联系人：Thomas Zimmermann <tzimmermann@suse.de>，Christian König，Simona Vetter

难度：Intermediate

### 审查所有驱动是否正确设struct drm_mode_config.{max_width,max_height}


`struct drm_mode_config.{max_width,max_height}` 中的值描述了所支持的最framebuffer 尺寸。它是虚拟屏幕大小，但许多驱动把它当作物理分辨率的限制
最大宽度取决于硬件的最大扫描行 pitch。最大高度取决于可寻址显存的容量。审查所有驱动，将这两个字段初始化为正确的值
联系人：Thomas Zimmermann <tzimmermann@suse.de>

难度：Intermediate

### 在所fbdev 驱动中申请内存区

老旧/古老的 fbdev 驱动没有正确地申请它们的内存。遍历这些驱动，添加代码以申请驱动所使用的内存区域。这需要添加对 `request_mem_region()`、`pci_request_region()` 或类似函数的调用。尽可能使用带托管的（managed）清helper。存在问题的区域包括VGA 这样拥有独占范围的硬件。VGA16fb 没有像预期的那样申请该范围。驱动在做这件事上相当糟糕，而且 DRM fbdev 驱动之间曾经有过冲突。不过，这样做是正确的
联系人：Thomas Zimmermann <tzimmermann@suse.de>

难度：Starter

### 移除驱动FB_DEVICE 的依

许多 fbdev 驱动通过 sysfs 提供属性，因此依赖于选中 `CONFIG_FB_DEVICE`。审查每个驱动，并尝试使任何`CONFIG_FB_DEVICE` 的依赖变为可选。至少，驱动中对应的代码可以通过 `ifdef CONFIG_FB_DEVICE` 进行条件编译。并非所有驱动都能去`CONFIG_FB_DEVICE`
联系人：Thomas Zimmermann <tzimmermann@suse.de>

难度：Starter

### 移除 panel-simple panel-edp remove/shutdown 中的 disable/unprepare


根据提交 d2aacaf07395（“drm/panel: Check for already prepared/enabled in drm_panel”），我们在 `drm_panel` 核心中增加了一个检查，以确保不会有人重复调prepare/enable/disable/unprepare。最终这可能应该变成一`WARN_ON()`，或者以某种方式让提示更明显
目前，我们预计在使用 panel-simple panel-edp 时仍可能`drm_panel` 核心中遇到这些警告。由于这panel 驱动与许多不同的 DRM modeset 驱动一起使用，它们仍会额外shutdown 时自disable/unprepare panel。具体来说，如果 panel 驱动DRM modeset 驱动 *之前* `shutdown()`，DRM modeset 驱动在其自身`shutdown()` 回调中正确地调用`drm_atomic_helper_shutdown()`，我们仍可能遇到这些警告。在这种情况下，可以通过使用类似 device link 的机制来确保 panel DRM modeset 驱动之后`shutdown()`，从而避免警告
一旦已知所DRM modeset 驱动都能正确 shutdown，就应该移除 panel-simple panel-edp remove/shutdown 中对 disable/unprepare 的额外调用，并将TODO 项标记为完成
联系人：Douglas Anderson <dianders@chromium.org>

难度：Intermediate

### 摆脱已弃用的 MIPI DSI 函数


`drm_mipi_dsi.c` 中定义了许多已被弃用的函数。每个被弃用的函数都是为了让位于`multi` 变体（例`mipi_dsi_generic_write()` `mipi_dsi_generic_write_multi()`）。函数的 `multi` 变体包含了改进的错误处理逻辑，并使连续进行多次调用更加方便，就像大多MIPI 驱动所做的那样
驱动应当更新为使用未弃用的函数。一旦所有对已弃MIPI DSI 函数的使用都被移除，它们的定义就可以`drm_mipi_dsi.c` 中删除
联系人：Douglas Anderson <dianders@chromium.org>

难度：Starter

### 移除 devm_drm_put_bridge()


由于 panel bridge 处理 `drm_bridge` 对象生命周期的方式，在移`panel_bridge` 时必须特别小心地释放 `drm_bridge` 对象。目前这通过 `devm_drm_put_bridge()` 来管理，但那是一个不安全、临时的权宜之计。要修复这个问题，需要重新设DRM panel 的生命周期。重新设计完成之后，移除 `devm_drm_put_bridge()` 以及 `drm_panel_bridge_remove()` 中的 TODO
联系人：Maxime Ripard <mripard@kernel.org>         Luca Ceresoli <luca.ceresoli@bootlin.com>

难度：Intermediate

### of_drm_find_bridge() 的使用者转换为 of_drm_find_and_get_bridge()


获取一`struct drm_bridge` 指针需要取得一个引用，并在释放该指针后归还它。大多数返回 `struct drm_bridge` 指针的函数已经调`drm_bridge_get()` 来增加引用计数，并且它们的使用者已经更新为在适当的时候调`drm_bridge_put()`。`of_drm_find_bridge()` 不会取得引用，它已被弃用，由会取得引用的 `of_drm_find_and_get_bridge()` 取代，但一些使用者仍需要被转换
联系人：Maxime Ripard <mripard@kernel.org>         Luca Ceresoli <luca.ceresoli@bootlin.com>

难度：Intermediate

## 核心重构


### panic 处理正常工作


这是一项内容非常多样的任务，包含许多零碎的小工作：

- panic 路径目前无法被测试，导致它不断出问题。这里的主要问题panic 可以hardirq 上下文中触发，因此所有与 panic 相关的回调都可能hardirq 上下文中运行。如果至少能通过例如 drm debugfs 文件触发调用来测fbdev helper 代码与驱动代码，那会很棒。hardirq 上下文可以通过向本地处理器发送一IPI 来实现
- 各种 panic handler 之间存在巨大的混乱。DRM fbdev 模拟 helper 曾经有自己的（早已移除），但除此之外 fbcon 代码本身也有一个。我们需要确保它们不再互相争抢。目前的权宜之计是在进入 DRM fbdev 模拟 helper 的各个入口点检`oops_in_progress`。这里更干净的做法是fbcon 切换`threaded printk support <https://lwn.net/Articles/800946/>`_
- `drm_can_sleep()` 是一团乱麻。它掩盖了正常操作中的真bug，并且对panic 路径来说也不是一个完整的解决方案。我们需要确保它仅在真正发生 panic 时才返回 true，并修复所有因此产生的问题
- panic handler 绝不能休眠，这也意味着它不能调`mutex_lock()`。它也不能无条件地获取任何其他锁，甚至包括自旋锁（因NMI hardirq 也可能发panic）。我们需要确保要么不调用这样的路径，要么对所有地方都使用 trylock。这真的很棘手
- 一个干净的解决方案是KMS 中提供一个完全独立的 panic 输出支持，绕过当前的 fbcon 支持。参`[PATCH v2 0/3] drm: Add panic handling <https://lore.kernel.org/dri-devel/20190311174218.51899-1-noralf@tronnes.org/>`_
- 将实际的 oops 以及之前dmesg 编码成二维码（QR），可能有助于解决“重要内容被滚走”这个令人头疼的问题。参`[RFC][PATCH] Oops messages transfer using QR codes <https://lore.kernel.org/lkml/1446217392-11981-1-git-send-email-alexandru.murtaza@intel.com/>`_ 中一些可以复用的示例代码
联系人：Simona Vetter

难度：Advanced

### 清理 debugfs 支持


它存在一堆问题：

- 将驱动转换为支持 `drm_debugfs_add_files()` 函数，而不`drm_debugfs_create_files()` 函数
- 通过connector crtc 也推行同样的 debugfs 预注册基础设施，改late-register debugfs。这样，驱动就无需再将它们的设置代码拆分为 init register 两部分
- 我们可能希望在核心中crtc/connector 以及也许其他 KMS 对象直接提供一debugfs 文件支持。这些对象的 funcs 中甚至有 `drm_print` 支持来转KMS 状态，所以一切都已经就位。然`->show()` 函数显然应该给你一个指向正确对象的指针
- 我们现有`drm_driver->debugfs_init` hook 只是旧的、中间层化的 load 流程的一个遗留物。DRM debugfs 应该更像 sysfs，你可以在任何想要的时候为某个对象创建属文件，由核心负责register/unregister 时发取消发布所有这些文件。驱动不应该需要操心这些技术细节，修复这个问题（连`drm_minor->drm_device` 的迁移）将使我们能够移除 `debugfs_init`
联系人：Simona Vetter

难度：Intermediate

### 对象生命周期修复


这里有两个相关的问题
- 清理各种各样`->destroy` 回调，这些回调通常都是相同的一段简单代码
- 大量驱动错误地使`devm_kzalloc` 分配 DRM modeset 对象，这会在驱动卸载时导use-after-free 问题。即便对于硬件集成在 SoC 上的驱动，由`EPROBE_DEFERRED` 回退，这也可能带来严重麻烦
这两个问题都可以通过切换`drmm_kzalloc()` 以及提供的各种便利包装器来解决，例如 `drmm_crtc_alloc_with_planes()`、`drmm_universal_plane_alloc()` 等等
联系人：Simona Vetter

难度：Intermediate

### dma-buf 导入中移除自动页映射


在导dma-buf 时，dma-buf PRIME 框架会自动将导入的页映射importer DMA 区域。`drm_gem_prime_fd_to_handle()` `drm_gem_prime_handle_to_fd()` 要求 importer 调用 `dma_buf_attach()`，即便它们从不进行真正的设备 DMA，而只通过 `dma_buf_vmap()` 进行 CPU 访问。这对不支持 DMA 操作USB 设备来说是个问题
为了修复这个问题，应当从 buffer 共享代码中移除自动页映射。修复起来稍微复杂一些，因为 import/export 缓存还与 `&drm_gem_object.import_attach` 绑定在一起。与此同时，我们通过在支DMA 的情况下找出 USB 主机控制器设备，来为 USB 设备掩盖这个问题。否则导入仍然可能不必要地失败
联系人：Thomas Zimmermann <tzimmermann@suse.de>，Simona Vetter

难度：Advanced

### 实现新的 DUMB_CREATE2 ioctl


当前`DUMB_CREATE` ioctl 定义得不够好。它不接受像素与 framebuffer 格式，只接受一个语义模糊的颜色模式。假设是线framebuffer，颜色模式给出了所支持像素格式的概念。但用户空间实际上不得不去猜测正确的值。它真正可靠的只XRGB8888 framebuffer。用户空间已经开始通过计算任意格式的缓冲区大小并以 XRGB8888 像素为单位计算其大小，来绕过这些限制
一个可能的解决方案是新ioctl `DUMB_CREATE2`。它应该接受一DRM 格式与一个格式修饰符（format modifier），以消除颜色模式的歧义。由framebuffer 可以是多平面的，ioctl 必须返回每个独立颜色平面的缓冲区大小、pitch GEM handle
第一步，ioctl 可以限定为现`DUMB_CREATE` 的当前功能。然后各个驱动可以扩展以支持多平面格式。Rockchip 可能需要这个，会是一个好的候选者
向用户空间提供关于潜在缓冲区（如果分配的话）大小的信息也可能有帮助。用户空间会提供几何形状与格式；内核会返回最小的分配大小与扫描行 pitch。人们有兴趣从另一个设备分配该内存并提供给 DRM 驱动（例如通过 dma-buf）
另一个被请求的特性是能够按大小（而不指定格式）分配缓冲区。加速器（Accelator）在它们的缓冲区分配中使用了这一点，并且很可能可以泛化
除了内核实现之外，还必须有用户空间对ioctl 的支持。Mesa 中有一些代码也许能够使用这个新调用
联系人：Thomas Zimmermann <tzimmermann@suse.de>

难度：Advanced

## 更好的测

### 使用内核单元测试（KUnit）框架添加单元测

`KUnit <https://www.kernel.org/doc/html/latest/dev-tools/kunit/index.html>`_ Linux 内核中的单元测试提供了一个通用框架。拥有一个测试套件可以更早地发现回归
第一批单元测试的一个好候选者是 `drm_format_helper.c` 中的格式转换 helper
联系人：Javier Martinez Canillas <javierm@redhat.com>

难度：Intermediate

### 清理并文档化以前selftests 套件


一KUnit 测试套件（drm_buddy、drm_cmdline_parser、drm_damage_helper、drm_format、drm_framebuffer、drm_dp_mst_helper、drm_mm、drm_plane_helper drm_rect）是以前selftests 套件，在 KUnit 最初引入时被转换了过来
这些套件当时几乎没有文档，而且目标与单元测试所能做的有所不同。尝试识别这些套件中每个测试实际测试的是什么，对于单元测试是否有意义，如果不合理则移除它，合理则为其编写文档，将会大有帮助
联系人：Maxime Ripard <mripard@kernel.org>

难度：Intermediate

### DRM 启用 trinity


并修复由此产生的问题。应该会真的很有趣…
难度：Advanced

### i-g-t 中的 KMS 测试成为通用测试


i915 驱动团队维护着一套广泛的 i915 DRM 驱动测试套件，其中包含大量针modesetting API 中边界情况的测试用例。如果那些测试（至少是那些不依赖 Intel 特定 GEM 特性的测试）能够运行在任何 KMS 驱动上，那就太棒了
在非 i915 上运i-g-t 测试的基础工作已经完成，现在缺的是将它们大规模转换过来。对modeset 测试，我们首先还需要一点基础设施来使dumb buffer 作为 untiled buffer，以便能够运行所有非 i915 特定modeset 测试
难度：Advanced

### 扩展虚拟测试驱动（VKMS

参见 VKMS <vkms> 的文档了解更多细节。这是一个理想的实习任务，因为它只需要一台虚拟机，并且可以根据可用时间调整规模
难度：See details

### Backlight 重构


Backlight 驱动有三enable/disable 状态，这有点过度了。修复计划：

1. 在所有地方推`backlight_enable()` `backlight_disable()` helper。这已经开始了2. 总体上，只看上述 helper 设置的三个状态位中的一个3. 移除另外两个状态位
联系人：Simona Vetter

难度：Intermediate

## 驱动特定


### AMD DC 显示驱动


AMD DC AMD 设备（从 Vega 开始）的显示驱动。在清理它方面已经取得了一些进展，但仍有大量工作要做
参见 drivers/gpu/drm/amd/display/TODO 中的任务
联系人：Harry Wentland，Alex Deucher

## 启动画面（Bootsplash

现在已经有对编写内部 DRM 客户端的支持，这使得可以拾起那个因为fbdev 编写而被拒绝的启动画面工作
- [v6,8/8] drm/client: Hack: Add bootsplash example
  https://patchwork.freedesktop.org/patch/306579/

- [RFC PATCH v2 00/13] Kernel based bootsplash
  https://lore.kernel.org/r/20171213194755.3409-1-mstaudt@suse.de

联系人：Sam Ravnborg

难度：Advanced

## 具有多个内部面板的设备上的亮度处

x86/ACPI 设备上，可能存在多个 backlight 固件接口：（ACPI）video、厂商特定的以及其他接口。还KMS 驱动对直原生（PWM）寄存器的编程
为了处理这个问题，用x86/ACPI backlight 驱动调用 `acpi_video_get_backlight_type()`，它使用启发式（quirk）来选择使用哪个 backlight 接口；而不匹配所返回类型backlight 驱动将不会注册自己，从而只有一backlight 设备被注册（在单 GPU 设置下，见下文）
目前这在很大程度上假设一个系统上只会有一个（内部）面板
在有两个面板的系统上，这可能是一个问题，取决`acpi_video_get_backlight_type()` 选择了什么接口：

1. native：这种情况下，KMS 驱动应当知道哪个 backlight 设备属于哪个输出，因此一切应该都能正常工作2. video：这确实支持控制多个 backlight，但需要做些工作来获得 output backlight 设备之间的映射
上面假设两个面板需要相同的 backlight 接口类型。当两个面板需要不同类型控制时，事情会出问题。例如，一个面板需ACPI video backlight 控制，而另一个使用原backlight 控制。目前在这种情况下，根据 `acpi_video_get_backlight_type()` 的返回值，两个所需 backlight 设备中只有一个会被注册
如果这种（理论上的）情况真的出现，那么支持它将需要一些工作。这里一个可能的解决方案是向 `acpi_video_get_backlight_type()` 传入一device connector-name，以便它能处理这种情况
注意，在某种意义上，我们已经有了用户在用户空间看到两个面板的情况，即在带mux 的双 GPU 笔记本设置中。在这些系统上，我们可能看到两个原生 backlight 设备；或者两个原backlight 设备
用户空间已经有代码通过检测相关面板是否处于活动状态（GPU 与面板之间的 mux 指向哪一边）来处理这个问题，然后使用那个 backlight 设备。不过用户空间在这里非常假设只有一个面板。它只在两个 backlight 设备中选择一个，然后只使用那一个
请注意，所有（我所知道的）用户空间代码目前都是硬编码假设单个面板的
在最近的变更之前（不为单个面板（在单 GPU 笔记本上）注册多个（例如 video + native）`/sys/class/backlight` 设备），用户空间会看到多backlight 设备，全部控制同一backlight
为了处理这个问题，用户空间总是选取 `/sys/class/backlight` 下一个偏好的设备，并忽略其他的。因此，要支持多个面板上的亮度控制，用户空间也需要被更新
有计划通过KMS API 添加一个新的“display brightness”属性到 `drm_connector` 对象（用于面板）来允许通过 KMS API 进行亮度控制。这解决`/sys/class/backlight` API 的一些问题，包括无法sysfs backlight 设备映射到特connector。任何为支持多个面板的设备添加亮度控制而做的用户空间变更，都确实应该构建在这个新的 KMS 属性之上
联系人：Hans de Goede

难度：Advanced

## 用于缓冲区损坏的缓冲区年龄或其他损坏累积算法


进行按缓冲区上传的驱动需要一种缓冲区损坏处理（而不是像plane 或按 CRTC 上传的驱动那样的帧损坏），但目前没有支持来获取缓冲区年龄或任何其他损坏累积算法
因此，损helper 在附加到 plane framebuffer 自上page-flip 以来发生变化时，只会回退到完整的 plane 更新。驱动将 `&drm_plane_state.ignore_damage_clips` 设置true，作为给 `drm_atomic_helper_damage_iter_init()` `drm_atomic_helper_damage_iter_next()` helper 的指示，表示应该忽略损坏 clips
这应当被改进，以使按缓冲区上传的驱动上损坏跟踪能正常工作
关于损坏跟踪的更多信息以及学习资料的参考，可以damage_tracking_properties 中找到
联系人：Javier Martinez Canillas <javierm@redhat.com>

难度：Advanced

## drm_syncobj 查询错误


`drm_syncobj` 容器可以被与驱动无关的代码用来发出提交完成的信号
仍然缺少的一个小特性是一个通用DRM IOCTL，用于查询二进制与时间线 `drm_syncobj` 的错误状态
这应该通过实现必要的内核接口并在用户空间栈中添加对该接口的支持来改进
联系人：Christian König

难度：Starter

## DRM GPU 调度器（Scheduler

### drm_sched_resubmit_jobs() 提供一个通用的替代

`drm_sched_resubmit_jobs()` 已被弃用。主要原因是它会导致重新初始`dma_fence`。详见该函数的文档。对amdgpu Xe 有效的重新提交，更好的方法（显然是）是弄清楚哪个 job（以及通过关联：哪entity）导致了挂起。然后，job 的缓冲区数据，连同当前在同一个硬ring 上的所有其job 的缓冲区数据，必须被置为无效。例如可以通过覆盖它来实现。amdgpu 目前通过保留 job 的副本来确定哪些 job ring 中需要被覆盖。Xe 通过直接访问 `drm_sched` `pending_list` 来获取该信息
任务
1. 实现调度器功能，使驱动能够获取当前在硬件 ring 中的哪些**损坏* job 的信息2. 这样的基础设施随后通常会被用在 `drm_sched_backend_ops.timedout_job()` 中。对此加以文档说明3. 移植一个驱动作为第一个使用者4. 在已弃用`drm_sched_resubmit_jobs()` 的文档中记录这个新的替代方案
联系人：Christian König <christian.koenig@amd.com>
         Philipp Stanner <phasta@kernel.org>

难度：Advanced

### runqueue 添加加锁


`include/drm/gpu_scheduler.h` 中有一Sima 留下的旧 `FIXME`。它详细说明 `struct drm_sched_rq` 在许多地方被读取而没有任何锁，甚至没`READ_ONCE`。在 XDC 2025 上，没有人能真正说清为什么是这种情况、是否需要锁以及是否可以添加锁。（但说真的，那大概应该加锁！）检查是否可能在所有地方添加锁，如果可以就这样做
联系人：Philipp Stanner <phasta@kernel.org>

难度：Intermediate

## DRM 之外


### fbdev 驱动转换DRM


有大量用于较旧硬件的 fbdev 驱动。有些硬件已经过时，但有些仍然提供（足够好的）framebuffer。仍然有用的驱动应当被转换为 DRM，然后在 fbdev 中移除
非常简单的 fbdev 驱动最好从创建一个新DRM 驱动开始转换。Simple KMS helper SHMEM 应当能够处理任何现有硬件。新驱动的回调函数由现有 fbdev 代码填充
更复杂的 fbdev 驱动可以DRM fbconv helper [^4^]_ 的帮助下逐步重构为一DRM 驱动。这helper 提供DRM 核心基础设施fbdev 驱动接口之间的过渡层。在 fbconv helper 之上创建一个新DRM 驱动，复fbdev 驱动，并将其挂接DRM 代码中。Thomas Zimmermann fbconv [^4^]_ 中提供了几个 fbdev 驱动的例子，以及一个该过程的教[^5^]_。结果是一个可以运X11 Weston 的原DRM 驱动
 .. [^4^] https://gitlab.freedesktop.org/tzimmermann/linux/tree/fbconv
 .. [^5^] https://gitlab.freedesktop.org/tzimmermann/linux/blob/fbconv/drivers/gpu/drm/drm_fbconv_helper.c

联系人：Thomas Zimmermann <tzimmermann@suse.de>

难度：Advanced
