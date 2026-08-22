
## Devlink 资源


`devlink` 提供了让驱动注册资源的能力，这可以让管理员查看给定资源的设备限制，以该给定资源当前的使用量。此外，这些资源可以选择具有可配置的大小。这可以使得管理能够限制所使用的资源数量
例如，`netdevsim` 驱动`/IPv4/fib` `/IPv4/fib-rules` 作为资源来限制给定设IPv4 FIB 条目和规则的数量
## 资源 Id


每个资源由一id 表示，并包含有关其当前大小以及相关子资源的信息。要访问子资源，
你需要指定该资源的路径。例`/IPv4/fib` `IPv4` 资源`fib` 子资源的 id
## 通用资源


通用资源用于描述可被多个设备驱动共享的资源，其描述必须添加到下表
   :widths: 10 90

   - - Name
     - Description
   - - `physical_ports`
     - 交换 ASIC 能够支持的物理端口的有限容量

### 使用示例


驱动暴露的资源可以被观察，例如：


    $devlink resource show pci/0000:03:00.0
    pci/0000:03:00.0:
      name kvd size 245760 unit entry
        resources:
          name linear size 98304 occ 0 unit entry size_min 0 size_max 147456 size_gran 128
          name hash_double size 60416 unit entry size_min 32768 size_max 180224 size_gran 128
          name hash_single size 87040 unit entry size_min 65536 size_max 212992 size_gran 128

某些资源的大小可以更改。例如：


    $devlink resource set pci/0000:03:00.0 path /kvd/hash_single size 73088
    $devlink resource set pci/0000:03:00.0 path /kvd/hash_double size 74368

更改不会立即生效，这可以通过 'size_new' 属性来验证，它代表待定（pending）的大小
更改。例如：


    $devlink resource show pci/0000:03:00.0
    pci/0000:03:00.0:
      name kvd size 245760 unit entry size_valid false
      resources:
        name linear size 98304 size_new 147456 occ 0 unit entry size_min 0 size_max 147456 size_gran 128
        name hash_double size 60416 unit entry size_min 32768 size_max 180224 size_gran 128
        name hash_single size 87040 unit entry size_min 65536 size_max 212992 size_gran 128

请注意，资源大小的更改可能需要重新加载设备才能正确生效
## 端口级资源与完整转储


除了设备级资源外，`devlink` 还支持端口级资源。这些资源与特定devlink 端口关联而非整个设备
要列出所devlink 设备和端口的资源

    $ devlink resource show
    pci/0000:03:00.0:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.0/196608:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.0/196609:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196708:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196709:
      name max_SFs size 128 unit entry dpipe_tables none

要显示特定端口的资源

    $ devlink resource show pci/0000:03:00.0/196608
    pci/0000:03:00.0/196608:
      name max_SFs size 128 unit entry dpipe_tables none

## 资源作用域过

在转储所有设备的资源时，`devlink resource show` 接受一个可选的 `scope` 参数，以响应限制为设备级资源、端口级资源，或两者（默认）
要仅转储所有设备的设备级资源：


    $ devlink resource show scope dev
    pci/0000:03:00.0:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1:
      name max_local_SFs size 128 unit entry dpipe_tables none
      name max_external_SFs size 128 unit entry dpipe_tables none

要仅转储所有设备的端口级资源：


    $ devlink resource show scope port
    pci/0000:03:00.0/196608:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.0/196609:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196708:
      name max_SFs size 128 unit entry dpipe_tables none
    pci/0000:03:00.1/196709:
      name max_SFs size 128 unit entry dpipe_tables none

请注意，端口级资源是只读的