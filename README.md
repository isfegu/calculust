# Calculust

Calculust es una calculadora básica (suma, resta, multiplica y divide) hecha en Rust.

El objetivo de Calculust no es tener una calculadora, sino el aprender cómo partir de una librería programada en Rust para tener un módulo WebAssembly y desplegarlo como servicio _serverless_ usando AWS Lambda.

## Contribuir

Para contribuir a Calculust debes leerte todo este documento y seguir las siguiente instrucciones:

1. Instalar todos los requisitos
2. Usar [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
3. Escribir pruebas unitarias
4. Hacer un _pull request_ a _main_ con tus cambios

### 1.- Requisitos

Para desarrollar Calculust:

* debes instalar Rust (en su versión estable).
* debes instalar las herramientas de WebAssembly.
* puedes utilizar Visual Studio Code. En caso de utilizarlo tienes un _[Workspace](calculust.code-workspace)_ y deberás instalar las extensiones recomendadas. Puedes ver la configuración de Visual Studio Code en la carpeta _[.vscode](/.vscode)_

#### Rust

Para instalar Rust, lo mejor es hacerlo mediante [rustup](https://rustup.rs/).

Validar que tenemos instalados el _target_ necesario para compilar a WebAssembly. Al ejecutar:

```bash
~ rustup target list
```

debemos ver __wasm32-unknown-unknown__ y __wasm32-wasi__ instalados. En caso que no estén instalados, ejecutar:

```bash
~ rustup target add wasm32-unknown-unknown
~ rustup target add wasm32-wasi
```

#### Herramientas de WebAssembly

Puedes instalarlas desde el gestor de paquetes de tu distribución:

```bash
~ sudo apt install wabt
```

o desde la página de [descargas oficial](https://github.com/WebAssembly/wabt/releases).

#### Wasmer

Antes de instalar [Wasmer](https://wasmer.io/), al menos en Ubuntu 20.04, es necesario instalar antes libtinfo5:

```bash
~ apt install libtinfo5
```

```bash
~ curl https://get.wasmer.io -sSfL | sh
```

### 2.- Conventional Commits

Todo _commit_ debe seguir Conventional Commits.

### 3.- Pruebas unitarias

Tratar en medida de lo posible y del sentido común, cubrir la mayor parte del código con pruebas unitarias.

La pruebas unitarias se ejecutan mediante:

```bash
~calculust cargo test
```

### 4.- _Pull request_

Desarrolla lo que quieras en una rama que parta de _main_ y haz un _pull request_ contra _main_ cuando quieras incorporar los cambios.

Todo _pull request_ desencadena un _workflow_ de github que valida el formato y busca errores en el código y ejecuta las pruebas unitarias. Puedes ver más detalle en el archivo [ci.yml](/.github/workflows/ci.yml).

Solo puede integrase con _main_ los _pull request_ que hayan pasado todas las validaciones del proceso de CI (Integración Continua). La integración se realizará mediante un _rebase and merge_.

## Uso

### Compilación

#### Como módulo WebAssembly

```bash
~calculust cargo build --target wasm32-unknown-unknown
```

El resultado de la compilación se puede encontrar en `target/wasm32-unknown-unknown/debug/`. Esta compilación es en modo _debug_, esto significa sin optimiar y con información de _debug_. Si se quiere tener el módulo sin todo el código extra de _debug_ deberemos ejectutar:

```bash
~calculust cargo build --release --target wasm32-unknown-unknown
```

El resultado de la compilación se puede encontrar en `target/wasm32-unknown-unknown/release/`.

### Ejecución

#### Como módulo WebAssembly

Podemos ejecutar __Calculust__ como módulo WebAssembly mediante __Wasmer__:

```bash
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i add 3 8 // Nos devolverá 11
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i sub 3 8 // Nos devolverá -5
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i mul 3 8 // Nos devolverá 24
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i div 10 5 // Nos devolverá 2
```
