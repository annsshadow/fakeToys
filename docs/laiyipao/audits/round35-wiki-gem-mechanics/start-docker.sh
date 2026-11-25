powershell -NoProfile -Command "Get-Process Docker Desktop -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Path"
powershell -NoProfile -Command "Start-Process C:\Program Files\Docker\Docker\Docker Desktop.exe"
echo DK_DONE
