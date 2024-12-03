# Define variables
$deploymentfile = "./target/thumbv6m-none-eabi/debug/wifi_tcp_server"
$deploymentscript = "./scripts/run.sh"
$targetUser = "harry"
$targetHost = "10.0.0.100"
$targetFilePath = "/home/harry/src/AOPW"

# Transfer file to Linux machine
Write-Host "Transferring file to Linux machine..."
scp -r $deploymentfile "${targetUser}@${targetHost}:${targetFilePath}"
scp -r $deploymentscript "${targetUser}@${targetHost}:${targetFilePath}"

if ($LASTEXITCODE -ne 0) {
    Write-Error "File transfer failed!"
    exit 1
}

Write-Host "File transfer completed. Connecting to Linux machine..."

# Define the remote commands with trap for SIGINT
$commands = @"
set -e
trap 'exit' INT
cd $targetFilePath
chmod +x run.sh
./run.sh
"@ -replace "`r", ""

# Execute the commands over SSH with pseudo-terminal allocation
ssh -t -v "$targetUser@$targetHost" "bash -c '$commands'"

if ($LASTEXITCODE -ne 0) {
    Write-Error "Remote command execution failed! Exit code: $LASTEXITCODE"
    exit 1
}
Write-Host "Deployment completed successfully."
