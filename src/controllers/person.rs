use sqlx::SqlitePool;

use crate::models;
use crate::errors::AppError;

async fn raw_get_all(pool: &SqlitePool) -> Result<Vec<models::Person>, sqlx::Error> {

    let sql = String::from("SELECT * FROM people");

    sqlx::query_as::<_,models::Person>(&sql)
        .fetch_all(pool)
        .await

}

async fn raw_get_all_for_doc(pool: &SqlitePool, id: i32) -> Result<Vec<models::Person>, sqlx::Error> {

    let sql = String::from("SELECT * FROM people WHERE id IN (SELECT idperson FROM docpeople WHERE iddoc = ?)");

    sqlx::query_as::<_,models::Person>(&sql)
        .bind(id)
        .fetch_all(pool)
        .await

}

async fn raw_set_for_docid(pool: &SqlitePool, docid: i32, people: &Vec<i32>) -> Result<(), sqlx::Error> {

    let sql1 = String::from("DELETE FROM docpeople WHERE iddoc = ?");
    let sql2 = String::from("INSERT INTO docpeople (iddoc, idperson) VALUES (?,?)");

    let mut trans = pool.begin()
        .await?;

    sqlx::query(&sql1)
        .bind(docid)
        .execute(&mut *trans)
        .await?;

    for person in people {
        sqlx::query(&sql2)
            .bind(docid)
            .bind(person)
            .execute(&mut *trans)
            .await?;
    }

    trans.commit().await?;

    Ok(())
}

pub async fn get_for_docid(pool: &SqlitePool, id: i32)
    -> Result<Vec<models::Person>, AppError> {

    let people = raw_get_all_for_doc(pool, id)
        .await?;

    Ok(people)
}

pub async fn set_for_docid(pool: &SqlitePool, id: i32, people: &Vec<i32>)
    -> Result<(), AppError> {

    raw_set_for_docid(pool, id, people)
        .await?;

    Ok(())
}

pub async fn get_all_people(pool: &SqlitePool)
    -> Result<Vec<models::Person>, AppError> {

    let people = raw_get_all(&pool)
        .await?;

    Ok(people)
}