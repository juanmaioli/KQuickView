use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("kquickview")
            .version(1, 0)
            .qml_files([
                "qml/main.qml",
                "qml/ImageViewer.qml",
                "qml/TextViewer.qml",
                "qml/PdfViewer.qml",
            ]),
    )
    .file("src/bridge.rs")
    .build();
}
