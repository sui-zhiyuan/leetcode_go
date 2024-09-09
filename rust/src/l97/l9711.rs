pub fn convert_date_to_binary(date: String) -> String {
    let values = date.split("-").collect::<Vec<&str>>();

    let values = values.iter().flat_map(|x| {
        let x = x.parse::<i32>().unwrap();
        [format!("{x:b}") , "-".to_string()]
    }).take(5).collect::<String>();

    values
}