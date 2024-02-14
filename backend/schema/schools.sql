-- -------------------------------------------------------------------------
-- School Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip This table holds school codes and links them to numbers and to
--          the name of the school.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schools (
    `id`        INT UNSIGNED PRIMARY KEY,
    `code`      VARCHAR(5) UNIQUE NOT NULL,
    `title`     VARCHAR(30) NOT NULL
);

