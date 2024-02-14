-- -------------------------------------------------------------------------
-- Saved Schedule Sections Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for linked sections with saved schedules.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedulecourses (
    `id`        INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `schedule`  INT UNSIGNED NOT NULL,
    `section`   INT UNSIGNED NOT NULL
);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE `schedulecourses`
    ADD FOREIGN KEY FK_schedcourses_schedule(`schedule`)
    REFERENCES `schedules`(`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

ALTER TABLE `schedulecourses`
    ADD FOREIGN KEY FK schedcourses_section(`section`)
    REFERENCES `sections`(`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE;

