use mongodb::{Client, options::ClientOptions};
use bson::{doc, bson};




use connect4_lib::{
    game, game::Board, game::BoardState, game::ChipDescrip, game::Game, games, io, GameIO,
};

static databaseName: &str = "Connect4DB";

pub fn lab4_example_code() -> Result<(), mongodb::error::Error> {
	let mut client_options =
		 ClientOptions::parse("mongodb://localhost:27017")?;
	//client_options.app_name = Some("My App".to_string());
	let client = Client::with_options(client_options)?;

//	for db_name in client.list_database_names(None)? {
//    	println!("{}", db_name);
//	}


    let serializedGame = doc!

	// Get a handle to a database.
	let db = client.database(databaseName);
	// List the names of the collections in that database.
	for collection_name in db.list_collection_names(None)? {
	    println!("{}", collection_name);
	}

	// Get a handle to a collection in the database.
	let collection = db.collection("games");

	let docs = vec![
	doc! { "title": "1984", "author": "George Orwell" },
	doc! { "title": "Animal Farm", "author": "George Orwell" },
	doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
	];
	// Insert some documents into the "mydb.books" collection.
	//collection.insert_many(docs, None)?;
	Ok(())
}
