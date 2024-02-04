use assert_cmd::Command;

#[test]
fn shows_bytes() {
    let mut cmd = Command::cargo_bin("cchead").unwrap();
    cmd.arg("-c")
        .arg("10")
        .arg("./test_files/text.txt")
        .assert()
        .stdout("The Project\n");
}

#[test]
fn shows_lines() {
    let mut cmd = Command::cargo_bin("cchead").unwrap();
    cmd.arg("-n")
        .arg("10")
        .arg("./test_files/text.txt")
        .assert()
        .stdout("The Project Gutenberg eBook of The Art of War\n    \nThis ebook is for the use of anyone anywhere in the United States and\nmost other parts of the world at no cost and with almost no restrictions\nwhatsoever. You may copy it, give it away or re-use it under the terms\nof the Project Gutenberg License included with this ebook or online\nat www.gutenberg.org. If you are not located in the United States,\nyou will have to check the laws of the country where you are located\nbefore using this eBook.\n\n");
}
