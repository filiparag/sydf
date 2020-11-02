mod bundle;
use bundle::Bundle;

fn main() {
    

    let config: Bundle = Bundle::from("/home/filiparag/.local/share/sydf").unwrap();

    // config.about.author = "s";

    println!("{}", config);

    // println!("{}", toml::to_string_pretty(&config.values).unwrap());

    // config.save("/home/filiparag/.local/share/sydf/.sydf/bundle.toml");

    config.save().unwrap();

}
