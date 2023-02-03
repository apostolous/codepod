mod services;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    let strategy_files =
        std::fs::read_dir("./strategies").expect("Strategies subdirectory not found.");

    let binaries = strategy_files
        .map(|s| s.unwrap().path().into_os_string().into_string().unwrap())
        .map(|s| services::strategy::Strategy::new_from_file(&s).unwrap())
        .map(|s| {
            s.check_binary_exists().unwrap()
        }).collect::<Vec<String>>();

    binaries[0].clone()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}