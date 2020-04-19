use bson::{doc, Bson::Document as BsonDocument, Document};
use log::trace;
use mongodb::error::Error;
use mongodb::options::FindOptions;
use mongodb::Database;
use serde::{Deserialize, Serialize};

#[cfg(feature = "write")]
use mongodb::{
    options::{DeleteOptions, UpdateModifications, UpdateOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
#[cfg(feature = "write")]
use std::fmt::Debug;

#[cfg(feature = "read")]
pub fn find_one<T>(
    filter_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    trace!("find_one");
    let coll = db.collection(collection_name);
    let result = coll.find_one(filter_document, None)?;
    if let Some(document) = result {
        let t = bson::from_bson::<T>(BsonDocument(document))?;
        return Ok(Some(t));
    } else {
        return Ok(None);
    }
}

#[cfg(feature = "read")]
pub fn find<T>(
    filter_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    trace!("find");
    find_generic(filter_document, None, collection_name, db)
}

#[cfg(feature = "read")]
pub fn find_with_sort<T>(
    filter_document: Document,
    sort_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    trace!("find_with_sort");
    find_generic(filter_document, Some(sort_document), collection_name, db)
}

fn find_generic<T>(
    filter_document: Document,
    sort_document_option: Option<Document>,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    trace!("find_generic");
    let coll = db.collection(collection_name);
    let find_options = get_sort_find_option(sort_document_option);
    let cursor = coll.find(filter_document, find_options)?;
    let mut items = Vec::<T>::new();
    for result in cursor {
        match result {
            Ok(document) => {
                let item = bson::from_bson::<T>(BsonDocument(document))?;
                items.push(item);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(items)
}

fn get_sort_find_option(sort_document_option: Option<Document>) -> Option<FindOptions> {
    if let Some(sort_document) = sort_document_option {
        Some(FindOptions::builder().sort(sort_document).build())
    } else {
        None
    }
}

#[cfg(feature = "read")]
pub fn _find_one_by_field<T>(
    field_name: String,
    value: String,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    self::find_one(doc! {field_name: value}, collection_name, db)
}

#[cfg(feature = "write")]
pub fn add<T>(t: &T, collection_name: &str, db: &Database) -> Result<InsertOneResult, Error>
where
    for<'a> T: Debug + Serialize + Deserialize<'a>,
{
    let serialized_item = bson::to_bson(&t)?;

    if let BsonDocument(document) = serialized_item {
        let coll = db.collection(collection_name);
        coll.insert_one(document, None)
    } else {
        panic!("Error converting the BSON object into a MongoDB document");
    }
}

#[cfg(feature = "write")]
pub fn update_one(
    query: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<UpdateResult, Error> {
    let coll = db.collection(collection_name);
    coll.update_one(query, update, options)
}

#[cfg(feature = "write")]
pub fn delete_one(
    query: Document,
    options: impl Into<Option<DeleteOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<DeleteResult, Error> {
    let coll = db.collection(collection_name);
    coll.delete_one(query, options)
}
