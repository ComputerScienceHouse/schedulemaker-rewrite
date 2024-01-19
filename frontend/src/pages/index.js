import { Link } from 'react-router-dom'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'

function Index() {
  return (
    <div id="container">
      <div ui-view="" autoscroll="false" style={null}>
        <div className="container">
          <div id="mainMenu" className="row">
            <div className="col-xs-4">
              <div className="navItem">
                <Link  to="/generate">
                  <FontAwesomeIcon icon={icon({name: 'calendar-days', style: 'regular'})} size="10x"/>
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
                <FontAwesomeIcon icon={icon({name: 'list'})} size="10x"/>
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
                <FontAwesomeIcon icon={icon({name: 'search'})} size="10x"/>
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
