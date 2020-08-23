struct Root {}

impl spud::Controller for Root {
    #[PathPart(/)]
    pub fn root(&self, c: impl Spud) {}

    #[Chained(/)]
    pub fn end_str(&self, c: impl Spud, arg: &str) {
        // only hit if arg is a &str
    }
    #[Chained(/)]
    pub fn end_u8(&self, c: impl Spud, arg: &u8) {
        // only hit is arg is a &u8 - how do we differentiate?
    }
    #[Chained(/)]
    pub fn many_args(&self, c: impl Spud, arg: &str, second_arg: &u8) {
        // same
    }
}
