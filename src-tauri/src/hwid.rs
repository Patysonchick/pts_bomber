use hardware_id::get_id;

pub fn check_key() -> Result<(), ()> {
    let hwid = get_id().unwrap();
    println!("{}", hwid);

    Ok(())
}
