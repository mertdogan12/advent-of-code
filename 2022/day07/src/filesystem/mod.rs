use std::{collections::HashMap, cell::RefCell};

enum FileType {
    File,
    Directory
}

struct File {
    file_type: FileType,
    children: HashMap<String, File>,
    size: i32
}

pub struct FileSystem {
    root: File,
    path: Vec<String>,
    dirs: RefCell<Vec<u32>>
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            root: File {
                    file_type: FileType::Directory,
                    children: HashMap::new(),
                    size: -1,
            },
            path: Vec::new(),
            dirs: RefCell::new(Vec::new())
        }
    }

    pub fn change_dir(&mut self, path: &str) {
        match path {
            "/" => self.path.clear(),
            ".." => {
                self.path.pop();
            },
            _ => self.path.push(path.to_string()),
        }
    }

    pub fn create_dir(&mut self, name: &str) {
        self.get_file().children.insert(name.to_string(), File {
            file_type: FileType::Directory,
            children: HashMap::new(),
            size: -1
        });
    }

    pub fn create_file(&mut self,  size: i32, name: &str) {
        self.get_file().children.insert(name.to_string(), File {
            file_type: FileType::File,
            children: HashMap::new(),
            size
        });
    }

    pub fn get_size(&self) -> u32 {
        self.calc_size(&self.root)
    }
    
    pub fn free_size(&self, size: u32) -> u32 {
        let mut out: u32 = 70000000;

        for dir in self.dirs.borrow().iter() {
            if *dir > size && *dir < out {
                out = *dir;
            }
        }

        out
    }

    fn calc_size(&self, file: &File) -> u32 {
        let mut out: u32 = 0;

        for child in file.children.iter() {
            let f: &File = child.1;

            match &f.file_type {
                FileType::Directory => out += self.calc_size(f),
                FileType::File => out += f.size as u32,
            }
        }

        self.dirs.borrow_mut().push(out);

        out
    }

    fn get_file(&mut self) -> &mut File {
        let mut out: &mut File = &mut self.root;

        for p in self.path.iter() {
           out = out.children.get_mut(p).unwrap_or_else(|| {
               panic!("Could not find file {p}");
           });
        }

        out
    }
}
