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
          <a className="navbar-brand" ui-sref="index" href="/">
            Schedule<strong>Maker</strong>
          </a>
        </div>
        <div
          className="collapse navbar-collapse navbar-right navbar-ex1-collapse"
          nav-close-on-mobile=""
        >
          <ul className="nav navbar-nav">
            <li ui-sref-active="active">
              <a ui-sref="generate" href="/generate">
                <i className="fa fa-calendar-o fa-fw"></i> Make a Schedule
              </a>
            </li>
            <li ui-sref-active="active">
              <a ui-sref="browse" href="/browse">
                <i className="fa fa-list fa-fw"></i> Browse Courses
              </a>
            </li>
            <li ui-sref-active="active">
              <a ui-sref="search" href="/search">
                <i className="fa fa-search fa-fw"></i> Search Courses
              </a>
            </li>
          </ul>
        </div>
      </div>
    </header>
  );
};

export default Header;