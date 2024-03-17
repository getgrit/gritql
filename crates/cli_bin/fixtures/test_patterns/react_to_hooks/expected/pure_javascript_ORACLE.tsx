import { Component } from "react";
import PropTypes from "prop-types";

const Link = (props) => {
  const { href } = props;

  return <a href={href}>Link Text</a>;
};

Link.propTypes = {
  href: PropTypes.string.isRequired,
};

export default Link;
