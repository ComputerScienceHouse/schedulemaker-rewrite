import React, { useEffect, useState } from "react";

const TermSelect = (props: any) => {
  let [terms, setTerms] = useState([]);
  useEffect(() => {
    const fetchData = async () => {
      let termData = await fetch("/api/terms", {
        method: "GET",
        headers: {
          'Content-Type': "application/json"
        }
      }).then((res) => res.json()).then((data) => { return data; }).catch((err) => { console.log(err); throw new Error("Unable to retrieve terms") });
      setTerms(termData());
      props.activeTerm = termData[0].terms[0].termId;
    }
    fetchData();
  }, [props]);
  return (
    <div className="panel panel-default form-horizontal">
      <div className="panel-heading">
        <div className="row form-horizontal">
          <div className="col-sm-4 col-xs-6">
            <h2 className="panel-title control-label pull-left">{props.title}</h2>
          </div>
          <div className="col-sm-8 col-xs-6">
            <div className="row">
              <label className="col-sm-6 control-label hidden-xs" htmlFor="term">
                Term:
              </label>
              <div className="col-sm-6">
                <select className="form-control" onChange={e => props.setActiveTerm(Number(e.target.value))} defaultValue={"Fall 2024"}>
                  {terms.map((entry: { year: number, terms: Array<{ termId: number, termName: string }> }) => {
                    return (
                      <optgroup label={`${entry.year}`}>
                        {entry.terms.map((term) =>
                          <option value={term.termId}>
                            {term.termName}
                          </option>
                        )}
                      </optgroup>
                    )
                  })}
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>
      {
        React.Children.map(
          props.children, child => React.cloneElement(child, { activeterm: props.activeTerm })
        )
      }
    </div>
  );
};

export default TermSelect;
