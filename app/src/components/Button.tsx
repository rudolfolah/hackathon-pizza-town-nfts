import React from "react";

import "./Button.css";
import {useHistory} from "react-router-dom";

interface ButtonProps {
  children: any;
  onClick?: any;
  to?: string;
}

export default function Button(props: ButtonProps) {
  const history = useHistory();
  const handleClick = (e: any) => {
    if (props.to) {
      history.push(props.to);
    } else if (props.onClick) {
      props.onClick(e);
    }
  };
  return (
    <div className="btn-container">
      <button className="btn-button" onClick={handleClick}>
        {props.children}
      </button>
    </div>
  );
}
