pub enum Command {
    Set { key: String, val: String },
    Remove { key: String}
}