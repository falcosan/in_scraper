use chrono::Local;
use std::io::Write;
use std::sync::Arc;
use serde::Serialize;
use std::path::PathBuf;
use tokio::sync::Mutex;
use crate::config::Config;
use anyhow::{ Result, Context };
use std::fs::{ self, OpenOptions };

pub struct JsonLinesPipeline {
    config: Arc<Config>,
    file_handles: Arc<Mutex<std::collections::HashMap<String, (std::fs::File, PathBuf)>>>,
}

impl JsonLinesPipeline {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            file_handles: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub async fn process_item<T: Serialize>(&self, spider_name: &str, item: T) -> Result<()> {
        let output_dir = PathBuf::from(&self.config.output_dir);
        fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

        let mut handles = self.file_handles.lock().await;

        let (file, _) = handles.entry(spider_name.to_string()).or_insert_with(|| {
            let timestamp = Local::now().format("%d_%Y%m%H%M%S");
            let filename = format!("{spider_name}_{timestamp}.jsonl");
            let filepath = output_dir.join(&filename);

            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&filepath)
                .expect("Failed to open output file");

            (file, filepath)
        });

        let json_line = serde_json::to_string(&item).context("Failed to serialize item")?;

        writeln!(file, "{json_line}").context("Failed to write to file")?;

        file.flush().context("Failed to flush file")?;

        Ok(())
    }
}
