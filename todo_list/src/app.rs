use tui::widgets::ListState;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn change(&mut self, a: usize) {
        self.index = (self.index + a) % self.titles.len();
    }
}

pub struct StateFullList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub progress: f64,
    pub tasks: StateFullList<&'a str>,
    pub task_desc: StateFullList<&'a str>,
    pub enhanced_gfx: bool,
}
