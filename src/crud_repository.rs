use futures::stream::StreamExt;
use log::trace;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson::Document as BsonDocument, Document};
use mongodb::error::Error;
use mongodb::options::FindOptions;
use mongodb::Database;
use mongodb::{bson, options::AggregateOptions};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use mongodb::{
    options::{
        DeleteOptions, FindOneAndReplaceOptions, FindOneAndUpdateOptions, ReplaceOptions,
        UpdateModifications, UpdateOptions,
    },
    results::{DeleteResult, InsertOneResult, UpdateResult},
};
use std::fmt::Debug;

pub async fn find_one<T>(
    filter_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_one");
    let coll = db.collection(collection_name);
    coll.find_one(filter_document, None).await
}

pub async fn find_by_id<T>(
    id: &ObjectId,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_by_id");
    let filter_document = doc! {"_id":  id};
    find_one(filter_document, &collection_name, &db).await
}

pub async fn find_by_string_id<T>(
    id: &str,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_by_string_id");
    let filter_document = doc! {"_id": id};
    find_one(filter_document, &collection_name, &db).await
}

pub async fn find_one_by_string_field<T>(
    name: &str,
    value: &str,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_by_string_id");
    let filter_document = doc! {name: value};
    find_one(filter_document, &collection_name, &db).await
}

pub async fn find_by_string_field<T>(
    name: &str,
    value: &str,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_by_string_field");
    let filter_document = doc! {name: value};
    find_simple(filter_document, &collection_name, &db).await
}

pub async fn find_all<T>(collection_name: &str, db: &Database) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_all");
    find(None, None, collection_name, db).await
}

pub async fn find_simple<T>(
    filter_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find");
    find(filter_document, None, collection_name, db).await
}

pub async fn find_with_sort<T>(
    filter_document: Document,
    sort_document: Document,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find_with_sort");
    let sort_option = get_sort_find_option(Some(sort_document));
    find(filter_document, sort_option, collection_name, db).await
}

pub async fn find<T>(
    filter_document: impl Into<Option<Document>>,
    options: impl Into<Option<FindOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("find");
    let coll = db.collection(collection_name);

    let mut cursor = coll.find(filter_document, options).await?;
    let mut items = Vec::<T>::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                items.push(document);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(items)
}

pub async fn count_documents(
    filter: impl Into<Option<Document>>,
    collection_name: &str,
    db: &Database,
) -> Result<u64, Error> {
    trace!("count_documents");
    let coll = db.collection::<Document>(collection_name);
    coll.count_documents(filter, None).await
}

pub async fn aggregate<T>(
    pipeline: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<Vec<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    trace!("aggregate");
    let coll = db.collection::<Document>(collection_name);
    let mut cursor = coll.aggregate(pipeline, options).await?;
    let mut items = Vec::<T>::new();

    while let Some(result) = cursor.next().await {
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

pub async fn _find_one_by_field<T>(
    field_name: String,
    value: String,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    self::find_one(doc! {field_name: value}, collection_name, db).await
}

pub async fn add<T>(item: T, collection_name: &str, db: &Database) -> Result<InsertOneResult, Error>
where
    for<'de> T: Debug + Serialize + Deserialize<'de> + DeserializeOwned + Unpin + Debug,
{
    let coll = db.collection(collection_name);
    coll.insert_one(item, None).await
}

pub async fn update_one(
    query: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<UpdateOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<UpdateResult, Error> {
    let coll = db.collection::<Document>(collection_name);
    coll.update_one(query, update, options).await
}

pub async fn find_one_and_update<T>(
    filter: Document,
    update: impl Into<UpdateModifications>,
    options: impl Into<Option<FindOneAndUpdateOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let coll = db.collection(collection_name);
    coll.find_one_and_update(filter, update, options).await
}

pub async fn find_one_and_replace<T>(
    filter: Document,
    replacement: T,
    options: impl Into<Option<FindOneAndReplaceOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<Option<T>, Error>
where
    for<'de> T: Serialize + DeserializeOwned + Deserialize<'de> + Unpin + Debug,
{
    let coll = db.collection(collection_name);
    coll.find_one_and_replace(filter, replacement, options)
        .await
}

pub async fn replace_one<T>(
    query: Document,
    replacement: T,
    options: impl Into<Option<ReplaceOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<UpdateResult, Error>
where
    for<'a> T: Serialize + DeserializeOwned + Unpin + Debug,
{
    let coll = db.collection(collection_name);
    coll.replace_one(query, replacement, options).await
}

pub async fn delete_one(
    query: Document,
    options: impl Into<Option<DeleteOptions>>,
    collection_name: &str,
    db: &Database,
) -> Result<DeleteResult, Error> {
    let coll = db.collection::<Document>(collection_name);
    coll.delete_one(query, options).await
}
