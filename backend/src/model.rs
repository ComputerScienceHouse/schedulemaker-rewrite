//! A collection of all the data entities parsed and used by ScheduleMaker.
//!
//! Some of these records are a direct mapping to the data files given by
//! SIS with some deserialization sugar to add meaning and make it is easier
//! to parse into other tables for the actual backend for ScheduleMaker.
//! All the other records represent the more permanent tables that the
//! backend queries for its information.

use serde::{Deserialize, Serialize};
use sqlx::{query_builder::Separated, Postgres, FromRow};
use std::fmt;
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
                $field_name: $field_type,
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
        last_accessed: chrono::DateTime<chrono::Utc>,
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
        title: String,
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
        ssr_componenet: String,
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
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema)]
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

/// Represents whether a course is open for enrollment or not.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema)]
pub enum SchedulePrint {
    #[serde(rename = "Y")]
    Visible,
    #[serde(rename = "N")]
    Hidden,
}

/// Represents whether a class is visible or not
impl Default for SchedulePrint {
    fn default() -> Self {
        Self::Visible
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
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone, ToSchema)]
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

impl fmt::Display for WeekdayScheduled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Scheduled => write!(f, "Y"),
            NotScheduled => write!(f, "N"),
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

