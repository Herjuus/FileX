use std::{ffi::OsString, fs::{self, FileType, OpenOptions}, io, os::unix::fs::MetadataExt, path::PathBuf};

use clipboard::{ClipboardContext, ClipboardProvider};
use open::that;

pub enum CurrentScreen {
    Main,
    Search,
    Help,
    Error
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub filesystem: Filesystem,
    pub last_error_message: String,
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::Main,
            filesystem: Filesystem::new(),
            last_error_message: String::new(),
        }
    }

    pub fn set_current_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
    }

    pub fn handle_error(&mut self, error_message: String) {
        self.last_error_message = format!("Error: {}", error_message);
        self.set_current_screen(CurrentScreen::Error);
    }

    pub fn move_up(&mut self, steps: i64) {
        let mut temp = self.filesystem.selected_index as i64 - steps;
        if temp < 0 {
            temp = 0;
        }
        self.filesystem.selected_index = temp as usize;
    }

    pub fn move_down(&mut self, steps: i64) {
        let max = self.filesystem.dirs.len() as i64 - 1;

        let mut temp = self.filesystem.selected_index as i64 + steps;
        if temp > max {
            temp = max;
        }

        self.filesystem.selected_index = temp as usize;
    }

    pub fn copy_path(&self) {
        let path = self.filesystem.current_path.to_string_lossy().to_string();

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(path.to_owned()).unwrap();
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
        // let _ = std::env::set_current_dir(&self.current_path);
    }

    pub fn open_go_forward(&mut self) {
        let item = &self.dirs[self.selected_index];

        if item.file_type.is_dir() {
            self.selected_index = 0;
            self.current_path.push(&PathBuf::from(&item.name))
        }

        if item.file_type.is_file() {
            let mut file_path = self.current_path.clone();
            file_path.push(&item.name);

            if let Err(e) = that(&file_path) {
                // eprintln!("Failed to open file: {:?}, error: {}", file_path, e);
            }
        }
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

        self.dirs.sort_by(|a, b| {
            let a_is_dir = a.file_type.is_dir();
            let b_is_dir = b.file_type.is_dir();
            if a_is_dir && !b_is_dir {
                return std::cmp::Ordering::Less;
            } else if !a_is_dir && b_is_dir {
                return std::cmp::Ordering::Greater;
            }

            a.name.cmp(&b.name)
        });

        Ok(())
    }
}

pub struct Directory {
    pub name: OsString,
    pub file_type: FileType,
    pub size: u64,
}
