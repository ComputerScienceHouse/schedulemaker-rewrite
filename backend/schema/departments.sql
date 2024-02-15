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
    id          SERIAL PRIMARY KEY,
    school      INT UNIQUE NOT NULL,
    code        VARCHAR(4) UNIQUE NOT NULL,
    title       VARCHAR(100) NOT NULL
);

-- UNSIGNED CONSTRAINT ----------------------------------------------------
ALTER TABLE departments
    ADD CONSTRAINT CH_dept_id
    CHECK (id >= 0);

ALTER TABLE departments
    ADD CONSTRAINT CH_dept_school
    CHECK (school >= 0);

-- FOREIGN KEYS -----------------------------------------------------------
ALTER TABLE departments
    ADD CONSTRAINT FK_dept_school
    FOREIGN KEY (school)
    REFERENCES schools(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

