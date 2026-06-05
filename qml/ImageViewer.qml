import QtQuick

Image {
    property string fileUrl: ""
    anchors.fill: parent
    anchors.margins: 16
    fillMode: Image.PreserveAspectFit
    source: fileUrl
    asynchronous: true
}
