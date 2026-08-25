
## NTFS3

## 概述与功
NTFS3 是一个功能完备的 NTFS 读写驱动。该驱动支持最3.1 版本NTFS。挂载时使用文件系统类型**ntfs3**
- 该驱动实现了对普通文件、稀疏（sparse）文件和压缩文件NTFS 写支持- 支持原生的日志重放（journal replaying）- 支持对已挂载 NTFS 卷的 NFS 导出- 支持扩展属性（extended attributes）。预定义的扩展属性：

 - **system.ntfs_security** 获取/设置安全描述
		Descriptor: SECURITY_DESCRIPTOR_RELATIVE

 - **system.ntfs_attrib** 获取/设置 ntfs 文件/目录属性
	  注意：应用于空文件时，这允许sparse(0x200)、compressed(0x800) 	  normal 之间切换类型
 - **system.ntfs_attrib_be** 获取/设置 ntfs 文件/目录属性
	  system.ntfs_attrib 取值相同，但始终以大端（big-endian）表	  （system.ntfs_attrib 的字节序CPU 相同）
## 挂载选项

下面的列表描述了 NTFS3 驱动除通用挂载选项外所支持的挂载选项。你可以将每个选项**no** 选项一起使用。如果某个选项在本表中标记no，意味着默认是不**no** 的
   :widths: 1 5
   :fill-cells:

   - - iocharset=name
     - 该选项告知驱动如何解释路径字符串，并将其转换为 Unicode 以及反向转换。如果未
       设置该选项，将使用默认代码页（CONFIG_NLS_DEFAULT）
       示例：iocharset=utf8

   - - uid=
     - `1`
   - - gid=

   - - umask=
     - 控制 NTFS 卷挂载后创建的文目录的默认权限
   - - dmask=
     - `1` 与指定同时适用于文件和目录umask 不同，fmask 只应用于文件，dmask
       只应用于目录   - - fmask=

   - - nohidden
     - 带有 Windows 特有HIDDEN（FILE_ATTRIBUTE_HIDDEN）属性的文件将不会在
       Linux 下显示
   - - sys_immutable
     - 带有 Windows 特有SYSTEM（FILE_ATTRIBUTE_SYSTEM）属性的文件将被标记       系统不可变（system immutable）文件
   - - hide_dot_files
     - 在创建、移动或重命名文件时更新 Windows 特有HIDDEN（FILE_ATTRIBUTE_HIDDEN       属性。以句点开头的文件名将被设HIDDEN 属性，不以句点开头的文件名将       清除该属性
   - - windows_names
     - 阻止创建名称不被 Windows 允许的文件和目录，原因包括：包含某些不允许的字符
       （即字符 " * / : < > ? \\ | 以及编码小于 0x20 的字符）；名称（带或不带扩展名）
       是保留文件名（CON、AUX、NUL、PRN、LPT1-9、COM1-9）；或者最后一个字符是空格
       或句点。已有的此类文件仍可被读取和重命名
   - - discard
     - 启用TRIM 命令的支持，以提升删除操作的性能，建议与固态硬盘（SSD）一起使用
   - - force
     - 强制驱动挂载分区，即使卷被标记为脏（dirty）。不建议使用
   - - sparse
     - 以稀疏方式创建新文件
   - - showmeta
     - 使用此参数可在已挂载NTFS 分区上显示所有元文件（System Files）。默认情况下       所有元文件都是隐藏的
   - - prealloc
     - 在写入时文件大小增长的情况下，过度地为文件预分配空间。在并行写入不同文件       可减少碎片化
   - - acl
     - 支持 POSIX ACL（访问控制列表）。在内核支持时生效。不要与 NTFS ACL 混淆。指       acl 的选项启用POSIX ACL 的支持
## 待办列表

- 基于 JBD 的完整日志（journaling）支持。目前支持日志重放，但效果未必能达到 JBD   程度
## 参考资
- NTFS 驱动Linux 商业版本	https://www.paragon-software.com/home/ntfs-linux-professional/

- NTFS3 实现的反馈与需求的直接电子邮件地址	almaz.alexandrovich@paragon-software.com
