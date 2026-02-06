mod config;
mod podcast;

use std::error::Error;

use podcast::{Podcast, PodcastList};

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::load_config("config.toml")?;
    config.validate()?;

    println!("Loaded config succesfully!");
    println!("{:#?}", config);

    let pod1 = Podcast {
        type_: String::from("short"),
        name: String::from("Podcast 1"),
        priority: true,
        released: false,
    };
    let pod2 = Podcast {
        type_: String::from("long"),
        name: String::from("Podcast 2"),
        priority: true,
        released: false,
    };

    let mut pods = PodcastList::new();
    pods.podcasts.push(pod1);
    pods.podcasts.push(pod2);

    println!("{:#?}", pods);

    println!("Short: {:#?}", pods.short_podcasts());
    println!("Long: {:#?}", pods.long_podcasts());

    let one = pods.get_at_index(0).unwrap();
    println!("{:#?}", one);

    pods.move_to_bottom(0);
    println!("Reordered: {:#?}", pods);

    Ok(())
}
