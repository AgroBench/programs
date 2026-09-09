# AgroBench — programa Solana (Anchor)

## Para jurados / org AgroBench

Este diretório é o **workspace Anchor** do programa on-chain `agrobench`: escrow de stake USDC do produtor (PDA + ATA), pool institucional e split mensal (`initialize`, `lock_stake`, `release_stake`, `credit_pool`, `distribute`). O crate está em `programs/agrobench/`.

| | |
|---|---|
| Program id | `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN` |
| Fonte do id | `declare_id!` em `programs/agrobench/src/lib.rs` · `[programs.devnet]` em `Anchor.toml` (esta raiz) · keypair `programs/agrobench/agrobench-keypair.json` |
| Cluster alvo | Solana Devnet |
| Workspace Anchor | **esta pasta.** `Anchor.toml` e o `Cargo.toml` do workspace estão aqui. O crate é `programs/agrobench/`. |
| Interface (cliente Go) | [`programs/agrobench/INTERFACE.md`](./programs/agrobench/INTERFACE.md) |
| Deploy | **Na Devnet, inicializado.** Status: [`DEPLOY-STATUS.md`](./DEPLOY-STATUS.md). Explorer: [`EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`](https://explorer.solana.com/address/EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN?cluster=devnet). Passo a passo (já executado): [`PASSO-A-PASSO-DEVNET.md`](./PASSO-A-PASSO-DEVNET.md). |

Ordem de leitura: org → este README → `programs/agrobench/src/lib.rs` → backend `pkg/adapter/chain/solana/chain.go` → frontend `src/models/stake.js` → [explorer Devnet](https://explorer.solana.com/address/EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN?cluster=devnet).

| Pasta local | Repo GitHub | O que o jurado olha |
|---|---|---|
| `backend/` | [AgroBench/backend](https://github.com/AgroBench/backend) | API Go |
| `frontend/` | [AgroBench/frontend](https://github.com/AgroBench/frontend) | App Vue |
| `landing-page/` | [AgroBench/landing-page](https://github.com/AgroBench/landing-page) | Landing |
| `pitch-deck/` | [AgroBench/pitch-deck](https://github.com/AgroBench/pitch-deck) | Deck |
| `programs/` (este dir = workspace Anchor) | [AgroBench/programs](https://github.com/AgroBench/programs) | Programa Anchor |

---

Deploy técnico. Cluster: **Devnet**. Mint USDC (Circle Devnet): `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`.

Program id (keypair em `target/deploy/agrobench-keypair.json` e backup em `programs/agrobench/agrobench-keypair.json`):

```
EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN
```

Confirme com:

```bash
solana address -k target/deploy/agrobench-keypair.json
# ou
solana address -k programs/agrobench/agrobench-keypair.json
```

O valor deve ser igual a `declare_id!` em `programs/agrobench/src/lib.rs` e a `[programs.devnet]` em `Anchor.toml`. Se gerar um keypair novo, atualize os três e copie o JSON para `target/deploy/agrobench-keypair.json`.

Interface congelada para o cliente Go: [`programs/agrobench/INTERFACE.md`](./programs/agrobench/INTERFACE.md).

## Pré-requisitos

- Rust (`rustup`)
- Solana CLI 1.18+ / Agave (`solana --version`)
- Anchor CLI **0.32.1** (`anchor --version`)
- Wallet Devnet em `~/.config/solana/id.json`

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

# Solana (Agave)
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Anchor via AVM
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.32.1
avm use 0.32.1

solana config set --url devnet
solana-keygen new -o ~/.config/solana/id.json
```

## Airdrop (gas)

```bash
solana config set --url https://api.devnet.solana.com
solana airdrop 2
solana balance
```

Se o airdrop RPC falhar, use a faucet oficial (https://faucet.solana.com) com o pubkey de `solana address`. A treasury precisa de SOL para fees e USDC Devnet para `credit_pool` / stake.

USDC Devnet não se airdropa via `solana airdrop`. Use o faucet Circle / contas de teste já provisionadas no backend.

## Build

Neste diretório (`/home/menegas/agrobench/programs`, onde está `Anchor.toml`):

```bash
anchor build
```

Equivalente sem o CLI Anchor (depois do Solana CLI instalado):

```bash
cargo build-sbf --manifest-path programs/agrobench/Cargo.toml
```

O `.so` sai em `target/deploy/agrobench.so`. Se o keypair ainda não existir, o `anchor build` cria `target/deploy/agrobench-keypair.json` — nesse caso copie o pubkey para `declare_id!` e `Anchor.toml` e rebuild.

Cópia do keypair deste repo (já alinhada ao `declare_id!`):

```bash
mkdir -p target/deploy
cp programs/agrobench/agrobench-keypair.json target/deploy/agrobench-keypair.json
```

### Sem `gcc` no host (build via Docker)

Este workspace compilou em 09/09/2026 com Agave `cargo-build-sbf` 4.1.0 + imagem `rust:bookworm` (o host não tinha toolchain C/glibc crt):

```bash
# resolver o symlink absolutista do Agave install
SBF_BIN="$HOME/.local/share/solana/install/releases/"*"/solana-release/bin"

docker run --rm \
  -v "$(pwd)":/work \
  -v "$HOME/.cargo/registry":/usr/local/cargo/registry \
  -v "$HOME/.cargo/git":/usr/local/cargo/git \
  -v "$HOME/.local/share/solana":/opt/solana \
  -v "$HOME/.cache/solana":/root/.cache/solana \
  -e HOME=/root \
  -w /work \
  rust:bookworm \
  bash -lc "export PATH=\"/opt/solana/install/releases/\$(ls /opt/solana/install/releases)/solana-release/bin:/usr/local/cargo/bin:\$PATH\"
    cargo-build-sbf --manifest-path programs/agrobench/Cargo.toml"
```

Artefato esperado: `target/deploy/agrobench.so`. Warnings `unexpected_cfgs` (custom-heap / anchor-debug) vêm do rustc novo vs `solana-program` 2.3 — não quebram o build.

## Deploy Devnet

```bash
anchor deploy --provider.cluster devnet
```

Ou:

```bash
solana program deploy target/deploy/agrobench.so \
  --program-id target/deploy/agrobench-keypair.json \
  --url devnet
```

Verifique:

```bash
solana program show EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN --url devnet
```

A wallet em `~/.config/solana/id.json` vira **upgrade authority**. Precisa de SOL Devnet. Sem essa keypair, não faça deploy (não gere uma authority descartável). Depois do deploy, rode `initialize` uma vez com a treasury e o mint Circle.

## PDAs e ATAs

Program id = `AGROBENCH_PROGRAM_ID`.

| Conta | Seeds / derivação | Tipo |
|---|---|---|
| `pool` | `["pool"]` | PDA, owner = programa, dados `Pool` |
| Pool USDC ATA | ATA(`pool`, USDC mint, Tokenkeg) | SPL Token account, owner = PDA `pool` |
| `stake` | `["stake", producer_pubkey]` | PDA, owner = programa, dados `Stake` |
| Stake USDC ATA | ATA(`stake`, USDC mint, Tokenkeg) | SPL Token account, owner = PDA `stake` |

Go:

```go
poolPda, poolBump, err := solana.FindProgramAddress([][]byte{
    []byte("pool"),
}, programID)

stakePda, stakeBump, err := solana.FindProgramAddress([][]byte{
    []byte("stake"),
    producer.Bytes(),
}, programID)
```

ATA (associada clássica, **não** Token-2022):

- Token program: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`
- ATA program: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`
- Sysvar rent: `SysvarRent111111111111111111111111111111111`
- System program: `11111111111111111111111111111111`

10 USDC = `10_000_000` (u64, 6 decimais).

## Instruções (ordem das contas = ordem Anchor)

Discriminators: `sha256("global:<nome>")[0..8]`. Ver tabela completa em [`programs/agrobench/INTERFACE.md`](./programs/agrobench/INTERFACE.md).

### 1. `initialize` (sem args)

Só uma vez. `authority` = treasury.

| # | Nome | Signer | Writable | Notas |
|---|---|---|---|---|
| 0 | `authority` | sim | sim | treasury, payer |
| 1 | `pool` | não | sim | PDA `["pool"]`, `init` |
| 2 | `usdc_mint` | não | não | Circle Devnet USDC |
| 3 | `pool_ata` | não | sim | ATA do PDA `pool`, `init_if_needed` |
| 4 | `system_program` | não | não | |
| 5 | `token_program` | não | não | Tokenkeg |
| 6 | `associated_token_program` | não | não | AToken |
| 7 | `rent` | não | não | Sysvar |

### 2. `lock_stake` (arg: `amount` u64 LE)

Transfere `amount` de `producer_ata` → `stake_ata`. Soma em `Stake.amount`. Tokens ficam na ATA do PDA (produtor **não** pode SPL-transfer sozinho).

| # | Nome | Signer | Writable | Notas |
|---|---|---|---|---|
| 0 | `producer` | sim | não | wallet do agricultor |
| 1 | `authority` | sim | sim | treasury, fee payer do `init` |
| 2 | `stake` | não | sim | PDA `["stake", producer]`, `init_if_needed` |
| 3 | `producer_ata` | não | sim | ATA USDC do produtor |
| 4 | `stake_ata` | não | sim | ATA USDC do PDA `stake`, `init_if_needed` |
| 5 | `usdc_mint` | não | não | |
| 6 | `token_program` | não | não | |
| 7 | `associated_token_program` | não | não | |
| 8 | `system_program` | não | não | |

### 3. `release_stake` (sem args)

Devolve `Stake.amount` → `producer_ata` e zera `amount`. Exige **duas** assinaturas (produtor + treasury).

| # | Nome | Signer | Writable | Notas |
|---|---|---|---|---|
| 0 | `producer` | sim | não | deve bater com `Stake.producer` |
| 1 | `authority` | sim | não | treasury / protocolo |
| 2 | `stake` | não | sim | PDA `["stake", producer]` |
| 3 | `producer_ata` | não | sim | destino |
| 4 | `stake_ata` | não | sim | origem (owner = PDA `stake`) |
| 5 | `token_program` | não | não | |

### 4. `credit_pool` (arg: `amount` u64 LE)

Crédito da assinatura: treasury USDC → pool ATA. `authority` deve ser `Pool.authority`.

| # | Nome | Signer | Writable | Notas |
|---|---|---|---|---|
| 0 | `authority` | sim | não | `Pool.authority` |
| 1 | `pool` | não | não | PDA `["pool"]` |
| 2 | `from_ata` | não | sim | ATA USDC da treasury |
| 3 | `pool_ata` | não | sim | ATA USDC do PDA `pool` |
| 4 | `token_program` | não | não | |

### 5. `distribute` (arg: `Vec<u64>` Borsh)

Split mensal. `amounts.len()` == número de dest ATAs em `remaining_accounts`. Cada `remaining_accounts[i]` recebe `amounts[i]` da `pool_ata`. Dest ATAs **writable**. `authority` = `Pool.authority`.

| # | Nome | Signer | Writable | Notas |
|---|---|---|---|---|
| 0 | `authority` | sim | não | `Pool.authority` |
| 1 | `pool` | não | não | PDA `["pool"]` |
| 2 | `pool_ata` | não | sim | origem |
| 3 | `token_program` | não | não | |
| 4+ | dest ATA (remaining) | não | sim | um por `amounts[i]` |

Borsh de `Vec<u64>`: `u32` LE (len) + `n` vezes `u64` LE.

## Erros (Anchor custom, base 6000)

| Código | Nome | Quando |
|---|---|---|
| 6000 | `InsufficientStake` | amount 0, saldo ATA insuficiente, pool sem USDC para o split |
| 6001 | `Unauthorized` | signer/mint/owner inválido; `has_one` falhou |
| 6002 | `LengthMismatch` | `amounts` ≠ remaining dest ATAs |
| 6003 | `Overflow` | soma `u64` estourou |

## Fluxo esperado (backend)

1. Uma vez: `initialize` com treasury + mint Circle Devnet.
2. Stake do produtor: `lock_stake(10_000_000)` — produtor assina, treasury paga fee. USDC sai da ATA do produtor e fica na ATA do PDA `stake`.
3. Assinatura institucional: `credit_pool(amount)` — USDC da treasury entra no pool on-chain.
4. Split mensal: `distribute(amounts)` com remaining = ATAs dos produtores.
5. Fim de ciclo: `release_stake` — treasury co-assina, USDC volta ao produtor.

## Layout dos accounts (após discriminator de 8 bytes)

`Pool` (8 + 32 + 32 + 1 = 73): `authority` Pubkey, `usdc_mint` Pubkey, `bump` u8.

`Stake` (8 + 32 + 8 + 1 = 49): `producer` Pubkey, `amount` u64 LE, `bump` u8.

## Notas

- SPL Token clássico (não Token-2022).
- `release_stake` não fecha contas; um novo `lock_stake` reutiliza o mesmo PDA/ATA.
- `lock_stake` / `release_stake` não recebem a conta `pool` (interface congelada). A trava on-chain é a ATA owned pelo PDA; o destravamento exige a ix do programa + co-assinatura da treasury.
