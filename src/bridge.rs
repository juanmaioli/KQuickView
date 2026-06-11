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

        #[qinvokable]
        fn next_file(self: Pin<&mut KQuickViewBridge>);

        #[qinvokable]
        fn previous_file(self: Pin<&mut KQuickViewBridge>);
    }
}

use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::pin::Pin;

pub struct KQuickViewBridgeRust {
    file_path: QString,
    file_url: QString,
    file_type: QString,
    text_content: QString,
    is_valid: bool,
    
    // Campos de navegación internos (no expuestos a QML)
    files: Vec<PathBuf>,
    current_idx: usize,
}

fn render_markdown(content: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let css = "<style>
        body { color: #eff0f1; font-family: sans-serif; font-size: 13px; line-height: 1.5; background-color: #121212; }
        h1, h2, h3, h4 { color: #ffffff; margin-top: 15px; margin-bottom: 8px; font-weight: bold; }
        h1 { font-size: 20px; border-bottom: 1px solid #2a2a2a; padding-bottom: 4px; }
        h2 { font-size: 16px; border-bottom: 1px solid #2a2a2a; padding-bottom: 2px; }
        h3 { font-size: 14px; }
        code { font-family: monospace; background-color: #2a2a2a; padding: 2px 4px; color: #eff0f1; }
        pre { background-color: #1a1a1a; padding: 8px; border: 1px solid #2a2a2a; }
        pre code { background-color: transparent; padding: 0; color: #eff0f1; }
        a { color: #ffffff; text-decoration: underline; }
        ul, ol { margin-top: 5px; margin-bottom: 5px; padding-left: 20px; }
        li { margin-bottom: 2px; }
        table { border-collapse: collapse; width: 100%; margin-top: 10px; margin-bottom: 10px; }
        th, td { border: 1px solid #2a2a2a; padding: 6px; text-align: left; }
        th { background-color: #1a1a1a; color: #ffffff; }
    </style>";

    format!("<html><head>{}</head><body>{}</body></html>", css, html_output)
}

impl Default for KQuickViewBridgeRust {
    fn default() -> Self {
        let file_path_opt = std::env::var("KQUICKVIEW_FILE").ok()
            .or_else(|| {
                let args: Vec<String> = std::env::args().collect();
                if args.len() > 1 {
                    Some(args[1].clone())
                } else {
                    None
                }
            });

        let path_str = match file_path_opt {
            Some(p) => p,
            None => {
                return Self {
                    file_path: QString::from(""),
                    file_url: QString::from(""),
                    file_type: QString::from("unknown"),
                    text_content: QString::from("Error: No se especificó ningún archivo.\nUso: kquickview <ruta_al_archivo>"),
                    is_valid: false,
                    files: Vec::new(),
                    current_idx: 0,
                };
            }
        };

        let path = Path::new(&path_str);
        if !path.exists() {
            return Self {
                file_path: QString::from(path_str.as_str()),
                file_url: QString::from(""),
                file_type: QString::from("unknown"),
                text_content: QString::from(format!("Error: El archivo '{}' no existe.", path_str).as_str()),
                is_valid: false,
                files: Vec::new(),
                current_idx: 0,
            };
        }

        let abs_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let abs_path_str = abs_path.to_string_lossy().to_string();
        let file_url_str = format!("file://{}", abs_path_str);

        // Obtener la lista de archivos en la misma carpeta para navegación
        let parent_dir = abs_path.parent().unwrap_or_else(|| Path::new("."));
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(parent_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let p = entry.path();
                    if p.is_file() {
                        let canon_p = p.canonicalize().unwrap_or_else(|_| p.clone());
                        files.push(canon_p);
                    }
                }
            }
        }
        
        // Ordenar alfabéticamente
        files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        // Buscar el índice del archivo actual
        let current_idx = files.iter().position(|p| p == &abs_path).unwrap_or(0);

        // Adivinar el tipo MIME
        let mime = mime_guess::from_path(&abs_path).first_or_octet_stream();
        let mime_str = mime.to_string();

        let is_md = abs_path.extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
            .map(|ext| ext == "md" || ext == "markdown")
            .unwrap_or(false)
            || mime_str == "text/markdown"
            || mime_str == "text/x-markdown";

        let mut file_type = "unknown";
        let mut text_content = String::new();

        if is_md {
            file_type = "markdown";
            if let Ok(mut file) = File::open(&abs_path) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    text_content = render_markdown(&content);
                }
            }
        } else if mime_str.starts_with("image/") {
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
            // Fallback para texto plano
            if let Ok(mut file) = File::open(&abs_path) {
                let mut buffer = vec![0; 10 * 1024];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    if std::str::from_utf8(&buffer[..bytes_read]).is_ok() {
                        file_type = "text";
                        text_content = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
                        let mut remaining = Vec::new();
                        if file.read_to_end(&mut remaining).is_ok() {
                            let mut full_buffer = buffer[..bytes_read].to_vec();
                            full_buffer.extend(remaining);
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
            files,
            current_idx,
        }
    }
}

impl qobject::KQuickViewBridge {
    pub fn next_file(mut self: Pin<&mut Self>) {
        let (new_idx, has_changed) = {
            let pin_ref = self.as_ref();
            let rust = pin_ref.rust();
            if rust.files.is_empty() {
                (0, false)
            } else if rust.current_idx + 1 < rust.files.len() {
                (rust.current_idx + 1, true)
            } else {
                (rust.current_idx, false)
            }
        };

        if has_changed {
            self.as_mut().rust_mut().current_idx = new_idx;
            self.as_mut().update_current_file();
        }
    }

    pub fn previous_file(mut self: Pin<&mut Self>) {
        let (new_idx, has_changed) = {
            let pin_ref = self.as_ref();
            let rust = pin_ref.rust();
            if rust.files.is_empty() {
                (0, false)
            } else if rust.current_idx > 0 {
                (rust.current_idx - 1, true)
            } else {
                (rust.current_idx, false)
            }
        };

        if has_changed {
            self.as_mut().rust_mut().current_idx = new_idx;
            self.as_mut().update_current_file();
        }
    }

    fn update_current_file(mut self: Pin<&mut Self>) {
        let path_buf = {
            let pin_ref = self.as_ref();
            let rust = pin_ref.rust();
            if rust.current_idx < rust.files.len() {
                rust.files[rust.current_idx].clone()
            } else {
                return;
            }
        };

        let abs_path_str = path_buf.to_string_lossy().to_string();
        let file_url_str = format!("file://{}", abs_path_str);

        let mime = mime_guess::from_path(&path_buf).first_or_octet_stream();
        let mime_str = mime.to_string();

        let is_md = path_buf.extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
            .map(|ext| ext == "md" || ext == "markdown")
            .unwrap_or(false)
            || mime_str == "text/markdown"
            || mime_str == "text/x-markdown";

        let mut file_type = "unknown";
        let mut text_content = String::new();

        if is_md {
            file_type = "markdown";
            if let Ok(mut file) = File::open(&path_buf) {
                let mut content = String::new();
                if file.read_to_string(&mut content).is_ok() {
                    text_content = render_markdown(&content);
                }
            }
        } else if mime_str.starts_with("image/") {
            file_type = "image";
        } else if mime_str.starts_with("text/") 
            || mime_str == "application/json" 
            || mime_str == "application/xml"
            || mime_str == "application/javascript"
        {
            file_type = "text";
            if let Ok(mut file) = File::open(&path_buf) {
                let mut buffer = vec![0; 100 * 1024];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    text_content = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
                }
            }
        } else if mime_str == "application/pdf" {
            file_type = "pdf";
        } else {
            if let Ok(mut file) = File::open(&path_buf) {
                let mut buffer = vec![0; 10 * 1024];
                if let Ok(bytes_read) = file.read(&mut buffer) {
                    if std::str::from_utf8(&buffer[..bytes_read]).is_ok() {
                        file_type = "text";
                        text_content = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
                        let mut remaining = Vec::new();
                        if file.read_to_end(&mut remaining).is_ok() {
                            let mut full_buffer = buffer[..bytes_read].to_vec();
                            full_buffer.extend(remaining);
                            let limit = std::cmp::min(full_buffer.len(), 100 * 1024);
                            text_content = String::from_utf8_lossy(&full_buffer[..limit]).into_owned();
                        }
                    }
                }
            }
        }

        // Modificar las propiedades expuestas a QML usando los setters
        self.as_mut().set_file_path(QString::from(abs_path_str.as_str()));
        self.as_mut().set_file_url(QString::from(file_url_str.as_str()));
        self.as_mut().set_file_type(QString::from(file_type));
        self.as_mut().set_text_content(QString::from(text_content.as_str()));
        self.as_mut().set_is_valid(true);
    }
}
