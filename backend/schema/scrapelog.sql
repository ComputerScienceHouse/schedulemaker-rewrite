-- -------------------------------------------------------------------------
-- Scrape Log Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing the status of past dump processing cron jobs.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS scrapelog (
    `id`                INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `timestarted`       INT(11) UNSIGNED,
    `timeended`         INT(11) UNSIGNED,
    `termsadded`        TINYINT(3) UNSIGNED,
    `coursesadded`      INT UNSIGNED,
    `coursesupdated`    INT UNSIGNED,
    `sectionsadded`     INT UNSIGNED,
    `failures`          INT UNSIGNED
);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE `scrapelog`
    ADD CONSTRAINT NN_scrapelog_all
    NOT NULL (`id`, `timeStarted`, `timeEnded`, `termsAdded`, `coursesAdded`, `coursesUpdated`, `sectionsAdded`, `failures`);

