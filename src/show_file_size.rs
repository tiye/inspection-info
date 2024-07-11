use bytesize::ByteSize;
use parse_size::parse_size;

use crate::args::InspectForFileSize;
use walkdir::WalkDir;

pub fn show_file_size(options: InspectForFileSize) -> Result<(), String> {
  let min_size = parse_size(&options.min).expect(&format!("parse file size from string: {}", &options.min));

  if options.sort {
    let mut file_and_size: Vec<(u64, String)> = Vec::new();
    for entry in WalkDir::new(options.base) {
      let entry = entry.map_err(|e| e.to_string())?;
      // get file size
      let the_size = entry.metadata().map_err(|e| e.to_string())?.len();
      if the_size < min_size {
        continue;
      }
      file_and_size.push((the_size, entry.path().display().to_string()));
    }
    file_and_size.sort_by(|a, b| a.0.cmp(&b.0));
    for (size, path) in file_and_size {
      println!("{} {}", ByteSize(size), path);
    }
  } else {
    for entry in WalkDir::new(options.base) {
      let entry = entry.map_err(|e| e.to_string())?;
      // get file size
      let the_size = entry.metadata().map_err(|e| e.to_string())?.len();
      if the_size < min_size {
        continue;
      }
      println!("{} {}", ByteSize(the_size), entry.path().display());
    }
  }

  Ok(())
}
