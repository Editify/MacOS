use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ServerCapabilities {
    position_encoding: String,
    text_document_sync: TextDocumentSync,
    selection_range_provider: bool,
    hover_provider: bool,
    completion_provider: CompletionProvider,
    signature_help_provider: SignatureHelpProvider,
    definition_provider: bool,
    type_definition_provider: bool,
    implementation_provider: bool,
    references_provider: bool,
    document_highlight_provider: bool,
    document_symbol_provider: bool,
    workspace_symbol_provider: bool,
    code_action_provider: bool,
    code_lens_provider: CodeLensProvider,
    document_formatting_provider: bool,
    document_range_formatting_provider: bool,
    document_on_type_formatting_provider: DocumentOnTypeFormattingProvider,
    rename_provider: RenameProvider,
    folding_range_provider: bool,
    declaration_provider: bool,
    workspace: Workspace,
    call_hierarchy_provider: bool,
    semantic_tokens_provider: SemanticTokensProvider,
    inlay_hint_provider: InlayHintProvider,
    experimental: Experimental,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextDocumentSync {
    open_close: bool,
    change: i32,
    save: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionProvider {
    trigger_characters: Vec<String>,
    completion_item: CompletionItem,
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionItem {
    label_details_support: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct SignatureHelpProvider {
    trigger_characters: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CodeLensProvider {
    resolve_provider: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct DocumentOnTypeFormattingProvider {
    first_trigger_character: String,
    more_trigger_character: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RenameProvider {
    prepare_provider: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Workspace {
    workspace_folders: WorkspaceFolders,
    file_operations: FileOperations,
}

#[derive(Serialize, Deserialize, Debug)]
struct WorkspaceFolders {
    supported: bool,
    change_notifications: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileOperations {
    will_rename: Vec<Filter>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Filter {
    scheme: String,
    pattern: Pattern,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pattern {
    glob: String,
    matches: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SemanticTokensProvider {
    legend: Legend,
    range: bool,
    full: Full,
}

#[derive(Serialize, Deserialize, Debug)]
struct Legend {
    token_types: Vec<String>,
    token_modifiers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Full {
    delta: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct InlayHintProvider {
    resolve_provider: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Experimental {
    external_docs: bool,
    hover_range: bool,
    join_lines: bool,
    matching_brace: bool,
    move_item: bool,
    on_enter: bool,
    open_cargo_toml: bool,
    parent_module: bool,
    runnables: Runnables,
    ssr: bool,
    workspace_symbol_scope_kind_filtering: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Runnables {
    kinds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InitializeResult {
    capabilities: ServerCapabilities,
    server_info: ServerInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InitializeResponse {
    jsonrpc: String,
    id: i32,
    result: InitializeResult,
}