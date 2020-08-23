use spud::Controller;

#[derive(Controller)]
struct Other {}

impl Other {
    #[Chained(/)]
    pub fn cross_controller(&self, c: impl Spud, arg: &str) {
        // only hit if arg is a &str
        // chains to root -> root
    }
}
