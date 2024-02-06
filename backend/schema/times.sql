-- -------------------------------------------------------------------------
-- Section Meetings Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @decsrip Table for storing the times and locations that a section meets.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS meetings (
    `id`        INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `section`   INT UNSIGNED,
    `day`       TINYINT(1) UNSIGNED,
    `start`     SMALLINT(4) UNSIGNED,
    `end`       SMALLINT(4) UNSIGNED,
    `building`  VARCHAR(5) DEFAULT "00",
    `room`      VARCHAR(10) DEFAULT "0000"
);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE meetings
    ADD CONSTRAINT NN_meetings_all
    NOT NULL (`id`, `section`, `day`, `start`, `end`, `building`, `room`);

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

