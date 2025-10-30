use eframe::{App, egui};
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;

// Estrutura para leitura de logs
pub struct LogReader {
    file: Arc<File>, // Uso compartilhado e seguro do arquivo
}

impl LogReader {
    // Cria uma nova instância de LogReader
    pub fn new(file: Arc<File>) -> Self {
        LogReader { file }
    }

    // Lê uma mensagem do arquivo
    pub fn read_message(&self) -> Result<String, io::Error> {
        let mut buffer = String::new();
        let mut file = self.file.try_clone()?; // Clona a referência do arquivo
        file.read_to_string(&mut buffer)?; // Lê o conteúdo do arquivo
        Ok(buffer)
    }
}

// Estrutura principal do aplicativo
struct ExchangeApp {
    log_message: String, // Armazena a mensagem do log
}

impl ExchangeApp {
    pub fn new() -> Self {
        ExchangeApp {
            log_message: String::new(),
        }
    }

    pub fn update_log(&mut self, log_reader: &LogReader) {
        match log_reader.read_message() {
            Ok(message) => self.log_message = message,
            Err(e) => self.log_message = format!("Erro ao ler mensagem: {}", e),
        }
    }
}

// Implementa a lógica da interface gráfica
impl App for ExchangeApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Bem-vindo à ExchangeApp!");
            ui.label(&self.log_message); // Exibe a mensagem do log
            if ui.button("Clique aqui").clicked() {
                println!("Botão clicado!");
            }
        });
    }
}

fn main() -> io::Result<()> {
    let file_path = "arquivo.log"; // Caminho do arquivo de log

    // Abre o arquivo de log
    let file = File::open(file_path)?;
    let log_reader = LogReader::new(Arc::new(file));

    // Criação do aplicativo
    let mut app = ExchangeApp::new();
    app.update_log(&log_reader); // Atualiza o log no app

    // Configuração e execução da aplicação gráfica
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("ExchangeApp", native_options, Box::new(|_| Box::new(app)))
        .expect("Erro ao executar a aplicação");

    Ok(())
}
