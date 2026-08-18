
## NET_FAILOVER


## 姒傝堪


net_failover 椹卞姩閫氳繃 API 鎻愪緵鑷姩鏁呴殰杞Щ鏈哄埗锛岀敤浜庡垱寤哄拰閿€姣佷竴涓晠闅滆浆绉讳富缃戠粶璁惧锛坣etdev锛夛紝骞剁鐞嗛€氳繃閫氱敤鏁呴殰杞Щ鍩虹璁炬柦娉ㄥ唽鐨勪富锛坧rimary锛変笌澶囩敤锛坰tandby锛変粠缃戠粶璁惧锛坰lave netdev锛夈€?
鏁呴殰杞Щ netdev 鍏呭綋涓昏澶囷紝鎺у埗 2 涓粠璁惧銆傚師濮嬬殑鍗婅櫄鎷熷寲鎺ュ彛琚敞鍐屼负 'standby' 浠?netdev锛岃€屽叿鏈夌浉鍚?MAC 鐨?passthru/vf 璁惧琚敞鍐屼负 'primary' 浠?netdev銆?standby' 涓?'failover' netdev 閮藉叧鑱斿埌鍚屼竴涓?'pci' 璁惧銆傜敤鎴烽€氳繃 'failover' netdev 璁块棶缃戠粶鎺ュ彛銆傚綋 'primary' netdev 鍙敤涓旈摼璺凡鍚敤骞惰繍琛屾椂锛?failover' netdev 浼氬皢鍏堕€変负鍙戦€侊紙transmit锛夌殑榛樿璁惧銆?
鍗婅櫄鎷熷寲椹卞姩鍙埄鐢ㄥ畠鏉ュ惎鐢ㄤ竴鏉′綆寤惰繜鐨勬浛浠ｆ暟鎹矾寰勩€傚畠杩樻敮鎸佸湪 VF 琚嫈鍑烘椂鏁呴殰杞Щ鍒板崐铏氭嫙鍖栨暟鎹矾寰勶紝浠庤€屽疄鐜扮敱铏氭嫙鏈虹洃鎺у櫒锛坔ypervisor锛夋帶鍒剁殑銆佸鐩磋繛 VF 鐨?VM 杩涜鐑縼绉汇€?
## virtio-net 鍔犻€熸暟鎹矾寰勶細STANDBY 妯″紡


net_failover 浠ヤ竴绉嶉€忔槑鐨勬柟寮忎负鍚敤浜?virtio-net 鐨?VM 鎻愪緵鐢?hypervisor 鎺у埗鐨勫姞閫熸暟鎹矾寰勶紝涓斿瀹㈡埛鏈虹敤鎴风┖闂寸殑鏀瑰姩涓洪浂鎴栨瀬灏忋€?
涓烘敮鎸佽繖涓€鐐癸紝hypervisor 闇€瑕佸湪 virtio-net 鎺ュ彛涓婂惎鐢?VIRTIO_NET_F_STANDBY 鐗规€э紝骞朵负 virtio-net 涓?VF 鎺ュ彛鍒嗛厤鐩稿悓鐨?MAC 鍦板潃銆?
涓嬮潰鏄竴涓睍绀烘绫婚厤缃殑 libvirt XML 鐗囨锛?```

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
鍦ㄦ閰嶇疆涓紝绗竴涓澶囧畾涔夌敤浜?virtio-net 鎺ュ彛锛屽畠鍏呭綋 'persistent'锛堟寔涔咃級璁惧锛岃〃绀鸿鎺ュ彛灏嗗缁堝浜庢彃鍏ョ姸鎬併€傝繖鐢?'teaming' 鏍囩鎸囧畾锛屽叾蹇呴渶鐨?type 灞炴€у彇鍊间负 'persistent'銆倂irtio-net 璁惧鐨勯摼璺姸鎬佽璁句负 'down'锛屼互纭繚 'failover' netdev 鍦ㄦ甯搁€氫俊鏃朵紭鍏堥€夌敤 VF 鐩撮€氳澶囥€倂irtio-net 璁惧浼氬湪鐑縼绉绘湡闂磋缃负 UP锛屼互淇濊瘉閫氫俊涓嶄腑鏂€?
绗簩涓澶囧畾涔夌敤浜?VF 鐩撮€氭帴鍙ｃ€傛澶?'teaming' 鏍囩鐨?type 涓?'transient'锛岃〃绀鸿璁惧鍙兘浼氬懆鏈熸€у湴琚嫈鍑恒€傝繕鎻愪緵浜嗙浜屼釜灞炴€?'persistent'锛屽畠鎸囧悜涓?virtio-net 璁惧澹版槑鐨勫埆鍚嶏紙alias锛夈€?
浣跨敤涓婅堪閰嶇疆鍚姩 VM 鍚庯紝浼氬湪 VM 鍐呭垱寤哄嚭浠ヤ笅 3 涓帴鍙ｏ細
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
姝ゅ锛宔ns10 鏄?'failover' 涓绘帴鍙ｏ紝ens10nsby 鏄粠 'standby' virtio-net 鎺ュ彛锛宔ns11 鏄粠 'primary' VF 鐩撮€氭帴鍙ｃ€?
闇€瑕佹敞鎰忎竴鐐癸細鏌愪簺鐢ㄦ埛绌洪棿缃戠粶閰嶇疆瀹堟姢杩涚▼锛堝 systemd-networkd銆乮fupdown 绛夛級鏃犳硶璇嗗埆 'net_failover' 璁惧锛涘湪棣栨鍚姩鏃讹紝VM 鍙兘瀵艰嚧 'failover' 璁惧涓?VF 閮戒粠 DHCP 鏈嶅姟鍣ㄨ幏鍙?IP 鍦板潃锛堢浉鍚屾垨涓嶅悓锛夈€傝繖灏嗗鑷存棤娉曡繛鎺ュ埌 VM銆傚洜姝ゅ彲鑳介渶瑕佸杩欎簺缃戠粶閰嶇疆瀹堟姢杩涚▼鍋氫竴浜涜皟鏁达紝浠ョ‘淇?IP 浠呬粠 'failover' 璁惧鑾峰彇銆?
浠ヤ笅鏄湪 'cloud-ifupdown-helper' 鑴氭湰涓娇鐢ㄧ殑琛ヤ竵鐗囨锛?```

  @@ -27,6 +27,8 @@ do_setup() {
       local working="$cfgdir/.$INTERFACE"
       local final="$cfgdir/$INTERFACE"

  +    if [ -d "/sys/class/net/${INTERFACE}/master" ]; then exit 0; fi
  +
       if ifup --no-act "$INTERFACE" > /dev/null 2>&1; then
           # interface is already known to ifupdown, no need to generate cfg
           log "Skipping configuration generation for $INTERFACE"


```
## 鍦?STANDBY 妯″紡涓嬪甯︽湁 SR-IOV VF 涓?virtio-net 鐨?VM 杩涜鐑縼绉?

net_failover 杩樻敮鎸佸鐩磋繛 SR-IOV VF 璁惧鐨?VM 杩涜鐢?hypervisor 鎺у埗鐨勭儹杩佺Щ锛氬綋 VF 琚嫈鍑烘椂锛岃嚜鍔ㄦ晠闅滆浆绉诲埌鍗婅櫄鎷熷寲鏁版嵁璺緞銆?
涓嬮潰鏄竴涓ず渚嬭剼鏈紝灞曠ず浜嗕粠婧?hypervisor 鍙戣捣鐑縼绉荤殑姝ラ銆傛敞鎰忥細鍋囪璇?VM 杩炴帴鍒颁竴涓蒋浠舵ˉ 'br0'锛屽叾涓婇櫎杩炴帴 VM 鐨?vnet 璁惧澶栵紝杩樻寕杞戒簡涓€涓?VF銆傝繖涓?VF 骞朵笉鏄洿閫氱粰 VM 鐨勯偅涓紙瑙?vf.xml 鏂囦欢锛夈€?```

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
鍦ㄧ洰鐨?hypervisor 涓婏紝浼氬湪杩佺Щ寮€濮嬪墠鍒涘缓涓€涓叡浜ˉ 'br0'锛屽苟灏嗘潵鑷洰鐨?PF 鐨勪竴涓?VF 鍔犲叆璇ユˉ銆傜被浼煎湴锛岃繕浼氭坊鍔犱竴鏉″悎閫傜殑 FDB 琛ㄩ」銆?
杩佺Щ瀹屾垚鍚庯紝浼氬湪鐩殑 hypervisor 涓婃墽琛屼互涓嬭剼鏈紝瀹冧細灏?VF 閲嶆柊鎸傝浇鍒?VM 骞跺叧闂?virtio-net
```

  # reattach-vf.sh
  #!/bin/bash

  bridge fdb del 52:54:00:00:12:53 dev ens36v0
  bridge fdb del 52:54:00:00:12:53 dev vmtap01 master
  virsh attach-device --config --live vm01 vf.xml
  virsh domif-setlink vm01 vmtap01 down

```
