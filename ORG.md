# AgroBench — mapa da organização (jurados)

A raiz local `/home/menegas/agrobench` **não é um repositório git** e **não vai ser**. Os jurados aceitaram o link da org: [github.com/AgroBench](https://github.com/AgroBench). Cada pasta da raiz (com `.git`) já é um repo da org — **exceto** esta pasta `programs/`, que ainda precisa de `git init`.

Este arquivo é o mapa neutro. Cole-o no README da org (`AgroBench/.github` → `profile/README.md`). Sem o 5º repo, quem abre a org **não vê** o programa on-chain.

## O que o jurado olha

| Pasta local | Repo GitHub | O que o jurado olha |
|---|---|---|
| `backend/` | [https://github.com/AgroBench/backend](https://github.com/AgroBench/backend) | API Go (commit-reveal, stake, painel). Já era repo. |
| `frontend/` | [https://github.com/AgroBench/AgroBenchFront](https://github.com/AgroBench/AgroBenchFront) | App Vue. Já era repo. |
| `landing-page/` | [https://github.com/AgroBench/agrobenchlanding](https://github.com/AgroBench/agrobenchlanding) | Landing. Já era repo. |
| `pitch-deck/` | [https://github.com/AgroBench/pitch-deck](https://github.com/AgroBench/pitch-deck) | Deck. Já era repo. |
| `programs/` | **AINDA NÃO É REPO — precisa criar** `AgroBench/programs` | Programa Anchor (escrow de stake + pool + split). Workspace Anchor completo. |

Remotes conferidos com `git -C <dir> remote -v` (2026-09-09):

- `backend/` → `git@github.com:AgroBench/backend.git` (`main`)
- `frontend/` → `git@github.com:AgroBench/AgroBenchFront.git` (`main`)
- `landing-page/` → `git@github.com:AgroBench/agrobenchlanding.git` (`main`)
- `pitch-deck/` → `git@github.com:AgroBench/pitch-deck.git` (`main`)
- `programs/` → **sem `.git`** (este diretório)
- raiz `/home/menegas/agrobench` → **sem `.git`** (e não vai ter)

Arquivos só locais (não sobem neste repo): `PLANO-SUBIR-NOTA.md`, `RELATORIO-HACKATON.md`, `docker-compose.yaml`, README de produto da raiz, `backend/`, `frontend/`, `landing-page/`, `pitch-deck/`.

## Programa on-chain (este repo)

Workspace Anchor completo. Raiz = esta pasta (`Anchor.toml` + `Cargo.toml`).

```
/home/menegas/agrobench/programs/     ← root Anchor (= futuro repo GitHub)
  Anchor.toml
  Cargo.toml
  Cargo.lock
  README.md
  DEPLOY-STATUS.md
  PASSO-A-PASSO-DEVNET.md
  ORG.md
  O-QUE-FOI-FEITO.md
  programs/agrobench/                 ← crate (src/lib.rs, INTERFACE.md, program keypair)
  target/deploy/                      ← .so (não commitar; .gitignore)
```

- Program id: `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN` (`declare_id!` em `programs/agrobench/src/lib.rs`)
- Interface congelada: `programs/agrobench/INTERFACE.md`
- README técnico: `README.md` (seção **Para jurados** no topo)
- Deploy: **não feito**. Ver `DEPLOY-STATUS.md` e `PASSO-A-PASSO-DEVNET.md`. Há build local: `target/deploy/agrobench.so`.

## SIM — criar o 5º repo aqui

**Não** `git init` na raiz `/home/menegas/agrobench`: lá estão `backend/`, `frontend/`, `landing-page/`, `pitch-deck/` (já são git). Isso viraria monorepo acidental.

**Sim:** `git init` **dentro de** `/home/menegas/agrobench/programs`. Sem staging `/tmp`.

Incluir: `Anchor.toml`, `Cargo.toml`, `Cargo.lock`, `.gitignore`, `ORG.md`, `O-QUE-FOI-FEITO.md`, `README.md`, `DEPLOY-STATUS.md`, `PASSO-A-PASSO-DEVNET.md`, `programs/agrobench/` (incluindo `agrobench-keypair.json`).

Não incluir: `target/`, `.anchor/`, `backend/`, `frontend/`, `landing-page/`, `pitch-deck/`, `.env`, `id.json`.

**Keypairs:** commitar `programs/agrobench/agrobench-keypair.json` (program id da demo; jurados conferem o pubkey). **Não** commitar `~/.config/solana/id.json` nem treasury do `backend/.env` (upgrade authority / fundos).

### Comandos (Felipe roda; não foram executados)

```bash
cd /home/menegas/agrobench/programs
# .gitignore já cobre target/ .anchor .env id.json
git init
git add Anchor.toml Cargo.toml Cargo.lock programs README.md DEPLOY-STATUS.md PASSO-A-PASSO-DEVNET.md ORG.md O-QUE-FOI-FEITO.md .gitignore
git status
# Esperado: workspace Anchor + crate programs/agrobench/
# Ausente: backend, frontend, landing-page, pitch-deck, target, .env, id.json

# --private se a org já deu acesso aos jurados; senão --public
gh repo create AgroBench/programs --private --source=. --remote=origin --push
```

Depois, no profile da org (para quem abre `github.com/AgroBench`):

```bash
# se ainda não existir o repo de perfil
gh repo create AgroBench/.github --private
# cole este ORG.md em profile/README.md e faça push
```

O clone de trabalho do Felipe continua com vários git side-by-side na raiz. Só esta pasta vira o 5º repo.

## Como o jurado descobre o programa

1. **Org** [github.com/AgroBench](https://github.com/AgroBench) — hoje vê os 4 repos antigos. Sem profile README / sem `AgroBench/programs`, o programa **não aparece**.
2. **Depois de criar o repo:** [github.com/AgroBench/programs](https://github.com/AgroBench/programs) — README na raiz (program id + links). Changelog: `O-QUE-FOI-FEITO.md`. Deploy: `PASSO-A-PASSO-DEVNET.md`.
3. **Este `ORG.md`** sobe neste repo; copiar também para o profile da org.

Ordem de leitura (detalhe em `O-QUE-FOI-FEITO.md`): org → `programs/agrobench/src/lib.rs` → no repo [backend](https://github.com/AgroBench/backend) `pkg/adapter/chain/solana/chain.go` → no [front](https://github.com/AgroBench/AgroBenchFront) `src/models/stake.js` → explorer **depois** do deploy.

## O que já estava documentado vs o que este mapa acrescenta

| Já existia (técnico, não é mapa da org) | Acrescentado agora |
|---|---|
| `README.md` — deploy, PDAs, ixs; seção **Para jurados** | — |
| `programs/agrobench/INTERFACE.md` — discriminators / contas | — |
| `DEPLOY-STATUS.md` — programa ainda não está na Devnet | — |
| `PASSO-A-PASSO-DEVNET.md` — o que fazer na mão | — |
| — | `ORG.md` (este arquivo) |
| — | `O-QUE-FOI-FEITO.md` (changelog stake/programa) |
| — | `.gitignore` nesta raiz (cobre `target/`, `.anchor/`, `.env`, `id.json`) |
