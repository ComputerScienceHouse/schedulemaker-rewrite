-- -------------------------------------------------------------------------
-- Saved Non-Course Schedule Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing non-course items for saved scheduled and linking
--          them to saved schedules.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedulenoncourses (
    `id`        INT UNSIGNED PRIMARY KEY,
    `schedule`  INT UNSIGNED,
    `title`     VARCHAR(30),
    `day`       TINYINT(1) UNSIGNED,
    `start`     SMALLINT(4) UNSIGNED,
    `end`       SMALLINT(4) UNSIGNED
);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE `schedulenoncourses`
    ADD CONSTRAINT NN_schednoncourses_all
    NOT NULL (`id`, `schedule`, `title`, `day`, `start`, `end`);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE `schedulenoncourses`
    ADD FOREIGN KEY FK_schednoncourses_schedule(`schedule`)
    REFERENCES `schedules`(`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

