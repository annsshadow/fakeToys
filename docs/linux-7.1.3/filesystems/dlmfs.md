
## DLMFS


一个通过虚拟文件系统实现的极简 DLM 用户空间接口
dlmfs OCFS2 一同构建，因为它需OCFS2 的大部分基础设施
:Project web page:    http://ocfs2.wiki.kernel.org
:Tools web page:      https://github.com/markfasheh/ocfs2-tools
:OCFS2 mailing lists: https://subspace.kernel.org/lists.linux.dev.html

除另有说明外，所有代码版权归 2005 Oracle 所有
## 致谢


部分代码取自 ramfs，其版权|copy| 2000 Linus Torvalds Transmeta Corp 所有
Mark Fasheh <mark.fasheh@oracle.com>

## 注意事项


- 目前它仅适用OCFS2 DLM，尽管支持其DLM 实现应该不是大问题
## 挂载选项


鏃。
## 用法


如果你只是对 OCFS2 感兴趣，请参ocfs2.rst。本文档的其余部分将面向那些想使dlmfs 以便在用户空轻松搭建、轻松使用集群锁的人
## 设置


dlmfs 要求 OCFS2 集群基础设施已经就位。请从上面的 URL 下载 ocfs2-tools 并配置一个集群
你需要在一个你锁空间中的所有节点都能访问的卷上启动心跳。最简单的方法是通过 ocfs2_hb_ctl（随 ocfs2-tools
一起分发）。目前它要求有一OCFS2 文件系统就位，以便它能自动找到自己的心跳区域，尽管最终将支持针对
裸盘的心跳
请参见随 ocfs2-tools 一起分发的 ocfs2_hb_ctl mkfs.ocfs2 手册页
一旦你启动了心跳，就可以轻松创销DLM 锁“域”（domain），并访问其中的锁
## 加锁


用户可以通过标准文件系统调用访问 dlmfs，也可以使用 'libo2dlm'（随 ocfs2-tools 分发），它抽象了文件系统
调用并提供一个更传统的加API
dlmfs 为用户自动处理锁缓存，因此对已获取锁的锁请求不会再产生另一DLM 调用。用户空间程序被假定自行处理
它们自己的本地加锁
支持两级锁——共享读（Shared Read）和独占（Exclusive）。同时也支持 Trylock（尝试加锁）操作
关于 libo2dlm 接口的信息，请参见随 ocfs2-tools 分发o2dlm.h
锁值块（LVB）可以通过针对你的 open(2) 调用所获得fd 执行 read(2) write(2) 来读写资源。目前支持的
最LVB 长度64 字节（尽管这OCFS2 DLM 的限制）。通过此机制，dlmfs 的用户可以在它们的节点之共享少量数据
mkdir(2) 通知 dlmfs 加入一个域（该域将与生成的目录同名
rmdir(2) 通知 dlmfs 离开该域

给定域的锁由域目录内的常inode 表示。针对它们加锁是通过 open(2) 系统调用完成的
open(2) 调用在你的锁被授予或发生错误之前不会返回，除非它被指示执trylock 操作。如果加锁成功，你将获得一fd
open(2) 带上 O_CREAT 以确保资inode 被创建——dlmfs 不会为已存在的锁资源自动创建 inode
============  ===========================
Open Flag     锁请求类============  ===========================
O_RDONLY      共享O_RDWR        独占
============  ===========================

============  ===========================
Open Flag     结果加锁行为
============  ===========================
O_NONBLOCK    Trylock 操作
============  ===========================

你必须恰好提O_RDONLY O_RDWR 之一
如果同时提供O_NONBLOCK，且 trylock 操作有效但无法锁住资源，open(2) 将返ETXTBUSY
close(2) 释放与你fd 关联的锁
传递给 mkdir(2) open(2) 的模式在本地被遵守。Chown 在本地也受支持。这意味着你可以使用它们来仅在本地节点上通过 dlmfs 限制对资源的访问
资源 LVB 可以通过 read(2) 系统调用以共享读或独占模式从 fd 读取。它只能通过 write(2) 在独占模式打开写入
一旦写入，LVB 对于在资源上获得只读或更高级别锁的其他节点可见
## 另见


http://opendlm.sourceforge.net/cvsmirror/opendlm/docs/dlmbook_final.pdf

关于 VMS 分布式加API 的更多信息