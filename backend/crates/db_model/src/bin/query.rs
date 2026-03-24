use self::models::*;
use diesel::prelude::*;
use db_model::*;
use chrono::Utc;

fn main() {
    use self::schema::topic::dsl::*;
    use self::schema::daily_track::dsl::*;

    let connection = &mut establish_connection();
    let results = daily_track
        // .filter(published.eq(true))
        .limit(5)
        .select(DailyTrack::as_select())
        .load(connection)
        .expect("Error loading daily track");

    println!("Displaying {} topics", results.len());
    for daily_track_item in results {
        println!("{}", daily_track_item);
        println!("-----------\n");
    }
    let data = DailyTrack::new(Utc::now().naive_utc(), None, None);
    let result = data.insert_into(schema::daily_track::table).execute(connection).expect("Error inserting daily track");
    println!("Inserted {} daily tracks", result);

    // println!("Displaying {} daily tracks", results.len());
}