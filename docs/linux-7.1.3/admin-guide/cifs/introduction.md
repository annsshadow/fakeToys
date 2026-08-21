## 简

  这是面向 SMB3 NAS 协议的客户端 VFS 模块，同时也面向较早的方言，例如通用互联网文件系  （CIFS）协议——它是服务器消息块（SMB）协议的继任者，SMB 是大多数早期 PC 操作系统  原生文件共享机制。CIFS 的新版本和改进版本现在称SMB2 SMB3。出于安全原因，强烈建议
  使用 SMB3（以及更晚的版本，包括最新方言 SMB3.1.1）而非使用 CIFS 等较早方言。所有现代方言  包括最新的 SMB3.1.1，都CIFS VFS 模块支持。SMB3 协议由所有主流文件服务器实现和支持，
  例如 Windows（包Windows 2019 Server），以及 Samba（它Linux 和许多其他操作系统提供了
  出色CIFS/SMB2/SMB3 服务器支持与工具）。Apple 系统也对 SMB3 支持良好，大多数网络附加存储
  （NAS）厂商也是如此，因此该网络文件系统客户端可以挂载到各种各样的系统。它还支持挂载到  （例Microsoft Azure），包括必要的安全特性
  该模块的意图是为兼容 SMB3 的服务器提供最先进的网络文件系统功能，包括高级安全特性  出色的并行化高性能 I/O、更好的 POSIX 兼容性、安全的每用户会话建立、加密、高性能安全分布  缓存（leases/oplocks）、可选的数据包签名、大文件、Unicode 支持以及其他国际化改进。由Samba
  服务器与该文件系统客户端都支CIFS Unix 扩展，且 Linux 客户端也支持 SMB3 POSIX 扩展，这  组合可以在某Linux Linux 环境中提供其他网络和集群文件系统的合理替代方案，而不仅仅是在
  Linux Windows（或 Linux Mac）环境中
  该文件系统带有挂载工具（mount.cifs）和各种用户空间工具（包smbinfo setcifsacl），
  可从以下位置获取
      https://git.samba.org/?p=cifs-utils.git

  鎴。
      git://git.samba.org/cifs-utils.git

  mount.cifs 应当安装到与其他挂载辅助程序相同的目录中
  有关该模块的更多信息，请参见项目 wiki 页面
      https://wiki.samba.org/index.php/LinuxCIFS

  以及

      https://wiki.samba.org/index.php/LinuxCIFS_utils
