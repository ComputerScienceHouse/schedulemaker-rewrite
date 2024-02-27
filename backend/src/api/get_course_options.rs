//use crate::UserData;
use crate::db::get_pool;
use crate::model::{ClassStatus, EnrollmentStatus, SchedulePrint, WeekdayScheduled};
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::{FromRow, QueryBuilder, Postgres, Row};
use std::collections::HashMap;
use std::fmt;
use utoipa::ToSchema;

fn serialize_int_as_string<S, T: ToString>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

lazy_static! {
    static ref BUILDING_CODES: HashMap<String, &'static str> =
        HashMap::from([("070".to_owned(), "GOL")]);
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CourseOption {
    course_num: String,
    title: String,
    description: String,
    credits: i32,
    enrolled_students: i32,
    enrollment_capacity: i32,
    instructor_name: String,
    online: bool,
    times: Vec<Time>,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    building: Building,
    day: WeekDay,
    start: u32,
    end: u32,
    room: String,
    off_campus: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
pub enum WeekDay {
    #[serde(rename = "0")]
    Sunday,
    #[serde(rename = "1")]
    Monday,
    #[serde(rename = "2")]
    Tuesday,
    #[serde(rename = "3")]
    Wednesday,
    #[serde(rename = "4")]
    Thursday,
    #[serde(rename = "5")]
    Friday,
    #[serde(rename = "6")]
    Saturday,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Building {
    code: String,
    number: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    course: String,
    term: i32,
    ignore_full: bool,
    credit_hours: Option<i32>,
    title: Option<String>,
    professor_name: Option<String>,
    keywords: Option<Vec<String>>,
    days: Option<Vec<bool>>,
    online: Option<bool>,
    honors: Option<bool>,
    off_campus: Option<bool>,
}

#[derive(Debug)]
pub struct SqlxError(sqlx::Error);
impl fmt::Display for SqlxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl ResponseError for SqlxError {}
impl From<sqlx::Error> for SqlxError {
    fn from(error: sqlx::Error) -> Self {
        Self(error)
    }
}

#[utoipa::path(
    context_path = "/api",
    request_body(content=Search, content_type="application/json", description="Course Preferences"),
    responses(
        (status = 200, description = "List all course sections matching criteria", body = [CourseOption]),
        (status = 500, description = "Error Created by Query"),
    )
)]
#[post("/generate/getCourseOpts")]
pub async fn get_course_options(options: web::Json<Search>) -> impl Responder {
    let course_number = format!("{}%", options.course).to_uppercase();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("
            SELECT c.id AS cid,
                c.course,
                c.title,
                c.description,
                c.credits,
                s.id AS sid,
                s.curr_enroll,
                s.max_enroll,
                s.instructor,
                d.code
            FROM courses c, departments d, sections s
            WHERE
                c.id = s.course AND
                c.department = d.id AND
                s.status != 'X' AND
                c.term = $1 AND
                (d.code || '-' || c.course || '-' || s.section) LIKE '$2'");

    query_builder.push_bind(options.term);
    query_builder.push_bind(course_number);

    let candidates = query_builder.build().fetch_all(get_pool().await?).await?;

    let stem = "
        SELECT t.day,
            t.start_time,
            t.end_time,
            t.room,
            b.code,
            b.number,
            b.off_campus
        FROM courses c, sections s, buildings b, times t
        WHERE
            c.id = s.course AND
            t.section = s.id AND
            t.building = b.id
    ";

    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new(stem);

    if !options.ignore_full {
        qb.push(" AND s.max_enroll != s.curr_enroll");
    }

    if let Some(credit_hours) = options.credit_hours {
        if credit_hours >= 0 && credit_hours <= 12 {
            qb.push(" AND c.credits = $1");
            qb.push_bind(credit_hours);
        }
    }

    if let Some(title) = &options.title {
        qb.push(" AND UPPER(c.title) LIKE '$1'");
        qb.push_bind(format!("%{}%", title.to_uppercase()));
    }

    if let Some(professor_name) = &options.professor_name {
        qb.push(" AND UPPER(c.instructor) LIKE '$1'");
        qb.push_bind(format!("%{}%", professor_name.to_uppercase()));
    }

    if let Some(keywords) = &options.keywords {
        for keyword in keywords {
            qb.push(" AND UPPER(c.description) LIKE '$1'");
            qb.push_bind(keyword);
        }
    }

    if let Some(false) = options.honors {
        qb.push(" AND NOT c.description LIKE 'honors'");
    }

    qb.push(" AND c.id = $1 AND s.id = $2");

    let courses = candidates
        .into_iter()
        .map(|row| {
            qb.push_bind(row.get::<i32, &str>("cid"));
            qb.push_bind(row.get::<i32, &str>("sid"));

            if let Some(days) = &options.days {
                for idx in 0..7 {
                    if days[idx] {
                        qb.push(" AND t.day = $1");
                        qb.push_bind(idx as i32);
                    }
                }
            }

            if let Some(false) = options.online {
                qb.push(" AND NOT b.code = 'ON'");
            }

            if let Some(false) = options.off_campus {
                qb.push(" AND NOT b.code = 'OFFC'");
            }

            let times = qb.build().fetch_all(get_pool())
                .into_iter()
                .map(|r| {
                    let building = Building {
                        code: r.get::<String, &str>("code"),
                        number: r.get::<String, &str>("number"),
                    };

                    let weekday: WeekDay = match r.get::<i32, &str>("day") {
                        0 => WeekDay::Sunday,
                        1 => WeekDay::Monday,
                        2 => WeekDay::Tuesday,
                        3 => WeekDay::Wednesday,
                        4 => WeekDay::Thursday,
                        5 => WeekDay::Friday,
                        6 => WeekDay::Saturday,
                        _ => panic!("invalid value received from times.day"),
                    };

                    Time {
                        building: building,
                        day: weekday,
                        start: r.get::<i32, &str>("start_time") as u32,
                        end: r.get::<i32, &str>("end_time") as u32,
                        room: r.get::<String, &str>("room"),
                        off_campus: r.get::<bool, &str>("off_campus"),
                    }
                })
                .collect::<Vec<Time>>();

            qb.reset();

            CourseOption {
                course_num: format!("{}-{}", row.get::<String, &str>("course"), row.get::<String, _>("code")),
                title: row.get::<String, &str>("title"),
                description: row.get::<String, &str>("description"),
                credits: row.get::<i32, &str>("credits"),
                enrolled_students: row.get::<i32, &str>("enrolled_students"),
                enrollment_capacity: row.get::<i32, &str>("enrollment_capacity"),
                instructor_name: row.get::<String, &str>("instructor"),
                online: times[0].building.code == "ON",
                times: times,
            }
        })
        .collect::<Vec<CourseOption>>();

    Ok::<_, SqlxError>(HttpResponse::Ok().json(courses))
}

