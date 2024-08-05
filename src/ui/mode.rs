use strum::Display;

#[derive(Debug, Display)]
pub enum Mode {
    //
    #[strum(to_string = "View")]
    View,

    #[strum(to_string = "Edit")]
    Edit,
}
