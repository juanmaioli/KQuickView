#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, file_path)]
        #[qproperty(QString, file_url)]
        #[qproperty(QString, file_type)]
        #[qproperty(QString, text_content)]
        #[qproperty(bool, is_valid)]
        type KQuickViewBridge = super::KQuickViewBridgeRust;
    }
}

use cxx_qt_lib::QString;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct KQuickViewBridgeRust {
    file_path: QString,
    file_url: QString,
    file_type: QString,
    text_content: QString,
    is_valid: bool,
}

impl Default for KQuickViewBridgeRust {
    fn default() -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            return Self {
                file_path: QString::from(""),
                file_url: QString::from(""),
                file_type: QString::from("unknown"),
                text_content: QString::from("Error: No se especificó ningún archivo.\nUso: kquickview <ruta_al_archivo>"),
                is_valid: false,
            };
        }

        let path_str = &args[1];
        let path = Path::new(path_str);
        if !path.exists() {
            return Self {
                file_path: QString::from(path_str.as_str()),
                file_url: QString::from(""),
                file_type: QString::from("unknown"),
                text_content: QString::from(format!("Error: El archivo '{}' no existe.", path_str).as_str()),
                is_valid: false,
            };
        }

        let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let abs_path_str = abs_path.to_string_lossy().to_string();
        let file_url_str = format!("file://{}", abs_path_str);

        // Adivinar el tipo MIME
        let mime = mime_guess::from_path(&abs_path).first_or_octet_stream();
        let mime_str = mime.to_string();

        let mut file_type = "unknown";
        let mut text_content = String::new();

        if mime_str.starts_with("image/") {
            file_type = "image";
        } else if mime_str.starts_with("text/") 
            || mime_str == "application/json" 
            || mime_str == "application/xml"
            || mime_str == "application/javascript"
        {
            file_type = "text";
            if let Ok(mut file) = File::open(&abs_path) {
                let mut buffer = vec![0; 100 * 1024]; // Cargar máximo 100KB por performance
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    text_content = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
                }
            }
        } else if mime_str == "application/pdf" {
            file_type = "pdf";
        } else {
            // Intento de fallback para texto plano si se puede decodificar como UTF-8
            if let Ok(mut file) = File::open(&abs_path) {
                let mut buffer = vec![0; 10 * 1024]; // Probar primeros 10KB
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    if std::str::from_utf8(&buffer[..bytes_read]).is_ok() {
                        file_type = "text";
                        text_content = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
                        // Intentar cargar más si es válido
                        let mut remaining = Vec::new();
                        if file.read_to_end(&mut remaining).is_ok() {
                            let mut full_buffer = buffer[..bytes_read].to_vec();
                            full_buffer.extend(remaining);
                            // Tomar primeros 100KB del total
                            let limit = std::cmp::min(full_buffer.len(), 100 * 1024);
                            text_content = String::from_utf8_lossy(&full_buffer[..limit]).into_owned();
                        }
                    }
                }
            }
        }

        Self {
            file_path: QString::from(abs_path_str.as_str()),
            file_url: QString::from(file_url_str.as_str()),
            file_type: QString::from(file_type),
            text_content: QString::from(text_content.as_str()),
            is_valid: true,
        }
    }
}
