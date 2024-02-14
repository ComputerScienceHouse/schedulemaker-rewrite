-- -------------------------------------------------------------------------
-- Saved Schedule Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing saved schedule records.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedules (
    `id`            INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `lastaccessed`  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `startday`      TINYINT(1) UNSIGNED NOT NULL DEFAULT 0,
    `endday`        TINYINT(1) UNSIGNED NOT NULL DEFAULT 6,
    `starttime`     TINYINT(2) UNSIGNED NOT NULL DEFAULT 8,
    `endtime`       TINYINT(2) UNSIGNED NOT NULL DEFAULT 22,
    `term`          SMALLINT(4) UNSIGNED NOT NULL,
    `image`         BOOL NOT NULL DEFAULT FALSE
);

-- FOREIGN KEY ------------------------------------------------------------
ALTER TABLE `schedules`
    ADD CONSTRAINT FK_schedules_term
    FOREIGN KEY `schedules`(`term`)
    REFERENCES `academicterms`(`term`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

