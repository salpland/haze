#!/usr/bin/env pwsh

$ErrorActionPreference = 'Stop'

$DownloadUrl = "https://github.com/sedgeland/haze/releases/latest/download/haze.zip"
$BinDir = "${Home}\.haze"
$HazeZip = "${BinDir}\haze.zip"
$HazeExe = "${BinDir}\haze.exe"

if (!(Test-Path $BinDir)) {
  New-Item $BinDir -ItemType Directory | Out-Null
}

curl.exe -Lo $HazeZip $DownloadUrl
tar.exe xf $HazeZip -C $BinDir
Remove-Item $HazeZip

$User = [System.EnvironmentVariableTarget]::User
$Path = [System.Environment]::GetEnvironmentVariable('Path', $User)
if (!(";${Path};".ToLower() -like "*;${BinDir};*".ToLower())) {
  [System.Environment]::SetEnvironmentVariable('Path', "${Path};${BinDir}", $User)
  $Env:Path += ";${BinDir}"
}

Write-Output "Haze was installed successfully to ${HazeExe}"
Write-Output "Run 'haze --help' to get started"
