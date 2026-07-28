#!/bin/bash

# var
cnt=0

# check service alive
cnt=`ps aux | grep minio | grep server | grep -v "grep" | wc -l`
if [[ "${cnt}s" != "0s" ]];then
    echo "[INFO] storage service is running, cnt:${cnt}, everything is fine"
    exit
fi

echo "[WARNING] storage service is not running, cnt:${cnt}, start program"

# get architecture type
atype=`arch`
prog="bin/minio-x86_64"
if [[ "${atype}" == "x86_64" || "${atype}" == "AMD64" || "${atype}" == "x64" ]]; then
    echo "[INFO] os arch intel/amd type: ${atype}"
else
    # aarch64、arm64
    prog="bin/minio-arm64"
    echo "[INFO] os arch arm type: ${atype}"
fi

# check dir
dir="/data"
if [[ ! -d "${dir}" ]]];then
    mkdir ${dir}
    echo "[INFO] create dir ${dir} success"
fi

# start service
chmod +x ./${prog}
nohup ./${prog} server ${dir} &