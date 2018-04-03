
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
//use mongodb::coll::Collection;
use std::vec::Vec;
use serde;
use bson;
use bson::Bson;
use mongodb::db::Database;
use bson::Document;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub _id: String,
    pub title: String,
    pub body: String,
    pub category: String, 
    pub votes: i64,
    pub comments: Vec<Comment>
}

impl Article {

    // Local method used to update model
    fn to_update(&self) -> Document {
        let comments : Vec<Bson> = self.comments.iter().map(|comment| comment.to_bson()).collect();
        doc! {
            "$set" => {
                "title": &self.title,
                "body": &self.body,
                "category": &self.category,
                "votes": self.votes,
                "comments": comments
            }
        }
    }

    // Local method used to get doc 
    fn to_doc(&self) -> Document {
        let comments : Vec<Bson> = self.comments.iter().map(|comment| comment.to_bson()).collect();
        doc! {
            "title": &self.title,
            "body": &self.body,
            "category": &self.category,
            "votes": self.votes,
            "comments": comments
        }
    }

    // // Uses serde to transfer object into JSON
    // pub fn to_json(&self) -> String {
    //     serde_json::to_string(self).expect("Error parsing object...")
    // }

    // Saves an object to the database
    pub fn save(&mut self, db : Database) {
        let coll = db.collection("articles");
        let doc = self.to_doc();
        if self._id.len() == 0 {
            self._id = coll.insert_one(doc, None).expect("Error inserting!")
                .inserted_id.expect("Error finding id!")
                .as_object_id().expect("")
                .to_hex();
        } else {
            let d = doc!{"_id" =>  bson::oid::ObjectId::with_string(&self._id).unwrap()};
            coll.update_one(d, self.to_update(), None).unwrap();
        }
            
    }
}


impl From<Document> for Article {
    
    fn from(doc : Document) -> Article {

        let comments : Vec<Comment> = doc.get_array("comments").unwrap().iter()
            .map(|comment| Comment::from(comment.as_document().unwrap().clone()))
            .collect();

        Article {
            _id: doc.get_object_id("_id").unwrap().to_string(), 
            title: doc.get_str("title").unwrap().to_string(), 
            body: doc.get_str("body").unwrap().to_string(), 
            category: doc.get_str("category").unwrap().to_string(),
            votes: doc.get_i64("votes").unwrap(), 
            comments: comments
        }

    }

}

pub trait Find<C> {
    fn find(Database) -> Vec<C> ;
    fn find_one(Database, String) -> C;
}

impl Find<Article> for Article {

    fn find(db : Database) -> Vec<Article> {
        let coll = db.collection("articles");
        // Find the document and receive a cursor

        let cursor = coll.find(None, None).unwrap();

        let mut articles = vec![];

        for result in cursor {
            if let Ok(item) = result {
                articles.push(Article::from(item))
            }
        }

        articles
    }
    
    fn find_one(db: Database, _id : String) -> Article {
        let coll = db.collection("articles");

        let d = doc!{"_id" =>  bson::oid::ObjectId::with_string(&_id).unwrap()};
        let r = coll.find_one(serde::export::Some(d), None).unwrap();

        Article::from(r.unwrap())
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub title: String,
    pub body: String,
}


impl Comment {

    pub fn to_bson(&self) -> Bson {
        Bson::from(self.to_doc())
    }

    pub fn to_doc(&self) -> Document {
        doc! {
            "title": &self.title,
            "body": &self.body,
        }
    }

}

impl From<Document> for Comment {
    
    fn from(doc : Document) -> Comment {
        Comment {
            title: doc.get_str("title").unwrap().to_string(),
            body: doc.get_str("body").unwrap().to_string()
        }
    
    }

}

pub fn connect() -> Database {
    println!("Connecting to database...");
    Client::connect("127.0.0.1", 27017)
            .expect("Failed to initialize standalone client.")
            .db("news")
}







