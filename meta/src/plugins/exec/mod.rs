use anyhow::Result;
use metarepo_core::MetaConfig;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::Arc;

pub mod iterator;
pub mod plugin;

// Export the plugin
use crate::plugins::shared::{OutputManager, ProgressIndicator};
pub use iterator::{ProjectInfo, ProjectIterator};
pub use plugin::ExecPlugin;

/// Options for executing commands
#[derive(Default)]
pub struct ExecuteOptions {
    pub include_main: bool,
    pub parallel: bool,
    pub no_progress: bool,
    pub streaming: bool,
    pub verbose: bool,
    pub show_progress: bool,
    pub stop_on_fail: bool,
}

pub fn execute_command_in_directory<P: AsRef<Path>>(
    command: &str,
    args: &[&str],
    directory: P,
    verbose: bool,
) -> Result<()> {
    let dir = directory.as_ref();
    if verbose {
        println!("\n=== Executing in {} ===", dir.display());
        println!("Command: {} {}", command, args.join(" "));
    }

    let mut cmd = Command::new(command);
    cmd.args(args)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn()?;

    // Read stdout in real-time
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    // Wait for the process to complete
    let status = child.wait()?;

    if !status.success() {
        // Read stderr if command failed
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                eprintln!("ERROR: {}", line?);
            }
        }
        return Err(anyhow::anyhow!(
            "Command failed with exit code: {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(())
}

pub fn execute_with_iterator(
    command: &str,
    args: &[&str],
    iterator: ProjectIterator,
    options: ExecuteOptions,
) -> Result<()> {
    let projects: Vec<_> = iterator.collect();

    if projects.is_empty() && !options.include_main {
        println!("No projects matched the criteria");
        return Ok(());
    }

    if options.verbose {
        println!("Command: {} {}", command, args.join(" "));
    }
    if options.parallel {
        println!("Mode: Parallel execution");
    }
    println!();

    // Execute in main repository if requested
    if options.include_main {
        let meta_file =
            MetaConfig::find_meta_file().ok_or_else(|| anyhow::anyhow!("No .meta file found"))?;
        let base_path = meta_file.parent().unwrap();

        println!("=== Main Repository ===");
        if let Err(e) = execute_command_in_directory(command, args, base_path, options.verbose) {
            eprintln!("Failed in main repository: {}", e);
            if options.stop_on_fail {
                return Err(anyhow::anyhow!("Stopping due to failure in main repository"));
            }
        }
    }

    // Execute in projects
    if options.parallel && projects.len() > 1 && !options.streaming {
        // Use buffered output for parallel execution
        let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
        let output_manager = Arc::new(OutputManager::new(project_names));
        let mut progress_indicator = ProgressIndicator::new(
            Arc::clone(&output_manager),
            format!("{} {}", command, args.join(" ")),
        );

        if options.verbose {
            println!(
                "Executing command in {} project(s) [parallel mode]",
                projects.len()
            );
            println!("Command: {} {}", command, args.join(" "));
        }

        if !options.no_progress {
            progress_indicator.start();
        }

        use std::thread;
        let mut handles = vec![];

        for project in projects.clone() {
            let cmd = command.to_string();
            let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            let output_manager_clone = Arc::clone(&output_manager);
            let project_name = project.name.clone();

            let handle = thread::spawn(move || {
                output_manager_clone.start_project(&project_name);

                if !project.exists {
                    let error_msg = "Directory does not exist, skipping";
                    output_manager_clone.complete_project(
                        &project_name,
                        -1,
                        Vec::new(),
                        error_msg.as_bytes().to_vec(),
                    );
                    return;
                }

                let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                match execute_command_in_directory_buffered(&cmd, &args_refs, &project.path) {
                    Ok((exit_code, stdout, stderr, command_str)) => {
                        output_manager_clone.set_project_command(&project_name, command_str);
                        output_manager_clone.complete_project(
                            &project_name,
                            exit_code,
                            stdout,
                            stderr,
                        );
                    }
                    Err(e) => {
                        let error_msg = format!("Error: {}", e);
                        output_manager_clone.complete_project(
                            &project_name,
                            -1,
                            Vec::new(),
                            error_msg.into_bytes(),
                        );
                    }
                }
            });
            handles.push(handle);
        }

        // Wait for handles and check for failures if stop_on_fail is enabled
        if options.stop_on_fail {
            for handle in handles {
                handle.join().unwrap();
                // Check if any project has failed
                let (_, _, failed_count) = output_manager.get_status_summary();
                if failed_count > 0 {
                    // Find the first failed project by checking each project
                    let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
                    let mut failed_project = None;
                    for project_name in &project_names {
                        if let Some(output) = output_manager.get_project_output(project_name) {
                            if matches!(output.status, crate::plugins::shared::JobStatus::Failed) {
                                failed_project = Some(output.name.clone());
                                break;
                            }
                        }
                    }
                    
                    if let Some(failed_project) = failed_project {
                        // Stop progress indicator
                        if !options.no_progress {
                            progress_indicator.stop();
                        } else {
                            print!("\r\x1b[K");
                        }
                        output_manager.display_final_results(options.verbose);
                        return Err(anyhow::anyhow!(
                            "Stopping due to failure in project '{}'",
                            failed_project
                        ));
                    }
                }
            }
        } else {
            for handle in handles {
                handle.join().unwrap();
            }
        }

        // Stop progress indicator and display results
        if !options.no_progress {
            progress_indicator.stop();
        } else {
            // Clear any partial output and show completion without progress
            print!("\r\x1b[K");
        }
        output_manager.display_final_results(options.verbose);

        // Check for failures after all completed
        if options.stop_on_fail {
            let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
            for project_name in &project_names {
                if let Some(output) = output_manager.get_project_output(project_name) {
                    if matches!(output.status, crate::plugins::shared::JobStatus::Failed) {
                        return Err(anyhow::anyhow!(
                            "Command failed in project '{}'",
                            output.name
                        ));
                    }
                }
            }
        }

        return Ok(());
    } else {
        for (idx, project) in projects.iter().enumerate() {
            if options.show_progress {
                println!("[{}/{}] {}", idx + 1, projects.len(), project.name);
            } else {
                println!("{}", project.name);
            }

            if !project.exists {
                println!("  WARNING: Directory does not exist, skipping");
                continue;
            }

            if let Err(e) =
                execute_command_in_directory(command, args, &project.path, options.verbose)
            {
                eprintln!("  ERROR: Failed: {}", e);
                if options.stop_on_fail {
                    return Err(anyhow::anyhow!(
                        "Stopping due to failure in project '{}'",
                        project.name
                    ));
                }
            } else if options.verbose {
                println!("  OK: Success");
            }
        }
    }

    Ok(())
}

/// Execute command in directory with buffered output (for parallel execution)
pub fn execute_command_in_directory_buffered<P: AsRef<Path>>(
    command: &str,
    args: &[&str],
    directory: P,
) -> Result<(i32, Vec<u8>, Vec<u8>, String)> {
    let dir = directory.as_ref();
    let command_str = if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    };

    let mut cmd = Command::new(command);
    cmd.args(args).current_dir(dir);

    let output = cmd.output()?;

    Ok((
        output.status.code().unwrap_or(-1),
        output.stdout,
        output.stderr,
        command_str,
    ))
}

pub fn execute_in_all_projects(command: &str, args: &[&str]) -> Result<()> {
    let meta_file = MetaConfig::find_meta_file()
        .ok_or_else(|| anyhow::anyhow!("No .meta file found. Run 'meta init' first."))?;

    let config = MetaConfig::load_from_file(&meta_file)?;
    let base_path = meta_file.parent().unwrap();

    let iterator = ProjectIterator::new(&config, base_path);
    execute_with_iterator(
        command,
        args,
        iterator,
        ExecuteOptions {
            include_main: true,
            ..Default::default()
        },
    )
}

pub fn execute_in_specific_projects(
    command: &str,
    args: &[&str],
    project_names: &[&str],
    verbose: bool,
) -> Result<()> {
    let meta_file = MetaConfig::find_meta_file()
        .ok_or_else(|| anyhow::anyhow!("No .meta file found. Run 'meta init' first."))?;

    let config = MetaConfig::load_from_file(&meta_file)?;
    let base_path = meta_file.parent().unwrap();

    for project_name in project_names {
        if config.projects.contains_key(*project_name) {
            let project_path = base_path.join(project_name);
            if project_path.exists() {
                println!("\n=== {} ===", project_name);
                if let Err(e) = execute_command_in_directory(command, args, &project_path, verbose) {
                    eprintln!("Failed in project '{}': {}", project_name, e);
                }
            } else {
                println!("\n=== {} ===", project_name);
                println!("Project directory not found, skipping");
            }
        } else {
            eprintln!(
                "Project '{}' not found in .meta configuration",
                project_name
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_options_default() {
        let options = ExecuteOptions::default();
        assert!(!options.include_main);
        assert!(!options.parallel);
        assert!(!options.no_progress);
        assert!(!options.streaming);
        assert!(!options.verbose);
        assert!(!options.show_progress);
        assert!(!options.stop_on_fail);
    }

    #[test]
    fn test_execute_options_stop_on_fail() {
        let options = ExecuteOptions {
            stop_on_fail: true,
            ..Default::default()
        };
        assert!(options.stop_on_fail);
    }
}
