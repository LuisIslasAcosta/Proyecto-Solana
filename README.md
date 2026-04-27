# Agencia de Motocicletas en Solana

Aplicación descentralizada desarrollada en Solana utilizando Rust y Anchor.

## Descripción

Este proyecto permite gestionar una agencia de motocicletas mediante operaciones CRUD almacenadas en la blockchain.

Se pueden registrar motocicletas con los siguientes datos:

- Modelo
- Marca
- Precio
- Stock
- Estado (disponible o no disponible)

## Funcionalidades

- Crear agencia
- Agregar motocicletas
- Eliminar motocicletas
- Actualizar stock
- Activar o desactivar disponibilidad
- Consultar inventario

## Tecnologías

- Rust
- Anchor
- Solana

## Ejecución

```bash
anchor build
anchor deploy
anchor test