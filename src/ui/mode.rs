use strum::Display;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Mode {
    //
    #[strum(to_string = "View")]
    View,

    #[strum(to_string = "Edit")]
    Edit,
}
