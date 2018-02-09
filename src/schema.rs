//! Schema builder API
//!

use generators::{DatabaseGenerator, TableGenerator};
use generators::postgres::Pg;

/// Represents an action done on a schema
#[derive(Clone)]
#[allow(unused)]
enum ChangeType {
    CreateTable,
    CreateTableIfNotExists,
    RenameTable,
    DropTable,
    DropTableIfExists,
    AlterTable,
    Raw,
}
use self::ChangeType::*;

/// A schema migration generator
/// 
/// Takes a generic argument that then is used to select the database backend.
pub struct Schema<T: DatabaseGenerator + TableGenerator + Default>(T, Vec<(ChangeType, Table<T>, Box<Fn(&mut Table<T>)>)>);
impl<T: DatabaseGenerator + TableGenerator + Default> Schema<T> {

    /// Create a new Schema with a database backend type
    /// 
    /// Example
    /// 
    /// ```notest
    /// Schema::<PGSQL>::new();
    /// ```
    pub fn new() -> Self {
        return Schema(Default::default(), Vec::new());
    }

    /// Add a table to the schema with a callback
    ///
    /// The callback is provided with a mutable table that fields
    /// can be worked on.
    pub fn create_table<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table<T>),
    {
        let t = Table::new(name);
        self.1.push((CreateTable, t, Box::new(cb)));
    }

    /// Only create a new table if one with the same name doesn't exist
    ///
    /// Provide a callback to manipulate the table. The callback
    /// is lazy and will only be invoked when calling [[exec]]
    pub fn create_table_if_not_exists<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table<T>),
    {
        let t = Table::new(name);
        self.1.push((CreateTableIfNotExists, t, Box::new(cb)));
    }

<<<<<<< HEAD
    /// Rename a table into another
    pub fn rename_table(&mut self, old_name: &str, new_name: &str) {
        
=======
    pub fn rename_table(&mut self, _: &str, _: &str) {
        unimplemented!();
>>>>>>> f02a7d2a20676701964627f749412665dfbaa780
    }

    pub fn drop_table(&mut self, _: &str) {
        unimplemented!();
    }

    pub fn drop_table_if_exists(&mut self, _: &str) {
        unimplemented!();
    }

    /// use this function to manupulate a table
    pub fn table<F: 'static>(&mut self, name: &str, cb: F)
    where
        F: Fn(&mut Table<T>),
    {
        let t = Table::new(name);
        self.1.push((AlterTable, t, Box::new(cb)));
    }


    /// Executes all hooks and does magic
    ///
    /// Needs to be mutable because it morphs the hooks
    pub fn exec(&mut self) -> String {
        let mut s = String::new();

        for pair in &mut self.1 {
            let (mut table, hook) = (&mut pair.1, &pair.2);
            hook(&mut table);
            let table_name: &String = table.get_name();
            let _type = pair.0.clone();

            let cmd: String = match _type {
                CreateTable => T::create_table(table_name),
                AlterTable => T::alter_table(table_name),
                _ => String::from("COMMAND NOT SUPPORTED 😭"),
            };

            /* Add the command, some space, then the table contents */
            s.push_str(&cmd);
            s.push(' ');
            s.push_str(&table.exec());
        }

        return s;
    }
}
