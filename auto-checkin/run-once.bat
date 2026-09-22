@echo off
REM Called by Windows Task Scheduler: run all sites once, then exit.
REM Uses run-once mode (no daemon, no panel, no single-instance lock) so there is
REM no duplicate-trigger risk. Checkin logs: logs\YYYY-MM-DD.log
REM Keep this file ASCII-only: cmd.exe mis-parses UTF-8 non-ASCII bytes under GBK.
cd /d "%~dp0"
call "C:\Users\Administrator\AppData\Local\nvm\v20.18.1\npm.cmd" run run-once
