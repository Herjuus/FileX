use std::{ffi::OsString, fs::{self, FileType}, io, os::unix::fs::MetadataExt, path::PathBuf};

pub enum CurrentScreen {
    Main,
    Search
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub filesystem: Filesystem,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            filesystem: Filesystem::new(),
        }
    }

    pub fn selection_up(&mut self) {
        if self.filesystem.selected_index >= 1 {
            self.filesystem.selected_index = self.filesystem.selected_index - 1;
        }
    }

    pub fn selection_down(&mut self) {
        self.filesystem.selected_index = self.filesystem.selected_index + 1;
    }
}

pub struct Filesystem {
    pub current_path: PathBuf,
    pub dirs: Vec<Directory>,
    pub selected_index: usize,
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem {
            current_path: std::env::current_dir().unwrap(),
            dirs: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn go_back(&mut self) {
        self.selected_index = 0;
        self.current_path.pop();
        let _ = std::env::set_current_dir(&self.current_path);
    }

    pub fn update_directories(&mut self) -> io::Result<()> {
        self.dirs.clear();

        for entry in fs::read_dir(&self.current_path)? {
            let entry = entry?;
            let name = entry.file_name();
            let file_type = entry.file_type()?;
            let size = entry.metadata()?.size();

            let dir: Directory = Directory {
                name,
                file_type,
                size
            };

            self.dirs.push(dir);
        }

        Ok(())
    }
}

pub struct Directory {
    pub name: OsString,
    pub file_type: FileType,
    pub size: u64,
}
