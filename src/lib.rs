pub mod dat;

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
    enrollment_status: char,
    class_status: char,
    class_type: char,
    schedule_print: char,
    enrollment_capacity: u32,
    enrollment_total: u32,
    institution: String,
    academic_org: String,
    academic_group: String,
    academic_career: String,
    instruction_mode: String,
    course_description_long: String,
}
