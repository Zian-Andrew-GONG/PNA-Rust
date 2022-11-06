use crate::KvsEngine::KvsEngine;
use crate::{KvStoreError, Result};
use serde::{Deserialize, Serialize};
use serde_json::{self, Deserializer};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

pub struct KvStore {
    index: HashMap<String, CommandPos>,
    pub readers: HashMap<u64, BufReader<File>>,
    writer: BufWriterWithPos,
    current_file_number: u64,
    #[allow(dead_code)]
    file_path: PathBuf,
    useless_size: u64,
}
#[derive(Deserialize, Serialize)]
enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl Command {
    fn set(key: String, value: String) -> Command {
        Command::Set { key, value }
    }
    fn remove(key: String) -> Command {
        Command::Remove { key }
    }
}
#[derive(Debug)]
struct CommandPos {
    file_number: u64,
    offset: u64,
    length: u64,
}
struct BufWriterWithPos {
    writer: BufWriter<File>,
    offset: u64,
}

impl BufWriterWithPos {
    fn write(&mut self, data: &Vec<u8>) -> Result<()> {
        self.writer.write_all(data)?;
        self.writer.flush()?;
        self.offset += data.len() as u64;
        Ok(())
    }
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;

        let mut readers = HashMap::new();
        let mut index = HashMap::new();

        let file_list: Vec<u64> = load_file_list(&path)?;
        let mut useless_size = 0;

        let mut max_file_number = 0;
        for file_number in file_list {
            if file_number > max_file_number {
                max_file_number = file_number;
            }
            let path = path.join(format!("{file_number}.log"));
            let mut reader = BufReader::new(File::open(&path)?);
            // load index!
            useless_size += load(&mut index, &mut reader, file_number)?;

            readers.insert(file_number, reader);
        }

        let current_file_number = max_file_number + 1;

        let writer = BufWriterWithPos {
            writer: new_log_file(&path, current_file_number, &mut readers)?,
            offset: 0,
        };
        let file_path = path;

        Ok(KvStore {
            index,
            readers,
            writer,
            current_file_number,
            file_path,
            useless_size,
        })
    }

    fn _set(&mut self, key: String, value: String) -> Result<()> {
        let command = Command::set(key, value);
        let data = serde_json::to_vec(&command)?;
        // write json to file
        self.writer.write(&data)?;
        if let Command::Set { key, .. } = command {
            if let Some(_old_value) = self.index.insert(
                key,
                CommandPos {
                    file_number: self.current_file_number,
                    offset: self.writer.offset - data.len() as u64,
                    length: data.len() as u64,
                },
            ) {
                self.useless_size += data.len() as u64;
            }
        }
        if self.useless_size > COMPACTION_THRESHOLD {
            self.compact()?;
        }

        Ok(())
    }
    fn _get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(cmd) = self.index.get(&key) {
            let reader = self
                .readers
                .get_mut(&cmd.file_number)
                .expect("Reader not found");
            reader.seek(SeekFrom::Start(cmd.offset))?;
            let taken_reader = reader.take(cmd.length);

            if let Command::Set { key: _, value } = serde_json::from_reader(taken_reader)? {
                Ok(Some(value))
            } else {
                Err(KvStoreError::UnexpectedCommandType)
            }
        } else {
            Ok(None)
        }
    }
    fn _remove(&mut self, key: String) -> Result<()> {
        if self.index.contains_key(&key) {
            self.index.remove(&key);
            let cmd = Command::remove(key);
            let data = serde_json::to_vec(&cmd)?;
            self.writer.write(&data)?;
            self.useless_size += data.len() as u64;
            if self.useless_size > COMPACTION_THRESHOLD {
                self.compact()?;
            }
            Ok(())
        } else {
            Err(KvStoreError::KeyNotFound)
        }
    }

    fn compact(&mut self) -> Result<()> {
        let compaction_file_number = self.current_file_number + 1;

        let mut compaction_writer =
            new_log_file(&self.file_path, compaction_file_number, &mut self.readers)?;
        let mut offset = 0;
        let mut counter = 0;
        for cmd in self.index.values_mut() {
            counter += 1;
            let reader = self
                .readers
                .get_mut(&cmd.file_number)
                .expect("Reader not found");
            reader.seek(SeekFrom::Start(cmd.offset))?;
            let mut taken_reader = reader.take(cmd.length);

            let len = io::copy(&mut taken_reader, &mut compaction_writer)?;
            cmd.file_number = compaction_file_number;
            cmd.offset = offset;
            offset += len;
        }
        compaction_writer.flush()?;
        assert!(counter == self.index.len());

        let delete_list: Vec<u64> = self
            .readers
            .iter()
            .map(|(key, _)| *key)
            .filter(|key| *key <= self.current_file_number)
            .collect();
        for delete in delete_list {
            self.readers.remove(&delete);
            fs::remove_file(self.file_path.join(format!("{delete}.log")))?;
        }
        self.current_file_number += 2;
        self.writer.writer =
            new_log_file(&self.file_path, self.current_file_number, &mut self.readers)?;
        self.writer.offset = 0;
        self.useless_size = 0;

        Ok(())
    }
}

fn new_log_file(
    path: &Path,
    file_number: u64,
    readers: &mut HashMap<u64, BufReader<File>>,
) -> Result<BufWriter<File>> {
    let path = path.join(format!("{file_number}.log"));
    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&path)?;
    let writer = BufWriter::new(f);
    readers.insert(file_number, BufReader::new(File::open(&path)?));
    Ok(writer)
}

fn load_file_list(path: &Path) -> Result<Vec<u64>> {
    let mut file_list: Vec<u64> = fs::read_dir(&path)?
        .flat_map(|res| -> Result<_> { Ok(res?.path()) })
        .filter(|path| path.is_file() && path.extension() == Some("log".as_ref()))
        .flat_map(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.trim_end_matches(".log"))
                .map(str::parse::<u64>)
        })
        .flatten()
        .collect();
    file_list.sort();
    Ok(file_list)
}

fn load(
    index: &mut HashMap<String, CommandPos>,
    reader: &mut BufReader<File>,
    file_number: u64,
) -> Result<u64> {
    let mut useless = 0;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Command>();
    let mut offset: u64 = 0;
    while let Some(cmd) = stream.next() {
        let new_offset = stream.byte_offset() as u64;
        match cmd? {
            Command::Set { key, .. } => {
                if let Some(old_command) = index.insert(
                    key,
                    CommandPos {
                        file_number,
                        offset,
                        length: new_offset - offset,
                    },
                ) {
                    useless += old_command.length;
                }
                offset = new_offset;
            }
            Command::Remove { key } => {
                if let Some(old_command) = index.remove(&key) {
                    useless += old_command.length;
                }
                offset = new_offset;
            }
        }
    }
    Ok(useless)
}


impl KvsEngine for KvStore {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self._set(key, value)
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        self._get(key)
    }
    fn remove(&mut self, key: String) -> Result<()> {
        self._remove(key)
    }
}