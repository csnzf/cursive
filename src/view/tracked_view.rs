use std::cell::Cell;

use view::{IdView, View, ViewWrapper};
use Printer;
use vec::Vec2;

/// Wrapper around a view that remembers its position.
pub struct TrackedView<T: View> {
    /// Wrapped view.
    pub view: T,
    /// Last position the view was located.
    offset: Cell<Vec2>,
}

impl<T: View> TrackedView<T> {
    /// Return the last offset at which the view was drawn.
    pub fn offset(&self) -> Vec2 {
        self.offset.get()
    }
}

impl<T: View> TrackedView<T> {
    /// Creates a new `TrackedView` around `view`.
    pub fn new(view: T) -> Self {
        TrackedView {
            view: view,
            offset: Cell::new(Vec2::zero()),
        }
    }

    /// Wraps itself in a `IdView` for easy retrieval.
    pub fn with_id(self, id: &str) -> IdView<Self> {
        IdView::new(id, self)
    }
}

impl<T: View> ViewWrapper for TrackedView<T> {
    wrap_impl!(&self.view);

    fn wrap_draw(&self, printer: &Printer) {
        self.offset.set(printer.offset);
        self.view.draw(printer);
    }
}
