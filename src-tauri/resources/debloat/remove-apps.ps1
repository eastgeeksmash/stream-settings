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
    Write-Error "アプリ一覧が見つかりません: $Catalog"
    exit 1
}

$data = Get-Content -LiteralPath $Catalog -Raw -Encoding UTF8 | ConvertFrom-Json
$apps = @($data.groups.$Group)
if ($apps.Count -eq 0 -or $null -eq $apps[0].id) {
    Write-Error "対象のアプリグループが見つかりません: $Group"
    exit 1
}

$failures = New-Object System.Collections.Generic.List[string]

foreach ($app in $apps) {
    $id = [string]$app.id
    $method = [string]$app.method
    Write-Output "Removing $id ($method)"

    if ($method -eq 'WinGet') {
        if ([string]::IsNullOrWhiteSpace($WingetPath) -or -not (Test-Path -LiteralPath $WingetPath)) {
            $failures.Add("${id}: winget.exe が見つかりません。")
            continue
        }
        & $WingetPath uninstall --id $id -e --accept-source-agreements --disable-interactivity | Out-Host
        $code = $LASTEXITCODE
        if ($code -ne 0 -and $alreadyAbsent -notcontains $code) {
            $failures.Add("${id}: winget uninstall が終了コード $code で失敗しました。")
        }
        continue
    }

    $pattern = "*$id*"
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
        $failures.Add("${id}: $($_.Exception.Message)")
    }
}

if ($failures.Count -gt 0) {
    foreach ($failure in $failures) {
        Write-Error $failure
    }
    exit 1
}

exit 0
