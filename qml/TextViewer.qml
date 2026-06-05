import QtQuick
import QtQuick.Controls

ScrollView {
    anchors.fill: parent
    anchors.margins: 16

    TextArea {
        text: bridge.text_content
        readOnly: true
        selectByMouse: true
        color: "#eff0f1"
        font.family: "monospace"
        font.pointSize: 10
        background: Rectangle {
            color: "#181818"
            radius: 4
            border.color: "#2a2a2a"
        }
        padding: 12
        wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere
    }
}
