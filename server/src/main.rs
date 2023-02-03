mod services;

fn main() {
    let strategy_files =
        std::fs::read_dir("./strategies").expect("Strategies subdirectory not found.");

    let strategies = strategy_files
        .map(|s| s.unwrap().path().into_os_string().into_string().unwrap())
		.map(|s| services::strategy::Strategy::new_from_file(&s).unwrap())
		.collect::<Vec<_>>();

    for strategy in strategies {
        let out = strategy.check_binary_exists();

        match out {
            Ok(x) => println!("Strategy Check Output '{}'" , std::str::from_utf8(&x.stdout).unwrap()),
            Err(e) => println!("Bad {:?}", e)
        }
    }

}
