import Alert from "../components/alert";
import Reset from "../components/reset";
import TermSelect from "../components/termSelect";
import SubPage from "./subpage";
import ScheduleCourse from "../components/scheduleCourse";
import NonCourse from "../components/nonCourse";
import NoCourse from "../components/noCourse";
import { Link } from "react-router-dom";
import React, { useState } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'


function Generate() {
  const [courses, setCourses] = useState([<ScheduleCourse />]);
  const [nonSchedule, setNonSchedule] = useState([]);
  const [noSchedule, setNoSchedule] = useState([]);
  const [activeTerm, setActiveTerm] = useState(-1);
  return (
    <SubPage>
      <Alert>
        Use a comma to separate courses to see which course fits your schedule
        better. Add courses from the Browse or Search page to your schedule as
        well so you can easily create schedule combinations from anywhere! Also,
        check out the{" "}
        <Link ui-sref="help" to="/help">
          help
        </Link>{" "}
        page for new keyboard shortcuts.
      </Alert>
      <TermSelect title="Generate Schedules" setActiveTerm={setActiveTerm}>
        <div className="panel-body">
          <div id="scheduleCourses">
            <div
              dynamic-items="state.courses"
              color="ui.colors"
              use-class="scheduleCourse"
            >
              {React.Children.map(courses, course => React.cloneElement(course, { activeTerm }))}
            </div>
          </div>
        </div>
        <div className="panel-footer">
          <input
            type="hidden"
            value="1"
            name="courseCount"
            id="courseCount"
            autoComplete="off"
          />
          <div className="row">
            <div className="col-md-4 col-xs-6">
              <button type="button" className="btn-default btn btn-block">
                <FontAwesomeIcon icon={icon({ name: "square", style: "regular" })} /> Ignore full
              </button>
            </div>
            <div className="col-md-4 col-md-offset-4 col-xs-6">
              <button
                className="btn btn-primary btn-block"
                type="button"
                title="Shortcut: Enter"
                onClick={() => setCourses([...courses, <ScheduleCourse />])}
              >
                <FontAwesomeIcon icon={icon({ name: "plus" })} /> Add Course
              </button>
            </div>
          </div>
        </div>
      </TermSelect>
      <div>&nbsp;</div>
      <div>
        <div className="panel panel-default panel-control-overlap">
          <div className="panel-heading form-horizontal">
            <div className="form-horizontal row">
              <div className="col-xs-12">
                <h2 className="panel-title">Non-Course Schedule Items</h2>
              </div>
            </div>
          </div>
          <div className={nonSchedule.length === 0 ? "panel-body ng-hide" : "panel-body"} hidden={nonSchedule.length === 0}>
            {nonSchedule}
          </div>
          <div className="panel-footer">
            <div className="row">
              <div className="col-md-4 col-md-offset-8">
                <button type="button" className="btn btn-block btn-primary">
                  <FontAwesomeIcon icon={icon({ name: "plus" })} /> Add Item
                </button>
              </div>
            </div>
          </div>
        </div>
        <div className="panel panel-default panel-control-overlap">
          <div className="panel-heading form-horizontal">
            <div className="form-horizontal row">
              <div className="col-xs-12">
                <h2 className="panel-title">Times You Don't Want Classes</h2>
              </div>
            </div>
          </div>
          <div className={noSchedule.length === 0 ? "panel-body ng-hide" : "panel-body"} hidden={noSchedule.length === 0}>
            {noSchedule}
          </div>
          <div className="panel-footer">
            <div className="row">
              <div className="col-md-4 col-md-offset-8">
                <button type="button" className="btn btn-block btn-primary">
                  <i className="plus"></i> Add Item
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="btn-group">
        <Reset />
      </div>
    </SubPage>
  );
}

export default Generate;
