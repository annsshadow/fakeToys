## 内核驱动 i2c-amd756


支持的适配器：
  - AMD 756
  - AMD 766
  - AMD 768
  - AMD 8111

    Datasheets: 可在 AMD 网站公开获取

  - nVidia nForce

    Datasheet: 不可

Authors:
 - Frodo Looijaard <frodol@dds.nl>,
 - Philip Edelbrock <phil@netroedge.com>

### Description


该驱动支AMD 7566668 8111 外设总线控制器，以及 nVidia nForce

注意，对8111，有两个 SMBus 适配器。SMBus 1.0 适配器由本驱动支持，SMBus 2.0 适配器由 i2c-amd8111 驱动支持
