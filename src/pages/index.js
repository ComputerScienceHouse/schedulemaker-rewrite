function Index() {
  return (
    <div id="container">
      <div ui-view="" autoscroll="false" style={null}>
        <div className="container">
          <div id="mainMenu" className="row">
            <div className="col-xs-4">
              <div className="navItem">
                <a ui-sref="generate" href="/generate">
                  <i className="fa fa-calendar"></i>
                </a>
                <div>
                  <a ui-sref="generate" href="/generate">
                    Make a Schedule
                  </a>
                </div>
              </div>
            </div>
            <div className="col-xs-4">
              <div className="navItem">
                <a ui-sref="browse" href="/browse">
                  <i className="fa fa-list"></i>
                </a>
                <div>
                  <a ui-sref="browse" href="/browse">
                    Browse Courses
                  </a>
                </div>
              </div>
            </div>
            <div className="col-xs-4">
              <div className="navItem">
                <a ui-sref="search" href="/search">
                  <i className="fa fa-search"></i>
                </a>
                <div>
                  <a ui-sref="search" href="/search">
                    Search Courses
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Index;
