use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use itertools::Itertools;

pub struct History {
    path: Option<PathBuf>,
    start_len: usize,
    history: Vec<String>,
}

impl History {
    pub fn from_file(path: PathBuf) -> Self {
        let history = fs::read_to_string(path.as_path()).map_or(Default::default(), |string| {
            string
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_owned)
                .collect_vec()
        });
        Self {
            path: path.into(),
            start_len: history.len(),
            history,
        }
    }

    pub fn in_memory() -> Self {
        Self {
            path: None,
            start_len: 0,
            history: Default::default(),
        }
    }

    pub fn push(&mut self, cmd: String) {
        if self.history.last() != Some(&cmd) {
            self.history.push(cmd);
        }
    }

    pub fn get(&self, idx: usize) -> Option<&String> {
        let idx = self.history.len().saturating_sub(1).saturating_sub(idx);
        self.history.get(idx)
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.history.iter().rev()
    }
}

impl Drop for History {
    fn drop(&mut self) {
        if let Some(path) = self.path.take()
            && let Ok(mut file) = OpenOptions::new()
                .read(false)
                .append(true)
                .create(true)
                .open(path)
        {
            for line in self.history.drain(..).skip(self.start_len) {
                let _ = writeln!(file, "{line}");
            }
        }
    }
}

pub fn enforce_line_limit(path: impl AsRef<Path>, limit: usize) {
    if let Ok(content) = fs::read_to_string(&path) {
        let lines = content.lines().count();
        if lines > limit {
            let skips = lines - limit;

            if let Ok(mut file) = File::create(path) {
                content.lines().skip(skips).for_each(|line| {
                    let _ = writeln!(file, "{line}");
                })
            }
        }
    }
}
