import { useEffect, useState } from "react";
import getCourseChildren from "../components/courseChildren";
import SectionOptions from "../components/sectionOptions";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'

const EnterCourse = (
  <button
    title="Shortcut: Ctrl + Alt + Down"
    type="button"
    class="btn btn-primary btn-block"
    disabled="disabled"
  >
    Please enter a course
  </button>
);

const NoMatches = (
  <button class="btn btn-block course-error alert alert-danger alert-sm">No Courses Match</button>
)

const CourseResults = (props) => {
  let action = props.opened ? "Hide" : "Show";
  let openedIcon = props.opened ? icon({name: "angle-up"}) : icon({name: "angle-down"})
  return (
    <button
      title="Shortcut: Ctrl + Alt + Down"
      type="button"
      class="btn btn-primary btn-block"
    >
      <FontAwesomeIcon icon={openedIcon}/> {action} {props.num} Results
    </button>
  );
};

const ScheduleCourse = (props) => {
  const [courseData, setCourseData] = useState("");
  const [courseStatus, setCourseStatus] = useState(EnterCourse);
  const [options, setOptions] = useState([]);
  const [loading, setLoading] = useState(false);
  const [loadIcon, setLoadIcon] = useState(<FontAwesomeIcon icon={icon({name: "times"})}/>);

  const updateCourse = (e) => {
    setCourseData(e.target.value);
  };

  useEffect(() => {
    console.log(props.activeTerm)
  }, [props.activeTerm]);

  useEffect(() => {
    if (courseData.length >= 4) {
      const fetchData = async () => {
        let courseChildren = await getCourseChildren({
          term: props.activeTerm,
          course: courseData,
          ignoreFull: false,
        }, setLoading);
        setOptions(courseChildren);
      }
      fetchData();
    }
    setOptions([]);
  }, [courseData, props.activeTerm]);

  useEffect(() => {
    if (courseData.length < 4) {
      setCourseStatus(EnterCourse);
    } else if (options.length === 0) {
      setCourseStatus(NoMatches);
    } else {
      setCourseStatus(<CourseResults opened num={options.length} />);
    }
  }, [options, courseData]);

  useEffect(() => {
    if (loading) {
      setLoadIcon(<FontAwesomeIcon icon={icon({name: "rotate"})} spin/>);
    } else {
      setLoadIcon(<FontAwesomeIcon icon={icon({name: "times"})}/>);
    }
  }, [loading]);

  return (
    <div class="scheduleCourse repeat-item no-repeat-item-animation">
      <div class="row margin-bottom-sm">
        <div class="col-md-8">
          <div class="form-group">
            <div class="col-sm-12 col-xs-12">
              <div class="input-group">
                <input
                  autocapitalize="off"
                  autocorrect="off"
                  spellcheck="off"
                  autocomplete="off"
                  id="courses1"
                  class="form-control searchField mousetrap"
                  type="text"
                  name="courses1"
                  placeholder="DEPT-CRS-SECT, DEPT-CRS-SECT..."
                  onChange={updateCourse}
                />{" "}
                <span class="input-group-btn">
                  <button
                    title="Shortcut: Esc"
                    type="button"
                    class="btn btn-default"
                  >
                    {loadIcon}
                  </button>
                </span>
              </div>
            </div>
          </div>
        </div>
        <div class="col-md-4">
          <div class="form-group course-result hidden-xs hidden-sm">
            <div class="col-xs-12">
              {courseStatus}
            </div>
          </div>
        </div>
      </div>
      <SectionOptions options={options}/>
    </div>
  );
};

export default ScheduleCourse;
