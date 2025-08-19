use serde::{Deserialize, Serialize};
use tower_lsp::{
    async_trait,
    jsonrpc::Result,
    lsp_types::{
        notification::Notification, DidChangeTextDocumentParams, InitializeParams,
        InitializeResult, InitializedParams, MessageType, ServerCapabilities,
        TextDocumentIdentifier, TextDocumentPositionParams, TextDocumentSyncCapability,
        TextDocumentSyncKind,
    },
    Client, LanguageServer, LspService, Server,
};

#[derive(Debug, Serialize, Deserialize)]
struct NotificationParams {
    title: String,
    message: String,
    description: String,
}

enum CustomNotification {}
impl Notification for CustomNotification {
    type Params = TextDocumentPositionParams;
    const METHOD: &'static str = "textDocument/cursorPosition";
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        log::info!("Event: initialize");
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        log::info!("Event: initialized");
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        log::info!("Event: shutdown");
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        log::info!("Event: did_change");
        let uri = params.text_document.uri;
        let content_changes = params.content_changes;

        for change in content_changes {
            self.client
                .log_message(
                    MessageType::INFO,
                    format!("Change in {}: {:?}", uri, change.range),
                )
                .await;

            if let Some(range) = change.range {
                let params = TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: uri.clone() },
                    position: range.start,
                };

                self.client
                    .send_notification::<CustomNotification>(params)
                    .await;
            }
        }
    }
}

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// log into given file
    #[arg(short, long, value_name = "FILE")]
    log_file: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Some(log_file) = cli.log_file {
        let file = std::fs::File::create(log_file).expect("could not open log file");
        let target = Box::new(file);
        env_logger::Builder::new()
            .target(env_logger::Target::Pipe(target))
            .filter(None, log::LevelFilter::Trace)
            .init();
    } else {
        env_logger::init()
    }
    /*    // construct a subscriber that prints formatted traces to stdout
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // use that subscriber to process traces emitted after this point
        tracing::subscriber::set_global_default(subscriber).expect("init log failed");
    */

    log::info!("Starting LSP server");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    log::info!("LSP server listening...");
    let (stream, _) = listener.accept().await.unwrap();
    log::info!("Client has connected to LSP service");
    let (read, write) = tokio::io::split(stream);
    let (service, socket) = LspService::new(|client| Backend { client });
    log::info!("LSP service has been created");

    Server::new(read, write, socket).serve(service).await;
}
