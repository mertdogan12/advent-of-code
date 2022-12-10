use std::{collections::HashMap, cell::RefCell};

enum FileType {
    File,
    Directory
}

struct File {
    file_type: FileType,
    children: HashMap<String, File>,
    size: i32,
    name: String,
}

pub struct FileSystem {
    root: File,
    path: Vec<String>,
    size: RefCell<u32>
}

impl FileSystem {
    pub fn new() -> FileSystem {
        FileSystem {
            root: File {
                    file_type: FileType::Directory,
                    children: HashMap::new(),
                    size: -1,
                    name: String::from("/")
            },
            path: Vec::new(),
            size: RefCell::new(0)
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
            size: -1,
            name: name.to_string()
        });
    }

    pub fn create_file(&mut self,  size: i32, name: &str) {
        self.get_file().children.insert(name.to_string(), File {
            file_type: FileType::File,
            children: HashMap::new(),
            size,
            name: name.to_string()
        });
    }

    pub fn get_size(&self) -> u32 {
        self.calc_size(&self.root);
        *self.size.borrow()
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

        if out < 100000 {
            let s = *self.size.borrow();

            self.size.replace(s + out);
        }

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
