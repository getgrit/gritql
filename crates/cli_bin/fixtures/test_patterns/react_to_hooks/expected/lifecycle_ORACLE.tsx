import { Component, useEffect } from "react";
import PropTypes from "prop-types";

const Foo = () => {
  useEffect(() => {
    console.log("mounted");
  }, []);

  useEffect(() => {
    return () => {
      console.log("unmounted");
    };
  });

  return <p>Foo</p>;
};

export default Foo;