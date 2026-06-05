# 🚀 KQuickView

[Repositorio en GitHub](https://github.com/juanmaioli/KQuickView)

Una herramienta de previsualización ultrarrápida y liviana para **KDE Plasma** (estilo macOS Quick Look), desarrollada en **Rust** y **Qt6/QML**.

---

## 1. 🛠️ Características

*   ⚡ **Rendimiento Máximo:** Escrita en Rust para un procesamiento de archivos instantáneo.
*   🎨 **Interfaz Integrada:** Diseñada en QML con estética oscura y fluida que se adapta a KDE.
*   ⌨️ **Control por Teclado:** Navegación entre archivos adyacentes de la misma carpeta con las flechas de dirección (◄/►/▲/▼) y cierre rápido presionando `Esc` o `Meta+Espacio`.

| Formato | Tipo de Archivo | Características de Previsualización |
| :--- | :--- | :--- |
| **Imágenes** | `.png`, `.jpg`, `.jpeg`, `.webp`, `.gif`, etc. | Carga asíncrona inteligente. |
| **Texto Plano** | `.txt`, `.json`, `.rs`, `.toml`, `.sh`, etc. | Renderizado rápido con tipografía monoespaciada clara (UTF-8 y fallbacks). |
| **Markdown** | `.md`, `.markdown` | Renderizado rico en HTML con CSS personalizado de tonos KDE. |
| **Documentos PDF** | `.pdf` | Renderizado nativo integrado usando el módulo oficial `QtQuick.Pdf` de forma diferida (lazy loading). |

---

## 2. ⚙️ Requisitos del Sistema

Para compilar y ejecutar KQuickView en sistemas basados en Debian (como Debian 13 "Trixie"):

```bash
sudo apt update
sudo apt install -y qt6-base-dev qt6-declarative-dev qt6-pdf-dev qml6-module-qtquick-pdf pkg-config build-essential
```

---

## 3. 🚀 Compilación e Instalación

Para simplificar el proceso, se incluye un script automatizado `install.sh` que compila la aplicación en modo optimizado y la instala junto con sus componentes en tu directorio de usuario.

1. **Ejecutar el instalador:**
   ```bash
   ./install.sh
   ```

2. **Destinos de instalación de los componentes:**

| Componente | Archivo de Origen | Ruta de Destino en el Sistema |
| :--- | :--- | :--- |
| **Binario ejecutable** | `target/release/kquickview` | `~/.local/bin/kquickview` |
| **Script disparador de atajo** | `kquickview-selected.sh` | `~/.local/bin/kquickview-selected.sh` |
| **Menú contextual (Dolphin)** | `kquickview.desktop` | `~/.local/share/kio/servicemenus/kquickview.desktop` |
| **Acceso directo de aplicación** | `kquickview-app.desktop` | `~/.local/share/applications/kquickview.desktop` |

3. **Configurar el Atajo Global (Meta+Espacio) en KDE Plasma:**
   Dado que Dolphin no permite asignar atajos directos a los Service Menus de forma nativa en todas las versiones:
   * Abrí las **Preferencias del Sistema** de KDE.
   * Navegá a **Atajos** -> **Añadir nuevo** -> **Comando**.
   * Nombre: `KQuickView - Previsualizar`
   * Comando: `/home/juan/.local/bin/kquickview-selected.sh`
   * Asignale la combinación de teclas **Meta+Espacio** (o la combinación que prefieras).

4. **Icono de la Ventana:**
   La aplicación y su menú de Dolphin se asocian automáticamente con el icono de la lupa emoji (`right-pointing_magnifying_glass.png`) configurado en el archivo desktop local.

---

## 4. 👨‍💻 Autor

Desarrollado por **Juan Gabriel Maioli**.
