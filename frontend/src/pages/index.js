import { Link } from 'react-router-dom'

function Index() {
  return (
    <div id="container">
      <div ui-view="" autoscroll="false" style={null}>
        <div className="container">
          <div id="mainMenu" className="row">
            <div className="col-xs-4">
              <div className="navItem">
                <Link  to="/generate">
                  <i className="fa fa-calendar"></i>
                </Link>
                <div>
                  <Link to="/generate">
                    Make a Schedule
                  </Link>
                </div>
              </div>
            </div>
            <div className="col-xs-4">
              <div className="navItem">
                <Link to="/browse">
                  <i className="fa fa-list"></i>
                </Link>
                <div>
                  <Link to="/browse">
                    Browse Courses
                  </Link>
                </div>
              </div>
            </div>
            <div className="col-xs-4">
              <div className="navItem">
                <Link to="/search">
                  <i className="fa fa-search"></i>
                </Link>
                <div>
                  <Link to="/search">
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
