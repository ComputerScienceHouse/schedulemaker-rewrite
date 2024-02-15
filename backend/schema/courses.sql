-- -------------------------------------------------------------------------
-- Courses Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for courses. These are linked to departments and quarters
--          in a one quarter/department to many courses. These are also linked
--          to sections in a one course to many sections fashion.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS courses (
    id              SERIAL PRIMARY KEY,
    term            SMALLINT NOT NULL,
    department      INT NOT NULL,
    course          VARCHAR(4) NOT NULL,
    credits         SMALLINT NOT NULL,
    title           VARCHAR(50) NOT NULL,
    description     TEXT NOT NULL
);

-- UNSIGNED CONSTRAINTS ----------------------------------------------------
ALTER TABLE courses
    ADD CONSTRAINT CH_courses_id_pos
    CHECK (id >= 0);

ALTER TABLE courses
    ADD CONSTRAINT CH_courses_term_pos
    CHECK (term >= 0);

ALTER TABLE courses
    ADD CONSTRAINT CH_courses_dept_pos
    CHECK (department >= 0);
 
ALTER TABLE courses
    ADD CONSTRAINT CH_courses_credits_pos
    CHECK (credits >= 0);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE courses
    ADD CONSTRAINT UQ_courses_term_dept_course
    UNIQUE (term, department, course);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE courses
    ADD CONSTRAINT FK_courses_term
    FOREIGN KEY (term)
    REFERENCES academicterms(term)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

ALTER TABLE courses
    ADD CONSTRAINT FK_courses_dept
    FOREIGN KEY (department)
    REFERENCES departments(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

