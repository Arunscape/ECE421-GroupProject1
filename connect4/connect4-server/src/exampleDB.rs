use mongodb::{Client, options::ClientOptions};
//use bson::{doc, bson, to_bson};
use bson::*;
use  bson::ordered::OrderedDocument;
use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
enum Gender {
    Male,
    Female,
}

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Person {
    gen: Gender,
    name: String,
    age: isize,
    phones: Vec<String>,
}
//
//
//// This is what #[derive(Serialize)] would generate.
//impl Serialize for Person {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer,
//    {
//        let mut s = serializer.serialize_struct("Person", 3)?;
//        s.serialize_field("name", &self.name)?;
//        s.serialize_field("age", &self.age)?;
//        s.serialize_field("phones", &self.phones)?;
//        s.end()
//    }
//}
//

static databaseName: &str = "Connect4DB";

pub fn add_chip() -> Result<(), mongodb::error::Error> {
	let mut client_options =
		 ClientOptions::parse("mongodb://localhost:27017")?;
	//client_options.app_name = Some("My App".to_string());
	let client = Client::with_options(client_options)?;

//	for db_name in client.list_database_names(None)? {
//    	println!("{}", db_name);
//	}


    let p = Person {
        gen: Gender::Male,
        name: "Alex".to_string(),
        age:22,
        phones:vec!["cell".to_string()]
    };
    //let p = game::ChipDescrip{bg_color:1, fg_color:1, graphic:'f'};

	let db = client.database(databaseName);
	let collection = db.collection("players");
    let doc = to_bson(&p)?.as_document().unwrap().clone();
    collection.insert_one(doc, None)?;

    let extractedDoc = collection
        .find_one(doc!{"name":"Alex"}, None)?.unwrap();

    let extractedBson = Bson::Document(extractedDoc);
    if let Ok(mut person) = from_bson(extractedBson) {
        if p != person {
            panic!("can't deserialize person to original ");
        }

    }


	Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn db_chip_test() {
        match add_chip() {
            Ok(()) => assert!(true),
            Err(x) => {
            println!("{:?}", x);
            assert!(false);},
        }
    }

}
