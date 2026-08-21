## 配额子系

配额子系统允许系统管理员为用户和/或组设置已用空间与已inode 数量（inode
是与每个文件或目录相关联的文件系统结构）的限制。对于已用空间和已用 inode
数量，实际上各有两个限制。第一个称为软限制（softlimit），第二个称为硬限制
（hardlimit）。用户永远不能超过任何资源的硬限制（除非其拥CAP_SYS_RESOURCE
能力）。允许用户在有限时间内超过软限制。该期限称为“宽限期”（grace
period）或“宽限时间”（grace time）。宽限时间结束后，用户将无法分配更多
空间/inode，直到释放足够多的资源使其低于软限制
配额限制（以及宽限时间的长短）针对每个文件系统进行独立设置
有关配额设计的更多细节，请参quota-tools 软件包中的文(https://sourceforge.net/projects/linuxquota)
## 配额 netlink 接口

当用户超过软限制、宽限时间耗尽或达到硬限制时，配额子系统传统上会向导致超限进程所在的控制终端打印一条消息。这种方法的缺点是：当用户使用图形桌面时，通常
无法看到该消息。因此设计了配额 netlink 接口，用于将上述事件的信息传递给用户态在用户态，这些信息可由应用程序捕获并相应处理
该接口使用通用 netlink 框架（有关该层的更多细节，请参见
https://lwn.net/Articles/208755/ http://www.infradead.org/~tgr/libnl/）配额通用 netlink 接口的名称为 "VFS_DQUOT"。下方常量的定义位于 <linux/quota.h>由于配额 netlink 协议不感知命名空间，配额 netlink 消息仅在初始网络命名空间中发送
目前，该接口仅支持一种消息类QUOTA_NL_C_WARNING。该命令用于发送关于上述任一
事件的通知。每条消息有六个属性。这些属性如下（参数类型在括号内）：

        QUOTA_NL_A_QTYPE (u32)
   - 被超过的配额类型（USRQUOTA、GRPQUOTA 之一        QUOTA_NL_A_EXCESS_ID (u64)
   - 超过限制的用户的 UID/GID（取决于配额类型        QUOTA_NL_A_CAUSED_ID (u64)
   - 导致该事件的用户UID
        QUOTA_NL_A_WARNING (u32)
   - 哪种限制被超过：

		QUOTA_NL_IHARDWARN
		    inode 硬限		QUOTA_NL_ISOFTLONGWARN
		    inode 软限制超过给定宽		    周期的时间更		QUOTA_NL_ISOFTWARN
		    inode 软限		QUOTA_NL_BHARDWARN
		    空间（块）硬限制
		QUOTA_NL_BSOFTLONGWARN
		    空间（块）软限制超过
		    给定宽限周期的时间更长		QUOTA_NL_BSOFTWARN
		    空间（块）软限制

   - 当用户停止超过某一限制时，也为该事件定义了四个警告
		QUOTA_NL_IHARDBELOW
		    inode 硬限		QUOTA_NL_ISOFTBELOW
		    inode 软限		QUOTA_NL_BHARDBELOW
		    空间（块）硬限制
		QUOTA_NL_BSOFTBELOW
		    空间（块）软限制

        QUOTA_NL_A_DEV_MAJOR (u32)
   - 受影响文件系统所在设备的主设备号
        QUOTA_NL_A_DEV_MINOR (u32)
   - 受影响文件系统所在设备的次设备号
