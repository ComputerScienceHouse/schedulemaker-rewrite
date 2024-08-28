import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { Link } from 'react-router-dom'
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'
import React from 'react';

const Header = () => {
  return (
    <header className="main navbar navbar-fixed-top navbar-default ng-scope">
      <div className="container">
        <div className="navbar-header">
          <button
            type="button"
            className="navbar-toggle btn btn-default"
            data-toggle="collapse"
            data-target=".navbar-ex1-collapse"
          >
            <span className="sr-only">Toggle navigation</span>
            <span className="icon-bar"></span>
            <span className="icon-bar"></span>
            <span className="icon-bar"></span>
          </button>
          <Link className="navbar-brand" to="/">
            Schedule<strong>Maker</strong>
          </Link>
        </div>
        <div
          className="collapse navbar-collapse navbar-right navbar-ex1-collapse"
          nav-close-on-mobile=""
        >
          <ul className="nav navbar-nav">
            <li ui-sref-active="active">
              <Link to="/generate">
                <FontAwesomeIcon icon={icon({ name: "calendar", style: 'regular' })} /> Make a Schedule
              </Link>
            </li>
            <li ui-sref-active="active">
              <Link to="/browse">
                <FontAwesomeIcon icon={icon({ name: "list" })} /> Browse Courses
              </Link>
            </li>
            <li ui-sref-active="active">
              <Link to="/search">
                <FontAwesomeIcon icon={icon({ name: "search" })} /> Search Courses
              </Link>
            </li>
          </ul>
        </div>
      </div>
    </header>
  );
};

export default Header;
