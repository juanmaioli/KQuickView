use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("kquickview")
            .version(1, 0)
            .qml_file("qml/main.qml"),
    )
    .file("src/bridge.rs")
    .build();
}
