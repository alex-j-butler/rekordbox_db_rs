pub mod rekordbox_db;

#[cfg(test)]
mod tests {
    use rekordbox_db::RekordboxDb;

    use super::*;

    #[test]
    fn test_rekordbox_db() {
        let db = RekordboxDb::new_with_default_path();
        db.unwrap();
    }

    #[test]
    fn test_rekordbox_analysis() {
        let db = RekordboxDb::new_with_default_path().unwrap();

        let song = db.get_song_by_id("242299082".to_string()).unwrap();

        assert_eq!(song.title, "On My Mind (Purple Disco Machine Remix)");
        assert_eq!(song.artist, "Diplo & SIDEPIECE");
        assert_eq!(song.bpm, 123.0);
        assert_eq!(song.first_beat, 578.0);
    }
}
