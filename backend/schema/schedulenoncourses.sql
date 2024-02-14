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
    `schedule`  INT UNSIGNED NOT NULL,
    `title`     VARCHAR(30) NOT NULL,
    `day`       TINYINT(1) UNSIGNED NOT NULL,
    `start`     SMALLINT(4) UNSIGNED NOT NULL,
    `end`       SMALLINT(4) UNSIGNED NOT NULL
);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE `schedulenoncourses`
    ADD FOREIGN KEY FK_schednoncourses_schedule(`schedule`)
    REFERENCES `schedules`(`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

