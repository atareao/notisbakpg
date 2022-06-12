use crate::label::Label;

#[utoipa::path(
    get,
    path = "/labels/{id}",
    responses(
        (status = 200, description = "Label found succesfully", body = Label),
        (status = 404, description = "Label was not found"),
    )
    ,params(
        ("id" = i32, path, description = "Label database id to get label"),
    )
)]
async fn get_label_by_id(label_id: i32) -> Label{
    Label {
        id: label_id,
        name: "Sample label".to_string(),
    }
}
