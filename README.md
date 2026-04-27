# Agencia de Motocicletas en Solana

Aplicación descentralizada desarrollada en Solana utilizando Rust y Anchor.

## Descripción

Este proyecto simula la gestión de una agencia de motocicletas en blockchain, donde se pueden realizar operaciones CRUD para administrar y gestionar un inventario.

Cada motocicleta cuenta con:

* Modelo
* Marca
* Precio
* Stock
* Estado (disponible o no disponible)

## Funcionalidades

* Crear agencia
* Agregar motocicletas
* Eliminar motocicletas
* Actualizar stock
* Activar o desactivar disponibilidad
* Consultar inventario

## Declare_id
**5ZjpnkFbwAaMJonsW7MmcwGzsBwzKRuQ2NJ7SYTAL5hM**

## Tecnologías

* Rust
* Anchor
* Solana

## Desarrollo

El proyecto fue desarrollado en local utilizando Kali Linux en mi laptop.
Los despliegues y pruebas se realizaron desde Solana Playground: https://beta.solpg.io/

## Uso en Solana Playground

Para correr el proyecto desde Playground:

1. Ir a la pestaña **Test**

2. Para crear la agencia:

   * Ingresar el **nombre (String)**
   * En la cuenta seleccionar:

     * From Seed
     * Public Key
     * Current Wallet
   * Ejecutar la función para crear la agencia

3. Para trabajar con motocicletas:

   * Llenar los datos requeridos (modelo, marca, precio, stock)
   * Usar la misma configuración de cuenta que la agencia
   * Utilizar el mismo **Public Key de la agencia (PDA)**

Con esto ya se pueden ejecutar todas las funciones del sistema.