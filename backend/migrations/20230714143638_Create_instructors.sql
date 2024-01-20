-- Add migration script here
CREATE TABLE IF NOT EXISTS instructors(
    course_id INTEGER NOT NULL,
    course_offer_number INTEGER NOT NULL,
    academic_term INTEGER NOT NULL,
    session_code TEXT NOT NULL,
    class_section TEXT NOT NULL,
    class_meeting_number INTEGER NOT NULL,
    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL
);
