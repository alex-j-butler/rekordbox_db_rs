pub mod rekordbox_db;
pub mod virtualdj_db;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rekordbox_db::RekordboxDb;
    use virtualdj_db::calc_virtualdj_bpm;

    use super::*;

    #[test]
    fn test_rekordbox_db() {
        let db = RekordboxDb::new_with_default_path();
        db.unwrap();
    }

    #[test]
    fn test_rekordbox_analysis() {
        let current_dir: PathBuf = std::env::current_dir().unwrap().join("tests/demo_db");
        let db = RekordboxDb::new(current_dir).unwrap();

        let song2 = db.get_song_by_id("81137556".to_string()).unwrap();
        assert_eq!(song2.title, "Demo Track 2", "demo track 2 name not equal");
        assert_eq!(song2.artist, "Loopmasters", "demo track 2 artist not equal");
        assert_eq!(song2.bpm, 120.0, "demo track 2 bpm not equal");
        assert_eq!(song2.first_beat, 25.0, "demo track 2 first beat not equal");

        let song1 = db.get_song_by_id("249743749".to_string()).unwrap();
        assert_eq!(song1.title, "Demo Track 1", "demo track 1 name not equal");
        assert_eq!(song1.artist, "Loopmasters", "demo track 1 artist not equal");
        assert_eq!(song1.bpm, 128.0, "demo track 1 bpm not equal");
        assert_eq!(song1.first_beat, 25.0, "demo track 1 first beat not equal");
    }


    /*
    #[test]
    fn test_virtualdj_save() {
        let current_dir: PathBuf = std::env::current_dir().unwrap().join("tests/demo_db");
        let mut db = RekordboxDb::new(current_dir.clone()).unwrap();
        let db = get_virtualdj_db_from_rb(&mut db).unwrap();

        let db_xml = String::from_utf8(fs::read(current_dir.join("database.xml")).unwrap()).unwrap();

        assert_eq!(db.clone(), db_xml);
    }
    */

    #[test]
    fn test_bpm_calc() {
        assert_eq!(calc_virtualdj_bpm(140.0), 0.42857143);
    }
}
