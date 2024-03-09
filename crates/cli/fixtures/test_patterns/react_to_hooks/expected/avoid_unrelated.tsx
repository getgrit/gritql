import { Component } from "react";
import PropTypes from "prop-types";

class Other {
  render() {
    const { href } = this.props;

    return <a href={href}>Link Text</a>;
  }
}

const Link = (props) => {
    

    

    const { href } = props;

    return <a href={href}>Link Text</a>; 
};



export default Link;
