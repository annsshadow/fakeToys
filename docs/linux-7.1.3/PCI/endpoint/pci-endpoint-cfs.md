
## 使用 CONFIGFS 配置 PCI Endpoint

:Author: Kishon Vijay Abraham I <kishon@ti.com>

PCI Endpoint Core 通过 configfs 入口（pci_ep）来配置 PCI endpoint function并将其与 endpoint controller 绑定。（关于配置 PCI Endpoint Function 的其机制，请参阅 [^1^]。）

## 挂载 configfs

PCI Endpoint Core 层会在已挂载configfs 中创pci_ep 目录
```

	mount -t configfs none /sys/kernel/config

```
## 目录结构

pci_ep configfs 在其根目录下有两个目录：controllers functions。系统中
存在的每EPC 设备都会**controllers** 目录下有一项，系统中存在的每个
EPF 驱动都会**functions** 目录下有一项```

	/sys/kernel/config/pci_ep/
		.. controllers/
		.. functions/

```
## 创建 EPF 设备

每个已注册的 EPF 驱动都会列在 controllers 目录下。与 EPF 驱动对应的项EPF
核心创建```

	/sys/kernel/config/pci_ep/functions/
		.. <EPF Driver1>/
			... <EPF Device 11>/
			... <EPF Device 21>/
			... <EPF Device 31>/
		.. <EPF Driver2>/
			... <EPF Device 12>/
			... <EPF Device 22>/

```
为了创建<EPF Driver> 探测的类型的 <EPF device>，用户必须在 <EPF DriverN>
内创建一个目录
每个 <EPF device> 目录都包含以下可用于配置 endpoint function 标准配置头的
项。（这些项在创建任何新的 <EPF Device> 时由框架创建```

		.. <EPF Driver1>/
			... <EPF Device 11>/
				... vendorid
				... deviceid
				... revid
				... progif_code
				... subclass_code
				... baseclass_code
				... cache_line_size
				... subsys_vendor_id
				... subsys_id
				... interrupt_pin
			        ... <Symlink EPF Device 31>/
                                ... primary/
			                ... <Symlink EPC Device1>/
                                ... secondary/
			                ... <Symlink EPC Device2>/

```
如果一EPF 设备需要关2 EPC（例如非透明桥的情况），则应将连接到（primary）接口的 endpoint controller 的符号链接添加到 'primary' 目录中，连接到从（secondary）接口的 endpoint controller 的符号链接添加到 'secondary'
目录中
<EPF Device> 目录可以包含指向其它 <EPF Device> 的符号链接列表（<Symlink EPF
Device 31>）。这些符号链接应由用户创建，用于表示绑定到物理功能的虚拟功能。在
上述目录结构中，<EPF Device 11> 是物理功能，<EPF Device 31> 是虚拟功能。一EPF 设备一旦链接到另一EPF 设备，就不能再链接到 EPC 设备
## EPC 设备

每个已注册的 EPC 设备都会列在 controllers 目录下。与 EPC 设备对应的项EPC
核心创建```

	/sys/kernel/config/pci_ep/controllers/
		.. <EPC Device1>/
			... <Symlink EPF Device11>/
			... <Symlink EPF Device12>/
			... start
		.. <EPC Device2>/
			... <Symlink EPF Device21>/
			... <Symlink EPF Device22>/
			... start

```
<EPC Device> 目录会包含一个指<EPF Device> 的符号链接列表。这些符号链接应
由用户创建，用于表示 endpoint 设备中的功能。只有表示物理功能的 <EPF Device>
才能链接EPC 设备
<EPC Device> 目录还会有一**start** 字段。一旦向该字段写"1"，endpoint
设备就准备好与主机建立链路。这通常是在所EPF 设备创建并链接到 EPC 设备之后
进行的```

			 | controllers/
				| <Directory: EPC name>/
					| <Symbolic Link: Function>
					| start
			 | functions/
				| <Directory: EPF driver>/
					| <Directory: EPF device>/
						| vendorid
						| deviceid
						| revid
						| progif_code
						| subclass_code
						| baseclass_code
						| cache_line_size
						| subsys_vendor_id
						| subsys_id
						| interrupt_pin
						| function

```
[^1^] Documentation/PCI/endpoint/pci-endpoint.rst
