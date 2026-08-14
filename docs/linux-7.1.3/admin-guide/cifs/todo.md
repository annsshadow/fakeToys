## 待办事项


截至 6.7 内核。关于按版本新增的特性列表，参见 https://wiki.samba.org/index.php/LinuxCIFSKernel

## 缺失特性（部分列表）


欢迎贡献。这个模块有大量可见且重要的贡献机会。以下是一份已知的缺陷与缺失特性的
部分列表：

a) SMB3（以及 SMB3.1.1）缺失的可选特性：
   多通道性能优化、算法化的通道选择、目录租约优化、
   对更快的数据包签名（GMAC）的支持、
   对通过网络进行压缩的支持、
   T10 副本卸载即 "ODX"（目前仅支持 copy chunk 和 "Duplicate Extents" ioctl
   两种服务端复制机制）

b) 针对稀疏文件支持更好地优化复合（compounding）与错误处理，或许新增可选的
   SMB3.1.1 fsctl，使 collapse range 和 insert range 更具原子性

c) 支持通过 QUIC 的 SMB3.1.1（以及或许其它基于套接字的协议，如 SCTP）

d) 配额支持（需要较小的内核改动，否则配额调用无法到达网络文件系统或无设备文件系统）

e) 更多用例可以优化以使用"复合"来减少往返服务器的次数并提升性能，例如
   open/query/close 和 open/setinfo/close。各种情况（stat、statfs、create、
   unlink、mkdir、xattrs）已经通过使用复合得到了改进，但仍有更多可做的。此外，我们
   可以通过使用延迟关闭（配合句柄缓存租约）以及更好地使用文件句柄上的引用计数器，
   来显著减少冗余的打开操作。

f) 完成 inotify 支持，使 kde 和 gnome 的文件列表窗口能够自动刷新（由 Asser 部分完成）。
   需要较小的内核 vfs 改动以支持在文件上移除 D_NOTIFY。

g) 添加 GUI 工具来配置 /proc/fs/cifs 设置并显示 CIFS 统计信息（已开始）

h) 实现对安全类和受信任类的 xattrs 的支持（需要较小的协议扩展），以更好地支持 SELINUX

i) 添加对树连接上下文（tree connect contexts，见 MS-SMB2）的支持，这是一个新的
   SMB3.1.1 协议特性（可能对虚拟化特别有用）

j) 创建 UID 映射设施，以便将服务器 UID 按每次挂载或每台服务器映射到客户端 UID，
   若没有映射则映射为 nobody。同时更好地与 winbind 集成以解析 SID 所有者

k) 添加工具以利用更多 smb3 特定的 ioctl 和特性（passthrough ioctl/fsctl 现已在
   cifs.ko 中实现，允许直接从用户空间发送各种 SMB3 fsctl 以及 query info 和
   set info 调用）。添加工具，使从工具设置各种非 POSIX 元数据属性更加容易
   （例如扩展 smb-info 工具中所做的）

l) 加密文件支持（目前会报告服务器上文件已加密的属性，但不支持更改该属性）

m) 改进统计收集工具（或许与 nfsometer 集成？），以扩展并简化当前 /proc/fs/cifs/Stats
   中的内容的使用

n) 添加对基于声明的 ACL（"DAC"）的支持

o) 挂载辅助 GUI（以简化挂载时的各种配置选项）

p) 扩展对 witness 协议的支持，以允许通知共享移动以及服务器网络适配器变更。目前 Linux
   客户端仅支持 witness 协议对服务器移动的的通知。

q) 允许 mount.cifs 在报告方言或不受支持的特性错误时更加详细。由于新挂载 API 的实现，
   这现在会更加容易。

r) 更新 cifs 文档和用户指南。

s) 解决通过运行标准文件系统 xfstest 套件中更广泛的 xfstests 所发现的缺陷。

t) 将 cifs 和 smb3 支持拆分为独立模块，以便在不需它们的环境中禁用传统的（且不太安全的）
   CIFS 方言并简化代码。

v) 对 SMB3.1.1 的 POSIX 扩展进行额外测试

w) 支持 Mac 的 SMB3.1.1 扩展，以改善与 Apple 服务器的互操作性

x) 支持额外的认证选项（例如 IAKERB、点对点 Kerberos、SCRAM 以及现有服务器支持的
   其它选项）

y) 改进追踪、更多 eBPF 跟踪点、更好的性能分析脚本

## 已知缺陷


参见 https://bugzilla.samba.org - 在产品中搜索 "CifsVFS" 获取当前缺陷列表。同时检查
http://bugzilla.kernel.org（产品 = File System，组件 = CIFS）以及 xfstest 结果，例如
https://wiki.samba.org/index.php/Xfstest-results-smb3

## 杂项待测试


1) 对照各类服务器类型检查最大路径名和最大路径名分量。尝试嵌套符号链接（8 层深）。
   在 stat -f 信息中返回最大路径名

2) 改进 xfstest 的 cifs/smb3 启用，并视需要在 xfstests 中做调整以更好地测试
   cifs/smb3

3) 使用 iozone 及类似工具进行更多性能测试与优化——有一些简单的改动可以并行化
   顺序写，并且在禁用签名时请求更大的读大小（大于协商大小）并向现代服务器发送
   更大的写大小。

4) 对较少见的服务器进行更详尽的测试

5) 继续扩展 smb3 "buildbot"，它目前针对 Windows、Samba 和 Azure 进行自动化
   xfstesting——以添加更多测试并让 buildbot 更快地执行测试。buildbot 的 URL 为：
   http://smb3-test-rhel-75.southcentralus.cloudapp.azure.com

6) 处理各种 coverity 警告（大多数本身不是缺陷，但处理的警告越多，未来就越容易发现
   静态分析器会指出的真正问题）。
