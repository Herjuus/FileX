use std::path::PathBuf;

pub enum CurrentScreen {
    Main,
    Search
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub filesystem: Filesystem,
    pub selected_item_index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            filesystem: Filesystem::new(),
            selected_item_index: 0,
        }
    }

    pub fn selection_up(&mut self) {
        if self.selected_item_index >= 1 {
            self.selected_item_index = self.selected_item_index - 1;
        }
    }

    pub fn selection_down(&mut self) {
        self.selected_item_index = self.selected_item_index + 1;
    }
}

pub struct Filesystem {
    pub current_path: PathBuf
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            current_path: std::env::current_dir().unwrap()
        }
    }

    pub fn go_back(&mut self) {
        self.current_path.pop();
        let _new_path = std::env::set_current_dir(&self.current_path);
    }
}