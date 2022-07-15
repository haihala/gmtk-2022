pub struct UIHelper {}

impl UIHelper {
    // Interface for public use
    pub fn show_line(&mut self, line: &'static str) {
        dbg!(line);
    }

    pub fn prompt(&mut self, prompt: &'static str, options: Vec<&'static str>) {
        dbg!(prompt);
        dbg!(options);
    }

    // Interface for internal use
    pub(super) fn new() -> Self {
        Self {}
    }
}
