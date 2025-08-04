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
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    let (read, write) = tokio::io::split(stream);
    let (service, socket) = LspService::new(|client| Backend { client });

    Server::new(read, write, socket).serve(service).await;
}
