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
    file_path: String,
    #[serde(rename = "@FileSize")]
    file_size: usize,

    #[serde(rename = "Tags")]
    tags: Tags,

    #[serde(rename = "Infos")]
    infos: Infos,

    #[serde(rename = "Scan")]
    scan: Scan,

    #[serde(rename = "Poi")]
    poi: Poi,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Tags {
    // Author="Joy Orbison" Title="Flight Fm" Genre="Electro" Year="2024" Flag="1"
    #[serde(rename = "@Title")]
    title: String,
    #[serde(rename = "@Genre")]
    genre: String,
    #[serde(rename = "@Year")]
    year: String,
    #[serde(rename = "@Flag")]
    flag: i32,
    #[serde(rename = "@Author")]
    author: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Infos {
    // SongLength="247.820975" LastModified="1724026202" FirstSeen="1723009108" FirstPlay="1723009857"
    // LastPlay="1724026202" PlayCount="5" Bitrate="1071" Cover="1"
    #[serde(rename = "@SongLength")]
    song_length: f32,

    #[serde(rename = "@LastModified")]
    last_modified: i32,

    #[serde(rename = "@FirstSeen")]
    first_seen: i32,

    #[serde(rename = "@FirstPlay")]
    first_play: i32,

    #[serde(rename = "@LastPlay")]
    last_play: i32,

    #[serde(rename = "@PlayCount")]
    play_count: i32,

    #[serde(rename = "@Bitrate")]
    bitrate: i32,

    #[serde(rename = "@Cover")]
    cover: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Scan {
    // Version="801" Bpm="0.428571" AltBpm="0.321429" Volume="1.009624" Key="D" Flag="32768"
    #[serde(rename = "@Version")]
    version: i32,

    #[serde(rename = "@Bpm")]
    bpm: f32,

    #[serde(rename = "@AltBpm")]
    alt_bpm: f32,

    #[serde(rename = "@Volume")]
    volume: f32,

    #[serde(rename = "@Key")]
    key: String,

    #[serde(rename = "@Flag")]
    flag: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Poi {
    // <Poi Pos="0.547" Type="beatgrid" />
    #[serde(rename = "@Pos")]
    pos: f32,

    #[serde(rename = "@Type")]
    poi_type: String,
}

pub fn get_virtualdj_db() -> Result<String, quick_xml::DeError> {
    let database = VirtualDJDatabase {
        version: "2024".to_string(),
        songs: vec![Song {
            file_path: "test.mp3".to_string(),
            file_size: 1000,
            tags: Tags {
                title: "Test Title".to_string(),
                genre: "".to_string(),
                year: "".to_string(),
                flag: 1,
                author: "".to_string(),
            },
            infos: Infos {
                song_length: 250.0,
                last_modified: 1,
                first_seen: 1,
                first_play: 1,
                last_play: 1,
                play_count: 1,
                bitrate: 1000,
                cover: 1,
            },
            scan: Scan {
                version: 801,
                bpm: 0.320,
                alt_bpm: 0.320,
                volume: 1.0,
                key: "A".to_string(),
                flag: 32768,
            },
            poi: Poi { pos: 0.547, poi_type: "beatgrid".to_string() },
        }],
    };

    // return to_string(&database);
    quick_xml::se::to_string(&database)
}

pub fn get_virtualdj_db_from_rb(rekordbox: &mut RekordboxDb) -> Result<String, quick_xml::DeError> {
    let s = rekordbox.songs.iter().map(|x|
        Song {
            file_path: x.1.file_path.clone(),
            file_size: x.1.file_size,
            tags: Tags {
                title: x.1.title.clone(),
                genre: "".to_string(),
                year: "".to_string(),
                flag: 1,
                author: "".to_string(),
            },
            infos: Infos {
                song_length: x.1.length as f32,
                last_modified: 1,
                first_seen: 1,
                first_play: 1,
                last_play: 1,
                play_count: 1,
                bitrate: 1000,
                cover: 1,
            },
            scan: Scan {
                version: 801,
                bpm: calc_virtualdj_bpm(x.1.bpm),
                alt_bpm: calc_virtualdj_bpm(x.1.bpm),
                volume: 1.0,
                key: "A".to_string(),
                flag: 32768,
            },
            poi: Poi { pos: calc_virtualdj_firstbeat(x.1.first_beat), poi_type: "beatgrid".to_string() },
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
