import QtQuick
import QtQuick.Pdf

PdfMultiPageView {
    anchors.fill: parent
    document: PdfDocument {
        source: bridge.file_url
    }
}
