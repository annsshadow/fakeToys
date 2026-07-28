#!/bin/bash

# var
cnt=0

# check install shell
basename=`echo $0`
echo "basename: ${basename}"
shellname=${basename##*/}
# subprogress + 1
cnt=`ps aux | grep ${shellname} | grep -v "grep" | wc -l`
if [[ "${cnt}s" > "2s" ]];then
    echo "[WARN] ${shellname} is running, cnt:${cnt}, please check the progress"
    exit
fi
echo "[INFO] ${shellname} is not running, cnt:${cnt}, start installation"

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

cnt=`ps aux | grep ${prog} | grep -v "grep" | wc -l`
if [[ "${cnt}s" != "0s" ]];then
    echo "[WARN] ${prog} is running, cnt:${cnt}, please check the progress"
    exit
fi
echo "[INFO] ${prog} is not running, cnt:${cnt}"

# check os version
cnt=`cat /etc/os-release | grep 'NAME' | grep -i -E 'CentOS|centos' | wc -l`
if [[ "${cnt}s" == "0s" || -z "${cnt}" ]]; then
    echo "[INFO]System is not CentOS, cnt:${cnt}"
else
    echo "[INFO]System is CentOS, cnt:${cnt}"

    # firewall-cmd
    cnt=`ps aux|grep firewall-cmd|grep -v 'grep'|wc -l`
    if [[ "${cnt}s" != "0s" ]]; then
        zone=`firewall-cmd --get-active-zones`
        ret=$?
        if [[ "${ret}" == "0" ]]; then
            firewall-cmd --zone=${zone} --add-port=9000/tcp --permanent
            firewall-cmd --reload
        else
            echo "[WARN]FirewallD maybe is not running, ret:${ret}"
        fi

    else
        echo "[INFO]System not use firewall, cnt:${cnt}"
    fi

    # iptables
    iptables -A INPUT -p tcp --dport 9000:9010 -j ACCEPT
    ##service iptables restart
    systemctl restart iptables.service
fi

# check dir
dir="/data/storage_service_data"
if [[ ! -d "${dir}" ]];then
    mkdir ${dir}
    echo "[INFO] create dir ${dir} success"
fi

# start service
chmod +x ./${prog}
nohup ./${prog} server ${dir} &
echo "[INFO] nohup start ${prog}"

# backup and append crontab
ymd=`date +"%Y%m%d"`
crontab -l > crontab.bak.${ymd}
cur_dir=`pwd`
echo "*/1 * * * * cd ${cur_dir}; bash ./check_storage_service.sh" >> /etc/crontab
echo "[INFO] backup and append crontab success"

echo "[INFO] storage start success"
