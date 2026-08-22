


######## 前端参数


传递给前端设备进行调谐的参数种类取决于你使用的硬件种类

结构`dvb_frontend_parameters` 使用一union 来保存特定于各系统的参数。然而，由于较新的传输系统需要更多数据，该结构体大小不足以容纳，而仅仅扩展其大小会破坏现有的应用程序。因此，这些参数被替换为使用
FE_GET_PROPERTY/FE_SET_PROPERTY <FE_GET_PROPERTY>
ioctl。新API 足够灵活，可以向现有传输系统添加新参数，也可以添加更新的传输系统

因此，较新的应用程序应当改用
FE_GET_PROPERTY/FE_SET_PROPERTY <FE_GET_PROPERTY>
，以便能够支持较新的传输系统，如 DVB-S2、DVB-T2、DVB-C2、ISDB 等

所有种类的参数`dvb_frontend_parameters` 结构体中组合为一union



    struct dvb_frontend_parameters {
	uint32_t frequency;     /** (绝对) 频率，QAM/OFDM 单位Hz **/
		    /** QPSK 单位kHz 的中**/
	fe_spectral_inversion_t inversion;
	union {
	    struct dvb_qpsk_parameters qpsk;
	    struct dvb_qam_parameters  qam;
	    struct dvb_ofdm_parameters ofdm;
	    struct dvb_vsb_parameters  vsb;
	} u;
    };

对于 QPSK 前端，`frequency` 字段指定中频，即实际加到 LNB 本振频率（LOF）上的偏移。中频必须以 kHz 为单位指定。对QAM OFDM 前端，`frequency` 指定绝对频率，以 Hz 给出



## QPSK 参数


对于卫星 QPSK 前端，你必须使用 `dvb_qpsk_parameters` 结构体：



     struct dvb_qpsk_parameters {
	 uint32_t        symbol_rate;  /** 符号率，单位：符**/
	 fe_code_rate_t  fec_inner;    /** 前向纠错（见上文**/
     };



## QAM 参数


对于有线 QAM 前端，你使用 `dvb_qam_parameters` 结构体：



     struct dvb_qam_parameters {
	 uint32_t         symbol_rate; /** 符号率，单位：符**/
	 fe_code_rate_t   fec_inner;   /** 前向纠错（见上文**/
	 fe_modulation_t  modulation;  /** 调制类型（见上文**/
     };



## VSB 参数


ATSC 前端`dvb_vsb_parameters` 结构体支持：



    struct dvb_vsb_parameters {
	fe_modulation_t modulation; /** 调制类型（见上文**/
    };



## OFDM 参数


DVB-T 前端`dvb_ofdm_parameters` 结构体支持：



     struct dvb_ofdm_parameters {
	 fe_bandwidth_t      bandwidth;
	 fe_code_rate_t      code_rate_HP;  /** 高优先级流码**/
	 fe_code_rate_t      code_rate_LP;  /** 低优先级流码**/
	 fe_modulation_t     constellation; /** 调制类型（见上文**/
	 fe_transmit_mode_t  transmission_mode;
	 fe_guard_interval_t guard_interval;
	 fe_hierarchy_t      hierarchy_information;
     };
