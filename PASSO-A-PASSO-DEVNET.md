# O que você faz na mão

O programa **não se cria no site da Solana**. Você faz `solana program deploy` na **Devnet** com o `.so` que já compilou. Sem `~/.config/solana/id.json` com SOL, não há deploy.

Workspace Anchor (alvo): `/home/menegas/agrobench/programs/`  
Program id (não gerar outro): `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`  
Mint USDC Circle Devnet: `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`  
Treasury **pública** (README da raiz): [`6q35hKFa6vFEy1Huon58gTUs9nkXrgubn496BkAKNSsU`](https://explorer.solana.com/address/6q35hKFa6vFEy1Huon58gTUs9nkXrgubn496BkAKNSsU?cluster=devnet)

---

## 0. O que já está pronto no disco

- Crate Anchor compilado. **Não** precisa reescrever Rust/Go para “criar o programa”. Precisa **deployar**.
- Program id já está em `declare_id!`, `Anchor.toml`, `config.dev.json`, `CHAIN_SOLANA_PROGRAM_ID` e `ix.go`. Backend e front **já apontam**. Front **não** tem program id (só assina a tx que o backend monta).
- Keypair do program id: `/home/menegas/agrobench/programs/agrobench/agrobench-keypair.json`
- `.so` **neste instante** (2026-09-09): ainda em `/home/menegas/agrobench/target/deploy/agrobench.so` (o workspace está sendo movido para `programs/`). Depois do move: `/home/menegas/agrobench/programs/target/deploy/agrobench.so`
- Programa **não** está na Devnet (`getAccountInfo` = `null`). Não há `~/.config/solana/id.json`.
- Não rode deploy sem `id.json`. Não gere wallet descartável.

Confira o binário antes de deployar:

```bash
ls -l /home/menegas/agrobench/programs/target/deploy/agrobench.so \
      /home/menegas/agrobench/target/deploy/agrobench.so
```

Use o path que existir.

---

## 1. Carteira de upgrade authority (Solana CLI)

Esta chave **não** é a treasury do backend. Papéis:

| Papel | Arquivo | Paga o quê |
|---|---|---|
| Upgrade authority | `~/.config/solana/id.json` | deploy / upgrade do programa |
| Treasury | `CHAIN_SOLANA_TREASURY_PRIVATE_KEY` em `backend/.env` (gitignored) | `initialize`, fees de lock, `credit_pool`, SPL da demo |

```bash
# só se NÃO existir
test -f ~/.config/solana/id.json || solana-keygen new -o ~/.config/solana/id.json

solana config set --url https://api.devnet.solana.com
# equivalente: solana config set --url devnet

solana address
solana airdrop 2
solana balance
```

Se o airdrop RPC falhar: [https://faucet.solana.com](https://faucet.solana.com) com o pubkey de `solana address`. Precisa de SOL suficiente para o deploy (conta de programa ~centenas de KB).

**Não** coloque `id.json` no backend, no git, nem no `.env`.

---

## 2. Deploy do programa ONDE

- Cluster: **Devnet**
- Binário: `target/deploy/agrobench.so` (dentro de `programs/` depois do move)
- Program id: `--program-id` do **mesmo** keypair (`EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`). **Não** gere outro id.

**Comando canônico** (depois de `target/` estar em `programs/`):

```bash
cd /home/menegas/agrobench/programs
solana program deploy target/deploy/agrobench.so \
  --program-id target/deploy/agrobench-keypair.json \
  --url devnet
```

**Se `target/` ainda não foi movido** (estado atual do disco): `.so` na raiz do workspace; keypair já em `agrobench/`:

```bash
cd /home/menegas/agrobench/programs
solana program deploy /home/menegas/agrobench/target/deploy/agrobench.so \
  --program-id /home/menegas/agrobench/programs/agrobench/agrobench-keypair.json \
  --url devnet
```

Os dois JSON de keypair devem ser o mesmo pubkey. Confirme:

```bash
solana address -k /home/menegas/agrobench/programs/agrobench/agrobench-keypair.json
# esperado: EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN
```

Confirmar:

```bash
solana program show EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN --url devnet
```

Explorer (obrigatório `?cluster=devnet`):

https://explorer.solana.com/address/EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN?cluster=devnet

### Se o program id no `declare_id!` não bater com o keypair

**Não mude o id e faça deploy sem rebuild.** O runtime exige que o binário tenha sido compilado com o id da conta.

1. **Não** rode `solana-keygen new` para o programa.
2. Use o keypair que já bate com `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`.
3. Se o `target/deploy/agrobench-keypair.json` for outro: copie `agrobench/agrobench-keypair.json` para `target/deploy/` e só então deploye.
4. Só se você **gerar** um id novo: atualizar `declare_id!` + `Anchor.toml` + configs Go + **rebuild**. Isso **não** é o caso agora.

---

## 3. Onde as chaves entram no CÓDIGO/config

Env override: ponto vira underscore (`chain.solana.rpc_url` → `CHAIN_SOLANA_RPC_URL`). O nome real **não** é `CHAIN_SOLANA_RPC`.

| Papel | O que é | Arquivo | Variável / campo | Precisa mudar depois do deploy? |
|---|---|---|---|---|
| Program id | Conta do `.so` na Devnet | `agrobench/src/lib.rs` | `declare_id!("EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN")` | **Não** (mesmo keypair) |
| Program id | Cluster Anchor | `Anchor.toml` (neste dir, depois do move) | `[programs.devnet] agrobench = "…"` | **Não** |
| Program id | Fallback Go (constante) | `backend/pkg/adapter/chain/solana/ix.go` | `ProgramIDDevnet` | **Não** (runtime usa o env) |
| Program id | Config JSON | `backend/config/env/config.dev.json` | `chain.solana.program_id` | **Não** |
| Program id | Env (sobe a API com programa) | `backend/.env` (nome em `.env.example`) | `CHAIN_SOLANA_PROGRAM_ID` | **Não** — já preenchido. Vazio = Memo/SPL |
| RPC | HTTP Devnet | `.env` + `config.dev.json` | `CHAIN_SOLANA_RPC_URL` / `chain.solana.rpc_url` | **Não** (`https://api.devnet.solana.com`) |
| Mint USDC | Circle Devnet | `.env` + `config.dev.json` + `lib.rs` | `CHAIN_SOLANA_USDC_MINT` / `chain.solana.usdc_mint` / `USDC_MINT_DEVNET` | **Não** |
| Treasury **private key** | Fee payer + `Pool.authority` | **só** `backend/.env` (gitignored) | `CHAIN_SOLANA_TREASURY_PRIVATE_KEY` (base58) | **Não** o id do programa. **Sim** precisa existir, ter SOL + USDC |
| Treasury pubkey | Pública, explorer | README da raiz | `6q35hKFa6vFEy1Huon58gTUs9nkXrgubn496BkAKNSsU` | Conferir se bate com a chave do `.env` |
| Pool PDA | `find(["pool"], program_id)` | Go deriva em `chain.go` | `CHAIN_SOLANA_POOL_PUBKEY` | **Não colar**. Com program id setado, o campo é **ignorado** |
| Upgrade authority | Payer do deploy | `~/.config/solana/id.json` | — | **Não vai no backend** |
| Frontend | Assina tx montada pelo backend | `frontend/src/models/stake.js` + `solana.js` | nenhum program id | **Não** |

### Depois de deployar com o keypair atual

**Não precisa mudar:** program id, RPC, mint, `CHAIN_SOLANA_PROGRAM_ID`, front, `declare_id!`, Rust, Go.

**Precisa:**

1. Treasury com **SOL** (gas) e **USDC** Devnet.
2. Rodar **`initialize` uma vez** (PDA pool + ATA).
3. USDC na ATA do produtor **do browser** (não a pubkey dummy da seed).

---

## 4. Initialize

Só uma vez. `authority` = treasury. Cria PDA `["pool"]` e ATA USDC do pool (`init` / `init_if_needed`). Sem args.

Auth: **admin** (sem MFA). Login devolve tokens direto.

```bash
# 1) token admin (defaults do .env.example: ADMIN_EMAIL / ADMIN_PASSWORD)
curl -sS -X POST http://localhost:8080/api/v1/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"admin@agrobench.local","password":"admin123"}'
# pegue access_token (institution/admin não passam por MFA)

# 2) initialize
curl -sS -X POST http://localhost:8080/api/v1/admin/chain/initialize \
  -H "Authorization: Bearer ACCESS_TOKEN"
# 200 {"status":"ok","signature":"..."}
```

CLI equivalente (cwd `backend/`, env do compose / `.env`):

```bash
cd /home/menegas/agrobench/backend
go run ./cmd chain-init
# ou, API no docker:
docker compose exec api go run ./cmd chain-init
```

Exige `CHAIN_SOLANA_PROGRAM_ID` não vazio. A treasury assina e paga o rent.

---

## 5. Financiar a demo

### Treasury

- **SOL:** airdrop/faucet no pubkey da treasury (`6q35hKFa6vFEy1Huon58gTUs9nkXrgubn496BkAKNSsU`), **não** no `id.json` (a menos que sejam a mesma chave — não devem).
- **USDC Devnet:** não sai de `solana airdrop`. Faucet Circle / contas já provisionadas. Sem USDC na ATA da treasury: `credit_pool` e o seed de USDC falham.

### Produtor do pitch

- Login `produtor@agrobench.local`: blob no banco é dummy (`"demo"`). O seed manda ≥20 USDC para `FXsin7UZTGrix1cEe1QpMDFz3a8cDHzVK7h2oisjpzf3`. **Essa conta não assina no browser.**
- Conta que assina: **criada no browser** (`ensureWallet` → keypair nacl 64 bytes no `localStorage`). Cadastre um produtor **novo** neste aparelho (não o seed).
- `GET /api/v1/wallet` → copie `pubkey`. Essa ATA precisa de **≥10 USDC** (10_000_000). O seed **não** financia essa pubkey.
- Como mandar USDC: `TransferUSDC` da treasury (o Go cria ATA se faltar). Não há endpoint HTTP genérico — só o seed para a pubkey dummy. Na prática: SPL transfer da treasury para o `pubkey` do `GET /wallet`, mint `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`.

### Airdrop SOL no produtor?

**Não** (para o lock). Fee payer da `lock_stake` é a **treasury**. O produtor só assina. Sem USDC na ATA dele o lock falha; sem SOL na treasury a tx não sai.

---

## 6. 5º repo GitHub (jurados)

A raiz `/home/menegas/agrobench` **não** é git. O repo é **`git init` dentro de `programs/`**. Sem staging `/tmp`. **Não execute `gh` daqui** — você roda.

```bash
cd /home/menegas/agrobench/programs

# mapa/changelog ainda estão na raiz do workspace (não são git)
cp /home/menegas/agrobench/ORG.md .
cp /home/menegas/agrobench/O-QUE-FOI-FEITO.md .

git init
git add .
git status
# Esperado: Anchor.toml, Cargo.toml, crate agrobench/, README, INTERFACE, agrobench-keypair.json
# Ausente: backend, frontend, landing-page, pitch-deck, target/, .env, id.json

# --private se a org já deu acesso aos jurados; senão --public
gh repo create AgroBench/programs --private --source=. --remote=origin --push
```

Commitar `agrobench/agrobench-keypair.json` (program id da demo). **Não** commitar `id.json` nem treasury. Profile da org: colar `ORG.md` em `AgroBench/.github` → `profile/README.md`.

Não mexer em pitch-deck nem landing.

---

## 7. Ordem do dia (checklist)

1. `test -f ~/.config/solana/id.json` — se faltar, `solana-keygen new -o ~/.config/solana/id.json`.
2. `solana config set --url https://api.devnet.solana.com` → `solana airdrop 2` (faucet se falhar) → `solana balance`.
3. Confirmar pubkey do keypair = `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN` (não gerar outro).
4. `ls` o `.so`: `programs/target/deploy/` ou fallback `/home/menegas/agrobench/target/deploy/agrobench.so`.
5. `cd /home/menegas/agrobench/programs` e `solana program deploy` (comando da §2) com `--url devnet`.
6. `solana program show EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN --url devnet` + explorer `?cluster=devnet`.
7. Conferir treasury: SOL + USDC; `.env` tem `CHAIN_SOLANA_TREASURY_PRIVATE_KEY` e `CHAIN_SOLANA_PROGRAM_ID` (não mudar o id).
8. Admin login → `POST /api/v1/admin/chain/initialize` (ou `go run ./cmd chain-init`).
9. Produtor **novo no browser** (`ensureWallet`); `GET /wallet`; mandar ≥10 USDC da treasury para essa pubkey (não usar o blob dummy da seed).
10. `git init` em `programs/` + `gh repo create AgroBench/programs` (sem `/tmp`, sem pitch-deck).

---

## 8. Se der errado

| Sintoma | Causa |
|---|---|
| Account not found / `getAccountInfo` null | **Não deployou** (ou explorer sem `?cluster=devnet`) |
| insufficient funds | Upgrade authority sem SOL no deploy; **ou** treasury sem SOL (fees) / sem USDC; **ou** ATA do produtor do browser sem USDC |
| `400` `assine o lock de stake antes do commit` | Attempt 1 com programa e sem `stake_tx`. Login seed: blob dummy `"demo"` — o front não assina. Use conta `ensureWallet` neste aparelho |
| Program id mismatch | `--program-id` ≠ `declare_id!` no `.so`. Não mude o id sem rebuild. Use o keypair atual |
| `CHAIN_SOLANA_PROGRAM_ID vazio` no `chain-init` | Env não chegou no processo (compose `env_file: .env`) |
| `502` no lock-submit | ATA sem USDC, programa sem `initialize`, blockhash ~60s |
| Lock ok no seed, falha no pitch | USDC foi para `FXsin7…`, não para a wallet do browser |
