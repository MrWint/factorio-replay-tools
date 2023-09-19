use std::{fmt::Display, io::{Read, Write}, path::Path};

use zip::write::FileOptions;


const FREEPLAY_LUA: &[u8] = include_bytes!("../../data/freeplay.lua");
const CONTROL_LUA: &[u8] = include_bytes!("../../data/control.lua");
const SAVES_DIR: &str = "/mnt/c/Users/chris/AppData/Roaming/Factorio/saves";

#[derive(Debug)]
pub enum SaveFileError {
  ZipError(zip::result::ZipError),
  IoError(std::io::Error),
}
impl From<std::io::Error> for SaveFileError {
  fn from(value: std::io::Error) -> Self {
    SaveFileError::IoError(value)
  }
}
impl From<zip::result::ZipError> for SaveFileError {
  fn from(value: zip::result::ZipError) -> Self {
    SaveFileError::ZipError(value)
  }
}
impl std::error::Error for SaveFileError {}
impl Display for SaveFileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SaveFileError::IoError(e) => e.fmt(f),
      SaveFileError::ZipError(e) => e.fmt(f),
    }
  }
}

pub struct SaveFile {
  pub level_init_dat: Vec<u8>,
  pub replay_dat: Vec<u8>,
}
impl SaveFile {
  fn from_raw_dat(level_init_dat: Vec<u8>, replay_dat: Vec<u8>) -> SaveFile {
    SaveFile { level_init_dat, replay_dat }
  }

  pub fn load_save_file(name: &str) -> std::result::Result<SaveFile, SaveFileError> {
    let save_file_path = Path::new(SAVES_DIR).join(format!("{}.zip", name));
    let save_file = std::fs::File::open(&save_file_path)?;
    let mut archive = zip::ZipArchive::new(save_file)?;
    let level_init_dat = {
      let file_name = archive.file_names().find(|s| s.ends_with("/level-init.dat")).unwrap().to_owned();
      let mut zip_file = archive.by_name(&file_name)?;
      let mut buf = Vec::with_capacity(zip_file.size() as usize);
      zip_file.read_to_end(&mut buf)?;
      buf
    };
    let replay_dat = {
      let file_name = archive.file_names().find(|s| s.ends_with("/replay.dat")).unwrap().to_owned();
      let mut zip_file = archive.by_name(&file_name)?;
      let mut buf = Vec::with_capacity(zip_file.size() as usize);
      zip_file.read_to_end(&mut buf)?;
      buf
    };
  
    Ok(SaveFile::from_raw_dat(level_init_dat, replay_dat))
  }

  pub fn write_save_file(&self, name: &str) -> std::result::Result<(), SaveFileError> {
    let save_file_path = Path::new(SAVES_DIR).join(format!("{}.zip", name));
    let new_save_file = std::fs::File::create(&save_file_path)?;
    let mut save_file_zip = zip::ZipWriter::new(new_save_file);

    save_file_zip.start_file(&format!("{}/control.lua", name), FileOptions::default())?;
    save_file_zip.write_all(CONTROL_LUA)?;
    save_file_zip.start_file(&format!("{}/freeplay.lua", name), FileOptions::default())?;
    save_file_zip.write_all(FREEPLAY_LUA)?;
    save_file_zip.start_file(&format!("{}/level-init.dat", name), FileOptions::default())?;
    save_file_zip.write_all(&self.level_init_dat)?;
    save_file_zip.start_file(&format!("{}/level.dat", name), FileOptions::default())?;
    save_file_zip.write_all(&self.level_init_dat)?;
    save_file_zip.start_file(&format!("{}/replay.dat", name), FileOptions::default())?;
    save_file_zip.write_all(&self.replay_dat)?;

    Ok(())
  }
}
