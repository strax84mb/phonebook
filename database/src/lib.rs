mod db;
mod entry;

pub use db::FileDB;
pub use db::DB;
pub use entry::Entry;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
