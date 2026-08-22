## 前端传统数据类型

本页汇DVB 前端（frontend）传API（基DVB v3）的数据类型与函数调用，内核仅为兼容性保留其支持。文档列fe 类型与读取误码率、信号强度等遗留接口，建议新代码避免使用



- [fe-type-t](fe-type-t)
- [fe-bandwidth-t](fe-bandwidth-t)
- [dvb-frontend-parameters](dvb-frontend-parameters)
- [dvb-frontend-event](dvb-frontend-event)


## 前端传统函数调用


这些函数定义DVB 版本 3。内核出于兼容性原因才保留对其的支持。强烈不建议使用它们


- [fe-read-ber](fe-read-ber)
- [fe-read-snr](fe-read-snr)
- [fe-read-signal-strength](fe-read-signal-strength)
- [fe-read-uncorrected-blocks](fe-read-uncorrected-blocks)
- [fe-set-frontend](fe-set-frontend)
- [fe-get-frontend](fe-get-frontend)
- [fe-get-event](fe-get-event)
- [fe-dishnetwork-send-legacy-cmd](fe-dishnetwork-send-legacy-cmd)

