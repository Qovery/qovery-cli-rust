use prettytable::format;
use prettytable::format::{FormatBuilder, LinePosition, LineSeparator};
use prettytable::Table;

pub fn get_table() -> Table {
    let mut table = Table::new();

    let tf = FormatBuilder::new()
        .padding(1, 1)
        .column_separator('|')
        .build();

    table.set_format(tf);
    table
}

pub fn wrap_or(text: String, default: &str) -> String {
    if text.trim().is_empty() {
        return default.to_string();
    }

    text
}
