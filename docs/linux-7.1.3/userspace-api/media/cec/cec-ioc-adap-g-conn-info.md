..
..


######## ioctl CEC_ADAP_G_CONNECTOR_INFO


## 名称


CEC_ADAP_G_CONNECTOR_INFO - 查询 HDMI 连接器信
## 概要



`int ioctl(int fd, CEC_ADAP_G_CONNECTOR_INFO, struct cec_connector_info *argp)`

## 参数


`fd`
    `open()` 返回的文件描述符
`argp`
    指向 struct cec_connector_info 的指针
## 描述


使用ioctl，应用程序可以获知此 CEC 设备对应于哪HDMI 连接器。调用此 ioctl 时，
应用程序应提供一个指cec_connector_info 结构体的指针，内核将用适配器驱动提供的信息
填充该结构体。仅当设置了 `CEC_CAP_CONNECTOR_INFO` 能力时，ioctl 才可用


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 8

    - - __u32
      - `type`
      - 该适配器所关联的适配器类型    - - union {
      - `(anonymous)`
    - - `struct cec_drm_connector_info`
      - drm
      - cec-drm-connector-info
    - - }
      -



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-CONNECTOR-TYPE-NO-CONNECTOR`:

      - `CEC_CONNECTOR_TYPE_NO_CONNECTOR`
      - 0
      - 没有与该适配器关联的连接驱动未提供该信息    - .. _`CEC-CONNECTOR-TYPE-DRM`:

      - `CEC_CONNECTOR_TYPE_DRM`
      - 1
      - 表示有一DRM 连接器与该适配器关联。有关该连接器的信息可在
	cec-drm-connector-info 中找到


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 8

    - .. _`CEC-DRM-CONNECTOR-TYPE-CARD-NO`:

      - __u32
      - `card_no`
      - DRM 卡编号：来自卡路径的编号，例/dev/card0 中的 0    - .. _`CEC-DRM-CONNECTOR-TYPE-CONNECTOR_ID`:

      - __u32
      - `connector_id`
      - DRM 连接ID