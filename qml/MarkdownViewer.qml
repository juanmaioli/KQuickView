import QtQuick
import QtQuick.Controls

ScrollView {
    anchors.fill: parent
    anchors.margins: 16

    Text {
        text: bridge.text_content
        textFormat: Text.RichText
        wrapMode: Text.Wrap
        color: "#eff0f1"
        font.family: "sans-serif"
        font.pointSize: 11
        padding: 12
        width: parent.width - 24
        
        onLinkActivated: (link) => {
            Qt.openUrlExternally(link)
        }
    }
}
