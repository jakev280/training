fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_ascii_uppercase())
        .collect()
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_colour_or(elements: &[String], search: &str, fallback: &str)-> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colours = vec![    
        String::from("Red"),
        String::from("Blue"),
        String::from("Yellow"),
    ];

    let found_colour = find_colour_or(&colours, "Re", "Orange");
    println!("{:#?}", found_colour);

    // let exploded = explode(&colours);
    // println!("{:#?}", exploded);

    // let uppercased = to_uppercase(&colours);
    // println!("{:#?}", uppercased);

    // shorten_strings(&mut colours[1..3]);
    // println!("{:#?}", colours)

    // print_elements(&colours);

}