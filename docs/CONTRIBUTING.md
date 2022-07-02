# Contribuir

Para contribuir a Calculust debes leerte todo este documento y seguir las siguiente instrucciones:

1. Instalar todos los requisitos
2. Usar [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
3. Escribir pruebas unitarias
4. Hacer un _pull request_ a _main_ con tus cambios

## 1.- Requisitos

Para desarrollar Calculust:

* debes instalar Rust.
* debes instalar _runtime_ de WebAssembly.
* puedes utilizar Visual Studio Code. En caso de utilizarlo tienes un _[Workspace](calculust.code-workspace)_ y deberás instalar las extensiones recomendadas. Puedes ver la configuración de Visual Studio Code en la carpeta _[.vscode](/.vscode)_

### Rust

Para instalar Rust (en su versión estable), lo mejor es hacerlo mediante [rustup](https://rustup.rs/).

Validar que tenemos instalados el _target_ necesario para compilar a WebAssembly. Al ejecutar:

```bash
~ rustup target list
```

debemos ver __wasm32-unknown-unknown__ y __wasm32-wasi__ instalados. En caso que no estén instalados, ejecutar:

```bash
~ rustup target add wasm32-unknown-unknown
~ rustup target add wasm32-wasi
```

### Runtime de WebAssembly

Para poder ejecutar WebAssembly desde línea de comando, instalaremos [Wasmer](https://wasmer.io/).

Al menos en Ubuntu 20.04, es necesario instalar antes libtinfo5:

```bash
~ apt install libtinfo5
```

```bash
~ curl https://get.wasmer.io -sSfL | sh
```

## 2.- Conventional Commits

Todo _commit_ debe seguir Conventional Commits.

## 3.- Pruebas unitarias

Tratar en medida de lo posible y del sentido común, cubrir la mayor parte del código con pruebas unitarias.

La pruebas unitarias se ejecutan mediante:

```bash
~calculust cargo test
```

## 4.- _Pull request_

Desarrolla lo que quieras en una rama que parta de _main_ y haz un _pull request_ contra _main_ cuando quieras incorporar los cambios.

Todo _pull request_ desencadena un _workflow_ de github que valida el formato y busca errores en el código y ejecuta las pruebas unitarias. Puedes ver más detalle en el archivo [ci.yml](/.github/workflows/ci.yml).

Solo puede integrase con _main_ los _pull request_ que hayan pasado todas las validaciones del proceso de CI (Integración Continua). La integración se realizará mediante un _rebase and merge_.
