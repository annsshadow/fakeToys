


######## 属性类

要调谐到一个数字电视（Digital TV）物理频道并开始解码，需要改变一组参数，以控调谐器（tuner）、解调器（demodulator）、线性低噪声放大器（LNA），并通过卫星设备
控制（Satellite Equipment Control，SEC，用于卫星系统）来设置天线子系统。具体的
参数因每种数字电视标准而异，并且可能随着数字电视规范的演进而变化
过去（直DVB API 3 版——DVBv3），所使用的策略是提供一union，将调谐
DVB-S、DVB-C、DVB-T ATSC 传输系统所需的参数归在一起。问题在于，随着第二标准的出现，这样一union 的大小不足以容纳那些新标准所需struct。而且，扩展它
会破坏用户空间
因此，基于旧 union/struct 的方法被弃用，转而采用基于属性集（properties set）的
方法。在这种方法中，使用 FE_GET_PROPERTY FE_SET_PROPERTY <FE_GET_PROPERTY> 设置前端（frontend）并读取其状态
具体的操作由一dtv_property cmd/data 对来决定。通过一ioctl，最多可获取/设置 64 个属性
本节描述了设置前端的新推荐方式，它支持所有的数字电视传输系统

   1. Linux DVB API 3 版中，设置前端是通过 struct `dvb_frontend_parameters`
      完成的
   2. 不要在支持新标准的硬件上使用 DVB API 3 版调用。该 API 对新标准或新硬件
      不提供或仅提供非常有限的支持
   3. 如今，大多数前端支持多种传输系统。只有使DVB API 5 版调用，才能在前      支持的多种传输系统之间切换
   4. DVB API 5 版也称为 **S2API**，因为加入它的第一个新标准DVB-S2
**示例**：为了将硬件设置为调谐到 651 kHz DVB-C 频道，采256-QAM 调制、FEC 3/4
以及 5.217 Mbauds 的符号率，应将以下属性发送给 FE_SET_PROPERTY <FE_GET_PROPERTY>
ioctl锛。
  DTV_DELIVERY_SYSTEM <DTV-DELIVERY-SYSTEM> = SYS_DVBC_ANNEX_A

  DTV_FREQUENCY <DTV-FREQUENCY> = 651000000

  DTV_MODULATION <DTV-MODULATION> = QAM_256

  DTV_INVERSION <DTV-INVERSION> = INVERSION_AUTO

  DTV_SYMBOL_RATE <DTV-SYMBOL-RATE> = 5217000

  DTV_INNER_FEC <DTV-INNER-FEC> = FEC_3_4

  DTV_TUNE <DTV-TUNE>

实现上述功能的代码展示在 dtv-prop-example 中
    :caption: 示例：设置数字电视前端属    :name: dtv-prop-example

    #include <stdio.h>
    #include <fcntl.h>
    #include <sys/ioctl.h>
    #include <linux/dvb/frontend.h>

    static struct dtv_property props[] = {
	{ .cmd = DTV_DELIVERY_SYSTEM, .u.data = SYS_DVBC_ANNEX_A },
	{ .cmd = DTV_FREQUENCY,       .u.data = 651000000 },
	{ .cmd = DTV_MODULATION,      .u.data = QAM_256 },
	{ .cmd = DTV_INVERSION,       .u.data = INVERSION_AUTO },
	{ .cmd = DTV_SYMBOL_RATE,     .u.data = 5217000 },
	{ .cmd = DTV_INNER_FEC,       .u.data = FEC_3_4 },
	{ .cmd = DTV_TUNE }
    };

    static struct dtv_properties dtv_prop = {
	.num = 6, .props = props
    };

    int main(void)
    {
	int fd = open("/dev/dvb/adapter0/frontend0", O_RDWR);

	if (!fd) {
	    perror ("open");
	    return -1;
	}
	if (ioctl(fd, FE_SET_PROPERTY, &dtv_prop) == -1) {
	    perror("ioctl");
	    return -1;
	}
	printf("Frontend set\\n");
	return 0;
    }

   上述示例强烈建议使用 `libdvbv5 <https://linuxtv.org/docs/libdvbv5/index.html>`__因为它提供了使用所支持数字电视标准的抽象，并提供了用于常规操作（如节目扫描以及
写频道描述符文件）的方法
- [fe_property_parameters](fe_property_parameters)
- [frontend-stat-properties](frontend-stat-properties)
- [frontend-property-terrestrial-systems](frontend-property-terrestrial-systems)
- [frontend-property-cable-systems](frontend-property-cable-systems)
- [frontend-property-satellite-systems](frontend-property-satellite-systems)
- [frontend-header](frontend-header)
