/*
By: <Your Name Here>
Date: 2026-11-04
Program Details: <Program Description Here>
*/

mod ui;
mod utils;



  use crate::utils::database::{create_database_client, DatabaseTable};
use macroquad::prelude::*;

    use crate::ui::text_input::TextInput;
    use crate::ui::text_button::TextButton;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "userpass".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {

    

let mut txt_user = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);

let mut txt_pass = TextInput::new(100.0, 150.0, 300.0, 40.0, 25.0);

let btn_text = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Login",
        BLUE,
        BLACK,
        30
    );

    let client = create_database_client();

 
    
    loop {
        clear_background(WHITE);



        
        
        
        
        if btn_text.click() {
          
    //        // Insert a record (from user text input)
    // let new_record = DatabaseTable { id: 0, username: txt_user.get_text().to_string(), password: txt_pass.get_text().to_string() };
    // let insert_results =  client.insert_record("logins", &new_record).await;
    // if let Ok(id) = insert_results {
    //     // Inserted, id contains the new record's id
    // } else {
    //     println!("Error inserting records from database: {} ",insert_results.err().unwrap());
    // }
let mut records: Vec<DatabaseTable> = Vec::new();
    let fetched_results = client.fetch_table("logins").await;
    if let Ok(result) = fetched_results {
        records = result;
    
      } else {
       println!("Error fetching records from database: {} ",fetched_results.err().unwrap());
    }


for i in 0..records.len() {
    println!("username {}, password {}", records[i].username, records[i].password);
        }
    }
        
        
        
        
        
        
        
        
        
        
        txt_pass.draw();
        txt_user.draw();
        next_frame().await;
    }
}
