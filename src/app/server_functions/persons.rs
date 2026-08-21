use crate::app::models::person::Person;
use leptos::*;
use serde::*;

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError>{

    let persons = retrieve_all_persons().await;
    Ok(persons)


    }