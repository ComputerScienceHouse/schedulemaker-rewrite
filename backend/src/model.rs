//! A collection of all the data entities parsed and used by ScheduleMaker.
//!
//! Each record type has a direct mapping to the data files given by SIS,
//! but has some nice deserialization sugar to add more semantic domain
//! knowledge into the definitions themselves. For example, the availability
//! of a class is encoded in the data file as "A" and "X" to represent that
//! the class has available seats or is full, respectively. In Rust, however,
//! we're able to encode the purposes of each field more cleanly with named
//! enum variants and a serde rename rule.

use serde::{Deserialize, Serialize};
use sqlx::{query_builder::Separated, FromRow, Postgres};
use std::fmt::Display;

pub trait ToRow {
    fn keys() -> &'static [&'static str];
    fn bind<'qb, 'args: 'qb, Sep: Display>(
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
            // This is purely an example—not a good one.
            fn keys() -> &'static [&'static str] {
                &[$(stringify!($field_name)),*]
            }

            fn bind<'qb, 'args: 'qb, Sep: Display>(
                &self,
                query_builder: &mut Separated<'qb,'args,Postgres,Sep>
            ) {
                query_builder
                $(.push_bind(self.$field_name.to_encode()))*;
            }
        }
    }
}

generate_to_row! {
/// Contains all of the metadata regarding a given course.
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
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
pub enum ClassStatus {
    #[serde(rename = "A")]
    Available,
    #[serde(rename = "X")]
    Full,
}

impl Default for ClassStatus {
    fn default() -> Self {
        Self::Available
    }
}

/// Represents whether a course is open for enrollment or not.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
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

#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
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

generate_to_row! {
/// Describes the schedule of a given course.
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
    meeting_time_start: String,
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

/// Whether a given day of the week is part of a course section's schedule.
#[derive(Debug, Serialize, Deserialize, sqlx::Type, Clone)]
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

generate_to_row! {
/// Associates an instructor's name to a given course.
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
