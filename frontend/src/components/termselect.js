import React, { useEffect, useState } from "react";

const TermSelect = (props) => {
  let [terms, setTerms] = useState([]);
  useEffect(() => {
    const fetchData = async () => {
      let termData = await fetch("/api/terms", {
          method: "GET"
      }).then((res) => {
          if (res.status === 200) {
              return res.json();
          } else {
              throw new Error("Unable to get terms");
          }
      }).then((data) => {
          return Object.entries(data).sort().reverse().map(
            (entry) => [entry[0], entry[1].sort((a,b) => a.termId > b.termId ? -1 : 1)]
          );
      }).catch((err) => {
          return [];
      });
      setTerms(termData);
      props.setActiveTerm(termData[0][1][0].termId);
    }
    fetchData();
  }, []);
  return (
    <div className="panel panel-default form-horizontal">
      <div className="panel-heading">
        <div className="row form-horizontal">
          <div class="col-sm-4 col-xs-6">
            <h2 class="panel-title control-label pull-left">{props.title}</h2>
          </div>
          <div class="col-sm-8 col-xs-6">
            <div class="row">
              <label class="col-sm-6 control-label hidden-xs" for="term">
                Term:
              </label>
              <div class="col-sm-6">
                <select className="form-control" onChange={e => props.setActiveTerm(Number(e.target.value))}>
                  {terms.map((entry) => {
                    console.log(entry);
                    const [year, innerTerms] = entry;
                    console.log(innerTerms);
                    return (
                      <optgroup label={year}>
                        {innerTerms.map((term) => 
                          <option value={term.termId}>
                            {term.termName}
                          </option>
                        )}
                      </optgroup>
                    );
                  })}
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>
      {
        React.Children.map(
          props.children, child => React.cloneElement(child, {activeTerm: props.activeTerm})
        )
      }
    </div>
  );
};

export default TermSelect;
