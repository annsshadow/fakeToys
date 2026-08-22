


######## ioctl MEDIA_IOC_ENUM_ENTITIES


## 名称


MEDIA_IOC_ENUM_ENTITIES - 枚举实体及其属
## 概要



`int ioctl(int fd, MEDIA_IOC_ENUM_ENTITIES, struct media_entity_desc *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 `media_entity_desc` 结构体的指针
## 描述


要查询某个实体的属性，应用程序需设置 `media_entity_desc` 结构体的 id 字段，并
以指向该结构体的指针调用 MEDIA_IOC_ENUM_ENTITIES ioctl。当 id 无效时，驱动填充结构体的其余部分，或返回 EINVAL 错误码

实体可以通过id `MEDIA_ENT_ID_FLAG_NEXT` 标志进行或运算来枚举。驱动将返回
id 严格大于所请求 id 的最小实体的信息（“下一个实体”）；若不存在，则返`EINVAL` 错误码
实体 ID 可以是非连续的。应用程*不得**尝试通过以递增id 不断调用
MEDIA_IOC_ENUM_ENTITIES 直到返回错误的方式来枚举实体


    :header-rows:  0
    :stub-columns: 0
    :widths: 2 2 1 8

    - -  __u32
       - `id`
       -
       - 实体 ID，由应用程序设置。当 ID `MEDIA_ENT_ID_FLAG_NEXT` 进行或运算时	  驱动会清除该标志并返ID 更大的第一个实体。不要期望每次打开设备实例	  ID 都相同。换言之，不要在应用程序中将实ID 硬编码
    - -  char
       - `name`\ [^32^]
       -
       - UTF-8 NULL 结尾字符串表示的实体名称。该名称在媒体拓扑内必须唯一
    - -  __u32
       - `type`
       -
       - 实体类型，详media-entity-functions
    - -  __u32
       - `revision`
       -
       - 实体版本号。始终为零（已废弃）
    - -  __u32
       - `flags`
       -
       - 实体标志，详media-entity-flag
    - -  __u32
       - `group_id`
       -
       - 实体ID。始终为零（已废弃）
    - -  __u16
       - `pads`
       -
       - pad 的数量
    - -  __u16
       - `links`
       -
       - 出站链接的总数。入站链接不计入该字段
    - -  __u32
       - `reserved[^4^]`
       -
       - 为未来扩展保留。驱动和应用程序必须将该数组置为零
    - -  union {
       - (anonymous)

    - -  struct
       - `dev`
       -
       - 对创建单个设备节点的（子）设备有效
    - -
       - __u32
       - `major`
       - 设备节点主设备号
    - -
       - __u32
       - `minor`
       - 设备节点次设备号
    - -  __u8
       - `raw`\ [^184^]
#        -

    - - }
       -

## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在
Generic Error Codes <gen-errors> 章节中描述
EINVAL
    结构`media_entity_desc` `id` 引用了一个不存在的实体
