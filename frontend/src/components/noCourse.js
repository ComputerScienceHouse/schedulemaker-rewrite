import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'

function minutesToTime(mins) {
  var h = Math.floor(mins / 60);
  var m = mins % 60;
  var ampm = "am";
  if (h === 24) {
    h = 12;
  } else if (h >= 12) {
    ampm = "pm";
    h = h - 12;
  }
  if (h === 0) {
    h = 12;
  }
  if (m < 10) {
    m = "0" + m;
  }
  return `${h}:${m} ${ampm}`;
}

const NoCourse = () => {
  let times = [];
  for (let min = 0; min <= 1440; min += 30) {
    times.push(
      <option label={minutesToTime(min)} value={`number:${min}`}>
        12:00am
      </option>
    );
  }
  return (
    <div className="container row form-group repeat-item">
      <div className="hidden-lg vert-spacer-static-md"></div>
      <div className="col-lg-5 col-md-6 col-sm-6">
        <div className="row form-inline">
          <div className="col-xs-12">
            <div className="form-group inline-sm">
              <select id="options-startTime" className="form-control">
                <option value="" className="" disabled selected="selected">
                  Start
                </option>
                {times}
              </select>
            </div>
            <div className="form-group inline-sm">&nbsp;to&nbsp;</div>
            <div className="form-group inline-sm">
              <select id="options-endTime" className="form-control">
                <option value="" className="" disabled selected="selected">
                  End
                </option>
                {times}
              </select>
            </div>
          </div>
        </div>
      </div>
      <div className="hidden-lg vert-spacer-static-md"></div>
      <div className="col-lg-4 col-sm-5">
        <div className="container-fluid">
          <div dow-select-fields="nonCourse.days">
            <div className="btn-group btn-group-dow">
              <button type="button" className="btn btn-default btn-dow">
                Su
              </button>
              <button type="button" className="btn btn-default btn-dow">
                Mo
              </button>
              <button type="button" className="btn btn-default btn-dow">
                Tu
              </button>
              <button type="button" className="btn btn-default btn-dow">
                We
              </button>
              <button type="button" className="btn btn-default btn-dow">
                Th
              </button>
              <button type="button" className="btn btn-default btn-dow">
                Fr
              </button>
              <button type="button" className="btn btn-default btn-dow">
                Sa
              </button>
            </div>
          </div>
        </div>
      </div>
      <div className="hidden-md hidden-lg vert-spacer-static-md"></div>
      <div className="col-sm-1">
        <div className="container-fluid">
          <button type="button" className="btn btn-danger hidden-xs">
            <FontAwesomeIcon icon={icon({ name: "times" })} />
          </button>{" "}
          <button type="button" className="btn btn-danger btn-block visible-xs">
            <FontAwesomeIcon icon={icon({ name: "times" })} /> Delete
          </button>
        </div>
      </div>
    </div>
  );
};

export default NoCourse;
