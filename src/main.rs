mod db;
mod page;
mod asset;

fn main() {
    let conn = db::get_connection();
    println!("{}", conn);

    let homepage = page::home::home();
    println!("{}", homepage);

    let png = asset::image_png();
    println!("{}", png);

    let mp4 = asset::video_mp4();
    println!("{}", mp4);

    println!("{}", page::post::post_one::one_content());
    println!("{}", page::post::post_two::two_content());
    println!("{}", page::post::post_two::content_with_image());
}
