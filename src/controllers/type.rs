use sqlx::SqlitePool;

use crate::models;
use crate::errors::AppError;

async fn raw_get_all(pool: &SqlitePool) -> Result<Vec<models::Type>, sqlx::Error> {

    let sql = String::from("SELECT * FROM types");

    sqlx::query_as::<_,models::Type>(&sql)
        .fetch_all(pool)
        .await

}

async fn raw_get_all_for_doc(pool: &SqlitePool, id: i32) -> Result<Vec<models::Type>, sqlx::Error> {

    let sql = String::from("SELECT * FROM types WHERE id IN (SELECT idtype FROM doctypes WHERE iddoc = ?)");

    sqlx::query_as::<_,models::Type>(&sql)
        .bind(id)
        .fetch_all(pool)
        .await

}

async fn raw_set_for_docid(pool: &SqlitePool, docid: i32, types: &Vec<i32>) -> Result<(), sqlx::Error> {

    let sql1 = String::from("DELETE FROM doctypes WHERE iddoc = ?");
    let sql2 = String::from("INSERT INTO doctypes (iddoc, idtype) VALUES (?,?)");

    let mut trans = pool.begin()
        .await?;

    sqlx::query(&sql1)
        .bind(docid)
        .execute(&mut *trans)
        .await?;

    for typeid in types {
        sqlx::query(&sql2)
            .bind(docid)
            .bind(typeid)
            .execute(&mut *trans)
            .await?;
    }

    trans.commit().await?;

    Ok(())
}

pub async fn get_for_docid(pool: &SqlitePool, id: i32)
    -> Result<Vec<models::Type>, AppError> {

    let types = raw_get_all_for_doc(pool, id)
        .await?;

    Ok(types)
}

pub async fn set_for_docid(pool: &SqlitePool, id: i32, types: &Vec<i32>)
    -> Result<(), AppError> {

    raw_set_for_docid(pool, id, types)
        .await?;

    Ok(())
}

pub async fn get_all_types(pool: &SqlitePool)
    -> Result<Vec<models::Type>, AppError> {

    let types = raw_get_all(&pool)
        .await?;

    Ok(types)
}