//use crate::UserData;
use crate::db::get_pool;
use crate::model::{ClassStatus, EnrollmentStatus, SchedulePrint, WeekdayScheduled};
use actix_web::{post, web::Json, HttpResponse, Responder, ResponseError};
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
pub struct SectionInfo {
    course_id: i32,
    title: String,
    description: String,
    credits: i32,
    section_id: i32,
    curr_enroll: i32,
    instructor: String,
    dept_code: i32,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SectionTimeInfo {
    day: i32,
    start_time: i32,
    end_time: i32,
    room: String,
    code: String,
    number: String,
    off_campus: bool,
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
pub async fn get_course_options(
    state: Data<AppState>,
    options: Json<Search>,
) -> impl Responder {
    log!(Level::Info, "GET /generate/getCourseOpts");
    let course_number = format!("{}%", options.course).to_uppercase();

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new("
            SELECT c.id AS course_id,
                c.course,
                c.title,
                c.description,
                c.credits,
                s.id AS section_id,
                s.curr_enroll,
                s.max_enroll,
                s.instructor,
                d.code AS dept_code
            FROM courses c, departments d, sections s
            WHERE
                c.id = s.course AND
                c.department = d.id AND
                s.status != 'X' AND
                c.term = $1 AND
                (d.code || '-' || c.course || '-' || s.section) LIKE '$2'");

    query_builder.push_bind(options.term);
    query_builder.push_bind(course_number);

    let sections: Vec<SectionInfo>;
    match log_query_as(
        query_builder.build_query_as<SectionInfo>()
            .fetch_all(&state.db)
            .await,
        None,
    ).await {
        Ok((_, secs)) => { sections = secs; },
        Err(e) => return e,
    };

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
            t.building = b.id";

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

    let courses: Vec<CourseOption> = sections
        .into_iter()
        .map(|section| {
            qb.push_bind(section.course_id);
            qb.push_bind(section.section_id);

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

            let section_times: Vec<SectionTimeInfo>;
            match log_query_as(
                qb.build_query_as<SectionTimeInfo>()
                    .fetch_all(&state.db)
                    .await,
                None,
            ).await {
                Ok((_, t)) => { section_times = t; },
                Err(e) => return e,
            };

            let times: Vec<Time> = section_times
                .into_iter()
                .map(|sec_time| {
                    let building: Building = Building {
                        code: sec_time.code,
                        number: sec_time.number,
                    };

                    let weekday: WeekDay = match sec_time.day {
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
                        start: sec_time.start_time,
                        end: sec_time.end_time,
                        room: sec_time.room,
                        ofF_campus: sec_time.off_campus,
                    }
                })
                .collect::<Vec<Time>>();

            qb.reset();

            CourseOption {
                course_num: format!("{}-{}", section.course, section.code),
                title: section.title,
                description: section.description,
                credits: section.credits,
                enrolled_students: section.enrolled_students,
                enrollment_capacity: section.enrollment_capacity,
                instructor_name: section.instructor,
                online: times[0].building.code == "ON",
                times: times,
            }
        })
        .collect::<Vec<CourseOption>>();

    Ok::<_, SqlxError>(HttpResponse::Ok().json(courses))
}

