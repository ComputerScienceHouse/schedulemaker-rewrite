-- -------------------------------------------------------------------------
-- Departments Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip This table holds department codes and numbers. We want to have
--          one record for a department in semesters and one record for a
--          department in quarters
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS departments (
    `id`        INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `school`    INT UNSIGNED UNIQUE NOT NULL,
    `code`      VARCHAR(4) UNIQUE NOT NULL,
    `title`     VARCHAR(100) NOT NULL
);

-- FOREIGN KEYS -----------------------------------------------------------
ALTER TABLE `departments` ADD INDEX `departments`(`school`);
ALTER TABLE `departments` ADD CONSTRAINT FK_school FOREIGN KEY `departments`(`school`)
    REFERENCES `schools`(`id`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

