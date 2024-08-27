pub mod rekordbox_db;
pub mod virtualdj_db;

#[cfg(test)]
mod tests {
    use std::fs;

    use rekordbox_db::RekordboxDb;
    use virtualdj_db::{calc_virtualdj_bpm, get_virtualdj_db_from_rb};

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

    #[test]
    fn test_virtualdj_save() {
        let mut db = RekordboxDb::new_with_default_path().unwrap();
        let db = get_virtualdj_db_from_rb(&mut db).unwrap();

        fs::write("./database.xml", db.clone()).expect("Unable to write file");

        assert_eq!(db, "");
    }

    #[test]
    fn test_bpm_calc() {
        assert_eq!(calc_virtualdj_bpm(140.0), 0.42857143);
    }
}
