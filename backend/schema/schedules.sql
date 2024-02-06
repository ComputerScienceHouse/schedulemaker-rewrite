-- -------------------------------------------------------------------------
-- Saved Schedule Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing saved schedule records.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedules (
    `id`            INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `lastaccessed`  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    `startday`      TINYINT(1) UNSIGNED DEFAULT 0,
    `endday`        TINYINT(1) UNSIGNED DEFAULT 6,
    `starttime`     TINYINT(2) UNSIGNED DEFAULT 8,
    `endtime`       TINYINT(2) UNSIGNED DEFAULT 22,
    `term`          SMALLINT(4) UNSIGNED,
    `image`         BOOL DEFAULT FALSE
);

-- NOTNULL CONSTRAINT -----------------------------------------------------
ALTER TABLE `schedules`
    ADD CONSTRAINT NN_schedules_all
    NOT NULL (`id`, `lastaccessed`, `startday`, `endday`, `starttime`, `endtime`, `term`, `image`);

-- FOREIGN KEY ------------------------------------------------------------
ALTER TABLE `schedules`
    ADD FOREIGN KEY FK_schedules_term(`term`)
    REFERENCES `academicterms`(`term`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

