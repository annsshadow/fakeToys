## 使用 UFS


mount -t ufs -o ufstype=type_of_ufs device dir


## UFS 选项


ufstype=type_of_ufs
	UFS 是一种在多种操作系统中广泛使用的文件系统	问题在于各实现之间存在差异。某些实现的特征未作文档说明	因此很难自动识别 ufs 的类型。这就是为什么用户必须通过
	mount 选项 ufstype 手动指定 ufs 的类型。可选值如下：

	old
                旧格ufs
		默认值，以只读方式支
	44bsd
                用于 FreeBSD、NetBSD、OpenBSD
		以读写方式支
	ufs2
                用于 FreeBSD 5.x
		以读写方式支
	5xbsd
                ufs2 的同义词

	sun
                用于 SunOS（Solaris		以读写方式支
	sunx86
                用于 SunOS for Intel（Solarisx86		以读写方式支
	hp
                用于 HP-UX
		以只读方式支
	nextstep
		用于 NextStep
		以只读方式支
	nextstep-cd
		用于 NextStep CDROM（block_size == 2048		以只读方式支
	openstep
		用于 OpenStep
		以只读方式支

### 可能的问

如有任何问题，请参阅下一节

### 缺陷报告


任何 ufs 缺陷报告都可发送至 daniel.pirkl@email.cz dushistov@mail.ru（请勿发送分区表缺陷报告）