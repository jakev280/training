mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Author"),
    };
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    
    // println!("{}", bad_book.description());
    // println!("{}", good_movie.description());
    // println!("{}", audiobook.description());

    let mut catalog = Catalog::new();

    catalog.add(bad_book);
    catalog.add(good_movie);
    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    let item = catalog.get_by_index(40);
    let placeholder = Media::Placeholder;

    // println!("{:#?}", item.unwrap())
    // println!("{:#?}", item.expect("expected an item here!"))
    println!("{:#?}", item.unwrap_or(&placeholder))


    // println!("{:#?}", item)

    // match catalog.get_by_index(990) {
    //     Some(value) => {
    //         println!("Item: {:#?}", value);
    //     }
    //     None => {
    //         println!("No Value Here!");
    //     }
    // }

    // if let Some(value) = catalog.get_by_index(0) {
    //     println!("Item in pattern match: {:#?}", value)
    // } else {
    //     println!("No value!!!!!!!!!!!");
    // }
}
