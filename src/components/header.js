import { Link } from 'react-router-dom'

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
          <Link className="navbar-brand" ui-sref="index" to="/">
            Schedule<strong>Maker</strong>
          </Link>
        </div>
        <div
          className="collapse navbar-collapse navbar-right navbar-ex1-collapse"
          nav-close-on-mobile=""
        >
          <ul className="nav navbar-nav">
            <li ui-sref-active="active">
              <Link ui-sref="generate" to="/generate">
                <i className="fa fa-calendar-o fa-fw"></i> Make a Schedule
              </Link>
            </li>
            <li ui-sref-active="active">
              <Link ui-sref="browse" to="/browse">
                <i className="fa fa-list fa-fw"></i> Browse Courses
              </Link>
            </li>
            <li ui-sref-active="active">
              <Link ui-sref="search" to="/search">
                <i className="fa fa-search fa-fw"></i> Search Courses
              </Link>
            </li>
          </ul>
        </div>
      </div>
    </header>
  );
};

export default Header;