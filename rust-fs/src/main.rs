use ::std::fs;
fn main() {
    // membaca string menggunakan (fs::read_to_string)
    let path = "word.txt";
    match fs::read_to_string(path) {
        Ok(file) => println!("isi file {}", file),
        Err(e) => println!("info : {}", e),
    }
    // membuat  atau menulis string menggunakan (fs::write)
    let data = "dari module fs";
    match fs::write("info_module.txt", data) {
        Ok(file) => println!("Succes : {:?}", file),
        Err(e) => println!("ifo : {e}"),
    }
    // copy file use fs::copy
    match fs::copy("word.txt", "test") {
        Ok(file) => println!("succer : {}", file),
        Err(e) => println!("eror : {}", e),
    };
    //membuat directory
    match fs::create_dir("form_module_fs") {
        Ok(file) => println!("succer : {:?}", file),
        Err(e) => println!("eror : {}", e),
    }
    //membuat directory with parent
    match fs::create_dir_all("form_module_fs/path/os") {
        Ok(file) => println!("succer : {:?}", file),
        Err(e) => println!("eror : {}", e),
    }
    match fs::exists("form_module_fs/path/os") {
        Ok(file) => println!("succer : {:?}", file),
        Err(e) => println!("eror : {}", e),
    }
    match fs::exists("src/main.rs") {
        Ok(file) => println!("succer : {:?}", file),
        Err(e) => println!("eror : {}", e),
    }

    //     remove_dir
    //     Removes an empty directory.
    // remove_dir_all
    //     Removes a directory at this path, after removing all its contents. Use carefully!
    // remove_file
    //     Removes a file from the filesystem.
    // rename
    //     Renames a file or directory to a new name, replacing the original file if to already exists.
    // set_permissions
    //     Changes the permissions found on a file or a directory.
    // soft_linkDeprecated
    //     Creates a new symbolic link on the filesystem.
    // symlink_metadata
    //     Queries the metadata about a file without following symlinks.
    // write
    //     Writes a slice as the entire contents of a file.
    // set_permissions_nofollowExperimental
    //     Set the permissions of a file, unless it is a symlink.
    // set_timesExperimental
    //     Changes the timestamps of the file or directory at the specified path.
    // set_times_nofollowExperimenta
}
