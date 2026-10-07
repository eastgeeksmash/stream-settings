# App removal excerpted from Raphire/Win11Debloat (MIT License).
# https://github.com/Raphire/Win11Debloat
# Only Appx removal, provisioned-package removal, and WinGet uninstall are included.
param(
    [Parameter(Mandatory = $true)]
    [string]$Catalog,
    [Parameter(Mandatory = $true)]
    [string]$Group,
    [string]$WingetPath
)

$ErrorActionPreference = 'Continue'
$alreadyAbsent = @(-1978335212, -1978335189)

if (-not (Test-Path -LiteralPath $Catalog)) {
    Write-Error "Catalog not found: $Catalog"
    exit 1
}

$data = Get-Content -LiteralPath $Catalog -Raw | ConvertFrom-Json
$apps = @($data.groups.$Group)
if ($apps.Count -eq 0 -or $null -eq $apps[0].id) {
    Write-Error "Unknown app group: $Group"
    exit 1
}

$failures = @()

foreach ($app in $apps) {
    $id = [string]$app.id
    $method = [string]$app.method
    Write-Output "Removing $id ($method)"

    if ($method -eq 'WinGet') {
        if ([string]::IsNullOrWhiteSpace($WingetPath) -or -not (Test-Path -LiteralPath $WingetPath)) {
            $failures += ($id + ': winget.exe was not found.')
            continue
        }
        & $WingetPath uninstall --id $id -e --accept-source-agreements --disable-interactivity | Out-Host
        $code = $LASTEXITCODE
        if ($code -ne 0 -and $alreadyAbsent -notcontains $code) {
            $failures += ($id + ': winget uninstall failed with exit code ' + $code)
        }
        continue
    }

    $pattern = '*' + $id + '*'
    try {
        $packages = @(Get-AppxPackage -AllUsers -Name $pattern -ErrorAction SilentlyContinue)
        foreach ($package in $packages) {
            Remove-AppxPackage -Package $package.PackageFullName -AllUsers -ErrorAction Stop
        }
        $provisioned = @(Get-AppxProvisionedPackage -Online -ErrorAction Stop | Where-Object { $_.PackageName -like $pattern })
        foreach ($package in $provisioned) {
            Remove-AppxProvisionedPackage -Online -AllUsers -PackageName $package.PackageName -ErrorAction Stop | Out-Null
        }
    }
    catch {
        $failures += ($id + ': ' + $_.Exception.Message)
    }
}

if ($failures.Count -gt 0) {
    foreach ($failure in $failures) {
        Write-Error $failure
    }
    exit 1
}

exit 0
