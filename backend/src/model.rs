//! A collection of all the data entities parsed and used by ScheduleMaker.
//!
//! Some of these records are a direct mapping to the data files given by
//! SIS with some deserialization sugar to add meaning and make it is easier
//! to parse into other tables for the actual backend for ScheduleMaker.
//! All the other records represent the more permanent tables that the
//! backend queries for its information.

use actix_web::ResponseError;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query_builder::Separated, FromRow, Postgres};
use std::{
    fmt::{self, *},
    string::ToString,
};
use utoipa::ToSchema;

pub trait ToRow {
    fn keys() -> &'static [&'static str];
    fn bind<'qb, 'args: 'qb, Sep: fmt::Display>(
        &self,
        query_builder: &mut Separated<'qb, 'args, Postgres, Sep>,
    );
}

trait ToEncode<'a, T: sqlx::Type<Postgres> + sqlx::Encode<'a, Postgres>> {
    fn to_encode(&self) -> T;
}

impl<'a, T: sqlx::Type<Postgres> + sqlx::Encode<'a, Postgres> + Clone> ToEncode<'a, T> for T {
    fn to_encode(&self) -> T {
        self.clone()
    }
}

impl<'a> ToEncode<'a, String> for char {
    fn to_encode(&self) -> String {
        self.to_string()
    }
}

macro_rules! generate_to_row {
    (
        #[doc = $doc:expr]
        pub struct $name:ident {
            $(
    $(#[doc = $field_doc:expr])?
    $field_name:ident: $field_type:ty,
    )*
    }
    ) => {
    #[derive(Debug, Serialize, Deserialize, FromRow, Default)]
        #[doc = $doc]
        pub struct $name {
            $(
                $(#[doc = $field_doc])?
                pub $field_name: $field_type,
            )*
        }

        impl ToRow for $name {
            fn keys() -> &'static [&'static str] {
                &[$(stringify!($field_name)),*]
            }

            fn bind<'qb, 'args: 'qb, Sep: fmt::Display>(
                &self,
                query_builder: &mut Separated<'qb, 'args, Postgres, Sep>
            ) {
                query_builder
                $(.push_bind(self.$field_name.to_encode()))*;
            }
        }
    }
}

generate_to_row! {
    /// Represents the record of academic terms
    pub struct AcademicTermRecord {
        term: i16,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    }
}

generate_to_row! {
    /// Represents the record of buildings
    pub struct BuildingRecord {
        number: String,
        code: String,
        name: String,
        off_campus: bool,
    }
}

generate_to_row! {
    /// Represents the courses that are recorded
    pub struct CourseRecord {
        id: i32,
        term: i16,
        department: i32,
        course: String,
        credits: i8,
        title: String,
        description: String,
    }
}

generate_to_row! {
    /// Represents the academic departments a class can be offered by
    pub struct DepartmentRecord {
        id: i32,
        school: i32,
        code: String,
        title: String,
    }
}

generate_to_row! {
    /// Represents a part of a schedule that is a course
    pub struct ScheduleCourseRecord {
        id: i32,
        schedule: i32,
        section: i32,
    }
}

generate_to_row! {
    /// Represents a part of a schedule that is not a course
    pub struct ScheduleNonCourseRecord {
        id: i32,
        schedule: i32,
        title: String,
        day: i8,
        start: i16,
        end: i16,
    }
}

generate_to_row! {
    /// Represents created schedules
    pub struct ScheduleRecord {
        id: i32,
        last_accessed: DateTime<Utc>,
        start_day: i8,
        end_day: i8,
        start_time: i8,
        end_time: i8,
        term: i16,
        image: bool,
    }
}

generate_to_row! {
    /// Represents the schools that a course can be offered by
    pub struct SchoolRecord {
        id: i32,
        code: String,
        title: Option<String>,
    }
}

generate_to_row! {
    /// Represents data about the parsing process
    pub struct ScrapeRecord {
        id: i32,
        time_started: i32,
        time_ended: i32,
        terms_added: i8,
        courses_added: i32,
        courses_updated: i32,
        sections_added: i32,
        failures: i32,
    }
}

generate_to_row! {
    /// Represents record about a section of a class
    pub struct SectionRecord {
        id: i32,
        course: i32,
        section: String,
        title: String,
        section_type: SectionType,
        status: SectionStatus,
        instructor: String,
        maxenroll: i16,
        curenroll: i16,
    }
}

/// Represents the type of section
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema)]
pub enum SectionType {
    #[serde(rename = "R")]
    Regular,
    #[serde(rename = "N")]
    None,
    #[serde(rename = "OL")]
    Online,
    #[serde(rename = "H")]
    Honors,
    #[serde(rename = "BL")]
    Blended,
}

impl Default for SectionType {
    fn default() -> Self {
        Self::Regular
    }
}

impl Display for SectionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "R"),
            Self::None => write!(f, "N"),
            Self::Online => write!(f, "OL"),
            Self::Honors => write!(f, "H"),
            Self::Blended => write!(f, "BL"),
        }
    }
}

/// Represents the current status of the section
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema)]
pub enum SectionStatus {
    #[serde(rename = "O")]
    Open,
    #[serde(rename = "C")]
    Closed,
    #[serde(rename = "X")]
    Cancelled,
}

impl Default for SectionStatus {
    fn default() -> Self {
        Self::Open
    }
}

impl Display for SectionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Open => write!(f, "O"),
            Self::Closed => write!(f, "C"),
            Self::Cancelled => write!(f, "X"),
        }
    }
}

generate_to_row! {
    /// Record used to lightly organize class data for later parsing
    pub struct ClassRecord {
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
        class_type: char,
        schedule_print: SchedulePrint,
        enrollment_capacity: i32,
        enrollment_total: i32,
        institution: String,
        academic_org: String,
        academic_group: String,
        academic_career: String,
        instruction_mode: String,
        course_description_long: String,
    }
}

/// Represents whether a class has available seats or is full.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema, PartialEq)]
pub enum ClassStatus {
    #[serde(rename = "A")]
    Available,
    #[serde(rename = "X")]
    Cancelled,
}

impl Default for ClassStatus {
    fn default() -> Self {
        Self::Available
    }
}

impl Display for ClassStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Available => write!(f, "A"),
            Self::Cancelled => write!(f, "X"),
        }
    }
}

impl ClassStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "A" => Self::Available,
            "X" => Self::Cancelled,
            _ => Self::default(),
        }
    }
}

/// Represents whether a course is open for enrollment or not.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema, PartialEq)]
pub enum EnrollmentStatus {
    #[serde(rename = "O")]
    Open,
    #[serde(rename = "C")]
    Closed,
}

impl Default for EnrollmentStatus {
    fn default() -> Self {
        Self::Open
    }
}

impl Display for EnrollmentStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Open => write!(f, "O"),
            Self::Closed => write!(f, "C"),
        }
    }
}

impl EnrollmentStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "O" => Self::Open,
            "C" => Self::Closed,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema, PartialEq)]
pub enum SchedulePrint {
    #[serde(rename = "Y")]
    Visible,
    #[serde(rename = "N")]
    Hidden,
}

impl Default for SchedulePrint {
    fn default() -> Self {
        Self::Visible
    }
}

impl Display for SchedulePrint {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Visible => write!(f, "Y"),
            Self::Hidden => write!(f, "N"),
        }
    }
}

impl SchedulePrint {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Y" => Self::Visible,
            "N" => Self::Hidden,
            _ => Self::default(),
        }
    }
}

generate_to_row! {
    /// Describes the meeting schedule of a course.
    pub struct MeetingRecord {
        course_id: i32,
        course_offer_number: i32,
        academic_term: i32,
        session_code: String,
        class_section: String,
        class_meeting_number: i32,
        start_date: String,
        end_date: String,
        building: String,
        room_number: String,
        meeting_time_start:  String,
        meeting_time_end: String,
        monday: WeekdayScheduled,
        tuesday: WeekdayScheduled,
        wednesday: WeekdayScheduled,
        thursday: WeekdayScheduled,
        friday: WeekdayScheduled,
        saturday: WeekdayScheduled,
        sunday: WeekdayScheduled,
    }
}

/// Represents whether a section meets on a given day of the week.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema, PartialEq)]
pub enum WeekdayScheduled {
    #[serde(rename = "Y")]
    Scheduled,
    #[serde(rename = "N")]
    NotScheduled,
}

impl Default for WeekdayScheduled {
    fn default() -> Self {
        Self::Scheduled
    }
}

impl Display for WeekdayScheduled {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Scheduled => write!(f, "Y"),
            Self::NotScheduled => write!(f, "N"),
        }
    }
}

impl WeekdayScheduled {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Y" | "Scheduled" => Self::Scheduled,
            "N" | "NotScheduled" => Self::NotScheduled,
            _ => Self::default(),
        }
    }
}

generate_to_row! {
    /// Associates instructors to the courses they teach.
    pub struct InstructorRecord {
        course_id: i32,
        course_offer_number: i32,
        academic_term: i32,
        session_code: String,
        class_section: String,
        class_meeting_number: i32,
        last_name: String,
        first_name: String,
    }
}

/// Represents an individual /* department */
#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    pub id: i32,
    pub code: String,
    pub title: Option<String>,
}

/// Represents a term
#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub term_id: i32,
    pub term_name: String,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Year {
    pub year: String,
    pub terms: Vec<Term>,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CourseInfo {
    pub id: i32,
    pub code: Option<String>,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SectionInfo {
    pub id: i32,
    pub code: String,
    pub title: String,
    pub instructor: String,
    pub times: Vec<TimeInfo>,
}

impl Default for SectionInfo {
    fn default() -> Self {
        Self {
            id: i32::default(),
            code: String::default(),
            title: String::default(),
            instructor: String::default(),
            times: vec![],
        }
    }
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TimeInfo {
    pub day: i32,
    pub start: i32,
    pub end: i32,
    pub building_code: String,
    pub building_number: String,
    pub room: String,
}

/// Represents a course that fulfills the criteria set by the search
#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CourseOption {
    pub course_num: String,
    pub title: String,
    pub description: String,
    pub credits: i32,
    pub enrolled_students: i32,
    pub enrollment_capacity: i32,
    pub instructor_name: String,
    pub online: bool,
    pub times: Vec<Time>,
}

/// Represents the relevant information about a single section of a course
#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SingleSection {
    pub course_id: i32,
    pub title: String,
    pub description: String,
    pub credits: i32,
    pub section_id: i32,
    pub curr_enroll: i32,
    pub max_enroll: i32,
    pub instructor: String,
    pub dept_code: i32,
}

/// Represents the relevant information about a single meeting time for a section
#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SectionTimeInfo {
    pub day: i32,
    pub start_time: i32,
    pub end_time: i32,
    pub room: String,
    pub code: String,
    pub number: String,
    pub off_campus: bool,
}

/// More readable format to store information about a single meeting of a course
#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Time {
    pub building: Building,
    pub day: WeekDay,
    pub start: u32,
    pub end: u32,
    pub room: String,
    pub off_campus: bool,
}

/// Represents a single building
#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Building {
    pub code: String,
    pub number: String,
}

/// Represents a search by a user, given to the API
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Search {
    pub course: String,
    pub term: i32,
    pub ignore_full: bool,
    pub credit_hours: Option<i32>,
    pub title: Option<String>,
    pub professor_name: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub days: Option<Vec<bool>>,
    pub online: Option<bool>,
    pub honors: Option<bool>,
    pub off_campus: Option<bool>,
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

impl Display for WeekDay {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Sunday => write!(f, "0"),
            Self::Monday => write!(f, "1"),
            Self::Tuesday => write!(f, "2"),
            Self::Wednesday => write!(f, "3"),
            Self::Thursday => write!(f, "4"),
            Self::Friday => write!(f, "5"),
            Self::Saturday => write!(f, "6"),
        }
    }
}

impl Default for WeekDay {
    fn default() -> Self {
        Self::Sunday
    }
}

impl WeekDay {
    pub fn from_str(s: &str) -> Self {
        match s {
            "0" => Self::Sunday,
            "1" => Self::Monday,
            "2" => Self::Tuesday,
            "3" => Self::Wednesday,
            "4" => Self::Thursday,
            "5" => Self::Friday,
            "6" => Self::Saturday,
            _ => Self::default(),
        }
    }
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

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempSemTime {
    pub academic_term: i32,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SemTime {
    pub academic_term: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub academic_term: i32,
    pub subject: String,
    pub units: String,
    pub academic_org: String,
    pub catalog_number: String,
    pub description: String,
    pub course_description_long: String,
    pub course_id: i32,
    pub course_offer_number: i32,
    pub session_code: String,
}

impl PartialEq for Course {
    fn eq(&self, other: &Self) -> bool {
        self.academic_term == other.academic_term
            && self.subject == other.subject
            && self.units == other.units
            && self.academic_org == other.academic_org
            && self.catalog_number == other.catalog_number
            && self.description == other.description
            && self.course_description_long == other.course_description_long
            && self.course_id == other.course_id
            && self.course_offer_number == other.course_offer_number
            && self.session_code == other.session_code
    }
}

impl Eq for Course {}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CourseIOUResult {
    pub insertorupdatecourse: Option<Vec<i32>>,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SectionIOUResult {
    pub insertorupdatesection: Option<Vec<i32>>,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempSection {
    pub class_section: String,
    pub description: String,
    pub topic: String,
    pub enrollment_status: String,
    pub class_status: String,
    pub class_type: String,
    pub enrollment_capacity: i32,
    pub enrollment_total: i32,
    pub instruction_mode: String,
    pub schedule_print: String,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    pub class_section: String,
    pub description: String,
    pub topic: String,
    pub enrollment_status: EnrollmentStatus,
    pub class_status: ClassStatus,
    pub class_type: char,
    pub enrollment_capacity: i32,
    pub enrollment_total: i32,
    pub instruction_mode: String,
    pub schedule_print: SchedulePrint,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TempSectionMeeting {
    pub building: String,
    pub room_number: String,
    pub meeting_time_start: String,
    pub meeting_time_end: String,
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub saturday: String,
    pub sunday: String,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SectionMeeting {
    pub building: String,
    pub room_number: String,
    pub meeting_time_start: String,
    pub meeting_time_end: String,
    pub monday: WeekdayScheduled,
    pub tuesday: WeekdayScheduled,
    pub wednesday: WeekdayScheduled,
    pub thursday: WeekdayScheduled,
    pub friday: WeekdayScheduled,
    pub saturday: WeekdayScheduled,
    pub sunday: WeekdayScheduled,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    pub code: String,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub name: Option<String>,
}

#[derive(Serialize, Debug, Clone, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct WeekdayInfo {
    pub weekday: WeekDay,
    pub scheduled: WeekdayScheduled,
}
