# AgroBench — interface on-chain (congelada)

Program name: `agrobench`  
Program id: `EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN`  
Cluster: Devnet  
USDC mint (Circle Devnet): `4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU`  
Token program: `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`  
ATA program: `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`

Discriminator Anchor: `sha256("global:<ixname>")[0:8]`.  
Account discriminator: `sha256("account:<Name>")[0:8]`.

Confirme após `anchor build` no IDL (`target/idl/agrobench.json` → `instructions[].discriminator`).

## Discriminators das instruções

| ix | bytes (Go) | hex |
|---|---|---|
| `initialize` | `[]byte{0xaf, 0xaf, 0x6d, 0x1f, 0x0d, 0x98, 0x9b, 0xed}` | `afaf6d1f0d989bed` |
| `lock_stake` | `[]byte{0x6f, 0xba, 0xaf, 0xe4, 0x31, 0xa5, 0x1b, 0xf8}` | `6fbaafe431a51bf8` |
| `release_stake` | `[]byte{0x33, 0x05, 0x1c, 0xfa, 0xb9, 0xa8, 0x12, 0x35}` | `33051cfab9a81235` |
| `credit_pool` | `[]byte{0x2c, 0x51, 0x1f, 0xf9, 0x7b, 0x2f, 0xb0, 0xfe}` | `2c511ff97b2fb0fe` |
| `distribute` | `[]byte{0xbf, 0x2c, 0xdf, 0xcf, 0xa4, 0xec, 0x7e, 0x3d}` | `bf2cdfcfa4ec7e3d` |

## Discriminators das contas

| account | bytes (Go) | hex |
|---|---|---|
| `Pool` | `[]byte{0xf1, 0x9a, 0x6d, 0x04, 0x11, 0xb1, 0x6d, 0xbc}` | `f19a6d0411b16dbc` |
| `Stake` | `[]byte{0x96, 0xc5, 0xb0, 0x1d, 0x37, 0x84, 0x70, 0x95}` | `96c5b01d37847095` |

## PDAs

```
pool  = find(["pool"], program_id)
stake = find(["stake", producer], program_id)
pool_ata  = ATA(pool,  usdc_mint)
stake_ata = ATA(stake, usdc_mint)
```

Seeds literais ASCII: `pool`, `stake`.

## Dados

```
Pool  = disc[8] || authority[32] || usdc_mint[32] || bump[1]
Stake = disc[8] || producer[32]  || amount_le[8]  || bump[1]
```

`amount` é u64 little-endian, 6 decimais. 10 USDC = `10000000`.

## Encoding das ixs (data)

Prefixo: 8 bytes de discriminator. Depois:

| ix | resto (Borsh / packed) |
|---|---|
| `initialize` | (vazio) |
| `lock_stake` | `u64` LE `amount` |
| `release_stake` | (vazio) |
| `credit_pool` | `u64` LE `amount` |
| `distribute` | `u32` LE `len` + `len` × `u64` LE |

## Ordem das contas (Anchor)

`is_signer` / `is_writable` como o runtime exige. Programas (system/token/ata) **não** assinam.

### `initialize`

```
0 authority                 signer, writable
1 pool                      writable, PDA ["pool"]
2 usdc_mint
3 pool_ata                  writable, ATA(pool, usdc_mint)
4 system_program
5 token_program
6 associated_token_program
7 rent
```

### `lock_stake`

```
0 producer                  signer
1 authority                 signer, writable   // fee payer
2 stake                     writable, PDA ["stake", producer]
3 producer_ata              writable
4 stake_ata                 writable, ATA(stake, usdc_mint)
5 usdc_mint
6 token_program
7 associated_token_program
8 system_program
```

Data: `disc || amount_u64_le`

### `release_stake`

```
0 producer                  signer
1 authority                 signer
2 stake                     writable, PDA ["stake", producer]
3 producer_ata              writable
4 stake_ata                 writable
5 token_program
```

### `credit_pool`

```
0 authority                 signer            // == Pool.authority
1 pool                      PDA ["pool"]
2 from_ata                  writable          // treasury USDC, owner == authority
3 pool_ata                  writable          // owner == pool
4 token_program
```

Data: `disc || amount_u64_le`

### `distribute`

```
0 authority                 signer            // == Pool.authority
1 pool                      PDA ["pool"]
2 pool_ata                  writable
3 token_program
4..n dest_ata               writable          // remaining_accounts, 1:1 com amounts
```

Data: `disc || u32_le(len) || amounts...`

Cada dest ATA deve ser marcada **writable** no `AccountMeta`. O programa transfere da `pool_ata` (authority = PDA `pool`, signed via seeds `["pool"]`).

## Erros

Anchor custom a partir de `0x1770` (6000):

| code | nome |
|---|---|
| 6000 | InsufficientStake |
| 6001 | Unauthorized |
| 6002 | LengthMismatch |
| 6003 | Overflow |

## Constantes Go sugeridas

```go
const (
    ProgramIDDevnet = "EytN8UaXrfTQc6Pq4AdQbQyJwUX37ddXsV7URayBBLrN"
    UsdcMintDevnet  = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
    Stake10USDC     = uint64(10_000_000)
)

var (
    IxInitialize   = []byte{0xaf, 0xaf, 0x6d, 0x1f, 0x0d, 0x98, 0x9b, 0xed}
    IxLockStake    = []byte{0x6f, 0xba, 0xaf, 0xe4, 0x31, 0xa5, 0x1b, 0xf8}
    IxReleaseStake = []byte{0x33, 0x05, 0x1c, 0xfa, 0xb9, 0xa8, 0x12, 0x35}
    IxCreditPool   = []byte{0x2c, 0x51, 0x1f, 0xf9, 0x7b, 0x2f, 0xb0, 0xfe}
    IxDistribute   = []byte{0xbf, 0x2c, 0xdf, 0xcf, 0xa4, 0xec, 0x7e, 0x3d}
)
```
