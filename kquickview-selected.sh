#!/bin/bash

# 1. Obtener la ventana activa
ACTIVE_WINDOW_ID=$(xdotool getactivewindow 2>/dev/null)
if [ -z "$ACTIVE_WINDOW_ID" ]; then
    exit 0
fi

# Obtener el título de la ventana
ACTIVE_WINDOW_NAME=$(xdotool getwindowname "$ACTIVE_WINDOW_ID" 2>/dev/null)

# 2. Verificar si es Dolphin (el título de la ventana contiene "Dolphin")
if [[ "$ACTIVE_WINDOW_NAME" == *"Dolphin"* ]]; then
    # Guardar el portapapeles previo
    PREV_CLIPBOARD=$(xclip -selection clipboard -o 2>/dev/null)

    # Limpiar portapapeles temporalmente
    xclip -selection clipboard /dev/null

    # Simular Ctrl+C limpiando teclas modificadoras físicas
    xdotool key --clearmodifiers ctrl+c

    # Espera activa inteligente (polling ultrarrápido cada 10ms, máximo 150ms)
    FILE_URI=""
    for i in {1..15}; do
        FILE_URI=$(xclip -selection clipboard -o 2>/dev/null)
        if [[ "$FILE_URI" == file://* ]]; then
            break
        fi
        sleep 0.01
    done

    # Restaurar el portapapeles original
    if [ -n "$PREV_CLIPBOARD" ]; then
        echo -n "$PREV_CLIPBOARD" | xclip -selection clipboard 2>/dev/null
    fi

    # 3. Validar y abrir KQuickView en modo RELEASE (máxima velocidad)
    if [[ "$FILE_URI" == file://* ]]; then
        # Remover prefijo file:// y decodificar caracteres especiales como %20
        FILE_PATH=$(echo "$FILE_URI" | sed 's|^file://||' | python3 -c "import sys, urllib.parse; print(urllib.parse.unquote(sys.stdin.read().strip()))")
        
        if [ -f "$FILE_PATH" ] || [ -d "$FILE_PATH" ]; then
            /home/juan/Documentos/Dev/Apps/KQuickView/target/release/kquickview "$FILE_PATH"
        fi
    fi
fi
