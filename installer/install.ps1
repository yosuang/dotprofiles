# Create installation directory
$installDir = "$env:USERPROFILE\.local\bin"
if (-not (Test-Path $installDir))
{
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

$downloadUrl = "https://github.com/yosuang/dotprofiles/releases/download/alpha/dotprofiles.exe"
Invoke-WebRequest -Uri $downloadUrl -OutFile "$installDir\dotprofiles.exe"

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installDir*")
{
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installDir", "User")
}

Write-Host "Installed Successfully!" -ForegroundColor Green
