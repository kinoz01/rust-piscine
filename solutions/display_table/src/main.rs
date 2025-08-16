use display_table::Table;

fn main() {
    let mut table = Table::new();
    table.headers = vec![
        "ID".to_string(),
        "Car Brand".to_string(),
        "Model".to_string(),
        "Is Electric".to_string(),
    ];
    table.add_row(&[
        "1".to_string(),
        "Tesla".to_string(),
        "Model 3".to_string(),
        "True".to_string(),
    ]);
    table.add_row(&[
        "2".to_string(),
        "Ford".to_string(),
        "Focus".to_string(),
        "False".to_string(),
    ]);
    println!("{}", table);
}

