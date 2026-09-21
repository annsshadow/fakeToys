@echo off
REM 供 Windows 计划任务调用：每天定时跑一次全部站点签到后退出。
REM 用 run-once 模式（不常驻、不起面板、不抢单实例锁），跑完即退，无重复触发风险。
REM 日志见 logs\YYYY-MM-DD.log。
cd /d "%~dp0"
call "C:\Users\Administrator\AppData\Local\nvm\v20.18.1\npm.cmd" run run-once
