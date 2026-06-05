# 🚀 KQuickView

Una herramienta de previsualización ultrarrápida y liviana para **KDE Plasma** (estilo macOS Quick Look), desarrollada en **Rust** y **Qt6/QML**.

---

## 🛠️ Características
*   ⚡ **Rendimiento Máximo:** Escrita en Rust para un procesamiento de archivos instantáneo.
*   🎨 **Interfaz Integrada:** Diseñada en QML con estética oscura y fluida que se adapta a KDE.
*   📄 **Formatos Soportados:** 
    *   **Imágenes:** Carga asíncrona inteligente.
    *   **Texto Plano:** Renderizado rápido con tipografía monoespaciada clara (soporta UTF-8 y fallbacks).
    *   **PDF:** Renderizado nativo integrado usando el módulo oficial `QtQuick.Pdf`.
*   ⌨️ **Control por Teclado:** Navegación entre archivos adyacentes de la misma carpeta con las flechas de dirección (◄/►/▲/▼) y cierre rápido presionando `Esc` o `Meta+Espacio`.

---

## ⚙️ Requisitos del Sistema
Para compilar y ejecutar KQuickView en sistemas basados en Debian (como Debian 13 "Trixie"):

```bash
sudo apt update
sudo apt install -y qt6-base-dev qt6-declarative-dev qt6-pdf-dev qml6-module-qtquick-pdf pkg-config build-essential
```

---

## 🚀 Compilación e Instalación

1. **Compilar el binario:**
   ```bash
   cargo build --release
   ```

2. **Integrar con Dolphin:**
   El archivo de Dolphin Service Menu se instala copiándolo a tu directorio de servicios local:
   ```bash
   mkdir -p ~/.local/share/kio/servicemenus/
   cp kquickview.desktop ~/.local/share/kio/servicemenus/
   ```

3. **Configurar el Atajo Global (Meta+Espacio):**
   Dado que Dolphin no permite asignar atajos a Service Menus en todas las versiones:
   * Ve a **Preferencias del Sistema de KDE** -> **Atajos** -> **Añadir nuevo** -> **Comando**.
   * Nombre: `KQuickView - Previsualizar`
   * Comando: `/home/juan/Documentos/Dev/Apps/KQuickView/kquickview-selected.sh`
   * Asignale el atajo **Meta+Espacio** (o la combinación que prefieras).
   
   El script detectará automáticamente si estás en Dolphin y obtendrá el archivo seleccionado de forma segura.

---

## 👨‍💻 Autor
Desarrollado por **Juan Gabriel Maioli**.
