-- -------------------------------------------------------------------------
-- Section Times Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @decsrip Table for storing the times and locations that a section meets.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS times (
    `id`        INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `section`   INT UNSIGNED NOT NULL,
    `day`       TINYINT(1) UNSIGNED NOT NULL,
    `start`     SMALLINT(4) UNSIGNED NOT NULL,
    `end`       SMALLINT(4) UNSIGNED NOT NULL,
    `building`  VARCHAR(5) NOT NULL DEFAULT "00",
    `room`      VARCHAR(10) NOT NULL DEFAULT "0000"
);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE times
    ADD CONSTRAINT FK_meetings_section
    FOREIGN KEY (`section`)
    REFERENCES sections(`id`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

ALTER TABLE meetings
    ADD CONSTRAINT FK_meetings_building
    FOREIGN KEY (`building`)
    REFERENCES buildings(`number`)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

