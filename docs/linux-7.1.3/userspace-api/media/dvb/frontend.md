
# 数字电视前端 API


数字电视前端 API 被设计用于支持三类传输系统：地面、有线和卫星。目前支持以下传输系统：

- 地面系统：DVB-T、DVB-T2、ATSC、ATSC M/H、ISDB-T、DVB-H、
   DTMB、CMMB

- 有线系统：DVB-C Annex A/C、ClearQAM（DVB-C Annex B）

- 卫星系统：DVB-S、DVB-S2、DVB Turbo、ISDB-S、DSS

数字电视前端控制多个子设备，包括：

- 调谐器（Tuner）

- 数字电视解调器

- 低噪声放大器（LNA）

- 卫星设备控制（SEC）[#f1]_。

前端可通过 `/dev/dvb/adapter?/frontend?` 访问。数据类型和 ioctl 定义可通过在应用程序中包含
`linux/dvb/frontend.h` 来使用。


   通过互联网（DVB-IP）和 MMT（MPEG Media Transport）进行的传输
   目前尚未由该 API 处理，但未来可能进行扩展。


   在卫星系统上，该 API 对卫星设备控制（SEC）的支持允许进行电源控制以及发送/接收信号来控制天线子系统，选择极化方式并选择低噪声块转换器馈源喇叭（LNBf）的中频（IF）。它支持 DiSEqC 和 V-SEC 协议。DiSEqC
   （数字 SEC）规范可在
   `Eutelsat <http://www.eutelsat.com/satellites/4_5_5.html>`__ 获取。


- [query-dvb-frontend-info](query-dvb-frontend-info)
- [dvb-fe-read-status](dvb-fe-read-status)
- [dvbproperty](dvbproperty)
- [frontend_fcalls](frontend_fcalls)
