pub mod bridge;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    // Si no se proporciona archivo, terminar con error
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No se especificó ningún archivo.");
        eprintln!("Uso: kquickview <ruta_al_archivo>");
        std::process::exit(1);
    }

    // Inicializar QGuiApplication
    let mut app = QGuiApplication::new();

    // Inicializar QQmlApplicationEngine
    let mut engine = QQmlApplicationEngine::new();

    // Cargar la vista QML
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/kquickview/qml/main.qml"));
    }

    // Ejecutar el ciclo de eventos de Qt
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
