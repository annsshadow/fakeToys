@echo off
chdir /d "%~dp0"
mvn compile -q
if errorlevel 1 (
    echo [FAIL] cool compile failed
    pause
    exit /b 1
)
echo [OK] cool compiled successfully
