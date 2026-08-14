
## NET_FAILOVER


## 概述


net_failover 驱动通过 API 提供自动故障转移机制，用于创建和销毁一个故障转移主网络设备（netdev），并管理通过通用故障转移基础设施注册的主（primary）与备用（standby）从网络设备（slave netdev）。

故障转移 netdev 充当主设备，控制 2 个从设备。原始的半虚拟化接口被注册为 'standby' 从 netdev，而具有相同 MAC 的 passthru/vf 设备被注册为 'primary' 从 netdev。'standby' 与 'failover' netdev 都关联到同一个 'pci' 设备。用户通过 'failover' netdev 访问网络接口。当 'primary' netdev 可用且链路已启用并运行时，'failover' netdev 会将其选为发送（transmit）的默认设备。

半虚拟化驱动可利用它来启用一条低延迟的替代数据路径。它还支持在 VF 被拔出时故障转移到半虚拟化数据路径，从而实现由虚拟机监控器（hypervisor）控制的、对直连 VF 的 VM 进行热迁移。

## virtio-net 加速数据路径：STANDBY 模式


net_failover 以一种透明的方式为启用了 virtio-net 的 VM 提供由 hypervisor 控制的加速数据路径，且对客户机用户空间的改动为零或极小。

为支持这一点，hypervisor 需要在 virtio-net 接口上启用 VIRTIO_NET_F_STANDBY 特性，并为 virtio-net 与 VF 接口分配相同的 MAC 地址。

下面是一个展示此类配置的 libvirt XML 片段：
```

  <interface type='network'>
    <mac address='52:54:00:00:12:53'/>
    <source network='enp66s0f0_br'/>
    <target dev='tap01'/>
    <model type='virtio'/>
    <driver name='vhost' queues='4'/>
    <link state='down'/>
    <teaming type='persistent'/>
    <alias name='ua-backup0'/>
  </interface>
  <interface type='hostdev' managed='yes'>
    <mac address='52:54:00:00:12:53'/>
    <source>
      <address type='pci' domain='0x0000' bus='0x42' slot='0x02' function='0x5'/>
    </source>
    <teaming type='transient' persistent='ua-backup0'/>
  </interface>

```
在此配置中，第一个设备定义用于 virtio-net 接口，它充当 'persistent'（持久）设备，表示该接口将始终处于插入状态。这由 'teaming' 标签指定，其必需的 type 属性取值为 'persistent'。virtio-net 设备的链路状态被设为 'down'，以确保 'failover' netdev 在正常通信时优先选用 VF 直通设备。virtio-net 设备会在热迁移期间被置为 UP，以保证通信不中断。

第二个设备定义用于 VF 直通接口。此处 'teaming' 标签的 type 为 'transient'，表示该设备可能会周期性地被拔出。还提供了第二个属性 'persistent'，它指向为 virtio-net 设备声明的别名（alias）。

使用上述配置启动 VM 后，会在 VM 内创建出以下 3 个接口：
```

  4: ens10: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
      link/ether 52:54:00:00:12:53 brd ff:ff:ff:ff:ff:ff
      inet 192.168.12.53/24 brd 192.168.12.255 scope global dynamic ens10
         valid_lft 42482sec preferred_lft 42482sec
      inet6 fe80::97d8:db2:8c10:b6d6/64 scope link
         valid_lft forever preferred_lft forever
  5: ens10nsby: <BROADCAST,MULTICAST> mtu 1500 qdisc fq_codel master ens10 state DOWN group default qlen 1000
      link/ether 52:54:00:00:12:53 brd ff:ff:ff:ff:ff:ff
  7: ens11: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc mq master ens10 state UP group default qlen 1000
      link/ether 52:54:00:00:12:53 brd ff:ff:ff:ff:ff:ff

```
此处，ens10 是 'failover' 主接口，ens10nsby 是从 'standby' virtio-net 接口，ens11 是从 'primary' VF 直通接口。

需要注意一点：某些用户空间网络配置守护进程（如 systemd-networkd、ifupdown 等）无法识别 'net_failover' 设备；在首次启动时，VM 可能导致 'failover' 设备与 VF 都从 DHCP 服务器获取 IP 地址（相同或不同）。这将导致无法连接到 VM。因此可能需要对这些网络配置守护进程做一些调整，以确保 IP 仅从 'failover' 设备获取。

以下是在 'cloud-ifupdown-helper' 脚本中使用的补丁片段：
```

  @@ -27,6 +27,8 @@ do_setup() {
       local working="$cfgdir/.$INTERFACE"
       local final="$cfgdir/$INTERFACE"

  +    if [ -d "/sys/class/net/${INTERFACE}/master" ]; then exit 0; fi
  +
       if ifup --no-act "$INTERFACE" > /dev/null 2>&1; then
           # interface is already known to ifupdown, no need to generate cfg
           log "Skipping configuration generation for $INTERFACE"


```
## 在 STANDBY 模式下对带有 SR-IOV VF 与 virtio-net 的 VM 进行热迁移


net_failover 还支持对直连 SR-IOV VF 设备的 VM 进行由 hypervisor 控制的热迁移：当 VF 被拔出时，自动故障转移到半虚拟化数据路径。

下面是一个示例脚本，展示了从源 hypervisor 发起热迁移的步骤。注意：假设该 VM 连接到一个软件桥 'br0'，其上除连接 VM 的 vnet 设备外，还挂载了一个 VF。这个 VF 并不是直通给 VM 的那个（见 vf.xml 文件）。
```

  # cat vf.xml
  <interface type='hostdev' managed='yes'>
    <mac address='52:54:00:00:12:53'/>
    <source>
      <address type='pci' domain='0x0000' bus='0x42' slot='0x02' function='0x5'/>
    </source>
    <teaming type='transient' persistent='ua-backup0'/>
  </interface>

  # Source Hypervisor migrate.sh
  #!/bin/bash

  DOMAIN=vm-01
  PF=ens6np0
  VF=ens6v1             # VF attached to the bridge.
  VF_NUM=1
  TAP_IF=vmtap01        # virtio-net interface in the VM.
  VF_XML=vf.xml

  MAC=52:54:00:00:12:53
  ZERO_MAC=00:00:00:00:00:00

  # Set the virtio-net interface up.
  virsh domif-setlink $DOMAIN $TAP_IF up

  # Remove the VF that was passthrough'd to the VM.
  virsh detach-device --live --config $DOMAIN $VF_XML

  ip link set $PF vf $VF_NUM mac $ZERO_MAC

  # Add FDB entry for traffic to continue going to the VM via
  # the VF -> br0 -> vnet interface path.
  bridge fdb add $MAC dev $VF
  bridge fdb add $MAC dev $TAP_IF master

  # Migrate the VM
  virsh migrate --live --persistent $DOMAIN qemu+ssh://$REMOTE_HOST/system

  # Clean up FDB entries after migration completes.
  bridge fdb del $MAC dev $VF
  bridge fdb del $MAC dev $TAP_IF master

```
在目的 hypervisor 上，会在迁移开始前创建一个共享桥 'br0'，并将来自目的 PF 的一个 VF 加入该桥。类似地，还会添加一条合适的 FDB 表项。

迁移完成后，会在目的 hypervisor 上执行以下脚本，它会将 VF 重新挂载到 VM 并关闭 virtio-net
```

  # reattach-vf.sh
  #!/bin/bash

  bridge fdb del 52:54:00:00:12:53 dev ens36v0
  bridge fdb del 52:54:00:00:12:53 dev vmtap01 master
  virsh attach-device --config --live vm01 vf.xml
  virsh domif-setlink vm01 vmtap01 down

```
