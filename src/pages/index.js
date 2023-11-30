import { Link } from 'react-router-dom'

function Index() {
  return (
    <div id="container">
      <div ui-view="" autoscroll="false" style={null}>
        <div className="container">
          <div id="mainMenu" className="row">
            <div className="col-xs-4">
              <div className="navItem">
                <Link ui-sref="generate" to="/generate">
                  <i className="fa fa-calendar"></i>
                </Link>
                <div>
                  <Link ui-sref="generate" to="/generate">
                    Make a Schedule
                  </Link>
                </div>
              </div>
            </div>
            <div className="col-xs-4">
              <div className="navItem">
                <Link ui-sref="browse" to="/browse">
                  <i className="fa fa-list"></i>
                </Link>
                <div>
                  <Link ui-sref="browse" to="/browse">
                    Browse Courses
                  </Link>
                </div>
              </div>
            </div>
            <div className="col-xs-4">
              <div className="navItem">
                <Link ui-sref="search" to="/search">
                  <i className="fa fa-search"></i>
                </Link>
                <div>
                  <Link ui-sref="search" to="/search">
                    Search Courses
                  </Link>
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
