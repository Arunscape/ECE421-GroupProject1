


use mongodb::{options::ClientOptions, Client};
//use bson::{doc, bson, to_bson};
use bson::ordered::OrderedDocument;
use bson::*;
use serde::{Deserialize, Serialize};

static DATABASE_NAME: &str = "Connect4DB";
static JWT_LIFETIME_SECONDS: u64 = 86400; // done day
static DATABASE_LOCATION: &str = "mongodb://localhost:27017";

// error handling wrapper for db connect
fn new_db(db_name: &str) -> Option<mongodb::Database> {
    match connect_to_db(DATABASE_NAME) {
        Ok(db) => Some(db),
        Err(_) => None, // TODO: error handle
    }
}

fn connect_to_db(db_name: &str) -> Result<mongodb::Database, mongodb::error::Error> {
    let mut client_options =
        ClientOptions::parse(DATABASE_LOCATION)?;
    let client = Client::with_options(client_options)?;
    let db = client.database(db_name);
    Ok(db)
}

fn bson_to_object<'a, T>(thing: bson::ordered::OrderedDocument)
     -> Option<T>
    where T: Deserialize<'a> {
    match from_bson(Bson::Document(thing)) {
        Err(_) => None, //TODO: error handle
        Ok(object) => Some(object),
    }
}


fn object_to_doc<T>(object: T) -> Option<bson::Document>
    where T: Serialize {
    //to_bson(object)?.as_document().unwrap().clone()
    match to_bson(&object) {
        Err(_) => None,
        Ok(bson) => Some(
            bson.as_document()
            .expect("bson should be a document")
            .clone()),
    }
}


fn docs_to_objects<'a, T>(docs_vector: Vec<bson::Document>)
    -> Vec<T>
    where T: Deserialize<'a> {
    docs_vector.iter()
    .map(|docs| bson_to_object::<T>(docs.clone()))
    .filter_map(|e| e) // filter out None's
    .collect()
}

fn query_collection_for_docs(db: mongodb::Database, coll_name: &str, query: bson::Document)
    -> Vec<bson::Document> {

    //let v = Vec::<bson::Document>::new();
    match db.collection(coll_name).find(query, None) {
        Err(_) => vec![],
        Ok(cursor) => {
            cursor
            .map(|res|
                match res {
                    Ok(doc) => Some(doc),
                    Err(_) => None, // erros are nothing
                })
            .filter_map(|e| e) // filter out None's
            .collect()
        },
    }
}

//// if length of doc vecror is 0
//fn exists_any_in(db, collection, query) -> bool {}
//
//fn insert_new_doc(db, collection, doc) -> Result {}
//



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn db_in_collection_test() {
    }

    #[test]
    fn db_sign_in_test() {
    }
}
