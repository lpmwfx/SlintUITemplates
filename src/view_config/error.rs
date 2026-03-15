/// Typed error for view-config evaluation and loading.
#[derive(thiserror::Error, Debug)]
/// V ie wc on fi ge rr or enum.
pub enum ViewConfigError {
    /// Rhai script evaluation failed.
    #[error("rhai: {0}")]
    Rhai(#[from] Box<rhai::EvalAltResult>),
    /// File I/O error.
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    /// `Rc::try_unwrap` failed — engine leaked a reference.
    #[error("Rc still has multiple owners")]
    RcUnwrap,
}
