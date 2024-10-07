use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct BlockDivisionStateRequestBody {
    user_id: String,
    division_id: String,
}
