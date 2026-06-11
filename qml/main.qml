import QtQuick
import QtQuick.Controls
import kquickview 1.0

Window {
    id: window
    visible: true
    width: 800
    height: 600
    title: bridge.file_path ? "KQuickView - " + bridge.file_path.split('/').pop() : "KQuickView"
    color: "#121212"
    flags: Qt.WindowStaysOnTopHint

    KQuickViewBridge {
        id: bridge
    }

    // Atajos de teclado para cerrar rápido
    Shortcut {
        sequence: "Escape"
        onActivated: Qt.quit()
    }
    Shortcut {
        sequence: "Meta+Space"
        onActivated: Qt.quit()
    }

    // Atajos de navegación con flechas de cursor
    Shortcut {
        sequence: "Left"
        context: Qt.ApplicationShortcut
        onActivated: bridge.previous_file()
    }
    Shortcut {
        sequence: "Up"
        context: Qt.ApplicationShortcut
        onActivated: bridge.previous_file()
    }
    Shortcut {
        sequence: "Right"
        context: Qt.ApplicationShortcut
        onActivated: bridge.next_file()
    }
    Shortcut {
        sequence: "Down"
        context: Qt.ApplicationShortcut
        onActivated: bridge.next_file()
    }

    // Cabecera superior premium
    Rectangle {
        id: header
        width: parent.width
        height: 48
        color: "#1a1a1a"
        border.color: "#2a2a2a"
        border.width: 1
        anchors.top: parent.top
        z: 10

        Row {
            anchors.left: parent.left
            anchors.leftMargin: 12
            anchors.verticalCenter: parent.verticalCenter
            spacing: 8

            // Botón para abrir con la app predeterminada
            Button {
                text: "Abrir con aplicación predeterminada"
                font.pointSize: 10
                flat: true
                palette.buttonText: "#3daee9" // Color acento azul KDE
                onClicked: {
                    if (bridge.file_url) {
                        Qt.openUrlExternally(bridge.file_url)
                        Qt.quit() // Cerrar al abrir
                    }
                }
            }
        }

        Text {
            text: bridge.file_path ? bridge.file_path.split('/').pop() : ""
            color: "#eff0f1"
            font.bold: true
            font.pointSize: 11
            anchors.centerIn: parent
            elide: Text.ElideMiddle
            width: parent.width * 0.4
            horizontalAlignment: Text.AlignHCenter
        }

        Text {
            text: "Navegar con ◄ / ► / ▲ / ▼  |  Cerrar con Esc o Meta+Espacio"
            color: "#7f8c8d"
            font.pointSize: 9
            anchors.right: parent.right
            anchors.rightMargin: 16
            anchors.verticalCenter: parent.verticalCenter
        }
    }

    // Contenedor principal de contenido
    Item {
        anchors.top: header.bottom
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right

        // Loader dinámico para Lazy Loading (evita cargar QtQuick.Pdf si no se visualizan PDFs)
        Loader {
            id: contentLoader
            anchors.fill: parent
            
            source: {
                if (bridge.file_type === "image") return "ImageViewer.qml";
                if (bridge.file_type === "text") return "TextViewer.qml";
                if (bridge.file_type === "pdf") return "PdfViewer.qml";
                if (bridge.file_type === "markdown") return "MarkdownViewer.qml";
                return "";
            }
        }

        // Vista de Fallback / Archivo no Soportado (sólo si no es un tipo conocido)
        Column {
            anchors.centerIn: parent
            spacing: 16
            visible: bridge.file_type === "unknown"

            Text {
                text: "📁 Previsualización no disponible"
                color: "#bdc3c7"
                font.pointSize: 16
                font.bold: true
                horizontalAlignment: Text.AlignHCenter
                anchors.horizontalCenter: parent.horizontalCenter
            }

            Text {
                text: bridge.file_path ? "Archivo: " + bridge.file_path : ""
                color: "#7f8c8d"
                font.pointSize: 11
                horizontalAlignment: Text.AlignHCenter
                anchors.horizontalCenter: parent.horizontalCenter
                wrapMode: Text.Wrap
                width: 600
            }

            Text {
                text: bridge.text_content
                color: "#e74c3c"
                font.pointSize: 10
                horizontalAlignment: Text.AlignHCenter
                anchors.horizontalCenter: parent.horizontalCenter
                wrapMode: Text.Wrap
                width: 600
            }

            Button {
                text: "Abrir archivo directamente"
                anchors.horizontalCenter: parent.horizontalCenter
                onClicked: {
                    if (bridge.file_url) {
                        Qt.openUrlExternally(bridge.file_url)
                        Qt.quit()
                    }
                }
            }
        }
    }

    Component.onCompleted: {
        window.requestActivate();
    }
}
