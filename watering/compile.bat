@echo off
chdir /d "%~dp0"
go build ./...
if errorlevel 1 (
    echo [FAIL] watering compile failed
    pause
    exit /b 1
)
echo [OK] watering compiled successfully
