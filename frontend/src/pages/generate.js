import Alert from "../components/alert";
import Reset from "../components/reset";
import TermSelect from "../components/termselect";
import SubPage from "./subpage";
import ScheduleCourse from "../components/scheduleCourse";
import NonCourse from "../components/nonCourse";
import NoCourse from "../components/noCourse";
import { Link } from "react-router-dom";
import { useState } from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'


function Generate() {
  const [courses, setCourses] = useState([<ScheduleCourse />]);
  const [nonSchedule, setNonSchedule] = useState([]);
  const [noSchedule, setNoSchedule] = useState([]);
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
      <TermSelect title="Generate Schedules">
        <div className="panel-body">
          <div id="scheduleCourses">
            <div
              dynamic-items="state.courses"
              colors="ui.colors"
              use-class="scheduleCourse"
              helpers="courses_helpers"
            >
              {courses}
            </div>
          </div>
        </div>
        <div class="panel-footer">
          <input
            type="hidden"
            value="1"
            name="courseCount"
            id="courseCount"
            autocomplete="off"
          />
          <div class="row">
            <div class="col-md-4 col-xs-6">
              <button type="button" class="btn-default btn btn-block">
              <FontAwesomeIcon icon={icon({name: "square", style: "regular"})}/> Ignore full
              </button>
            </div>
            <div class="col-md-4 col-md-offset-4 col-xs-6">
              <button
                class="btn btn-primary btn-block"
                type="button"
                title="Shortcut: Enter"
                onClick={() => setCourses([...courses, <ScheduleCourse />])}
              >
                <FontAwesomeIcon icon={icon({name: "plus"})}/> Add Course
              </button>
            </div>
          </div>
        </div>
      </TermSelect>
      <div>&nbsp;</div>
      <div>
        <div class="panel panel-default panel-control-overlap">
          <div class="panel-heading form-horizontal">
            <div class="form-horizontal row">
              <div class="col-xs-12">
                <h2 class="panel-title">Non-Course Schedule Items</h2>
              </div>
            </div>
          </div>
          <div class={nonSchedule.length === 0 ? "panel-body ng-hide" : "panel-body"} hidden={nonSchedule.length === 0}>
            {nonSchedule}
          </div>
          <div class="panel-footer">
            <div class="row">
              <div class="col-md-4 col-md-offset-8">
                <button type="button" class="btn btn-block btn-primary" onClick={() => setNonSchedule([...nonSchedule, <NonCourse/>])}>
                  <FontAwesomeIcon icon={icon({name: "plus"})}/> Add Item
                </button>
              </div>
            </div>
          </div>
        </div>
        <div class="panel panel-default panel-control-overlap">
          <div class="panel-heading form-horizontal">
            <div class="form-horizontal row">
              <div class="col-xs-12">
                <h2 class="panel-title">Times You Don't Want Classes</h2>
              </div>
            </div>
          </div>
          <div class={noSchedule.length === 0 ? "panel-body ng-hide" : "panel-body"} hidden={noSchedule.length === 0}>
            {noSchedule}
          </div>
          <div class="panel-footer">
            <div class="row">
              <div class="col-md-4 col-md-offset-8">
                <button type="button" class="btn btn-block btn-primary" onClick={() => setNoSchedule([...noSchedule, <NoCourse/>])}>
                  <i class="plus"></i> Add Item
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
