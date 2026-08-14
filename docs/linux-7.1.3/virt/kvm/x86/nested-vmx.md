
## Nested VMX锛堝祵濂?VMX锛?


### 姒傝堪

鍦?Intel 澶勭悊鍣ㄤ笂锛孠VM 鍒╃敤 Intel 鐨?VMX锛圴irtual-Machine eXtensions锛岃櫄鎷熸満鎵╁睍锛夋潵杞绘澗銆侀珮鏁堝湴杩愯瀹㈡埛鏈烘搷浣滅郴缁熴€傞€氬父鎯呭喌涓嬶紝瀹㈡埛鏈?*涓嶈兘**鑷韩浣滀负 hypervisor 鍐嶅幓杩愯鑷繁鐨勫鎴锋満锛屽洜涓哄鎴锋満鏃犳硶浣跨敤 VMX 鎸囦护銆?

"Nested VMX"锛堝祵濂?VMX锛夌壒鎬у～琛ヤ簡杩欎竴缂哄け鐨勮兘鍔涒€斺€斿畠鍏佽瀹㈡埛鏈?hypervisor锛堜娇鐢?VMX锛夊啀鍘昏繍琛屽祵濂楃殑瀹㈡埛鏈恒€傞€氳繃鍏佽瀹㈡埛鏈轰娇鐢?VMX 鎸囦护锛屽苟浠ュ崟绾?VMX 鍙敤鐨勭‖浠舵纭湴銆侀珮鏁堝湴妯℃嫙锛屾潵瀹炵幇杩欎竴鐐广€?

鍏充簬宓屽 VMX 鐗规€ц儗鍚庣殑鐞嗚銆佸疄鐜板強鎬ц兘鐗瑰緛鐨勬洿璇︾粏鎻忚堪锛屽彲鍙傝€?OSDI 2010 璁烘枃 "The Turtles Project: Design and Implementation of Nested Virtualization"锛屽湴鍧€濡備笅锛?

https://www.usenix.org/events/osdi10/tech/full_papers/Ben-Yehuda.pdf


### Terminology锛堟湳璇級

鍗曠骇铏氭嫙鍖栧寘鍚袱涓眰绾р€斺€攈ost锛圞VM锛変笌瀹㈡埛鏈猴紙guests锛夈€傚湪宓屽铏氭嫙鍖栦腑锛屽垯鏈変笁涓眰绾э細host锛圞VM锛岀О涓?L0锛夈€乬uest hypervisor锛堢О涓?L1锛夈€佸祵濂楃殑瀹㈡埛鏈猴紙绉颁负 L2锛夈€?


### 杩愯 nested VMX

鑷?Linux 鍐呮牳 v4.20 璧凤紝nested VMX 鐗规€ч粯璁ゅ惎鐢ㄣ€傚湪鏇存棭鐨?Linux 鍐呮牳涓婏紝鍙€氳繃缁?kvm-intel 妯″潡浼犻€?"nested=1" 閫夐」鏉ュ惎鐢ㄣ€?

杩欎笉闇€瑕佸鐢ㄦ埛绌洪棿锛坬emu锛夊仛浠讳綍淇敼銆備笉杩囷紝qemu 榛樿妯℃嫙鐨?CPU 绫诲瀷锛坬emu64锛夊苟鏈垪鍑?"VMX" CPU 鐗规€э紝鍥犳闇€瑕佹樉寮忓惎鐢紝鍙€氳繃浠ヤ笅浠讳竴绉?qemu 閫夐」锛?

- 澶勭悊鍣?host锛堜娇妯℃嫙鐨?CPU 鐗规€х瓑鍚屼簬鐪熷疄 CPU锛?

- 澶勭悊鍣?qemu64,+vmx锛堝湪宸插懡鍚嶇殑 CPU 绫诲瀷涓婁粎娣诲姞 vmx 鐗规€э級


### ABIs

Nested VMX 鐨勭洰鏍囨槸锛堟渶缁堬級鍚?guest hypervisor 鍛堢幇涓€涓爣鍑嗐€佸畬鍏ㄥ彲鐢ㄧ殑 VMX 瀹炵幇銆傚洜姝わ紝瀹樻柟瑙勮寖鐨?ABI 鐢?Intel 鐨?VMX 瑙勮寖鎻愪緵锛屽嵆銆奍ntel 64 涓?IA-32 浣撶郴缁撴瀯杞欢寮€鍙戣€呮墜鍐屻€嬬 3B 鍗枫€傜洰鍓?VMX 鐨勬煇浜涚壒鎬у凡琚畬鍏ㄦ敮鎸侊紝鐩爣鏄渶缁堟敮鎸佸叏閮ㄧ壒鎬э紝浼樺厛鏀寔閭ｄ簺鍦ㄥ疄璺典腑娴佽鐨?hypervisor锛圞VM 鍙婂叾瀹冿級鎵€鐢ㄧ殑 VMX 鐗规€с€?

鍦?VMX 鐨勫疄鐜颁腑锛宯ested VMX 鍚?L1 鍛堢幇 VMCS 缁撴瀯浣撱€傛寜瑙勮寖瑕佹眰锛屽叾涓袱涓瓧娈?`revision_id` 涓?`abort` 鏄敤鎴峰彲瑙佺殑锛涜缁撴瀯浣撳鐢ㄦ埛鑰岃█鏄?*涓嶉€忔槑**锛坥paque锛夌殑锛岀敤鎴蜂笉搴斿叧蹇冨唴閮ㄧ粨鏋勶紝鑰屽簲閫氳繃 VMREAD 涓?VMWRITE 鎸囦护鏉ヨ闂€?

涓嶈繃锛屽嚭浜庤皟璇曠洰鐨勶紝KVM 寮€鍙戣€呭彲鑳芥湁鍏磋叮浜嗚В璇ョ粨鏋勪綋鐨勫唴閮ㄣ€傝缁撴瀯浣?`vmcs12` 瀹氫箟鍦?`arch/x86/kvm/vmx.c` 涓€?

鍚嶇О "vmcs12" 鎸囩敱 L1 涓?L2 鏋勫缓鐨?VMCS锛涗唬鐮佷腑鐨?"vmcs01" 鎸囩敱 L0 涓?L1 鏋勫缓鐨?VMCS锛?vmcs02" 鎸囩敱 L0 瀹為檯涓鸿繍琛屼腑鐨?L2 鏋勫缓鐨?VMCS鈥斺€旇繖鍦ㄥ墠杩拌鏂囦腑鏈夎缁嗚В閲娿€?

涓烘柟渚胯捣瑙侊紝杩欓噷澶嶈堪 `vmcs12` 缁撴瀯浣撶殑鍐呭銆傝嫢鍏跺唴閮ㄥ彂鐢熷彉鍖栵紝灏嗙牬鍧忚法 KVM 鐗堟湰鐨勫疄鏃惰縼绉伙紙live migration锛夈€傚彧鏈夊綋 `vmcs12` 鐨勫唴閮ㄧ粨鏋勪綋鎴?`shadow_vmcs` 鍙戠敓鍙樺寲鏃讹紝鎵嶉渶瑕佷慨鏀?`VMCS12_REVISION`锛堜綅浜?vmx.c 涓級銆?

```
	typedef u64 natural_width;
	struct __packed vmcs12 {
		/* According to the Intel spec, a VMCS region must start with
		 * these two user-visible fields */
		u32 revision_id;
		u32 abort;

		u32 launch_state; /* set to 0 by VMCLEAR, to 1 by VMLAUNCH */
		u32 padding[7]; /* room for future expansion */

		u64 io_bitmap_a;
		u64 io_bitmap_b;
		u64 msr_bitmap;
		u64 vm_exit_msr_store_addr;
		u64 vm_exit_msr_load_addr;
		u64 vm_entry_msr_load_addr;
		u64 tsc_offset;
		u64 virtual_apic_page_addr;
		u64 apic_access_addr;
		u64 ept_pointer;
		u64 guest_physical_address;
		u64 vmcs_link_pointer;
		u64 guest_ia32_debugctl;
		u64 guest_ia32_pat;
		u64 guest_ia32_efer;
		u64 guest_pdptr0;
		u64 guest_pdptr1;
		u64 guest_pdptr2;
		u64 guest_pdptr3;
		u64 host_ia32_pat;
		u64 host_ia32_efer;
		u64 padding64[8]; /* room for future expansion */
		natural_width cr0_guest_host_mask;
		natural_width cr4_guest_host_mask;
		natural_width cr0_read_shadow;
		natural_width cr4_read_shadow;
		natural_width dead_space[4]; /* Last remnants of cr3_target_value[0-3]. */
		natural_width exit_qualification;
		natural_width guest_linear_address;
		natural_width guest_cr0;
		natural_width guest_cr3;
		natural_width guest_cr4;
		natural_width guest_es_base;
		natural_width guest_cs_base;
		natural_width guest_ss_base;
		natural_width guest_ds_base;
		natural_width guest_fs_base;
		natural_width guest_gs_base;
		natural_width guest_ldtr_base;
		natural_width guest_tr_base;
		natural_width guest_gdtr_base;
		natural_width guest_idtr_base;
		natural_width guest_dr7;
		natural_width guest_rsp;
		natural_width guest_rip;
		natural_width guest_rflags;
		natural_width guest_pending_dbg_exceptions;
		natural_width guest_sysenter_esp;
		natural_width guest_sysenter_eip;
		natural_width host_cr0;
		natural_width host_cr3;
		natural_width host_cr4;
		natural_width host_fs_base;
		natural_width host_gs_base;
		natural_width host_tr_base;
		natural_width host_gdtr_base;
		natural_width host_idtr_base;
		natural_width host_ia32_sysenter_esp;
		natural_width host_ia32_sysenter_eip;
		natural_width host_rsp;
		natural_width host_rip;
		natural_width paddingl[8]; /* room for future expansion */
		u32 pin_based_vm_exec_control;
		u32 cpu_based_vm_exec_control;
		u32 exception_bitmap;
		u32 page_fault_error_code_mask;
		u32 page_fault_error_code_match;
		u32 cr3_target_count;
		u32 vm_exit_controls;
		u32 vm_exit_msr_store_count;
		u32 vm_exit_msr_load_count;
		u32 vm_entry_controls;
		u32 vm_entry_msr_load_count;
		u32 vm_entry_intr_info_field;
		u32 vm_entry_exception_error_code;
		u32 vm_entry_instruction_len;
		u32 tpr_threshold;
		u32 secondary_vm_exec_control;
		u32 vm_instruction_error;
		u32 vm_exit_reason;
		u32 vm_exit_intr_info;
		u32 vm_exit_intr_error_code;
		u32 idt_vectoring_info_field;
		u32 idt_vectoring_error_code;
		u32 vm_exit_instruction_len;
		u32 vmx_instruction_info;
		u32 guest_es_limit;
		u32 guest_cs_limit;
		u32 guest_ss_limit;
		u32 guest_ds_limit;
		u32 guest_fs_limit;
		u32 guest_gs_limit;
		u32 guest_ldtr_limit;
		u32 guest_tr_limit;
		u32 guest_gdtr_limit;
		u32 guest_idtr_limit;
		u32 guest_es_ar_bytes;
		u32 guest_cs_ar_bytes;
		u32 guest_ss_ar_bytes;
		u32 guest_ds_ar_bytes;
		u32 guest_fs_ar_bytes;
		u32 guest_gs_ar_bytes;
		u32 guest_ldtr_ar_bytes;
		u32 guest_tr_ar_bytes;
		u32 guest_interruptibility_info;
		u32 guest_activity_state;
		u32 guest_sysenter_cs;
		u32 host_ia32_sysenter_cs;
		u32 padding32[8]; /* room for future expansion */
		u16 virtual_processor_id;
		u16 guest_es_selector;
		u16 guest_cs_selector;
		u16 guest_ss_selector;
		u16 guest_ds_selector;
		u16 guest_fs_selector;
		u16 guest_gs_selector;
		u16 guest_ldtr_selector;
		u16 guest_tr_selector;
		u16 host_es_selector;
		u16 host_cs_selector;
		u16 host_ss_selector;
		u16 host_ds_selector;
		u16 host_fs_selector;
		u16 host_gs_selector;
		u16 host_tr_selector;
	};
```


### Authors

琛ヤ竵鐢变互涓嬩汉鍛樼紪鍐欙細

- Abel Gordon, abelg < > il.ibm.com
- Nadav Har'El, nyh < > il.ibm.com
- Orit Wasserman, oritw < > il.ibm.com
- Ben-Ami Yassor, benami < > il.ibm.com
- Muli Ben-Yehuda, muli < > il.ibm.com

璐＄尞鑰咃細

- Anthony Liguori, aliguori < > us.ibm.com
- Mike Day, mdday < > us.ibm.com
- Michael Factor, factor < > il.ibm.com
- Zvi Dubitzky, dubi < > il.ibm.com

鏈変环鍊肩殑瀹￠槄锛?

- Avi Kivity, avi < > redhat.com
- Gleb Natapov, gleb < > redhat.com
- Marcelo Tosatti, mtosatti < > redhat.com
- Kevin Tian, kevin.tian < > intel.com
- 浠ュ強鍏朵粬浜恒€?
