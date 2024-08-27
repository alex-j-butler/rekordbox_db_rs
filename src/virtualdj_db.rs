use serde::{Deserialize, Serialize};

use crate::rekordbox_db::RekordboxDb;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "VirtualDJ_Database")]
pub struct VirtualDJDatabase {
    #[serde(rename = "@Version")]
    version: String,
    #[serde(rename = "Song")]
    songs: Vec<Song>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Song {
    #[serde(rename = "@FilePath")]
    FilePath: String,
    #[serde(rename = "@FileSize")]
    FileSize: usize,

    Tags: Tags,
    Infos: Infos,
    Scan: Scan,
    Poi: Poi,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tags {
    // Author="Joy Orbison" Title="Flight Fm" Genre="Electro" Year="2024" Flag="1"
    #[serde(rename = "@Title")]
    Title: String,
    #[serde(rename = "@Genre")]
    Genre: String,
    #[serde(rename = "@Year")]
    Year: String,
    #[serde(rename = "@Flag")]
    Flag: i32,
    #[serde(rename = "@Author")]
    Author: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Infos {
    // SongLength="247.820975" LastModified="1724026202" FirstSeen="1723009108" FirstPlay="1723009857"
    // LastPlay="1724026202" PlayCount="5" Bitrate="1071" Cover="1"
    #[serde(rename = "@SongLength")]
    SongLength: f32,

    #[serde(rename = "@LastModified")]
    LastModified: i32,

    #[serde(rename = "@FirstSeen")]
    FirstSeen: i32,

    #[serde(rename = "@FirstPlay")]
    FirstPlay: i32,

    #[serde(rename = "@LastPlay")]
    LastPlay: i32,

    #[serde(rename = "@PlayCount")]
    PlayCount: i32,

    #[serde(rename = "@Bitrate")]
    Bitrate: i32,

    #[serde(rename = "@Cover")]
    Cover: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Scan {
    // Version="801" Bpm="0.428571" AltBpm="0.321429" Volume="1.009624" Key="D" Flag="32768"
    #[serde(rename = "@Version")]
    Version: i32,

    #[serde(rename = "@Bpm")]
    Bpm: f32,

    #[serde(rename = "@AltBpm")]
    AltBpm: f32,

    #[serde(rename = "@Volume")]
    Volume: f32,

    #[serde(rename = "@Key")]
    Key: String,

    #[serde(rename = "@Flag")]
    Flag: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Poi {
    // <Poi Pos="0.547" Type="beatgrid" />
    #[serde(rename = "@Pos")]
    Pos: f32,

    #[serde(rename = "@Type")]
    Type: String,
}

pub fn get_virtualdj_db() -> Result<String, quick_xml::DeError> {
    let database = VirtualDJDatabase {
        version: "2024".to_string(),
        songs: vec![Song {
            FilePath: "test.mp3".to_string(),
            FileSize: 1000,
            Tags: Tags {
                Title: "Test Title".to_string(),
                Genre: "".to_string(),
                Year: "".to_string(),
                Flag: 1,
                Author: "".to_string(),
            },
            Infos: Infos {
                SongLength: 250.0,
                LastModified: 1,
                FirstSeen: 1,
                FirstPlay: 1,
                LastPlay: 1,
                PlayCount: 1,
                Bitrate: 1000,
                Cover: 1,
            },
            Scan: Scan {
                Version: 801,
                Bpm: 0.320,
                AltBpm: 0.320,
                Volume: 1.0,
                Key: "A".to_string(),
                Flag: 32768,
            },
            Poi: Poi { Pos: 0.547, Type: "beatgrid".to_string() },
        }],
    };

    // return to_string(&database);
    quick_xml::se::to_string(&database)
}

pub fn get_virtualdj_db_from_rb(rekordbox: &mut RekordboxDb) -> Result<String, quick_xml::DeError> {
    let s = rekordbox.songs.iter().map(|x|
        Song {
            FilePath: x.1.file_path.clone(),
            FileSize: x.1.file_size,
            Tags: Tags {
                Title: x.1.title.clone(),
                Genre: "".to_string(),
                Year: "".to_string(),
                Flag: 1,
                Author: "".to_string(),
            },
            Infos: Infos {
                SongLength: x.1.length as f32,
                LastModified: 1,
                FirstSeen: 1,
                FirstPlay: 1,
                LastPlay: 1,
                PlayCount: 1,
                Bitrate: 1000,
                Cover: 1,
            },
            Scan: Scan {
                Version: 801,
                Bpm: calc_virtualdj_bpm(x.1.bpm),
                AltBpm: calc_virtualdj_bpm(x.1.bpm),
                Volume: 1.0,
                Key: "A".to_string(),
                Flag: 32768,
            },
            Poi: Poi { Pos: calc_virtualdj_firstbeat(x.1.first_beat), Type: "beatgrid".to_string() },
        }
    ).collect::<Vec<_>>();
    
    let database = VirtualDJDatabase {
        version: "2024".to_string(),
        songs: s,
    };

    quick_xml::se::to_string(&database)
}

pub fn calc_virtualdj_bpm(rb_bpm: f32) -> f32 {
    return 1.0 / (rb_bpm / 60.0);
}

pub fn calc_virtualdj_firstbeat(rb_firstbeat: f32) -> f32 {
    return rb_firstbeat / 1000.0;
}
