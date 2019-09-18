//! A collection of all the data entities parsed and used by ScheduleMaker.
//!
//! Each record type has a direct mapping to the data files given by SIS,
//! but has some nice deserialization sugar to add more semantic domain
//! knowledge into the definitions themselves. For example, the availability
//! of a class is encoded in the data file as "A" and "X" to represent that
//! the class has available seats or is full, respectively. In Rust, however,
//! we're able to encode the purposes of each field more cleanly with named
//! enum variants and a serde rename rule.

use serde::Deserialize;

/// Contains all of the metadata regarding a given course.
#[derive(Debug, Deserialize)]
pub struct ClassRecord {
    course_id: u32,
    course_offer_number: u32,
    strm: u32,
    session_code: String,
    class_section: String,
    subject: String,
    catalog_number: String,
    description: String,
    topic: String,
    class_number: u32,
    ssr_component: String,
    units: String,
    enrollment_status: EnrollmentStatus,
    class_status: ClassStatus,
    class_type: char,
    schedule_print: SchedulePrint,
    enrollment_capacity: u32,
    enrollment_total: u32,
    institution: String,
    academic_org: String,
    academic_group: String,
    academic_career: String,
    instruction_mode: String,
    course_description_long: String,
}

/// Represents whether a class has available seats or is full.
#[derive(Debug, Deserialize)]
pub enum ClassStatus {
    #[serde(rename = "A")]
    Available,
    #[serde(rename = "X")]
    Full,
}

/// Represents whether a course is open for enrollment or not.
#[derive(Debug, Deserialize)]
pub enum EnrollmentStatus {
    #[serde(rename = "O")]
    Open,
    #[serde(rename = "C")]
    Closed,
}

#[derive(Debug, Deserialize)]
pub enum SchedulePrint {
    #[serde(rename = "Y")]
    Visible,
    #[serde(rename = "N")]
    Hidden,
}

/// Describes the schedule of a given course.
#[derive(Debug, Deserialize)]
pub struct MeetingRecord {
    course_id: u32,
    course_offer_number: u32,
    strm: u32,
    session_code: String,
    class_section: String,
    class_meeting_number: u32,
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

/// Whether a given day of the week is part of a course section's schedule.
#[derive(Debug, Deserialize)]
pub enum WeekdayScheduled {
    #[serde(rename = "Y")]
    Scheduled,
    #[serde(rename = "N")]
    NotScheduled,
}

/// Associates an instructor's name to a given course.
#[derive(Debug, Deserialize)]
pub struct InstructorRecord {
    course_id: u32,
    course_offer_number: u32,
    strm: u32,
    session_code: String,
    class_section: String,
    class_meeting_number: u32,
    last_name: String,
    first_name: String,
}
