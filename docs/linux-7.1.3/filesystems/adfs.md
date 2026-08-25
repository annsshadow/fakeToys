
## Acorn Disc Filing System - ADFS


### ADFS 支持的文件系

ADFS 模块支持具有以下特征的以Filecore 格式
- 新映射（new maps- 新目录或 big 目录

就命名的格式而言，这意味着我们支持
- E E+，带或不带启动块
- F 涓?F+

我们完全支持从这些文件系统中读取文件，以及在其现有分配范围内写入已有文件本质上，我们不支持更改任何文件系统元数据
这旨在支持在 RISC OS Filecore 文件系统上的回环挂载 Linux 原生文件系统，但允许
更改文件中的数据
如果配置了写支持（ADFS_FS_RW），我们允许基本的目录更新，具体为更新访问模式与
时间戳
### ADFS 的挂载选项


  ============  ======================================================
  uid=nnn	分区中的所有文件都将归属于用户
		id nnn。默认为 0（root）  gid=nnn	分区中的所有文件都将位于组
		nnn 中。默认为 0（root）  ownmask=nnn	ADFS 'owner' 权限的权限掩		将为 nnn。默认为 0700  othmask=nnn	ADFS 'other' 权限的权限掩		将为 nnn。默认为 0077  ftsuffix=n	ftsuffix=0 时，不附加任何文件类型后缀		ftsuffix=1 时，会添加与 RISC OS 文件类型
		对应的十六进制后缀。默认为 0  ============  ======================================================

### ADFS 权限Linux 权限的映

  ADFS 权限包含以下各项
 - 属主 - 属主 - 其他 - 其他
  （在较早版本中，曾经存在一'execute' 权限，但它的含义Linux 'execute'
  权限不同，现已废弃）
```

	Owner read				-> -r--r--r--
	Owner write				-> --w--w---w
	Owner read and filetype UnixExec	-> ---x--x--x
    These are then masked by ownmask, eg 700	-> -rwx------
	Possible owner mode permissions		-> -rwx------

	Other read				-> -r--r--r--
	Other write				-> --w--w--w-
	Other read and filetype UnixExec	-> ---x--x--x
    These are then masked by othmask, eg 077	-> ----rwxrwx
	Possible other mode permissions		-> ----rwxrwx

  因此，在默认掩码下，如果一个文件是属主写，且不UnixExec 文件类型  则其权限将为::

			-rw-------

  然而，如果掩码ownmask=0770,othmask=0007，则会被修改:

			-rw-rw----

  对这些掩码的使用没有任何限制。你可能希望任一种读位都向所有人授予文件的读访问  但保留默认的写保护（ownmask=0755,othmask=0577:

			-rw-r--r--

  因此，你可以根据需要，将权限转换裁剪成Linux 下期望的任何权限
```
### RISC OS 文件类型后缀


  RISC OS 文件类型存储在文件加载地址的第 19..8 位中
  为了使非 RISC OS 系统能够用于存储文件而不丢失文件类型信息，人们设计了一种文  命名约定（最初用NFS），即以 ,xyz 形式的十六进制后缀表示文件类型：例  BasicFile,ffb 是一BASICxffb）文件。这种命名约定现在也RPCEmu   RISC OS 模拟器使用
  ftsuffix=1 选项挂载 ADFS 光盘会导致从目录读取的文件名被追加相应的文件类型
  后缀。如ftsuffix 选项为零或省略，则不会添加任何文件类型后缀