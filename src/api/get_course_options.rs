//use crate::UserData;
use crate::db::get_pool;
use crate::model::{ClassStatus, EnrollmentStatus, SchedulePrint, WeekdayScheduled};
use actix_web::{post, web, HttpResponse, Responder, ResponseError};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::FromRow;
use std::collections::HashMap;
use std::fmt;

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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CourseOption {
    course_id: String,
    course_num: String,
    course_parent_num: String,
    credits: String,
    #[serde(rename = "curenroll", serialize_with = "serialize_int_as_string")]
    enrolled_students: i32,
    id: String,
    instructor: String,
    #[serde(rename = "maxenroll", serialize_with = "serialize_int_as_string")]
    maximum_enrolled_students: i32,
    online: bool,
    times: Vec<Time>,
    title: String,
    raw: DatabaseCourseOption,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Time {
    #[serde(rename = "bldg")]
    building: Building,
    day: WeekDay,
    #[serde(serialize_with = "serialize_int_as_string")]
    end: u32,
    off_campus: bool,
    room: String,
    #[serde(serialize_with = "serialize_int_as_string")]
    start: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum WeekDay {
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

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Building {
    code: String,
    number: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    course: String,
    term: i32,
    ignore_full: bool,
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

#[derive(Serialize, FromRow, Debug, Clone)]
struct DatabaseCourseOption {
    course_id: i32,
    course_offer_number: i32,
    academic_term: i32,
    session_code: String,
    class_section: String,
    subject: String,
    catalog_number: String,
    description: String,
    topic: String,
    class_number: i32,
    ssr_component: String,
    units: String,
    enrollment_status: EnrollmentStatus,
    class_status: ClassStatus,
    class_type: String,
    schedule_print: SchedulePrint,
    enrollment_capacity: i32,
    enrollment_total: i32,
    institution: String,
    academic_org: String,
    academic_group: String,
    academic_career: String,
    instruction_mode: String,
    course_description_long: String,
    class_meeting_number: i32,
    start_date: String,
    end_date: String,
    building: String,
    room_number: String,
    meeting_time_start: String,
    meeting_time_end: String,
    monday: WeekdayScheduled,
    tuesday: WeekdayScheduled,
    wednesday: WeekdayScheduled,
    thursday: WeekdayScheduled,
    friday: WeekdayScheduled,
    saturday: WeekdayScheduled,
    sunday: WeekdayScheduled,
    last_name: String,
    first_name: String,
}

#[post("/generate/getCourseOpts")]
pub async fn get_course_options(options: web::Form<Search>) -> impl Responder {
    let rit_term = ((options.term / 10000) * 1000) + (options.term % 1000);
    println!("Rit_term: {rit_term}");
    //  WHERE (subject || '-' || catalog_number || '-' || class_section) = ?
    let candidate_classes = sqlx::query_as!(
        DatabaseCourseOption,
        "SELECT classes.course_id,
            classes.course_offer_number,
            classes.academic_term,
            classes.session_code,
            classes.class_section,
            classes.subject,
            classes.catalog_number,
            classes.description,
            classes.topic,
            classes.class_number,
            classes.ssr_component,
            classes.units,
            classes.enrollment_status as \"enrollment_status: EnrollmentStatus\",
            classes.class_status as \"class_status: ClassStatus\",
            classes.class_type,
            classes.schedule_print as \"schedule_print: SchedulePrint\",
            classes.enrollment_capacity,
            classes.enrollment_total,
            classes.institution,
            classes.academic_org,
            classes.academic_group,
            classes.academic_career,
            classes.instruction_mode,
            classes.course_description_long,
            meetings.class_meeting_number,
            meetings.start_date,
            meetings.end_date,
            meetings.building,
            meetings.room_number,
            meetings.meeting_time_start,
            meetings.meeting_time_end,
            meetings.monday as \"monday: WeekdayScheduled\",
            meetings.tuesday as \"tuesday: WeekdayScheduled\",
            meetings.wednesday as \"wednesday: WeekdayScheduled\",
            meetings.thursday as \"thursday: WeekdayScheduled\",
            meetings.friday as \"friday: WeekdayScheduled\",
            meetings.saturday as \"saturday: WeekdayScheduled\",
            meetings.sunday as \"sunday: WeekdayScheduled\",
            instructors.last_name,
            instructors.first_name
        FROM classes
        LEFT JOIN meetings ON
            classes.course_id = meetings.course_id AND
            classes.course_offer_number = meetings.course_offer_number AND
            classes.academic_term = meetings.academic_term AND
            classes.session_code = meetings.session_code AND
            classes.class_section = meetings.class_section
        LEFT JOIN instructors ON
            classes.course_id = instructors.course_id AND
            classes.course_offer_number = instructors.course_offer_number AND
            classes.academic_term = instructors.academic_term AND
            classes.session_code = instructors.session_code AND
            classes.class_section = instructors.class_section AND
            meetings.class_meeting_number = instructors.class_meeting_number
        WHERE
            (classes.subject || '-' || classes.catalog_number || '-' || classes.class_section) LIKE $1 AND
            classes.academic_term = $2",
        format!("{}%", options.course),
        rit_term
    )
    .fetch_all(get_pool().await?)
    .await?;
    let course_options = candidate_classes
        .into_iter()
        .map(|course| {
            let course_clone = course.clone();
            let DatabaseCourseOption {
                subject,
                catalog_number,
                class_section,
                first_name,
                last_name,
                ..
            } = course;
            let course_parent_num = format!("{subject}-{catalog_number}");
            let course_num = format!("{course_parent_num}-{class_section}");
            let mut times: Vec<Time> = Vec::with_capacity(7);
            let building = Building {
                code: BUILDING_CODES
                    .get(&course.building)
                    .cloned()
                    .map(|building| building.to_string())
                    .unwrap_or_else(|| course.building.clone()),
                number: course.building,
            };
            macro_rules! add_day {
                ($day:ident, $Day:ident) => {{
                    if let WeekdayScheduled::Scheduled = course.$day {
                        times.push(Time {
                            building: building.clone(),
                            day: WeekDay::$Day,
                            end: time_str_to_minutes(&course.meeting_time_end),
                            start: time_str_to_minutes(&course.meeting_time_start),
                            // TODO
                            off_campus: false,
                            room: course.room_number.clone(),
                        });
                    }
                }};
            }
            add_day!(monday, Monday);
            add_day!(tuesday, Tuesday);
            add_day!(wednesday, Wednesday);
            add_day!(thursday, Thursday);
            add_day!(friday, Friday);
            add_day!(saturday, Saturday);
            add_day!(sunday, Sunday);

            CourseOption {
                id: course_num.clone(),
                course_id: course.course_id.to_string(),
                course_num,
                course_parent_num,
                credits: course.units,
                enrolled_students: course.enrollment_total,
                maximum_enrolled_students: course.enrollment_capacity,
                instructor: format!("{first_name} {last_name}"),
                online: false,
                times,
                title: course.description,
                raw: course_clone,
            }
        })
        .collect::<Vec<_>>();
    Ok::<_, SqlxError>(HttpResponse::Ok().json(course_options))
}

fn time_str_to_minutes(timestamp: &str) -> u32 {
    let mut minutes = (timestamp[0..2].parse::<u32>().expect("Bad Hours") * 60)
        + (timestamp[3..5].parse::<u32>().expect("Bad minutes"));
    if &timestamp[6..8] == "PM" {
        minutes += 12 * 60;
    }
    minutes
}
