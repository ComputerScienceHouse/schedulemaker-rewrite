-- -------------------------------------------------------------------------
-- Academic Terms Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for academic terms. Changed RIT's quarters terms to more
--          closely match the current format of semesters. This change keeps
--          the database ordered and can be parsed as needed in the API.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS academicterms (
    `term`      SMALLINT(5) UNSIGNED PRIMARY KEY,
    `start`     DATE NOT NULL,
    `end`       DATE NOT NULL
);

