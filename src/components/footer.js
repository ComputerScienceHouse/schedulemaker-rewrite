import { Link } from 'react-router-dom'

const Footer = () => {
  return (
    <footer className="main default">
      <div className="container">
        <div className="csh">
          <a target="_blank" rel="noreferrer" href="https://www.csh.rit.edu/">
            <img
              width="90"
              src="./csh_logo_square.svg"
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
        <Link to="/help">
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
          Development v4: Sam Cordry (samc@csh.rit.edu), Joe Abbate (skyz@csh.rit.edu)
          <br />
          Development v3.4: Mary Strodl (mstrodl@csh.rit.edu)
          <br />
          Development v3.1-3.3: Devin Matte (matted@csh.rit.edu)
          <br />
          Development v3: Ben Grawi (bgrawi@csh.rit.edu)
          <br />
          Development v2: Ben Russell (benrr101@csh.rit.edu)
          <br />
          Idea: John Resig (phytar@csh.rit.edu)
          <br />
          Hosting: <a href="https://www.csh.rit.edu">Computer Science House</a>
          <br />
        </div>
      </div>
    </footer>
  );
};

export default Footer;
