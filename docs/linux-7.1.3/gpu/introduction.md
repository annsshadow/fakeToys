## 简

Linux DRM 层包含旨在支持复杂图形设备需求的代码，这类设备通常包含非常适合 3D 图形加速的可编程流水线内核中的图形驱动可以使用 DRM 函数来简化内存管理、中断处理和 DMA 等任务，并向应用程序提供统一的接口
关于版本的说明：本指南涵DRM 树中的特性，包括 TTM 内存管理器、输出配置与模式设置，以及新vblank 内部机制此外还包括当前内核中所有的常规特性
[在此插入典型 DRM 协议栈图]

## 风格指南


为保持一致性，本文档使用美式英语。缩写全部以大写字母书写，例如：DRM、KMS、IOCTL、CRTC 等为便于阅读，文档充分利用 kerneldoc 提供的标记字符：@parameter 表示函数参数，@member 表示结构体成员（同一结构体内），
&struct 表示引用结构体，function() 表示函数。如果被引用对象kerneldoc 存在，这些都会自动生成超链接当引用函数虚表（以及一般的结构体成员）中的条目时，请使&vtable_name.vfunc。遗憾的是，这目前尚不能生成指向该成员的直接链接，只能指向结构体
除特殊情况外（用于区分加锁与不加锁的变体），函数的加锁要求在 kerneldoc 中并不记载相反，加锁应在运行时通过例如 `WARN_ON(!mutex_is_locked(...));` 进行检查。由于文档比运行时告警更容易被忽略，
这样做更有价值。而且运行时检查在加锁规则改变时确实需要更新，从而提高了其正确性。在文档中，
加锁规则应在相关结构体中说明：既可以在锁的注释中解释其保护什么，也可以为数据字段添加关于哪个锁保护它的说明，或两者兼有
具有`void` 返回值的函数应有一个名为“Returns”的小节，说明不同情况下的预期返回值及其含义目前对于该小节名称是否应全部大写、是否应以冒号结尾尚无统一定论。请遵循文件本地的风格其他常见的小节名称包括“Notes”（危险或棘手边界情况的信息）以及“FIXME”（接口可清理之处）
另请阅读面向内核整体的文档指<doc_guide>
### kAPI 的文档要

所有导出给其他模块kernel API 都必须编写文档，包括其数据结构，以及至少一个简短的引言小节来解释整体概念文档应尽可能放在代码本身中，采用 kerneldoc 注释的形式
不要盲目地为所有内容编写文档，而只记录对驱动作者相关的内容：drm.ko 的内部函数以及确定是静态的函数不应具有正式kerneldoc 注释如果认为需要注释，请使用普通的 C 注释。你可以在注释中使用 kerneldoc 语法，但它不应以 /** kerneldoc 标记开头数据结构类似，请按照文档指南`/** private: **/` 注释标注完全私有的内容
## 入门


欢迎有意参与 DRM 子系统开发的开发者。人们经常会针对 checkpatch sparse 报告的各种问题提交补丁。我们欢迎此类贡献
想要更进一步的人可以在 TODO 列表 <todo> 上找到一份清理任务清单
## 贡献流程


DRM 子系统大体上与其他内核子系统工作方式相同，参:ref:`主流程指南与文档 <process_index>` 了解运作方式此处我们仅记GPU 子系统的一些特殊之处
### 特性合并截止时

所有特性工作必须在当前发布周期-rc6 版本前进linux-next 树，否则必须推迟，无法进入下一个合并窗口所有补丁最迟必须在 -rc7 前进drm-next 树，但如果你的分支不linux-next 中，则这必须-rc6 前已经发生
此后只允许缺陷修复（如同上游合并窗口-rc1 发布而关闭之后那样）。不允许新增平台支持或新的驱动
这意味着存在一个约一个月的特性工作无法合并的封禁期。推荐的应对方式是维护一个始终开放的 -next 树，
但确保在封禁期内不把它喂linux-next。例drm-misc 就是这样工作的
### 行为准则


作为 freedesktop.org 项目，dri-devel 以及 DRM 社区遵循贡献者公约（Contributor Covenant），地址为：
https://www.freedesktop.org/wiki/CodeOfConduct

在邮件列表、IRC 或缺陷跟踪器上与社区成员交流时，请保持尊重与文明的举止。社区代表着整个项目项目不容abusive 或欺凌行为
## 可用作示例的简DRM 驱动


DRM 子系统包含大量辅助函数，以简化为简单图形设备编写驱动的工作。例如，`drivers/gpu/drm/tiny/` 目录中有一足够简单、可以用单个源文件实现的驱动。tiny DRM 驱动是理DRM 驱动应是什么样子的好例子。由于只有几百行代码，它们相当易读
## 外部参

首次深入一Linux 内核子系统可能是一种令人不知所措的体验，需要熟悉所有概念并了解该子系统的内部机制等诸多细节
为了平缓学习曲线，本节列出一份可用于学习 DRM/KMS 以及图形一般知识的演讲和文档清单
人们想了DRM 的原因各不相同：移植现有fbdev 驱动、为新硬件编DRM 驱动、修复在处理图形用户空间协议栈时可能遇到的缺陷等因此，学习材料涵盖了 Linux 图形协议栈的许多方面，从内核与用户空间协议栈的概览到非常具体的主题
清单按时间倒序排列，以使最新的材料位于顶部。但它们都包含有用的信息，浏览较旧的材料有助于理DRM 子系统所做变更的缘由和背景
### 会议演讲


- `An Overview of the Linux and Userspace Graphics Stack <https://www.youtube.com/watch?v=wjAJmqwg47k>`_ - Paul Kocialkowski (2020)
- `Getting pixels on screen on Linux: introduction to Kernel Mode Setting <https://www.youtube.com/watch?v=haes4_Xnc5Q>`_ - Simon Ser (2020)
- `Everything Great about Upstream Graphics <https://www.youtube.com/watch?v=kVzHOgt6WGE>`_ - Simona Vetter (2019)
- `An introduction to the Linux DRM subsystem <https://www.youtube.com/watch?v=LbDOCJcDRoo>`_ - Maxime Ripard (2017)
- `Embrace the Atomic (Display) Age <https://www.youtube.com/watch?v=LjiB_JeDn2M>`_ - Simona Vetter (2016)
- `Anatomy of an Atomic KMS Driver <https://www.youtube.com/watch?v=lihqR9sENpc>`_ - Laurent Pinchart (2015)
- `Atomic Modesetting for Drivers <https://www.youtube.com/watch?v=kl9suFgbTc8>`_ - Simona Vetter (2015)
- `Anatomy of an Embedded KMS Driver <https://www.youtube.com/watch?v=Ja8fM7rTae4>`_ - Laurent Pinchart (2013)

### 幻灯片与文章


- `The Linux graphics stack in a nutshell, part 1 <https://lwn.net/Articles/955376/>`_ - Thomas Zimmermann (2023)
- `The Linux graphics stack in a nutshell, part 2 <https://lwn.net/Articles/955708/>`_ - Thomas Zimmermann (2023)
- `Understanding the Linux Graphics Stack <https://bootlin.com/doc/training/graphics/graphics-slides.pdf>`_ - Bootlin (2022)
- `DRM KMS overview <https://wiki.st.com/stm32mpu/wiki/DRM_KMS_overview>`_ - STMicroelectronics (2021)
- `Linux graphic stack <https://studiopixl.com/2017-05-13/linux-graphic-stack-an-overview>`_ - Nathan Gau毛r (2017)
- `Atomic mode setting design overview, part 1 <https://lwn.net/Articles/653071/>`_ - Simona Vetter (2015)
- `Atomic mode setting design overview, part 2 <https://lwn.net/Articles/653466/>`_ - Simona Vetter (2015)
- `The DRM/KMS subsystem from a newbie鈥檚 point of view <https://bootlin.com/pub/conferences/2014/elce/brezillon-drm-kms/brezillon-drm-kms.pdf>`_ - Boris Brezillon (2014)
- `A brief introduction to the Linux graphics stack <https://blogs.igalia.com/itoral/2014/07/29/a-brief-introduction-to-the-linux-graphics-stack/>`_ - Iago Toral (2014)
- `The Linux Graphics Stack <https://blog.mecheye.net/2012/06/the-linux-graphics-stack/>`_ - Jasper St. Pierre (2012)
