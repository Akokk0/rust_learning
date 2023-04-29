use crate::models::course::Course;
use crate::errors::MyError;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_courses_for_teacher_db(
    pool: &PgPool, teacher_id: i32
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1"#,
        teacher_id
    )
        .fetch_all(pool)
        .await?;

    Ok(rows)

    /*let courses: Vec<Course> = rows
        .into_iter()
        .map(|r| Course {
        id: Some(r.id),
        teacher_id: r.teacher_id,
        name: r.name,
        time: r.time
    }).collect();

    match courses.len() {
        0 => Err(MyError::NotFound("Courses not found for teacher".into())),
        _ => Ok(courses)
    }*/
}

pub async fn get_course_details_db(
    pool: &PgPool, teacher_id: i32, course_id: i32
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"SELECT * FROM course WHERE teacher_id = $1 and id = $2"#,
        teacher_id,
        course_id
    )
        .fetch_optional(pool)
        .await?;

    if let Some(course) = row {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course Id not found".into()))
    }

    /*Ok(Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name,
        time: row.time
    })*/
}

pub async fn post_new_course_db(
    pool: &PgPool, new_course: Course
) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (teacher_id, name, description)
        VALUES ($1, $2)
        RETURNING id, teacher_id, name, time"#,
        new_course.teacher_id,
        new_course.name
    )
        .fetch_one(pool)
        .await?;

    /*Ok(Course {
        id: Some(row.id),
        teacher_id: row.teacher_id,
        name: row.name,
        time: row.time
    })*/
}