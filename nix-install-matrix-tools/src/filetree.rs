
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

#[derive(Debug)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct DirectoryNode {
    pub name: String,
    pub subtree: FileTree,
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

    /// Removes a directory entry from the FileTree and returns it as a
    /// DirectoryNode
    pub fn directory(&mut self, name: &str) -> Option<DirectoryNode> {
        if let Some(FileTreeNode::Directory(name, subtree)) = self.files.remove(name) {
            return Some(DirectoryNode {
                name: name,
                subtree: subtree,
            });
        }

        return None;
    }


    /// Split a FileTree's top level files in to a list of files and a
    /// list of directories
    pub fn partition(self) -> (Vec<FileNode>, Vec<DirectoryNode>) {
        let mut files: Vec<FileNode> = vec![];
        let mut directories: Vec<DirectoryNode> = vec![];

        for (_, file) in self.files {
            match file {
                FileTreeNode::File(name, path) => {
                    files.push(FileNode {
                        name: name,
                        path: path,
                    });
                }
                FileTreeNode::Directory(name, subtree) => {
                    directories.push(DirectoryNode {
                        name: name,
                        subtree: subtree,
                    });
                }
            }
        }

        return (files, directories);
    }
}
