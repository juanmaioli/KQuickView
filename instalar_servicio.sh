#!/bin/bash

# ==============================================================================
# Script de Instalación de Servicio Systemd para Receptor Smart Rust (Debian 13)
# Desarrollado por Juan Gabriel Maioli
# ==============================================================================

# Colores para la salida en consola
VERDE='\033[0;32m'
ROJO='\033[0;31m'
AZUL='\033[0;34m'
AMARILLO='\033[1;33m'
SIN_COLOR='\033[0m'

echo -e "${AZUL}======================================================================${SIN_COLOR}"
echo -e "${AZUL}🛰️  Instalador de Servicio Systemd - Receptor Smart (Debian 13)       ${SIN_COLOR}"
echo -e "${AZUL}======================================================================${SIN_COLOR}"

# 1. Autodetección de la ruta física absoluta del script
DIR_ACTUAL=$(dirname "$(readlink -f "$0")")
echo -e "📂 Directorio detectado: ${AMARILLO}$DIR_ACTUAL${SIN_COLOR}"

# 2. Validar presencia del binario y de la configuración
if [ ! -f "$DIR_ACTUAL/receptor" ]; then
    echo -e "${ROJO}❌ Error: No se encontró el binario 'receptor' en este directorio.${SIN_COLOR}"
    echo -e "💡 Asegurate de compilar el proyecto en modo release con 'cargo build --release' y copiar el binario 'target/release/receptor' a esta carpeta antes de correr este script."
    exit 1
fi

if [ ! -f "$DIR_ACTUAL/.env" ]; then
    echo -e "${ROJO}❌ Error: No se encontró el archivo de configuración '.env' en este directorio.${SIN_COLOR}"
    echo -e "💡 Creá o copiá tu archivo '.env' con las variables adecuadas antes de continuar."
    exit 1
fi

# 3. Validar si se ejecuta como root / sudo para la instalación del servicio
if [ "$EUID" -ne 0 ]; then
    echo -e "${AMARILLO}🔑 Este script necesita privilegios de administrador para configurar systemd.${SIN_COLOR}"
    echo -e "🔄 Reintentando ejecución con sudo..."
    exec sudo "$0" "$@"
fi

# Obtener el nombre del usuario real que invocó sudo para asignarlo al servicio
USUARIO_REAL=${SUDO_USER:-$USER}
if [ "$USUARIO_REAL" = "root" ]; then
    USUARIO_REAL="juan" # Fallback por defecto si se corre directo como root
fi

echo -e "👤 El servicio se configurará bajo el usuario: ${AMARILLO}$USUARIO_REAL${SIN_COLOR}"

# 4. Creación dinámica del archivo de servicio para systemd
SERVICE_PATH="/etc/systemd/system/receptor.service"

echo -e "⚙️  Creando archivo de unidad de servicio en ${AMARILLO}$SERVICE_PATH${SIN_COLOR}..."

cat <<EOF > $SERVICE_PATH
[Unit]
Description=Receptor Smart Daemon (Rust asíncrono y concurrente para NETIO)
After=network.target mysql.service mariadb.service

[Service]
Type=simple
User=$USUARIO_REAL
WorkingDirectory=$DIR_ACTUAL
ExecStart=$DIR_ACTUAL/receptor
Restart=always
RestartSec=5
AmbientCapabilities=CAP_NET_BIND_SERVICE
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=receptor-smart

[Install]
WantedBy=multi-user.target
EOF

# 5. Recarga de la configuración del gestor de servicios de systemd
echo -e "🔄 Recargando daemon de systemd..."
systemctl daemon-reload

# 6. Habilitar el servicio para el arranque automático
echo -e "🔌 Habilitando el servicio en el inicio del sistema..."
systemctl enable receptor.service

# 7. Preguntar de forma interactiva si se desea iniciar el servicio inmediatamente
echo -e "${VERDE}✅ ¡Servicio 'receptor.service' configurado e instalado con éxito!${SIN_COLOR}"
echo -e "${AZUL}----------------------------------------------------------------------${SIN_COLOR}"
read -p "¿Querés iniciar el servicio ahora mismo? (S/n): " INICIAR
INICIAR=${INICIAR:-S}

if [[ "$INICIAR" =~ ^[Ss]$ ]]; then
    echo -e "🚀 Iniciando servicio receptor..."
    systemctl start receptor.service
    sleep 1
    systemctl status receptor.service --no-pager
else
    echo -e "ℹ️  Servicio configurado. Podés iniciarlo manualmente ejecutando:"
    echo -e "   ${AMARILLO}sudo systemctl start receptor.service${SIN_COLOR}"
fi

echo -e "${AZUL}======================================================================${SIN_COLOR}"
EOF
