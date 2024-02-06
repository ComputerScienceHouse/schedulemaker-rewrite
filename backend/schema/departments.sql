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
    `id`        INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    `school`    INT UNSIGNED,
    `code`      VARCHAR(4),
    `title`     VARCHAR(100)
);

-- UNIQUE CONSTRAINT ------------------------------------------------------
ALTER TABLE `departments`
    ADD CONSTRAINT UQ_departments_number_code
    UNIQUE (`number`, `code`);

-- NOTNULL CONSTRAINT -----------------------------------------------------
ALTER TABLE `departments`
    ADD CONSTRAINT NN_departments_all
    NOT NULL (`id`, `school`, `code`, `title`);

-- FOREIGN KEYS -----------------------------------------------------------
ALTER TABLE `departments` ADD INDEX `departments`(`school`);

ALTER TABLE `departments` ADD CONSTRAINT FK_school FOREIGN KEY `departments`(`school`)
    REFERENCES `schools`(`id`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

