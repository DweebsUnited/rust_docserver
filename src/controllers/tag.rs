use sqlx::SqlitePool;

use crate::models;
use crate::errors::AppError;

async fn raw_get_all(pool: &SqlitePool) -> Result<Vec<models::Tag>, sqlx::Error> {

    let sql = String::from("SELECT * FROM tags");

    sqlx::query_as::<_,models::Tag>(&sql)
        .fetch_all(pool)
        .await

}

async fn raw_get_all_for_doc(pool: &SqlitePool, id: i32) -> Result<Vec<models::Tag>, sqlx::Error> {

    let sql = String::from("SELECT * FROM tags WHERE id IN (SELECT idtag FROM doctags WHERE iddoc = ?)");

    sqlx::query_as::<_,models::Tag>(&sql)
        .bind(id)
        .fetch_all(pool)
        .await

}

async fn raw_set_for_docid(pool: &SqlitePool, docid: i32, tags: &Vec<i32>) -> Result<(), sqlx::Error> {

    let sql1 = String::from("DELETE FROM doctags WHERE iddoc = ?");
    let sql2 = String::from("INSERT INTO doctags (iddoc, idtag) VALUES (?,?)");

    let mut trans = pool.begin()
        .await?;

    sqlx::query(&sql1)
        .bind(docid)
        .execute(&mut *trans)
        .await?;

    for tag in tags {
        sqlx::query(&sql2)
            .bind(docid)
            .bind(tag)
            .execute(&mut *trans)
            .await?;
    }

    trans.commit().await?;

    Ok(())
}

pub async fn get_for_docid(pool: &SqlitePool, id: i32)
    -> Result<Vec<models::Tag>, AppError> {

    let tags = raw_get_all_for_doc(pool, id)
        .await?;

    Ok(tags)
}

pub async fn set_for_docid(pool: &SqlitePool, id: i32, tags: &Vec<i32>)
    -> Result<(), AppError> {

    raw_set_for_docid(pool, id, tags)
        .await?;

    Ok(())
}

pub async fn get_all_tags(pool: &SqlitePool)
    -> Result<Vec<models::Tag>, AppError> {

    let tags = raw_get_all(&pool)
        .await?;

    Ok(tags)
}