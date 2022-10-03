use utoipa::Component;
use crate::label::Label;

#[derive(Debug, Component)]
pub struct Labels{
    labels: Vec<Label>
}

#[utoipa::path(
    get,
    path = "/labels",
    responses(
        (status = 200, description = "Label found succesfully", body = Labels),
        (status = 404, description = "Label was not found"),
    )
)]
async fn get_all_labels() -> Vec<Label>{
    let mut labels: Vec<Label> = Vec::new();
    labels.push(Label{id: 1, name:"Label 1".to_string(),});
    labels.push(Label{id: 2, name:"Label 2".to_string(),});
    labels.push(Label{id: 3, name:"Label 3".to_string(),});
    labels.push(Label{id: 4, name:"Label 4".to_string(),});
    labels
}

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
