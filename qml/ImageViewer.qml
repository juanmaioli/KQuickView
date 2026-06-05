import QtQuick

Image {
    anchors.fill: parent
    anchors.margins: 16
    fillMode: Image.PreserveAspectFit
    source: bridge.file_url
    asynchronous: true
}
