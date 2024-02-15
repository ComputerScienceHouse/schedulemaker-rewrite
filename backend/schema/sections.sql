-- -------------------------------------------------------------------------
-- Sections Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing all the sections and their information. They are
--          also linked up with their parent course.
-- -------------------------------------------------------------------------

-- TYPE CREATION -----------------------------------------------------------
CREATE TYPE SECTIONTYPE AS ENUM('R', 'N', 'OL', 'H', 'BL');
CREATE TYPE SECTIONSTATUS AS ENUM('O', 'C', 'X');

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS sections (
    id              SERIAL PRIMARY KEY,
    course          INT NOT NULL,
    section         VARCHAR(4) NOT NULL,
    title           VARCHAR(30) NOT NULL,
    section_type    SECTIONTYPE NOT NULL DEFAULT 'R',
    status          SECTIONSTATUS NOT NULL,
    instructor      VARCHAR(64) NOT NULL DEFAULT 'TBA',
    max_enroll      SMALLINT NOT NULL,
    curr_enroll     SMALLINT NOT NULL
);

-- UNSIGNED CONSTRAINTS ----------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT CH_sections_id_pos
    CHECK (id >= 0);

ALTER TABLE sections
    ADD CONSTRAINT CH_sections_course_pos
    CHECK (course >= 0);

ALTER TABLE sections
    ADD CONSTRAINT CH_sections_maxenroll_pos
    CHECK (max_enroll >= 0);

ALTER TABLE sections
    ADD CONSTRAINT CH_sections_currenroll_pos
    CHECK (curr_enroll >= 0);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT UQ_sections_course_section
    UNIQUE (course, section);

-- FOREIGN KEY -------------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT FK_sections_course
    FOREIGN KEY (course)
    REFERENCES courses(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

