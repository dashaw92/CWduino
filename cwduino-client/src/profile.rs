use std::process::Command;

pub(crate) struct Profile<'a> {
    folder: &'a str,
}

impl<'a> Profile<'a> {
    pub fn load(folder: &'a str) -> Result<Profile<'a>, String> {
        if !folder_exists(folder) {
            return Err(format!("Profile '{folder}' does not exist."));
        }

        if !script_exists(&format!("{folder}/dit.sh"))
            || !script_exists(&format!("{folder}/dah.sh"))
        {
            return Err(format!(
                "Profile {folder} does not contain dit.sh or dah.sh."
            ));
        }

        Ok(Self { folder })
    }

    pub fn dit_child_command(&self) -> Command {
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(format!("{}/dit.sh", self.folder));
        cmd
    }

    pub fn dah_child_command(&self) -> Command {
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(format!("{}/dah.sh", self.folder));
        cmd
    }
}

fn exists(path: &str) -> bool {
    matches!(std::fs::exists(path), Ok(true))
}

fn folder_exists(path: &str) -> bool {
    exists(path) && std::path::Path::new(path).is_dir()
}

fn script_exists(script: &str) -> bool {
    exists(script) && std::path::Path::new(script).is_file()
}
