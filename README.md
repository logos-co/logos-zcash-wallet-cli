# logos-zcash-wallet-cli

`zcash_wallet_cli`: headless Zcash wallet sessions for `logosctl`. It relays to
`zcash_wallet_backend`, which admits it only once it holds the roles a call needs.

## Enrolling it

The backend refuses `logosctl`'s own host identity, so headless use goes through this
module. Give it both roles in the backend's `roles.json` (see that repository), or have a
custodian call `configure`. A refusal comes back with the exact `configure` that would
admit this module, keeping the names already in force.

## Secrets

Pass passwords and recovery phrases as `@file` arguments, never on the command line.
The module scrubs them after relaying. Note that `logosctl watch zcash_wallet_cli` with no
filter publishes every method reply as an event, including `reveal_seed`'s.

## A session

```bash
logosctl module load zcash_wallet_cli
logosctl call zcash_wallet_cli set_active_network testnet
logosctl call zcash_wallet_cli set_proxy '{"proxy":"socks5h://127.0.0.1:9050","proxyRequired":true}'
logosctl call zcash_wallet_cli open_wallet main @password.txt
logosctl call zcash_wallet_cli job_status b1
logosctl call zcash_wallet_cli balances
logosctl call zcash_wallet_cli prepare_send '{"recipients":[{"address":"u1...","amount":100000}]}'
logosctl call zcash_wallet_cli send_status s1
logosctl call zcash_wallet_cli approve_send s1 @password.txt
```
