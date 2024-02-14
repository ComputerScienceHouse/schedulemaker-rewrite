-- -------------------------------------------------------------------------
-- Sections Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing all the sections and their information. They are
--          also linked up with their parent course.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS sections (
    `id`            INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `course`        INT UNSIGNED NOT NULL,
    `section`       VARCHAR(4) NOT NULL,
    `title`         VARCHAR(30) NOT NULL,
    `type`          ENUM('R', 'N', 'OL', 'H', 'BL') NOT NULL DEFAULT 'R',
    `status`        ENUM('O', 'C', 'X') NOT NULL,
    `instructor`    VARCHAR(64) NOT NULL DEFAULT 'TBA',
    `maxenroll`     SMALLINT(3) UNSIGNED NOT NULL,
    `curenroll`     SMALLINT(3) UNSIGNED NOT NULL
);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT UQ_sections_course_section
    UNIQUE (`course`, `section`);

-- FOREIGN KEY -------------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT FK_sections_course
    FOREIGN KEY `sections`(`course`)
    REFERENCES `courses`(`id`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

