
## Devlink Region


`devlink` region 支持使用 devlink 访问由驱动定义的地址区域
每个设备可以创建并注册它自己支持的地址区域。随后可通过 devlink region 接口访问该区域
区域快照由驱动采集，并可通过 read dump 命令访问。这允许对创建的快照进行后续分析区域可以选择性地支持按需触发快照
快照标识符的作用域是 devlink 实例，而不是某个区域。同一 devlink 实例中所有具有相同快id 快照对应于同一事件
创建区域的主要好处是提供对内部地址区域的访问，这些区域原本对用户是不可访问的
区域也可用于提供调试复杂错误状态的额外方式，但另请参见 Documentation/networking/devlink/devlink-health.rst

区域可以选择性地支持通过 `DEVLINK_CMD_REGION_NEW` netlink 消息按需捕获快照。一个希望允请求快照的驱动必须在它的 `devlink_region_ops` 结构体中实现 `.snapshot` 回调。如果在
`DEVLINK_CMD_REGION_NEW` 请求中没有设置快id，内核将分配一个并把快照信息发送给用户空间
区域可以选择性地允许在没有快照的情况下直接从其内容读取。直接读取请求不是原子的。特别地，大256 字节或更大的读取请求会被拆分成多个块。如果需要原子访问，请使用快照。一个希望为此启的驱动应该在 `devlink_region_ops` 结构体中实现 `.read` 回调。用户空间可以通过使用
`DEVLINK_ATTR_REGION_DIRECT` 属性而不是指定快id 来请求直接读取
### 使用示例


    $ devlink region help
    $ devlink region show [ DEV/REGION ]
    $ devlink region del DEV/REGION snapshot SNAPSHOT_ID
    $ devlink region dump DEV/REGION [ snapshot SNAPSHOT_ID ]
    $ devlink region read DEV/REGION [ snapshot SNAPSHOT_ID ] address ADDRESS length LENGTH

    # 显示所有暴露的区域及其区域大小    $ devlink region show
    pci/0000:00:05.0/cr-space: size 1048576 snapshot [1 2] max 8
    pci/0000:00:05.0/fw-health: size 64 snapshot [1 2] max 8

    # 使用以下命令删除一个快照：
    $ devlink region del pci/0000:00:05.0/cr-space snapshot 1

    # 请求一个即时快照（如果该区域支持）
    $ devlink region new pci/0000:00:05.0/cr-space
    pci/0000:00:05.0/cr-space: snapshot 5

    # 转储一个快照：
    $ devlink region dump pci/0000:00:05.0/fw-health snapshot 1
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8
    0000000000000020 0016 0bb8 0016 1720 0000 0000 c00f 3ffc
    0000000000000030 bada cce5 bada cce5 bada cce5 bada cce5

    # 读取快照的特定部分：
    $ devlink region read pci/0000:00:05.0/fw-health snapshot 1 address 0 length 16
    0000000000000000 0014 95dc 0014 9514 0035 1670 0034 db30

    # 在没有快照的情况下从区域读取
    $ devlink region read pci/0000:00:05.0/fw-health address 16 length 16
    0000000000000010 0000 0000 ffff ff04 0029 8c00 0028 8cc8

由于区域很可能非常依赖于设备或驱动，因此没有定义通用的区域。有关某个驱动支持的具体区域的信息，
请参见驱动专有文档文件