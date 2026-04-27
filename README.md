# Refaccionaria en Solana

Aplicación descentralizada desarrollada en Solana utilizando Rust y Anchor.

## Descripción

Este proyecto permite gestionar una refaccionaria automotriz mediante operaciones CRUD almacenadas en la blockchain.

Se pueden registrar refacciones con los siguientes datos:

* Nombre
* Precio
* Stock
* Estado (activo o inactivo)

La información se almacena utilizando Program Derived Addresses (PDA), lo que permite manejar cuentas seguras sin necesidad de llaves privadas adicionales.

## Funcionalidades

* Crear refaccionaria
* Agregar refacciones
* Eliminar refacciones
* Actualizar stock
* Activar o desactivar refacciones
* Consultar inventario

## Tecnologías

* Rust
* Anchor
* Solana

## Ejecución del proyecto

1. Clonar el repositorio

```bash
git clone https://github.com/TU_USUARIO/TU_REPO.git
cd TU_REPO
```

2. Compilar el programa

```bash
anchor build
```

3. Desplegar el programa

```bash
anchor deploy
```

4. Ejecutar pruebas

```bash
anchor test
```

## Ejemplo de uso

Se crea una refaccionaria y se agregan refacciones con su precio y cantidad disponible. Posteriormente se pueden actualizar los datos, eliminar registros o consultar el estado del inventario.

## Autor

Proyecto desarrollado como parte del proceso de certificación en Solana.
