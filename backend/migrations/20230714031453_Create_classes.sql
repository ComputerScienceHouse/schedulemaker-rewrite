CREATE TYPE classstatus AS ENUM ('Available', 'Full');
CREATE TYPE enrollmentstatus AS ENUM ('Open', 'Closed');
CREATE TYPE scheduleprint AS ENUM ('Visible', 'Hidden');

CREATE TABLE IF NOT EXISTS classes (
    course_id INTEGER NOT NULL,
    course_offer_number INTEGER NOT NULL,
    academic_term INTEGER NOT NULL,
    session_code TEXT NOT NULL,
    class_section TEXT NOT NULL,
    subject TEXT NOT NULL,
    catalog_number TEXT NOT NULL,
    description TEXT NOT NULL,
    topic TEXT NOT NULL,
    class_number INTEGER NOT NULL,
    ssr_component TEXT NOT NULL,
    units TEXT NOT NULL,
    enrollment_status enrollmentstatus NOT NULL,
    class_status classstatus NOT NULL,
    class_type CHAR NOT NULL,
    schedule_print scheduleprint NOT NULL,
    enrollment_capacity INTEGER NOT NULL,
    enrollment_total INTEGER NOT NULL,
    institution TEXT NOT NULL,
    academic_org TEXT NOT NULL,
    academic_group TEXT NOT NULL,
    academic_career TEXT NOT NULL,
    instruction_mode TEXT NOT NULL,
    course_description_long TEXT NOT NULL
);
