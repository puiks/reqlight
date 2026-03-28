use crate::models::collection::RequestCollection;
use crate::services::openapi_import;

/// IPC: Import an OpenAPI/Swagger spec (JSON or YAML) and return collections.
#[tauri::command]
pub fn import_openapi(spec: String) -> Result<Vec<RequestCollection>, String> {
    openapi_import::import_openapi(&spec)
}
