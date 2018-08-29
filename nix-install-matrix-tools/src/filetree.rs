
use std::io;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;
use std::path::Path;

#[derive(Debug)]
pub struct FileTree {
    pub files: HashMap<String, FileTreeNode>
}

#[derive(Debug)]
pub enum FileTreeNode {
    File(String, PathBuf),
    Directory(String, FileTree)
}

impl FileTree {
    pub fn new(start: &Path) -> Result<FileTreeNode, io::Error> {
        let filename = start.file_name()
            .expect("why can't we find a filename")
            .to_owned();
        let filename_string = filename.to_string_lossy().to_string();

        if start.is_file() {
            return Ok(FileTreeNode::File(filename_string, start.to_path_buf()))
        } else {
            return Ok(FileTreeNode::Directory(filename_string, FileTree {
                files: fs::read_dir(start)?
                    .collect::<Result<Vec<_>, io::Error>>()?
                    .into_iter()
                    .map::<Result<(String, FileTreeNode), io::Error>, _>(|entry| {
                        Ok((
                            entry.file_name().to_string_lossy().to_string(),
                            FileTree::new(&entry.path())?
                        ))
                    })
                    .collect::<Result<HashMap<String, FileTreeNode>, io::Error>>()?,
            }))
        }
    }
}
