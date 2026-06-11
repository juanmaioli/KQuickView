pub mod bridge;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread;
use std::path::Path;

fn url_decode(input: &str) -> String {
    let mut decoded_bytes = Vec::new();
    let mut bytes = input.as_bytes().iter();
    while let Some(&b) = bytes.next() {
        if b == b'%' {
            if let (Some(&h1), Some(&h2)) = (bytes.next(), bytes.next()) {
                let hex = vec![h1, h2];
                if let Ok(hex_str) = std::str::from_utf8(&hex) {
                    if let Ok(byte) = u8::from_str_radix(hex_str, 16) {
                        decoded_bytes.push(byte);
                        continue;
                    }
                }
            }
        }
        decoded_bytes.push(b);
    }
    String::from_utf8_lossy(&decoded_bytes).into_owned()
}

fn get_dolphin_selected_file() -> Option<String> {
    // 1. Obtener ventana activa
    let output = Command::new("xdotool")
        .arg("getactivewindow")
        .output()
        .ok()?;
    let window_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if window_id.is_empty() {
        return None;
    }

    // Obtener título de la ventana
    let output = Command::new("xdotool")
        .args(["getwindowname", &window_id])
        .output()
        .ok()?;
    let window_name = String::from_utf8_lossy(&output.stdout);
    if !window_name.contains("Dolphin") {
        return None;
    }

    // 2. Guardar portapapeles previo
    let prev_clip = Command::new("xclip")
        .args(["-selection", "clipboard", "-o"])
        .output()
        .map(|o| o.stdout)
        .unwrap_or_default();

    // Limpiar portapapeles
    let _ = Command::new("xclip")
        .args(["-selection", "clipboard", "-i", "/dev/null"])
        .status();

    // Simular Ctrl+C
    let _ = Command::new("xdotool")
        .args(["key", "--clearmodifiers", "ctrl+c"])
        .status();

    // Polling inteligente de hasta 150ms cada 10ms
    let start = Instant::now();
    let mut file_uri = String::new();
    while start.elapsed() < Duration::from_millis(150) {
        let output = Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output();
        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if s.starts_with("file://") {
                file_uri = s;
                break;
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    // Restaurar portapapeles previo
    if !prev_clip.is_empty() {
        if let Ok(mut child) = Command::new("xclip")
            .args(["-selection", "clipboard", "-i"])
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(&prev_clip);
            }
            let _ = child.wait();
        }
    }

    if file_uri.starts_with("file://") {
        let path_encoded = file_uri.trim_start_matches("file://");
        let path_str = url_decode(path_encoded);
        if Path::new(&path_str).exists() {
            return Some(path_str);
        }
    }

    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_to_open = if args.len() < 2 {
        if let Some(path) = get_dolphin_selected_file() {
            path
        } else {
            // Salir en silencio si no hay selección en Dolphin ni argumento
            std::process::exit(0);
        }
    } else {
        args[1].clone()
    };

    unsafe {
        std::env::set_var("KQUICKVIEW_FILE", file_to_open);
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
