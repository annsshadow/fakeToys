
## 铏氭嫙鍙墿灞曞眬鍩熺綉锛圴XLAN锛夋枃妗?

VXLAN 鍗忚鏄竴绉嶉毀閬撳崗璁紝鏃ㄥ湪瑙ｅ喅 IEEE 802.1q 涓?VLAN ID 鏁伴噺鏈夐檺锛?096 涓級鐨勯棶棰樸€傚€熷姪 VXLAN锛屾爣璇嗙鐨勯暱搴︽墿灞曞埌 24 浣嶏紙16777216 涓級銆?
VXLAN 鐢?IETF RFC 7348 鎻忚堪锛屽苟宸茶澶氬鍘傚晢瀹炵幇銆傝鍗忚杩愯鍦?UDP 涔嬩笂锛屼娇鐢ㄥ崟涓€鐩殑绔彛銆傛湰鏂囨。鎻忚堪鐨勬槸 Linux 鍐呮牳鐨勯毀閬撹澶囷紝姝ゅ Openvswitch 涔熸湁涓€浠界嫭绔嬬殑 VXLAN 瀹炵幇銆?
涓庡ぇ澶氭暟闅ч亾涓嶅悓锛孷XLAN 鏄竴涓?1 瀵?N 鐨勭綉缁滐紝鑰屼笉浠呬粎鏄竴瀵逛竴鐐瑰鐐广€俈XLAN 璁惧鏃㈠彲浠ュ儚瀛︿範妗ラ偅鏍峰姩鎬佸湴瀛︿範瀵圭 IP 鍦板潃锛屼篃鍙互浣跨敤闈欐€侀厤缃殑杞彂琛ㄩ」銆?
vxlan 鐨勭鐞嗘柟寮忎笌鍏朵袱涓渶鎺ヨ繎鐨勯偦灞?GRE 鍜?VLAN 绫讳技銆傞厤缃?VXLAN 闇€瑕佷娇鐢ㄤ笌 VXLAN 棣栨鍚堝叆涓婃父鏃跺唴鏍哥増鏈浉鍖归厤鐨?iproute2 鐗堟湰銆?
```

    # ip link add vxlan0 type vxlan id 42 group 239.1.1.1 dev eth1 dstport 4789

```
杩欎細鍒涘缓涓€涓悕涓?vxlan0 鐨勬柊璁惧銆傝璁惧閫氳繃 eth1 涓婄殑缁勬挱缁?239.1.1.1 鏉ュ鐞嗚浆鍙戣〃涓病鏈夎〃椤瑰搴旂殑娴侀噺銆傜洰鐨勭鍙ｅ彿琚涓?IANA 鍒嗛厤鐨?4789銆侺inux 鐨?VXLAN 瀹炵幇鏃╀簬 IANA 閫夊畾鏍囧噯鐩殑绔彛鍙凤紝涓轰繚鎸佸悜鍚庡吋瀹癸紝榛樿浣跨敤 Linux 閫夊畾鐨勫€笺€?
```

    # ip link delete vxlan0

```
```

    # ip -d link show vxlan0

```
鍙互浣跨敤鏂扮殑 bridge 鍛戒护鏉ュ垱寤恒€侀攢姣佸拰鏄剧ず vxlan 杞彂琛ㄣ€?
```

    # bridge fdb add to 00:17:42:8a:b4:05 dst 192.19.0.2 dev vxlan0

```
```

    # bridge fdb delete 00:17:42:8a:b4:05 dev vxlan0

```
```

    # bridge fdb show dev vxlan0

```
浠ヤ笅 NIC 鐗规€у彲鑳芥剰鍛崇潃瀵?UDP 闅ч亾鐩稿叧鍗歌浇鐨勬敮鎸侊紙鏈€甯歌鐨勬槸 VXLAN 鐗规€э紝浣嗗鐗瑰畾灏佽鍗忚鐨勬敮鎸佸彇鍐充簬鍏蜂綋 NIC锛夛細

 - `tx-udp_tnl-segmentation`
 - `tx-udp_tnl-csum-segmentation`
    瀵?UDP 灏佽甯ф墽琛?TCP 鍒嗘鍗歌浇鐨勮兘鍔?
 - `rx-udp_tunnel-port-offload`
    瀵?UDP 灏佽甯х殑鎺ユ敹绔В鏋愶紝浣?NIC 鑳藉鎵ц鍗忚鎰熺煡鐨勫嵏杞斤紝渚嬪鍐呭眰甯х殑鏍￠獙鍜岄獙璇佸嵏杞斤紙浠呭湪娌℃湁鍗忚鏃犲叧鍗歌浇鐨?NIC 涓婃墠闇€瑕侊級

瀵逛簬鏀寔 `rx-udp_tunnel-port-offload` 鐨勮澶囷紝褰撳墠鍒楄〃鍙互
```

  $ ethtool --show-tunnels eth0
  Tunnel information for eth0:
    UDP port table 0:
      Size: 4
      Types: vxlan
      No entries
    UDP port table 1:
      Size: 4
      Types: geneve, vxlan-gpe
      Entries (1):
          port 1230, vxlan-gpe

```
