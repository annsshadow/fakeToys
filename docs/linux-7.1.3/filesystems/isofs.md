
## ISO9660 文件系统


与 msdos 和 vfat 分区相同的挂载选项。

  =========	========================================================
  gid=nnn	分区中的所有文件都将属于组 nnn。
  uid=nnn	分区中的所有文件都将由用户 id nnn 拥有。
  umask=nnn	分区的权限掩码（参见 umask(1)）。
  =========	========================================================

与 vfat 分区相同的挂载选项。这些选项仅在使用 Microsoft 的 Joliet 扩展
编码的光盘时有用。

 ==============	=============================================================
 iocharset=name 用于从 Unicode 转换为 ASCII 的字符集。Joliet 文件名以
		Unicode 格式存储，但 Unix 在大多数情况下不知道如何处理
		Unicode。也可以使用 utf8 选项进行 UTF-8 转换。
  utf8          以 UTF-8 格式编码 Unicode 文件名。默认为否。
 ==============	=============================================================

isofs 文件系统独有的挂载选项。

 ================= ============================================================
  block=512       将磁盘的块大小设置为 512 字节
  block=1024      将磁盘的块大小设置为 1024 字节
  block=2048      将磁盘的块大小设置为 2048 字节
  check=relaxed    匹配大小写不同的文件名
  check=strict     仅匹配大小写完全相同的文件名
  cruft            尝试处理格式错误的光盘。
  map=off          不将非 Rock Ridge 文件名映射为小写
  map=normal       将非 Rock Ridge 文件名映射为小写
  map=acorn       同 map=normal，但若存在则同时应用 Acorn 扩展
  mode=xxx         将文件的权限设置为 xxx，除非 Rock Ridge 扩展另行设定了权限
  dmode=xxx        将目录的权限设置为 xxx，除非 Rock Ridge 扩展另行设定了权限
  overriderockperm 即使存在 Rock Ridge 扩展，也根据 'mode' 和 'dmode'
		  设置文件和目录的权限。
  nojoliet         若 Joliet 扩展存在则忽略它。
  norock           若 Rock Ridge 扩展存在则忽略它。
  hide		  从文件系统中完全剔除隐藏文件。
  showassoc	  显示标记为 'associated' 位的文件
  unhide	  已弃用；现在默认显示隐藏文件；
		  若指定，它等价于 'showassoc'，会重现此前的 unhide 行为
  session=x        在多区段光盘上选择区段编号
  sbsector=xxx     区段从 xxx 扇区开始
 ================= ============================================================

关于 ISO 9660 标准的推荐文档位于：

- http://www.y-adagio.com/
- https://ecma-international.org/wp-content/uploads/ECMA-119_2nd_edition_december_1987.pdf

引用自该 PDF：“本标准 ECMA-119 第 2 版在技术上与 ISO 9660 完全相同。” 因此它是
官方 ISO 规范的一个有效且免费的替代版本。

