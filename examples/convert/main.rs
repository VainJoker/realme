use realm::Realm;

fn main() {

    let toml_value1 = toml::toml! {
        key1 = "value1"
    };

    let realm = Realm::try_serialize(&toml_value1).unwrap();

    let toml_value2 = realm.try_deserialize::<toml::Value>().unwrap();
    println!("{realm:#?}");
    println!("{toml_value2:#?}");
}
