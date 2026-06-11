#!/bin/bash

# Evitar que el script falle silenciosamente
set -e

# Colores para la salida
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # Sin color

echo -e "${BLUE}=== Iniciando instalación de KQuickView ===${NC}"

# 1. Compilar el proyecto en modo Release
echo -e "${BLUE}[1/5] Compilando KQuickView en modo Release...${NC}"
cargo build --release

# 2. Crear directorios de destino si no existen
echo -e "${BLUE}[2/5] Creando directorios del sistema de usuario...${NC}"
mkdir -p "$HOME/.local/bin"
mkdir -p "$HOME/.local/share/kio/servicemenus"
mkdir -p "$HOME/.local/share/applications"
mkdir -p "$HOME/.local/share/icons"

# 3. Copiar ejecutables y dar permisos
echo -e "${BLUE}[3/5] Instalando binarios e iconos...${NC}"
cp target/release/kquickview "$HOME/.local/bin/kquickview"
chmod +x "$HOME/.local/bin/kquickview"

# Copiar e instalar el icono de la lupa en el sistema
cp right-pointing_magnifying_glass.png "$HOME/.local/share/icons/kquickview.png"

# 4. Configurar e instalar el menú contextual de Dolphin (Service Menu)
echo -e "${BLUE}[4/5] Instalando el menú contextual para Dolphin...${NC}"
# Reemplazar la ruta genérica de Exec por la ruta absoluta instalada del usuario para evitar fallas del PATH
sed "s|Exec=kquickview %f|Exec=$HOME/.local/bin/kquickview %f|g" kquickview.desktop > "$HOME/.local/share/kio/servicemenus/kquickview.desktop"
chmod +x "$HOME/.local/share/kio/servicemenus/kquickview.desktop"

# 5. Configurar e instalar la entrada de aplicación de escritorio (.desktop)
echo -e "${BLUE}[5/5] Registrando la aplicación en el sistema...${NC}"
sed "s|Exec=__EXEC_PATH__|Exec=$HOME/.local/bin/kquickview|g" kquickview-app.desktop > "$HOME/.local/share/applications/kquickview.desktop"
chmod +x "$HOME/.local/share/applications/kquickview.desktop"

echo -e "${GREEN}=== ¡Instalación finalizada con éxito! ===${NC}"
echo -e "${YELLOW}Notas importantes:${NC}"
echo -e "1. Asegurate de que ${GREEN}~/.local/bin${NC} esté en tu variable PATH para ejecutar 'kquickview' desde la terminal."
echo -e "2. Para configurar el atajo de teclado global en KDE Plasma:"
echo -e "   - Andá a 'Preferencias del Sistema' -> 'Atajos' -> 'Añadir nuevo' -> 'Comando'."
echo -e "   - Asigná el comando: ${GREEN}$HOME/.local/bin/kquickview${NC}"
echo -e "   - Configurá la combinación de teclas: ${GREEN}Meta+Espacio${NC}"
