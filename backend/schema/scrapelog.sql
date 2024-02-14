-- -------------------------------------------------------------------------
-- Scrape Log Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing the status of past dump processing cron jobs.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS scrapelog (
    `id`                INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    `timestarted`       INT(11) UNSIGNED NOT NULL,
    `timeended`         INT(11) UNSIGNED NOT NULL,
    `termsadded`        TINYINT(3) UNSIGNED NOT NULL,
    `coursesadded`      INT UNSIGNED NOT NULL,
    `coursesupdated`    INT UNSIGNED NOT NULL,
    `sectionsadded`     INT UNSIGNED NOT NULL,
    `failures`          INT UNSIGNED NOT NULL
);

