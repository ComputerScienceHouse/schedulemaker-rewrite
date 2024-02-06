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
    `course`        INT UNSIGNED,
    `section`       VARCHAR(4),
    `title`         VARCHAR(30),
    `type`          ENUM('R', 'N', 'OL', 'H', 'BL') DEFAULT 'R',
    `status`        ENUM('O', 'C', 'X'),
    `instructor`    VARCHAR(64) DEFAULT 'TBA',
    `maxenroll`     SMALLINT(3) UNSIGNED,
    `curenroll`     SMALLINT(3) UNSIGNED
);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT UQ_sections_course_section
    UNIQUE (`course`, `section`);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT NN_sections_all
    UNIQUE (`id`, `course`, `section`, `title`, `type`, `status`, `instructor`, `maxenroll`, `curenroll`);

-- FOREIGN KEY -------------------------------------------------------------
ALTER TABLE sections
    ADD CONSTRAINT FK_sections_course
    FOREIGN KEY (`course`)
    REFERENCES `courses`(`id`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

