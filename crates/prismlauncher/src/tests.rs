use crate::LaunchParameters;

#[test]
fn data_directory() {
    let output = super::data_directory();

    println!("data directory located at {}", output);
}

#[test]
fn list_accounts() {
    let output = super::list_accounts();

    println!("Accounts: {:#?}", output);
}

#[test]
#[ignore]
/// Launches an instance named `test`, bad environments are not a bug.
fn launch() {
    super::launch(LaunchParameters {
        instance_id: "test".to_string(),
        gamemode: None,
        profile: None,
    });
}

#[test]
fn version() {
    let output = super::version();

    println!("Version {}", output);
}
