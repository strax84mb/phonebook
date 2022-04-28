use crate::entry::Entry;
use std::fs;

pub trait DB {
    fn create(&mut self, e: Entry) -> Result<Entry, String>;
    fn update(&mut self, id: u16, e: Entry) -> Result<Entry, String>;
    fn delete(&mut self, id: u16) -> Result<Entry, String>;
    fn read_all(&self) -> Vec<Entry>;
    fn read_by_id(&self, id: u16) -> Option<Entry>;
    fn search(&self, term: String) -> Vec<Entry>;
}

struct FileDB {
    path: String,
    entries: Vec<Entry>,
}

impl FileDB {
    pub fn new(path: String) -> Result<FileDB, String> {
        let content: String = match fs::read_to_string(path.as_str()) {
            Ok(s) => s,
            Err(e) => {
                let mut err = "failed to load DB: ".to_string();
                err.push_str(e.to_string().as_str());
                return Err(err);
            }
        };
        let entries: Vec<Entry> = content
            .split(';')
            .map(|s| Entry::from(s))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        Ok(FileDB { path, entries })
    }

    fn save(&self) -> Result<(), String> {
        let content: String = self
            .entries
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        match fs::write(self.path.as_str(), content) {
            Ok(()) => Ok(()),
            Err(e) => {
                let mut err = "failed to save DB: ".to_string();
                err.push_str(e.to_string().as_str());
                Err(err)
            }
        }
    }
}

impl DB for FileDB {
    fn create(&mut self, e: Entry) -> Result<Entry, String> {
        todo!()
    }

    fn update(&mut self, id: u16, e: Entry) -> Result<Entry, String> {
        todo!()
    }

    fn delete(&mut self, id: u16) -> Result<Entry, String> {
        let entry_index = match self.entries.iter().enumerate().find(|(i, e)| (*e).id == id) {
            Some((i, _)) => i,
            None => return Err(format!("could not find entry with ID {}", id)),
        };
        let entry = self.entries.remove(entry_index);
        match self.save() {
            Ok(()) => (),
            Err(e) => return Err(e),
        };
        Ok(entry)
    }

    fn read_all(&self) -> Vec<Entry> {
        self.entries.clone()
    }

    fn read_by_id(&self, id: u16) -> Option<Entry> {
        match self.entries.iter().find(|e| e.id == id) {
            Some(e) => Some(e.clone()),
            None => None,
        }
    }

    fn search(&self, term: String) -> Vec<Entry> {
        self.entries
            .iter()
            .filter(|&e| {
                let mut name = e.first_name.clone();
                name.push(' ');
                name.push_str(e.last_name.as_str());
                return name.contains(term.as_str())
                    || e.phone.contains(term.as_str())
                    || e.address.contains(term.as_str())
                    || e.e_mail.contains(term.as_str());
            })
            .map(|e| e.clone())
            .collect()
    }
}
