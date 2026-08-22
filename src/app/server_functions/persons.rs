use crate::app::models::person::Person;
use leptos::*;
use serde::*;

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError>{

    let persons = retrieve_all_persons().await;
    Ok(persons)
    }

cfg_if::cfg_if!{

    if #[cfg(feature = "ssr")]{

        use crate::app::db::database;
        use chrono::{DateTime, Local};
        use uuid::Uuid;

        pub async fn retrieve_all_persons() -> Vec<Person>{

            let get_all_persons_result = database::get_all_persons().await;
            match get_all_persons_result{
                Some(found_persons) => found_persons,
                None => Vec::new()

                }
            }

        pub async fn add_new_person<T>(name: T, title: T, level: T, compensation:i32)
        -> Option<Person> where T: Into<String> {

            let mud buffer = Uuid::encode_buffer();
            let uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);

            // getting the current timestamp
            let current_now = Local::now();
            let current_formatted = current_now.to_String();

            let new_person = Person::new(
                String::from(uuid),
                name.into(),
                title.into(),
                level.into(),
                compensation,
                current_formatted

                );

                database::add_person(new_person).await
            }
        }
    }