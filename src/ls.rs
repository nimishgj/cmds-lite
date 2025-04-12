use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use std::time::SystemTime;
use std::os::unix::fs::PermissionsExt;

pub struct LsOptions {
    pub show_hidden: bool,
    pub long_format: bool,
}

impl Default for LsOptions {
    fn default() -> Self {
        LsOptions {
            show_hidden: false,
            long_format: false,
        }
    }
}

pub struct FileEntry {
    path: PathBuf,
    metadata: fs::Metadata,
    name: String,
}

impl FileEntry {
    fn new(dir_entry: fs::DirEntry) -> io::Result<Self> {
        let metadata = dir_entry.metadata()?;
        let name = dir_entry.file_name().to_string_lossy().to_string();
        let path = dir_entry.path();
        Ok(FileEntry { path, metadata, name })
    }

    fn is_dir(&self) -> bool {
        self.metadata.is_dir()
    }

    fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }

    fn size(&self) -> u64 {
        self.metadata.len()
    }

    fn modified_timestamp(&self) -> u64 {
        self.metadata
            .modified()
            .unwrap_or(SystemTime::now())
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    fn permissions(&self) -> u32 {
        self.metadata.permissions().mode()
    }
}

trait EntryFilter {
    fn should_include(&self, entry: &FileEntry) -> bool;
}

trait EntryFormatter {
    fn format(&self, entry: &FileEntry) -> String;
}

struct HiddenFilter {
    show_hidden: bool,
}

impl EntryFilter for HiddenFilter {
    fn should_include(&self, entry: &FileEntry) -> bool {
        self.show_hidden || !entry.is_hidden()
    }
}

struct SimpleFormatter;

impl EntryFormatter for SimpleFormatter {
    fn format(&self, entry: &FileEntry) -> String {
        if entry.is_dir() {
            format!("{}/", entry.name)
        } else {
            entry.name.clone()
        }
    }
}

struct LongFormatter;

impl EntryFormatter for LongFormatter {
    fn format(&self, entry: &FileEntry) -> String {
        let mode = entry.permissions();
        let file_type = if entry.is_dir() { "d" } else { "-" };
        
        let permissions = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            file_type,
            if (mode & 0o400) != 0 { "r" } else { "-" },
            if (mode & 0o200) != 0 { "w" } else { "-" },
            if (mode & 0o100) != 0 { "x" } else { "-" },
            if (mode & 0o040) != 0 { "r" } else { "-" },
            if (mode & 0o020) != 0 { "w" } else { "-" },
            if (mode & 0o010) != 0 { "x" } else { "-" },
            if (mode & 0o004) != 0 { "r" } else { "-" },
            if (mode & 0o002) != 0 { "w" } else { "-" },
            if (mode & 0o001) != 0 { "x" } else { "-" }
        );
        
        format!(
            "{} {:>8} {:>12} {}", 
            permissions, 
            entry.size(), 
            entry.modified_timestamp(), 
            entry.name
        )
    }
}

struct FileCollector;

impl FileCollector {
    fn collect_entries(path: &Path) -> io::Result<Vec<FileEntry>> {
        let entries = fs::read_dir(path)?;
        let mut entries_vec = Vec::new();
        
        for entry_result in entries {
            let entry = entry_result?;
            let file_entry = FileEntry::new(entry)?;
            entries_vec.push(file_entry);
        }
        
        entries_vec.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(entries_vec)
    }
}

struct FileProcessor<'a> {
    formatter: Box<dyn EntryFormatter + 'a>,
    filters: Vec<Box<dyn EntryFilter + 'a>>,
}

impl<'a> FileProcessor<'a> {
    fn process(&self, entries: Vec<FileEntry>) -> io::Result<()> {
        for entry in entries {
            if self.should_process(&entry) {
                println!("{}", self.formatter.format(&entry));
            }
        }
        
        Ok(())
    }
    
    fn should_process(&self, entry: &FileEntry) -> bool {
        self.filters.iter().all(|filter| filter.should_include(entry))
    }
}

pub fn run(dir_path: &str, options: &LsOptions) -> io::Result<()> {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Path does not exist"));
    }
    
    if !path.is_dir() {
        println!("{}", path.file_name().unwrap().to_string_lossy());
        return Ok(());
    }
    
    let entries = FileCollector::collect_entries(path)?;
    
    let formatter: Box<dyn EntryFormatter> = if options.long_format {
        Box::new(LongFormatter)
    } else {
        Box::new(SimpleFormatter)
    };
    
    let mut filters: Vec<Box<dyn EntryFilter>> = Vec::new();
    filters.push(Box::new(HiddenFilter { show_hidden: options.show_hidden }));
    
    let processor = FileProcessor {
        formatter,
        filters,
    };
    
    processor.process(entries)
}
