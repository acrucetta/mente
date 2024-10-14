
## Notes on Azure Powershell

pwsh -> to start Azure Powershell

Install-Module -Name Az -Repository PSGallery -Force

Connect-AzAccount

### To connect with a container instance:

az container exec -g "rg-indigorx-akscluster-eastus" --name "adf-dbt-docker-instance-dev-eus" --exec-command "/bin/bash"

# Set secrets
https://dev-api.cassiansolutions.com"
$secretvalue = ConvertTo-SecureString "YourSecretValue" -AsPlainText -Force

Set-AzKeyVaultSecret -VaultName "YourKeyVaultName" -Name "YourSecretName" -SecretValue $secretvalue
