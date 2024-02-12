//! A collection of all the data entities parsed and used by ScheduleMaker.
//!
//! Some of these records are a direct mapping to the data files given by
//! SIS with some deserialization sugar to add meaning and make it is easier
//! to parse into other tables for the actual backend for ScheduleMaker.
//! All the other records represent the more permanent tables that the
//! backend queries for its information.

use serde::{Deserialize, Serialize};
use sqlx::{mysql, query_builder::Separated, types::chrono, FromRow, MySql};
use std::fmt::Display;
use utoipa::ToSchema;

pub trait ToRow {
    fn keys() -> &'static [&'static str];
    fn bind<'qb, 'args: 'qb, Sep: Display>(
        &self,
        query_builder: &mut Separated<'qb, 'args, MySql, Sep>,
    );
}

trait ToEncode<'a, T: sqlx::Type<MySql> + sqlx::Encode<'a, MySql>> {
    fn to_encode(&self) -> T;
}

impl<'a, T: sqlx::Type<MySql> + sqlx::Encode<'a, MySql> + Clone> ToEncode<'a, T> for T {
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
                $(#[doc = field_doc:expr])?
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

            fn bind<'qb, 'args: 'qb, Sep: Display>(
                &self,
                query_builder: &mut Separated<'qb, 'args, MySql, Sep>
            ) {
                query_builder
                $(.push_bind(self.$field_name.to_encode()))*;
            }
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
    /// Represents the record of academic terms
    pub struct AcademicTermRecord {
        term: u16,
        start: chrono::Date,
        end: chrono::Date,
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
        id: u32,
        term: u16,
        department: u32,
        course: String,
        credits: u8,
        title: String,
        description: String,
    }
}

generate_to_row! {
    /// Represents the academic departments a class can be offered by
    pub struct DepartmentRecord {
        id: u32,
        school: u32,
        code: String,
        title: String,
    }
}

generate_to_row! {
    /// Represents a part of a schedule that is a course
    pub struct ScheduleCourseRecord {
        id: u32,
        schedule: u32,
        section: u32,
    }
}

generate_to_row! {
    /// Represents a part of a schedule that is not a course
    pub struct ScheduleNonCourseRecord {
        id: u32,
        schedule: u32,
        title: String,
        day: u8,
        start: u16,
        end: u16,
    }
}

generate_to_row! {
    /// Represents created schedules
    pub struct ScheduleRecord {
        id: u32,
        last_accessed: chrono::DateTime<chrono::Utc>,
        start_day: u8,
        end_day: u8,
        start_time: u8,
        end_time: u8,
        term: u16,
        image: bool,
    }
}

generate_to_row! {
    /// Represents the schools that a course can be offered by
    pub struct SchoolRecord {
        id: u32,
        code: String,
        title: String,
    }
}

generate_to_row! {
    /// Represents data about the parsing process
    pub struct ScrapeRecord {
        id: u32,
        time_started: u32,
        time_ended: u32,
        terms_added: u8,
        courses_added: u32,
        courses_updated: u32,
        sections_added: u32,
        failures: u32,
    }
}

generate_to_row! {
    /// Represents record about a section of a class
    pub struct SectionRecord {
        id: u32,
        course: u32,
        section: String,
        title: String,
        type: SectionType,
        status: SectionStatus,
        instructor: String,
        maxenroll: u16,
        curenroll: u16,
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
    /// Represents a meeting for a section of a class
    pub struct MeetingRecord {
        id: u32,
        section: u32,
        day: u8,
        start: u16,
        end: u16,
        building: String,
        room: String,
    }
}

