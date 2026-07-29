@echo off
chdir /d "%~dp0"
go mod tidy
go build -o csv2sql.exe ./src
if errorlevel 1 (
    echo [FAIL] csv2sql compile failed
    pause
    exit /b 1
)
echo [OK] csv2sql compiled successfully
