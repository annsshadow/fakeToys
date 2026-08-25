


######## ioctl MEDIA_IOC_ENUM_LINKS


## 名称


MEDIA_IOC_ENUM_LINKS - 枚举给定实体的所pad 和链
## 概要


`int ioctl(int fd, MEDIA_IOC_ENUM_LINKS, struct media_links_enum *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct `media_links_enum` 的指针
## 描述


为枚举给定实体的 pad 或链接，应用程序设置 struct `media_links_enum` entity 字段，并初始化由 `pads` `links` 字段指向struct `media_pad_desc` struct `media_link_desc` 结构数组。然后它们以指向该结构的指针调用 MEDIA_IOC_ENUM_LINKS ioctl
如果 `pads` 字段NULL，驱动会用关于该实体 pad 的信息填`pads` 数组。该数组必须有足够空间存储该实体的所pad。pad 的数量可通过 MEDIA_IOC_ENUM_ENTITIES 获取
如果 `links` 字段NULL，驱动会用关于该实体出站链接的信息填`links` 数组。该数组必须有足够空间存储该实体的所有出站链接。出站链接的数量可通过 MEDIA_IOC_ENUM_ENTITIES 获取
在枚举过程中，仅返回起源于该实体某个 source pad 的前向链接



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  __u32
       - `entity`
       - 实体 id，由应用程序设置
    - -  struct `media_pad_desc`
       - \*\ `pads`
       - 指向由应用程序分配的 pads 数组的指针。若NULL 则忽略
    - -  struct `media_link_desc`
       - \*\ `links`
       - 指向由应用程序分配的 links 数组的指针。若NULL 则忽略
    - -  __u32
       - `reserved[^4^]`
       - 为未来扩展保留。驱动和应用程序必须将该数组置零



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  __u32
       - `entity`
       - pad 所属实体的 ID
    - -  __u16
       - `index`
       - pad 索引，从 0 开始
    - -  __u32
       - `flags`
       - pad 标志，详media-pad-flag
    - -  __u32
       - `reserved[^2^]`
       - 为未来扩展保留。驱动和应用程序必须将该数组置零



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - -  struct `media_pad_desc`
       - `source`
       - 此链接起点的 pad
    - -  struct `media_pad_desc`
       - `sink`
       - 此链接目标的 pad
    - -  __u32
       - `flags`
       - 链接标志，详media-link-flag
    - -  __u32
       - `reserved[^2^]`
       - 为未来扩展保留。驱动和应用程序必须将该数组置零
## 杩斿洖鍊。

成功时返0，出错时返回 -1 并相应地设置 `errno` 变量。通用错误码在 Generic Error Codes <gen-errors> 章节中描述
EINVAL
    struct `media_links_enum` `id` 引用了不存在的实体