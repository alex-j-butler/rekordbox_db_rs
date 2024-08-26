extern crate rusqlite;
use binrw::BinRead as _;
use rekordcrate::anlz::ANLZ;
use rusqlite::{Connection, OpenFlags};

use std::{collections::HashMap, env::VarError, error::{self, Error}, path::{PathBuf}};

type DbResult<T> = std::result::Result<T, RekordboxDbError>;
type Result<T> = std::result::Result<T, RekordboxError>;

#[derive(Debug, Clone)]
pub struct RekordboxError;

#[derive(Debug)]
pub enum RekordboxDbError {
    Unknown,
    Path,
    Env(VarError),
    Serialise(binrw::Error),
    IOError(std::io::Error),
}

impl std::fmt::Display for RekordboxDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            RekordboxDbError::Unknown =>
                write!(f, "unknown error"),
            RekordboxDbError::Path =>
                write!(f, "path error"),
            // The wrapped error contains additional information and is available
            // via the source() method.
            RekordboxDbError::Env(..) =>
                write!(f, "the local appdata path could not be determined: {:?}", self.source()),
                RekordboxDbError::Serialise(..) =>
                write!(f, "serialisation error: {:?}", self.source()),
                RekordboxDbError::IOError(..) =>
                write!(f, "io error: {:?}", self.source()),
        }
    }
}
impl error::Error for RekordboxDbError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RekordboxDbError::Unknown => None,
            RekordboxDbError::Path => None,
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            RekordboxDbError::Env(ref e) => Some(e),
            RekordboxDbError::Serialise(ref e) => Some(e),
            RekordboxDbError::IOError(ref e) => Some(e),
        }
    }
}
impl From<VarError> for RekordboxDbError {
    fn from(err: VarError) -> RekordboxDbError {
        RekordboxDbError::Env(err)
    }
}
impl From<binrw::Error> for RekordboxDbError {
    fn from(err: binrw::Error) -> RekordboxDbError {
        RekordboxDbError::Serialise(err)
    }
}
impl From<std::io::Error> for RekordboxDbError {
    fn from(err: std::io::Error) -> RekordboxDbError {
        RekordboxDbError::IOError(err)
    }
}

impl RekordboxDb {

    pub fn new_with_default_path() -> DbResult<Self> {
        let appdata = std::env::var("APPDATA")?;
        let mut db_path = std::path::PathBuf::new();
        db_path.push(appdata);
        db_path.push("Pioneer/rekordbox/master.db");

        let path = db_path.to_str().ok_or(
            RekordboxDbError::Path
        )?;
        Self::new(path)
    }

    pub fn new(path: &str) -> DbResult<Self> {
        let mut songs_map:HashMap<String, RekordboxAnalysis> = HashMap::new();

        let conn_result = 
            Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY);
        let conn = match conn_result {
            Ok(conn) => conn,
            Err(error) => panic!("Rekordbox DB error: {error:?}")
        };
        conn.pragma_update(None, "cipher_compatibility", 4).unwrap();
        conn.pragma_update(None, "key", "402fd482c38817c35ffa8ffb8c7d93143b749e7d315df7a81732a1ff43608497").unwrap();
        
        let mut stmt: rusqlite::Statement = conn.prepare(
            "SELECT djmdContent.ID, djmdContent.AnalysisDataPath, djmdContent.FolderPath, djmdContent.FileNameL, djmdContent.FileSize, djmdContent.Title, IFNULL(djmdArtist.Name, \"\") AS ArtistName, `Length` FROM djmdContent LEFT JOIN djmdArtist ON djmdContent.ArtistID == djmdArtist.ID;").unwrap();
        let rows_result = stmt.query_map([], |row| {
            Ok(RekordboxAnalysis {
                id: row.get(0)?,
                analysis_path: row.get(1)?,
                file_path: row.get(2)?,
                file_name: row.get(3)?,
                file_size: row.get(4)?,
                title: row.get(5)?,
                artist: row.get(6)?,
                length: row.get(7)?,
                bpm: 0.0,
                first_beat: 0.0,
            })
        });

        let rows = match rows_result {
            Ok(rows) => rows,
            Err(error) => panic!("Rekordbox DB select error: {error:?}")
        };

        let appdata = std::env::var("APPDATA")?;
        let mut db_path = std::path::PathBuf::new();
        db_path.push(appdata);
        db_path.push("Pioneer/rekordbox/share");

        for song in rows {
            let mut s = song.unwrap();

            // Get the analysis file:
            
            let _ = get_song_analysis(&mut s, db_path.clone());

            // println!("ID: {}, Title: {}", s.id, s.title);
            songs_map.insert(s.id.clone(), s);
        }

        Ok(Self {
            songs: songs_map,
        })
    }

    pub fn get_song_by_id(&self, id: String) -> Result<&RekordboxAnalysis> {
        let song = self.songs.get(&id);
        return song.ok_or_else(|| RekordboxError {})
    }

    pub fn get_title_by_id(&self, id: String) -> Result<String> {
        let title: &RekordboxAnalysis = match self.songs.get(&id) {
            None => panic!("No song found! {}", id),
            Some(title) => title,
        };
        Ok(title.title.clone())
    }
}

fn get_song_analysis(song: &mut RekordboxAnalysis, db_path: PathBuf) -> DbResult<bool> {
    let apath = song.analysis_path.clone();
    if apath.len() == 0 {
        return Ok(false)
    }

    let analysis_path = db_path.join(apath.split_at(1).1);
    let mut reader = std::fs::File::open(analysis_path)?;
    let anlz: ANLZ = ANLZ::read(&mut reader)?;
    

    let mut beat_grid_section = None;
    for section in anlz.sections {
        if section.header.kind == rekordcrate::anlz::ContentKind::BeatGrid {
            beat_grid_section = Some(section);
        }
    }

    if let Some(bg) = beat_grid_section {
        if let rekordcrate::anlz::Content::BeatGrid(beat_grid) = bg.content {
            if beat_grid.beats.len() > 1 {
                let mut beat = None;
                for n in 0..(std::cmp::min(10, beat_grid.beats.len())) {
                    let first_beat = beat_grid.beats.get(n).unwrap();

                    if first_beat.beat_number == 1 {
                        beat = Some(first_beat);
                        break
                    }
                }

                let first_beat = beat.unwrap();
                println!("{}", first_beat.beat_number);

                song.bpm = first_beat.tempo as f32 / 100.0;
                song.first_beat = first_beat.time as f32;
            } else {
                return Ok(false)
            }
        }
    }

    Ok(true)
}

#[derive(Clone)]
pub struct RekordboxDb {
    pub songs: HashMap<String, RekordboxAnalysis>,
}

#[derive(Clone)]
pub struct RekordboxSong {
    pub id: String,
    pub folder_path: String,
    pub file_name: String,
    pub title: String,
    pub analysis_path: String,
}

#[derive(Clone)]
pub struct RekordboxAnalysis {
    pub id: String,
    pub analysis_path: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: usize,

    pub title: String,
    pub artist: String,
    pub length: i32,

    pub bpm: f32,
    pub first_beat: f32,
}
