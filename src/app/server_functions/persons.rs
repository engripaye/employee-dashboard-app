use leptos::*;
use crate::app::models::Person;
use serde::*;

#[server(GetPersons, "/api")]
pub async fn get_persons() -> Result<Vec<Person>, ServerFnError>{

    let persons = retrieve_all_persons().await;

    }