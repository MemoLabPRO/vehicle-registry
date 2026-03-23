# Vehicle Registry — Programa en Solana

Un registro on-chain de títulos vehiculares construido con Anchor sobre Solana.  
Cada vehículo es una PDA derivada de su VIN — inmutable, verificable y transferible.

## El problema

En las ventas P2P de vehículos no existe una forma confiable de verificar la propiedad antes del pago.  
Los compradores dependen de títulos en papel que pueden falsificarse, duplicarse o llegar con retraso.  
Este programa coloca la propiedad vehicular on-chain: consultable, transferible y composable con contratos de venta.

## La solución

Un programa de Solana que registra vehículos como cuentas PDA (Program Derived Accounts) usando el VIN como semilla.  
Cualquier contrato de venta — incluyendo un contrato P2P con cobro de impuestos — puede llamar a `transfer_title` mediante CPI para actualizar la propiedad de forma atómica junto con el pago.

## Program ID (Devnet)

```
3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3
```

🔍 [Ver en Solana Explorer](https://explorer.solana.com/address/3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3?cluster=devnet)

## Instrucciones

| Instrucción | Quién la llama | Qué hace |
|---|---|---|
| `register_vehicle` | Propietario del vehículo | Crea una cuenta PDA para un VIN y establece el propietario inicial |
| `update_status` | Propietario actual | Cambia el estado: `Active`, `ForSale` o `Stolen` |
| `transfer_title` | Propietario o contrato de venta via CPI | Transfiere la propiedad a una nueva wallet y regresa el estado a `Active` |

## Estructura de la cuenta

```rust
pub struct VehicleRecord {
    pub vin: [u8; 17],        // Número de identificación vehicular
    pub owner: Pubkey,         // Wallet del propietario actual
    pub make: String,          // Ej. "Honda"
    pub model: String,         // Ej. "Civic"
    pub year: u16,             // Ej. 2021
    pub status: VehicleStatus, // Active | ForSale | Stolen
    pub registered_at: i64,    // Timestamp Unix
    pub bump: u8,              // Bump seed de la PDA
}
```

## Derivación de la PDA

```
seeds = ["vehicle", vin_bytes]
program_id = 3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3
```

Dado cualquier VIN, cualquier persona puede derivar la dirección on-chain de forma determinista — sin necesidad de un indexador externo.

## Por qué esto importa para Solana

Este programa es un **primitivo** — un bloque de construcción.  
Está diseñado para ser compuesto con otros programas mediante CPI:

- Un contrato de venta P2P puede llamar a `transfer_title` de forma atómica junto con el pago en SOL
- Un contrato de financiamiento puede bloquear el estado mientras un crédito está activo
- Un oráculo de inspección puede escribir reportes certificados en la misma cuenta

Un solo registro. Múltiples programas construidos encima.

## Desarrollo local

```bash
# Instalar dependencias
yarn install

# Compilar
anchor build

# Ejecutar tests
anchor test

# Desplegar en devnet
anchor deploy
```

## Tests

```
✔ Registra un vehículo
✔ Cambia estado a ForSale
✔ Transfiere el título a nuevo propietario

3 passing (1s)
```

## Stack tecnológico

- Rust · Anchor 0.32.1
- Solana CLI 3.0.15
- TypeScript · Mocha (tests)

## Autor

Guillermo (Memo) — Solana Bootcamp · WayLearn Latam  
GitHub: [@MemoLabPRO](https://github.com/MemoLabPRO)
