## VFAT


## 使用 VFAT


```
  mount -t vfat /dev/fd0 /mnt
```
不需要特殊的分区格式化工具，如果你想在 Linux 内部格式化，`mkdosfs` 就可以很好地工作。

## VFAT 挂载选项


**uid=###**
	设置此文件系统中所有文件的拥有者。
	默认值是当前进程的 uid。

**gid=###**
	设置此文件系统中所有文件的组。
	默认值是当前进程的 gid。

**umask=###**
	权限掩码（针对文件与目录，参见 **umask(1)**）。
	默认值是当前进程的 umask。

**dmask=###**
	针对目录的权限掩码。
	默认值是当前进程的 umask。

**fmask=###**
	针对文件的权限掩码。
	默认值是当前进程的 umask。

**allow_utime=###**
	该选项控制对 mtime/atime 的权限检查。

		**-20**: 如果当前进程属于文件组 ID 所在的组，
                你就可以更改时间戳。

		**-2**: 其他用户可以更改时间戳。

	默认由 dmask 选项设置。如果目录可写，也允许 utime(2)。即 ~dmask & 022。

	通常 utime(2) 会检查当前进程是否为文件拥有者，或者是否具有 CAP_FOWNER 能力。但 FAT 文件系统磁盘上没有 uid/gid，所以常规检查过于僵化。通过该选项可以放宽它。

**codepage=###**
	设置用于在 FAT 文件系统上转换为短文件名字符的 codepage 编号。
	默认使用 FAT_DEFAULT_CODEPAGE 设置。

**iocharset=<name>**
	用于转换用户可见文件名所用编码与 16 位 Unicode 字符之间
	的字符集。长文件名以 Unicode 格式存储在磁盘上，但 Unix 在
	很大程度上不知道如何处理 Unicode。
	默认使用 FAT_DEFAULT_IOCHARSET 设置。

	也有一个使用 utf8 选项进行 UTF-8 转换的选择。

	  改为使用 utf8 选项。

**utf8=<bool>**
	UTF-8 是控制台所使用的、对文件系统安全的 Unicode 版本。
	可以通过该选项为文件系统启用或禁用它。
	如果设置了 'uni_xlate'，UTF-8 会被禁用。
	默认使用 FAT_DEFAULT_UTF8 设置。

**uni_xlate=<bool>**
	将未处理的 Unicode 字符转换为特殊的转义序列。这可以让你
	备份并恢复使用任何 Unicode 字符创建的文件名。在 Linux 真正
	支持 Unicode 之前，这给了你一种替代方案。没有该选项时，在
	无法进行转换时会使用 '?'。转义字符是 ':'，因为该字符在
	vfat 文件系统上是非法的。所使用的转义序列是 ':' 加四个
	十六进制数字表示的 unicode。

**nonumtail=<bool>**
	在创建 8.3 别名时，别名通常以 '~1' 或波浪号后跟某个数字结尾。
	如果设置了该选项，那么当文件名为 "longfilename.txt" 且
	"longfile.txt" 当前在目录中不存在时，short 别名将是 longfile.txt
	而非 longfi~1.txt。

**usefree**
	使用存储在 FSINFO 上的 “free clusters”（空闲簇）值。它将
	用于确定空闲簇的数量而无需扫描磁盘。但默认不使用，因为
	最近的 Windows 在某些情况下不能正确更新它。如果你确定 FSINFO 上的
	“free clusters” 是正确的，通过该选项可以避免扫描磁盘。

**quiet**
	停止打印某些警告消息。

**check=s|r|n**
	大小写敏感检查设置。

	**s**: 严格（strict），大小写敏感

	**r**: 宽松（relaxed），大小写不敏感

	**n**: 普通（normal），默认设置，目前大小写不敏感

**nocase**
	该选项在 vfat 中已弃用。请改用 `shortname=win95`。

**shortname=lower|win95|winnt|mixed**
	短文件名显示/创建设置。

	**lower**: 显示时转换为小写，创建时模拟 Windows 95 规则。

	**win95**: 显示/创建时模拟 Windows 95 规则。

	**winnt**: 显示/创建时模拟 Windows NT 规则。

	**mixed**: 显示时模拟 Windows NT 规则，创建时模拟 Windows 95 规则。

	默认设置为 `mixed`。

**tz=UTC**
	将时间戳解释为 UTC 而非本地时间。
	该选项禁用了本地时间（Windows 在 FAT 上使用的）与 UTC
	（Linux 内部使用的）之间的时间戳转换。这在挂载被设置为 UTC 的
	设备（如数码相机）时特别有用，可以避免本地时间带来的陷阱。

**time_offset=minutes**
	设置从 FAT 使用的本地时间转换到 UTC 的偏移量。即每个时间戳会
	减去 <minutes> 分钟，转换为 Linux 内部使用的 UTC。当 `sys_tz` 中
	设置的时区不是文件系统所使用的时区时这很有用。注意，在存在
	DST（夏令时）的情况下，该选项仍然不能在所有情况下提供正确的
	时间戳——处于不同 DST 设置下的时间戳会偏差一小时。

**showexec**
	如果设置，则只有当名称的扩展部分为 .EXE、.COM 或 .BAT 时，
	文件的执行权限位才被允许。默认不设置。

**debug**
	可以设置，但当前实现中未使用。

**sys_immutable**
	如果设置，FAT 上的 ATTR_SYS 属性会被当作 Linux 上的 IMMUTABLE 标志处理。默认不设置。

**flush**
	如果设置，文件系统会比正常情况更早尝试刷新到磁盘。默认不设置。

**rodir**
	FAT 具有 ATTR_RO（只读）属性。在 Windows 上，目录的 ATTR_RO
	会被忽略，仅被应用程序用作一个标志（例如，它为自定义文件夹而设置）。

	如果你想把 ATTR_RO 作为只读标志用于目录，请设置该选项。

**errors=panic|continue|remount-ro**
	指定 FAT 在遇到严重错误时的行为：panic（恐慌）、continue（不做任何处理继续）
	还是以只读模式重新挂载分区（默认行为）。

**discard**
	如果设置，当块被释放时向块设备发出 discard/TRIM 命令。这对 SSD 设备
	以及稀疏/精简配置的 LUN 很有用。

**nfs=stale_rw|nostale_ro**
	仅当你想要通过 NFS 导出 FAT 文件系统时启用此选项。

		**stale_rw**: 该选项维护一个按 **i_logstart** 索引（缓存）的目录
		**inode**，NFS 相关代码用它来改善查找。支持通过 NFS 的完整文件
		操作（读/写），但由于 NFS 服务器上的缓存驱逐，这可能导致 ESTALE 问题。

		**nostale_ro**: 该选项将 **inode** 号与文件句柄建立在 MS-DOS 目录项
		中文件在磁盘上的位置之上。这确保了文件从 inode 缓存中被驱逐后
		不会返回 ESTALE。然而，这意味着 rename、create 与 unlink 等操作
		可能导致先前指向某个文件的文件句柄指向另一个文件，潜在地造成数据损坏。
		因此，该选项也会以只读方式挂载文件系统。

	为了保持向后兼容，`'-o nfs'` 也被接受，默认为 "stale_rw"。

**dos1xfloppy  <bool>: 0,1,yes,no,true,false**
	如果设置，使用由后备设备大小决定的回退默认 BIOS 参数块
	配置。这些静态参数匹配 DOS 1.x 为 160 kiB、180 kiB、320 kiB
	与 360 kiB 软盘及软盘镜像所假设的默认值。



## 限制


在使用带有 FALLOC_FL_KEEP_SIZE 的 fallocate 时，文件的 fallocated 区域会在
umount/evict（卸载/回收）时被丢弃。因此，用户应当假设在有内存压力导致 inode
从内存中被回收时，fallocated 区域可能在最后一次关闭时被丢弃。因此，对于任何
对 fallocated 区域的依赖，用户应当确保在重新打开文件后重新检查 fallocate。

## TODO


需要去掉原始的扫描代码。改为始终使用获取下一个目录项的方式。目前仍在使用
原始扫描的只剩下目录重命名代码。


## 可能存在的问题


- vfat_valid_longname 没有正确检查保留名。
- 当卷名与文件系统根目录中的某个目录名相同时，该目录名有时显示为
  一个空文件。
- autoconv 选项不能正确工作。


## 测试套件


如果你打算对 vfat 文件系统做任何修改，请获取随 vfat 发行版一起提供的测试套件，地址为

`<http://web.archive.org/web/*/http://bmrc.berkeley.edu/people/chaffee/vfat.html>`_

该套件测试了 vfat 文件系统的相当多部分，欢迎为新的或未经测试的特性提供额外的测试。

## 关于 VFAT 文件系统结构的说明


本文档由 Galen C. Hunt gchunt@cs.rochester.edu 提供，并经 Gordon Chaffee 略作注解。

本文档非常粗略、技术性地概述了我对 Windows NT 3.5 与 Windows 95 中使用的扩展 FAT
文件系统的了解。我不保证以下内容有任何正确性，但看起来确实如此。

扩展 FAT 文件系统几乎与 DOS（含 **6.223410239847** 版本）及更早版本中使用的 FAT
文件系统完全相同 :-)。显著的变化是增加了长文件名。这些名字支持最多 255 个字符，
包括空格与小写字符，而传统的 8.3 短名则不然。

以下是当前传统 FAT 项的描述：
```
        struct directory { // Short 8.3 names
                unsigned char name[8];          // file name
                unsigned char ext[3];           // file extension
                unsigned char attr;             // attribute byte
		unsigned char lcase;		// Case for base and extension
		unsigned char ctime_ms;		// Creation time, milliseconds
		unsigned char ctime[2];		// Creation time
		unsigned char cdate[2];		// Creation date
		unsigned char adate[2];		// Last access date
		unsigned char reserved[2];	// reserved values (ignored)
                unsigned char time[2];          // time stamp
                unsigned char date[2];          // date stamp
                unsigned char start[2];         // starting cluster number
                unsigned char size[4];          // size of the file
        };
```
lcase 字段指定 8.3 名字的基名和/或扩展名是否应大写。该字段似乎不被 Windows 95 使用，但被 Windows NT 使用。文件名的大小写在 Windows NT 到 Windows 95 之间并不完全兼容。反过来方向同样不完全兼容。适合 8.3 命名空间、且在 Windows NT 上以小写写入的文件名，在 Windows 95 上会显示为大写。

          字节序（endian）整数值。该结构中各字段的描述是公开知识，可以在别处找到。

通过扩展 FAT 系统，Microsoft 为任何具有扩展名的文件插入了额外的目录项。
（任何合法地符合旧 8.3 编码方案的名称没有额外项。）我称这些额外项为槽（slot）。
基本上，一个槽是一个特殊格式的目录项，持有文件名扩展名中最多 13 个字符。将槽
视为与其对应的文件目录项的附加标签。Microsoft 倾向于将文件的 8.3 项称为其别名（alias），
将扩展槽目录项称为文件名。
```
        struct slot { // Up to 13 characters of a long name
                unsigned char id;               // sequence number for slot
                unsigned char name0_4[10];      // first 5 characters in name
                unsigned char attr;             // attribute byte
                unsigned char reserved;         // always 0
                unsigned char alias_checksum;   // checksum for 8.3 alias
                unsigned char name5_10[12];     // 6 more characters in name
                unsigned char start[2];         // starting cluster number
                unsigned char name11_12[4];     // last 2 characters in name
        };
```
如果槽的布局看起来有点奇怪，那只是因为 Microsoft 努力保持与旧软件的兼容性。槽必须被伪装以防止旧软件恐慌。为此，采取了一些措施：

        1) 槽目录项的属性字节总是设置为 0x0f。这对应于一个具有 “hidden”（隐藏）、
           “system”（系统）、“read-only”（只读）与 “volume label”（卷标）属性的旧目录项。
           大多数旧软件会忽略任何设置了 “volume label” 位的目录项。真正的卷标项
           不会设置其他三个位。

        2) 起始簇总是设置为 0，这对一个 DOS 文件来说是不可能的取值。

由于扩展 FAT 系统是向后兼容的，旧软件可能修改目录项。必须采取措施确保槽的有效性。扩展 FAT 系统可以通过如下方式验证一个槽确实属于某个 8.3 目录项：

        1) 位置。一个文件的槽总是紧接在其对应的 8.3 目录项之前。此外，每个槽
           都有一个 id，标记其在扩展文件名中的顺序。下面是一个 8.3 目录项及其
           对应长名槽非常简略的视图，针对文件
```
                <proceeding files...>
                <slot #3, id = 0x43, characters = "h is long">
                <slot #2, id = 0x02, characters = "xtension whic">
                <slot #1, id = 0x01, characters = "My Big File.E">
                <directory entry, name = "MYBIGFIL.EXT">
```
           .. note:: 注意槽是从最后到最前存储的。槽从 1 编号到 N。第 N 个槽与
		     0x40 进行 ``or`` 运算以标记为最后一个。

        2) 校验和。每个槽都有一个 alias_checksum 值。校验和使用如下算法从 8.3 名字计算而来::
```
                for (sum = i = 0; i < 11; i++) {
                        sum = (((sum&1)<<7)|((sum&0xfe)>>1)) + name[i]
                }
```
	3) 如果最后一个槽中有空闲空间，在最后一个字符之后存储一个 Unicode ``NULL (0x0000)``。
	   之后，最后一个槽中所有未使用的字符被设置为 Unicode 0xFFFF。

```
最后，注意扩展名是以 Unicode 存储的。每个 Unicode 字符占用两个或四个字节，以 UTF-16LE 编码。
```
