use bson::*;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};

pub static DATABASE_NAME: &str = "Connect4DB";
pub static JWT_LIFETIME_SECONDS: u64 = 86400; // done day
pub static DATABASE_LOCATION: &str = "mongodb://mongo:27017";
pub static USER_COLLECTION_NAME: &str = "users";
pub static GAME_COLLECTION_NAME: &str = "rooms";

// error handling wrapper for db connect
pub fn new_db(db_name: &str) -> Option<mongodb::Database> {
    match connect_to_db(db_name) {
        Ok(db) => Some(db),
        Err(_) => None, // TODO: error handle
    }
}

pub fn connect_to_db(db_name: &str) -> Result<mongodb::Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(DATABASE_LOCATION)?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    Ok(db)
}

pub fn bson_to_object<'a, T>(thing: bson::ordered::OrderedDocument) -> Option<T>
where
    T: Deserialize<'a>,
{
    match from_bson(Bson::Document(thing)) {
        Err(_) => None, //TODO: error handle
        Ok(object) => Some(object),
    }
}

pub fn object_to_doc<T>(object: &T) -> Option<bson::Document>
where
    T: Serialize,
{
    match to_bson(object) {
        Err(_) => None,
        Ok(bson) => match bson.as_document() {
            Some(doc) => Some(doc.clone()),
            None => None,
        },
    }
}

pub fn docs_to_objects<'a, T>(docs_vector: Vec<bson::Document>) -> Vec<T>
where
    T: Deserialize<'a>,
{
    docs_vector
        .iter()
        .map(|docs| bson_to_object::<T>(docs.clone()))
        .filter_map(|e| e) // filter out None's
        .collect()
}

pub fn query_collection_for_docs(
    db: &mongodb::Database,
    coll_name: &str,
    query: bson::Document,
) -> Vec<bson::Document> {
    //let v = Vec::<bson::Document>::new();
    match db.collection(coll_name).find(query, None) {
        Err(_) => vec![],
        Ok(cursor) => {
            cursor
                .map(|res| match res {
                    Ok(doc) => Some(doc),
                    Err(_) => None, // erros are nothing
                })
                .filter_map(|e| e) // filter out None's
                .collect()
        }
    }
}

pub fn exists_any_in(db: &mongodb::Database, coll_name: &str, q: bson::Document) -> bool {
    query_collection_for_docs(db, coll_name, q).len() != 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn db_query_and_exists_test() {
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        enum Gender {
            Male,
            Female,
        }

        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
        struct Person {
            gen: Gender,
            name: String,
            age: isize,
            phones: Vec<String>,
        }
        let p = Person {
            gen: Gender::Male,
            name: "Alex".to_string(),
            age: 22,
            phones: vec!["cell".to_string()],
        };

        let collection_name = "testymctestface";
        let db = new_db(DATABASE_NAME).expect("No mongo, is it running?");

        let doc = object_to_doc(&p).expect("Object shuould convert to doc?");

        // TODO: handle result
        if db
            .collection(collection_name)
            .insert_one(doc, None)
            .is_err()
        {
            panic!("collection insert failed!");
        }

        assert!(exists_any_in(&db, collection_name, doc! {"name":"Alex"}));

        let matches = query_collection_for_docs(&db, collection_name, doc! {"name":"Alex"});

        let extracted_p: Person = bson_to_object(matches[0].clone()).expect("Cant extract person");
        assert!(extracted_p.name == "Alex");
    }
}
