filter c ([switch]$message)
{
  if ($message) { Out-Host -InputObject $_.Message }
  else { $_ }
}