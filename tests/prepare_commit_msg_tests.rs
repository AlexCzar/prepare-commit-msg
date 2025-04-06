#[cfg(test)]
mod tests {
    use git2::{Commit, Repository};
    use googletest::prelude::*;
    use prepare_commit_msg::hooks::run_hook;
    use std::fs;
    use tempfile::tempdir;

    fn create_initial_commit(repo: &Repository) -> (String, Commit) {
        let mut index = repo.index().unwrap();
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let sig = repo.signature().unwrap();
        let oid = repo
            .commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])
            .unwrap();
        let default_head_name = repo.head().unwrap().name().unwrap().to_owned();
        (default_head_name, repo.find_commit(oid).unwrap())
    }

    fn setup_test(branch_name: &str) -> (tempfile::TempDir, String) {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let repo_path = temp_dir.path().to_owned();
        let repo = Repository::init(&repo_path).expect("Failed to initialize git repository");
        let (default_head_name, commit) = create_initial_commit(&repo);

        let head_name = format!("refs/heads/{}", branch_name);

        if default_head_name != head_name {
            repo.branch(branch_name, &commit, false).unwrap();
            repo.set_head(&head_name).expect("Failed to set head");
        }
        (temp_dir, repo_path.display().to_string())
    }

    #[test]
    fn adds_prefix_when_branch_has_ticket_id() -> Result<()> {
        let (_temp_dir, repo_path) = setup_test("TICKET-123");
        let msg_path = format!("{}/commit_msg", repo_path);
        fs::write(&msg_path, "test message").unwrap();

        let result = run_hook(&msg_path, &repo_path);
        assert_pred!(result.is_ok());
        assert_that!(
            fs::read_to_string(&msg_path).unwrap(),
            eq("TICKET-123: test message")
        );
        Ok(())
    }

    #[test]
    fn no_action_on_non_ticket_branches() -> Result<()> {
        let (_temp_dir, repo_path) = setup_test("main");
        let msg_path = format!("{}/commit_msg", repo_path);
        fs::write(&msg_path, "no prefix").unwrap();

        let result = run_hook(&msg_path, &repo_path);
        assert_pred!(result.is_ok());
        assert_that!(fs::read_to_string(&msg_path).unwrap(), eq("no prefix"));
        Ok(())
    }

    #[test]
    fn detects_correct_existing_prefix() -> Result<()> {
        let (_temp_dir, repo_path) = setup_test("TICKET-456");
        let msg_path = format!("{}/commit_msg", repo_path);
        fs::write(&msg_path, "TICKET-456: valid").unwrap();

        let result = run_hook(&msg_path, &repo_path);
        assert_pred!(result.is_ok());
        assert_that!(
            fs::read_to_string(&msg_path).unwrap(),
            eq("TICKET-456: valid")
        );
        Ok(())
    }

    #[test]
    fn rejects_incorrect_prefix() -> Result<()> {
        let (_temp_dir, repo_path) = setup_test("TICKET-789");
        let msg_path = format!("{}/commit_msg", repo_path);
        fs::write(&msg_path, "WRONG-123: invalid").unwrap();

        let result = run_hook(&msg_path, &repo_path);
        assert_pred!(result.is_err());
        assert_that!(
            result.unwrap_err().to_string(),
            contains_substring("does not match")
        );
        Ok(())
    }

    #[test]
    fn ignores_fixup_squash_commits() -> Result<()> {
        let (_temp_dir, repo_path) = setup_test("TICKET-000");
        let msg_path = format!("{}/commit_msg", repo_path);
        fs::write(&msg_path, "fixup! previous").unwrap();

        let result = run_hook(&msg_path, &repo_path);
        assert_pred!(result.is_ok());
        assert_that!(
            fs::read_to_string(&msg_path).unwrap(),
            eq("fixup! previous")
        );
        Ok(())
    }

    #[test]
    fn handles_semantic_branch_names() -> Result<()> {
        let (_temp_dir, repo_path) = setup_test("feature/ABC-456");
        let msg_path = format!("{}/commit_msg", repo_path);
        fs::write(&msg_path, "semantic branch").unwrap();

        let result = run_hook(&msg_path, &repo_path);
        assert_pred!(result.is_ok());
        assert_that!(
            fs::read_to_string(&msg_path).unwrap(),
            eq("ABC-456: semantic branch")
        );
        Ok(())
    }
}
