Get-Process -Name "Docker Desktop" -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Path
Start-Process -FilePath "C:\Program Files\Docker\Docker\Docker Desktop.exe"
Write-Output "DK_DONE"
