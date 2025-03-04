use std::io::Write;
use std::{collections::HashMap, process::Command};

fn main() -> anyhow::Result<()> {
    let aliases = get_zsh_aliases()?;
    let hist = get_zsh_history()?;
    let count = hist.len();

    let it = hist.into_iter().map(|(ts, cmd)| {
        let cmd = canonicalize_command(cmd, aliases.clone());
        let cmd = convert_zsh_to_fish(cmd);
        (ts, cmd)
    });

    let fish_history_path = dirs::home_dir()
        .unwrap()
        .join(".local/share/fish/fish_history");

    let backup_fish_history_path = fish_history_path.with_extension("bak");
    if fish_history_path.exists() {
        print!("Backing up {fish_history_path:?} to {backup_fish_history_path:?}... ");
        std::fs::copy(&fish_history_path, &backup_fish_history_path)?;
    }

    let mut fish_history_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&fish_history_path)?;

    for (ts, cmd) in it {
        writeln!(fish_history_file, "- cmd: {}", cmd)?;
        writeln!(fish_history_file, "  when: {}", ts)?;
    }
    println!("Wrote {count} entries to {fish_history_path:?}.");
    Ok(())
}

fn convert_zsh_to_fish(cmd: String) -> String {
    let mut fish_cmd = cmd.clone();
    if fish_cmd.contains("&&") {
        fish_cmd = fish_cmd.replace("&&", "; and");
    }
    if fish_cmd.contains("||") {
        fish_cmd = fish_cmd.replace("||", "; or");
    }
    if fish_cmd.contains(">|") {
        fish_cmd = fish_cmd.replace(">|", ">");
    }
    if fish_cmd.contains(">>|") {
        fish_cmd = fish_cmd.replace(">>|", ">>");
    }
    fish_cmd
}

// canonicalize_command expands aliases.
fn canonicalize_command(cmd: String, aliases: HashMap<String, String>) -> String {
    let arg0 = cmd.split_whitespace().next().unwrap();
    if let Some(alias) = aliases.get(arg0) {
        let mut new_cmd = alias.clone();
        if cmd.len() > arg0.len() {
            new_cmd.push(' ');
            new_cmd.push_str(&cmd[arg0.len() + 1..]);
        }
        new_cmd
    } else {
        cmd
    }
}

fn get_zsh_history() -> anyhow::Result<Vec<(usize, String)>> {
    let mut hist = Vec::new();
    Command::new("zsh")
        .arg("-i")
        .arg("-c")
        .arg("fc -R $HISTFILE; fc -l -t '%s' 0")
        .output()
        .map(|output| {
            String::from_utf8(output.stdout)
                .expect("Failed to convert output to string")
                .lines()
                .for_each(|line| {
                    let mut chunks = line.split_whitespace().skip(1);
                    let ts = chunks.next().unwrap().parse().unwrap();
                    let cmd = chunks.collect::<Vec<&str>>().join(" ");
                    hist.push((ts, cmd));
                });
            hist
        })
        .map_err(|e| e.into())
}

fn get_zsh_aliases() -> anyhow::Result<HashMap<String, String>> {
    let mut aliases = HashMap::new();
    Command::new("zsh")
        .arg("-i")
        .arg("-c")
        .arg("alias")
        .output()
        .map(|output| {
            String::from_utf8(output.stdout)
                .expect("Failed to convert output to string")
                .lines()
                .for_each(|line| match line.split_once("=") {
                    Some((alias, mut command)) => {
                        if command.starts_with("'") && command.ends_with("'") {
                            command = &command[1..command.len() - 1];
                        }
                        aliases.insert(alias.to_string(), command.to_string());
                    }
                    None => (),
                });
            aliases
        })
        .map_err(|e| e.into())
}
