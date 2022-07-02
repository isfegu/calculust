# Usar

## Como módulo WebAssembly

### Compilación

```bash
~calculust cargo build --target wasm32-unknown-unknown
```

El resultado de la compilación se puede encontrar en `target/wasm32-unknown-unknown/debug/`. Esta compilación es en modo _debug_, esto significa sin optimizar y con información de _debug_. Si se quiere tener el módulo sin todo el código extra de _debug_ deberemos ejectutar:

```bash
~calculust cargo build --release --target wasm32-unknown-unknown
```

El resultado de la compilación se puede encontrar en `target/wasm32-unknown-unknown/release/`.

### Ejecución

#### Wasmer

```bash
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i add 3 8 // Nos devolverá 11
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i sub 3 8 // Nos devolverá -5
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i mul 3 8 // Nos devolverá 24
~calculust/target/wasm32-unknown-unknown/release/ wasmer run calculust.wasm -i div 10 5 // Nos devolverá 2
```

#### Navegador

```html
<script>
    let importObject = {};
    WebAssembly.instantiateStreaming(fetch('./calculust.wasm'), importObject)
        .then(obj => {
            const addResultContainer = document.getElementById("add");
            addResultContainer.innerHTML = obj.instance.exports.add(5, 7);

            const subResultContainer = document.getElementById("sub");
            subResultContainer.innerHTML = obj.instance.exports.sub(15, 9);

            const mulResultContainer = document.getElementById("mul");
            mulResultContainer.innerHTML = obj.instance.exports.mul(5, 7);

            const divResultContainer = document.getElementById("div");
            divResultContainer.innerHTML = obj.instance.exports.div(15, 5);
        });
</script>
```

### Demo

Antes de ejecutar la demo es necesario hacer lo siguiente:

* [Compilar Calculust](#compilación)
* Copiar el resultado de la compilación al directorio donde está la demo.

Un [Makefile](../Makefile) facilita esta tarea, simplemente debes ejecutar `make demo` y se compilará __Calculust__ y copiarán los archivos adecuados para poder ejecutar la demo.

  ```bash
  ~calculust make demo
  ```

Para ver cómo funciona Calculust integrado desde una página HTML cargando el módulo WebAssembly directamente mediante Javascript, debes ir a [demo/html.js.wasm](demo/html.js.wasm) e iniciar un servidor web, por ejemplo con Python (que viene instalado en la mayoría de distribuciones Linux):

```bash
~calculust/demo/html.js.wasm python3 -m http.server 8000
```

Ahora solo tenemos que abrir la URL [http://127.0.0.1:8000/](http://127.0.0.1:8000/) en el navegador.
