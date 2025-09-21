use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateErrItem {
    field: String,
    code: String,
}

pub fn map_validation(err: ValidationErrors) -> Vec<ValidateErrItem> {
    err.field_errors()
        .iter()
        .flat_map(|(field, erros)| {
            erros.iter().map(|e| ValidateErrItem {
                field: field.to_string(),
                code: e.code.to_string(),
            })
        })
        .collect::<Vec<ValidateErrItem>>()
}
