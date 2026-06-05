pub mod bridge;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() {
    let start_time = std::time::Instant::now();
    println!("[KQuickView] Inicio del main a los {} ms", start_time.elapsed().as_millis());

    // Si no se proporciona archivo, terminar con error
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: No se especificó ningún archivo.");
        eprintln!("Uso: kquickview <ruta_al_archivo>");
        std::process::exit(1);
    }

    println!("[KQuickView] Argumentos parseados a los {} ms", start_time.elapsed().as_millis());

    // Inicializar QGuiApplication
    let mut app = QGuiApplication::new();
    println!("[KQuickView] QGuiApplication creada a los {} ms", start_time.elapsed().as_millis());

    // Inicializar QQmlApplicationEngine
    let mut engine = QQmlApplicationEngine::new();
    println!("[KQuickView] QQmlApplicationEngine creada a los {} ms", start_time.elapsed().as_millis());

    // Cargar la vista QML
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/kquickview/qml/main.qml"));
    }
    println!("[KQuickView] QML cargado a los {} ms", start_time.elapsed().as_millis());

    // Ejecutar el ciclo de eventos de Qt
    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
