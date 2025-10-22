// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad language server.

mod processor;

use serde::{Deserialize, Serialize};
use tower_lsp::{
    Client, LanguageServer, LspService, Server, async_trait,
    jsonrpc::Result,
    lsp_types::{
        DiagnosticOptions, DiagnosticServerCapabilities, DidChangeTextDocumentParams,
        DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
        DocumentDiagnosticParams, DocumentDiagnosticReportResult, InitializeParams,
        InitializeResult, InitializedParams, MessageType, ServerCapabilities,
        TextDocumentIdentifier, TextDocumentPositionParams, TextDocumentSyncCapability,
        TextDocumentSyncKind, notification::Notification,
    },
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
    processor: processor::ProcessorInterface,
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
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions::default(),
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
        log::info!("Did open: {}", params.text_document.uri);

        self.processor
            .send_request(processor::ProcessorRequest::AddDocument(
                params.text_document.uri,
            ))
            .expect("No error");
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        log::info!("Did save: {}", params.text_document.uri);

        self.processor
            .send_request(processor::ProcessorRequest::UpdateDocument(
                params.text_document.uri,
            ))
            .expect("No error");
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.processor
            .send_request(processor::ProcessorRequest::RemoveDocument(
                params.text_document.uri,
            ))
            .expect("No error")
    }

    async fn diagnostic(
        &self,
        _params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        todo!()
    }
}

use clap::Parser;

use crate::processor::WorkspaceSettings;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// log into given file
    #[arg(short, long, value_name = "FILE")]
    log_file: Option<std::path::PathBuf>,

    /// Paths to search for files.
    ///
    /// By default, `./lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append)]
    pub search_paths: Vec<std::path::PathBuf>,
}

impl Cli {
    /// Returns microcad's config dir, even if it does not exist.
    ///
    /// On Linux, the config dir is located in `~/.config/microcad`.
    pub fn config_dir() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|dir| dir.join("microcad"))
    }

    /// Returns global root dir, even if it does not exist.
    ///
    /// On Linux, the root dir is located in `~/.config/microcad/lib`.
    pub fn global_root_dir() -> Option<std::path::PathBuf> {
        Self::config_dir().map(|dir| dir.join("lib"))
    }

    /// `./lib` (if exists) and `~/.config/microcad/lib` (if exists).
    pub fn default_search_paths() -> Vec<std::path::PathBuf> {
        let local_dir = std::path::PathBuf::from("./lib");
        let mut search_paths = Vec::new();

        if let Some(global_root_dir) = Self::global_root_dir() {
            if global_root_dir.exists() {
                search_paths.push(global_root_dir);
            }
        }
        if local_dir.exists() {
            search_paths.push(local_dir);
        }

        search_paths
    }
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

    // add default paths if no search paths are given.
    let mut search_paths = cli.search_paths.clone();

    if search_paths.is_empty() {
        search_paths.append(&mut Cli::default_search_paths())
    };

    log::info!("Starting LSP server");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5007")
        .await
        .unwrap();
    log::info!("LSP server listening...");
    let (stream, _) = listener.accept().await.unwrap();
    log::info!("Client has connected to LSP service");
    let (read, write) = tokio::io::split(stream);

    let processor = processor::ProcessorInterface::run(WorkspaceSettings { search_paths });

    let (service, socket) = LspService::new(|client| Backend { client, processor });
    log::info!("LSP service has been created");

    Server::new(read, write, socket).serve(service).await;
}
