use crate::entry::Entry;
use std::fs;
use std::time::SystemTime;

pub trait DB {
    fn create(&mut self, e: Entry) -> Result<Entry, String>;
    fn update(&mut self, id: u16, e: Entry) -> Result<Entry, String>;
    fn delete(&mut self, id: u16) -> Result<Entry, String>;
    fn read_all(&self) -> Vec<Entry>;
    fn read_by_id(&self, id: u16) -> Option<Entry>;
    fn search(&self, term: String) -> Vec<Entry>;
}

pub struct FileDB {
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
    fn create(&mut self, mut e: Entry) -> Result<Entry, String> {
        let next_id = match self.entries.iter().map(|x| x.id).max() {
            Some(x) => x + 1,
            None => 1,
        };
        e.id = next_id;
        self.entries.push(e.clone());
        match self.save() {
            Ok(()) => (),
            Err(err) => return Err(err),
        };
        Ok(e)
    }

    fn update(&mut self, id: u16, e: Entry) -> Result<Entry, String> {
        let to_update = match self.entries.iter_mut().find(|x| (**x).id == id) {
            Some(x) => x,
            None => return Err(format!("could not find entry with ID {}", id)),
        };
        to_update.first_name = e.first_name;
        to_update.last_name = e.last_name;
        to_update.phone = e.phone;
        if e.address.eq("--") {
            to_update.address = "".to_string();
        } else {
            to_update.address = e.address;
        }
        if e.e_mail.eq("--") {
            to_update.e_mail = "".to_string();
        } else {
            to_update.e_mail = e.e_mail;
        }
        to_update.updated_at = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let return_entry: Entry = to_update.clone();
        match self.save() {
            Ok(()) => Ok(return_entry),
            Err(msg) => Err(msg)
        }
    }

    fn delete(&mut self, id: u16) -> Result<Entry, String> {
        let entry_index = match self
            .entries
            .iter()
            .enumerate()
            .find(|(_i, e)| (*e).id == id)
        {
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
