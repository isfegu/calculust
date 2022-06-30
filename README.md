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

* debes instalar Rust (en su versión estable). Es recomendable hacerlo mediante [rustup](https://rustup.rs/).
* puedes utilizar Visual Studio Code. En caso de utilizarlo tienes un _[Workspace](calculust.code-workspace)_ y deberás instalar las extensiones recomendadas. Puedes ver la configuración de Visual Studio Code en la carpeta _[.vscode](/.vscode)_

### 2.- Conventional Commits

Todo _commit_ debe seguir Conventional Commits.

### 3.- Pruebas unitarias

Tratar en medida de lo posible y del sentido común, cubrir la mayor parte del código con pruebas unitarias.

### 4.- _Pull request_

Desarrolla lo que quieras en una rama que parta de _main_ y haz un _pull request_ contra _main_ cuando quieras incorporar los cambios.

Todo _pull request_ desencadena un _workflow_ de github que valida el formato y busca errores en el código y ejecuta las pruebas unitarias. Puedes ver más detalle en el archivo [ci.yml](/.github/workflows/ci.yml).

Solo puede integrase con _main_ los _pull request_ que hayan pasado todas las validaciones del proceso de CI (Integración Continua). La integración se realizará mediante un _rebase and merge_.