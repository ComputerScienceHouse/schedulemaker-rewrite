-- Add migration script here
CREATE TYPE weekdayscheduled AS ENUM ('Scheduled', 'NotScheduled');

CREATE TABLE IF NOT EXISTS meetings(
    course_id INTEGER NOT NULL,
    course_offer_number INTEGER NOT NULL,
    academic_term INTEGER NOT NULL,
    session_code TEXT NOT NULL,
    class_section TEXT NOT NULL,
    class_meeting_number INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    building TEXT NOT NULL,
    room_number TEXT NOT NULL,
    meeting_time_start TEXT NOT NULL,
    meeting_time_end TEXT NOT NULL,
    monday weekdayscheduled NOT NULL,
    tuesday weekdayscheduled NOT NULL,
    wednesday weekdayscheduled NOT NULL,
    thursday weekdayscheduled NOT NULL,
    friday weekdayscheduled NOT NULL,
    saturday weekdayscheduled NOT NULL,
    sunday weekdayscheduled NOT NULL
);
