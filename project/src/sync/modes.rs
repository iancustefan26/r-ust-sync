// Sync mode enum, helpful for knowing if the sync has to delete, modify, or create a file
// in case a location has something in plus or minus from others
pub enum SyncMode {
    Create,
    Modify,
    Delete,
    Any,
}

// What type of creation has to be done
pub enum CreateType {
    File,
    Folder,
}
