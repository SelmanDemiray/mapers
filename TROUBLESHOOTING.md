# Connection Troubleshooting Guide

## Current Situation
- Docker containers are running on your NAS (192.168.0.178)
- Frontend: Port 41968
- Backend: Port 37291
- Database: Port 28473

## Most Likely Issue: Windows Firewall

The ports are probably blocked by Windows Firewall on your NAS. Follow these steps:

### Quick Fix: Allow Ports Through Firewall

**Option 1: PowerShell (Run as Administrator on the NAS)**

```powershell
# Allow Frontend Port
New-NetFirewallRule -DisplayName "Emulator Platform Frontend" -Direction Inbound -LocalPort 41968 -Protocol TCP -Action Allow

# Allow Backend Port
New-NetFirewallRule -DisplayName "Emulator Platform Backend" -Direction Inbound -LocalPort 37291 -Protocol TCP -Action Allow

# Verify the rules were created
Get-NetFirewallRule | Where-Object {$_.DisplayName -like "*Emulator Platform*"}
```

**Option 2: GUI Method**

1. On the NAS, open **Windows Defender Firewall with Advanced Security**
2. Click **Inbound Rules** → **New Rule**
3. Select **Port** → Click **Next**
4. Select **TCP** and enter **41968** in Specific local ports → **Next**
5. Select **Allow the connection** → **Next**
6. Check all profiles (Domain, Private, Public) → **Next**
7. Name it "Emulator Platform Frontend" → **Finish**
8. Repeat for port **37291** (Backend)

### Verify Docker is Working

Run these commands on the NAS:

```powershell
# Check container status
docker ps

# Check frontend logs
docker logs 19-frontend-1 --tail 50

# Check backend logs
docker logs 19-backend-1 --tail 50

# Check if ports are listening locally
netstat -ano | findstr "41968"
netstat -ano | findstr "37291"
```

### Test Local Access First

On the NAS itself, try accessing:
- http://localhost:41968 (Frontend)
- http://localhost:37291/api/emulators (Backend API)

If these work locally but not from your other computer, it's definitely a firewall issue.

### Test from Another Computer

After opening the firewall ports, access:
- http://192.168.0.178:41968 (Frontend)
- http://192.168.0.178:37291/api/emulators (Backend API test)

## Alternative: Temporarily Disable Firewall (Testing Only)

**WARNING: Only do this temporarily for testing!**

```powershell
# Disable Windows Firewall (all profiles) - Run as Administrator
Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled False

# After testing, re-enable it
Set-NetFirewallProfile -Profile Domain,Public,Private -Enabled True
```

If the app works with the firewall disabled, you know the ports just need to be allowed.

## Other Potential Issues

### 1. Docker Network Mode
Verify Docker is using bridge mode (default). Check docker-compose.yml - there's no network_mode specified, which is correct.

### 2. Restart Docker Containers
```bash
docker-compose down
docker-compose up -d --build
```

### 3. Check Docker Host IP Binding
The docker ps output shows `0.0.0.0:41968->3000/tcp` which means it's binding to all interfaces (correct).

### 4. Router/Network Issues
If none of the above work, check if your router has any firewall rules or if there's network isolation between devices.

## Quick Health Check Script

Save this as `check-services.ps1` and run on the NAS:

```powershell
Write-Host "=== Docker Container Status ===" -ForegroundColor Cyan
docker ps --filter name=19-

Write-Host "`n=== Port Listening Status ===" -ForegroundColor Cyan
$ports = @(41968, 37291, 28473)
foreach ($port in $ports) {
    $listening = netstat -ano | Select-String ":$port "
    if ($listening) {
        Write-Host "✓ Port $port is listening" -ForegroundColor Green
    } else {
        Write-Host "✗ Port $port is NOT listening" -ForegroundColor Red
    }
}

Write-Host "`n=== Firewall Rules ===" -ForegroundColor Cyan
Get-NetFirewallRule | Where-Object {$_.DisplayName -like "*41968*" -or $_.DisplayName -like "*Emulator*"} | Select-Object DisplayName, Enabled, Direction

Write-Host "`n=== Testing Local Connectivity ===" -ForegroundColor Cyan
try {
    $response = Invoke-WebRequest -Uri "http://localhost:41968" -TimeoutSec 5 -UseBasicParsing
    Write-Host "✓ Frontend responds locally" -ForegroundColor Green
} catch {
    Write-Host "✗ Frontend not responding locally: $($_.Exception.Message)" -ForegroundColor Red
}

try {
    $response = Invoke-WebRequest -Uri "http://localhost:37291/api/emulators" -TimeoutSec 5 -UseBasicParsing
    Write-Host "✓ Backend API responds locally" -ForegroundColor Green
} catch {
    Write-Host "✗ Backend API not responding locally: $($_.Exception.Message)" -ForegroundColor Red
}
```

## Expected Result

After fixing the firewall, you should be able to access:
- **http://192.168.0.178:41968** - Main application interface
- **http://192.168.0.178:37291/api/emulators** - Backend API (should return JSON)

