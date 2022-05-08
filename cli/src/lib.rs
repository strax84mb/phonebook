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

struct parameters {
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

impl Default for parameters {
    fn default() -> parameters {
        parameters {
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

fn parse_arguments(args: Vec<String>) -> Result<parameters, String> {
    let mut result = parameters {
        ..Default::default()
    };
    let mut it = args.iter();
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

fn execute(args: Vec<String>) {
    let parameters = parse_arguments(args);
}
