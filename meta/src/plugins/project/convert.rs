use anyhow::{Context, Result};
use metarepo_core::{MetaConfig, ProjectEntry, ProjectMetadata};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

/// Convert a normal repository to a bare repository with worktrees
pub fn convert_to_bare(project_name: &str, base_path: &Path, verbose: bool) -> Result<()> {
    // Load configuration
    let meta_file_path = base_path.join(".meta");
    if !meta_file_path.exists() {
        return Err(anyhow::anyhow!(
            "No .meta file found. Run 'meta init' first."
        ));
    }

    let mut config = MetaConfig::load_from_file(&meta_file_path)?;

    // Check if project exists
    if !config.projects.contains_key(project_name) {
        return Err(anyhow::anyhow!(
            "Project '{}' not found in workspace",
            project_name
        ));
    }

    let project_path = base_path.join(project_name);

    // Check if directory exists
    if !project_path.exists() {
        return Err(anyhow::anyhow!(
            "Project directory '{}' does not exist",
            project_name
        ));
    }

    // Check if it's already a bare repository
    if config.is_bare_repo(project_name) {
        println!(
            "\n  INFO: Project is already configured as a bare repository"
        );
        return Ok(());
    }

    // Check if .git exists
    if !project_path.join(".git").exists() {
        return Err(anyhow::anyhow!(
            "Project '{}' is not a git repository",
            project_name
        ));
    }

    println!(
        "\n  WARNING: Converting to Bare Repository"
    );
    println!("  {}", "═".repeat(60));
    println!("\n  INFO: This operation will:");
    println!(
        "     • Convert {} to a bare repository",
        project_name
    );
    println!(
        "     • Create a worktree for the current branch"
    );
    println!("     • Update the .meta configuration");
    println!(
        "\n  WARNING: This operation modifies your repository structure!"
    );
    println!(
        "  Make sure you have committed all changes before proceeding."
    );

    // Check for uncommitted changes
    let status_output = Command::new("git")
        .arg("-C")
        .arg(&project_path)
        .arg("status")
        .arg("--porcelain")
        .output()
        .context("Failed to check git status")?;

    if !status_output.stdout.is_empty() {
        println!(
            "\n  ERROR: Uncommitted changes detected!"
        );
        println!(
            "     └ Commit or stash your changes first"
        );
        return Err(anyhow::anyhow!(
            "Cannot convert repository with uncommitted changes"
        ));
    }

    // Get current branch
    let branch_output = Command::new("git")
        .arg("-C")
        .arg(&project_path)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .context("Failed to get current branch")?;

    let current_branch = String::from_utf8_lossy(&branch_output.stdout)
        .trim()
        .to_string();

    if current_branch.is_empty() {
        return Err(anyhow::anyhow!("Could not determine current branch"));
    }

    println!(
        "\n  Current branch: {}",
        current_branch
    );

    // Prompt for confirmation
    use std::io::{self, Write};
    print!(
        "\n  Continue with conversion? [y/N]: "
    );
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let response = input.trim().to_lowercase();

    if response != "y" && response != "yes" {
        println!(
            "\n  INFO: Conversion cancelled"
        );
        return Ok(());
    }

    println!("\n  Starting conversion...");

    // Step 1: Move .git to .git.tmp
    println!("\n  [1/5] Backing up .git directory...");
    let git_backup = project_path.join(".git.tmp");
    std::fs::rename(project_path.join(".git"), &git_backup)
        .context("Failed to backup .git directory")?;
    if verbose {
        println!("     OK: Backed up to .git.tmp");
    }

    // Step 2: Clone as bare repository
    println!("\n  [2/5] Creating bare repository...");
    let bare_path = project_path.join(".git");

    // Clone from the backup
    let clone_output = Command::new("git")
        .arg("clone")
        .arg("--bare")
        .arg(&git_backup)
        .arg(&bare_path)
        .output();

    match clone_output {
        Ok(output) if output.status.success() => {
            println!(
                "     OK: Created bare repository"
            );
        }
        _ => {
            // Restore on failure
            println!(
                "     ERROR: Failed to create bare repository"
            );
            println!("     Restoring original .git...");
            if git_backup.exists() {
                std::fs::rename(&git_backup, project_path.join(".git")).ok();
            }
            return Err(anyhow::anyhow!("Failed to clone as bare repository"));
        }
    }

    // Step 3: Create worktree for current branch
    println!(
        "\n  [3/5] Creating worktree for '{}'...",
        current_branch
    );
    let worktree_path = project_path.join(&current_branch);

    let worktree_output = Command::new("git")
        .arg("-C")
        .arg(&bare_path)
        .arg("worktree")
        .arg("add")
        .arg(&worktree_path)
        .arg(&current_branch)
        .output()
        .context("Failed to create worktree")?;

    if !worktree_output.status.success() {
        let stderr = String::from_utf8_lossy(&worktree_output.stderr);
        println!(
            "     ERROR: Failed: {}",
            stderr.trim()
        );

        // Cleanup on failure
        println!("     Cleaning up...");
        std::fs::remove_dir_all(&bare_path).ok();
        if git_backup.exists() {
            std::fs::rename(&git_backup, project_path.join(".git")).ok();
        }

        return Err(anyhow::anyhow!("Failed to create worktree"));
    }

    println!(
        "     OK: Created at {}",
        worktree_path.display()
    );

    // Step 4: Remove backup
    println!("\n  [4/5] Removing backup...");
    std::fs::remove_dir_all(&git_backup).context("Failed to remove backup")?;
    if verbose {
        println!("     OK: Backup removed");
    }

    // Step 5: Update .meta configuration
    println!("\n  [5/5] Updating .meta configuration...");

    // Get project URL
    let project_url = config
        .get_project_url(project_name)
        .ok_or_else(|| anyhow::anyhow!("Could not get project URL"))?;

    // Update to ProjectMetadata format with bare flag
    config.projects.insert(
        project_name.to_string(),
        ProjectEntry::Metadata(ProjectMetadata {
            url: project_url,
            aliases: Vec::new(),
            scripts: HashMap::new(),
            env: HashMap::new(),
            worktree_init: None,
            bare: Some(true),
        }),
    );

    config.save_to_file(&meta_file_path)?;
    if verbose {
        println!("     OK: Configuration updated");
    }

    println!("\n  {}", "─".repeat(60));
    println!(
        "  Conversion complete!"
    );
    println!("\n  Next steps:");
    println!(
        "     • Your current branch is now at: {}",
        worktree_path.display()
    );
    println!(
        "     • Create new worktrees with: meta worktree add <branch> --project {}",
        project_name
    );
    println!(
        "     • New worktrees will be created at: {}/",
        project_path.display()
    );
    println!();

    Ok(())
}
