use sqlx::SqlitePool;

use crate::controllers;
use crate::{models, timeutil};
use crate::errors::AppError;

async fn raw_get_all_docs(pool: &sqlx::SqlitePool) -> Result<Vec<models::DocDB>, sqlx::Error> {

    let sql = String::from("SELECT * FROM docs");

    sqlx::query_as::<_,models::DocDB>(&sql)
        .fetch_all(pool)
        .await

}

async fn raw_get_doc(pool: &sqlx::SqlitePool, id: i32) -> Result<models::DocDB, sqlx::Error> {

    let sql = String::from("SELECT * FROM docs WHERE id = ?");

    sqlx::query_as::<_,models::DocDB>(&sql)
        .bind(id)
        .fetch_one(pool)
        .await

}

async fn raw_insert_doc(pool: &sqlx::SqlitePool, doc: &models::DocAPI)
    -> Result<models::DocDB, sqlx::Error> {

    let sql = String::from("INSERT INTO docs (\"name\", \"storagename\", \"date\", \"lastupdate\") VALUES (?,'',?,?)");
    let res = sqlx::query(&sql)
        .bind(doc.name.to_owned())
        .bind(doc.date.to_owned())
        .bind(timeutil::now())
        .execute(pool)
        .await?;

    let newid = res.last_insert_rowid() as i32;

    Ok(raw_get_doc(pool, newid).await?)

}

async fn raw_update_doc(pool: &sqlx::SqlitePool, doc: &models::DocAPI)
    -> Result<models::DocDB, sqlx::Error> {

    let docid = doc.id.unwrap();

    let sql = String::from("UPDATE docs SET \"name\" = ?, \"date\" = ?, \"lastupdate\" = ? WHERE id = ?");
    sqlx::query(&sql)
        .bind(doc.name.to_owned())
        .bind(doc.date.to_owned())
        .bind(timeutil::now())
        .bind(docid)
        .execute(pool)
        .await?;

    Ok(raw_get_doc(pool, docid).await?)

}

async fn fill_doc(pool: &SqlitePool, doc: models::DocDB) -> Result<models::Doc, AppError> {

    let doc_tags = controllers::tag::get_for_docid(pool, doc.id);
    let doc_types = controllers::r#type::get_for_docid(pool, doc.id);
    let doc_people = controllers::person::get_for_docid(pool, doc.id);

    let doc_tags = doc_tags.await?;
    let doc_types = doc_types.await?;
    let doc_people = doc_people.await?;

    Ok(models::Doc {
        id: doc.id,
        name: doc.name,
        storagename: doc.storagename,
        date: doc.date,
        lastupdate: doc.lastupdate,

        tags: doc_tags,
        types: doc_types,
        people: doc_people,
    })

}

pub async fn get_all_docs(pool: &SqlitePool)
    -> Result<Vec<models::Doc>, AppError> {

    let dbdocs = raw_get_all_docs(&pool)
        .await
        .map_err(|err| {
            AppError::InternalServerError(err.to_string())
        })?;

    let mut docs = Vec::<models::Doc>::new();

    for doc in dbdocs {
        let doc = fill_doc(pool, doc).await?;
        docs.push(doc);
    }

    Ok(docs)
}

pub async fn get_doc(pool: &SqlitePool, id: i32)
    -> Result<models::Doc, AppError> {

    let doc = raw_get_doc(pool, id)
        .await
        .map_err(|err| {
            match err {
                sqlx::Error::RowNotFound => AppError::NotFound(format!("ID ({}) not found", id)),
                _ => AppError::InternalServerError(err.to_string()),
            }
        })?;

    let doc = fill_doc(pool, doc)
        .await?;

    Ok(doc)
}

pub async fn post_doc(pool: &SqlitePool, doc: models::DocAPI)
    -> Result<models::Doc, AppError> {

    // TODO: THIS NEEDS TO USE A TRANSACTION THE WHOLE WAY THROUGH

    let docdb = raw_insert_doc(pool, &doc)
        .await
        .map_err(|err| {
            AppError::InternalServerError(err.to_string())
        })?;

    let tag_update = controllers::tag::set_for_docid(pool, docdb.id, &doc.tags);
    let type_update = controllers::r#type::set_for_docid(pool, docdb.id, &doc.types);
    let person_update = controllers::person::set_for_docid(pool, docdb.id, &doc.people);
    tag_update.await?;
    type_update.await?;
    person_update.await?;

    let doc = fill_doc(pool, docdb)
        .await?;

    Ok(doc)

}

pub async fn put_doc(pool: &SqlitePool, doc: models::DocAPI)
    -> Result<models::Doc, AppError> {

    // TODO: THIS NEEDS TO USE A TRANSACTION THE WHOLE WAY THROUGH

    let docdb = raw_update_doc(&pool, &doc)
        .await
        .map_err(|err| {
            AppError::InternalServerError(err.to_string())
        })?;

    let tag_update = controllers::tag::set_for_docid(pool, docdb.id, &doc.tags);
    let type_update = controllers::r#type::set_for_docid(pool, docdb.id, &doc.types);
    let person_update = controllers::person::set_for_docid(pool, docdb.id, &doc.people);
    tag_update.await?;
    type_update.await?;
    person_update.await?;

    let doc = fill_doc(pool, docdb)
        .await?;

    Ok(doc)

}