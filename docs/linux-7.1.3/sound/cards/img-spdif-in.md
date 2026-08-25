## Imagination Technologies SPDIF 杈撳叆鎺у埗鍣。

Imagination Technologies SPDIF 输入控制器包含以下控件：

- name='IEC958 Capture Mask',index=0

此控件返回一个掩码，显示哪些 IEC958 状态位可以使用 'IEC958 Capture Default' 控件读取
- name='IEC958 Capture Default',index=0

此控件返回正在接收的 SPDIF 流中所包含的状态位IEC958 Capture Mask' 显示可以从该控件读取哪些位
- name='SPDIF In Multi Frequency Acquire',index=0
- name='SPDIF In Multi Frequency Acquire',index=1
- name='SPDIF In Multi Frequency Acquire',index=2
- name='SPDIF In Multi Frequency Acquire',index=3

此控件用于尝试获取最多四种不同的采样率。活动速率可通过读取 'SPDIF In Lock Frequency' 控件获得
当此控件的值被设为 {0,0,0,0} 时，提供hw_params 的速率将决定该模块捕获的单一速率。否则，提供hw_params 的速率将被忽略，该模块将尝试捕获此处设置的四种采样率中的每一个
如果需要的速率少于四种，可以多次指定相同的速率

- name='SPDIF In Lock Frequency',index=0

此控件返回活动捕获速率，如果尚未获取锁定则返回 0

- name='SPDIF In Lock TRK',index=0

此控件用于修改该模块的锁抖动抑制特性。较大的值会扩大锁定范围，但降低抖动抑制能力
- name='SPDIF In Lock Acquire Threshold',index=0

此控件用于更改获取锁定时所需的阈值
- name='SPDIF In Lock Release Threshold',index=0

此控件用于更改释放锁定时所需的阈值