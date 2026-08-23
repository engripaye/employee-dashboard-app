cfg_if::cfg_if! {

    if #[cfg(feature - "ssr")]{
        use create::app::models::Person;
        use surrealdb::engine::remote::ws::{Client, Ws};
        use surrealdb::opt::auth::Root;
        use surrealdb::{Error, Surreal};
        use once_cell::sync::Lazy;

        static Db: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

        pub async fn open_db_connection() {

            DB.connect::<Ws>("127.0.0.1:8000").await;
            DB.signin(Root {
                username: "Root",
                password: "Root",
            })
            .await;
            DB.use_ns("surreal").use_db("person").await;
        }

        pub async fn get_all_persons() -> Option<Vec<Person>> {

            open_db_connection().await;
            let get_all_persons = DB.query("SELECT * FROM person ORDER BY joined_date DESC;").await;
            DB.invalidate().await;

            match get_all_persons{

                OK(mut res) => {
                    let found = res.take(0);
                    match found {
                        OK(found_persons) => Some(found_persons),
                        Err(_) => None,
                        }
                    },
                Err(_) => None,
                }
            }

        pub async fn add_person(new_person: Person) -> Option<Person> {
            open_db_connection().await;
            let results = DB.create(("person", new_person.uuid.clone()))
            .content(new_person)
            .await;
           DB.invalidate().await;

           match results {
               OK(created_person) => created_person,
               Err(_) => None

               }
            }
    }

}