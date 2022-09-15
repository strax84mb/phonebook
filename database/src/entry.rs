pub struct Entry {
    pub id: u16,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub address: String,
    pub e_mail: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Entry {
    pub fn from(s: &str) -> Result<Entry, String> {
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() != 8 {
            return Err("wrong number of arguments".to_string());
        }
        let id = match parts[0].parse::<u16>() {
            Ok(x) => x,
            Err(e) => {
                let mut err = "failed to parse id: ".to_string();
                err.push_str(e.to_string().as_str());
                return Err(err);
            }
        };
        let created_at = match parts[6].parse::<u64>() {
            Ok(x) => x,
            Err(e) => {
                let mut err = "failed to parse created_at: ".to_string();
                err.push_str(e.to_string().as_str());
                return Err(err);
            }
        };
        let updated_at = match parts[7].parse::<u64>() {
            Ok(x) => x,
            Err(e) => {
                let mut err = "failed to parse updated_at: ".to_string();
                err.push_str(e.to_string().as_str());
                return Err(err);
            }
        };
        Ok(Entry {
            id,
            first_name: parts[1].to_string(),
            last_name: parts[2].to_string(),
            phone: parts[3].to_string(),
            address: parts[4].to_string(),
            e_mail: parts[5].to_string(),
            created_at,
            updated_at,
        })
    }

    pub fn to_string(&self) -> String {
        [
            format!("{}", self.id).as_str(),
            self.first_name.as_str(),
            self.last_name.as_str(),
            self.phone.as_str(),
            self.address.as_str(),
            self.e_mail.as_str(),
            format!("{}", self.created_at).as_str(),
            format!("{}", self.updated_at).as_str(),
        ]
        .join(";")
    }
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        Entry {
            id: self.id.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            phone: self.phone.clone(),
            address: self.address.clone(),
            e_mail: self.e_mail.clone(),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.id = source.id.clone();
        self.first_name = source.first_name.clone();
        self.last_name = source.last_name.clone();
        self.phone = source.phone.clone();
        self.address = source.address.clone();
        self.e_mail = source.e_mail.clone();
        self.created_at = source.created_at.clone();
        self.updated_at = source.updated_at.clone();
    }
}
