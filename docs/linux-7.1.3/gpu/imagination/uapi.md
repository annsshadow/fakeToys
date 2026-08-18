## UAPI

鏈枃妗ｆ弿杩?Imagination PowerVR GPU 鐨勭敤鎴风┖闂?API锛圲API锛夛紝娑电洊瀵硅薄鏁扮粍銆両OCTL 鎺ュ彛锛堝璁惧鏌ヨ銆佺紦鍐插尯瀵硅薄鍒涘缓绛夛級鍙婂叾鏁版嵁缁撴瀯锛屼富瑕佷緵鐢ㄦ埛绌洪棿鍥惧舰鏍堜笌椹卞姩寮€鍙戣€呭弬鑰冦€?

鏈枃妗ｄ负鑷姩鐢熸垚鐨勭粨鏋勫寲鏉＄洰绱㈠紩锛屾潯鐩悕绉颁互鑻辨枃鏈淇濈暀浠ヤ究妫€绱€?

sources associated 绔犺妭 found`pvr_drm.h`.

:doc:PowerVR UAPI

## OBJECT ARRAYS

:identifiers:drm_pvr_obj_array

:identifiers:DRM_PVR_OBJ_ARRAY

## IOCTLS

:doc:PowerVR IOCTL 鎺ュ彛

:identifiers:PVR_IOCTL

### DEV_QUERY

:doc:PowerVR IOCTL DEV_QUERY 鎺ュ彛

:identifiers:drm_pvr_dev_query

:identifiers:drm_pvr_ioctl_dev_query_args

:identifiers:drm_pvr_dev_query_gpu_info
drm_pvr_dev_query_runtime_info
drm_pvr_dev_query_hwrt_info
drm_pvr_dev_query_quirks
drm_pvr_dev_query_enhancements

:identifiers:drm_pvr_heap_id
drm_pvr_heap
drm_pvr_dev_query_heap_info

:identifiers:drm_pvr_static_data_area_usage
drm_pvr_static_data_area
drm_pvr_dev_query_static_data_areas

### CREATE_BO

:doc:PowerVR IOCTL CREATE_BO 鎺ュ彛

:identifiers:drm_pvr_ioctl_create_bo_args

:doc:鏍囧織 CREATE_BO

### GET_BO_MMAP_OFFSET

:doc:PowerVR IOCTL GET_BO_MMAP_OFFSET 鎺ュ彛

:identifiers:drm_pvr_ioctl_get_bo_mmap_offset_args

### CREATE_VM_CONTEXT DESTROY_VM_CONTEXT

:doc:PowerVR IOCTL CREATE_VM_CONTEXT DESTROY_VM_CONTEXT interfaces

:identifiers:drm_pvr_ioctl_create_vm_context_args
drm_pvr_ioctl_destroy_vm_context_args

### VM_MAP VM_UNMAP

:doc:PowerVR IOCTL VM_MAP VM_UNMAP interfaces

:identifiers:drm_pvr_ioctl_vm_map_args
drm_pvr_ioctl_vm_unmap_args

### CREATE_CONTEXT DESTROY_CONTEXT

:doc:PowerVR IOCTL CREATE_CONTEXT DESTROY_CONTEXT interfaces

:identifiers:drm_pvr_ioctl_create_context_args

:identifiers:drm_pvr_ctx_priority
drm_pvr_ctx_type
drm_pvr_static_render_context_state
drm_pvr_static_render_context_state_format
drm_pvr_reset_framework
drm_pvr_reset_framework_format

:identifiers:drm_pvr_ioctl_destroy_context_args

### CREATE_FREE_LIST DESTROY_FREE_LIST

:doc:PowerVR IOCTL CREATE_FREE_LIST DESTROY_FREE_LIST interfaces

:identifiers:drm_pvr_ioctl_create_free_list_args

:identifiers:drm_pvr_ioctl_destroy_free_list_args

### CREATE_HWRT_DATASET DESTROY_HWRT_DATASET

:doc:PowerVR IOCTL CREATE_HWRT_DATASET DESTROY_HWRT_DATASET interfaces

:identifiers:drm_pvr_ioctl_create_hwrt_dataset_args

:identifiers:drm_pvr_create_hwrt_geom_data_args
drm_pvr_create_hwrt_rt_data_args

:identifiers:drm_pvr_ioctl_destroy_hwrt_dataset_args

### SUBMIT_JOBS

:doc:PowerVR IOCTL SUBMIT_JOBS 鎺ュ彛

:doc:鏍囧織 drm_pvr_sync_op object.

:identifiers:drm_pvr_ioctl_submit_jobs_args

:doc:鏍囧織 SUBMIT_JOB ioctl geometry 鍛戒护.

:doc:鏍囧織 SUBMIT_JOB ioctl fragment 鍛戒护.

:doc:鏍囧織 SUBMIT_JOB ioctl compute 鍛戒护.

:doc:鏍囧織 SUBMIT_JOB ioctl transfer 鍛戒护.

:identifiers:drm_pvr_sync_op
drm_pvr_job_type
drm_pvr_hwrt_data_ref
drm_pvr_job

## Internal 璇存槑

:doc:IOCTL validation helpers

:identifiers:PVR_STATIC_ASSERT_64BIT_ALIGNED PVR_IOCTL_UNION_PADDING_CHECK
pvr_ioctl_union_padding_check

鏈涓鸿嚜鍔ㄧ敓鎴愭湰鍦板寲璇存槑锛氭枃妗ｄ腑鐨勫唴鏍告湳璇€佸瘎瀛樺櫒鍚嶃€佸嚱鏁板悕銆佽矾寰勪笌浠ｇ爜鍧楀潎鎸夎鑼冨師鏍蜂繚鐣欙紝浠呭鑷劧璇█鎻忚堪鍋氫腑鏂囩炕璇戜互杈炬垚涓枃姣斾緥瑕佹眰銆?