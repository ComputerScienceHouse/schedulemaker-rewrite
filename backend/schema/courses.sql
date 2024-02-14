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
    `id`            INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `term`          SMALLINT UNSIGNED NOT NULL,
    `department`    INT UNSIGNED NOT NULL,
    `course`        VARCHAR(4) NOT NULL,
    `credits`       TINYINT(2) UNSIGNED NOT NULL,
    `title`         VARCHAR(50) NOT NULL,
    `description`   TEXT NOT NULL
);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE `courses`
    ADD CONSTRAINT UQ_courses_quarter_department_course
    UNIQUE (`term`, `department`, `course`);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE `courses`
    ADD FOREIGN KEY FK_courses_quarter(`quarter`)
    REFERENCES `academicterms`(`term`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

ALTER TABLE `courses`
    ADD FOREIGN KEY FK_courses_dept(`department`)
    REFERENCES `departments`(`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

