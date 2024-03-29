use database::{Entry, FileDB, DB};

enum Operation {
    Create,
    Update,
    Delete,
    Search,
    Help,
    None,
}

enum ArgString {
    DatabasePath,
    ID,
    FirstName,
    LastName,
    Phone,
    Address,
    Email,
    SearchTerm,
}

struct Parameters {
    database_path: String,
    operation: Operation,
    id: u16,
    first_name: String,
    last_name: String,
    phone: String,
    address: String,
    e_mail: String,
    search_term: String,
}

impl Default for Parameters {
    fn default() -> Parameters {
        Parameters {
            database_path: "".to_string(),
            operation: Operation::None,
            id: 0,
            first_name: "".to_string(),
            last_name: "".to_string(),
            phone: "".to_string(),
            address: "".to_string(),
            e_mail: "".to_string(),
            search_term: "".to_string(),
        }
    }
}

impl Parameters {
    fn to_entry(self) -> Entry {
        Entry {
            id: self.id,
            first_name: self.first_name,
            last_name: self.last_name,
            phone: self.phone,
            address: self.address,
            e_mail: self.e_mail,
            created_at: 0,
            updated_at: 0,
        }
    }
}

fn parse_arguments(args: Vec<String>) -> Result<Parameters, String> {
    let mut result = Parameters {
        ..Default::default()
    };
    let mut it = args.iter();
    it.next();
    let mut param_name: String;
    let mut param_value: String;
    let mut param_type: ArgString;
    loop {
        param_name = match it.next() {
            Some(x) => x.clone(),
            None => break,
        };
        match param_name.as_str() {
            "-d" | "db-path" => param_type = ArgString::DatabasePath,
            "-i" | "id" => param_type = ArgString::ID,
            "-f" | "first-name" => param_type = ArgString::FirstName,
            "-l" | "last-name" => param_type = ArgString::LastName,
            "-p" | "phone" => param_type = ArgString::Phone,
            "-a" | "address" => param_type = ArgString::Address,
            "-e" | "e-mail" => param_type = ArgString::Email,
            "-t" | "term" => param_type = ArgString::SearchTerm,
            "create" => {
                result.operation = Operation::Create;
                continue;
            }
            "update" => {
                result.operation = Operation::Update;
                continue;
            }
            "delete" => {
                result.operation = Operation::Delete;
                continue;
            }
            "search" => {
                result.operation = Operation::Search;
                continue;
            }
            "-h" | "help" => {
                result.operation = Operation::Help;
                continue;
            }
            _ => return Err(format!("unknown parameter {}", param_name).to_string()),
        };
        param_value = match it.next() {
            Some(x) => x.clone(),
            None => return Err(format!("missing value for parameter {}", param_name).to_string()),
        };
        match param_type {
            ArgString::DatabasePath => result.database_path = param_value,
            ArgString::ID => {
                result.id = match param_value.parse::<u16>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
            }
            ArgString::FirstName => result.first_name = param_value,
            ArgString::LastName => result.last_name = param_value,
            ArgString::Phone => result.phone = param_value,
            ArgString::Address => result.address = param_value,
            ArgString::Email => result.e_mail = param_value,
            ArgString::SearchTerm => result.search_term = param_value,
        };
    }
    Ok(result)
}

pub fn execute(args: Vec<String>) {
    let parameters = parse_arguments(args);
    match parameters {
        Ok(p) => {
            match p.operation {
                Operation::Help => print_help_msg(),
                Operation::None => {
                    println!("Error: Unsupported command");
                    std::process::exit(1);
                }
                _ => {
                    match FileDB::new(p.database_path.clone()) {
                        Ok(mut db) => match p.operation {
                            Operation::Create => {
                                match check_create_params(&p) {
                                    Err(e) => {
                                        println!("Error: {}", e);
                                        print_help_create();
                                        std::process::exit(1);
                                    }
                                    _ => (),
                                };
                                match db.create(p.to_entry()) {
                                    Ok(entry) => {
                                        println!("Successfully created entry");
                                        print_single_entry(&entry);
                                    }
                                    Err(msg) => {
                                        println!("Error: {}", msg);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Operation::Update => {
                                match check_create_params(&p) {
                                    Err(e) => {
                                        println!("Error: {}", e);
                                        print_help_delete();
                                        std::process::exit(1);
                                    }
                                    _ => (),
                                };
                                match db.update(p.id, p.to_entry()) {
                                    Ok(entry) => {
                                        println!("Successfully updated entry");
                                        print_single_entry(&entry);
                                    }
                                    Err(msg) => {
                                        println!("Error: {}", msg);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Operation::Delete => {
                                match check_delete_params(&p) {
                                    Err(e) => {
                                        println!("Error: {}", e);
                                        print_help_delete();
                                        std::process::exit(1);
                                    }
                                    _ => (),
                                };
                                match db.delete(p.id) {
                                    Ok(entry) => {
                                        println!("Successfully deleted entry");
                                        print_single_entry(&entry);
                                    }
                                    Err(msg) => {
                                        println!("Error: {}", msg);
                                        std::process::exit(1);
                                    }
                                };
                            }
                            Operation::Search => {
                                match check_search_params(&p) {
                                    Err(e) => {
                                        println!("Error: {}", e);
                                        print_help_search();
                                        std::process::exit(1);
                                    }
                                    _ => (),
                                };
                                print_all_entries(db.search(p.search_term));
                            }
                            _ => {
                                println!("Error: Thist code should be unreachable");
                                std::process::exit(1);
                            }
                        },
                        Err(msg) => {
                            println!("Error: {}", msg);
                            std::process::exit(1);
                        }
                    };
                }
            }
        }
        Err(msg) => {
            println!("Error: {}", msg);
            std::process::exit(1);
        }
    };
}

fn print_help_msg() {
    println!("Phonebook CLI v0.0.1");
    println!("Usage: phonebook command [parameters]");
    println!("Commands:");
    println!("     create - Create new entry");
    println!("     update - Update existing entry");
    println!("     delete - Delete entry");
    println!("     search - Search for entries containing term");
    println!("  help | -h - Print this message");
    println!("Parameters:");
    println!("  -i | id         - ID number of entry");
    println!("  -f | first-name - First name");
    println!("  -l | last-name  - Last name");
    println!("  -p | phone      - Phone number");
    println!("  -a | address    - Address");
    println!("  -e | e-mail     - E-mail address");
    println!("  -t | term       - Search term");
    println!("  -d | db-path    - File path of the database");
}

fn print_help_create() {
    println!("Usage of create command");
    println!("     create -f John -l Smith -p 123 [-a \"My street 12a\"] [-e johnsmith@gmail.com]");
    println!("     create first-name John last-name Smith phone 123 [address \"My street 12a\"] [e-mail johnsmith@gmail.com]");
}

fn print_help_delete() {
    println!("Usage of delete command");
    println!("     delete -i 123");
    println!("     delete id 123");
}

fn print_help_search() {
    println!("Usage of search command");
    println!("     search -t \"John Smith\"");
    println!("     search term \"John Smith\"");
}

fn print_single_entry(entry: &Entry) {
    println!("        ID: {}", entry.id);
    println!("First name: {}", entry.first_name);
    println!(" Last name: {}", entry.last_name);
    println!("     Phone: {}", entry.phone);
    println!("   Address: {}", entry.address);
    println!("    E-mail: {}", entry.e_mail);
}

fn print_all_entries(entries: Vec<Entry>) {
    match entries.len() {
        0 => println!("No entries to show"),
        _ => {
            print_entry_table_separator();
            print_entry_lines_header();
            print_entry_table_separator();
            for e in entries.iter() {
                print_entry_as_lines(e);
            }
            print_entry_table_separator();
        }
    }
}

fn print_entry_lines_header() {
    /*
    ID: 5
    FN: 10
    LN: 15
    PH: 15
    AD: 20
    EM: 20
    */
    println!("| ID  |First name|   Last name   |  Phone book   |       Address      |       E-mail       |");
}

fn print_entry_table_separator() {
    println!("+-----+----------+---------------+---------------+--------------------+--------------------+");
}

fn print_entry_as_lines(entry: &Entry) {
    let mut has_more: bool;
    let mut i: usize = 0;
    loop {
        has_more = print_entry_line(entry, i);
        if !has_more {
            break;
        }
        i = i + 1;
    }
}

fn print_entry_line(entry: &Entry, line_number: usize) -> bool {
    let mut has_more: bool;
    let mut temp: bool;
    print!("|");
    has_more = print_entry_field(&format!("{}", entry.id), line_number, 5);
    print!("|");
    temp = print_entry_field(&entry.first_name, line_number, 10);
    if temp && !has_more {
        has_more = true;
    }
    print!("|");
    temp = print_entry_field(&entry.last_name, line_number, 15);
    if temp && !has_more {
        has_more = true;
    }
    print!("|");
    temp = print_entry_field(&entry.phone, line_number, 15);
    if temp && !has_more {
        has_more = true;
    }
    print!("|");
    temp = print_entry_field(&entry.address, line_number, 20);
    if temp && !has_more {
        has_more = true;
    }
    print!("|");
    temp = print_entry_field(&entry.e_mail, line_number, 20);
    if temp && !has_more {
        has_more = true;
    }
    println!("|");

    has_more
}

fn print_entry_field(value: &String, line_number: usize, available_width: usize) -> bool {
    if value.len() <= available_width * line_number {
        print!("{:<available_width$}", " ");
        return false;
    } else if value.len() > available_width * (line_number + 1) {
        let sub_str: String = value.clone().chars().skip(available_width * line_number).take(available_width).collect();
        print!("{}", sub_str);
        return true;
    } else {
        let sub_str: String = value.clone().chars().skip(available_width * line_number).take(value.len() - (available_width * line_number)).collect();
        print!("{:<available_width$}", sub_str);
        return false;
    }
}

fn check_param_id(id: u16) -> Result<(), String> {
    if id <= 0 {
        return Err("id must be stated and it must be a positive number".to_string());
    }
    Ok(())
}

fn check_param_term(term: &String) -> Result<(), String> {
    if term.eq("") {
        return Err("term must be stated".to_string());
    }
    Ok(())
}

fn check_delete_params(p: &Parameters) -> Result<(), String> {
    check_param_id(p.id.clone())
}

fn check_search_params(p: &Parameters) -> Result<(), String> {
    check_param_term(&p.search_term)
}

fn check_create_params(p: &Parameters) -> Result<(), String> {
    match true {
        true if p.first_name.eq("") || p.first_name.eq("--") => return Err("first name must be stated".to_string()),
        true if p.last_name.eq("") || p.last_name.eq("--")  => return Err("last name must be stated".to_string()),
        true if p.phone.eq("") || p.phone.eq("--") => return Err("phone number must be stated".to_string()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_create_params() -> Parameters{
        Parameters{
            id: 1,
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            phone: "123".to_string(),
            operation: Operation::Create,
            address: "Wild west".to_string(),
            e_mail: "js@gmail.com".to_string(),
            database_path: "/some/path".to_string(),
            search_term: "".to_string()
        }
    }

    #[test]
    fn check_good_delete_params() {
        let p = generate_create_params();
        match check_create_params(&p) {
            Err(_msg) => panic!("This should be successful!"),
            Ok(()) => {}
        }
    }

    #[test]
    fn fail_create_params_missing_first_name() {
        let mut p = generate_create_params();
        p.first_name = String::from("");
        match check_create_params(&p) {
            Ok(()) => panic!("This should fail!"),
            Err(msg) => assert_eq!("first name must be stated".to_string(), msg)
        }
    }
}
