## 映射设备的早期创

可以通过两种方式device-mapper 设备配置为系统的根设备
第一种是构建一个初始内存盘（initramfs），它引导到一个最小用户空间，该用户空间配置好设备，然pivot_root(8) 进入其中
第二种是通过内核启动命令行参数，使用模块参数 "dm-mod.create=" 创建一个或多个 device-mapper
其格式指定为一个由逗号分隔、可选使用分号的数据字符串，其中
 - 逗号用于分隔字段，如 name、uuid、flags table（指定一个设备）
 - 分号用于分隔设备
```

 dm-mod.create=<name>,<uuid>,<minor>,<flags>,<table>[,<table>+][;<name>,<uuid>,<minor>,<flags>,<table>[,<table>+]+]

```
```

	<name>		::= 设备名称	<uuid>		::= xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx | ""
	<minor>		::= 设备次设备号 | ""
	<flags>		::= "ro" | "rw"
	<table>		::= <start_sector> <num_sectors> <target_type> <target_args>
	<target_type>	::= "verity" | "linear" | ...（见下表
```
dm 行应等价dmsetup 工具使用 `--concise` 参数时所用的一行
## 目标类型


并非所有目标类型都可用，因为在未先使用用户空间工具检查相关元数据有效性就激活某DM 目标时，存在
严重风险
======================= =======================================================
`cache`			受限，用户空间应验证缓存设备
`crypt`			允许
`delay`			允许
`era`			受限，用户空间应验证元数据设`flakey`		受限，用于测`linear`		允许
`log-writes`		受限，用户空间应验证元数据设`mirror`		受限，用户空间应验证镜像设备
`raid`			受限，用户空间应验证元数据设`snapshot`		受限，用户空间应验证目标设备
`snapshot-origin`	允许
`snapshot-merge`		受限，用户空间应验证目标设备
`striped`		允许
`switch`		受限，用户空间应验证设备路径
`thin`			受限，需要来自用户空间的 dm target 消息
`thin-pool`		受限，需要来自用户空间的 dm target 消息
`verity`		允许
`writecache`		受限，用户空间应验证缓存设备
`zero`			受限，不用于根文件系======================= =======================================================

如果目标类型未在上面列出，则默认受限（未经测试）
## 示例


一个引导到一个由用户Linux 块设备组成的线性阵列的示例
```

  dm-mod.create="lroot,,,rw, 0 4096 linear 98:16 0, 4096 4096 linear 98:32 0" root=/dev/dm-0

```
这将引导到一个由 8192 个扇区组成的读写 dm-linear 目标，该目标跨越两个通过其主:次设备号标识的块设备启动后，udev 会根据规则将此目标重命名/dev/mapper/lroot。没有分uuid
多个 device-mapper 的示例，dm-mod.create="..." 的内```

  dm-linear,,1,rw,
    0 32768 linear 8:1 0,
    32768 1024000 linear 8:2 0;
  dm-verity,,3,ro,
    0 1638400 verity 1 /dev/sdc1 /dev/sdc2 4096 4096 204800 1 sha256
    ac87db56303c9c1da433d7209b5a6ef3e4779df141200cbd7c157dcb8dd89c42
    5ebfe87f7df3235b80a117ebc4078e44f55045487ad4a96581d1adb564615b51

```
其他示例（按目标类型）：

```

  dm-crypt,,8,ro,
    0 1048576 crypt aes-xts-plain64
    babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe 0
    /dev/sda 0 1 allow_discards

```
```

  dm-delay,,4,ro,0 409600 delay /dev/sda1 0 500

```
```

  dm-linear,,,rw,
    0 32768 linear /dev/sda1 0,
    32768 1024000 linear /dev/sda2 0,
    1056768 204800 linear /dev/sda3 0,
    1261568 512000 linear /dev/sda4 0

```
```

  dm-snap-orig,,4,ro,0 409600 snapshot-origin 8:2

```
```

  dm-striped,,4,ro,0 1638400 striped 4 4096
  /dev/sda1 0 /dev/sda2 0 /dev/sda3 0 /dev/sda4 0

```
```

  dm-verity,,4,ro,
    0 1638400 verity 1 8:1 8:2 4096 4096 204800 1 sha256
    fb1a5a0f00deb908d8b53cb270858975e76cf64105d412ce764225d53b8f3cfd
    51934789604d1b92399c52e7cb149d1b3a1b74bbbcb103b2a0aaacbed5c08584

```
对于在异步探测的块设备（MMC、USB 等）之上使用 device-mapper 的设置，可能需要告dm-init
在建device-mapper 表之前显式等待它们变为可用。这可以通过 "dm-mod.waitfor=" 完成
```

  dm-mod.waitfor=<device1>[,..,<deviceN>]

```
