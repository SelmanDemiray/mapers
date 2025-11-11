# Start script for Maplex Games with Cloudflare Tunnel
# This script starts all services and displays your shareable URL

Write-Host "Starting Maplex Games Platform..." -ForegroundColor Cyan
Write-Host ""

# Change to the script directory (where docker-compose.yml is)
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
if (Test-Path (Join-Path $scriptPath "docker-compose.yml")) {
    Set-Location $scriptPath
} else {
    Write-Host "ERROR: docker-compose.yml not found in $scriptPath" -ForegroundColor Red
    exit 1
}

# Step 1: Build and start all services
Write-Host "Building and starting services..." -ForegroundColor Yellow
docker-compose up --build -d

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to start services!" -ForegroundColor Red
    exit 1
}

# Step 2: Wait for services to be ready
Write-Host "Waiting for services to be ready..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Step 3: Start Cloudflare Tunnel (restart if already running to get fresh URL)
Write-Host "Starting Cloudflare Tunnel..." -ForegroundColor Yellow
docker-compose --profile cloudflare stop cloudflared 2>&1 | Out-Null
docker-compose --profile cloudflare rm -f cloudflared 2>&1 | Out-Null
docker-compose --profile cloudflare up -d cloudflared

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to start Cloudflare Tunnel!" -ForegroundColor Red
    exit 1
}

# Step 4: Wait for tunnel to initialize
Write-Host "Waiting for tunnel to initialize..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Step 5: Extract and display the URL
Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host "Your app is now globally accessible!" -ForegroundColor Green
Write-Host ""

$logs = docker-compose --profile cloudflare logs cloudflared 2>&1
$urlLine = $logs | Select-String "trycloudflare.com" | Select-Object -Last 1

$url = $null
if ($urlLine) {
    # Extract URL from log line
    $match = $urlLine.Line | Select-String -Pattern 'https://[^\s]+trycloudflare\.com'
    if ($match) {
        $url = $match.Matches[0].Value
    }
}

if (-not $url) {
    Write-Host "URL not found in initial logs. Checking again..." -ForegroundColor Yellow
    Start-Sleep -Seconds 3
    $logs = docker-compose --profile cloudflare logs cloudflared 2>&1
    $urlLine = $logs | Select-String "trycloudflare.com" | Select-Object -Last 1
    if ($urlLine) {
        $match = $urlLine.Line | Select-String -Pattern 'https://[^\s]+trycloudflare\.com'
        if ($match) {
            $url = $match.Matches[0].Value
        }
    }
}

if ($url) {
    Write-Host "Share this URL with your friends:" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "   $url" -ForegroundColor White -BackgroundColor DarkBlue
    Write-Host ""
    Write-Host "URL copied to clipboard!" -ForegroundColor Green
    Set-Clipboard -Value $url
} else {
    Write-Host "WARNING: URL not found in logs yet." -ForegroundColor Yellow
    Write-Host "Run this to see it:" -ForegroundColor Yellow
    Write-Host "   docker-compose --profile cloudflare logs cloudflared | Select-String 'trycloudflare.com'" -ForegroundColor Gray
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Green
Write-Host ""
Write-Host "View logs: docker-compose logs -f" -ForegroundColor Gray
Write-Host "Stop all: docker-compose down" -ForegroundColor Gray
Write-Host "Restart tunnel: docker-compose --profile cloudflare restart cloudflared" -ForegroundColor Gray
Write-Host ""
