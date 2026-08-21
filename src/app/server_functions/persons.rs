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

        pub async fn retrieve_all_persons() -> Vec<Person>{

            let get_all_persons_result = database::get_all_persons().await;
            match get_all_persons_result{
                Some(found_persons) => found_persons,
                None => Vec::new()

                }
            }
        }
    }