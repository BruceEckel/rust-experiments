$rootPath = Get-Location
$files = Get-ChildItem -Path $rootPath -Recurse -Filter "Cargo.toml" -File

$outputPath = Join-Path -Path $rootPath -ChildPath "cargo_all.txt"

foreach ($file in $files) {
    $output = "Found file: $($file.FullName)`r`n"
    $content = Get-Content -Path $file.FullName -Raw
    $output += "File contents:`r`n$content`r`n---------------------`r`n"

    Add-Content -Path $outputPath -Value $output
}

Write-Host "Search results stored in $outputPath"
