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
    `term`          SMALLINT UNSIGNED,
    `department`    INT UNSIGNED,
    `course`        VARCHAR(4),
    `credits`       TINYINT(2) UNSIGNED,
    `title`         VARCHAR(50),
    `description`   TEXT
);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE `courses`
    ADD CONSTRAINT UQ_courses_quarter_department_course
    UNIQUE (`term`, `department`, `course`);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE `courses`
    ADD CONSTRAINT NN_courses_all
    NOT NULL (`id`, `term`, `department`, `course`, `credits`, `title`, `description`);

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

