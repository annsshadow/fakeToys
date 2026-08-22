:orphan:


## mac80211_hwsim - 用于 mac80211 802.11 无线设备软件模拟

:Copyright: |copy| 2008, Jouni Malinen <j@w1.fi>

本程序是自由软件；你可以在自由软件基金会发布GNU 通用公共许可证第 2 条款下重新分发和/或修改它

## 简

mac80211_hwsim 是一Linux 内核模块，可用于mac80211 模拟任意数量IEEE 802.11 无线设备。它可以用来测试 mac80211 的大部分功能以及用户空间工具（例hostapd wpa_supplicant），其方式与使用真实 WLAN 硬件的普通情况非常接近。从 mac80211 的角度看，mac80211_hwsim 只不过是另一个硬件驱动，即使用这个测试工具不需要对 mac80211 做任何修改
mac80211_hwsim 的主要目标是让开发人员更容易测试他们的代码，并处mac80211、hostapd wpa_supplicant 的新特性。模拟的无线设备没有真实硬件的限制，因此可以轻松生成任意的测试环境，并为将来的测试始终复现相同的环境。此外，由于所有无线操作都是模拟的，测试中可以使用任何信道，而不受监管规则的约束
mac80211_hwsim 内核模块有一个参'radios'，可用于选择要模拟多少个无线设备（默2）。这样就可以配置非常简单的小环境（例如，仅一个接入点和一个站点），或大规模的测试（带有数百个站点的多个接入点）
mac80211_hwsim 通过跟踪每个虚拟无线设备的当前信道，并将所有发送的帧复制到当前已启用且与发送方处于同一信道的其他无线设备来工作。mac80211 中的软件加密被使用，以便帧在虚拟空中接口上被实际加密，从而允许对加密进行更完整的测试
一个全局的监控网络设备或 hwsim#，独立于 mac80211 创建。该接口可用于监控所有发送的帧，而不受信道限制

## 简单示

本示例展示了如何使用 mac80211_hwsim 模拟两个无线设备：一个充当接入点，另一个充当与AP 关联的站点。hostapd wpa_supplicant 负责处理 WPA2-PSK 认证。此外，hostapd 还处理关联中接入点侧的部分
```

    # mac80211_hwsim 编译进内核配
    # 加载模块
    modprobe mac80211_hwsim

    # wlan0 运行 hostapd（AP    hostapd hostapd.conf

    # wlan1 运行 wpa_supplicant（站点）
    wpa_supplicant -Dnl80211 -iwlan1 -c wpa_supplicant.conf


```

更多测试用例可在 hostap.git 中找到：
git://w1.fi/srv/git/hostap.git 以及 mac80211_hwsim/tests 子目(http://w1.fi/gitweb/gitweb.cgip=hostap.git;a=tree;f=mac80211_hwsim/tests)
