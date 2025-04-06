pub mod hooks {
    use git2::Repository;
    use regex::Regex;
    use std::{error::Error, fs};

    pub fn run_hook(message_file: &str, repo_path: &str) -> Result<(), Box<dyn Error>> {
        let original_message = fs::read_to_string(message_file)?;

        if original_message.trim().is_empty() {
            return Ok(());
        }

        if original_message.starts_with("fixup!") || original_message.starts_with("squash!") {
            println!("commit-msg-hook: fixup or squash commit; no action.");
            return Ok(());
        }

        let repo = Repository::open(repo_path)?;

        let branch_name = repo
            .head()?
            .shorthand()
            .ok_or("commit-msg-hook: failed to determine branch name.")?
            .to_uppercase();

        let branch_regex = Regex::new(r"^([A-Z]+/)?([A-Z]+-[0-9]+)")?;
        let desired_prefix = match branch_regex
            .captures(&branch_name)
            .and_then(|captures| captures.get(2).map(|m| m.as_str().to_string()))
        {
            Some(prefix) => prefix,
            None => {
                println!("commit-msg-hook: on non-prefixed branch; no action.");
                return Ok(());
            }
        };

        let prefix_regex = Regex::new(r"^([A-Z]+-[0-9]+):? ")?;
        if let Some(captures) = prefix_regex.captures(&original_message) {
            let existing_prefix = captures
                .get(1)
                .map(|m| m.as_str())
                .expect("commit-msg-hook: failed to extract commit msg prefix.");
            return if existing_prefix == desired_prefix {
                println!("commit-msg-hook: message already prefixed correctly; no action.");
                Ok(())
            } else {
                Err(format!(
                "commit-msg-hook: the message prefix '{}' does not match the branch prefix '{}'",
                existing_prefix, desired_prefix
            )
            .into())
            };
        }

        let new_message = format!("{}: {}", desired_prefix, original_message);
        fs::write(message_file, new_message)?;
        println!("commit-msg-hook: prefix added successfully");
        Ok(())
    }
}
