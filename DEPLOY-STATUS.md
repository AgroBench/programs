# Deploy Devnet — status

Programa **não deployado**. RPC `getAccountInfo` de `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN` na Devnet devolveu `value: null`.

`.so` e program-id keypair existem:

- `target/deploy/agrobench.so`
- `target/deploy/agrobench-keypair.json` (pubkey = `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`)

Falta o payer/upgrade authority: **não há** `~/.config/solana/id.json`. Sem essa keypair com SOL Devnet, não há deploy (não gerar wallet descartável).

Comando quando a keypair existir e tiver SOL:

```bash
solana config set --url https://api.devnet.solana.com
solana balance
solana program deploy target/deploy/agrobench.so \
  --program-id target/deploy/agrobench-keypair.json \
  --url https://api.devnet.solana.com
```

Depois: `POST /api/v1/admin/chain/initialize` (ou `go run ./cmd chain-init` no backend) com a treasury.

Seed `produtor@agrobench.local` grava `encrypted_blob` dummy (`"demo"`). O front só assina lock se houver blob nacl (64 bytes) no localStorage deste aparelho — conta criada no browser via `ensureWallet`, não o login de seed.
