# 🛰️ Daemon Receptor Smart - Rust

Servicio receptor asíncrono de alto rendimiento programado en **Rust**, diseñado especialmente para la recepción, descifrado y procesamiento en tiempo real de tramas de red provenientes de centrales de alarmas que utilizan el protocolo **NETIO**.

---

### 1. ⚙️ Características Principales

Este daemon reemplaza y optimiza la lógica del sistema original desarrollado en Visual Basic 6, aportando las siguientes ventajas tecnológicas:

*   **⚡ Concurrencia Total:** Procesamiento asíncrono y concurrente de tramas en hilos independientes mediante `tokio::spawn`. La recepción UDP principal es completamente no bloqueante y robusta ante fallos de red.
*   **🔒 Criptografía AES-128-ECB:** Rutinas de descifrado y cifrado simétrico integradas en `crypto.rs` para asegurar la compatibilidad bidireccional y el correcto envío de confirmaciones (ACKs) al hardware NETIO.
*   **🗄️ Persistencia Dinámica (MySQL):** Creación y mantenimiento automático de tablas mensuales con el esquema exacto de producción histórica (`rep_YYYYM` sin ceros a la izquierda, por ejemplo: `rep_20265`).
*   **🧠 Mapeo Contact ID e Índices Corregidos:** Decodificación alineada al 100% con los offsets de la central (offsets reales basados en la lógica original `base = 5`).
*   **🛡️ Reglas de Negocio Robustas:** Conversión y normalización de letras de eventos (ej. `A` ➡️ `999`), prefijos de restauración automáticos (`REESTABLECE-->` para eventos que inician con `3`), y gestión de eventos cableados especiales (fallos de 220V, batería, keep alive, sirenas) asignándoles la severidad correspondiente (`Evento` o `AlarmaN`).

---

### 2. 📁 Estructura del Código Fuente

El proyecto está organizado de manera modular en los siguientes archivos dentro de la carpeta `src/`:

| Archivo | Responsabilidad |
| :--- | :--- |
| [src/main.rs](file:///home/juan/Documentos/Dev/Apps/Ipcom/receptor/src/main.rs) | Bucle principal UDP con tolerancia a fallos y delegación de tareas concurrentes. |
| [src/database.rs](file:///home/juan/Documentos/Dev/Apps/Ipcom/receptor/src/database.rs) | Lógica de persistencia, consultas de usuarios, tablas mensuales dinámicas e inserciones. |
| [src/crypto.rs](file:///home/juan/Documentos/Dev/Apps/Ipcom/receptor/src/crypto.rs) | Funciones criptográficas para cifrado y descifrado de tramas utilizando AES-128-ECB. |
| [src/config.rs](file:///home/juan/Documentos/Dev/Apps/Ipcom/receptor/src/config.rs) | Inicialización y mapeo de variables de entorno desde el archivo `.env`. |
| [src/models.rs](file:///home/juan/Documentos/Dev/Apps/Ipcom/receptor/src/models.rs) | Estructuras de datos del modelo, incluyendo el mapeo de eventos. |

---

### 3. 🚀 Guía de Configuración y Despliegue

#### Configuración del Entorno
Antes de ejecutar el daemon, asegurate de tener un archivo local [.env](file:///home/juan/Documentos/Dev/Apps/Ipcom/receptor/.env) con la siguiente estructura:

```text
DATABASE_URL=mysql://usuario:password@localhost/nombre_base_datos
LISTEN_PORT=9000
AES_KEY=tu_clave_hexadecimal_de_32_caracteres_16_bytes
```

#### Comandos Útiles de Compilación
Podés administrar el ciclo de vida del servicio utilizando la herramienta **Cargo** de Rust:

```bash
# Compilar el daemon en modo desarrollo
cargo build

# Ejecutar el receptor localmente
cargo run

# Correr los tests unitarios y de integración
cargo test

# Compilar una versión optimizada para producción (Release)
cargo build --release
```

---

### 4. 🧠 Desarrollado por
*   **Juan Gabriel Maioli** — *Desarrollador y Propietario del Proyecto*
*   *Antigravity AI* — *Asistente de Codificación*
