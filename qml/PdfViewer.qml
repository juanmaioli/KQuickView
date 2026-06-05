import QtQuick
import QtQuick.Pdf

PdfMultiPageView {
    property string fileUrl: ""
    anchors.fill: parent
    document: PdfDocument {
        source: fileUrl
    }
}
