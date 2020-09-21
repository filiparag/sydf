use std::io::prelude::*;

pub struct TrackerFile {
    file: std::fs::File,
    list: std::vec::Vec<std::path::PathBuf>
}

impl TrackerFile {
    pub fn new(name: &str, wd: &std::path::Path) -> TrackerFile {
        let t: std::fs::File;
        match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(wd.join(".sydf").join(name)) {
            Ok(f) => t = f,
            Err(e) => {
                eprintln!("Can not open tracker file: {}", e);
                quit::with_code(3)
            }
        }
        let mut l: std::vec::Vec<std::path::PathBuf> = std::vec::Vec::new();
        let reader = std::io::BufReader::new(&t);
        for line in reader.lines() {
            match line {
                Ok(e) => l.push(std::path::PathBuf::from(e)),
                _ => {}
            }
        }
        TrackerFile {
            file: t,
            list: l
        }
    }
    fn save(&mut self) {
        match self.file.set_len(0) {
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
        match self.file.seek(std::io::SeekFrom::Start(0)) {
            Err(e) => eprintln!("Error: {}", e),
            _ => {}
        }
        let mut writer = std::io::BufWriter::new(&self.file);
        for entry in &self.list {
            let mut l = entry.display().to_string();
            l.push('\n');
            match writer.write(l.as_bytes()) {
                Err(e) => eprintln!("Error: {}", e),
                _ => {}
            }
        }
    }
    pub fn contains(&self, path: &std::path::PathBuf) -> bool {
        for entry in &self.list {
            if path.starts_with(entry) {
                return true
            }
        }
        return false;
    }
    pub fn add(&mut self, path: &std::path::PathBuf) -> bool {
        if !self.contains(path) {
            self.list.push(path.to_path_buf());
            return true;
        } else {
            return false;
        }
    }
    pub fn remove(&mut self, path: &std::path::PathBuf) -> bool {
        let mut l: std::vec::Vec<std::path::PathBuf> = std::vec::Vec::new();
        let mut c: u32 = 0;
        for entry in &self.list {
            if !entry.starts_with(path) {
                l.push(entry.clone());
            } else {
                c += 1;
            }
        }
        self.list = l;
        return c > 0;
    }
    pub fn list(&self) -> &std::vec::Vec<std::path::PathBuf> {
        &self.list
    }
}

impl Drop for TrackerFile {
    fn drop(&mut self) {
        self.save();
    }
}