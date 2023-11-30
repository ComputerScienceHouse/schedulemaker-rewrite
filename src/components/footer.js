import { Link } from 'react-router-dom'

const Footer = () => {
  return (
    <footer className="main default">
      <div className="container">
        <div className="csh">
          <a target="_blank" rel="noreferrer" href="https://www.csh.rit.edu/">
            <img
              width="90"
              src="//schedulemaker.csh.rit.edu/img/csh_logo_square.svg"
              alt="CSH"
            />
          </a>
        </div>
        <a
          target="_blank"
          rel="noreferrer"
          href="https://github.com/ComputerScienceHouse/schedulemaker"
        >
          Version: 4.0.0
        </a>{" "}
        |{" "}
        <Link ui-sref="help" to="/help">
          Help
        </Link>{" "}
        | <Link to="/status">Status</Link> |{" "}
        <a
          target="_blank"
          rel="noreferrer"
          href="https://github.com/ComputerScienceHouse/schedulemaker/issues"
        >
          Report Issues
        </a>
        <div>
          Development v4: Mary Strodl (mstrodl at csh.rit.edu), Sam Cordry (samc
          at csh.rit.edu), Joe Abbate (skyz at csh.rit.edu)
          <br />
          Development v3.1: Devin Matte (matted at csh.rit.edu)
          <br />
          Development v3: Ben Grawi (bgrawi at csh.rit.edu)
          <br />
          Development v2: Ben Russell (benrr101 at csh.rit.edu)
          <br />
          Idea: John Resig (phytar at csh.rit.edu)
          <br />
          Hosting: <a href="https://www.csh.rit.edu/">Computer Science House</a>
          <br />
        </div>
      </div>
    </footer>
  );
};

export default Footer;
