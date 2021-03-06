/// Used to define the layout direction on a widget.
#[derive(Debug, PartialEq, Eq)]
pub enum Alignment {
    /// The children will be aligned horizontal.
    Horizontal,

    /// The children will be aligned vertical.
    Vertical,
}