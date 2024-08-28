import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'
import React from "react";

const numToDay = [
  "Sun",
  "Mon",
  "Tue",
  "Wed",
  "Thu",
  "Fri",
  "Sat",
];

function minutesToTime(mins: number) {
  var h = Math.floor(mins / 60);
  var m: string = `${mins % 60}`;
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
  if (mins % 60 < 10) {
    m = "0" + m;
  }
  return `${h}:${m}${ampm}`;
}

const sectionOptions = (props: any) => {
  let out: Array<React.JSX.Element> = [];
  for (let i = 0; i < props.options.length; i++) {
    let days = props.options[i].times.map((time: { day: number }) => numToDay[time.day]).toString();
    let start = days.length > 0 ? minutesToTime(props.options[i].times[0].start) : "N/A";
    let end = days.length > 0 ? minutesToTime(props.options[i].times[0].end) : "N/A";
    out.push(
      <div
        className="inline-col col-md-6 ng-scope"
        ng-repeat="section in item.sections"
      >
        <ul className="list-group">
          <li className="list-group-item course-info">
            <div className="row">
              <div className="col-sm-8">
                <h4 className="list-group-item-heading">
                  <span
                    course-detail-popover="section.id"
                    className="ng-binding ng-isolate-scope"
                  >
                    {i + 1}. {props.options[i].courseNum}
                  </span>
                </h4>
                <small className="ng-binding">{props.options[i].title}</small>
                <p className="list-group-item-text label-line">
                  <span
                    className="label label-default label-professor ng-binding"
                    ng-bind-html="section.instructor | RMPUrl"
                  >
                    <a
                      target="_blank"
                      rel="noreferrer"
                      href="http://www.ratemyprofessors.com/search.jsp?queryBy=teacherName&amp;queryoption=HEADER&amp;query=Bruce%20Herring&amp;facetSearch=true&amp;schoolName=rochester+institute+of+technology"
                    >
                      {props.options[i].instructor}
                    </a>
                  </span>
                </p>
                <div ng-init="parsedTimes = (section.times | parseSectionTimes)">
                  <div
                    ng-repeat="time in parsedTimes"
                    style={{ fontSize: "small" }}
                    className="ng-binding ng-scope"
                  >
                    {days}{" "}
                    <span style={{ whiteSpace: "nowrap" }} className="ng-binding">
                      {days.length > 0 ? `${start} - ${end}` : ""}
                    </span>
                  </div>
                </div>
              </div>
              <div className="col-sm-4">
                <div className="row">
                  <div className="col-xs-12">
                    <button
                      type="button"
                      className="btn btn-block btn-danger"
                      ng-click="section.selected = !section.selected"
                      ng-className="{'btn-danger':section.selected, 'btn-success':!section.selected}"
                    >
                      <i
                        className="minus"
                        ng-className="{'fa-minus':section.selected, 'fa-plus':!section.selected}"
                      ></i>{" "}
                      <FontAwesomeIcon icon={icon({ name: "shopping-cart" })} />
                    </button>
                  </div>
                </div>
                <div className="text-center">
                  <div
                    className="well-sm ng-binding"
                    style={{ background: "#ddd", margin: "8px 0" }}
                    title="Other students enrolled as of 6AM today"
                  >
                    {props.options[i].curenroll}/{props.options[i].maxenroll} <FontAwesomeIcon icon={icon({ name: "user" })} />
                  </div>
                </div>
              </div>
            </div>
          </li>
        </ul>
      </div>
    );
  }
  return (
    <div
      ng-if="showResults &amp;&amp; !item.sections[0].isError"
      className="ng-scope"
    >
      <div className="visible-xs visible-sm vert-spacer-static-md"></div>
      <div className="course-results-cont row">
        {out}
      </div>
      <div className="visible-xs visible-sm vert-spacer-md"></div>
    </div>
  );
};

export default sectionOptions;
