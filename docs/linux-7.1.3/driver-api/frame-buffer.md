## 帧缓冲库


帧缓冲驱动严重依赖四个数据结构。这些结构在 include/linux/fb.h 中声明。它fb_info、fb_var_screeninfo、fb_fix_screeninfo fb_monospecs。最后三可在用户空间与内核之间共享
fb_info 定义了特定显卡的当前状态。在 fb_info 内部存在一fb_ops 结构，它fbdev fbcon 工作所需的函数集合。fb_info 仅对内核可见
fb_var_screeninfo 用于描述显卡中用户定义的特征。通过 fb_var_screeninfo，可定义深度和分辨率等
下一个结构是 fb_fix_screeninfo。它定义了设置模式时创建且无法以其他方式更改卡属性。一个很好的例子是帧缓冲内存的起始位置。这“锁定”了帧缓冲内存的地址使其无法被更改或移动
最后一个结构是 fb_monospecs。在API 中，fb_monospecs 几乎不重要。这允许一被禁止的事情，例如在定频监视器上设置 800x600 模式。在API 中，fb_monospecs
阻止此类事情，如果使用正确，可以防止监视器被烧毁。fb_monospecs 2.5.x 内核
之前没有用处
### 甯х紦鍐插唴瀛。

   :export:

### 帧缓冲颜色映

   :export:

### 帧缓冲视频模式数据库


   :internal:

   :export:

### 帧缓Macintosh 视频模式数据

   :export:

### 帧缓冲字

   Refer to the file lib/fonts/fonts.c for more information.
