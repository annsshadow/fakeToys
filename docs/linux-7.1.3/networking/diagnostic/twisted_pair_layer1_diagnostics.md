
## 针对 OSI 第一层双绞线以太网变体的诊断思路


### 简

本文档面向两类主要读者：

1. **用户与系统管理员**：对于那些面对真实世界以太网问题的人，本指南提供了一   实用的、逐步的故障排查流程，帮助识别并解决双绞线以太网在 OSI 第一层（Layer 1   的常见问题。如果你遇到不稳定的链路、速率下降或莫名其妙的网络问题，直接跳   分步指南，跟着它走到底来找到你的解决方案
2. **内核开发*：对于从事网络驱动和 PHY 支持的开发者，本文档概述了诊断流程   并突出了 Linux 内核诊断接口可以扩展或改进的地方。通过理解诊断流程，开发者可   更好地为未来的增强排定优先级
### 基于 Linux 的分步诊断指南（通用以太网）


本诊断指南涵盖常见的以太网故障排查场景，侧重于跨不同以太网环境的**链路稳定与检*，包*单对以太网（SPE，Single-Pair Ethernet**多对以太网（MPEMulti-Pair Ethernet*，以及像 **PoDL**（数据线上供电，Power over Data Line）和
**PoE**（Clause 33 PSE）这样的供电技术
本指南旨在帮助用户诊断运**Linux 内核 6.11 或更新版*、使**ethtool 6.10 更高版本**以及 **iproute2 6.4.0 或更高版*的系统上的物理层（Layer 1）问题
在本指南中，我们假设用户可能**有限或完全无法访问链路对*，并将专注于在本诊断问题
#### 诊断场景


- **链路已建立且稳定，但没有数据传输**：如果链路稳定但数据传输有问题，请参  **OSI 第二层故障排查指*
- **链路不稳*：链路复位、速率下降或其它波动表明硬件或物理层存在潜在问题
- **未检测到链路**：接口已 up，但没有建立链路
#### 验证接口状

首先验证以太网接口的状态，检查它是否在管理上处于 up。与提供链路PHY 状态信息的
`ethtool` 不同，它不显示接口的**管理状态（administrative state*。要检查这一点，
你应该使`ip` 命令，它在其输出的尖括号 `"<>"` 内描述接口状态
例如，在输出 `<NO-CARRIER,BROADCAST,MULTICAST,UP>` 中，重要的关键字是：

- **UP**：接口处于管理上的“UP”状态- **NO-CARRIER**：接口在管理上已 up，但未检测到物理链路
如果输出显示 `<BROADCAST,MULTICAST>`，这表明接口处于管理上的“DOWN”状态
- **命令* `ip link show dev <interface>`

- **预期输出*

  .. code-block:: bash

     4: eth0: <NO-CARRIER,BROADCAST,MULTICAST,UP> mtu 1500 ...
        link/ether 88:14:2b:00:96:f2 brd ff:ff:ff:ff:ff:ff

- **解读输出*

  - **管理上的 UP 状*
    - 如果输出包含 **"UP"**，接口在管理上已 up，系统正在尝试建立物理链路
    - 如果你还看到 **"NO-CARRIER"**，意味着未检测到物理链路，表明存在潜在的 Layer 1
      问题，例如线缆故障、配置错误或链路对端未连接。在这种情况下，转到**检查链路状      PHY 配置**一节
  - **管理上的 DOWN 状*
    - 如果输出缺少 **"UP"** 且只显示**"<BROADCAST,MULTICAST>"** 这样的状态，意味着
      接口在管理上down。在这种情况下，使用以下命令将接up
      .. code-block:: bash

         ip link set dev <interface> up

- **后续步骤*

  - 如果接口**在管理上up** 但显**NO-CARRIER**，转*检查链路状态与 PHY 配置**
    一节，排查潜在的物理层问题
  - 如果接口之前**在管理上down** 而你已将up，请务必**重复这一验证步骤**，在
    继续之前确认接口的新状态
  - **如果接口up 且检测到了链路：**

    - 如果输出显示 **"UP"** 且没**`NO-CARRIER`**，接口在管理上已 up，并且物理链      已成功建立。如果一切按预期工作，Layer 1 的诊断就完成了，无需进一步操作
    - 如果接口up 且检测到了链路，*没有数据传输**，问题很可能超出Layer 1，你
      应继续进OSI 模型更高层的诊断。这可能涉及检Layer 2 配置（如 VLAN MAC
      地址问题）、Layer 3 设置（如 IP 地址、路由或 ARP），Layer 4 及以上（防火墙      服务等）
    - 如果**链路不稳**频繁复位或掉*，这可能表明物理层问题，例如有故障的线缆      干扰或供电问题。在这种情况下，继续进行本指南的下一步
#### 检查链路状态与 PHY 配置


使用 `ethtool -I` 检查链路状态、PHY 配置、受支持的链路模式，以及诸如 **Link Down
Events**（链路断开事件）计数器这样的额外统计。这一步对于诊断速度不匹配、双工问题和
链路不稳定等 Layer 1 问题至关重要
对于**单对以太网（SPE**多对以太网（MPE*设备，你都会使用这一步来收集有关
链路的关键细节*SPE** 链路通常支持单一速度和模式、且不进行自动协商（**10BaseT1L**
除外），**MPE** 设备通常支持多种链路模式和自动协商
- **命令* `ethtool -I <interface>`

- **SPE 接口示例输出（非自动协商）：**

  .. code-block:: bash

     Settings for spe4:
         Supported ports: [ TP ]
         Supported link modes:   100baseT1/Full
         Supported pause frame use: No
         Supports auto-negotiation: No
         Supported FEC modes: Not reported
         Advertised link modes: Not applicable
         Advertised pause frame use: No
         Advertised auto-negotiation: No
         Advertised FEC modes: Not reported
         Speed: 100Mb/s
         Duplex: Full
         Auto-negotiation: off
         master-slave cfg: forced slave
         master-slave status: slave
         Port: Twisted Pair
         PHYAD: 6
         Transceiver: external
         MDI-X: Unknown
         Supports Wake-on: d
         Wake-on: d
         Link detected: yes
         SQI: 7/7
         Link Down Events: 2

- **MPE 接口示例输出（自动协商）*

  .. code-block:: bash

     Settings for eth1:
         Supported ports: [ TP    MII ]
         Supported link modes:   10baseT/Half 10baseT/Full
                                 100baseT/Half 100baseT/Full
         Supported pause frame use: Symmetric Receive-only
         Supports auto-negotiation: Yes
         Supported FEC modes: Not reported
         Advertised link modes:  10baseT/Half 10baseT/Full
                                 100baseT/Half 100baseT/Full
         Advertised pause frame use: Symmetric Receive-only
         Advertised auto-negotiation: Yes
         Advertised FEC modes: Not reported
         Link partner advertised link modes:  10baseT/Half 10baseT/Full
                                              100baseT/Half 100baseT/Full
         Link partner advertised pause frame use: Symmetric Receive-only
         Link partner advertised auto-negotiation: Yes
         Link partner advertised FEC modes: Not reported
         Speed: 100Mb/s
         Duplex: Full
         Auto-negotiation: on
         Port: Twisted Pair
         PHYAD: 10
         Transceiver: internal
         MDI-X: Unknown
         Supports Wake-on: pg
         Wake-on: p
         Link detected: yes
         Link Down Events: 1

- **后续步骤*

  - 记录 `ethtool` 提供的输出，特别注意 **master-slave status**（主从状态）    **speed**（速率）*duplex**（双工）以及其它相关字段。这些信息将有助于进一    分析或故障排查。一旦收集并保存**ethtool** 输出，就进入下一个诊断步骤
#### 检查供电（PoDL PoE

如果已知系统**未实* **PoDL** **PoE**，或**PSE**（供电设备，Power Sourcing
Equipment）由专有用户空间软件或外部工具管理，你可以跳过这一步。在这种情况下，通过
替代方法验证供电，例如检查硬件指示灯（LED）、使用万用表，或查阅厂商特定的软件来
监视供电状态
如果 **PoDL** **PoE** Linux 直接实现和管理，请按照以下步骤确保供电正确传递：

- **命令* `ethtool --show-pse <interface>`

- **预期输出示例*

  1. **PSE 不受支持**
     如果未连PSE，或接口不支PSE，预期出现以下输出：

     .. code-block:: bash

        netlink error: No PSE is attached
        netlink error: Operation not supported

  2. **PoDL（单对以太网*
     当实现了 PoDL 时，你可能会看到以下属性：

     .. code-block:: bash

        PSE attributes for eth1:
        PoDL PSE Admin State: enabled
        PoDL PSE Power Detection Status: delivering power

  3. **PoE（Clause 33 PSE*
     对于标准 PoE，输出可能如下：

     .. code-block:: bash

        PSE attributes for eth1:
        Clause 33 PSE Admin State: enabled
        Clause 33 PSE Power Detection Status: delivering power
        Clause 33 PSE Available Power Limit: 18000

- **调整供电限制（如有需要）*

  - 有时，可用的供电限制可能不足以满足链路对端。你可以按需提高供电限制
  - **命令* `ethtool --set-pse <interface> c33-pse-avail-pw-limit <limit>`

    示例
    .. code-block:: bash

      ethtool --set-pse eth1 c33-pse-avail-pw-limit 18000
      ethtool --show-pse eth1

    **预期输出**（调整供电限制后）：

    .. code-block:: bash

      Clause 33 PSE Available Power Limit: 18000


- **后续步骤*

  - **未使PoE PoDL**：如果系统上**未实**未使* **PoE** **PoDL**    继续下一步诊断，因为供电传递与本配置无关
  - **PoE PoDL 由外部控*：如*使用****PoE** **PoDL**，但不是Linux
    内核**PSE-PD** 框架管理（即由专有用户空间软件或外部工具控制），这部分不在本文档
    范围内。请查阅厂商特定的文档或外部工具来监视和管理供电传递
  - **PSE 管理状态已禁用**
    - 如果 `PSE Admin State:` **disabled**，通过运行以下命令之一来启用它
      .. code-block:: bash

         ethtool --set-pse <devname> podl-pse-admin-control enable

      或者，对于 Clause 33 PSE（PoE）：

         ethtool --set-pse <devname> c33-pse-admin-control enable

    - 启用 PSE 管理状态后，返*检查供电（PoDL PoE*步骤的开头，重新检查供      传递状态
  - **未传递供*：如`Power Detection Status` 显示的不"delivering power"（例    `over current`），则排**PSE**。检查潜在问题，例如线缆中的短路、供电不足，PSE
    本身的故障
  - **已供电但无链*：如果正在供电但未建立链路，通过执行**线缆诊断**或回*检查链    状态与 PHY 配置**步骤来进行进一步诊断，以识别物理链路或设置中的任何潜在问题
#### 线缆诊断


使用 `ethtool` 测试物理层问题，例如线缆故障。测试结果可能因线缆状况、所用技术以链路对端的状态而异。线缆测试的结果有助于诊断开路、短路、阻抗不匹配和噪声相关问题
- **命令* `ethtool --cable-test <interface>`

以下*单对以太网（SPE**多对以太网（MPE*的典型输出：

- **对于单对以太网（SPE）：**
  - **预期输出（SPE）：**

  .. code-block:: bash

    Cable test completed for device eth1.
    Pair A, fault length: 25.00m
    Pair A code Open Circuit

  这表明在报告的距离处存在开路或线缆故障，但结果可能受链路对端状态的影响。请参阅
  **"根据线缆测试结果进行故障排查"**一节，以进一步解读这些结果
- **对于多对以太网（MPE）：**
  - **预期输出（MPE）：**

  .. code-block:: bash

    Cable test completed for device eth0.
    Pair A code OK
    Pair B code OK
    Pair C code Open Circuit

  这里，Pair C 被报告为开路，Pair A Pair B 工作正常。但是，如果Pair A Pair B
  上使用了自动协商，线缆测试可能会被打断。请参阅**"根据线缆测试结果进行故障排查"**一节，
  以获取这些问题的详细解释以及如何解决它们
有关各种可能的线缆测试结果的详细描述，请参阅**"根据线缆测试结果进行故障排查"**一节
##### 根据线缆测试结果进行故障排查


运行线缆测试后，结果有助于识别物理连接中的具体问题。不过，需要注意的是，**线缆测试
结果在很大程度上取决于本地硬件和链路对端的能力与特*。结果的准确性和可靠性在不同
硬件实现之间可能有显著差异
在某些情况下，这会在当前的线缆测试实现中引入**盲点**，某些结果可能无法准确反映线缆的
实际物理状态。例如：

- **开路（Open Circuit*结果可能不仅表明线缆受损或断开，也可能在线缆正确连接到
  已断电的链路对端时发生
- 某些 PHY 在链路对端处*强制从模式（forced slave mode*时，即使线缆中实际没  短路，也可能报告**线对内短路（Short within Pair*
为了帮助用户更有效地解读结果，扩*内核 UAPI**（User API，用户API）以根据硬件
特性提供额外上下文或问题的**可能变体**，可能是有益的。由于这些怪癖通常是硬件特定的**内核驱动**将是此类信息的理想来源。通过为每个测试结果提供与潜在误报相关的标志或提示用户将更好地理解需要验证什么、以及进一步在哪里排查
在做出此类改进之前，用户应意识到这些限制，并在需要时手动验证线缆问题。物理检查可能有
助于解决与误报结果相关的不确定性
结果可能是以下几种之一
- **OK（正常）**
  - 线缆工作正常，未检测到问题
  - **后续步骤**：如果你仍然遇到问题，它可能与更高层的问题有关，例如双工不匹配或速度
    协商，这些都不是物理层问题
  - **`BaseT1` 的特殊情况（1000/100/10BaseT1*：在 `BaseT1` 系统中，"OK" 结果通常    意味着链路up，并且可能处*从模式（slave mode*，因为线缆测试通常只在这种模式
    下通过。对于某**10BaseT1L** PHY，即使线缆长度超过了 PHY 所配置范围（例如，范围
    被配置为短距离模式），也可能出现 "OK" 结果
- **开路（Open Circuit*
  - **开路（Open Circuit*结果通常表明线缆在报告的故障长度处受损或断开。考虑以下可能
    性：

    - 如果链路对端处于 **admin down**（管down）状态或已断电，即使线缆功能正常，你
      仍可能得"Open Circuit" 结果
    - **后续步骤**：在故障长度处检查线缆是否有可见的损坏或松动的连接。确认链路对端已
      通电并处于正确的模式
- **线对内短路（Short within Pair*
  - **线对内短路（Short within Pair*表示同一对导线内存在非预期的连接，通常由线缆的
    物理损坏引起
    - **后续步骤**：更换或修复线缆，并检查是否有任何物理损坏或压接不当的连接器
- **与另一对短路（Short to Another Pair*
  - **与另一对短路（Short to Another Pair*意味着来自不同对的导线短路，这可能由物    损坏或接线错误引起
    - **后续步骤**：更换或修复受损线缆。检查线缆是否有不正确的端接或被夹坏的布线
- **阻抗不匹配（Impedance Mismatch*
  - **阻抗不匹配（Impedance Mismatch*表示由线缆中阻抗不连续引起的反射。这可能发生在线
    缆的某部分具有异常阻抗时（例如，当不同类型的线缆被拼接在一起，或线缆中存在缺陷）
    - **后续步骤**：检查线缆质量，并确保整条线缆阻抗一致。更换任何不符合规格的线缆段
- **噪声（Noise*
  - **噪声（Noise*意味着时域反射计（TDR，Time Domain Reflectometry）测试由于线缆上    过量噪声而无法完成，这可能由电磁源的干扰引起
    - **后续步骤**：识别并消除线缆附近的电磁干扰（EMI）源。考虑使用屏蔽线缆，或将线      重新布线以远离噪声源
- **无法分辨（Resolution Not Possible*
  - **无法分辨（Resolution Not Possible*意味着 TDR 测试由于测试的分辨率限制，或因为
    故障超出了测试可测量的距离，而无法检测到问题
    - **后续步骤**：如果可能，手动检查线缆，或使用能够处理更大距离或更高分辨率的替代
      诊断工具
- **未知（Unknown*
  - **未知（Unknown*结果可能在测试无法对故障分类，或特定问题超出工具检测能力范围时
    发生
    - **后续步骤**：重新运行测试，验证链路对端的状态，并在必要时手动检查线缆
#### 验证链路对端 PHY 配置


如果线缆测试通过，但链路仍然无法正常工作，就必须验证链路对端PHY 配置。速度、双设置或主从角色的不匹配都可能导致连接问题
##### 自动协商不匹

- 如果两个链路对端都支持自动协商，请确保两端都启用了自动协商，并且通告了所有受支持  链路模式。不匹配可能导致连接问题或次优性能
- **快速修复：** 将自动协商重置为默认设置，这将通告所有默认的链路模式
  .. code-block:: bash

     ethtool -s <interface> autoneg on

- **检查配置的命令* `ethtool <interface>`

- **预期输出* 确保两端通告兼容的链路模式。如果自动协商关闭，请验证两个链路对端都
  配置为相同的速度和双工
  以下示例展示了一个本PHY 通告的链路模式少于其支持的情况。这会减少对端重叠的链路
  模式数量。在最坏情况下，将没有共同的链路模式，链路将无法建立：

  .. code-block:: bash

     Settings for eth0:
        Supported link modes:  1000baseT/Full, 100baseT/Full
        Advertised link modes: 1000baseT/Full
        Speed: 1000Mb/s
        Duplex: Full
        Auto-negotiation: on

##### 组合模式不匹配（一端自动协商，另一端强制）


- 当一端使*自动协商**（如大多数现代系统），而另一端被设置*强制链路模式**（例  具有单速集线器的较旧硬件）时，可能会出现一种问题。在这种情况下，现代 PHY 将尝试检  另一端的强制模式。如果建立了链路，你可能会注意到
  - **没有或为"Link partner advertised link modes"**（链路对端通告的链路模式）
  - **"Link partner advertised auto-negotiation:"**（链路对端通告的自动协商：）将    **"no"** 或不出现
- 这类检测并非总是可靠工作
  - 通常，现PHY 会默认到**半双工（Half Duplex*，即使链路对端实际配置为
    **全双工（Full Duplex*
  - 如果链路对端从一种强制模式切换到另一种，某些 PHY 可能无法可靠工作。在这种情况下，
    只有一down/up 循环可能有帮助
- **后续步骤**：将两端设置为相同的固定速度和双工模式，以避免潜在的检测问题
  .. code-block:: bash

     ethtool -s <interface> speed 1000 duplex full autoneg off

##### 从角色不匹配（BaseT1 1000BaseT PHY

- **BaseT1** 系统（例1000BaseT100BaseT1）中，建立链路要求一个设备配置为
  **master**（主），另一个配置为 **slave**（从）。这种主从配置的不匹配会阻止链路建立  不过*1000BaseT** 也支持可配置的主/从角色，并可能面临类似问题
- **1000BaseT 中的角色偏好***1000BaseT** 规范允许链路对端在自动协商期间协商主从角  或角色偏好。某PHY 有硬件限制或缺陷，使它们无法在特定角色下正常工作。在这种情况下，
  驱动可能会将这些 PHY 强制进入特定角色（例**forced master** **forced slave**），
  或通过设置偏好来尝试较弱的选项。如果两个链路对端有相同的问题、并且都被强制进入相同的
  模式（例如都被强制进入主模式），它们将无法建立链路
- **后续步骤**：确保一端配置为 **master**，另一端配置为 **slave**，以避免此问题，特别  在涉及硬件限制时；或者尝试较弱的 **preferred**（偏好）选项，而不**forced**（强制）  检查任何与驱动相关的限制或强制模式
- **强制从模式的命令**
  .. code-block:: bash

     ethtool -s <interface> master-slave forced-master

  或：

  .. code-block:: bash

     ethtool -s <interface> master-slave forced-master speed 1000 duplex full autoneg off


- **检查当前的从状*
  .. code-block:: bash

     ethtool <interface>

  示例输出
  .. code-block:: bash

     master-slave cfg: forced-master
     master-slave status: master

- **硬件缺陷与驱动强*：如果已知的硬件问题PHY 强制进入特定模式，必须检查驱动源代码
  或硬件文档以了解细节。确保角色在两个链路对端之间兼容，如果两PHY 都被强制进入相同
  模式，相应地调整一端以解决不匹配
#### 监视链路复位与速率下降


如果链路不稳定，显示出频繁的复位或速率下降，这可能表明线缆、PHY 配置或环境因素存在问题虽然 Linux 中仍没有一种完全统一的方式通过用户空间工具直接监视降速（downshift）事件或
链路速度变化，但 Linux 内核日志`ethtool` 都能提供有价值的见解，尤其是在驱动支报告此类事件时
- **监视内核日志中的链路复位与速率下降**
  - Linux 内核会在系统日志中打印链路状态变化，包括降速事件。这些消息通常包含速度变化    双工模式，以及降速后的链路速度（如果驱动支持）
  - **实时监视内核日志的命令：**

    .. code-block:: bash

      dmesg -w | grep "Link is Up\|Link is Down"

  - 示例输出（如果发生降速）
    .. code-block:: bash

      eth0: Link is Up - 100Mbps/Full (downshifted) - flow control rx/tx
      eth0: Link is Down

    这表明链路已建立，但已从更高速度降速
  - **注意**：并非所有驱动或 PHY 都支持降速报告，因此你可能不会在所有设备上看到    信息
- **使用 `ethtool` 监视链路断开事件**
  - 从最新的内核`ethtool` 版本开始，你可以使`ethtool -I` 命令跟踪 **Link Down
    Events**（链路断开事件）。这将提供链路掉线的计数器，在驱动支持的情况下有助于诊断
    链路不稳定问题
  - **监视链路断开事件的命令：**

    .. code-block:: bash

      ethtool -I <interface>

  - 示例输出（如果支持）
    .. code-block:: bash

      PSE attributes for eth1:
      Link Down Events: 5

    这表明链路已掉线 5 次。频繁的链路断开事件可能表明需要深入调查的线缆或环境问题
- **检查链路状态与速度**
  - 尽管降速计数或事件不容易跟踪，你仍然可以使`ethtool` 手动检查当前的链路速度    状态
  - **命令* `ethtool <interface>`

  - **预期输出*

    .. code-block:: bash

      Speed: 1000Mb/s
      Duplex: Full
      Auto-negotiation: on
      Link detected: yes

    预期速度或双工设置中的任何不一致都可能表明存在问题
- **为诊断禁用节能以太网（EEE*
  - **EEE**（Energy-Efficient Ethernet，节能以太网）可能由于进出低功耗状态的转换而成    链路不稳定的来源。出于诊断目的，**临时**禁用 EEE 以判断它是否导致了链路不稳定，可    是有用的。这**不是**禁用电源管理的通用建议
  - **后续步骤**：禁EEE 并监视链路是否变得稳定。如果禁EEE 解决了问题，请报告该
    bug，以便修复驱动
  - **命令*

    .. code-block:: bash

      ethtool --set-eee <interface> eee off

  - **重要**：如果禁EEE 解决了不稳定问题，应将该问题作为 bug 报告给维护者，并且驱动
    应被修正为在不引起不稳定的情况下正确处理 EEE。永久禁EEE 不应被视为一种解决方案
- **监视错误计数*
  - 如果驱动支持统一接口，使`ethtool -S <interface> --all-groups` 来获取标准化接口
    统计
  - **命令* `ethtool -S <interface> --all-groups`

  - **示例输出（如果支持）*

    .. code-block:: bash

      phydev-RxFrames: 100391
      phydev-RxErrors: 0
      phydev-TxFrames: 9
      phydev-TxErrors: 0

  - 如果不支持统一接口，使`ethtool -S <interface>` 来获MAC PHY 计数器。请注意    非标准化PHY 计数器名称因驱动而异，必须相应地解读
  - **命令* `ethtool -S <interface>`

  - **示例输出（如果支持）*

    .. code-block:: bash

      rx_crc_errors: 123
      tx_errors: 45
      rx_frame_errors: 78

  - **注意**：如果没有有意义的错误计数器可用，或者计数器不受支持，你可能需要依赖物    检查（例如线缆状况）或内核日志消息（例如链up/down 事件）来进一步诊断问题
  - **比较计数*
    - 比较 PHY MAC 报告的出口和入口帧计数
    - 小的差异可能MAC PHY 驱动之间的采样率差异引起，或者由 PHY MAC UP       DOWN 状态并非始终完全同步引起
    - 显著的差异表MAC PHY 之间的数据路径存在潜在问题
#### 当一切都失败时…

所以你已经检查了线缆、监视了日志、禁用了 EEE，但仍然……什么都没有？别担心，你并不
孤单。有时候，以太网小妖精就是不想配合
但在你认输（或认输在线缆上）之前，深呼吸。总有可能
1. 你的 PHY 有独特、未文档化的“个性”
2. 问题处于休眠状态，正等待合适的时机神奇地自行解决（嘿，这种事会发生！）
3. 或者，终极解决方案根本还没被发明出来
如果以上都不能给你安慰，还有最后一步：贡献！如果你发现了新的或不寻常的问题，或者有
创造性的诊断方法，欢迎分享你的发现并扩展本文档。同心协力，我们能追查到每一个难以捉的网络问题——一次一对双绞线
记住：有时解决方案只需一次重启，但如果没有，是时候深入挖掘——或者报告那bug 了！
