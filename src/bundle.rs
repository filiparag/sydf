use std::fmt;
use std::io;
use std::path::PathBuf;
use std::fs::{read_to_string, write, create_dir_all};
use serde_derive::{Serialize, Deserialize};

use crate::path::{norm_from};

#[derive(Serialize, Deserialize, Debug)]
struct BundleFileAbout {
    name: String,
    author: String,
    version: String,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct BundleFileConfig {
    root: PathBuf,
    ignore: Vec<PathBuf>,
    directories: Vec<PathBuf>,
    modules: Vec<PathBuf>
}

#[derive(Serialize, Deserialize, Debug)]
struct BundleFile {
    about: BundleFileAbout,
    config: BundleFileConfig,
}

pub struct Bundle {
    values: BundleFile,
    location: PathBuf
}

#[derive(Debug)]
pub enum BundleError {
    Io(io::Error),
    Parse(toml::ser::Error),
    Save(toml::de::Error)
}

impl From<io::Error> for BundleError {
    fn from(err: io::Error) -> BundleError {
        BundleError::Io(err)
    }
}

impl From<toml::ser::Error> for BundleError {
    fn from(err: toml::ser::Error) -> BundleError {
        BundleError::Parse(err)
    }
}

impl From<toml::de::Error> for BundleError {
    fn from(err: toml::de::Error) -> BundleError {
        BundleError::Save(err)
    }
}

const ROOTDIR: &str = "/";
const SYDFDIR: &str = ".sydf";
const BUNDLE: &str = "bundle.toml";

impl Bundle {

    // TODO: replace hardcoded values
    pub fn new(path: &str) -> Result<Bundle, BundleError> {
        let abspath = norm_from(path);
        let bundle = Bundle {
            values: BundleFile {
                about: BundleFileAbout {
                    name: "linux".to_string(),
                    author: "torvals".to_string(),
                    version: "5.9.2".to_string(),
                    url: "git::kernel.org".to_string()
                },
                config: BundleFileConfig {
                    root: PathBuf::from(ROOTDIR),
                    ignore: vec![],
                    directories: vec![],
                    modules: vec![]
                }
            },
            location: abspath
        };
        Ok(bundle)
    }

    pub fn from(path: &str) -> Result<Bundle, BundleError> {
        let abspath = norm_from(path);
        let mut bundlefile = abspath.clone();
        bundlefile.push(SYDFDIR);
        bundlefile.push(BUNDLE);
        let file = read_to_string(&bundlefile)?;
        let values = toml::from_str(&file)?;
        let bundle = Bundle {
            values: values,
            location: abspath
        };
        Ok(bundle)
    }

    pub fn save(&self) -> Result<(), BundleError> {
        let data = toml::to_string_pretty(&self.values)?;
        let mut bundlefile = self.location.clone();
        bundlefile.push(SYDFDIR);
        create_dir_all(&bundlefile)?;
        bundlefile.push(BUNDLE);
        write(bundlefile.as_path().display().to_string(), data)?;
        Ok(())
    }

}

impl fmt::Display for Bundle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?}", self.location)?;
        writeln!(f, "{:?}", self.values)?;
        Ok(())
    }
}