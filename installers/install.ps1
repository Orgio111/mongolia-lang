[CmdletBinding()]
param(
    [Parameter(Mandatory)][string] $Binary,
    [Parameter(Mandatory)][ValidatePattern('^[A-Fa-f0-9]{64}$')][string] $Sha256,
    [string] $BinDirectory = (Join-Path $HOME '.local/bin')
)
$ErrorActionPreference = 'Stop'
$directory = [System.IO.Directory]::CreateDirectory($BinDirectory).FullName
$target = Join-Path $directory 'mn.exe'
$temp = Join-Path $directory ('.mn-install-' + [Guid]::NewGuid().ToString('N'))
try {
    [System.IO.File]::Copy((Get-Item -LiteralPath $Binary).FullName, $temp, $false)
    if ((Get-FileHash -LiteralPath $temp -Algorithm SHA256).Hash -ne $Sha256) {
        throw 'Binary checksum mismatch'
    }
    [System.IO.File]::Move($temp, $target)
    Write-Output "Installed $target"
} finally {
    if ([System.IO.File]::Exists($temp)) { [System.IO.File]::Delete($temp) }
}
