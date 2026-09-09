# O que foi feito — código se adapta à copy

Changelog da sessão de 2026-09-09. Para o Felipe e para colar no GitHub de `AgroBench/programs` ou no profile da org.

Mapa pasta → repo: [`ORG.md`](./ORG.md). Deploy do programa: [`programs/README.md`](./programs/README.md). Interface congelada: [`programs/agrobench/INTERFACE.md`](./programs/agrobench/INTERFACE.md). Contrato HTTP: [`backend/docs/frontend-api.md`](./backend/docs/frontend-api.md) §11.

A tese de produto (deck, landing, README de venda) **não mudou**. Isto é o que o código passou a fazer para não contradizer a copy.

---

## Problema

A copy prometia smart contract, stake de **10 USDC** do produtor e split do pool **on-chain**.

O código, até esta sessão, registrava lock/release/credit como **Memo** (texto na Devnet). A treasury assinava sozinha. Nenhum USDC saía da ATA do agricultor. Não havia programa Anchor.

## O que o código faz agora

Programa Anchor `agrobench`, id `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`. Compilou (`target/deploy/agrobench.so`). **Deploy Devnet ainda não feito.**

| Ix | Quem assina | Efeito |
|---|---|---|
| `initialize` | treasury | Cria PDA `pool` + ATA USDC do pool |
| `lock_stake(amount)` | **produtor** + treasury (fee payer) | Transfere USDC da ATA do produtor → ATA do PDA `stake` |
| `release_stake` | **produtor** + treasury | Devolve `Stake.amount` e zera |
| `credit_pool(amount)` | treasury | Assinatura institucional: treasury → ATA do pool |
| `distribute(amounts)` | treasury | Split: pool ATA → ATAs destino (`remaining_accounts`) |

PDAs: `["pool"]` e `["stake", producer_pubkey]`. Mint Circle Devnet: `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`. 10 USDC = `10_000_000` (6 decimais). Tokenkeg clássico (não Token-2022).

Backend (`CHAIN_SOLANA_PROGRAM_ID` setado): deixa de ser Memo-teatro em lock/release/credit/distribute. `LockStake` / `ReleaseStake` devolvem `ErrNeedsCoSign` — o app tem de assinar. `Pool()` passa a ser o PDA, não a treasury. Commit, atestação e CAR **continuam Memo**.

Backend (`CHAIN_SOLANA_PROGRAM_ID` vazio): fallback antigo (Memo + SPL da treasury). A API sobe sem o programa na Devnet.

Frontend: no stepper, o agricultor assina o lock **antes** do commit (`stake.js` + `solana.js`, `@solana/web3.js`). A signature vai no commit como `stake_tx`.

Seed-demo: tenta mandar ≥20 USDC da treasury para a pubkey `FXsin7UZTGrix1cEe1QpMDFz3a8cDHzVK7h2oisjpzf3`. O blob dessa conta no banco é dummy (`"demo"`) — o browser **não** assina com ela.

---

## Contratos HTTP

Prefixo `/api/v1`. JSON. Bearer do produtor, salvo `initialize` (admin).

| Método | Path | Auth | Request | Response |
|---|---|---|---|---|
| `POST` | `/chain/stake/lock-tx` | producer | `{ "amount": 10000000 }` (`0`/omitido → config 10 USDC) | `{ "tx": "<base64>" }` — fee payer = treasury, já parcial |
| `POST` | `/chain/stake/lock-submit` | producer | `{ "tx": "<base64 com sig do produtor>" }` | `{ "signature": "<base58>" }` |
| `POST` | `/contributions/commit` | producer | `cycle_id`, `property_id`, `level`, `hash`, **`stake_tx`** (ou `signed_tx`) | contribuição `committed`; `commit_tx` = Memo |
| `POST` | `/contributions/{id}/release-stake/tx` | producer | `{}` | `{ "tx": "<base64>" }` |
| `POST` | `/contributions/{id}/release-stake/submit` | producer | `{ "tx": "<base64>" }` | `{ "signature": "<base58>" }` |
| `POST` | `/admin/chain/initialize` | admin | (vazio) | `{ "status": "ok", "signature": "..." }` |
| `POST` | `/admin/pool/distribute?month=YYYY-MM` | admin | (vazio) | `202` — worker chama `distribute` on-chain se o programa estiver setado |

Com programa, attempt 1 **sem** `stake_tx`/`signed_tx` → `400` `assine o lock de stake antes do commit`. Attempt 2 (já teve `rejected` no ciclo) não relocka.

O worker de validação **não** libera stake quando o programa exige o produtor (log + stake fica `locked`). O app chama release depois de 2 ciclos aceitos.

CLI equivalente ao initialize: `agrobench chain-init`.

---

## Fluxo do agricultor no stepper

Tela `ContributeView.vue`: 1 Qual safra → 2 Os números → 3 Confirmar.

No confirmar (`commitAndStoreDraft`):

1. Canonicaliza o envelope, calcula o hash, guarda o draft.
2. Se não for retry (não há contribuição `rejected` neste ciclo): `lockStake()`.
3. `POST /chain/stake/lock-tx` → deserialize → `partialSign` com a keypair do aparelho (`secretKey` 64 bytes no localStorage) → `POST /chain/stake/lock-submit`.
4. `POST /contributions/commit` com `stake_tx` = signature do passo 3.
5. Reveal (sealed box) + poll até `accepted` / `rejected`.

Textos de progresso: “Travando os 10 dólares digitais da sua conta…” → “Registrando o envio da safra…”.

Login `produtor@agrobench.local` **não** assina: blob dummy. Conta criada neste browser (`ensureWallet`) assina. Essa pubkey precisa de ATA USDC com ≥10 USDC; o seed financia a pubkey dummy, não a do browser.

---

## IDs e o que ainda falta

| | |
|---|---|
| Program id | `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN` |
| USDC mint (Circle Devnet) | `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU` |
| Pubkey seed (não assina no browser) | `FXsin7UZTGrix1cEe1QpMDFz3a8cDHzVK7h2oisjpzf3` |
| Env | `CHAIN_SOLANA_PROGRAM_ID` em `backend/.env.example` (já preenchido). Fallback = string vazia. |

Ainda falta (não feito nesta sessão):

1. **Deploy Devnet** (`anchor deploy` / `solana program deploy`). `getAccountInfo` do program id estava vazio em 2026-09-09.
2. **`initialize`** uma vez (admin HTTP ou `agrobench chain-init`) depois do deploy.
3. **ATA + USDC da conta que o jurado usa no browser** — não a pubkey dummy da seed.
4. Confirmar no explorer a tx de `lock_stake` (só depois do deploy).

Treasury, upgrade authority e `backend/.env` **não** vão para o git.

---

## Arquivos-chave

| Path | Papel |
|---|---|
| `programs/agrobench/src/lib.rs` | Programa (ixs, PDAs, erros) |
| `programs/agrobench/INTERFACE.md` | Discriminators / ordem das contas para o Go |
| `programs/agrobench/agrobench-keypair.json` | Program id (pode ir no git) |
| `Anchor.toml` | Workspace Anchor (um nível acima de `programs/`) |
| `backend/pkg/adapter/chain/solana/chain.go` | Memo vs programa; `ErrNeedsCoSign` |
| `backend/pkg/adapter/chain/solana/ix.go` | Encoding das ixs |
| `backend/pkg/adapter/chain/solana/tx.go` | Monta tx parcial, co-assina treasury, envia |
| `backend/internal/contribution/usecase/stake.go` | lock-tx / lock-submit / release |
| `backend/internal/contribution/usecase/commit.go` | `stake_tx` / `signed_tx`; fallback `LockStake` |
| `backend/internal/contribution/rest/router.go` | Rotas HTTP |
| `backend/internal/admin/seed/demo.go` | Seed USDC + blob dummy |
| `frontend/src/models/solana.js` | `partialSign` / `@solana/web3.js` |
| `frontend/src/models/stake.js` | Two-step lock/release |
| `frontend/src/models/contribution.js` | Lock antes do commit |
| `frontend/src/views/producer/ContributeView.vue` | Stepper |

---

## O que NÃO mudou

- Copy de produto: pitch-deck, landing, README de venda da raiz.
- Enclave / TEE: continua **mock**.
- SICAR: continua **mock**.
- Commit-reveal, hash, sealed box, painel, instituição, Stripe mock.
- `commit_tx` e atestação: ainda Memo (`agrobench:v1:…`).

---

## Como o jurado reproduz / o que abrir primeiro

1. Org [github.com/AgroBench](https://github.com/AgroBench) — hoje 4 repos. O 5º (`AgroBench/programs`) **ainda não existe**. Comandos em `ORG.md` (Felipe cria; não foi executado).
2. **Programa:** `programs/agrobench/src/lib.rs` (depois `INTERFACE.md`).
3. **Backend:** `backend/pkg/adapter/chain/solana/chain.go` → `tx.go` → `usecase/stake.go` + `commit.go`.
4. **Frontend:** `frontend/src/models/stake.js` → `solana.js` → stepper em `ContributeView.vue`.
5. **Explorer** só depois do deploy: `https://explorer.solana.com/address/EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN?cluster=devnet`.

Build local: na raiz do workspace (`Anchor.toml`), `anchor build` ou o Docker em `programs/README.md`. Sem `gcc` no host, o `.so` já foi gerado via `cargo-build-sbf` em container.
