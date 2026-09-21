@echo off
cd /d "%~dp0"
echo === diag2 start %date% %time% === > "logs\diag2.txt"
echo APPDATA=%APPDATA% >> "logs\diag2.txt"
echo USERPROFILE=%USERPROFILE% >> "logs\diag2.txt"
echo --- npm -v --- >> "logs\diag2.txt"
call "C:\Users\Administrator\AppData\Local\nvm\v20.18.1\npm.cmd" -v >> "logs\diag2.txt" 2>&1
echo npm-v exit=%errorlevel% >> "logs\diag2.txt"
echo --- npm run run-once --- >> "logs\diag2.txt"
call "C:\Users\Administrator\AppData\Local\nvm\v20.18.1\npm.cmd" run run-once >> "logs\diag2.txt" 2>&1
echo run-once exit=%errorlevel% >> "logs\diag2.txt"
echo === diag2 end === >> "logs\diag2.txt"
