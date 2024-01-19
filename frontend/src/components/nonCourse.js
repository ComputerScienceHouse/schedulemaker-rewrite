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

const NonCourse = () => {
  let times = [];
  for (let min = 0; min <= 1440; min += 30) {
    times.push(
      <option label={minutesToTime(min)} value={`number:${min}`}>
        12:00am
      </option>
    );
  }
  return (
    <div class="container row form-group repeat-item">
      <div class="col-lg-2 col-md-12">
        <div class="container-fluid">
          <input
            autocomplete="off"
            id="nonCourses0"
            class="form-control"
            type="text"
            name="nonCourses0"
            placeholder="Title"
          />
        </div>
      </div>
      <div class="hidden-lg vert-spacer-static-md"></div>
      <div class="col-lg-5 col-md-6 col-sm-6">
        <div class="row form-inline">
          <div class="col-xs-12">
            <div class="form-group inline-sm">
              <select id="options-startTime" class="form-control">
                <option value="" class="" disabled selected="selected">
                  Start
                </option>
                {times}
              </select>
            </div>
            <div class="form-group inline-sm">&nbsp;to&nbsp;</div>
            <div class="form-group inline-sm">
              <select id="options-endTime" class="form-control">
                <option value="" class="" disabled selected="selected">
                  End
                </option>
                {times}
              </select>
            </div>
          </div>
        </div>
      </div>
      <div class="hidden-lg vert-spacer-static-md"></div>
      <div class="col-lg-4 col-sm-5">
        <div class="container-fluid">
          <div dow-select-fields="nonCourse.days">
            <div class="btn-group btn-group-dow">
              <button type="button" class="btn btn-default btn-dow">
                Su
              </button>
              <button type="button" class="btn btn-default btn-dow">
                Mo
              </button>
              <button type="button" class="btn btn-default btn-dow">
                Tu
              </button>
              <button type="button" class="btn btn-default btn-dow">
                We
              </button>
              <button type="button" class="btn btn-default btn-dow">
                Th
              </button>
              <button type="button" class="btn btn-default btn-dow">
                Fr
              </button>
              <button type="button" class="btn btn-default btn-dow">
                Sa
              </button>
            </div>
          </div>
        </div>
      </div>
      <div class="hidden-md hidden-lg vert-spacer-static-md"></div>
      <div class="col-sm-1">
        <div class="container-fluid">
          <button type="button" class="btn btn-danger hidden-xs">
            <i class="fa fa-times"></i>
          </button>{" "}
          <button type="button" class="btn btn-danger btn-block visible-xs">
            <i class="fa fa-times"></i> Delete
          </button>
        </div>
      </div>
    </div>
  );
};

export default NonCourse;