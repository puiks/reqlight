use crate::models::collection::RequestCollection;
use crate::services::har_import;

#[tauri::command]
pub fn import_har(json_str: String) -> Result<RequestCollection, String> {
    har_import::import_har(&json_str)
}
