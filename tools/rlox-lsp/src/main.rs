use rlox::scanner::{Scanner, TokenType};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, DidChangeTextDocumentParams, DidOpenTextDocumentParams,
    DocumentSymbol, DocumentSymbolOptions, DocumentSymbolParams, DocumentSymbolResponse,
    InitializeParams, InitializeResult, MessageType, OneOf, Position, Range, ServerCapabilities,
    SymbolKind, TextDocumentContentChangeEvent, TextDocumentSyncCapability, TextDocumentSyncKind,
    Url,
};
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug, Clone)]
struct SymbolRecord {
    name: String,
    kind: SymbolKind,
    range: Range,
}

#[derive(Debug, Clone, Default)]
struct AnalyzedDocument {
    diagnostics: Vec<Diagnostic>,
    symbols: Vec<SymbolRecord>,
}

struct Backend {
    client: Client,
    docs: Arc<RwLock<HashMap<Url, AnalyzedDocument>>>,
}

impl Backend {
    fn analyze_text(source: &str) -> AnalyzedDocument {
        let mut scanner = Scanner::new(source.to_string());

        let mut diagnostics = Vec::new();
        let mut symbols = Vec::new();

        let mut pending_symbol_kind: Option<SymbolKind> = None;

        loop {
            let token = scanner.scan_token();

            match token.token_type {
                TokenType::Error => {
                    diagnostics.push(Diagnostic {
                        range: token_range(source, token.start, token.length.max(1)),
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("rlox-lsp".to_string()),
                        message: "Scanner error token".to_string(),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                    pending_symbol_kind = None;
                }
                TokenType::Fun => {
                    pending_symbol_kind = Some(SymbolKind::FUNCTION);
                }
                TokenType::Class => {
                    pending_symbol_kind = Some(SymbolKind::CLASS);
                }
                TokenType::Var => {
                    pending_symbol_kind = Some(SymbolKind::VARIABLE);
                }
                TokenType::Identifier => {
                    if let Some(kind) = pending_symbol_kind.take() {
                        let start = token.start;
                        let end = token.start + token.length;
                        let name = source[start..end].to_string();
                        let range = token_range(source, token.start, token.length.max(1));
                        symbols.push(SymbolRecord { name, kind, range });
                    }
                }
                TokenType::Semicolon
                | TokenType::LeftBrace
                | TokenType::RightBrace
                | TokenType::Eof => {
                    pending_symbol_kind = None;
                }
                _ => {}
            }

            if token.token_type == TokenType::Eof {
                break;
            }
        }

        AnalyzedDocument {
            diagnostics,
            symbols,
        }
    }

    async fn analyze_and_store(&self, uri: Url, text: &str) {
        let analyzed = Self::analyze_text(text);

        self.client
            .publish_diagnostics(uri.clone(), analyzed.diagnostics.clone(), None)
            .await;

        let mut docs = self.docs.write().await;
        docs.insert(uri, analyzed);
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                document_symbol_provider: Some(OneOf::Right(DocumentSymbolOptions {
                    work_done_progress_options: Default::default(),
                    label: Some("rlox".to_string()),
                })),
                ..ServerCapabilities::default()
            },
            server_info: None,
        })
    }

    async fn initialized(&self, _: tower_lsp::lsp_types::InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "rlox-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.analyze_and_store(params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let text = extract_full_text(params.content_changes);
        self.analyze_and_store(params.text_document.uri, &text)
            .await;
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let docs = self.docs.read().await;
        let maybe_doc = docs.get(&params.text_document.uri);

        let Some(doc) = maybe_doc else {
            return Ok(None);
        };

        let symbols = doc
            .symbols
            .iter()
            .map(|symbol| DocumentSymbol {
                name: symbol.name.clone(),
                detail: None,
                kind: symbol.kind,
                tags: None,
                deprecated: None,
                range: symbol.range,
                selection_range: symbol.range,
                children: None,
            })
            .collect::<Vec<_>>();

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }
}

fn extract_full_text(changes: Vec<TextDocumentContentChangeEvent>) -> String {
    // With FULL sync, LSP clients send the full text in the first change.
    changes
        .into_iter()
        .next()
        .map(|change| change.text)
        .unwrap_or_default()
}

fn token_range(source: &str, start_offset: usize, length: usize) -> Range {
    let start = offset_to_position(source, start_offset);
    let end = offset_to_position(source, start_offset.saturating_add(length));

    Range::new(start, end)
}

fn offset_to_position(source: &str, offset: usize) -> Position {
    let mut line: u32 = 0;
    let mut column: u32 = 0;

    let clamped_offset = offset.min(source.chars().count());

    for (idx, ch) in source.chars().enumerate() {
        if idx >= clamped_offset {
            break;
        }

        if ch == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }

    Position::new(line, column)
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        docs: Arc::new(RwLock::new(HashMap::new())),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
